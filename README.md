# oomplay

[![CICD](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml)
![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/oomplay.svg?colorB=319e8c)](https://crates.io/crates/oomplay)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/oomplay/releases)


Playground manager for [oomstore](https://github.com/oom-ai/oomstore).

## Usage

```
$ oomplay --help
oomplay 0.1.2

Playground manager for oomstore

USAGE:
    oomplay <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    stop          Stop playgrounds
    init          Initialize playgrounds
    completion    Output shell completion code
    help          Print this message or the help of the given subcommand(s)
```

## Example

```
$ oomplay init -c config.yaml
[*] init offline-store...
[*] init online-store...
[*] init metadata-store...
```

The `config.yaml` used above:
```yaml
online-store:
  redis:
    host: 127.0.0.1
    port: 6379
    password: redis
    database: 0

offline-store:
  postgres:
    host: 127.0.0.1
    port: 5432
    user: postgres
    password: postgres
    database: oomstore_test

metadata-store:
  mysql:
    host: 127.0.0.1
    port: 3306
    user: mysql
    password: mysql
    database: oomstore_test
```

Run `oomplay --help` to get detailed usage.

## Installation

### From binaries

Pre-built versions of `oomplay` for various architectures are available at [Github release page](https://github.com/oom-ai/oomplay/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

### From source

`oomplay` is also published on [crates.io](https://crates.io). If you have Rust toolchains (nightly) installed you can use `cargo` to install it from source:

```
cargo install --locked oomplay
```

If you want the latest version, clone this repository and run `cargo install --path .`.
