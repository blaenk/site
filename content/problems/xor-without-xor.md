+++
title = "XOR without XOR"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["bitwise"]
+++

XOR can be replicated without actually using XOR by simply writing out what XOR means, i.e. exclusive OR: (a AND NOT b) OR (NOT a AND b)

``` python
# emulate XOR
x ^ y == (x & ~y) | (~x & y)
```
