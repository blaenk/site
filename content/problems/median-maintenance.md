+++
title = "Median Maintenance"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["heaps"]
+++

The median of a streaming sequence of numbers can be computed in constant time by maintaining two heaps: a max-heap for the lower/left half and a min-heap for the upper/right half. This has the effect of keeping the elements sorted, and the top of the max-heap yields one of the overall middle elements, whereas the top of the min-heap yields the other middle element.

The elements must be kept equal in size, or the min-heap may be larger by one element, in which case there are an odd number of numbers, so the top of the min-heap is the middle element alone.

If one of the heaps grows larger, its top element should be popped and pushed onto the other heap to balance them.
