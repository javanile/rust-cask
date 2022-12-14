# Cask

> Docker-based application deployment tool for developer

<!--
<p align="center">
  <a href="https://turbo.build">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/4060187/196936123-f6e1db90-784d-4174-b774-92502b718836.png">
      <img src="https://user-images.githubusercontent.com/4060187/196936104-5797972c-ab10-4834-bd61-0d1e5f442c9c.png" height="128">
    </picture>
    <h1 align="center">Turbo</h1>
  </a>
</p>

<p align="center">
  <a aria-label="Vercel logo" href="https://vercel.com">
    <img src="https://img.shields.io/badge/MADE%20BY%20Vercel-000000.svg?style=for-the-badge&logo=Vercel&labelColor=000">
  </a>
  <a aria-label="NPM version" href="https://www.npmjs.com/package/turbo">
    <img alt="" src="https://img.shields.io/npm/v/turbo.svg?style=for-the-badge&labelColor=000000">
  </a>
  <a aria-label="License" href="https://github.com/vercel/turbo/blob/main/LICENSE">
    <img alt="" src="https://img.shields.io/npm/l/turbo.svg?style=for-the-badge&labelColor=000000&color=">
  </a>
  <a aria-label="Join the community on GitHub" href="https://github.com/vercel/turbo/discussions">
    <img alt="" src="https://img.shields.io/badge/Join%20the%20community-blueviolet.svg?style=for-the-badge&logo=turborepo&labelColor=000000&logoWidth=20&logoColor=white">
  </a>
</p>

Turbo is an next-generation toolchain for frontend development, written in Rust. It consists of 3 major parts:

- [**Turbopack:**](https://turbo.build/pack) an incremental bundler (the successor to Webpack)
- [**Turborepo:**](https://turbo.build/repo) an incremental build system
- The Turbo engine: a low-level incremental computation and memoization engine
-->

## Getting Started

### Cargo

```shell
cargo install cask-cli
```

> NOTE: Run this '$ xcode-select --install' on macOS

## Usage

### Create server

'''shell
$ cask create:server SERVERNAME | sh
'''

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more information.

<!-- 
## Community

The Turbo community can be found on [GitHub Discussions](https://github.com/vercel/turbo/discussions), where you can ask questions, voice ideas, and share your projects.

To chat with other community members, you can join the [Turbo Discord](https://turbo.build/discord)

Our [Code of Conduct](https://github.com/vercel/turbo/blob/main/CODE_OF_CONDUCT.md) applies to all Turbo community channels.
-->

## Who is using Cask?

Cask is used by the world's leading developer (but not at moment). Check out the [#CaskDev](https://twitter.com/hashtag/CaskDev) to learn more.

## Updates

Follow [#CaskDev](https://twitter.com/hashtag/CaskDev) on Twitter and for project updates

## Authors

- Francesco Bianco ([@francescobianco](https://twitter.com/francescobianco))

## Security

If you believe you have found a security vulnerability in Cask, we encourage you to responsibly disclose this and not open a public issue. We will investigate all legitimate reports. Email `bianco@javanile.org` to disclose any security vulnerabilities.

## Reference

- <https://github.com/vercel/turbo/tree/main/shim>
- <https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs>
- <https://apple.stackexchange.com/questions/254380/why-am-i-getting-an-invalid-active-developer-path-when-attempting-to-use-git-a>
