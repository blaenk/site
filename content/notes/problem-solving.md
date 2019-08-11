+++
title = "Problem Solving"
date = 2018-10-10T21:14:47-07:00
draft = true

[note]
kind = "concept"
+++

These are notes on the general problem solving techniques through the application of [algorithms](/notes/algorithms) and [data structures](/notes/data-structures).

<nav id="toc"></nav>

# Process

## Algorithms

1. Preliminaries

    1. <span class="highlight">Clarify</span> and <span class="highlight">summarize</span> the problem

        * Distill requirements
        * <span class="highlight">Inputs</span>: types, structure (sorted?), size, range
        * <span class="highlight">Outputs</span>: types, error handling
        * Compare the inputs to the outputs

    2. Design and confirm <span class="highlight">simple test cases</span> (<span class="highlight">expected</span>, <span class="highlight">edge</span>, and <span class="highlight">failure</span> inputs)
    
        * Especially if the provided test cases are overwhelming

    3. Design the <span class="highlight">brute-force</span> approach (complexity)
    4. Design <span class="highlight">alternatives</span> and prioritize them for implementation
    
        * Optimizing for <span class="highlight">space</span> or <span class="highlight">speed</span>?
    
    5. <span class="highlight">Outline</span> approaches with higher-level functions
    6. <span class="highlight">Trace</span> through approach with test cases

2. Implementation

    1. Implement at a high-level, top-down. Focus on the algorithm, fill in the rest later
    2. <span class="highlight">Trace</span> through implementation with test cases
    3. Complexity
    4. Optimizations

3. Reflect

    1. Approaches taken
    2. Identify patterns and tricks
    3. Lessons learned

        * Pitfalls, misunderstandings, mistakes, overthinking

    4. Further optimizations

