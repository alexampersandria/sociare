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
$ cargo install --locked cocogitto
```

### Diesel

This project uses [Diesel](https://diesel.rs/) and Postgres.

You will need to install and setup both `diesel_cli` and `postgresql`, as well as creating a `.env` file to store the database url.

### setup.sh

There is a `setup.sh` script that will install the required dependencies, setup the database, and create a `.env` file.

> ⚠️ **WARNING:** This will overwrite any existing `.env` file.

```bash
$ ./setup.sh
```

### Manual Setup

#### Installing postgres

```bash
$ sudo apt-get install postgresql postgresql-contrib libpq-dev
$ sudo -u postgres createuser <username>
$ sudo -u postgres createdb <database>
```

#### Creating a user and database

```sql
$ sudo -u postgres psql
psql=# ALTER USER <username> WITH PASSWORD <password>;
psql=# GRANT ALL PRIVILEGES ON DATABASE <database> TO <username>;
```

#### Installing diesel_cli and running migrations

```bash
$ cargo install diesel_cli --no-default-features --features postgres
$ echo DATABASE_URL=postgres://<username>:<password>@<host>/<database> > .env
$ diesel setup
$ diesel migration run
```

### Run

```bash
$ cargo run
```

### Test

```bash
$ cargo test
```

### Lint

```bash
$ cargo fmt
$ cargo clippy
```

### Build

```bash
$ cargo build
```

## Conventional Commits

### Cocogitto

This project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to manage commits.

Use the following format when committing:

```bash
$ git commit -m "type(scope): message"
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