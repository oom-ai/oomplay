# oomplay

[![CICD](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/oomplay/actions/workflows/CICD.yml)
![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)
[![crates.io](https://img.shields.io/crates/v/oomplay.svg?colorB=319e8c)](https://crates.io/crates/oomplay)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/oomplay/releases)


Playground manager for [oomstore](https://github.com/oom-ai/oomstore).

## Usage

```
$ oomplay --help
Playground manager for oomstore

USAGE:
    oomplay <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

[SUBCOMMANDS](SUBCOMMANDS):
    init          Initialize playgrounds
    stop          Stop playgrounds
    completion    Output shell completion code
```

## Example

```
$ oomplay init redis
[*] 🎮 Initializing playground 'redis' ...
[*] ⚡ Checking health ...
[*] 🚚 Pulling image 'redis:alpine' ...
[*] 📦 Creating container 'oomplay-redis' ...
[*] 💫 Starting container 'oomplay-redis' ...
[*] ⚡ Checking health ...
[*] 🌀 Initializing database ...
[*] 🔰 Store is ready
[*] ✨ Initialized playground 'redis'
```

```
$ oomplay init -f config.yaml
[*] 🎮 Initializing playground 'online-store' ...
[*] ⚡ Checking health ...
[*] 🌀 Initializing database ...
[*] 🔰 Store is ready
[*] ✨ Initialized playground 'online-store'
[*] 🎮 Initializing playground 'metadata-store' ...
[*] ⚡ Checking health ...
[*] 🔰 Store is already running
[*] 🌀 Initializing database ...
[*] ✨ Initialized playground 'metadata-store'
[*] 🎮 Initializing playground 'offline-store' ...
[*] ⚡ Checking health ...
[*] 🔰 Store is already running
[*] 🌀 Initializing database ...
[*] ✨ Initialized playground 'offline-store'
```

The `config.yaml` used above:
```yaml
online-store:
  redis:
    host: 127.0.0.1
    port: 6379
    password: test
    database: 0

offline-store:
  postgres:
    host: 127.0.0.1
    port: 5432
    user: test
    password: test
    database: test

metadata-store:
  mysql:
    host: 127.0.0.1
    port: 3306
    user: test
    password: test
    database: test
```

Run `oomplay --help` to get detailed usage.

## Supported playgrounds

- [x] Redis
- [x] Postgres
- [x] MySQL
- [x] DynamoDB
- [x] Cassandra

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
