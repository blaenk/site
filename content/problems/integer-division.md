+++
title = "Integer Division"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

**Problem**: Implement the integer division of `$\frac x y$`.

**Approach 1 - Iterative subtraction**: Continuously subtract `$y$` from `$x$` until it is no longer possible.

**Approach 2 - Subtract in large chunks**:

1. While `$x \ge y$`

    1. Find the largest `$k$` such that `$2^k y \le x$`
    2. Subtract `$2^k y$` from `$x$`
    3. Add `$2^k y$` to the resulting quotient

``` python
def divide(x, y):
    result = 0

    while x >= y:
        exp = 1

        # Find largest 2^k such that 2^k * y <= x
        while (exp << 1) * y <= x:
            exp <<= 1

        x -= exp * y
        result += exp

    return result
```
