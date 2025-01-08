use globset;
use regex;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use walkdir;

#[derive(Debug, Clone)]
pub struct Glob {
    is_negated: bool,
    glob: globset::GlobMatcher,
}

impl PartialEq for Glob {
    fn eq(&self, other: &Self) -> bool {
        self.is_negated == other.is_negated && self.glob.glob() == other.glob.glob()
    }
}

#[derive(Debug, PartialEq)]
pub enum GlobErrorKind {
    DanglingEscape,
    InvalidEscape,
    InvalidGlobStar,
    UnsupportedAlternates,
    UnsupportedCharacterClass,
    UnsupportedAnyCharacter,
}

impl fmt::Display for GlobErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::DanglingEscape => "Unterminated escape sequence.",
            Self::InvalidEscape => "Invalid escape sequence.",
            Self::InvalidGlobStar => {
                r"`**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`."
            }
            Self::UnsupportedAlternates => {
                r"Alternates `{}` are not supported. Use `\{` and `\}` to escape the characters."
            }
            Self::UnsupportedCharacterClass => {
                r"Character class `[]` are not supported. Use `\[` and `\]` to escape the characters."
            }
            Self::UnsupportedAnyCharacter => {
                r"`?` matcher is not supported. Use `\?` to escape the character."
            }
        };
        write!(f, "Invalid glob pattern: {}", desc)
    }
}

#[derive(Debug)]
pub struct GlobError {
    kind: GlobErrorKind,
    index: Option<u32>,
}

impl GlobError {
    fn new(kind: GlobErrorKind, index: Option<u32>) -> Self {
        Self { kind, index }
    }
}

impl std::error::Error for GlobError {}

impl fmt::Display for GlobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl FromStr for Glob {
    type Err = GlobError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (is_negated, value) = if let Some(stripped) = value.strip_prefix('!') {
            (true, stripped)
        } else {
            (false, value)
        };
        validate_glob(value)?;
        let mut glob_builder = globset::GlobBuilder::new(value);
        // Allow escaping with `\` on all platforms
        glob_builder.backslash_escape(true);
        // Only `**` can match `/`
        glob_builder.literal_separator(true);
        match glob_builder.build() {
            Ok(glob) => Ok(Glob {
                is_negated,
                glob: glob.compile_matcher(),
            }),
            Err(error) => Err(GlobError::new(GlobErrorKind::InvalidGlobStar, None)),
        }
    }
}

impl fmt::Display for Glob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negation = if self.is_negated { "!" } else { "" };
        write!(f, "{}{}", negation, self.glob.glob())
    }
}

#[derive(Debug)]
pub struct CandidatePath<'a>(globset::Candidate<'a>);

impl<'a> CandidatePath<'a> {
    pub fn new(path: &'a impl AsRef<Path>) -> Self {
        Self(globset::Candidate::new(path))
    }

    pub fn matches(&self, glob: &Glob) -> bool {
        self.raw_matches(glob) != glob.is_negated
    }

    pub fn matches_with_exceptions<'b, I>(&self, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        self.matches_with_exceptions_or(false, globs)
    }

    pub fn matches_directory_with_exceptions<'b, I>(&self, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        self.matches_with_exceptions_or(true, globs)
    }

    fn matches_with_exceptions_or<'b, I>(&self, default: bool, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        // Iterate in reverse order to avoid unnecessary glob matching
        for glob in globs.into_iter().rev() {
            if self.raw_matches(glob) {
                return !glob.is_negated;
            }
        }
        default
    }

    fn raw_matches(&self, glob: &Glob) -> bool {
        glob.glob.is_match_candidate(&self.0)
    }
}

impl Glob {
    pub fn is_negated(&self) -> bool {
        self.is_negated
    }

    pub fn is_match(&self, path: impl AsRef<Path>) -> bool {
        self.is_raw_match(path) != self.is_negated
    }

    fn is_raw_match(&self, path: impl AsRef<Path>) -> bool {
        self.glob.is_match(path)
    }
}

