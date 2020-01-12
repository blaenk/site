+++
title = "Count Order Inversions"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Given an array, find the number of _order inversions_ within it. An order inversion is when a number earlier in the sequence is larger than later smaller numbers, in which case there is an order inversion for each later number that it is greater than. For example, given the array [2, 4, 1, 3, 5], the number 2 is larger than the 1 that comes after it and the number 4 is larger than the numbers 1 and 3 that come after it, hence three order inversions.

---

**Approach 1 - Brute Force**: For each number, count each later number that it's greater than.

{{< complexity time="n^2" space="1" />}}

``` python
def count_inversions(seq):
    inversions = 0

    for i in range(len(seq)):
        for j in range(i + 1, len(seq)):
            if seq[i] > seq[j]:
                inversions += 1

    return inversions
```

---

**Approach 2 - Divide and Conquer (Merge Sort)**: The number of _inversions_ in an array can be counted in `$O(n \log n)$` by reducing the problem to merge sort. Specifically, during a merge, each time an element from the right half is merged and there are elements remaining in the left half, then the chosen element from the right half represents an inversion between each of the elements remaining on the left half.

{{< complexity time="n \log n" space="n" />}}

``` cpp
left: [1, 2, 12, 13] right: [6, 8, 9, 10]

// after merging two from the left
merged: [1, 2, _, _, _, _, _, _]

left: [12, 13] right: [6, 8, 9, 10]

// the ones on the right are lesser, e.g. merge 6
merged: [1, 2, 6, _, _, _, _, _]

left: [12, 13] right: [8, 9, 10]

// the act of having merged 6 instead of [12, 13] means that
// the original array had two inversions concerning 6: 12⟷6 and 13⟷6
```

A possible implementation would have merge return the inversions it encountered, which has to be passed up the recursion tree by having the sort functions return the sum of the recursive sorts and merges.

``` python
def sort_and_count(seq, aux, lo, hi):
    if (hi - lo) <= 1: return 0

    mid = lo + ((hi - lo) // 2)

    left = sort_and_count(seq, aux, lo, mid)
    right = sort_and_count(seq, aux, mid, hi)
    combined = merge_and_count(seq, aux, lo, mid, hi)

    return left + right + combined

def merge_and_count(seq, aux, lo, mid, hi):
    for i in range(lo, hi):
        aux[i] = seq[i]

    left = lo
    right = mid
    i = lo
    inversions = 0

    while left < mid or right < hi:
        if right == hi:
            seq[i] = aux[left]
            left += 1
        elif left == mid:
            seq[i] = aux[right]
            right += 1
        elif aux[left] < aux[right]:
            seq[i] = aux[left]
            left += 1
        else: # Inversion: Taking from the right while left is not yet empty
            # Inversion for every remaining element on the left
            inversions += (mid - left)
            seq[i] = aux[right]
            right += 1

        i += 1

    return inversions

def count_inversions(seq):
    aux = seq[:]

    return sort_and_count(seq, aux, 0, len(seq))

def test_count_inversions():
    assert count_inversions([2, 4, 1, 3, 5]) == 3
    assert count_inversions([1, 20, 6, 4, 5]) == 5
```
