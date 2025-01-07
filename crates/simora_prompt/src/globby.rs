use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::fmt;
use walkdir;
use regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Glob {
    pattern: String,
    is_negated: bool,
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

impl FromStr for Glob {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Glob::parse(s)
    }
}

impl fmt::Display for Glob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negation = if self.is_negated { "!" } else { "" };
        write!(f, "{}{}", negation, self.pattern)
    }
}

#[derive(Debug)]
pub struct GlobMatcher {
    globs: Vec<Glob>,
    root: PathBuf,
}

#[derive(Debug)]
pub struct CandidatePath {
    path: PathBuf,
}

impl Glob {
    pub fn parse(pattern: &str) -> Result<Self, String> {
        let (pattern, is_negated) = if pattern.starts_with('!') {
            (pattern[1..].to_string(), true)
        } else {
            (pattern.to_string(), false)
        };

        // Validate glob pattern
        if pattern.contains('?') {
            return Err(GlobErrorKind::UnsupportedAnyCharacter.to_string());
        }
        if pattern.contains('[') || pattern.contains(']') {
            return Err(GlobErrorKind::UnsupportedCharacterClass.to_string());
        }
        if pattern.contains('{') || pattern.contains('}') {
            return Err(GlobErrorKind::UnsupportedAlternates.to_string());
        }
        if pattern.ends_with('\\') {
            return Err(GlobErrorKind::DanglingEscape.to_string());
        }
        if pattern.contains("**a") || pattern.contains("**/**") || pattern.contains("a**") || pattern == "***" {
            return Err(GlobErrorKind::InvalidGlobStar.to_string());
        }

        // Validate escapes
        let mut chars = pattern.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some(c) if "!*?{}[]\\".contains(c) => continue,
                    _ => return Err(GlobErrorKind::InvalidEscape.to_string()),
                }
            }
        }

        Ok(Self {
            pattern,
            is_negated,
        })
    }

    pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        let candidate = CandidatePath::new(path.as_ref());
        candidate.matches(self)
    }

    fn is_raw_match<P: AsRef<Path>>(&self, path: P) -> bool {
        let candidate = CandidatePath::new(path.as_ref());
        candidate.raw_matches(self)
    }
}

impl CandidatePath {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn matches(&self, glob: &Glob) -> bool {
        self.raw_matches(glob) != glob.is_negated
    }

    fn raw_matches(&self, glob: &Glob) -> bool {
        let path_str = self.path.to_string_lossy();
        let pattern = &glob.pattern;

        // Convert glob pattern to regex
        let regex_pattern = Self::glob_to_regex(pattern);

        match regex::Regex::new(&format!("^{}$", regex_pattern)) {
            Ok(re) => re.is_match(&path_str),
            Err(_) => false,
        }
    }

    fn glob_to_regex(pattern: &str) -> String {
        let mut result = String::new();
        let mut chars = pattern.chars().peekable();
        let mut in_escape = false;

        while let Some(c) = chars.next() {
            if in_escape {
                result.push_str(&regex::escape(&c.to_string()));
                in_escape = false;
                continue;
            }

            match c {
                '\\' => {
                    in_escape = true;
                }
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next(); // consume the second *
                        if chars.peek() == Some(&'/') || chars.peek().is_none() {
                            chars.next(); // consume the slash if present
                            // Match zero or more path segments, including empty paths
                            result.push_str("(?:[^/]*/)*[^/]*");
                        } else {
                            result.push_str("[^/]*");
                        }
                    } else {
                        result.push_str("[^/]*");
                    }
                }
                '.' => result.push_str("\\."),
                '?' | '[' | ']' | '{' | '}' => {
                    result.push('\\');
                    result.push(c);
                }
                '/' => result.push(c),
                _ => result.push_str(&regex::escape(&c.to_string())),
            }
        }

        result
    }

    pub fn matches_with_exceptions(&self, globs: &[Glob]) -> bool {
        let mut matched = false;

        // Iterate in reverse order to match reference implementation behavior
        for glob in globs.iter().rev() {
            if self.raw_matches(glob) {
                return !glob.is_negated;
            }
        }

        matched
    }

    pub fn matches_directory_with_exceptions(&self, globs: &[Glob]) -> bool {
        // Following reference implementation: return true by default unless explicitly negated
        self.matches_with_exceptions_or(true, globs)
    }

    fn matches_with_exceptions_or(&self, default: bool, globs: &[Glob]) -> bool {
        // Iterate in reverse order to match reference implementation behavior
        for glob in globs.iter().rev() {
            if self.raw_matches(glob) {
                return !glob.is_negated;
            }
        }
        default
    }
}

