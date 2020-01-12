+++
title = "Bit Parity"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["bitwise"]
+++

The parity can be computed by continuously right-shifting and ANDing the shifted bit with 1 and adding the result to the running parity. However, only the 1 bits will have an effect on the parity, so a quicker way is to continuously be accessing and turning off the next lowest set bit. Finally, lookup tables of e.g. 16-bit chunks can be precomputed so that on a given input, the parity for each 16-bit chunk of the input is looked up and added together.
