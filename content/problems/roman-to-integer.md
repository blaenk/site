+++
title = "Roman to Integer"
date = 2020-01-04T14:23:03-08:00
draft = true
categories = ["other"]
+++

**Source**: https://leetcode.com/problems/roman-to-integer

``` python
class Solution:
    def roman_to_decimal(self, c: str) -> int:
        if c == 'I':
            return 1
        elif c == 'V':
            return 5
        elif c == 'X':
            return 10
        elif c == 'L':
            return 50
        elif c == 'C':
            return 100
        elif c == 'D':
            return 500
        elif c == 'M':
            return 1000
        else:
            raise Exception('Invalid Roman Numeral')

    def romanToInt(self, s: str) -> int:
        if not s:
            return 0
        
        total = previous
        
        for digit in s[1:]:
            decimal = self.roman_to_decimal(digit)
            
            if decimal > previous:
                total = (total - previous) + (decimal - previous)
            else:
                total += decimal
                
            previous = decimal
        
        return total
```

![](/images/problems/roman-to-integer.png)