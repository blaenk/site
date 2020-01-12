+++
title = "Find Beginning of Cycle"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Find the beginning of a cycle.

**Approach**:

1. Detect a cycle
2. When the nodes meet, reset the slow pointer to the start of the list
3. When the pointers meet again, it will be at the start of the cycle.
