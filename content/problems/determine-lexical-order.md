+++
title = "Determine Lexical Order"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["graphs"]
+++

**Problem**: Determine the lexical order of the characters of an unknown alphabet given a list of words sorted by that lexical order.

**Approach**: Create a digraph of the characters with a known lexical order. Then perform a topological sort of the graph to compute a possible lexical ordering.

For example, if the word _apple_ comes before _banana_ in the list, then we know that the letter _a_ comes before the letter _b_ in this alphabet.
