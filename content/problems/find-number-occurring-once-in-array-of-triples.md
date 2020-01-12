+++
title = "Find Number Occurring Once In Array Of Triples"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["bitwise"]
+++

Given an array of numbers, each of which appears exactly three times _except_ for one number, the number that appears only once can be obtained using a combination of the above operations.

There will be two sets. The `ones` set will contain those numbers that have been observed to appear once so far. The `twos` set will contain those numbers that have been observed to appear twice so far. It's not necessary to track those numbers that appear three times, as that is implicit.

For each number in the array, it will be added to the `ones` set if it has been seen exactly once. If a number is already in the `ones` set and is seen again, it's removed from the `ones` set and added to the `twos` set. If a number is already in the `twos` set and is seen again, it's removed from the `twos` set. Ultimately this will leave the `ones` set as a singleton set containing the number that appears only once.

First, the number being considered is checked to see if it has already been seen once so far:

```
ones & num
```

This would mean that this is the second time that this number has been seen. If the number is indeed in the `ones` set, the above expression evaluates to the number itself by nature of the AND operation, otherwise it evaluates to 0.

The result of the above expression can directly be used to add the number to the `twos` set using the OR operation, which adds the number to the set, or does nothing if the number is 0.

```
twos |= ones & num
```

The number is then _conditionally_ added to or removed from the `ones` set using the XOR operation. If the `ones` set already contained the number, then the previous operation will have added the number to the `twos` set, in which case it's no longer needed inside the `ones` set, since the `ones` set only contains those numbers that have appeared _exactly_ once. If the `ones` set didn't already contain the number, then the above presence check results in 0 which does nothing when added to the `twos` set.

Therefore, we can add the number to the `ones` set if it wasn't already contained, or remove it if it was contained.

```
ones ^= num
```

However, if this has been the _third_ time that this number has been seen, then the number will have been in the `twos` set already but _not_ in the `ones` set. The above expression would therefore _also_ add it to the `ones` set. Therefore we can be sure that if the number was present in both the `ones` and `twos` sets after the above operation, then this is the _third_ time that this number has been observed.

In this case, the number should be removed from both sets. This would ensure that the `ones` set really does only contain those numbers that have appeared _exactly_ once, which is what we're interested in.

However, the `ones` set and the `twos` set may contain various numbers. We know that the one that appears in both is the one that has appeared three times, and hence the one we want to remove, so we can obtain this number by intersecting both sets.

```
int threes = twos & ones;
```

We then want to remove this number from both sets:

```
twos &= ~threes;
ones &= ~threes;
```

Here is the full algorithm.

``` python
def single_number(nums):
  ones = twos = 0

  for num in nums:
    twos |= ones & num
    ones ^= num

    threes = twos & ones
    twos &= ~threes
    ones &= ~threes

  return ones
```
