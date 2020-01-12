+++
title = "In-Place Removal"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Remove all elements of a given value in-place and return the new length of the array.

**Approach**: Partition the array into a filtered region and a removed region. Scan a forward index looking for newly-seen numbers, then write them at the end of the left region denoted by the back index.

``` python
def remove(nums, n):
  if not nums: return 0
  
  i = 1
  j = 1

  while j < len(nums):
    if nums[i - 1] != nums[j]:
      nums[i] = nums[j]
      i += 1

    j += 1

  return i
```
