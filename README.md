# rfz
Kinda fzf but in Rust using the [Nucleo](https://github.com/helix-editor/nucleo) crate

### Why it's better than using fzf?
Simple Nucleo search faster than fzf.
When we are talking about his modes uses [jwalk](https://docs.rs/jwalk/latest/jwalk/) to parallelize the search of the possible path and doesn't need to wait ford find.
There is a Stdin mode but it's still a work in progress.





### Options:

- `-f, --file <PATH>`: Search files from the given PATH.
- `-d, --directory <PATH>`: Search directories from the given PATH.
- `-w, --working-dir <PATH>`: Search directories and files from the given PATH.
- `-h, --help`: Print help.
- `-V, --version`: Print version. 

## Example Usage:
- `File mode`
  
![rfz-files-2024-03-10_20 04 35](https://github.com/MangoTzara/rfz/assets/71153363/62a5ecb4-4cec-43c4-bc26-df4982ece71d)

- `Directory mode`

![rfz-directorymode](https://github.com/MangoTzara/rfz/assets/71153363/39aa0b1e-e2c7-44e1-b531-0b1c1b0ce48a)

# rmenu
Uses rfz as a [dmenu_run](https://manpages.debian.org/stretch/suckless-tools/dmenu_run.1.en.html) alternative

![rmenu-2024-03-10_20 05 46](https://github.com/MangoTzara/rfz/assets/71153363/c3031efb-c8e6-4af9-8fdb-3499162082e2)

