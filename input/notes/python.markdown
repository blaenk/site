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

Global variables are attributes of the module object.

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

## Generator Expressions

Generator expressions (aka genexps) are similar to list comprehensions except that they they're surrounded in parentheses and construct generator objects, which can be omitted if passed directly as a function argument.

``` python
sum(x*x for x in range(10))
```

# Functions

A function always returns a result value or `None` if none was provided.

Python computes a default value for a parameter exactly once, not each time the function is called, which means that the same object gets bound to the parameter whenever one isn't supplied by the caller. For example, if an empty list were used as a default parameter value and then appended to, the list would grow across calls.

``` python
def f(x, y=[]):
  y.append(x)
  return y, id(y)

print(f(23)) #=> ([23], 123)
print(f(42)) #=> ([23, 42], 123)
```

To prevent this from happening, an idiom is to set the default parameter value to `None`, then test for it within the function to determine whether to set the actual default value:

``` python
def f(x, y=None):
  if y is None: y = []
  y.append(x)
  return y, id(y)

print(f(23)) #=> ([23], 123)
print(f(42)) #=> ([42], 456)
```

One circumstance in which it may be desirable may be for caching, but that may be better done through memoization.

Functions may take parameters of the form `*args` or `**kwargs` (or `**kwds`) at the end of the parameter list. The `*args` form causes arguments to be collected into a possibly empty tuple. The `**kwds` form causes arguments to be collected into a dictionary.

Iterables can be expanded so that each item binds as an argument to a function parameter using the same `*seq` and `**dct` syntax, but `seq` must have the right number of items to correspond to arguments, and `dct` must have the right identifier strings as its keys. In Python 3, a function call may have zero or more of each form.

Python 3 supports optionally specifying parameters that must correspond to named arguments of the form `ident=expr`, which are known as keyword-only parameters. Such parameters, if present, must appear after `*args` (if any) and before `**kwargs` (if any). If there is no `*args` form, then keyword-only parameters must follow a null parameter consisting solely of an asterisk `*`. The parameter may be a simple identifier, in which case its presence at the call-site is mandatory, or in the form `ident=default` in which case it takes the default value when it is omitted at the call-site.

Note that keyword-only parameters cannot be matched with a positional argument, but must instead match a named argument.

``` python
# No *args parameter, so mark keyword-only parameters
# with a null parameter '*'
def f(a, *, b, c=56):
  # b and c are keyword-only
```

Named arguments can be passed for readability purposes because positional parameters can be matched by named arguments in the absence of matching positional arguments.

``` python
def divide(divisor, dividend):
  return dividend // divisor

print(divide(dividend=94, divisor=12))
```

Arguments can be passed as positional arguments even when the parameters are named.

The `def` statement sets certain attributes of a function object, including the `__name__` which refers to the function's name as a string, and `__defaults__` which refers to the tuple of default values for optional parameters.

The `__doc__` attribute corresponds to a function's documentation string (docstring) which is usually bound to the first statement of a function body if it's a string literal. Since docstrings typically span many lines, they're often specified with triple-quoted strings.

By convention, the first line of a docstring should be a concise, single-line summary of the entity's purpose. If it's multiline, then the summary line and more in-depth documentation are separated by an empty line.

Function objects may have arbitrary attributes bound.

``` python
def counter():
  counter.count += 1
  return counter.count

counter.count = 0
```

Python 3 allows every parameter in a `def` clause to be annotated with an arbitrary expression of the form `ident:expr`, such that the expression's value becomes its annotation. The return value of a function can likewise be annotated with the form `->expr` following the parameter list's closing parenthesis `)`, which annotates the name `'return'`.

``` python-console
>>> def f(a: 'foo', b)->'bar': pass
>>> f.__annotations__
{'a': 'foo', 'return': 'bar'}
```

If a function needs to bind or rebind a global variable instead of a local variable, then the first statement of the function must be a `global` statement listing the comma `,` delimited identifiers corresponding to global variables. This is only necessary if the function rebinds a global variable, not if it simply uses a global variable.

Without the `global` statement below, `_count` would be an uninitialized local variable.

``` python
_count = 0

def counter():
  global _count
  _count += 1
  return _count
```

A nested function may access, but not rebind, local variables of an outer function, also known as free variables of the nested function. Alternatively a free variable can be bound to a nested function's parameter's default value. A nested function that does access values from an outer scope is known as a closure.

Python 3 has the `nonlocal` keyword which acts similar to `global`, allowing a nested function to refer to a name in the namespace of some outer function and causing an error if the name is not found.

``` python
def make_counter():
  count = 0
  def counter():
    nonlocal count
    count += 1
    return count
  return counter
```

Python 2 could simulate this by storing the variable in a mutable object, such as a single-element list.

