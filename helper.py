import numpy as np

s = "const U8_TO_ARRAY_BOOL: [[bool; 8]; 256] = ["
for n in range(256):
    s += "["
    for c in np.binary_repr(n, 8):  # 8-bit binary representation as string (eg: 0 -> "00000000")
        if c == "0":
            s += "false, "
        else:
            s += "true, "

    s = s[:-2] + "], "

s = s[:-1] + "];"

print(s)