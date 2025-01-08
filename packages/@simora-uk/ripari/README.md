**Ripari** is a performant toolchain for web projects, it aims to provide developer tools to maintain the health of said projects.

**Ripari is a fast formatter** for _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ and _GraphQL_.

**Ripari is a performant linter** for _JavaScript_, _TypeScript_, _JSX_, _CSS_ and _GraphQL_.

**Ripari** is designed from the start to be used interactively within an editor.
It can format and lint malformed code as you are writing it.

### Installation

```shell
npm install --save-dev --save-exact @simora-uk/ripari
```

### Usage

```shell
# format files
npx @simora-uk/ripari format --write ./src

# lint files and apply the safe fixes
npx @simora-uk/ripari lint --write ./src

# run format, lint, etc. and apply the safe fixes
npx @simora-uk/ripari check --write ./src

# check all files against format, lint, etc. in CI environments
npx @simora-uk/ripari ci ./src
```

## Documentation

Check out our [homepage][simora] to learn more about Ripari,
or directly head to the [Getting Started guide][getting-started] to start using Ripari.

## More about Ripari

**Ripari** has sane defaults and it doesn't require configuration.

**Ripari** aims to support [all main languages][language-support] of modern web development.

**Ripari** doesn't require Node.j to function.

**Ripari** has first-class LSP support, with a sophisticated parser that represents the source text in full fidelity and top-notch error recovery.

**Ripari** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelize work, caching, and configuration.

## Funding

You can fund the project in different ways

### Project sponsorship and funding

You can sponsor or fund the project via [GitHub sponsors](https://github.com/sponsors/simeonpashley)

Ripari offers a simple sponsorship program that allows companies to get visibility and recognition among various developers.

### Issue funding

We use [Polar.sh](https://polar.sh/simora-uk/issues) to up-vote and promote specific features that you would like to see and implement. Check our backlog of  and help us.

## License

This application is licensed for personal, non-commercial use. For commercial use, please contact [simora@pashley.org](mailto:simora@pashley.org).

See the [LICENSE.md](./LICENSE.md) file for full details.