Recursion has a limit and Python raises `RecursionLimitExceeded` if it is reached.

## Lambdas

Lambda expressions take the form:

``` python
lambda params: expr
```

## Generators

A generator is a function (aka generator function) that contains one or more `yield` expressions. Calling a generator doesn't execute the function, but instead returns a special iterator object known as a _generator object_ which wraps the function body, its parameters and local variables, and the current point of execution.

Calling `next()` on a generator object causes the function body to execute from the current point up until the next `yield`, at which point the point of execution is saved. Calling `next()` again causes it to continue execution where it left off.

When the function body ends or executes a `return` statement, the iterator raises a `StopIteration` exception.

In Python 3, `return` statements can take an argument which is passed as an argument to the resulting `StopIteration`.

`yield` expressions take the form:

``` python
yield expr

# Equivalent to `yield None`
yield
```

When `send(value)` is called on a generator object, the value of the overall `yield` expression becomes that passed value, so that simply calling `next()` implies a value of `None`.

Generator functions are easy ways to build iterators, which are then often used to loop on with a `for` loop.

``` python
def updown(n):
  for x in range(1, n): yield x
  for x in range(n, 0, -1): yield x

for i in updown(3): print i
```

In Python 3, `yield from` can be used to yield values from an iterable expression. This also facilitates the use of generators as coroutines.

``` python
def updown(n):
  yield from range(1, n)
  yield from range(n, 0, -1)

for i in updown(3): print i
```

# Classes

Instantiating a class involves calling the class object as if it were a function. _Dunder methods_ (double underscore names) are methods that are surrounded by two underscores `__`, and are special methods that Python implicitly calls for various operations.

When an instance can't looks up and can't find an attribute within itself, it implicitly delegates to its class.

Classes are objects (values) handled like other objects.

A class' name is bound (or rebound) to the class object after the `class` statement finishes executing. The class body executes immediately and the class object does not exist until it finishes. The statement doesn't create any instance of the new class, but defines the set of attributes shared by all instances when they're created.

``` python
class ClassName(base-classes):
  statements
```

Optional base classes may be specified whose values must be class objects.

In Python 3, the base class list can include a named argument `metaclass=…` to set the class' metaclass. In Python 2, omitting base classes creates an old-style class, unless the `__metaclass__` attribute is defined, otherwise a new-style class can be created by specifying an explicit base class of at least `object`, which is already implicit in Python 3 (i.e. all classes are new-style classes), but may be explicitly specified for backward-compatibility.

The built-in function `issubclass(C1, C2)` returns `True` if `C1` extends `C2`.

An attribute of a class can itself be another class, so `class` statements may be nested.

Attributes of a class object are typically specified by binding a value to an identifier within the class body, but they may also be bound outside of the class body. All instances share all of the class attributes.

``` python
class C:
  x = 1

C.y = 2

assert C.x == 1
assert C.y == 2
```

In statements directly within the class body, class attributes must use a simple, unqualified name, but in statements within methods of a class body, class attributes must use a fully qualified name.

``` python
class C:
  x = 25
  y = x + 1

  def method(self):
    print(C.x)
```

The `class` statement implicitly sets some class attributes.

| Name        | Purpose                                             |
| :---        | :------                                             |
| `__name__`  | class name as a string                              |
| `__bases__` | tuple of class objects of bases                     |
| `__dict__`  | dict object used to hold attributes (its namespace) |

Methods require an explicit first parameter referring to the instance object, conventionally named `self`.

``` python
class C:
  def hello(self):
    print('Hello')
```

Identifiers within a class body prefixed with two underscores such as `__ident`  are implicitly prefixed with `__classname` by the Python compiler, into `__classname__ident`. This simulates "private" names for attributes, methods, and global variables.

Identifiers prefixed with a single underscore `_` are meant to be private to the scope that bound them, whether or not it's a class, and these identifiers aren't manipulated by the Python compiler.

A descriptor is an object whose class supplies a special method name `__get__`. When descriptors are used as class attributes on a class, they control the semantics of accessing and setting attributes on instances of that class. When accessing an instance attribute, Python gets its value by calling `__get__` on the descriptor.

``` python
class Const:
  def __init__(self, value): self.value = value
  def __set__(self, *_): pass
  def __get__(self, *_): return self.value

class C:
  c = Const(23)

x = X()

assert x.c == 23
x.c = 1
assert x.c == 23
```

An overriding descriptor (aka data descriptor) is a descriptor that also defines `__set__`. A descriptor that only defines `__get__` (as above) is known as a nonoverriding descriptor (aka nondata descriptor). When assigning to an instance attribute, Python sets its value by calling `__set__` on the descriptor.

