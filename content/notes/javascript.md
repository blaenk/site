+++
title = "JavaScript"
date = 2018-03-07

[note]
kind = "language"
+++

<nav id="toc"></nav>

Identifying arrays isn't always possible to do with `instanceof`. For example, if an array is passed between two browser frames, each page has its own global context, so `instanceof` won't work because the array will be an instance of the other frame's `Array`. Instead, `Array.isArray()` should be used.
