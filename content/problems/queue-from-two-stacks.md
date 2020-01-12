+++
title = "Queue From Two Stacks"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["stacks"]
+++

**Problem**: Implement a queue using two stacks.

**Approach**: Maintain two separate stacks for enqueuing and dequeueing.

``` python
def enqueue(self, x):
    self.front.push(x)

def pop(self):
  if self.back.empty():
    self.back.push_all(self.front.pop_all())
    
  return self.back.pop()
```
