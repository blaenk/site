+++
title = "Find Missing Number From Two Arrays"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Given two arrays, determine the number that is missing from the other. There is no guaranteed order, that is, the set difference between the larger array and the smaller array.

**Source**: https://interviewing.io/recordings/Python-Airbnb-1/

**Approach 1 - Sorting**: Sort both arrays, then pairwise iterate through both to find the mismatch. The missing number will be the number being compared from the larger array.

{{< complexity time="n \log n" space="1" />}}

**Approach 2 - Sets**: Add the elements of the smaller array into a set, then determine which element of the larger array is not in the set.

| Case | Order |
| :---- | :---- |
| Time | `$O(n)$` |
| Space | `$O(n)$` |

**Approach 3 - Summing**: Add all of the elements in the larger array and subtract from that sum the elements from the smaller array. The total _must_ be stored in some kind of BigNum type or the overflow and underflow (if there are negative numbers) would corrupt the result.

The space complexity is proportional to the number of bits needed to store the total in the BigNum.

{{< complexity time="n" space="\log n" />}}

**Approach 4 - XOR**: Given XOR's property where the same numbers cancel each other out, `$A \oplus A = 0$`, we can XOR every number in both arrays together to arrive at the missing number, since each pair will cancel itself out, `$A \oplus A \oplus B = B$`.

| Case | Order |
| :---- | :---- |
| Time | `$O(n)$` |
| Space | `$O(1)$` |
