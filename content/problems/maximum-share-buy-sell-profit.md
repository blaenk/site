+++
title = "Maximum Share Buy/Sell Profit"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Determine the maximum profit that can be obtained over a series of days.

**Source**: _Elements of Programming Interviews_ p. 1, Introduction

**Questions**:

* **What is the input format?**
    * Three arrays to each hold the low, high, and starting prices for each day.
* **Can the purchase and sale on a given day be made with any of these three values?**
    * No, they must be made with the starting prices for the day.

**Analysis**:

* This seems similar to the [Maximum Subarray](#maximum-subarray) problem.
* We can ignore all inputs beside the starting prices, since they only seem to serve to distract.

---

**Approach 1 - Brute Force**: Consider all pairs, returning the one with the highest profit.

{{< complexity time="n^2" space="1" />}}

---

**Approach 2 - Divide-and-Conquer**: Recursively split the array in half and find the best profit pair in each half. In the "combine" step, be careful to account for the case where the buy occurs in the left half and the sell occurs in the right half. In this case, the optimum buy would be the minimum price in the left half, and the optimum sell would be the maximum price in the right half. This makes sense because it's necessary to obtain the higher possible profit.

The recurrence relation is:

<div>$$T(n) = 2T\frac n 2 + O(n)$$</div>

{{< complexity time="n \log n" space="1" />}}

---

**Approach 3 - Linear Scan**: Recognize that the maximum profit that can be made is tied to the minimum buy price, and a sale has to occur at some point. Go through each day keeping track of the lowest buy price seen. If the difference between the current element (i.e. the prospective "sell") and the current minimum (i.e. the cheapest "buy") is greater than the maximum profit recorded, save it as the new maximum profit.
