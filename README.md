# TGIF: Lossless image compression

Simple CLI program to encode/decode grayscale images to/from the **T**urbo **G**ray **I**mage **F**ormat
```bash  
tgif input.png output.tgif --remainder-bits 2 --parallel-encoding-units 1 # Encoding
tgif output.tgif input.bmp  # Decoding
```
