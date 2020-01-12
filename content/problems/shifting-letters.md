+++
title = "Shifting Letters"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["strings"]
+++

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
