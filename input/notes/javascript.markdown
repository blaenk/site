---
title = "JavaScript"
published = "March 7, 2018"
excerpt = "The JavaScript Programming Language"
comments = false
---

<toc />

Identifying arrays isn't always possible to do with `instanceof`. For example, if an array is passed between two browser frames, each page has its own global context, so `instanceof` won't work because the array will be an instance of the other frame's `Array`. Instead, `Array.isArray()` should be used.