4. Retain

    1. Add an entry here for:
        1. The problem
        2. [Tricks](#tricks)
        3. [Patterns](#patterns)
        4. [Best Practices](#best-practices)
        5. [Pitfalls](#pitfalls)
        6. [Modeling](#modeling)

## System Design

1. Determine the <span class="highlight">scope</span> by asking clarifying questions

    * Use cases
    * Constraints

2. Make <span class="highlight">reasonable assumptions</span>

    * <span class="highlight">Capacity estimation</span>
    * Load parameters
    * Percentiles
    * How much traffic is being served
    * How much data needs to be stored
    * Data consistency requirements

3. Draw <span class="highlight">major components</span> on the whiteboard

    * Front-end servers, back-end servers, databases, etc.

4. Identify <span class="highlight">key issues</span>

    * Bottlenecks, challenges

5. <span class="highlight">Redesign</span> for key issues

    * Be open about limitations

Designing algorithms that scale:

1. Ask questions
2. Imagine there are no limitations

    * Infinite memory, storage

3. Redesign for real limitations

    * How to logically divide the data, look up other pieces on other machines

4. Solve the problems from step #2

Keep in mind:

* Start broad
* Lead the process
* Maintain communication
* Use the whiteboard
* Careful with assumptions
* Estimate

## Behavioral

Behavioral interviews try to:

* Assess:

    * Honesty
    * Culture fit
    * Collaborativeness
    * Communication ability
    * Long-running passion and enthusiasm

* Validate the resume
* Gauge their job offer

_"Can you tell me about a time when …"_

Use the STAR format. Be careful to focus on what _you_ did, not the group: use _I_ not _we_.

1. <span class="highlight">Situation</span>: Describe the situation.
2. <span class="highlight">Task/Target</span>: Describe the tasks/goals you were required to achieve.
3. <span class="highlight">Action</span>: Describe the actions taken to achieve the goals.
4. <span class="highlight">Result</span>: Describe the outcome. What did you learn?

# Best Practices

## Work Top-Bottom

Don't waste time on trivial aspects like finding the maximum of an array. Focus on the high-level algorithm implementation, <span class="highlight">details can be filled in later if necessary</span>.

## Employ Abstractions

As software engineers we make complex problems manageable by breaking them down into abstractions. Why throw this out during interviews? It puts you at a disadvantage. <span class="highlight">Create types and functions</span>.

Abstractions signal the acknowledgment that code exists but can be filled in later in order to focus on the meat of solving precisely what is being asked.

The cleaner code that comes from this will greatly help you as you reason about the code mid-way and later on when you seek to optimize it.

The cleaner code will also make a better impression on the interviewer.

## Codify Invariants

When a problem specifies conditions or predicates under which a solution satisfies the problem, codify those conditions in code to make it easier to reason about the solution.

## Simplify Test Cases

It's easy to get overwhelmed or confused with the test case(s) provided by the problem. Other times instead of giving many small manageable test cases, they give one large test case that exhibits each of the edge cases that will be tested for. Trying to mentally model a solution for that large, comprehensive test case can be daunting and time consuming.

<span class="highlight">Try to reduce the test case to multiple minimal test cases</span>, but be careful to ensure that you are still representing the problem. Seek confirmation if possible.

Once a solution is designed, make sure it works for the original test case and not just the minimal test cases you decomposed it to.

## Assume Valid Inputs

Don't waste time on validating every edge case of an input, but make these assumptions clear to the interviewer. An invalid input is for example a numeric string containing non-numeric characters.

## Acknowledge Pathological Inputs

Make it clear to the interviewer that you are aware of pathological inputs and can address them if required. Write unit tests for these if there is time. For example, a pathological input might be one that causes the algorithm to index an array out of bounds.

## Theoretical Solutions

Do mention theoretical approaches even if they're not eventually implemented. For example, if an algorithm would only work if a BigNum type were used, to prevent overflows, mention it. It will communicate that you're at least aware of that approach, even if you don't or can't ultimately use it.

In other words, try to lose the "not invented here" (NIH) horse-blinders often associated with algorithm puzzle solutions, where we limit our search space to what we can implement from scratch.

# Pitfalls

## Premature Optimization

Don't fall for the trap of premature optimization. It's true that solutions should preferably be optimal, but thinking about the solution _and_ making it optimal from the beginning can be overwhelming. You may end up doing both poorly, or not at all.

## "Not Invented Here"

Don't fall into the trap of assuming that everything in your solution must be written from scratch.

Instead of biasing from that direction, bias from the direction of not reinventing the wheel. If the interviewer wants you to implement something you're delegating to, you can do so, but otherwise you will have saved yourself a lot of time.

## Apparent Impossibility

If a problem seems impossible, you are very likely overthinking it. Ask for examples and clarifying questions and try to think about it a different way. Maybe you misunderstood a simple component of the problem.

On the other hand, <span class="highlight">don't confuse intractability with impossibility</span>. Even if a problem is intractable and the (possibly brute-force) solution can't run in any reasonable amount of time for large inputs, it is still a solution, and a solution is better than none.

It is also possible, maybe even likely, that there is a subtle fact about the problem that unlocks an embarrassingly efficient solution.

## Daunting Complexity

Similar to impossibility, if a problem seems overly complex or it seems like a solution would be really long, it is likely an indication that you are overthinking the problem and need to reconsider your approach. Maybe you misunderstood a simple component of the problem.

Consider that a reasonable interviewer wouldn't ask something too complex to reason about, implement, _and_ have them verify within a ~45 minute interview.

_Maybe_ the interviewer is knowingly giving you an "impossible" problem without actually expecting you to solve it, instead wanting to see how you approach the problem and handle a seemingly-impossible situation---a so-called "stress interview". I think this is unlikely, but it emphasizes the importance of establishing consensus on what the problem is and conveying your ideas for the approach before you begin and as you go.

## The Einstellung Effect

The [Einstellung effect](https://en.wikipedia.org/wiki/Einstellung_effect) refers to a situation in which when you become fixated on one misguided approach to solving a problem

In my experience, I have gravely blown up a simple geometric problem involving cells on a grid to one involving Bresenham rasterization or A* path-finding. I allowed the prestige of the company to psyche me out into believing that the company would reasonably expect a typical candidate to solve something that complicated or niche.

## The God Loop

Sometimes it's easy to let a simple loop grow into a "God Loop" that does too much, maybe as more conditions need to be handled, making the code messier and more difficult for you to reason about.

Consider if it might be easier to split up while still being the same complexity class, noting to the interviewer that the loops can be optimized by combining them.

Another option is to factor the body out to separate functions.

# Tricks

## Dummy Head

Sometimes in linked list problems we need to produce a new list from others, but we're not yet sure which node to make the head. Naively, this would entail "unrolling" the first iteration of the loop to determine the head, to then be able to attach the rest of the elements within the loop:

``` python
seq = take_from_one_of_many_lists()

while some_condition:
    if first_iteration: continue

    seq.next = take_from_one_of_many_lists()
    seq = seq.next
```

We can side-step this decision and simplify things by simply creating a "dummy" head node and then link it to the actual head. When we return the "new" list, we return `dummy.next`.

``` python
dummy = ListNode(-1)

while some_condition:
    dummy.next = take_from_one_of_many_lists()
    dummy = dummy.next

return dummy.next
```

## Sorted Sequence Merge

Sometimes we want to merge two sorted sequences, such as with merge sort. This is a pretty straightforward operation, but the loop that accomplishes this can end up with at least four branches---two for the case where one sequence is empty and two for picking the smaller element of the two non-empty sequences.

``` python
while left is not None or right is not None:
    if right is None: # Right is empty
        merged.next = left
        left = left.next
    elif left is None: # Left is empty
        merged.next = right
        right = right.next
    elif left.val <= right.val: # Left and right non-empty, left head smaller
        merged.next = left
        left = left.next
    else: # Left and right non-empty, right head smaller or equal
        merged.next = right
        right = right.next
```

This can be simplified by iterating as long as one sequence is non-empty, then taking from the left sequence if the right one is empty or if both are non-empty and the left one's head is smaller.

``` python
while left is not None or right is not None:
    both = left is not None and right is not None

    if right is None or (both and left.val <= right.val):
        merged.next = left
        left = left.next
    else:
        merged.next = right
        right = right.next
    
    merged = merged.next
```

# Patterns

## Slow-Fast Iteration

The "normal" iterator iterates one element at a time, while the "fast" iterator either starts ahead at the same pace or at a faster pace. There can also be a "slow" iterator---instead of a "fast" one---that lags behind the normal iterator.

One thing to remember is that slow and fast are relative terms. Most times one is slow or fast relative to the other, rarely are both slow and fast.

Examples:

* [Remove Duplicates](#remove-duplicates)

## Double-Ended Iteration

The "left" iterator iterates from the left-to-right of the array and the "right" iterator iterates right-to-left of the array.

Examples:

* Remove Element

# Modeling

| Model | Examples |
| :---- | :------- |
| permutations | arrangements, tours, orderings, or sequences |
| subsets | clusters, collections, committees, groups, packagings, or selections |
| trees | hierarchies, dominance relationships, ancestor/descendant relationships, or taxonomies |
| graphs | networks, circuits, webs, relationships |
| points | sites, positions, data records, locations |
| polygons | shapes, regions, configurations, boundaries |
| strings | text, characters, patterns, labels |

# Complexity

| Name         | Complexity      | Examples |
| :-------     | :-------------- | :----------------- |
| Constant     | `$O(1)$`        | Adding two numbers |
| Logarithmic  | `$O(\log n)$`   | Branching |
| Linear       | `$O(n)$`        | Scanning the input |
| Linearithmic | `$O(n \log n)$` | Branching _and_ scanning at each level |
| Quadratic    | `$O(n^2)$`      | Looking at all pairs of an input |
| Cubic        | `$O(n^3)$`      | Looking at all triples of an input |
| Exponential  | `$O(c^n)$`      | Looking at all subsets of an input |
| Factorial    | `$O(n!)$`       | Looking at all permutations/orderings of `$n$` items |

# Numbers

## Appending Digits

It's possible to "shift" a number to the left by `$n$` digits by multiplying it with `$\text {base}^\text {n digits}$`. For example, the number `$321$` can be shifted to the left by 2 digits to `$32100$` by multiplying it by `$10^2$`.

<div>$$\text {shifted left by n} = x * \text {base}^\text {n digits}$$</div>

## Get Last N Digits

Similarly, it's possible to get the last `$n$` digits of a number by getting the remainder of dividing it by `$\text {base}^\text {n digits}$`. For example, the last 2 digits of `$321$` can be obtained by getting the remainder of dividing it (i.e. the modulo) by `$10^2$`.

<div>$$\text {right-most n digits} = x \bmod \text {base}^\text {n digits}$$</div>

## Square Root

The square root of a number can be found via binary search by searching from `$[1.0, n]$` for a number which, squared, is equal to the input (given some tolerance or until convergence). If the input is real and less than 1, the bounds change to `$[n, 1.0]$`.

## Logarithm

The logarithm of a number can naively be computed by counting how many times the operand can be divided by the base without going below 1. For example, `$\log_2 64$` can divide `$64 / 2$` a total of six times until it reaches 1, doing it a seventh time causes the value to go below 1.

## Check Odd

**Problem**: Check if an integer is odd.

**Approach 1 - Bitwise**: Bitwise AND with 1. If the result is 1 then the integer is odd, otherwise even.

**Approach 2 - Remainder**: Divide by 2 and check the remainder (modulo). If the remainder is 1 then the integer is odd.

## Find Nearest Number With Equal Number of One Bits

**Problem**: Given a number with a certain bit population count (i.e. count of ones), find the nearest number with the same bit population count.

**Insight**: Avoid measuring the distance by searching in the direction that naturally increases the distance.

**Approach 1 - Iterative**: Iteratively increment in both directions away from the input number to find the first number with the same population count.

``` python
def closest_integer(n):
    if n == 0: return 0

    w = weight(n)

    up = n + 1
    down = n - 1

    while True:
        if weight(up) == w:
            return up
        elif down != 0 && weight(down) == w:
            return down
        
        if down != 0:
            down -= 1
        
        up += 1
    
    return n
```

**Approach 2 - Flipping right-most differing bits**:

1. Iterate consecutive bit positions from the right to find the first differing bits.
2. Create a mask of the two bit positions
3. XOR the input number with the mask to flip those bits, preserving the population count.

``` python
def closest_integer(n):
    for i in range(64):
        left_bit = (n >> i) & 1
        right_bit = (n >> (i + 1)) & 1

        if left_bit != right_bit:
            mask = (1 << i) | (1 << (i + 1))

            return n ^ mask
        
    return n
```

## Integer Division

**Problem**: Implement the integer division of `$\frac x y$`.

**Approach 1 - Iterative subtraction**: Continuously subtract `$y$` from `$x$` until it is no longer possible.

**Approach 2 - Subtract in large chunks**:

1. While `$x \ge y$`

    1. Find the largest `$k$` such that `$2^k y \le x$`
    2. Subtract `$2^k y$` from `$x$`
    3. Add `$2^k y$` to the resulting quotient

``` python
def divide(x, y):
    result = 0

    while x >= y:
        exp = 1

        # Find largest 2^k such that 2^k * y <= x
        while (exp << 1) * y <= x:
            exp <<= 1

        x -= exp * y
        result += exp

    return result
```

## Exponentiation

**Problem**: Implement integer exponentiation of `$x^y$`.

**Approach 1 - Iterative multiplication**: Multiple `$x$` by itself `$y$` times.

**Approach 2 - Iterative squaring**:

1. While `$y \gt 0$`:

    1. If `$y$` is odd:

        1. Break a term of `$x$` off, e.g. `$x^3 \rightarrow x^2 * x$`
        2. Multiply term into the result
    
    2. Square the base `$x$`
    4. Halve the exponent `$y$`

``` python
def exponent(base, power):
    result = 1.0

    if power == 0: return 1

    if power < 0:
        power = -power
        base = 1.0 / base
    
    while power:
        if power & 1:
            result *= base
        
        base *= base
        power >>= 1
    
    return result
```

## Insert Bits At Position

**Problem**: Insert the bits of M into N at position [i, j).

**Approach**: Use a mask of ones to pass everything from M _except_ slice [i, j), then bitwise OR that with `M << i`.

``` python
def insert_bits(n, m, i, j):
    left = ~0 << (j + 1)
    right = (1 << i) - 1
    region_mask = left | right
    cleared_region = n & ignore_region_mask

    shifted_into_position = m << i

    return cleared_region | shifted_into_position
```

## Print Double As Binary

**Problem**: Print a double that is `$0 \lt x \lt 1$` as a binary number.

**Approach**: Multiply the number by 2 to bitwise shift the first number past the decimal.

If the number is now 1, then it means that the shifted bit was a 1, so print a 1 and remove the left part of the decimal by subtracting 1.

Otherwise the shifted bit was a 0, so print a 0.

``` python
def double_to_binary(num):
    out = "0."

    while num > 0:
        shifted = num * 2

        if shifted >= 1.0:
            out += "1"
            num = shift - 1
        else:
            out += "0"
            num = shift
    
    return out
```

## Midpoint of Given End-Points

**Problem**: Given end-points $[a, b)$ where $a$ is not necessarily 0, determine the mid-point.

**Solution**: Subtract the endpoints and divide them by two to get the middle, then shift the middle to $a$.

<div>
$$\frac {(b - a)} 2 + a$$
</div>

# Bit Manipulation

## XOR without XOR

XOR can be replicated without actually using XOR by simply writing out what XOR means, i.e. exclusive OR: (a AND NOT b) OR (NOT a AND b)

``` python
# emulate XOR
x ^ y == (x & ~y) | (~x & y)
```

## Get Least Significant Bit

The least significant set bit can be obtained with:

``` python
x = 0101 0100
least_significant_set_bit = x & ~(x - 1)
```

The way this works is:

1. all bits up-to-and-including the least significant set bit are flipped

    ``` python
    (x - 1)  = 0101 0011
                     →
    ```

2. flip _all_ bits so that only the previously-least significant set bit will pass when ANDed with the original value

    ``` python
    ~(x - 1) = 1010 1100
    ```

3. AND with the original value so that only the least significant set bit passes

    ``` python
    x & ~(x - 1) = 0000 0100
    ```

## Get Least Significant Set Bit

The least significant set bit can be unset with:

``` python
x = 0101 0100
least_significant_set_bit_unset = x & (x - 1)
```

This works because all bits up-to-and-including the least significant set bit are flipped, such that the least significant set bit is now unset (0) and all prior bits are set (1). All other more significant bits remain unchanged. This way, ANDing both causes the previously least significant set bit to fail.

## Bit Parity

The parity can be computed by continuously right-shifting and ANDing the shifted bit with 1 and adding the result to the running parity. However, only the 1 bits will have an effect on the parity, so a quicker way is to continuously be accessing and turning off the next lowest set bit. Finally, lookup tables of e.g. 16-bit chunks can be precomputed so that on a given input, the parity for each 16-bit chunk of the input is looked up and added together.

## Find Number Occurring Once In Array Of Doubles

Given a list of numbers where each number appears exactly twice _except_ for one number, the number that appears only once can be obtained by XORing each element with the next one, since duplicate elements would cancel themselves out.

## Find Number Occurring Once In Array Of Triples

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

# Strings

## Check Rotation

**Problem**: Determine if an input string is the rotate form of the given text.

**Approach**: Concatenate the string with itself, then search within it.

``` python
def is_rotated(text, input):
    "Determine if `input` is a rotation of `text`."
    return input in (text + text)
```

## Longest Unique Substring

**Problem**: Find the length of the longest substring without repeating characters.

**Source**: https://leetcode.com/problems/longest-substring-without-repeating-characters

---

**Approach 1 - Brute force**: For each position of `$i$` count each character that hasn't been seen. When the first repeat is found, increment to the next position of `$i$`.

``` python
def lengthOfLongestSubstring(s):
  seen = set()
  max_len = 0
  
  for i in range(len(s)):
    for j in range(i, len(s)):
      if s[j] in seen:
        max_len = max(max_len, j - i)
        seen.clear()
        break
      elif j + 1 == len(s):
        max_len = max(max_len, j - i + 1)
        seen.clear()
      else:
        seen.add(s[j])
  
  return max_len
```

---

**Approach 2 - Sliding Window**: Grow a sliding window over the substring from `$[i, j)$`.

* Keep growing to the right by incrementing `$j$` as long as the absorbed character is unique.
* On the first repeat, shrink the window from the left by incrementing `$i$` and removing the character from the seen set.

``` python
def lengthOfLongestSubstring(s):
  seen = set()
  length = len(s)
  max_len = 0
  
  i = 0
  j = 0
  
  while i < length and j < length:
    if s[j] not in seen:
      seen.add(s[j])
      j += 1
      max_len = max(max_len, j - i)

    else:
      seen.remove(s[i])
      i += 1
  
  return max_len
```

## Longest At Least K Repeating Substring

**Problem**: Find the length of the latest substring such that every character appears _at least_ $k$ times.

**Source**: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/

---

**Approach 1 - Brute Force**: For every substring (i.e. pair of substring endpoints), check if that substring is valid under the conditions by counting each character's occurrence and ensuring that each occurs at least $k$ times.

{{< complexity time="n^3" space="n" />}}

``` python
def is_valid_string(s, start, end, k):
    counts = {}
    
    for i in range(start, end + 1):
        count = counts.get(s[start], 0)
        counts[s[start]] = count + 1
    
    for count in counts.values():
        if count < k:
            return False
    
    return True

def longestSubstring(s, k):
    counts = {}
    longest = 0
    
    for i in range(len(s)):
        for j in range(i, len(s)):
            if is_valid_string(s, i, j, k):
                longest = max(longest, j + 1 - i)
    
    return longest
```

---

**Approach 2 - Divide and Conquer**: Start with the entire string. Count the occurrences of every character. If the entire string is valid, return it's length, otherwise find the first invalid character and recurse on the substrings on either side of it, returning the max length of both.

``` python
def helper(s, start, end, k):
    if end - start < k: return 0
    
    counts = {}
    
    for i in range(start, end):
        counts[s[i]] = counts.get(s[i], 0) + 1
    
    for key in counts:
        if counts[key] < k:
            for i in range(start, end):
                if s[i] == key:
                    left = helper(s, start, i, k)
                    right = helper(s, i + 1, end, k)
                    
                    return max(left, right)
    
    return end - start

def longestSubstring(s, k):
    return helper(s, 0, len(s), k)
```

## Longest Common Prefix

**Problem**: Find the longest common prefix of an array of strings.

**Source**: https://leetcode.com/problems/longest-common-prefix/description/

**Approach**: For the first string's character `$c$` at position `$i$`, ensure that all other strings have character `$c$` at position `$i$`.

``` python
def longestCommonPrefix(strs):
    if not strs: return ""
  
    i = 0
  
    while i < len(strs[0]):
        c = strs[0][i]
    
        for s in strs[1:]:
            if i >= len(s) or c != s[i]:
                return s[:i]
    
        i += 1
  
    return strs[0]
```

## Shifting Letters

**Problem**: Given a string and an array of integer shifts, shift the first character by the shift at the first index, the first two characters by the shift at the second index, the first three characters by the shift at the third index, etc.

Shift modulo 26.

Input: `"abc"` and `[3, 5, 9]`

1. `"dbc"`: shift first letter by 3
2. `"igc"`: shift first two letters by 5
3. `"rpl"`: shift first three letters by 9

Output: `"rpl"`

**Source**: https://leetcode.com/problems/shifting-letters/

**Approach 1 - Brute force**: For each letter, separately shift once for each available shift in the array that it would shift by.

``` python
def shiftingLetters(self, S, shifts):
    shift_count = len(shifts)
    s = list(S)
    
    for i in range(0, shift_count):
        for j in range(0, i + 1):
            rooted = ord(s[j]) - ord('a')
            shifted = (rooted + shifts[i]) % 26
            s[j] = chr(shifted + ord('a'))

    return ''.join(s)
```

---

**Approach 2 - Running sum**: Start from the last character that would be shifted, and pop the final shift count for it and add it. For the next character (second-to-last), pop the next shift count and add it to that running sum. Continue doing this for each character.

The space complexity below is actually $n$ because Python has no mutable strings so a separate list has to be created of the string's characters, then joined back into a string. However, this is an implementation detail. Normally mutable strings would allow this to be constant space.

{{< complexity time="n" space="1" />}}

``` python
def shiftingLetters(self, S, shifts):
    s = list(S)
    running = 0
    
    for i in range(len(shifts) - 1, -1, -1):
        running += shifts.pop()
        
        shifted = (ord(s[i]) - ord('a') + running) % 26
        s[i] = chr(shifted + ord('a'))

    return ''.join(s)
```

# Generation

## Generate All Unique Pairs

**Problem**: Compute all of the unique pairs of numbers where each number can be a max of `$N$`.

**Approach**: A nested loop where the second component starts after the first.

``` python
def generate_unique_pairs(n):
    """
    Generate all unique combinations of paris (a, b) such that a and b
    can be any number from [0, n).
    """
    for i in range(n):
        for j in range(i + 1, n):
            yield (i, j)
```

## Generate Permutations

**Problem**: Generate all possible permutations of something, for example a string.

**Approach**: Build the result in the parameter position of successive recursive calls.

## Generate Phone Letter Combinations

**Problem**: Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

**Source**: https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/

**Approach**: Recursively generate the combinations by building them up in the recursive calls' parameter position.

``` python
def generate(self, letters, combinations, digits, built):
    if not digits:
        combinations.append(built)
        return
    
    digit = digits[0]
    
    for letter in letters[digit]:
        generate(letters, combinations, digits[1:], built + letter)

def letterCombinations(self, digits):
    if not digits: return []
    
    letters = {
        "2": ["a", "b", "c"],
        "3": ["d", "e", "f"],
        "4": ["g", "h", "i"],
        "5": ["j", "k", "l"],
        "6": ["m", "n", "o"],
        "7": ["p", "q", "r", "s"],
        "8": ["t", "u", "v"],
        "9": ["w", "x", "y", "z"]
    }
    
    combinations = []
    
    generate(letters, combinations, digits, "")
    
    return combinations
```

# Search

## Find First Bad Version

**Problem**: Given a version number and a function to see if it's a bad version, find the first bad version number, similar to `git bisect`.

**Approach**: Perform a binary search on the versions.

``` python
def first_bad_version(n):
  return helper(1, n)

def helper(start, end):
  if start >= end: return start

  middle = start + (end - start) // 2

  if is_bad_version(middle):
    return helper(start, middle)
  else:
    return helper(middle + 1, end)
```

# Arrays

## Two Sum

**Problem**: Find all pair of numbers in an array that sum up to some target.

**Approach 1 - Brute force**: For each number, check all other numbers.

**Approach 2 - Set of complements**: For each number, determine the complement: number that would need to exist to sum up to the target, `$t - n$`. If the complement is in the set then that number exists in the array, otherwise add the _number_ to the set.

---

The 2-SUM problem is one where the input is a sorted array of integers and a target value `$k$` and the objective is to determine if there is a pair of entries `$i$` and `$j$` such that:

<div>$$A[i] + A[j] = k$$</div>

The brute-force approach would be to check if this is true for all possible pairs of values in the array, of which there are `$n^2$`. Alternatively, a [hash table](#hash-tables) can be used.

Another approach is to iterate from both ends of the array. For example, at first, the first and last elements will be used. If their sum is greater than the target, the right end will be iterated leftward so as to attempt to obtain a smaller element. If instead their sum is less than the target, the left end will be iterated rightward in hopes of increasing their sum.

``` python
while left < right:
  sum = A[i] + A[j]

  if sum > k:
    right -= 1
  elif sum < K:
    left +- 1
  else:
    return true
```

## Two Sum Of Sorted Array

**Problem**: Find a pair of numbers in a _sorted_ array that sum up to some target.

**Approach 1 - Brute force**: For each number, check all other numbers.

**Approach 2 - Iteratively adjust end-points**: Nudge the end-points based on the direction of the target sum from the sum of the end-points.

## Maximum Subarray

**Problem**: Find the sum of the contiguous subarray with the largest sum.

**Source**: https://leetcode.com/problems/maximum-subarray/description/

**Approach**: [Kadane's algorithm](https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm).

Scan the array. For each number, determine which is greater: adding it to the existing sum or keeping it on its own. Then determine which is greater, the previous result or the previous maximum.

``` python
def maxSubArray(nums):
    if not nums: return 0
    
    current = nums[0]
    maximum = nums[0]
    
    for i in nums[1:]:
        current = max(i, i + current)
        maximum = max(maximum, current)

    return maximum
```

## Maximum Share Buy/Sell Profit

**Problem**: Determine the maximum profit that can be obtained over a series of days.

**Source**: _Elements of Programming Interviews_ p. 1, Introduction

**Questions**:

* **What is the input format?**
    * Three arrays to each hold the low, high, and starting prices for each day.
* **Can the purchase and sale on a given day be made with any of these three values?**
    * No, they must be made with the starting prices for the day.

**Analysis**:

* This seems similar to the [Maximum Subarray](#maximum-subarray) problem.
* We can ignore all inputs beside the starting prices, since they only seem to serve to distract.

---

**Approach 1 - Brute Force**: Consider all pairs, returning the one with the highest profit.

{{< complexity time="n^2" space="1" />}}

---

**Approach 2 - Divide-and-Conquer**: Recursively split the array in half and find the best profit pair in each half. In the "combine" step, be careful to account for the case where the buy occurs in the left half and the sell occurs in the right half. In this case, the optimum buy would be the minimum price in the left half, and the optimum sell would be the maximum price in the right half. This makes sense because it's necessary to obtain the higher possible profit.

The recurrence relation is:

<div>$$T(n) = 2T\frac n 2 + O(n)$$</div>

{{< complexity time="n \log n" space="1" />}}

---

**Approach 3 - Linear Scan**: Recognize that the maximum profit that can be made is tied to the minimum buy price, and a sale has to occur at some point. Go through each day keeping track of the lowest buy price seen. If the difference between the current element (i.e. the prospective "sell") and the current minimum (i.e. the cheapest "buy") is greater than the maximum profit recorded, save it as the new maximum profit.

## Rotate Matrix Clockwise

**Problem**: Rotate a matrix clockwise (-90°).

**Approach**: Transpose the matrix, then reverse each column.

## Rotate Matrix Counter-Clockwise

**Problem**: Rotate a matrix counter-clockwise (+90°).

**Approach**: Transpose the matrix, then reverse each row.

## In-Place Removal

**Problem**: Remove all elements of a given value in-place and return the new length of the array.

**Approach**: Partition the array into a filtered region and a removed region. Scan a forward index looking for newly-seen numbers, then write them at the end of the left region denoted by the back index.

``` python
def remove(nums, n):
  if not nums: return 0
  
  i = 1
  j = 1

  while j < len(nums):
    if nums[i - 1] != nums[j]:
      nums[i] = nums[j]
      i += 1

    j += 1

  return i
```

## Find Missing Number From Two Arrays

**Problem**: Given two arrays, determine the number that is missing from the other. There is no guaranteed order, that is, the set difference between the larger array and the smaller array.

**Source**: https://interviewing.io/recordings/Python-Airbnb-1/

**Approach 1 - Sorting**: Sort both arrays, then pairwise iterate through both to find the mismatch. The missing number will be the number being compared from the larger array.

{{< complexity time="n \log n" space="1" />}}

**Approach 2 - Sets**: Add the elements of the smaller array into a set, then determine which element of the larger array is not in the set.

| Case | Order |
| :---- | :---- |
| Time | `$O(n)$` |
| Space | `$O(n)$` |

**Approach 3 - Summing**: Add all of the elements in the larger array and subtract from that sum the elements from the smaller array. The total _must_ be stored in some kind of BigNum type or the overflow and underflow (if there are negative numbers) would corrupt the result.

The space complexity is proportional to the number of bits needed to store the total in the BigNum.

{{< complexity time="n" space="\log n" />}}

**Approach 4 - XOR**: Given XOR's property where the same numbers cancel each other out, `$A \oplus A = 0$`, we can XOR every number in both arrays together to arrive at the missing number, since each pair will cancel itself out, `$A \oplus A \oplus B = B$`.

| Case | Order |
| :---- | :---- |
| Time | `$O(n)$` |
| Space | `$O(1)$` |

## Find Smallest Missing Positive Integer

**Problem**: Find the smallest missing positive integer in an array.

**Approach 1 - Set (Speed)**: Push all numbers into a set, then increment from 1 until the first missing number is found.

**Approach 2 - Sort (Space)**: Sort the array, then iterate until the first missing number is found.

## Find Kth Smallest

|Case    |Growth|
|:-----  |:--------|
|Average |`$\Theta(n)$`|

Selecting the `$k^\text{th}$` smallest item in a sequence can be accomplished by using QuickSort's partition algorithm. This is guaranteed by the invariant held by QuickSort's partition algorithm which states that given the partition index `$j$`, all elements to the left are less than or equal to `$j$` and all elements to the right are greater than or equal to `$j$`, effectively making the sub-sequence up to `$j$` consist of the smallest `$j$` elements in the sequence.

With that in mind, the desired index `$k$` is input to QuickSelect. After partitioning in `$O(n)$`, the resulting position `$j$` of the `$k^\text{th}$` element is compared to the input `$k$`. If the resulting position `$j$` is less than the desired `$k$` then QuickSelect is repeated on the right region `$A[j ..]$` with a compensated `$k$`, i.e. `$k - j$`. If the resulting position `$j$` is greater than the desired `$k$` then QuickSelect is repeated on the left region `$A[.. j]$` with the same `$k$`.

``` cpp
template <typename Iter>
auto QuickSelect(Iter begin, Iter end, std::size_t i) -> decltype(*begin) {
  std::size_t length = std::distance(begin, end);

  if (length == 1) {
    return *begin;
  }

  Iter pivot = Partition(begin, end);

  std::size_t distance = std::distance(begin, pivot);

  if (distance == i) {
    return *pivot;
  } else if (i > distance) {
    // -1 to shift past pivot
    return QuickSelect(std::next(pivot), end, i - distance - 1);
  } else {
    return QuickSelect(begin, pivot, i);
  }
}
```

``` python
def quick_select(items, i):
    if len(items) == 1: return items[0]

    pivot = partition(items)

    if pivot == i:
        return items[i]
    elif i > pivot:
        # -1 to shift past the pivot
        return quick_select(items[pivot + 1 :], i - pivot - 1)
    else:
        return quick_select(items[: pivot], i)
```

## In-Place Reverse Words

**Problem**: Given an exploded string---i.e. `str.split('')`---which is an array consisting of the letters of the words in a phrase, reverse the _words_ in the array in-place.

For example:

``` python
words = "one two three"
exploded = list(words)

exploded = ['o', 'n', 'e', ' ', 't', 'w', 'o', ' ', 't', 'h', 'r', 'e', 'e']
reversed = ['t', 'h', 'r', 'e', 'e', ' ', 't', 'w', 'o', ' ', 'o', 'n', 'e']
```

**Source**: https://interviewing.io/recordings/Java-LinkedIn-1

---

**Approach 1 - Bubble Swap**: Find the boundaries of each word, then reverse the positions of words at a time as one would do in an in-place array reversal, except that the words are swapped by bubbling each character to its destination, thereby preserving the character order.

{{< complexity time="n^2" space="1" />}}

``` python
class Word:
    def __init__(self, start, end):
        self.start = start
        self.end = end

# O(n)
def find_words(a):
    words = []
    start = 0
    end = 0
    length = len(a)

    for i, c in enumerate(a):
        end_of_seq = c != ' ' and i + 1 == length
        end_of_word = c == ' ' and i > 0 and a[i - 1] != ' '

        if end_of_seq or end_of_word:
            end = i - 1
            words.append(Word(start, end))

            start = i + 1

    return words

# O(n)
def bubble_letter(a, source, destination):
    step = 1 if source < destination else -1
    condition = operator.lt if source < destination else operator.gt

    while condition(source, destination):
        a[source], a[source + step] = a[source + step], a[source]

        source += step

# O(n)
def swap_words(a, first, second):
    while first.end >= first.start:
        bubble_letter(a, first.end, second.end)

        first.end -= 1

        # Shift the second word to the left
        second.start -= 1
        second.end -= 1
    
    while second.start <= second.end:
        bubble_letter(a, second.start, first.start)

        first.start += 1
        second.start += 1

def reverse_words(a):
    # O(n)
    words = find_words(a)

    i = 0
    j = len(words) - 1

    # O(n)
    while i < j:
        # O(n)
        swap_words(a, words[i], words[j])

        i += 1
        j -= 1
```

**Lesson**: It's conceptually simple but it feels pretty complicated. The takeaway is that when wanting to move things around while preserving order, one way to do that is to bubble elements.

---

**Approach 2 - Two-Phase Reversal**: Reverse the entire string so that the words are in their proper place, but their characters will be reversed, so simply in-place reverse the slice corresponding to each word.

{{< complexity time="n" space="1" />}}

``` python
# O(n)
def reverse_seq(seq, start, end):
    i = start
    j = end

    while i < j:
        seq[i], seq[j] = seq[j], seq[i]

        i += 1
        j -= 1

def reverse_words(a):
    # Reverse the entire sequence.
    # O(n)
    reverse_seq(a, 0, len(a) - 1)
    last = 0

    # O(n)
    for i, c in enumerate(a):
        if c == ' ':
            reverse_seq(a, last, i - 1)
            
            last = i + 1
    
    # O(n)
    reverse_seq(a, last, len(a) - 1)

    return a
```

**Lesson**: Think about the structure of the problem. I immediately thought about reversing the array, but I just as immediately dismissed it when I realized that the words wouldn't be spelled correctly, instead of taking even just a second to realize that all it would take to fix them would be to reverse the individual words.

# Heaps

## Median Maintenance

The median of a streaming sequence of numbers can be computed in constant time by maintaining two heaps: a max-heap for the lower/left half and a min-heap for the upper/right half. This has the effect of keeping the elements sorted, and the top of the max-heap yields one of the overall middle elements, whereas the top of the min-heap yields the other middle element.

The elements must be kept equal in size, or the min-heap may be larger by one element, in which case there are an odd number of numbers, so the top of the min-heap is the middle element alone.

If one of the heaps grows larger, its top element should be popped and pushed onto the other heap to balance them.

# Linked Lists

## Created And Return Linked List

**Problem**: Create a singly-linked list by creating the node, attaching successor nodes, then returning the list.

**Approach**: Create a dummy head node. Attach successor nodes, then return the dummy head node's next node.

## Remove Node Given its Predecessor

**Problem**: Remove a node from a singly-linked list, given its predecessor node.

**Approach**: Re-link the predecessor node to the target node's successor, i.e. link _past_ the to-be-removed node.

``` python
def remove_node(predecessor):
    if predecessor and predecessor.next:
        predecessor.next = predecessor.next.next
```

## Merge Sorted Lists

**Problem**: Given two sorted lists, merge them into a single sorted list.

**Source**: https://leetcode.com/problems/merge-two-sorted-lists/description/

**Approach**: Set the result's next node to the lesser first node of the two lists as long as both lists have nodes remaining. Then set the result's next node to the list that still has remaining nodes.

``` python
def mergeTwoLists(l1, l2):
  head = dummy = ListNode(0)
  
  while l1 and l2:
    if l1.val < l2.val:
      dummy.next = l1
      l1 = l1.next
    else:
      dummy.next = l2
      l2 = l2.next
    
    dummy = dummy.next
  
  dummy.next = l1 or l2
  
  return head.next
```

## Remove Given Node

**Problem**: Remove a node from a singly-linked list, given that to-be-removed node.

**Approach**: Absorb the value of the successor, then link past the successor.

``` python
def remove_node(node):
    if node and node.next:
        node.value = node.next.value
        node.next = node.next.next
```

## Remove Duplicates

**Problem**: Remove duplicates from a linked list without using additional storage.

**Approach**: Traverse the list and for each node, traverse the rest of the list to find duplicates of that node.

``` python
while node:
    previous = node
    duplicate_finder = node.next

    while duplicate_finder:
        if node.value == duplicate_finder.value:
            previous.next = duplicate_finder.next
            duplicate_finder = previous.next
        else:
            previous = duplicate_finder
            duplicate_finder = duplicate_finder.next

    node = node.next
```

## Add Two Lists As Numbers

**Problem**: A number is represented in a linked list with its digits represented as nodes in reverse. The number 123 is stored as 3 → 2 → 1. Given two such numbers, add them together and express the result as a linked list as well.

**Approach 1 - Digit-wise Addition**: Add each corresponding digit in lock-step, keeping track of carries.

**Approach 2 - Convert to and from numbers**: Convert a linked list to a number, add, then convert back to a linked list.

## Find k-th Last Node

**Problem**: Find the k-th last node in a singly-linked list.

**Approach 1 - Single traversal, using two pointers**:

1. Advance the forward pointer `$k$` nodes ahead
2. Advance both pointers one node at a time until the forward pointer reaches the end
3. The behind pointer will be `$k$` nodes away from the end

**Approach 2 - More than one traversal, counting**:

1. One traversal to count the nodes `$n$`
2. Another traversal up to node `$n - k + 1$`

## Detect Palindrome

**Problem**: Determine if a singly-linked list is a palindrome with a single traversal.

**Approach**: Use a slow pointer and a fast pointer that moves two nodes at a time.

1. Push each node reached by the slow pointer onto a stack
2. After the fast pointer reaches the end, pop and compare for each node reached by the slow pointer

**Note**: If it's an odd-sized list, the slow pointer needs to skip the midpoint. Check if the fast pointer has reached the end by checking if `next` or `next.next` is null. If `next.next`, then skip the midpoint.

## Keep Track of Minimum

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

## Sort

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

## Queue From Two Stacks

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

## Detect Cycle

**Problem**: Detect a cycle.

**Approach**: Traverse the list with a slow and fast pointer. The fast pointer moves two nodes at a time. If there is a cycle, then the two nodes will eventually meet.

## Find Beginning of Cycle

**Problem**: Find the beginning of a cycle.

**Approach**:

1. Detect a cycle
2. When the nodes meet, reset the slow pointer to the start of the list
3. When the pointers meet again, it will be at the start of the cycle.

# Trees

## Find Maximal Subtree

**Problem**: Find the subtree with the largest tree-sum.



## Construct Balanced Binary Search Tree From Sorted Array

**Problem**: Construct a balanced binary search tree from a sorted array.

**Approach**: Insert the midpoint, then recurse on each half slice.

## Find Next In-Order Node in Binary Search Tree

**Problem**: Find the next in-order node in a binary search tree.

**Approach**:

1. If the right sub-tree isn't empty, then return left-most node

    Else find the parent that has the traversed node as it's left child.

``` python
def next_in_order(node):
    if node.right:
        node = node.right

        while node.left:
            node = node.left
        
        return node
    else:
        while node.parent:
            if node == node.parent.left:
                return node
            else:
                node = node.parent
    
    return None
```

## Find All Paths Summing a Total

**Problem**: Find all paths in a tree whose nodes sum to a given total.

**Approach**: Recurse on each child with a running total and max as the parameters.

``` python
def sum_paths(tree, max, running_total):
    paths = []

    if not tree:
        return paths

    if running_total == max:
        return node

    running_total += tree.value

    if running_total > max:
        return paths
    
    if running_total == sum:
        paths.insert(0, tree.value)
    
    left_paths = sum_paths(tree.left, max, running_total)
    right_paths = sum_paths(tree.right, max, running_total)

    for path in left_paths:
        path.insert(0, tree.value)
        paths.insert(0, path)
    
    for path in right_paths:
        path.insert(0, tree.value)
        paths.insert(0, path)
    
    return paths
```

## Check Sub-Tree Containment

**Problem**: Determine if one sub-tree is a sub-tree of another.

**Approach**: Write a function that determines if two trees are equal, recursively. Then use that function to recurse down the tree to see if the sub-tree is present rooted at any given node.

``` python
def matches(a, b):
  if not a and not b: return True
  if not a or not b: return False

  if a.value != b.value: return False

  return matches(a.left, b.left) and matches(a.right, b.right)

def contains(tree, sub):
  if not tree or not sub: return False

  if tree.value == sub.value:
    return matches(tree, sub)
  else:
    return contains(tree.left, sub) or contains(tree.right, sub)
```

## Check Balance

**Problem**: Determine if a tree is balanced, such that the height of sub-trees don't differ by more than one.

**Approach**: Write a function to compute the height of a tree: the max of the height of the left and write sub-trees plus one for the root. Then use that function to determine that a tree is balanced if the absolute difference of the height of each sub-tree is less than or equal to 1 _and_ both sub-trees are also balanced.

**Optimization**: The height can be a parameter of `is_balanced()` and each recursive call can write to it so that the caller can access the height without having to recurse down again by calling `height()`.

``` python
def height(tree):
    return max(height(tree.left), height(tree.right)) + 1

def is_balanced(tree):
    return abs(height(tree.left) - height(tree.right)) <= 1 and
           is_balanced(tree.left) and
           is_balanced(tree.right)
```

## Find Lowest Common Ancestor of Binary Tree

**Problem**: Find the lowest common ancestor (LCA) of two nodes in a binary tree.

**Approach**: Recursively look for the two nodes in both branches. If they're found in separate branches, then the current node must be the LCA.

``` python
def lowest_common_ancestor(root, left, right):
    if not root: return None

    if root == left or root == right:  return root

    else:
    left_lca = lowest_common_ancestor(root.left, left, right)
    right_lca = lowest_common_ancestor(root.right, left, right)

    if left_lca and right_lca: return root

    return left_lca if left_lca else right_rca
```

## Find Lowest Common Ancestor of Binary Search Tree

**Problem**: Find the lowest common ancestor (LCA) of two nodes in a binary search tree.

**Approach**: Descend the tree until a root is found where the nodes are found in separate branches _based on_ the BST invariant conditions.

``` python
while True:
  if root.value > left.value and root.value > right.value:
    root = root.left
  elif root.value < left.value and root.value < right.value:
    root = root.right
  else:
    return root
```

## Get Each Level of Binary Tree

**Problem**: Get each level of a binary tree as a list of lists.

**Approach**: BFS

1. Collect each level into a list
2. Push it onto the levels list
3. Enqueue each level's node's children

``` python
levels = []
next = []

next.push(root)

while next:
  current = next
  next = []
  level = []

  for node in current:
    level.push(node)
    next.push_all(child.children)

  levels.push(level)
```

## Find Maximum Sum Tree

**Problem**: Given a tree with an arbitrary number of children, find the sub-tree with the largest tree sum, where the tree sum is the sum of the values in all of the nodes in the tree.

**Source**: Technical phone interview

**Approach**: Define a function to recursively compute a tree's sum, preferably cache this value when it is computed. Then define a function to recursively return the node with the maximum tree sum.

``` python
class Node:
    def __init__(self, value):
        self.value = value
        self.parent = None
        self.children = []
        self._sum = None

    def attach(self, node):
        self.children.append(node)

        return node

    def sum(self):
        if not self.children:
            return self.value

        if self._sum is not None:
            return self._sum

        children_sum = 0

        for child in self.children:
            children_sum += child.sum()
        
        total = self.value + children_sum

        if self._sum is None:
            self._sum = total
        
        return total

    def max_sum_tree(self):
        if not self.children:
            return self
        
        max_tree = self

        for child in self.children:
            max_child = child.max_sum_tree()

            if max_child.sum() > max_tree.sum():
                max_tree = max_child
        
        return max_tree
```

## Add Row At Depth

**Problem**: Given a binary tree, a depth, and a number, modify the tree so that nodes with the given value populate the level at the specified depth.

**Source**: https://leetcode.com/problems/add-one-row-to-tree/

**Approach 1 - Breadth-first Search**: 

``` python
from collections import deque

def addOneRow(self, root, v, d):
    if d == 1:
        new_root = TreeNode(v)
        new_root.left = root
        return new_root
    
    frontier = deque()
    frontier.append((1, root))
    
    while frontier:
        level, node = frontier.popleft()
        
        if not node: continue
        
        # We're at the level to reattach children
        if level + 1 == d:
            new_left = TreeNode(v)
            new_left.left = node.left
            node.left = new_left

            new_right = TreeNode(v)
            new_right.right = node.right
            node.right = new_right
        else:
            frontier.append((level + 1, node.left))
            frontier.append((level + 1, node.right))
    
    return root
```

**Approach 2 - Depth-first Search**: 

``` python
from collections import deque

def helper(root, v, d, level=1):
    if not root:
        return root
    
    if d == 1:
        new_root = TreeNode(v)
        new_root.left = root
        
        return new_root
    
    if level + 1 == d:
        new_left = TreeNode(v)
        new_left.left = root.left
        root.left = new_left
        
        new_right = TreeNode(v)
        new_right.right = root.right
        root.right = new_right
    else:
        root.left = helper(root.left, v, d, level + 1)
        root.right = helper(root.right, v, d, level + 1)
    
    return root

def addOneRow(root, v, d):
    return helper(root, v, d)
```

# Binary Search Trees

## Convert Sorted Array To BST

**Problem**: Create a balanced binary search tree given a sorted array in ascending order.

**Source**: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree

**Approach 1 - Recursive**: Create a root node from the midpoint, attach the children by recursing the left and right sub-regions, then return the root node.

``` python
def sortedArrayToBST(self, nums):
    if not nums: return None

    mid = len(nums) // 2
    node = TreeNode(nums[mid])

    node.left = sortedArrayToBST(nums[:mid])
    node.right = sortedArrayToBST(nums[mid+1:])

    return node
```

# Graphs

## Find Strongly-Connected Components

**Problem**: Find all of the strongly-connected components in a graph.

**Approach 1 - DFS**:

1. DFS the reverse graph, saving the order of visited nodes.
2. DFS the normal graph in the above order.

    Each vertex from which the DFS is initiated is part of the same connected component.

**Approach 2 - Union-Find**: Use the [Union-Find algorithm].

[Union-Find algorithm]: /notes/algorithms/#dynamic-connectivity

## Determine Lexical Order

**Problem**: Determine the lexical order of the characters of an unknown alphabet given a list of words sorted by that lexical order.

**Approach**: Create a digraph of the characters with a known lexical order. Then perform a topological sort of the graph to compute a possible lexical ordering.

For example, if the word _apple_ comes before _banana_ in the list, then we know that the letter _a_ comes before the letter _b_ in this alphabet.
