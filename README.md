# life
My game of life implementation. Accepts arbitrary rules.

Space to pause/play, click to add cells, r to reset the screen, esc to quit.

# Command line arguments

Most of the features of this program are accessible with command line arguments, which you can see with the --help flag

```
USAGE:
    life [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dimensions <RES>    Sets the resolution. Default is 350x200
    -f, --framerate <FPS>     Sets the framerate. Default is 60
    -r, --rule <RULE>         Sets the rule to be used. Default is 23/3
    -s, --scaling <SCALE>     Sets the display scaling. Can be powers of 2 up to 32
```

# installation

you can build it from source with `cargo build --release`, or you can just download a binary from the releases page.
