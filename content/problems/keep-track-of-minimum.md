+++
title = "Keep Track of Minimum"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["stacks"]
+++

**Problem**: Implement a stack that keeps track of the smallest value within the stack at any given moment.

**Approach**: Maintain an internal stack to keep track of the minimums.

When pushing a value, if it's equal to or less than the old minimum (check top of minimums stack), push it onto the minimums stack.

When popping a value, if it's equal to the current minimum (check the top of minimums stack), pop the minimums stack.

``` python
def push(self, x):
  self.stack.push(x)

  if self.stack.top() <= self.min.top():
    self.min.push(x)

def pop(self, x):
  if self.stack.top() == self.min.top():
    self.min.pop()

  return self.stack.pop()
```
