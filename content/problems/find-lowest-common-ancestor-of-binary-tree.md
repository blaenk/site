+++
title = "Find Lowest Common Ancestor of Binary Tree"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Find the lowest common ancestor (LCA) of two nodes in a binary tree.

**Approach**: Recursively look for the two nodes in both branches. If they're found in separate branches, then the current node must be the LCA.

``` python
def lowest_common_ancestor(root, left, right):
    if not root: return None

    if root == left or root == right:  return root

    else:
    left_lca = lowest_common_ancestor(root.left, left, right)
    right_lca = lowest_common_ancestor(root.right, left, right)

    if left_lca and right_lca: return root

    return left_lca if left_lca else right_rca
```
