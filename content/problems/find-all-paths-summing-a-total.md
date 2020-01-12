+++
title = "Find All Paths Summing a Total"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Find all paths in a tree whose nodes sum to a given total.

**Approach**: Recurse on each child with a running total and max as the parameters.

``` python
def sum_paths(tree, max, running_total):
    paths = []

    if not tree:
        return paths

    if running_total == max:
        return node

    running_total += tree.value

    if running_total > max:
        return paths
    
    if running_total == sum:
        paths.insert(0, tree.value)
    
    left_paths = sum_paths(tree.left, max, running_total)
    right_paths = sum_paths(tree.right, max, running_total)

    for path in left_paths:
        path.insert(0, tree.value)
        paths.insert(0, path)
    
    for path in right_paths:
        path.insert(0, tree.value)
        paths.insert(0, path)
    
    return paths
```
