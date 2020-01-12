+++
title = "Longest Common Prefix"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["strings"]
+++

**Problem**: Find the longest common prefix of an array of strings.

**Source**: https://leetcode.com/problems/longest-common-prefix

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

![](/images/problems/longest-common-prefix.png)