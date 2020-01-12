+++
title = "Insert Bits At Position"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

**Problem**: Insert the bits of M into N at position [i, j).

**Approach**: Use a mask of ones to pass everything from M _except_ slice [i, j), then bitwise OR that with `M << i`.

``` python
def insert_bits(n, m, i, j):
    left = ~0 << (j + 1)
    right = (1 << i) - 1
    region_mask = left | right
    cleared_region = n & ignore_region_mask

    shifted_into_position = m << i

    return cleared_region | shifted_into_position
```
