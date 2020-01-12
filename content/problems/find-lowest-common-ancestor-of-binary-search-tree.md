+++
title = "Find Lowest Common Ancestor of Binary Search Tree"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Find the lowest common ancestor (LCA) of two nodes in a binary search tree.

**Approach**: Descend the tree until a root is found where the nodes are found in separate branches _based on_ the BST invariant conditions.

``` python
while True:
  if root.value > left.value and root.value > right.value:
    root = root.left
  elif root.value < left.value and root.value < right.value:
    root = root.right
  else:
    return root
```