Note that descriptors are _class_ attributes which affect the behavior of _instance_ attributes.

## Instances

The built-in function `isinstance(i, C)` returns `True` if `i` is an instance of class `C`.

Calling the class object implicitly calls the `__init__` method on the new instance to perform per-instance initialization. Its main purpose is to bind and create the attributes of a newly created instance.

An instance object can be given an arbitrary binding outside of the class body.

``` python
class C: pass

c = C()
c.x = 1

assert c.x == 1
```

The special method `__setattr__` intercepts every attempt to bind an attribute.

When attempting to bind to an attribute whose name corresponds to an overriding descriptor, the descriptor's `__set__` method is invoked.

``` python
# Where C.x is an overriding descriptor:
c = C()
c.x = 1

# Equivalent to:
# Note this uses type(x) to account for subclass.
type(c).x.__set__(c, 1)
```

Creating an instance implicitly creates two instance attributes.

| Name        | Purpose                            |
| :--         | :--                                |
| `__class__` | class object it's an instance of   |
| `__dict__`  | instance's attribute namespace map |

There is no difference between instance attributes created by assigning to attributes directly from those created through the `__dict__` map.

``` python
x.a = 1
x.__dict__['b'] = 1

assert x.a == x.b
```

When a class object is called, Python first calls the `__new__` class and uses the return value as the newly created instance, on which it then calls `__init__` _only_ when the instance returned by `__new__` is indeed an instance or instance of a subclass of the class object on which `__new__` was called.

``` python
# Calling a class object:
instance = C(*args, **kwds)

# Equivalent to:
instance = C.__new__(C, *args, **kwds)

# Note this uses type(x) to account for subclass.
if isinstance(instance, C):
  type(instance).__init__(instance, *args, **kwds)
```

The base definition of `object.__new__` simply creates a new, uninitialized instance of the class it receives as its first argument.

A definition of `__new__` may choose to return an existing instance instead of returning a new one. For example, this `Singleton` class causes any derived classes to have only one instance by overriding the `__new__` method to only ever create a new instance if one hasn't been created yet.

``` python
class Singleton:
  _singletons = {}

  def __new__(cls, *args, **kwds):
    if cls not in cls._singletons:
      cls._singletons[cls] = super(Singleton, cls).__new__(cls)

    return cls._singletons[cls]
```

Methods are also attributes. Special dunder-name attributes cannot be unbound.

## Class Attribute Lookup

When getting an attribute from a class with the syntax `C.name`, the lookup process is:

1. If `name` is a key in the `__dict__`, get its associated value.

    Then if the value is a descriptor which defines `__get__`, the returned value of `C.name` is the result of calling:

    ``` python
    type(v).__get__(v, None, C)
    ```

    Otherwise the value is the one obtained from the `__dict__`.

2. Otherwise delegate lookup to `C`'s base classes and restart the lookup process there.

## Instance Attribute Lookup

When getting an attribute from an instance with the syntax `c.name` on an instance of class `C`, the lookup process is:

1. If `name` is found in the class `C` or one of its ancestor classes as per [class attribute lookup] is the name of an overriding descriptor `v` that defines `__get__` and `__set__`, the value is the result of:

    ``` python
    type(v).__get__(v, c, C)
    ```

2. Otherwise if `name` is a key in the instance attribute `__dict__`, then return the associated value.

3. Otherwise `c.name` delegates the lookup to `c`'s class as per [class attribute lookup].

[class attribute lookup]: #class-attribute-lookup

If the lookup doesn't find an attribute, Python raises an `AttributeError` exception. However, if the class defines or inherits `__getattr__`, Python instead calls `C.__getattr__(c, 'name')` instead of raising the exception, which itself may return a value or raise an exception.

## Attribute Bind Lookup

The aforementioned lookup processes only refer to getting the value of an attribute. Setting an attribute only affects the `__dict__` entry for the attribute in a class or instance. It involves no lookup process.

## Bound and Unbound Methods

A function object's `__get__` method returns either an unbound method object in Python 2 or the function object itself in Python 3, or a bound method object that wraps the function. An unbound method object is one that is not associated with any particular instance; it's only available in Python 2.

A bound method is obtained when a method attribute reference is on an instance, and an unbound method (function objects in Python 3) is obtained when the method attribute reference is on a class.

Unbound methods can be used to access overridden methods, although `super()` should be preferred. Unbound methods can also be used for higher-order functions.

When instance attribute lookup finds a function attribute that's an attribute in the instance's class, the lookup calls the function's `__get__` method which creates and returns a bound method that wraps the function. Note that this doesn't occur when the attribute lookup finds a function object in the `__dict__`, since Python doesn't treat the function as a descriptor, so doesn't call its `__get__` method. This also applies to built-in functions since they're not descriptors.

