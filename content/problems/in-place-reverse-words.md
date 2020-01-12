+++
title = "In-Place Reverse Words"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["arrays"]
+++

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
