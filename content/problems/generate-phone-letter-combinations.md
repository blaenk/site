+++
title = "Generate Phone Letter Combinations"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["combinatorial"]
+++

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
