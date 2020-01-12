+++
title = "Remove Node Given Predecessor"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Remove a node from a singly-linked list, given its predecessor node.

**Approach**: Re-link the predecessor node to the target node's successor, i.e. link _past_ the to-be-removed node.

``` python
def remove_node(predecessor):
    if predecessor and predecessor.next:
        predecessor.next = predecessor.next.next
```
