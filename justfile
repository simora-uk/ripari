# This is your Justfile for Ripari CLI

# Default task, lists all tasks
_default:
  just --list -u

# Task to build the CLI in release mode
release:
  cargo build --release

# Task to format Rust and TOML files
format:
  cargo format
  taplo format

# Task to run tests for the entire project
test:
  cargo test --no-fail-fast

# Task to run tests for a specific crate
test-crate name:
  cargo test -p {{name}} --no-fail-fast

# Task to lint the entire project using `cargo clippy`
lint:
  cargo clippy

# Task to install necessary development tools
install-tools:
  cargo install cargo-binstall
  cargo binstall cargo-insta taplo-cli

# Task to install necessary development tools
install:
  cargo install --force --path crates/ripari_cli

# Task to generate documentation
documentation:
  RUSTDOCFLAGS='-D warnings' cargo doc

# Task to prepare the project for release, checks diffs, formats code, runs tests
ready:
  git diff --exit-code --quiet
  just format
  just lint
  just test
  just documentation
  git diff --exit-code --quiet
