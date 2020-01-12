+++
title = "Find First Bad Version"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["search"]
+++

**Problem**: Given a version number and a function to see if it's a bad version, find the first bad version number, similar to `git bisect`.

**Approach**: Perform a binary search on the versions.

``` python
def first_bad_version(n):
  return helper(1, n)

def helper(start, end):
  if start >= end: return start

  middle = start + (end - start) // 2

  if is_bad_version(middle):
    return helper(start, middle)
  else:
    return helper(middle + 1, end)
```
