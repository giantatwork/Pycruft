# Pycbye

Pycbye is a command-line tool written in Rust for cleaning up Python bytecode files (`.pyc` and `.pyo`) and associated cache directories (`__pycache__`).

## Overview

Pycbye provides a simple, fast and efficient way to remove unnecessary Python bytecode files and cache directories from your project directories. It helps keep your project directories clean and organized by removing clutter generated during Python development. It can be employed, for instance, within a Git post-checkout hook, to mitigate conflicts.

## Features

- Recursively scans directories for `.pyc` files and `__pycache__` directories.
- Safe approach: removes all bytecode files (`*.pyc`, `*.pyo`) found in the directory tree and removes only **empty** parent `__pycache__` directories after cleaning bytecode files.
- Allows users to specify a directory to clean or clean the current directory by default.
- Supports verbose mode to display detailed information about the cleaning process.
- Cross-platform support for Windows, macOS, and Linux.

## Getting Started

### Prerequisites

- Rust compiler (https://www.rust-lang.org/tools/install)

### Installation

To install Pycbye, run the following command inside the project directory:

```bash
cargo install --path .
```

## Usage

Run `pycbye .` inside the directory you want to clean.

For more options, run `pycbye --help`.

## License

**Pycbye** is licensed under the MIT License.

## Acknowledgements

Pycbye was inspired by **pyclean** (https://github.com/bittner/pyclean)
