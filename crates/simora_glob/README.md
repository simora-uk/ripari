# Simora Glob

A glob pattern matching library for the Simora project, providing efficient file path filtering capabilities.

## Overview

`simora_glob` provides globbing functionality for file path matching, with support for both inclusive and exclusive patterns. This is primarily used within the Simora project for file selection and filtering.

## Features

- Case-sensitive glob matching
- Support for standard glob patterns
- Exception-based filtering with `!` prefix
- Directory-aware matching
- Efficient multi-pattern matching

## Pattern Syntax

A glob pattern matches file paths using the following rules:

- `*` - Matches zero or more characters within a path segment
- `**` - Matches zero or more path segments (must be enclosed by `/` or start/end of pattern)
- `\` - Escapes special characters (`*`, `?`, `[`, `]`, `{`, `}`)
- `!` - When used as first character, negates the pattern

### Examples

```rust
use simora_glob::Glob;

// Basic matching
let glob: Glob = "*.rs".parse().unwrap();
assert!(glob.is_match("lib.rs"));
assert!(!glob.is_match("src/lib.rs"));

// Multiple segments
let glob: Glob = "**/*.rs".parse().unwrap();
assert!(glob.is_match("src/lib.rs"));
assert!(glob.is_match("lib.rs"));
```

## Usage

### Single Pattern Matching

```rust
use simora_glob::Glob;

let glob: Glob = "*.rs".parse().expect("valid glob pattern");
assert!(glob.is_match("lib.rs"));
```

### Multiple Pattern Matching

For efficient matching against multiple patterns, use `CandidatePath`:

```rust
use simora_glob::{CandidatePath, Glob};

let globs: &[Glob] = &[
    "**/*.rs".parse().unwrap(),
    "**/*.md".parse().unwrap(),
];

let path = CandidatePath::new("src/lib.rs");
assert!(globs.iter().any(|glob| path.matches(glob)));
```

### Exception Patterns

Use negated patterns with `!` prefix for exclusions:

```rust
use simora_glob::{CandidatePath, Glob};

let globs: &[Glob] = &[
    "**/*.rs".parse().unwrap(),
    "!**/test/**".parse().unwrap(),
];

let path = CandidatePath::new("src/lib.rs");
assert!(path.matches_with_exceptions(globs));

let test_path = CandidatePath::new("src/test/lib.rs");
assert!(!test_path.matches_with_exceptions(globs));
```

### Directory Matching

Special handling for directory traversal:

```rust
use simora_glob::{CandidatePath, Glob};

let globs: &[Glob] = &[
    "**/*.rs".parse().unwrap(),
    "!test".parse().unwrap(),
];

let dir = CandidatePath::new("src");
assert!(dir.matches_directory_with_exceptions(globs));
```

## Notes

- Glob patterns are case-sensitive
- Path segments are delimited by `/`
- `**` must be properly enclosed and cannot be followed by another `**`
- Reserved characters (`?`, `[`, `]`, `{`, `}`) must be escaped with `\`

## License

MIT License - See LICENSE file for details
