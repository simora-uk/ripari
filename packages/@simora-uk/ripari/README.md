# Ripari - Taming ChatGPT’s Markdown Mayhem

## Overview

Ripari is a lightweight, purpose-built **Markdown cleaner** and **ChatGPT Markdown tool** designed to **fix and standardise Markdown files**. Whether you’re dealing with ChatGPT's quirky Markdown output or messy files, Ripari provides a smarter, faster, and safer solution. It simplifies formatting tasks, making it an essential tool for content creators, developers, and anyone working with Markdown.


## Features

- **Designed for ChatGPT Markdown Output**:
  - Corrects quirks such as smart punctuation, over-emphasised headings, unnecessary horizontal rules, and inconsistent em-dashes.
- **Batch Processing**:
  - Supports single files, directories, and glob patterns for bulk operations.
- **Customisable Transformations**:
  - Disable or adjust specific rules to fit your unique requirements.
- **Intuitive CLI**:
  - Simple and efficient commands to streamline your workflow.
- **Sane Defaults, Zero Configuration**
  - Ripari works out of the box with sensible defaults, requiring no configuration for most users.
- **Lightning-Fast and Reliable**
  - Built in Rust, Ripari eliminates the need for Node.js dependencies, delivering a robust and speedy experience.

**Why Ripari Over Search & Replace?**

- **Context-Aware**: Avoids damaging code blocks and quotes by understanding Markdown structure.
- **Safe Transformations**: Applies changes without altering sensitive content.
- **Configurable**: Customise rules to match your specific needs.
- **Batch Processing**: Cleans multiple files simultaneously, saving time.
- **Deterministic**: Consistent, repeatable results with every run.

### Installation

```shell
npm install --save-dev --save-exact @simora-uk/ripari
```

### Usage

```shell
# Format files
npx @simora-uk/ripari format --write ./src

# Lint files and apply safe fixes
npx @simora-uk/ripari lint --write ./src

# Run format, lint, and apply safe fixes
npx @simora-uk/ripari check --write ./src

# Check files in CI environments
npx @simora-uk/ripari ci ./src
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

## Configuration

Ripari provides fine-grained control over Markdown formatting through a configuration file (`simora.json`). It adapts to the structure and context of Markdown, avoiding indiscriminate search-and-replace logic.

### Available Rules

1. **Smart Quotes** (`smart_quotes`):

   - Converts "smart" quotes to standard ASCII quotes.
   - Preserves apostrophes in contractions and ignores code blocks.

2. **Headings** (`headings`):

   - Removes bold/italic emphasis from headings.

3. **Horizontal Rules** (`remove_horizontal_rules`):

   - Removes unnecessary rules outside of code blocks.

4. **Punctuation** (`punctuation`):
   - Converts em-dashes to hyphens and ellipses to three dots.

### Configuration Example

```json
{
  "markdown": {
    "enabled": true,
    "rules": {
      "smart_quotes": { "enabled": true },
      "headings": { "enabled": true, "remove_emphasis": true },
      "remove_horizontal_rules": { "enabled": true },
      "punctuation": {
        "enabled": true,
        "standardize_dashes": true,
        "standardize_ellipsis": true
      }
    }
  },
  "files": {
    "include": ["**/*.md"]
  },
  "root": true
}
```

### Context-Aware Processing

Ripari ensures your Markdown retains its structure:

1. **Code Blocks**: No transformations are applied inside code blocks.
2. **Blockquotes**: Quoted content remains untouched.
3. **Markdown Context**: Rules are applied selectively based on content type.

## Funding and Sponsorship

Support Ripari's development by sponsoring via [GitHub Sponsors](https://github.com/sponsors/simeonpashley). Companies can also benefit from increased visibility among developers.

Use [Polar.sh](https://polar.sh/simora-uk/issues) to fund specific features or vote on the backlog.

## License

Ripari is licensed for personal, non-commercial use. For commercial use, contact [simora@pashley.org](mailto:simora@pashley.org).

For full details, see [LICENSE.md](./LICENSE.md).
