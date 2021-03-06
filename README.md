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

### Annoying to type password everytime?

1. Create this function in your `~/.bashrc` (bash 4.2+).

    ```sh
    function cryptenv () { 
        local varname=CRYPTENV_PASSWORD;
        if [ ! -v $varname ]; then
            read -s -p 'password? ' $varname;
            echo;
            export $varname;
        fi;
        $(which cryptenv) --password-from $varname "$@"
    }
    ```

    - A password is required only the first time.
    - Then, use the password stored in the env var of the shell without prompting.
