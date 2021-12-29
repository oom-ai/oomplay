# oomplay

[![CICD](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml)
![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/oomplay.svg?colorB=319e8c)](https://crates.io/crates/oomplay)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/oomplay/releases)


Playground manager for [oomstore](https://github.com/oom-ai/oomstore).

## Usage

```help
$ oomplay --help
oomplay 0.4.4
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
[*] 📡 Pinging database ...
[*] 📦 Creating container ...
[*] 🚀 Starting container ...
[*] 📡 Pinging database ...
[*] 💫 Initializing database ...
[*] 🟢 Store is ready.
[*] 🎮 Initializing oomplay-postgres ...
[*] 📡 Pinging database ...
[*] 📦 Creating container ...
[*] 🚀 Starting container ...
[*] 📡 Pinging database ...
[*] 📡 Pinging database ...
[*] 📡 Pinging database ...
[*] 💫 Initializing database ...
[*] 🟢 Store is ready.
```

```
$ oomplay stop redis postgres
[*] 🔌 Stopping oomplay-redis ...
[*] 🔴 Stopped.
[*] 🔌 Stopping oomplay-postgres ...
[*] 🔴 Stopped.
```

## Supported playgrounds

- [x] Redis
- [x] Postgres
- [x] MySQL
- [x] DynamoDB
- [x] Cassandra
- [x] TiDB
- [x] TiKV

## Installation

### On macOS

You can install `oomplay` with Homebrew:

```
brew tap oom-ai/oomplay
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
