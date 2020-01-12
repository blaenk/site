+++
title = "Remove Given Node"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Remove a node from a singly-linked list, given that to-be-removed node.

**Approach**: Absorb the value of the successor, then link past the successor.

``` python
def remove_node(node):
    if node and node.next:
        node.value = node.next.value
        node.next = node.next.next
```
