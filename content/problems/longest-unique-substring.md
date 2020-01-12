+++
title = "Longest Unique Substring"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["strings"]
+++

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
