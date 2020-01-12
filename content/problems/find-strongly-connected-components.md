+++
title = "Find Strongly-Connected Components"
date = 2020-01-03T02:55:24-08:00
draft = true
categories = ["graphs"]
+++

**Problem**: Find all of the strongly-connected components in a graph.

**Approach 1 - DFS**:

1. DFS the reverse graph, saving the order of visited nodes.
2. DFS the normal graph in the above order.

    Each vertex from which the DFS is initiated is part of the same connected component.

**Approach 2 - Union-Find**: Use the [Union-Find algorithm].

[Union-Find algorithm]: /notes/algorithms/#dynamic-connectivity
