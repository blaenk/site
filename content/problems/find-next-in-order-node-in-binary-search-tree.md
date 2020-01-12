+++
title = "Find Next In-Order Node in Binary Search Tree"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Find the next in-order node in a binary search tree.

**Approach**:

1. If the right sub-tree isn't empty, then return left-most node

    Else find the parent that has the traversed node as it's left child.

``` python
def next_in_order(node):
    if node.right:
        node = node.right

        while node.left:
            node = node.left
        
        return node
    else:
        while node.parent:
            if node == node.parent.left:
                return node
            else:
                node = node.parent
    
    return None
```
