+++
title = "Convert Sorted Array To BST"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["trees"]
+++

**Problem**: Create a balanced binary search tree given a sorted array in ascending order.

**Source**: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree

**Approach 1 - Recursive**: Create a root node from the midpoint, attach the children by recursing the left and right sub-regions, then return the root node.

``` python
def sortedArrayToBST(self, nums):
    if not nums: return None

    mid = len(nums) // 2
    node = TreeNode(nums[mid])

    node.left = sortedArrayToBST(nums[:mid])
    node.right = sortedArrayToBST(nums[mid+1:])

    return node
```
