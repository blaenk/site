+++
title = "Detect Palindrome"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Determine if a singly-linked list is a palindrome with a single traversal.

**Approach**: Use a slow pointer and a fast pointer that moves two nodes at a time.

1. Push each node reached by the slow pointer onto a stack
2. After the fast pointer reaches the end, pop and compare for each node reached by the slow pointer

**Note**: If it's an odd-sized list, the slow pointer needs to skip the midpoint. Check if the fast pointer has reached the end by checking if `next` or `next.next` is null. If `next.next`, then skip the midpoint.
