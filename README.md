# TGIF: Lossless image compression
> **T**urbo **G**ray **I**mage **F**ormat

Simple CLI program to decode grayscale images :rocket: **blazingly fast** :rocket:  
On my old `i7-4700MQ CPU @ 2.40GHz` I could achieve a decoding speed of `0.75 [GBit/s]` using a single core and `2.3 [Gbit/s]` using all 4 cores.  
The encoding speed is `0.4 [GBit/s]` on my machine and there

Uses Rice-Coding with delta filter and bit-padding to enable parallel decoding.


```bash
# Example usage
tgif input.png output.tgif --rem-bits 2 --chunk-size 128
tgif input.tgif output.png

# Use cargo to build the project
cargo run --release -- help 

Usage: tgif [OPTIONS] <SRC> <DST>

Arguments:
  <SRC>  Input image (eg: TGIF, PNG, ...)
  <DST>  Output image (eg: TGIF, PNG, ...)

Options:
  -r, --rem-bits <REM_BITS>      Number of bits used to encode the remainder. Should be 0..=7. [Default: 2]
  -c, --chunk-size <CHUNK_SIZE>  Size of self contained chunk in Kibibyte. Should be equal to L1 cache size. [Default: 128]
  -h, --help                     Print help information
  -V, --version                  Print version information
```
