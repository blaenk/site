+++
title = "Get Least Significant Set Bit"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["bitwise"]
+++

The least significant set bit can be unset with:

``` python
x = 0101 0100
least_significant_set_bit_unset = x & (x - 1)
```

This works because all bits up-to-and-including the least significant set bit are flipped, such that the least significant set bit is now unset (0) and all prior bits are set (1). All other more significant bits remain unchanged. This way, ANDing both causes the previously least significant set bit to fail.
