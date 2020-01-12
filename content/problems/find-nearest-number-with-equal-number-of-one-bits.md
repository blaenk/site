+++
title = "Find Nearest Number With Equal Number of One Bits"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

**Problem**: Given a number with a certain bit population count (i.e. count of ones), find the nearest number with the same bit population count.

**Insight**: Avoid measuring the distance by searching in the direction that naturally increases the distance.

**Approach 1 - Iterative**: Iteratively increment in both directions away from the input number to find the first number with the same population count.

``` python
def closest_integer(n):
    if n == 0: return 0

    w = weight(n)

    up = n + 1
    down = n - 1

    while True:
        if weight(up) == w:
            return up
        elif down != 0 && weight(down) == w:
            return down
        
        if down != 0:
            down -= 1
        
        up += 1
    
    return n
```

**Approach 2 - Flipping right-most differing bits**:

1. Iterate consecutive bit positions from the right to find the first differing bits.
2. Create a mask of the two bit positions
3. XOR the input number with the mask to flip those bits, preserving the population count.

``` python
def closest_integer(n):
    for i in range(64):
        left_bit = (n >> i) & 1
        right_bit = (n >> (i + 1)) & 1

        if left_bit != right_bit:
            mask = (1 << i) | (1 << (i + 1))

            return n ^ mask
        
    return n
```
