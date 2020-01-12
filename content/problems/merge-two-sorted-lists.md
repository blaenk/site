+++
title = "Merge Two Sorted Lists"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["linked-lists"]
+++

**Problem**: Given two sorted lists, merge them into a single sorted list.

**Source**: https://leetcode.com/problems/merge-two-sorted-lists

**Approach**: Set the result's next node to the lesser first node of the two lists as long as both lists have nodes remaining. Then set the result's next node to the list that still has remaining nodes.

``` python
def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
    merged = ListNode(-1)
    head = merged

    while l1 is not None or l2 is not None:
        both = l1 is not None and l2 is not None

        if l2 is None or (both and l1.val <= l2.val):
            merged.next = l1
            l1 = l1.next
        else:
            merged.next = l2
            l2 = l2.next

        merged = merged.next

    return head.next
```

![](/images/problems/merge-two-sorted-lists.png)