+++
title = "Maximum Subarray"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Find the sum of the contiguous subarray with the largest sum.

**Source**: https://leetcode.com/problems/maximum-subarray/description/

**Approach**: [Kadane's algorithm](https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm).

Scan the array. For each number, determine which is greater: adding it to the existing sum or keeping it on its own. Then determine which is greater, the previous result or the previous maximum.

``` python
def maxSubArray(nums):
    if not nums: return 0
    
    current = nums[0]
    maximum = nums[0]
    
    for i in nums[1:]:
        current = max(i, i + current)
        maximum = max(maximum, current)

    return maximum
```
