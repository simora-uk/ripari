 `mr_globby` provides globbing functionality. When listing the globs to match,
 it also possible to provide globs that function as "exceptions" by prefixing the globs with `!`.

 A glob is primarlly used to select or filter a set of file paths by matching every file paths against the glob.
 A file path either matches or doesn't match a glob.
 For example, the path `lib.rs` matches the glob `*.rs`.

 Biome globs are case-sensitive. This means that `lib.RS` doesn't match `*.rs`.

 You have to understand the structure of a path to understand which path match a glob.
 A path is divided in path segments.
 Every path segment is delimited by the path separator `/` or the start/end of the path.
 For instance `src/lib.rs` cosnists of two path segments: `src` and `lib.rs`.
 A Biome glob supports the following patterns:

 - star `*` that matches zero or more characters inside a path segment

   `lib.rs` matches `*.rs`.
   Conversely, `src/lib.rs` doesn't match `*.rs`

 - globstar `**` that matches zero or more path segments
   `**` must be enclosed by path separators `/` or the start/end of the glob.
   For example, `**a` is not a valid glob.
   Also, `**` must not be followed by another globstar.
   For example, `**/**` is not a valid glob.

   `lib.rs` and `src/lib.rs` match `**` and `**/*.rs`
   Conversely, `README.txt` doesn't match `**/*.rs` because the pat hends with `.txt`.

 - Use `\*` to escape `*`

   the path `*` matches `\*`.

 - `?`, `[`, `]`, `{`, and `}` must be escaped using `\`.
   These characters are reserved for possible future use.

 - Use `!` as first character to negate a glob

   `README.txt` matches `!*.rs`.
   `src/lib.rs` matches `!*.rs` because the path contains several segments.

 ## Matching a path against a glob

 You can create a glob from a string using the `parse` method.
 Use [Glob::is_match] to match against anything that can be turned into a [std::path::Path], such as a string.

 In the following example we parse the string `"*.rs"` into a glob and we match against two strings.
 `lib.rs` matches the glob because the path has a single path segment that ends with `.rs`.
 Conversely, `src/lib.rs` doesn't match because it has two path segments (`src` and `lib.rs`).

 ```
 use mr_globby::Glob;

 let glob: Glob = "*.rs".parse().expect("correct glob");
 assert!(glob.is_match("lib.rs"));
 assert!(!glob.is_match("src/lib.rs"));
 ```

 ## Matching against multiple globs

 When a path is expected to be matched against several globs,
 you should compile the path into a [CandidatePath] using [CandidatePath::new].
 [CandidatePath] may speed up matching against several globs.
 To get adavantage of the speed-up, you have to use the [CandidatePath::matches] method instead of [Glob::is_match].

 In the following example, we create a list of two globs and we match them against a path compiled into a candidate path.
 The path matches the second glob of the list.

 ```
 use mr_globby::{CandidatePath, Glob};

 let globs: &[Glob] = &[
     "**/*.rs".parse().expect("correct glob"),
     "**/*.txt".parse().expect("correct glob"),
 ];

 let path = CandidatePath::new(&"a/path/to/file.txt");

 assert!(globs.iter().any(|glob| path.matches(glob)));
 ```

 ## Matching against multiple globs and exceptions

 Negated globs are particularly useful to denote exceptions in a list of globs.
 To interpret negated globs as exceptions, use [CandidatePath::matches_with_exceptions].
 This semantic is taken from the [.gitignore](https://git-scm.com/docs/gitignore#_pattern_format) format.

 In the following example we accept all files in the `src` directory, except the ones ending with the `txt` extension.

 ```
 use mr_globby::{CandidatePath, Glob};

 let globs: &[Glob] = &[
     "**/*.rs".parse().expect("correct glob"),
     "!**/*.txt".parse().expect("correct glob"),
 ];

 let path = CandidatePath::new(&"a/path/to/file.txt");

 assert!(!path.matches_with_exceptions(globs));
 ```

 ## Matching a directory path against multiple globs and exceptions

 Taking the previous example, the directory path `a/path` doesn't match `**/*.rs` the list of glob,
 because the path doesn't end with the `.rs` extension.
 This behavior is porblematic when you write a file crawler that traverse the file hierarchy and
 ignore directories with files that never match the list of globs.
 Biome provides a deidcated method [CandidatePath::matches_directory_with_exceptions] for this purpose.
 The method only check if the directory is not excluded by an exception.

 In the following example, `dir1` matches the list of globs, while `dir2` doesn't.

 ```
 use mr_globby::{CandidatePath, Glob};

 let globs: &[Glob] = &[
     "**/*.rs".parse().expect("correct glob"),
     "!test".parse().expect("correct glob"),
 ];

 let dir1 = CandidatePath::new(&"src");
 let dir2 = CandidatePath::new(&"test");

 assert!(dir1.matches_directory_with_exceptions(globs));
 assert!(!dir2.matches_directory_with_exceptions(globs));
 ```
