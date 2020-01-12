+++
title = "Square Root"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["math"]
+++

The square root of a number can be found via binary search by searching from `$[1.0, n]$` for a number which, squared, is equal to the input (given some tolerance or until convergence). If the input is real and less than 1, the bounds change to `$[n, 1.0]$`.
