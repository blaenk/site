+++
title = "Longest At Least K Repeating Substring"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["strings"]
+++

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
