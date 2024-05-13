# Pycruft

Pycruft is a command-line tool written in Rust for cleaning up Python bytecode `__pycache__`  directories containing `.pyc` and `.pyo` files.

## Overview

Pycruft provides a simple, fast and efficient way to remove unnecessary Python bytecode directories from your project. It helps keep your project directories clean and organized by removing clutter generated during Python development. It can be employed, for instance, within a Git post-checkout hook, to mitigate conflicts during development.

## Features

- Recursively scans directories `__pycache__` directories.
- Verbose mode (`-v`, `--verbose`), display detailed information about the cleaning process.
- Confirmation mode (`-c`, `--confirm`), show directories to be deleted and ask for confirmation.
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

Run `pycruft .` inside the directory you want to clean or provide a path to this directory like this: `pycruft my/path`.

For more options, run `pycruft --help`.

## License

**Pycruft** is licensed under the MIT License.

## Acknowledgements

Pycruft was inspired by **pyclean** (https://github.com/bittner/pyclean)