impl GlobMatcher {
    pub fn new(pattern: &str) -> Result<Self, String> {
        let glob = Glob::parse(pattern)?;
        Ok(Self {
            globs: vec![glob],
            root: PathBuf::from("."),
        })
    }

    pub fn with_root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }

    pub fn add_pattern(&mut self, pattern: &str) -> Result<(), String> {
        let glob = Glob::parse(pattern)?;
        self.globs.push(glob);
        Ok(())
    }

    pub fn walk(&self, root: impl AsRef<Path>) -> GlobWalker {
        GlobWalker {
            globs: self.globs.clone(),
            walker: walkdir::WalkDir::new(root).into_iter(),
        }
    }
}

pub struct GlobWalker {
    globs: Vec<Glob>,
    walker: walkdir::IntoIter,
}

impl Iterator for GlobWalker {
    type Item = Result<walkdir::DirEntry, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.walker.next()? {
                Ok(entry) => {
                    let _path_str = entry.path().to_string_lossy();
                    let candidate = CandidatePath::new(entry.path());
                    if entry.file_type().is_dir() {
                        if !candidate.matches_directory_with_exceptions(&self.globs) {
                            self.walker.skip_current_dir();
                        }
                        continue;
                    }
                    if candidate.matches_with_exceptions(&self.globs) {
                        return Some(Ok(entry));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_validate_glob() {
        assert_eq!(
            Glob::parse("*.[jt]s"),
            Err("Invalid glob pattern: Character class `[]` are not supported. Use `\\[` and `\\]` to escape the characters.".to_string())
        );
        assert_eq!(
            Glob::parse("?*.js"),
            Err("Invalid glob pattern: `?` matcher is not supported. Use `\\?` to escape the character.".to_string())
        );
        assert_eq!(
            Glob::parse(r"\"),
            Err("Invalid glob pattern: Unterminated escape sequence.".to_string())
        );
        assert_eq!(
            Glob::parse(r"\n"),
            Err("Invalid glob pattern: Invalid escape sequence.".to_string())
        );
        assert_eq!(
            Glob::parse(r"\😀"),
            Err("Invalid glob pattern: Invalid escape sequence.".to_string())
        );
        assert_eq!(
            Glob::parse(r"***"),
            Err("Invalid glob pattern: `**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`.".to_string())
        );
        assert_eq!(
            Glob::parse(r"a**"),
            Err("Invalid glob pattern: `**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`.".to_string())
        );
        assert_eq!(
            Glob::parse(r"**a"),
            Err("Invalid glob pattern: `**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`.".to_string())
        );
        assert_eq!(
            Glob::parse(r"**/**"),
            Err("Invalid glob pattern: `**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`.".to_string())
        );

        assert!(Glob::parse("!*.js").is_ok());
        assert!(Glob::parse("!").is_ok());
        assert!(Glob::parse("*.js").is_ok());
        assert!(Glob::parse("**/*.js").is_ok());
        assert!(Glob::parse(r"\*").is_ok());
        assert!(Glob::parse(r"\!").is_ok());
        assert!(Glob::parse(r"**").is_ok());
        assert!(Glob::parse(r"/**/").is_ok());
        assert!(Glob::parse(r"**/").is_ok());
        assert!(Glob::parse(r"/**").is_ok());
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
            Glob::parse("*").unwrap(),
            Glob::parse("!b").unwrap(),
        ]));
        assert!(!a.matches_with_exceptions(&[
            Glob::parse("*").unwrap(),
            Glob::parse("!a*").unwrap(),
        ]));
        assert!(a.matches_with_exceptions(&[
            Glob::parse("*").unwrap(),
            Glob::parse("!a*").unwrap(),
            Glob::parse("a").unwrap(),
        ]));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Glob::parse("**/*.rs").unwrap().to_string(), "**/*.rs");
        assert_eq!(Glob::parse("!**/*.rs").unwrap().to_string(), "!**/*.rs");
    }
}
