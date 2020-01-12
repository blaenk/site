+++
title = "Exponentiation"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

**Problem**: Implement integer exponentiation of `$x^y$`.

**Approach 1 - Iterative multiplication**: Multiple `$x$` by itself `$y$` times.

**Approach 2 - Iterative squaring**:

1. While `$y \gt 0$`:

    1. If `$y$` is odd:

        1. Break a term of `$x$` off, e.g. `$x^3 \rightarrow x^2 * x$`
        2. Multiply term into the result
    
    2. Square the base `$x$`
    4. Halve the exponent `$y$`

``` python
def exponent(base, power):
    result = 1.0

    if power == 0: return 1

    if power < 0:
        power = -power
        base = 1.0 / base
    
    while power:
        if power & 1:
            result *= base
        
        base *= base
        power >>= 1
    
    return result
```
