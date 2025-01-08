# Ripari CLI by Simora

A performant toolchain for managing and formatting Markdown files.

**Ripari** is a performant toolchain for Markdown files, designed to maintain consistent formatting and style across your documentation.

**Ripari** is a [fast formatter](#) for _Markdown_ files that ensures consistent styling and formatting across your documentation. It handles common issues like smart quotes, dashes, and heading styles.

### Installation

```shell
cargo install ripari
```

### Usage

```shell
# Format files
ripari format --write ./src

# Lint files (coming soon)
ripari lint --write ./src
```

## Example Transformations

### Input

```markdown
# **Introduction to "Smart" Formatting—A Guide**

Here's what ChatGPT typically outputs:
- It uses "smart quotes" everywhere
- Also uses 'single quotes'
- It loves em-dashes—like this—in sentences
```

### Output

```markdown
# Introduction to "Smart" Formatting-A Guide

Here's what ChatGPT typically outputs:
- It uses "smart quotes" everywhere
- Also uses 'single quotes'
- It loves em-dashes-like this-in sentences
```

## Features

- **Fast Formatting**: Quickly process Markdown files with consistent styling
- **Smart Quote Handling**: Converts smart quotes to standard quotes
- **Dash Normalization**: Standardizes em-dashes and en-dashes
- **Heading Cleanup**: Removes unnecessary formatting from headers
- **Glob Pattern Support**: Process multiple files using glob patterns

## Documentation

Check out our documentation to learn more about Ripari:

- [Getting Started](./docs/getting-started.md)
- [Configuration](./docs/configuration.md)
- [Glob Pattern Support](./docs/globby.md)

## Project Philosophy

**Ripari** is designed with these principles in mind:

- **Zero Configuration**: Works out of the box with sensible defaults
- **Extensible**: Modular design allows for easy addition of new features
- **Performance**: Built in Rust for maximum speed and efficiency
- **IDE Integration**: First-class support for VS Code and command-line usage

## License

**Ripari** is MIT licensed.
```

