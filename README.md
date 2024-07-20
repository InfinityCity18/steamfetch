# steamfetch
Fetch steam data from the terminal

Still not finished, especially since everything is single-threaded

To display images, make sure you are using kitty or something that supports displaying through viuer crate

![Preview image of steamfetch](/preview.png)

```
Usage: steamfetch [options] [commands]

Options:
    -h, --help                  Print help and exit
    -v, --version               Print version and exit
    -f, --font                  Fallback to not using nerd font for display
    -t, --token <TOKEN>         Provide Steam API token, not implemented yet
    -l, --language <LANGUAGE>   Change language of some elements, provided by Steam; e.g. app description
    -o, --offset <OFFSET>       Inserts amount of whitespaces before display, i.e. shifts the whole thing to right. Default is 2

Commands:
    ...     <APP TITLE>         Show app information, ... means that you can supply APP TITLE instantly; every argument after merges into one
    app     <APP TITLE>         As the one above, just that the first argument is not included in search
```

To build:
```
git clone https://github.com/InfinityCity18/steamfetch.git
cd steamfetch
cargo build --release
```
the binary will be in `target/release`
