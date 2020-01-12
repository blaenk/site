+++
title = "Remove Duplicates"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
tags = ["slow-fast-iteration"]
+++

**Problem**: Remove duplicates from a linked list without using additional storage.

**Approach**: Traverse the list and for each node, traverse the rest of the list to find duplicates of that node.

``` python
while node:
    previous = node
    duplicate_finder = node.next

    while duplicate_finder:
        if node.value == duplicate_finder.value:
            previous.next = duplicate_finder.next
            duplicate_finder = previous.next
        else:
            previous = duplicate_finder
            duplicate_finder = duplicate_finder.next

    node = node.next
```
