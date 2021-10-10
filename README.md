# `cryptenv`

[![Rust](https://github.com/shosatojp/cryptenv/actions/workflows/rust.yml/badge.svg)](https://github.com/shosatojp/cryptenv/actions/workflows/rust.yml)

A transparent environment variables decryptor.

- encrypt with AES-256-CBC

## Install

```sh
cargo install cryptenv
```

## Usage

### Prepare encrypted environment variable

```sh
cryptenv --data <(echo -n "THIS IS TOP SECRET")

# password? # input password
# cryptenv://ndDGOi3AUgcB4XOiiimRmfY8lEvoBtYZF8mrappszvuhyjAqtqt2IxIf2iFXx+If
```

use this URI string for environment variable value.

```sh
# ~/.bashrc
TOP_SECRET=cryptenv://ndDGOi3AUgcB4XOiiimRmfY8lEvoBtYZF8mrappszvuhyjAqtqt2IxIf2iFXx+If
```

### Run command with crypted secrets

```sh
cryptenv -- env | grep TOP_SECRET

# password? # input password
# TOP_SECRET=THIS IS TOP SECRET
```
