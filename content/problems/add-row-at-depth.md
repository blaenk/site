+++
title = "Add Row At Depth"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Given a binary tree, a depth, and a number, modify the tree so that nodes with the given value populate the level at the specified depth.

**Source**: https://leetcode.com/problems/add-one-row-to-tree/

**Approach 1 - Breadth-first Search**: 

``` python
from collections import deque

def addOneRow(self, root, v, d):
    if d == 1:
        new_root = TreeNode(v)
        new_root.left = root
        return new_root
    
    frontier = deque()
    frontier.append((1, root))
    
    while frontier:
        level, node = frontier.popleft()
        
        if not node: continue
        
        # We're at the level to reattach children
        if level + 1 == d:
            new_left = TreeNode(v)
            new_left.left = node.left
            node.left = new_left

            new_right = TreeNode(v)
            new_right.right = node.right
            node.right = new_right
        else:
            frontier.append((level + 1, node.left))
            frontier.append((level + 1, node.right))
    
    return root
```

**Approach 2 - Depth-first Search**: 

``` python
from collections import deque

def helper(root, v, d, level=1):
    if not root:
        return root
    
    if d == 1:
        new_root = TreeNode(v)
        new_root.left = root
        
        return new_root
    
    if level + 1 == d:
        new_left = TreeNode(v)
        new_left.left = root.left
        root.left = new_left
        
        new_right = TreeNode(v)
        new_right.right = root.right
        root.right = new_right
    else:
        root.left = helper(root.left, v, d, level + 1)
        root.right = helper(root.right, v, d, level + 1)
    
    return root

def addOneRow(root, v, d):
    return helper(root, v, d)
```
