+++
title = "Print Double As Binary"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

**Problem**: Print a double that is `$0 \lt x \lt 1$` as a binary number.

**Approach**: Multiply the number by 2 to bitwise shift the first number past the decimal.

If the number is now 1, then it means that the shifted bit was a 1, so print a 1 and remove the left part of the decimal by subtracting 1.

Otherwise the shifted bit was a 0, so print a 0.

``` python
def double_to_binary(num):
    out = "0."

    while num > 0:
        shifted = num * 2

        if shifted >= 1.0:
            out += "1"
            num = shift - 1
        else:
            out += "0"
            num = shift
    
    return out
```
