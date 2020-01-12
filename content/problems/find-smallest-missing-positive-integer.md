+++
title = "Find Smallest Missing Positive Integer"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Find the smallest missing positive integer in an array.

**Approach 1 - Set (Speed)**: Push all numbers into a set, then increment from 1 until the first missing number is found.

**Approach 2 - Sort (Space)**: Sort the array, then iterate until the first missing number is found.
