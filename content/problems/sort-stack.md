+++
title = "Sort Stack"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["stacks"]
+++

**Problem**: Sort a stack.

**Solution**: Use intermediary stacks

Continuously pop from the input stack onto the sorted stack as long as the newly pushed value is still in-order (i.e. it's less than the top of the sorted stack).

If the pushed value would be out-of-order because it's larger than the top of the sorted stack, then continuously pop from the sorted stack onto the input stack the region/values that would be out-of-order, _then_ push the value onto the sorted stack.

Then continue popping from input to sorted.

``` python
while input:
  top = input.pop()

  while top < sorted.top():
    input.push(sorted.pop())
    
  sorted.push(top)
```
