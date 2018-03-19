---
title = "Python"
published = "March 14, 2018"
excerpt = "The Python Programming Language"
comments = false
---

<toc />

A long line can be split into two lines by ending the first line with a backslash `\`. Python also automatically joins adjacent lines if an open parentheses `(`, bracket `[`, or brace `{` hasn't been closed. Lines after the first physical line are _continuation lines_. Indentation rules only apply to the first physical line and not to continuation lines.

By convention, an identifier starting with a single leading underscore `_` indicates that it's meant to be private, while an identifier starting with two leading underscores indicates a strongly private identifier, but if it also ends in two underscores it usually indicates a language-defined special name.

Note that although a set literal is expressed as `{a, b}`, the literal `{}` denotes an empty _dictionary_, not an empty set. An empty set has no literal syntax, but can be expressed as `set()`.

# Data Types

The built-in `type(obj)` returns the type object that is the type of `obj`. The built-in `isinstance(obj, type)` is `True` if `obj` has type `type`.

## Numbers

Python 3.6 added support for single underscores between digits or after any base specifier.

## Strings

The items of a string are strings; there is no character type in Python.

Escape sequences work in both single and double-quoted strings, unlike other languages where single-quoted strings are treated literally. String lines can be continued with a backslash `\` continuation, but necessary newlines must be included explicitly. On the other hand, triple-quoted strings preserve line breaks.

By convention, single-quoted strings are preferred.

Raw string literals are possible by preceding the string literal with an `r` or `R`. Unicode literals can be specified by name with the `\N{name}` syntax, such as `\N{Copyright Sign}`.

Multiple, adjacent string literals of any kind are concatenated by the compiler into a single string object.

Since strings are immutable, attempting to rebind or delete an item or slice of a string raises an exception.

## Tuples

Tuple parentheses are optional unless the context would make the commas ambiguous, such as in function calls, or to denote empty or nested tuples. A tuple of one item requires a comma after the single item. The built-in `tuple()` creates a tuple with an item for each item in the provided iterable.

Tuple objects are immutable, so attempting to rebind or delete an item or slice of a tuple raises an exception. Individual tuple items may be mutable.

## Sequences

Sequences are ordered containers that are accessible by indexing and slicing.

The built-in `len()` function returns the number of items in a container.

A sequence can be repeated by multiplying it with an integer. If the integer is zero or less, the container is emptied.

The `in` operator tests to check whether an object equals any item in the given sequence or iterable. The converse is `not in`. In dictionaries, `in` tests for key presence, while in strings it tests for substring presence.

Negative indexing starts from the end of the sequence of length $L$, i.e. `-1` is the last element, at index $L - 1$.

A slicing of `[i:j]` is exclusive: it excludes the upper bound. When the upper bound is less than or equal to the lower bound, the slicing is an empty subsequence. When a bound is omitted, the extent is implied: `0` for the lower bound, sequence length $L$ for the upper bound. Omitting both bounds takes a slicing of the entire sequence, which is often used to make shallow copies.

A slicing can include a _stride_ denoting the distance between successive indices, in the form `[i:j:k]`. For example, `[::2]` would include all even indices of the original sequence, while `[::-1]` would contain the sequence elements in reverse, in which case the lower and upper bound should be flipped, which makes sense if thought of as "from" and "to" indicators.

## Lists

The built-in `list()` creates a list with an item for each item in the provided iterable.

Lists may be heterogeneous.

A list slice can be assigned an iterable. If the slice includes a stride, then the assignment must have as many items as there are in the slice, otherwise the assignment may be of any length, which can end up making the list longer or shorter.

Assigning an empty list `[]` to a slice removes that slice from the list, so that `L[i:j] = []` is equivalent to `del L[i:j]`, which is also equivalent to repeating zero times with `L[i:j] *= 0`.

Assigning to an empty slice `[i:i]` inserts the target items at that position, before the element previously at that index.

Assigning to a complete slice `[:]` replaces the contents of the list (it's _not_ a simple rebinding).

`count(x)` returns the number of items in the list that are equal to `x`.

`index(x)` returns the index of the first occurrence of an item in the list equal to `x`, or raises an exception if there is none.

`append(x)` appends `x` to the end of the list.

`extend(s)` appends the items of iterable `s` to the end of the list. Equivalent to list `+=`.

`insert(i,x)` inserts `x` before the element previously at `i`.

`pop(i=-1)` returns and removes the value at index `i`, which is the last element by default.

`sort(key=None,reverse=False)` sorts the list in-place in a stable manner.

The built-in `sorted()` produces a sorted list from any input iterable.

## Sets

There are `set`s and `frozenset`s. Instances of `set` are mutable, but not hashable, while instance of `frozenset` are immutable, but hashable, so they can be stored within a set.

Sets can be created with the builtin `set` or `frozenset` and a given iterable, or with set literal syntax `{a, b}`.

The `in` operator checks for set membership.

## Dictionaries

The built-in `dict()` creates a dictionary from the iterable of key/value pairs, and if a key appears multiple times, only the last occurrence takes effect. Additionally, named arguments can be passed to explicitly specify key/value pairs, and they override any previous duplicate key occurrences.

The `dict.fromkeys()` function can also be used to construct a dictionary from an iterable of keys and a value to associate with each.

Indexing with a non-existent key raises an exception, however, assignment to a non-existent key is valid.

Removing a non-existent key entry with `del` raises an exception.

`get(k[, x])` returns the value associated with key `k` or the default value `x`.

`items()` returns an iterable `dict_items` instance.

`keys()` returns an iterable `dict_keys` instance.

`values()` returns an iterable `dict_values` instance.

`pop(k,[, x])` is similar to the same method for lists. It removes and returns the value associated with key `k`, or `x` if there is none.

`popitem()` removes and returns an arbitrary key/value pair.

`setdefault(k[, x])` returns the value associated with key `k` if any, and if there is none, establishes a key-value pair with `x` as the value.

`update(d)` merges the entries of dictionary `d` into the dictionary. It can also accept an iterable of key/value pairs, as well as named arguments.

## Callables

Classes that supply a special method named `__call__` are callable.

## Booleans

Any nonzero number or nonempty container is `True`, while zero of any numeric type, `None`, and empty containers are `False`.

The `bool` type is a subclass of `int`.

## Variables

Variables or other references have no intrinsic type, but rather the object to which a reference is bound has a type; a given reference may bound to objects of various types.

A global variable is an attribute of a module object. A local variable is local to a function.

An object's attribute is accessed through a period `.`, while an object's item is accessed through brackets `[]`. Callable attributes are also known as methods. Plain assignment to an object attribute is a request to create or rebind the attribute with that name, while plain assignment to an item in a container is a request to the container to create or rebind the item at that index or key.

Functions and callables are first-class objects; they can be bound to variables.

Assigning to `object[start:stop:stride]` is equivalent to `obj[slice(start,stop,stride)]`.

An unpacking assignment is one where the right-hand side is an iterable with exactly as many items as there are references on the left-hand side. One of the multiple left-hand side targets can be a starred target preceded by an asterisk `*` which is bound to a list of all items that were not bound to the other targets.

``` python
a, b, c = [1, 2, 3]

a, b = b, a

first, *middle, last = ["John", "Smith", "Quincy", "Doe"]
```

Augmented assignment works by first checking if the left-hand side is an object with a special in-place version of the augmented operator, in which case it is called, otherwise it applies the corresponding unaugmented binary operator to the left and right objects, then rebinds the target reference to the operator's result. So `x+=y` uses `__iadd__` if it exists, otherwise it works as `x=x+y`.

Augmented assignment (e.g. `+=`) generally cannot create new references.

The `del` statement unbinds references, rather than actually delete objects, which instead is a natural consequence of garbage collection. It can take one or more target references separated by commas `,`, and each target can be a variable, attribute, indexing, or slicing. Each target must be bound at the time `del` executes.

When the `del` target is an identifier, it is unconditionally unbound. Otherwise, it's a request to an object to unbind one or more of its attributes or items, a request which the object may refuse.

Unbinding a slicing is conventionally equivalent to assigning an empty sequence to that slicing, but a given container can specify other behavior.

# Expressions

Comparisons can be chained, implying logical `and`.

``` python
a < b <= c < d

# Equivalent:
a < b and b <= c and c < d
```

The ternary operator takes the form of `… if … else …`.

Use `//` for truncating division, with mnemonic "extra `/` to chop/truncate the decimal". To avoid truncation, use `/` and ensure that at least one operand is not an integer, such as by multiplying an operand by `1.0`.

# Control Structures

Containers should not be modified while being iterated over, unless rebinding an item at an existing index or key.

A for loop's target can have multiple identifiers as in an unpacking assignment, but the iterator's items must be iterables with exactly as many items as there are identifiers, or one of the identifiers may be preceded by an asterisk `*` to absorb all items that were not assigned to other targets.

Note a for loo's control variable remains bound to the last value that the loop statement set.

``` python
for x in [1, 2, 3]:
  f(x)

assert x == 3
```

while and for loops may have an optional trailing `else` clause which executes when the loop terminates naturally and not prematurely.

The `pass` statement can be used to perform no action, as an explicit placeholder, for the body of a compound statement. Classes and functions should use a docstring instead.

# Iterators

An iterator is some object `i` on which the built-in `next()` can be called to return the next item or raise a `StopIteration` exception. The `next()` built in also takes an optional default value to return when the iterator has no more items.

Classes can define the `__next__` method to allow instances to be iterators.

Iterators are built by implicitly (in the case of a for loop) or explicitly calling the built-in `iter()`, which itself calls a special method `__iter__` to actually obtain an iterator.

``` python
for x in c:
  f(x)

# Equivalent:
_temp_iterator = iter(c)

while True:
  try: x = next(_temp_iterator)
  except StopIteration: break
  f(x)
```

The built-in `range(x)` returns an object yielding consecutive integers from `[0, x)`. The `range(x,y)` form yields consecutive integers from `[x, y)`. The `range(x,y,s)` form accepts a stride `s`.

# Comprehensions

## List Comprehensions

A list comprehension is equivalent to a for loop that builds a list by repeated calls to the `append` method and has the form:

``` python
[ expr for target in iterable lc-clauses ]
```

where `lc-clauses` is one of:

``` python
for target in iterable

if expression
```

For example:

``` python
incremented = [x + 1 for x in sequence]

flattened = [x for sublist in listoflists for x in sublist]

evens = [x for x in sequence if x % 2 == 0]
```

Note that lists shouldn't be built if they're only going to be used for iteration, in which case generator expressions should be used instead.

Note that only in v2, and only for list comprehensions, the target variables remain bound to their last value outside of the list comprehension.

## Set Comprehensions

Set comprehensions have the same syntax and semantics as list comprehensions, except that they use curly braces `{}` instead, and the order of the items is irrelevant.

``` python
s = {n//2 for n in range(10)}
#   {0, 1, 2, 3, 4}
```

## Dictionary Comprehensions

Dictionary comprehensions have the same syntax as set comprehensions except that a key/value pair is specified with two expressions separated by a colon `:`.

``` python
d = {n : n//2 for n in range(5)}
#   {0: 0, 1: 0, 2: 1, 3: 1, 4: 2}
```
