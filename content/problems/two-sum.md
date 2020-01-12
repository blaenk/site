+++
title = "Two Sum"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Find all pair of numbers in an array that sum up to some target.

**Source**: https://leetcode.com/problems/two-sum

**Approach 1 - Brute force**: For each number, check all other numbers.

**Approach 2 - Set of complements**: For each number, determine the complement: number that would need to exist to sum up to the target, `$t - n$`. If the complement is in the set then that number exists in the array, otherwise add the _number_ to the set.

---

The 2-SUM problem is one where the input is a sorted array of integers and a target value `$k$` and the objective is to determine if there is a pair of entries `$i$` and `$j$` such that:

<div>$$A[i] + A[j] = k$$</div>

The brute-force approach would be to check if this is true for all possible pairs of values in the array, of which there are `$n^2$`. Alternatively, a [hash table](#hash-tables) can be used.

Another approach is to iterate from both ends of the array. For example, at first, the first and last elements will be used. If their sum is greater than the target, the right end will be iterated leftward so as to attempt to obtain a smaller element. If instead their sum is less than the target, the left end will be iterated rightward in hopes of increasing their sum.

``` python
while left < right:
  sum = A[i] + A[j]

  if sum > k:
    right -= 1
  elif sum < K:
    left +- 1
  else:
    return true
```

![](/images/problems/two-sum.png)