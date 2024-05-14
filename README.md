# Pycruft

Pycruft is a command-line tool written in Rust for cleaning up Python bytecode `__pycache__`  directories containing `.pyc` and `.pyo` files.

## Overview

Pycruft provides a simple, fast and efficient way to remove unnecessary Python bytecode directories from your project. It helps keep your project directories clean and organized by removing clutter generated during Python development. It can be employed, for instance, within a Git post-checkout hook, to mitigate conflicts during development.

## Features

- Recursively scans for `__pycache__` directories.
- Skip confirmation flag (`-s`, `--skip-confirmation`), remove directories without confirmation.
- Verbose flag (`-v`, `--verbose`), display detailed information about the cleaning process.
- Cross-platform support for Windows, macOS, and Linux.

## Getting Started

### Prerequisites

- Rust compiler (https://www.rust-lang.org/tools/install)

### Installation

To install Pycruft, run the following command inside the project directory:

```bash
cargo install --path .
```

## Usage

```
Usage: pycruft [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  Directory to start searching

Options:
  -v, --verbose            Verbose mode
  -s, --skip-confirmation  Ask for confirmation
  -h, --help               Print help
  -V, --version            Print version
```

Run `pycruft .` to clean the current directory. To clean another directory provide a path like this: `pycruft my/path`.

For more options, run `pycruft --help`.

## License

**Pycruft** is licensed under the MIT License.

## Acknowledgements

Pycruft was inspired by **pyclean** (https://github.com/bittner/pyclean)
