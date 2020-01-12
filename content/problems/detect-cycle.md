+++
title = "Detect Cycle"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Detect a cycle.

**Approach**: Traverse the list with a slow and fast pointer. The fast pointer moves two nodes at a time. If there is a cycle, then the two nodes will eventually meet.
