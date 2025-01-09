./packages/@simora-uk/ripari/README.md

### CURSORRULES for Claude Sonnet

Claude Sonnet was trained on XML formatted rules, so it's best to use that format.

Best Practices for cursorrules:
- Use enumerated or bulletized rules
- Keep rules short, clear, and concise
- Group related rules together using <XML> tags

Example:

```
- Speak to me in Spanish
- My build system is bazel
- Don't modify any files in app/config
- Don't use any APIs that require authorization
```

Then modifed with XML tags:

```
 <communication>
1. Speak to me in English
2. My name is Bob.
</communication>

<filesytem>
1. Don't modify any files in app/config
</filesystem>

<coding>
1. Don't use any APIs that require authorization
2. My project's programming language is python
3. Use pytest as my test framework
</coding>
```
