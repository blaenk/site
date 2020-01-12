+++
title = "Check Sub-Tree Containment"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Determine if one sub-tree is a sub-tree of another.

**Approach**: Write a function that determines if two trees are equal, recursively. Then use that function to recurse down the tree to see if the sub-tree is present rooted at any given node.

``` python
def matches(a, b):
  if not a and not b: return True
  if not a or not b: return False

  if a.value != b.value: return False

  return matches(a.left, b.left) and matches(a.right, b.right)

def contains(tree, sub):
  if not tree or not sub: return False

  if tree.value == sub.value:
    return matches(tree, sub)
  else:
    return contains(tree.left, sub) or contains(tree.right, sub)
```
