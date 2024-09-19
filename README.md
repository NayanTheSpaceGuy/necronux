# Necronux

NayanTheSpaceGuy's CLI orchestrator for seamless automation of infrastructure setup, system tweaks, and app installations.

[![Version](https://img.shields.io/github/v/release/NayanTheSpaceGuy/necronux?color=blue&label=Latest%20Release&style=for-the-badge)](https://github.com/NayanTheSpaceGuy/necronux/releases/latest)
[![License](https://img.shields.io/badge/license-GPLv3.0%2B-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0.html)

## Table of Contents
- [Introduction](#necronux)
- [Important Documents](#important-documents)
- [Building the source](#building-the-source)
- [Running](#running)
- [Connect & Support](#connect--support)

## Important Documents

Please refer to the following documents for additional information:

- [CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md): Guidelines for contributing to the project, including expected behavior and how to handle conflicts.
- [AUTHORS.md](AUTHORS.md): Acknowledgment of the core author and contributors to the project.
- [LEGAL.md](LEGAL.md): Detailed licensing information and copyright notices for the project and its components.
- [SECURITY.md](.github/SECURITY.md): Information on supported versions and how to report security vulnerabilities.

Contributors and users are encouraged to review these documents thoroughly.

## Building the Source
### Dependencies
- Ensure you have [Rust](https://www.rust-lang.org/learn/get-started) installed. You can install Rust using `rustup`:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation
- Clone the repository and navigate to the project directory:
```shell
git clone https://github.com/NayanTheSpaceGuy/necronux.git
cd necronux
```

### Building
- Compile the project with:
```shell
cargo build --release
```
- The compiled binary will be located in `target/release/necronux`.

## Running
```shell
./target/release/necronux [OPTIONS] [SUBCOMMAND]
```
To view the help message, you can use:
```shell
./target/release/necronux --help
```
Or simply:
```shell
./target/release/necronux
```

## Connect & Support

[![GitHub](https://img.shields.io/badge/GitHub-NayanTheSpaceGuy-181717?style=for-the-badge&logo=github)](https://github.com/NayanTheSpaceGuy)

### Star this repo if you find it useful! 🌟

[![License](https://img.shields.io/badge/license-GPLv3.0%2B-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0.html)
![Last Commit](https://img.shields.io/github/last-commit/NayanTheSpaceGuy/necronux?style=for-the-badge)
![Repo Size](https://img.shields.io/github/repo-size/NayanTheSpaceGuy/necronux?style=for-the-badge)
