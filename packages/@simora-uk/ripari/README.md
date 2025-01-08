<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/simora/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/simora/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Ripari, with its logo and the phrase 'Ripari - Toolchain of the web'." src="https://raw.githubusercontent.com/simora/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

  [![CI on main][ci-badge]][ci-url]
  [![Discord chat][discord-badge]][discord-url]
  [![npm version][npm-badge]][npm-url]
  [![VSCode version][vscode-badge]][vscode-url]
  [![Open VSX version][open-vsx-badge]][open-vsx-url]
  [![Polar bounties][polar-badge]][polar-url]

  [ci-badge]: https://github.com/simora-uk/ripari/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/simora-uk/ripari/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://pashley.org/chat
  [npm-badge]: https://badgen.net/npm/v/@simora-uk/ripari?icon=npm&color=60a5fa&label=%40simora%2Fripari
  [npm-url]: https://www.npmjs.com/package/@simora-uk/ripari/v/latest
  [vscode-badge]: https://badgen.net/vs-marketplace/v/simora.ripari?label=vscode&icon=visualstudio&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=simora.ripari
  [open-vsx-badge]: https://badgen.net/open-vsx/version/simora/ripari?label=open-vsx&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/simora/ripari
  [polar-badge]: https://polar.sh/embed/seeks-funding-shield.svg?org=simora
  [polar-url]: https://polar.sh/simora

<!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/simora-uk/ripari/tree/main/packages/@simora-uk/ripari -->

  [हिन्दी](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.hi.md) | English | [Français](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.fr.md) | [繁體中文](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.zh-TW.md) | [简体中文](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.zh-CN.md) | [日本語](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.ja.md) | [Português do Brasil](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.pt-BR.md) | [한국어](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.kr.md) | [Русский](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.ru.md) | [Українська](https://github.com/simora-uk/ripari/blob/main/packages/%40simora/ripari/README.uk.md)
</div>

<br>

**Ripari** is a performant toolchain for web projects, it aims to provide developer tools to maintain the health of said projects.

**Ripari is a [fast formatter](./benchmark#formatting)** for _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ and _GraphQL_ that scores **[97% compatibility with _Prettier_](https://console.algora.io/challenges/prettier)**.

**Ripari is a [performant linter](https://github.com/simora-uk/ripari/tree/main/benchmark#linting)** for _JavaScript_, _TypeScript_, _JSX_, _CSS_ and _GraphQL_ that features **[more than 270 rules](https://pashley.org/linter/rules/)** from ESLint, typescript-eslint, and [other sources](https://github.com/simora-uk/ripari/discussions/3).
It **outputs detailed and contextualized diagnostics** that help you to improve your code and become a better programmer!

**Ripari** is designed from the start to be used [interactively within an editor](https://pashley.org/guides/integrate-in-editor/).
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

If you want to give Ripari a run without installing it, use the [online playground](https://pashley.org/playground/), compiled to WebAssembly.

## Documentation

Check out our [homepage][simora] to learn more about Ripari,
or directly head to the [Getting Started guide][getting-started] to start using Ripari.

## More about Ripari

**Ripari** has sane defaults and it doesn't require configuration.

**Ripari** aims to support [all main languages][language-support] of modern web development.

**Ripari** [doesn't require Node.js](https://pashley.org/guides/manual-installation/) to function.

**Ripari** has first-class LSP support, with a sophisticated parser that represents the source text in full fidelity and top-notch error recovery.

**Ripari** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelize work, caching, and configuration.

Read more about our [project philosophy][ripari-philosophy].

**Ripari** is [MIT licensed](https://github.com/simora-uk/ripari/tree/main/LICENSE-MIT) or [Apache 2.0 licensed](https://github.com/simora-uk/ripari/tree/main/LICENSE-APACHE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/simora-uk/ripari/tree/main/CODE_OF_CONDUCT.md).

## Funding

You can fund the project in different ways

### Project sponsorship and funding

You can sponsor or fund the project via [Open collective](https://opencollective.com/ripari) or [GitHub sponsors](https://github.com/sponsors/simora)

Ripari offers a simple sponsorship program that allows companies to get visibility and recognition among various developers.

### Issue funding

We use [Polar.sh](https://polar.sh/simora) to up-vote and promote specific features that you would like to see and implement. Check our backlog and help us:

## Sponsors

### Gold Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://vercel.com/?utm_source=ripari&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/simora/resources/refs/heads/main/sponsors/vercel-dark.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/simora/resources/refs/heads/main/sponsors/vercel-light.png" />
            <img src="https://raw.githubusercontent.com/simora/resources/refs/heads/main/sponsors/vercel-light.png" width="400" alt="Vercel" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### Silver Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100"></a>
      </td>
    </tr>
  </tbody>
</table>

### Bronze Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=ripari&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>

[simora]: https://pashley.org/
[ripari-philosophy]: https://pashley.org/internals/philosophy/
[language-support]: https://pashley.org/internals/language-support/
[getting-started]: https://pashley.org/guides/getting-started/
