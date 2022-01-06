# oomplay

[![CICD](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml)
![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/oomplay.svg?colorB=319e8c)](https://crates.io/crates/oomplay)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/oomplay/releases)


Playground manager for [oomstore](https://github.com/oom-ai/oomstore).

## Supported playgrounds

- [x] Redis
- [x] Postgres
- [x] MySQL
- [x] DynamoDB
- [x] Cassandra
- [x] TiDB
- [x] TiDBExt
- [x] TiKV
- [x] TiKVExt
- [x] SQLite
- [x] SnowflakeExt
- [x] BigQueryExt
- [x] RedshiftExt

*Postfix `Ext` means using the external service supplied by the user.*

## Usage

```help
$ oomplay --help
oomplay 0.5.0
Playground manager for oomstore

USAGE:
    oomplay <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    init          Initialize playgrounds
    stop          Stop playgrounds
    list          List supported playgrounds
    completion    Output shell completion code
```

## Example

```
$ oomplay init redis postgres
[*] 🎮 Initializing oomplay-redis ...
[*] 📡 Pinging oomplay-redis ...
[*] 🎮 Initializing oomplay-postgres ...
[*] 📡 Pinging oomplay-postgres ...
[*] 🚚 Pulling image 'redis:alpine' ...
[*] 📦 Creating container oomplay-postgres ...
[*] 🚀 Starting container oomplay-postgres ...
[*] 📡 Pinging oomplay-postgres ...
[*] 📡 Pinging oomplay-postgres ...
[*] 📡 Pinging oomplay-postgres ...
[*] 📡 Pinging oomplay-postgres ...
[*] 💫 Initializing oomplay-postgres ...
[*] 🟢 oomplay-postgres is ready. (8.376110205s)
[*] 📦 Creating container oomplay-redis ...
[*] 🚀 Starting container oomplay-redis ...
[*] 📡 Pinging oomplay-redis ...
[*] 💫 Initializing oomplay-redis ...
[*] 🟢 oomplay-redis is ready. (15.544168376s)
```

```
$ oomplay stop redis postgres
[*] 🔌 Stopping oomplay-redis ...
[*] 🔌 Stopping oomplay-postgres ...
[*] 🔴 oomplay-redis stopped.
[*] 🔴 oomplay-postgres stopped.
```

**SnowflakeExt**
SnowflakeExt playground requires the following environment variables:

```
SNOWFLAKE_ACCOUNT
SNOWFLAKE_USER
SNOWFLAKE_PASSWORD
SNOWFLAKE_DATABASE
```

**BigQueryExt**
BigQueryExt playground requires the following environment variables:

```
BIGQUERY_CREDENTIALS
BIGQUERY_DATASET_ID
```
**RedshiftExt**
RedshiftExt playground requires the following environment variables:

```
REDSHIFT_HOST
REDSHIFT_USER
REDSHIFT_DATABASE
REDSHIFT_PASSWORD
REDSHIFT_DEFAULT_DATABASE
```

## Installation

### On macOS

You can install `oomplay` with Homebrew:

```
brew tap oom-ai/oom-ai
brew install oomplay
```

### From binaries

Pre-built versions of `oomplay` for various architectures are available at [Github release page](https://github.com/oom-ai/oomplay/releases).

### From source

`oomplay` is also published on [crates.io](https://crates.io). If you have Rust toolchains (nightly) installed you can use `cargo` to install it from source:

```
cargo install --locked oomplay
```

If you want the latest version, clone this repository and run `cargo install --path .`.
