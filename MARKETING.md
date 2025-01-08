**Ripari: Fix ChatGPT Markdown, Fast and Flawless**

Meet **Ripari**, the definitive solution for anyone fed up with the quirky markdown ChatGPT insists on generating. Built by **Simora UK**, Ripari is a **command-line tool purpose-built to clean up and standardise ChatGPT output** in Markdown files. It's the ultimate time-saver for anyone who uses ChatGPT extensively for documentation and wants consistent, high-quality results without the frustration.

### Why Ripari?

**ChatGPT Markdown Output is... Problematic**
If you've tried asking ChatGPT to format Markdown correctly, you know the struggle:
- **Smart punctuation** that looks fancy but breaks consistency.
- **Unnecessary bold in headings** that feels overstyled.
- **Unwanted horizontal rules** where you didn't ask for them.
- **Dashes instead of hyphens**-why does it ignore the prompt?
Ripari fixes all of this and more in a single command.

**A Unified Solution**
Instead of wrestling with endless **search & replace commands** or crafting **error-prone regex**, Ripari brings everything together in a streamlined CLI. No more cobbling together workarounds-just clean, standardised Markdown at the push of a button.

### Key Features

- **Quirk-Free Markdown**: Automatically remove ChatGPT's common formatting flaws, including smart punctuation, bold headings, dashes, and more.
- **Bulk Processing**: Process single files, entire directories, or custom file globs to clean up Markdown in bulk.
- **Simple Yet Powerful**: Ripari's default behaviours are all you need, but you can configure the tool to disable specific transformations for greater flexibility.
- **Future-Ready**: While Ripari is a CLI-first tool, it's paving the way for a **Visual Studio Code extension** to bring seamless cleanup directly into your editor.

### Who is Ripari For?

If you:
- Use ChatGPT extensively for writing documentation, code snippets, or notes,
- Need Markdown output that's clean and professional,
- Want a no-nonsense tool to fix formatting issues at scale,
Ripari is built for you.

### How Does it Work?

Run Ripari from your terminal with intuitive commands:
- **Fix a single file**:
  ```bash
  ripari format example.md
  ```
- **Clean an entire directory**:
  ```bash
  ripari format --path ./docs
  ```
- **Refactor multiple Markdown files with globs**:
  ```bash
  ripari format --path ./notes/**/*.md
  ```

With **Ripari**, say goodbye to Markdown headaches and hello to formatting consistency. It's **Markdown that works, without the quirks.**
