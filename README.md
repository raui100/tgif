# TGIF: Lossless image compression

Simple CLI program to encode/decode grayscale images to/from the **T**urbo **G**ray **I**mage **F**ormat.  
Uses Rice-Coding with delta filter and bit-padding to enable parallel decoding. 

```bash  
tgif -r 1 input.png output.tgif # Encoding
tgif output.tgif input.bmp  # Decoding
```
