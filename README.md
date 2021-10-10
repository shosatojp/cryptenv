# `cryptenv`

A transparent environment variables decryptor.

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
