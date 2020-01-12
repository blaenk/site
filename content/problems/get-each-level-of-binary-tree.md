+++
title = "Get Each Level of Binary Tree"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Get each level of a binary tree as a list of lists.

**Approach**: BFS

1. Collect each level into a list
2. Push it onto the levels list
3. Enqueue each level's node's children

``` python
levels = []
next = []

next.push(root)

while next:
  current = next
  next = []
  level = []

  for node in current:
    level.push(node)
    next.push_all(child.children)

  levels.push(level)
```
