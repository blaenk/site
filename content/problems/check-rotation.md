+++
title = "Check Rotation"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["strings"]
+++

**Problem**: Determine if an input string is the rotate form of the given text.

**Approach**: Concatenate the string with itself, then search within it.

``` python
def is_rotated(text, input):
    "Determine if `input` is a rotation of `text`."
    return input in (text + text)
```
