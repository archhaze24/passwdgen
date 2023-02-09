# passwdgen

A pretty simple tool for password generation, written in Rust.

## Usage:
```
passwdgen - a pretty simple tool for password generation

Usage: passwdgen [OPTIONS]

Options:
  -l, --length <LENGTH>         Length of generated passwords. Default is 16.
  -q, --quantity <QUANTITY>     Quantity of generated passwords. Default is 1.
      --remove-similar          Remove similar characters from passwords (i, l, 1, L, o, O, 0). Default is false.
      --remove-numbers          Remove numbers from passwords. Default is false.
      --remove-uppercase        Remove uppercase characters from passwords. Default is false.
      --remove-lowercase        Remove lowercase characters from passwords. Default is false.
  -s, --add-special-characters  Add special characters (!";#$%&'()*+,-./:;<=>?@[]^_`{|}~) to passwords. Default is false.
  -h, --help                    Print help
  -V, --version                 Print version
```

# Building:
Prerequisites: [Rust nightly toolchain](https://rustup.rs/).

`cargo build --release`

## Contributing:
Contributions are welcome - both issues and pull requests!