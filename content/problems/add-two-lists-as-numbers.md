+++
title = "Add Two Lists As Numbers"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: A number is represented in a linked list with its digits represented as nodes in reverse. The number 123 is stored as 3 → 2 → 1. Given two such numbers, add them together and express the result as a linked list as well.

**Approach 1 - Digit-wise Addition**: Add each corresponding digit in lock-step, keeping track of carries.

**Approach 2 - Convert to and from numbers**: Convert a linked list to a number, add, then convert back to a linked list.
