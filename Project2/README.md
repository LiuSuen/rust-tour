# Project 2 Image to ASCII
## Introduction
This program converts an image to ASCII characters.

## Usage
```text
USAGE:
    cargo build
    cargo run <filename>
    
    img_to_ascii [FLAGS] [OPTIONS] <filename>

FLAGS:
    -f, --force      replace existing output file without notification
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>        height of output (default change with width)
    -o, --output <outputfile>    output file
    -w, --width <width>          (默认80)width of output (default=80)

ARGS:
    <filename>    complete file path
```

## Reference
* [img2ascii-Aloxaf](https://github.com/Aloxaf/Rust-toys/tree/master/img2ascii)
