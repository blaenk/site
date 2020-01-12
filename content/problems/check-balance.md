+++
title = "Check Balance"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Determine if a tree is balanced, such that the height of sub-trees don't differ by more than one.

**Approach**: Write a function to compute the height of a tree: the max of the height of the left and write sub-trees plus one for the root. Then use that function to determine that a tree is balanced if the absolute difference of the height of each sub-tree is less than or equal to 1 _and_ both sub-trees are also balanced.

**Optimization**: The height can be a parameter of `is_balanced()` and each recursive call can write to it so that the caller can access the height without having to recurse down again by calling `height()`.

``` python
def height(tree):
    return max(height(tree.left), height(tree.right)) + 1

def is_balanced(tree):
    return abs(height(tree.left) - height(tree.right)) <= 1 and
           is_balanced(tree.left) and
           is_balanced(tree.right)
```
