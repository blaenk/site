+++
title = "Find Maximum Sum Tree"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Given a tree with an arbitrary number of children, find the sub-tree with the largest tree sum, where the tree sum is the sum of the values in all of the nodes in the tree.

**Source**: Technical phone interview

**Approach**: Define a function to recursively compute a tree's sum, preferably cache this value when it is computed. Then define a function to recursively return the node with the maximum tree sum.

``` python
class Node:
    def __init__(self, value):
        self.value = value
        self.parent = None
        self.children = []
        self._sum = None

    def attach(self, node):
        self.children.append(node)

        return node

    def sum(self):
        if not self.children:
            return self.value

        if self._sum is not None:
            return self._sum

        children_sum = 0

        for child in self.children:
            children_sum += child.sum()
        
        total = self.value + children_sum

        if self._sum is None:
            self._sum = total
        
        return total

    def max_sum_tree(self):
        if not self.children:
            return self
        
        max_tree = self

        for child in self.children:
            max_child = child.max_sum_tree()

            if max_child.sum() > max_tree.sum():
                max_tree = max_child
        
        return max_tree
```