``` python
func.__get__(c, C)
```

A bound method has three read-only attributes in addition to those of the function object it wraps.

| Name       | Purpose                            |
| :---       | :---                               |
| `im_class` | class object supplying the method  |
| `im_func`  | wrapped function                   |
| `im_self`  | instance used to obtain the method |

Calls to a bound method do not explicitly supply the first parameter `self`, as that is obtained from the `im_self` attribute.

In the following example, the lookup process is:

1. check if `name` is an overriding descriptor (defines `__set__`)
2. check if `name` is in `c.__dict__`
3. notice that `f` is a descriptor, so calls `f.__get__(c, C)` thereby creating a bound method object with:

    * `im_func` set to `f`
    * `im_class` set to `C`
    * `im_self` set to `c`

4. calls bound method object with `arg` as the only argument
5. bound method object calls `im_func` with arguments `im_self` and `arg`
6. equivalent to calling:

    ``` python
    x.__class__.__dict__['name'](x, arg)
    ```

``` python
def f(a, b): pass

class C:
  name = f

c = C()

c.name(1, 2)
```

Bound method objects can be used wherever a callable object can, and is a flexible alternative to a closure.

``` python
def make_adder_as_closure(augend):
  def add(addend, _augend=augend): return addend + _augend
  return add

def make_adder_as_bound_method(augend):
  class Adder:
    def __init__(self, augend): self.augend = augend
    def add(self, addend): return addend + self.augend

  return Adder(augend).add

def make_adder_as_callable_instance(augend):
  class Adder:
    def __init__(self, augend): self.augend = augend
    def __call__(self, addend): return addend + self.augend

  return Adder(augend)
```

## Inheritance

When an attribute is not a key in a class' `__dict__`, attribute lookup implicitly proceeds on each class object in the `__bases__` tuple in method resolution order (MRO).

In method resolution order, each ancestor class is visited in left-to-right, depth-first order. In the presence of multiple inheritance, MRO causes only the right-most occurrence of a given class to remain.

A built-in read-only attribute called `__mro__` is a tuple of the types used for method resolution, in MRO.

A subclass method definition can delegate to a superclass' definition using a function object (unbound method in Python 2).

``` python
class Base:
  def greet(self, name): print('Welcome', name)

class Sub:
  def greet(self, name):
    print('Well Met and', end=' ')
    Base.greet(self, name)

x = Sub()
x.greet('Alex')
```

Method delegation is also common with `__init__` in order for base classes to perform their initialization. A base class' `__init__` should never be called if that's the only thing in the subclass' definition, since that would already occur by inheriting the base class definition.

``` python
def Base:
  def __init__(self):
    self.attr = 1

class Derived:
  def __init__(self):
    Base.__init__(self)
    self.attr = 2
```

Using explicit superclass method delegation can end up calling the same method multiple times in the case of multiple inheritance.

``` python
class A:
  def met(self): print('A.met')

class B(A):
  def met(self):
    print('B.met')
    A.met(self)
    # super(B, self).met()

class C(A):
  def met(self):
    print('C.met')
    A.met(self)
    # super(C, self).met()

class D(B, C):
  # NOTE
  # This calls A.met() twice through B and C's met()
  def met(self):
    print('D.met')
    B.met(self)
    C.met(self)
    # super(D, self).met()
```

Multiple calls to the same method due to multiple inheritance can be avoided by using the `super()` built-in. In Python 2, `super(class, obj)` returns a special superobject of `obj` such that attribute lookup on it begins _after_ the class `class` in `obj`'s MRO. In Python 3, `super()` with no arguments works the same way.

``` python
class A:
  def met(self): print('A.met')

class B(A):
  def met(self):
    print('B.met')
    super(B, self).met()

class C(A):
  def met(self):
    print('C.met')
    super(C, self).met()

class D(B, C):
  def met(self):
    print('D.met')
    super(D, self).met()
```

Explicitly calling superclass methods through function objects can still be useful when calling methods with different and incompatible signatures but same name.

Inheritance doesn't provide a way to delete or hide a base class' attributes noninvasively. Workarounds include overriding a method and raising an exception, hiding attributes and defining `__getattr__` for selective delegation, or overriding `__getattribute__`.

## Static Methods

A static method can be called on a class _or_ any instance of the class without special behavior with regard to the first parameter `self`.

A static method can be created by calling the built-in type `staticmethod()` and binding its result to a class attribute, or by using it as a decorator `@staticmethod`.

``` python
class C:
  def astatic():
    return 'static method'

  astatic = staticmethod(astatic)

  # Or:
  @staticmethod
  def other():
    return 'static method'

c = C()
assert C.astatic() == c.astatic()
```