fn validate_glob(pattern: &str) -> Result<(), GlobError> {
    let mut it = pattern.bytes().enumerate();
    let mut allow_globstar = true;
    while let Some((i, c)) = it.next() {
        match c {
            b'*' => {
                let mut lookahead = it.clone();
                if matches!(lookahead.next(), Some((_, b'*'))) {
                    if !allow_globstar || !matches!(lookahead.next(), None | Some((_, b'/'))) {
                        return Err(GlobError::new(
                            GlobErrorKind::InvalidGlobStar,
                            Some(i as u32),
                        ));
                    }
                    // Eat `*`
                    it.next();
                    // Eat `/`
                    it.next();
                }
            }
            b'\\' => {
                // Accept a restrictive set of escape sequences
                if let Some((_, c)) = it.next() {
                    if !matches!(c, b'!' | b'*' | b'?' | b'{' | b'}' | b'[' | b']' | b'\\') {
                        return Err(GlobError::new(GlobErrorKind::InvalidEscape, Some(i as u32)));
                    }
                } else {
                    return Err(GlobError::new(
                        GlobErrorKind::DanglingEscape,
                        Some(i as u32),
                    ));
                }
            }
            b'?' => {
                return Err(GlobError::new(
                    GlobErrorKind::UnsupportedAnyCharacter,
                    Some(i as u32),
                ));
            }
            b'[' | b']' => {
                return Err(GlobError::new(
                    GlobErrorKind::UnsupportedCharacterClass,
                    Some(i as u32),
                ));
            }
            b'{' | b'}' => {
                return Err(GlobError::new(
                    GlobErrorKind::UnsupportedAlternates,
                    Some(i as u32),
                ));
            }
            _ => {}
        }
        allow_globstar = c == b'/';
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;
    #[cfg(test)]
    use tempfile::TempDir;

    #[test]
    fn test_validate_glob() {
        assert!(matches!(
            Glob::from_str("*.[jt]s"),
            Err(GlobError {
                kind: GlobErrorKind::UnsupportedCharacterClass,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str("?*.js"),
            Err(GlobError {
                kind: GlobErrorKind::UnsupportedAnyCharacter,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"\"),
            Err(GlobError {
                kind: GlobErrorKind::DanglingEscape,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"\n"),
            Err(GlobError {
                kind: GlobErrorKind::InvalidEscape,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"***"),
            Err(GlobError {
                kind: GlobErrorKind::InvalidGlobStar,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"a**"),
            Err(GlobError {
                kind: GlobErrorKind::InvalidGlobStar,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"**a"),
            Err(GlobError {
                kind: GlobErrorKind::InvalidGlobStar,
                ..
            })
        ));
        assert!(matches!(
            Glob::from_str(r"**/**"),
            Err(GlobError {
                kind: GlobErrorKind::InvalidGlobStar,
                ..
            })
        ));

        assert!(Glob::from_str("!*.js").is_ok());
        assert!(Glob::from_str("!").is_ok());
        assert!(Glob::from_str("*.js").is_ok());
        assert!(Glob::from_str("**/*.js").is_ok());
        assert!(Glob::from_str(r"\*").is_ok());
        assert!(Glob::from_str(r"\!").is_ok());
        assert!(Glob::from_str(r"**").is_ok());
        assert!(Glob::from_str(r"/**/").is_ok());
        assert!(Glob::from_str(r"**/").is_ok());
        assert!(Glob::from_str(r"/**").is_ok());
    }

    #[test]
    fn test_is_match() {
        assert!("*.rs".parse::<Glob>().unwrap().is_match("lib.rs"));
        assert!(!"*.rs".parse::<Glob>().unwrap().is_match("src/lib.rs"));
        assert!("**/*.rs".parse::<Glob>().unwrap().is_match("src/lib.rs"));
    }

    #[test]
    fn test_matches_with_exceptions() {
        let a = CandidatePath::new(&"a");

        assert!(a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!b").unwrap(),
        ]));
        assert!(!a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!a*").unwrap(),
        ]));
        assert!(a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!a*").unwrap(),
            Glob::from_str("a").unwrap(),
        ]));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Glob::from_str("**/*.rs").unwrap().to_string(), "**/*.rs");
        assert_eq!(Glob::from_str("!**/*.rs").unwrap().to_string(), "!**/*.rs");
    }
}
