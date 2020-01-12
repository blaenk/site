+++
title = "Generate All Unique Pairs"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["combinatorial"]
+++

**Problem**: Compute all of the unique pairs of numbers where each number can be a max of `$N$`.

**Approach**: A nested loop where the second component starts after the first.

``` python
def generate_unique_pairs(n):
    """
    Generate all unique combinations of paris (a, b) such that a and b
    can be any number from [0, n).
    """
    for i in range(n):
        for j in range(i + 1, n):
            yield (i, j)
```
