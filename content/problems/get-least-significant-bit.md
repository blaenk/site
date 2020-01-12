+++
title = "Get Least Significant Bit"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["bitwise"]
+++

The least significant set bit can be obtained with:

``` python
x = 0101 0100
least_significant_set_bit = x & ~(x - 1)
```

The way this works is:

1. all bits up-to-and-including the least significant set bit are flipped

    ``` python
    (x - 1)  = 0101 0011
                     →
    ```

2. flip _all_ bits so that only the previously-least significant set bit will pass when ANDed with the original value

    ``` python
    ~(x - 1) = 1010 1100
    ```

3. AND with the original value so that only the least significant set bit passes

    ``` python
    x & ~(x - 1) = 0000 0100
    ```
