+++
title = "Two Sum Of Sorted Array"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

**Problem**: Find a pair of numbers in a _sorted_ array that sum up to some target.

**Approach 1 - Brute force**: For each number, check all other numbers.

**Approach 2 - Iteratively adjust end-points**: Nudge the end-points based on the direction of the target sum from the sum of the end-points.
