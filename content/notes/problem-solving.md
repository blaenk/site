+++
title = "Problem Solving"
date = 2018-10-10T21:14:47-07:00
draft = true

[note]
kind = "concept"
+++

These are notes on the general problem solving techniques through the application of [algorithms](/notes/algorithms) and [data structures](/notes/data-structures).

<nav id="toc"></nav>

# Algorithms

1. Preliminaries

    1. Clarify and summarize the problem

        * Distill requirements
        * Inputs: types, structure (sorted?), size, range
        * Outputs: types, error handling
        * Compare the inputs to the outputs

    2. Design and confirm simple test cases (expected, edge, and failure inputs)
    
        * Especially if the provided test cases are overwhelming

2. Design

    1. Design the brute-force approach and determine its complexity
    2. Design alternatives and prioritize them for implementation
    
        * Optimizing for space or speed?
    
    3. Outline approaches with pseudocode or higher-level functions
    4. Trace through approach with test cases

3. Implementation

    1. High-level, top-down implementation

        * Focus on the algorithm, fill in the rest later

    2. Trace through implementation with test cases
    3. Complexity
    4. Further optimizations

3. Retain

    1. Create an entry for the problem

        * Include the question, its source, solution, and any reflections

    2. Add to any of the following:

        1. Its category
        2. [Best Practices](#best-practices)
        3. [Pitfalls](#pitfalls)

    2. Create Anki cards
        1. High-level solution
        2. Low-level details

Practicing:

* **Cap the allotted time to a maximum of 30 minutes when practicing.**

    Generally a 45 minute to 1 hour interview will leave about this much for the actual problem-solving portion so it also serves as time-management practice.

* **Prefer questions with answers.**

    There is a plethora of practice problems out there. Refer to the solution if you can't solve it by the allotted time. Follow the retention steps all the same. This is better than getting stuck and hung up on trying to solve one particular problem because it maintains pacing which is important psychologically and increases coverage.

# System Design

1. Determine the scope by asking clarifying questions

    * Use cases
    * Constraints

2. Make reasonable assumptions

    * Capacity planning
    * Load parameters
    * Percentiles
    * How much traffic is being served
    * How much data needs to be stored
    * Data consistency requirements

3. Draw major components on the whiteboard

    * Front-end servers, back-end servers, databases, etc.

4. Identify key issues

    * Bottlenecks, challenges

5. Redesign for key issues

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

# Behavioral

Behavioral interviews try to:

* Assess:

    * Honesty
    * Culture fit
    * Collaborative-ness
    * Communication ability
    * Long-running passion and enthusiasm

* Validate the resume
* Gauge their job offer

_"Can you tell me about a time when …"_

Use the STAR format. Be careful to focus on what _you_ did, not the group: use _I_ not _we_.

1. Situation: Describe the situation.
2. Task/Target: Describe the tasks/goals you were required to achieve.
3. Action: Describe the actions taken to achieve the goals.
4. Result: Describe the outcome. What did you learn?

# Best Practices

## Work Top-Bottom

Sometimes the high-level solution is enough to convey understanding of the problem and its solution. Unnecessarily delving into specifics at a very low-level risks pedantic mistakes that detract from your perceived overall comprehension.

This applies especially for system design questions, where a high-level solution with the ability to drill-down really conveys understanding.

It also applies to algorithm questions. Don't waste time on trivial aspects like finding the maximum of an array. Focus on the high-level algorithm implementation using code you wish you had. The details can be filled in later as needed or requested.

## Use Abstractions

As software engineers we make complex problems manageable by breaking them down into abstractions. Why throw this out during interviews? It puts you at a disadvantage. Create types and functions.

Abstractions signal the acknowledgment that code exists but can be filled in later in order to focus on the meat of solving precisely what is being asked.

The cleaner code that comes from this will greatly help you as you reason about the code mid-way and later on when you seek to optimize it.

The cleaner code will also make a better impression on the interviewer.

## Codify Invariants

When a problem specifies conditions or predicates under which a solution satisfies the problem, codify those conditions in code to make it easier to reason about the solution.

## Simplify Test Cases

It's easy to get overwhelmed or confused with the test case(s) provided by the problem. Other times instead of getting many small, manageable test cases you will get one large test case that exhibits each of the edge cases that will be tested for. Trying to mentally model a solution for that large, comprehensive test case can be daunting and time consuming.

Try to reduce the test case to multiple minimal test cases, but be careful to ensure that you are still representing the problem. Seek confirmation if possible.

Once a solution is designed, make sure it works for the original test case and not just the minimal test cases you decomposed it to.

## Assume Valid Inputs

Don't waste time on validating every edge case of an input, but make these assumptions clear to the interviewer. An invalid input is for example a numeric string containing non-numeric characters.

One idea in line with [Work Top-Bottom](#work-top-bottom) might be to start the function with a call to a validation function that can be implemented later if the acknowledgement of the necessity of input validation is not enough.

## Acknowledge Pathological Inputs

A pathological input might be one that would lead to pathologically worst-case performance, such as a deep singly-branched tree.

Make it clear to the interviewer that you are aware of pathological inputs.

## Theoretical Solutions

Do mention theoretical approaches even if they're not eventually implemented. For example, if an algorithm would only work if a BigNum type were used, to prevent overflows, mention it. It will communicate that you're at least aware of that approach, even if you don't or can't ultimately use it.

# Pitfalls

## Premature Optimization

Don't fall for the trap of premature optimization. It's true that solutions should preferably be optimal, but thinking about the solution _and_ making it optimal from the beginning can be overwhelming. You may end up doing both poorly or none at all.

Premature optimization can be insidious in how it skews your perspective of what is being asked and/or how to solve it, funneling you toward [The Einstellung Effect](#the-einstellung-effect).

Even so, at times optimizing as-you-go is viewed as the ability to "walk and chew bubble gum at the same time," such that foregoing it _may_ be viewed negatively, as if you were outright unaware of the possibly obvious optimization.

For this reason only forego a premature optimization if you note it to the interviewer as something that can be done later. You might find that the interviewer requests that you do it then and there.

## Not Invented Here

Don't fall into the trap of assuming that everything in your solution must be written from scratch.

Instead of biasing from that direction, bias from the direction of not reinventing the wheel. If the interviewer wants you to implement something you're delegating to, you can do so, but otherwise you will have saved yourself a lot of time.

## Apparent Impossibility

If a problem seems impossible, you are very likely overthinking it or completely misunderstanding it. Ask for examples and clarifying questions and try to think about it a different way. Maybe you misunderstood a simple component of the problem.

On the other hand, don't confuse intractability with impossibility. Even if a problem is intractable and the (possibly brute-force) solution can't run in any reasonable amount of time for large inputs, it is still a solution, and a solution is better than none.

It is also possible, maybe even likely, that there is a subtle fact about the problem that unlocks an embarrassingly efficient solution.

## Daunting Complexity

Similar to impossibility, if a problem seems overly complex or it seems like a solution would be really long, it is likely an indication that you are overthinking the problem and need to reconsider your approach. Maybe you misunderstood a simple component of the problem.

Consider that a reasonable interviewer wouldn't ask something too complex to reason about, implement, _and_ have them verify within a ~45 minute interview.

_Maybe_ the interviewer is knowingly giving you an "impossible" problem without actually expecting you to solve it, instead wanting to see how you approach the problem and handle a seemingly-impossible situation---a so-called "stress interview". I think this is unlikely, but it emphasizes the importance of establishing consensus on what the problem is and conveying your ideas for the approach before you begin and as you go.

## The Einstellung Effect

The [Einstellung effect](https://en.wikipedia.org/wiki/Einstellung_effect) refers to a situation in which you become fixated on one misguided approach to solving a problem.

In my experience, I have gravely blown up a simple geometry problem involving cells on a grid to one involving Bresenham rasterization or A* path-finding. I allowed the prestige of the company to psyche me out into believing that the company would reasonably expect a typical candidate to solve something that complicated or niche.

## The God Loop

Sometimes it's easy to let a simple loop grow into a "God Loop" that does too much with ever increasing edge case conditions that need to be handled, making the code messier and more difficult for you to reason about.

Consider if it might be easier to split it up into separate loops while still being the same complexity class (see [Sum of Bounds](/notes/algorithms/#sum-of-bounds)), noting to the interviewer that the loops can be optimized by combining them.

Another option is to factor the body out to separate functions.

## Starting Too Low-Level

If you start thinking or implementing at too low a level you may risk "not seeing the forest for the trees". It can become more difficult to reason about the overall problem if skewed by a low-level perspective, which reduces adaptability to changes in the problem.

Try to resist the urge to implement low-level pieces as soon as you identify them, as that feels like the most obvious thing to do. Instead work at a high level with code you don't have but wish you did, then implement as needed.

See [Work Top-Bottom](#work-top-bottom).

Sometimes the top-bottom perspective can seem hazy and vague, whereas at least some of the necessary low-level functions feel more concrete and obvious. This contrast can drive one to implement the low-level aspects in hopes that it may help to flesh out the higher-level pieces, funneling one toward this problem.

One possible remedy to counter this urge is to try sketching the problem out in pseudocode. Using pseudocode removes the urge to write perfect working code, which means you don't have to know all of the pieces involved, freeing your mind for high-level sketching to determine the pieces involved.

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

# Taxonomy

## Categories

This is mostly an intersection of categories found in books like EPI, CTCI, Algorithm Design Manual, and sites like leetcode.

I bias toward keeping the category count low, such as coalescing `binary-search-trees` into `binary-trees` into `trees`.

* {{< count-problems "arrays" >}}
* {{< count-problems "bitwise" >}}
* {{< count-problems "combinatorial" >}}
* {{< count-problems "divide-and-conquer" >}}
* {{< count-problems "dynamic-programming" >}}
* {{< count-problems "graphs" >}}
* {{< count-problems "greedy" >}}
* {{< count-problems "hash-tables" >}}
* {{< count-problems "heaps" >}}
* {{< count-problems "linked-lists" >}}
* {{< count-problems "math" >}}
* {{< count-problems "other" >}}
* {{< count-problems "queues" >}}
* {{< count-problems "recursion" >}}
* {{< count-problems "search" >}}
* {{< count-problems "sets" >}}
* {{< count-problems "sorting" >}}
* {{< count-problems "stacks" >}}
* {{< count-problems "strings" >}}
* {{< count-problems "trees" >}}
* {{< count-problems "union-find" >}}

## Tags

* {{< count-problems tag="double-ended-iteration" >}}
* {{< count-problems tag="dummy-head" >}}
* {{< count-problems tag="slow-fast-iteration" >}}
* {{< count-problems tag="sorted-sequence-merge" >}}

# Hash Tables

{{< problems "hash-tables" >}}

# Other

{{< problems "other" >}}

# Math

{{< problems "math" >}}

# Bit Manipulation

{{< problems "bitwise" >}}

# Strings

{{< problems "strings" >}}

# Combinatorial

{{< problems "combinatorial" >}}

# Recursion

{{< problems "recursion" >}}

# Search

{{< problems "search" >}}

# Arrays

{{< problems "arrays" >}}

## Double-Ended Iteration

The "left" iterator iterates from the left-to-right of the array and the "right" iterator iterates right-to-left of the array.

{{< problems tag="double-ended-iteration" >}}

# Heaps

{{< problems "heaps" >}}

# Linked Lists

{{< problems "linked-lists" >}}

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
    if right is None:           # Right is empty
        merged.next = left
        left = left.next
    elif left is None:          # Left is empty
        merged.next = right
        right = right.next
    elif left.val <= right.val: # Left and right non-empty, left head smaller
        merged.next = left
        left = left.next
    else:                       # Left and right non-empty, right head smaller or equal
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

## Slow-Fast Iteration

The "normal" iterator iterates one element at a time, while the "fast" iterator either starts ahead at the same pace or at a faster pace. There can also be a "slow" iterator---instead of a "fast" one---that lags behind the normal iterator.

One thing to remember is that slow and fast are relative terms. Most times one is slow or fast relative to the other, rarely are both slow and fast.

{{< problems tag="slow-fast-iteration" >}}

# Stacks

{{< problems "stacks" >}}

# Trees

{{< problems "trees" >}}

# Sets

{{< problems "sets" >}}

# Graphs

{{< problems "graphs" >}}
