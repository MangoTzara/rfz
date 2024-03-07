# rfz
Kinda fzf but in Rust using the [Nucleo](https://github.com/helix-editor/nucleo) crate

## Options:

- `-f, --file <PATH>`: Search files from the given PATH.
- `-d, --directory <PATH>`: Search directories from the given PATH.
- `-w, --working-dir <PATH>`: Search directories and files from the given PATH.
- `-h, --help`: Print help.
- `-V, --version`: Print version. 

## Example Usage:

```sh
rfz -f /path/to/file
rfz -d /path/to/directory
rfz -w /path/to/working/directory
rfz -h
rfz -V
```
# rmenu
Uses rfz as a [dmenu_run](https://manpages.debian.org/stretch/suckless-tools/dmenu_run.1.en.html) alternative
