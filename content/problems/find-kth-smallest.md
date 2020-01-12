+++
title = "Find Kth Smallest"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

|Case    |Growth|
|:-----  |:--------|
|Average |`$\Theta(n)$`|

Selecting the `$k^\text{th}$` smallest item in a sequence can be accomplished by using QuickSort's partition algorithm. This is guaranteed by the invariant held by QuickSort's partition algorithm which states that given the partition index `$j$`, all elements to the left are less than or equal to `$j$` and all elements to the right are greater than or equal to `$j$`, effectively making the sub-sequence up to `$j$` consist of the smallest `$j$` elements in the sequence.

With that in mind, the desired index `$k$` is input to QuickSelect. After partitioning in `$O(n)$`, the resulting position `$j$` of the `$k^\text{th}$` element is compared to the input `$k$`. If the resulting position `$j$` is less than the desired `$k$` then QuickSelect is repeated on the right region `$A[j ..]$` with a compensated `$k$`, i.e. `$k - j$`. If the resulting position `$j$` is greater than the desired `$k$` then QuickSelect is repeated on the left region `$A[.. j]$` with the same `$k$`.

``` cpp
template <typename Iter>
auto QuickSelect(Iter begin, Iter end, std::size_t i) -> decltype(*begin) {
  std::size_t length = std::distance(begin, end);

  if (length == 1) {
    return *begin;
  }

  Iter pivot = Partition(begin, end);

  std::size_t distance = std::distance(begin, pivot);

  if (distance == i) {
    return *pivot;
  } else if (i > distance) {
    // -1 to shift past pivot
    return QuickSelect(std::next(pivot), end, i - distance - 1);
  } else {
    return QuickSelect(begin, pivot, i);
  }
}
```

``` python
def quick_select(items, i):
    if len(items) == 1: return items[0]

    pivot = partition(items)

    if pivot == i:
        return items[i]
    elif i > pivot:
        # -1 to shift past the pivot
        return quick_select(items[pivot + 1 :], i - pivot - 1)
    else:
        return quick_select(items[: pivot], i)
```
