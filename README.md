<h1 align="center">
  🤝 sociare
</h1>

<p align="center">
  A WeShare replacement API in Rust
</p>

<p align="center">
  <img src="https://github.com/alexampersandria/sociare/actions/workflows/rust.yml/badge.svg" alt="GitHub Actions Build Badge" />
</p>

## Getting Started

It is recommended that you install [Cocogitto](https://github.com/cocogitto/cocogitto) to ensure that your commits follow the Conventional Commits specification.

```bash
cargo install --locked cocogitto
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo fmt
cargo clippy
```

### Build

```bash
cargo build
```

## Conventional Commits

### Cocogitto

This project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to manage commits.

Use the following format when committing:

```bash
git commit -m "type(scope): message"
```

#### Commit Types

`feat` A new feature

`fix` A bug fix

`docs` Documentation only changes

`style` Changes that do not affect the meaning of the code

`refactor` A code change that neither fixes a bug nor adds a feature

`perf` A code change that improves performance

`test` Adding missing tests or correcting existing tests

`build` Changes that affect the build system or external dependencies

`ci` Changes to our CI configuration files and scripts

`chore` Other changes that don't modify src or test files

`revert` Reverts a previous commit

#### Scope

The scope is optional and should be a GitHub issue number if applicable.