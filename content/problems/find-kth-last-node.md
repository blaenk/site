+++
title = "Find kth Last Node"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Find the k-th last node in a singly-linked list.

**Approach 1 - Single traversal, using two pointers**:

1. Advance the forward pointer `$k$` nodes ahead
2. Advance both pointers one node at a time until the forward pointer reaches the end
3. The behind pointer will be `$k$` nodes away from the end

**Approach 2 - More than one traversal, counting**:

1. One traversal to count the nodes `$n$`
2. Another traversal up to node `$n - k + 1$`
