# rfz
Kinda fzf but in Rust using the [Nucleo](https://github.com/helix-editor/nucleo) crate

### Why it's better than using fzf?
Simple Nucleo search faster than fzf.
When we are talking about his modes uses [jwalk](https://docs.rs/jwalk/latest/jwalk/) to parallelize the search of the possible path and doesn't need to wait for find.
There is a Stdin mode but it's still a work in progress.





### Options:

- `-f, --file <PATH>`: Search files from the given PATH.
- `-d, --directory <PATH>`: Search directories from the given PATH.
- `-w, --working-dir <PATH>`: Search directories and files from the given PATH.
- `-h, --help`: Print help.
- `-V, --version`: Print version. 

## Example Usage:
- `File mode`
  
![rfz-filemode-2024-03-11_16 47 55](https://github.com/MangoTzara/rfz/assets/71153363/ae9b3944-3fe7-4a9a-b641-327c0510aa52)


- `Directory mode`

![rfz-directorymode-2024-03-11_16 51 04](https://github.com/MangoTzara/rfz/assets/71153363/0974f1f8-d7f4-4224-8e52-2bb671594ec4)


# rmenu
Uses rfz as a [dmenu_run](https://manpages.debian.org/stretch/suckless-tools/dmenu_run.1.en.html) alternative

![rmenu-2024-03-10_20 05 46](https://github.com/MangoTzara/rfz/assets/71153363/c3031efb-c8e6-4af9-8fdb-3499162082e2)

