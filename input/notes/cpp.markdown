---
title = "C++"
published = "September 10, 2013"
excerpt = "Keeping track of C++"
comments = false
---

A lot of people really dislike C++ because it's a very complex language that often catches one by surprise. Despite this, C++ is undisputed when it comes to striking a balance between abstraction and speed. Those that need to use it for these reasons generally take one of two approaches, while the rest completely dismiss it as an option to begin with.

The first consists of restricting its usage to a specific subset of the language specification; for example, exceptions are generally avoided.

The other approach, perhaps an extreme, is for people to become "language lawyers," poring over the language specification for every minute detail.

I try to take a pragmatic approach. I do appreciate C++'s advantage in striking a balance between speed and abstraction, I do limit my use of it to a certain subset of the language, and I do try to learn as much about the language short of actually reading the specification to lower the probability that the language may catch me off guard.

To that end, these are non-exhaustive notes about C++---particularly the trickier bits---including C++11 and C++14 changes.

For C++11 in particular, some compilers are faster than others at adopting the new feature set. [Visual Studio](http://msdn.microsoft.com/en-us/library/vstudio/hh567368%28v=vs.120%29.aspx) is particularly behind the rest, while [Clang](http://clang.llvm.org/cxx_status.html) and [GCC](http://gcc.gnu.org/projects/cxx0x.html) seem to be very quick on adopting the new features.

<toc/>

# Type Conversions

## Signed-to-Unsigned

When a signed value is assigned to an unsigned variable, the underlying bit representation **is not altered**. Instead, the signed value is simply treated literally as if it were an unsigned value.

If the signed value is negative, then it is likely represented at the bit-level in [Two's Complement](http://en.wikipedia.org/wiki/Two%27s_complement). For example, given:

``` cpp
uint8_t var = -1;
```

The value `-1` is encoded by first representing it as a positive number:

$$ 0000\ 0001 $$

The digits are then flipped, so that 1s become 0s and vice versa:

$$ 1111\ 1110 $$

Finally, the value is incremented by 1 to arrive at the Two's Complement representation of `-1`:

$$ 1111\ 1111 $$

When this value is assigned to an unsigned integer, the value is simply interpreted as if it were unsigned to begin with. Therefore, this value is interpreted as being `255`.

## Implicit Conversions

The compiler automatically converts operands in the following circumstances:

* usually integral types smaller than `int` are first promoted to an appropriate larger integral type (presumably to the smallest working unit, i.e. the size of a register on a computer?)
* in **conditions**, non-`bool` expressions are converted to `bool`
* in **initializations**, initializer is converted to the type of the variable
* in **arithmetic and relational expressions** with operands of mixed types, the types are converted to a common type
* during certain **function calls**

### Integer Promotion

In general, operands are converted to the same type of the widest operand type in the expression. Loss of precision is avoided, so this also means that when integral and floating-point values are mixed, they're all converted to floating-point values.

**Integer promotion** concerns converting small integral types to larger integral types.

`bool`, `char`, `signed char`, `unsigned char`, `short`, `unsigned short` are promoted to `int` if all possible values fit within an `int`. Otherwise, they are promoted to `unsigned int`.

Larger types are promoted to the smallest type of `int`, `unsigned int`, `long`, `unsigned long`, `long long`, or `unsigned long long` which fits the value.

### Mixing Unsigned Types

If the types still don't match but the signs match, then the type of the smaller value is promoted to the type of the larger value.

If the signs don't match and the type of the unsigned operand is the same as or larger than that of the signed operand, then the signed operand is converted to unsigned as described in [Signed-to-Unsigned](#signed-to-unsigned), which most likely yields unexpected behavior.

If the signs don't match and the type of the unsigned operand is smaller than that of the signed operand, the **result is machine-dependent**. If all values in the unsigned type fit in the larger signed type, it's converted to the larger signed type. Otherwise, the signed operand is converted to the unsigned type as described in [Signed-to-Unsigned](#signed-to-unsigned), which most likely yields unexpected behavior.

## Negative Modulus

The modulus operation `%` simply calculates the remainder of the left expression divided by the right expression. There is confusion when it comes to modulus operations with negative operands, which as far as I know isn't clearly defined mathematically. For example, the operation `-1 % 256`.

The equation generally used to calculate the modulus is:

$$ \text{mod}(a, n) = a - \lfloor a / n \rfloor * n $$

The operation `-1 % 256` yields the result `255` with this implementation. This is the result yielded in languages such as Python and Ruby.

C and C++ uses the same equation as the above, **but** the division operation has an additional restriction when used with negative operands:

$$ \text{div}(-a, n) = \text{div}(a, -n) = -(a/n) $$

With these definitions, the division of `-1 / 256` in the above equation becomes `-(1 / 256)`. The result of `1 / 256` is zero due to truncation. The negation of this result is still zero, so the result of the modulus operation is simply `-1`, which is **very different** from the result of `256` yielded above without these restrictions.

Given the above restriction on the division operation with negative operands, the definition of the modulus operation with negative operands can be simplified to:

$$
\begin{align}
  \text{mod}(\phantom {-} a, -n) &= \phantom {-} \text{mod}(a, n) \\
  \text{mod}(-a, \phantom {-} n) &= -\text{mod}(a, n)
\end{align}
$$

# Classes

It's a good thing to remember that the _only_ distinction between a `class` type and a `struct` type is that `struct` has by default public visibility and `class` has default private visibility. That's all!

## Rule of Five

The copy constructor, move constructor, copy-assignment operator, move-assignment operator, and destructor should be thought of as a unit: if one needs to be defined, then the rest should be defined as well.

* if a class needs a destructor, it likely also needs a copy-assignment operator and copy constructor
* if a class needs a copy constructor, it likely so needs a copy-assignment operator, **and vice versa**

## Rule of Zero

This [recent rule] is unlike the [other two] in that it instead says that classes that contain custom destructors, copy/move constructors, or copy/move assignment operators should deal _exclusively_ with ownership, i.e. encapsulating a so called _ownership policy_ which handles the allocation and deallocation of a particular resource (via RAII). All other classes should **not have** custom destructors, copy/move constructors, or copy/move assignment operators.

[recent rule]: http://flamingdangerzone.com/cxx11/2012/08/15/rule-of-zero.html
[other two]: http://en.cppreference.com/w/cpp/language/rule_of_three

*[RAII]: Resource Allocation Is Initialization

This rule is enforceable out-of-the-box in C++11 through the use of smart pointers such as `shared_ptr` and `unique_ptr` along with custom deleters when necessary.

## Class Initialization

Classes are initialized as follows:

1. virtual base classes in depth-first, left-to-right order
2. direct base classes in left-to-right order
3. in-class initializers top-to-bottom
4. constructor initializer lists in top-to-bottom member definition order
5. constructor body initialization

## Member Initialization

The order of initializing member variables is:

1. in-class initializers
2. constructor initializer lists in top-to-bottom member definition order
3. constructor body initialization

Constructor initializer lists initialize member variables. If a member variable is missing from the initializer list it is default initialized. Members that are `const` or references must be initialized in the constructor initializer lists. Members in a constructor initializer list are initialized in the order in which they are defined in the class definition.

It is considered best practice to use in-class initializers for member variables, opting for constructor initializer lists for edge cases, and for constructor initialization in the worst case.

Value initialization occurs when:

* in an array initialization, fewer declarations appear than the size of the array
* defining a local static object without an initializer
* explicitly requesting value initialization by writing expressions of the form `T()` where `T` is the name of the type

Member functions defined inside the class definition are inlined.

## Copy-Initialization

Copy-initialization occurs when:

* Assigning to a new object with the equals sign (not assignment operator):

    ``` cpp
    T object = other;
    ```

* Passing a parameter by-value:

    ``` cpp
    f(other);
    ```

* Returning a by-value:

    ``` cpp
    return other;
    ```

* Throwing or catching by-value:

    ``` cpp
    throw object;

    catch (T object) { … }
    ```

* Placing an object in a brace-initializer:

    ``` cpp
    T array[N] = {other};
    ```

## Default Constructors

The best practice is to always define a default constructor if any other constructors are defined.

Default constructors are synthesized only if all of the following criteria are met:

1. no other constructors are defined
2. all of the members of built-in or compound type have in-class initializers
3. all members of class type have default constructors

If other constructors are defined but otherwise all other criteria is met for synthesizing a default constructor, the default constructor can be constructed using the `= default` directive:

``` cpp
class A {
  A() = default;
  A(int a, int b);
};
```

Class members can be initialized inside the class definition. These initializers are known as _in-class initializers_. In-class initializers must be defined either using the `=` assignment operator or list initialization syntax `{}`.

Constructors can _delegate_ their constructing to other constructors inside the constructor initializer list.

Virtual functions can be explicitly overridden in derived classes using the `override` trailing keyword.

Class methods or entire classes can be defined `final` which prevents their overriding or deriving, respectively.

## Destructors

Destructors do whatever work must be done to free resources used by an object, e.g. file handles. While in constructors the members are initialized before the constructor body runs, a destructor body's body executes first and then the members are destroyed afterward, in the reverse order of declaration in the class definition.

## Copy Constructors

A copy constructor is one consisting of a single parameter that is a reference to the same type of the constructor:

``` cpp
struct A {
  A(const A&);
};
```

Copy constructors are _synthesized_ if none are defined. Synthesized copy constructors perform member-wise copies of the argument. Members of class type are copied using their respective copy constructors and members of built-in type---including arrays---are copied directly.

``` cpp
A::A(const A& toCopy) :
  firstMember(toCopy.firstMember),
  secondMember(toCopy.secondMember) {}
```

_Copy initialization_ occurs when:

* assigning with the `=` assignment operator to a new object
* passing the object as an argument to parameter of non-reference type. **note** that this is why the parameter to the copy constructor has to be a reference type, or infinite recursion would occur
* returning by value
* placing in a brace initializer

The compiler can perform [copy elision](http://en.wikipedia.org/wiki/Copy_elision) to avoid unnecessary copies, short of using actual move semantics.

## Copy-Assignment Operators

Assignment operators control how objects of its class are assigned. They generally should return a reference to the left-hand object.

``` cpp
A& A::operator=(const A& rhs) {
  if (this != &rhs) {
    firstMember = rhs.firstMember;
    secondMember = rhs.secondMember;
  }

  return *this;
}
```

Copy-assignment operators are _synthesized_ if none are define. Synthesized copy-assignment operators perform member-wise assignment before returning a reference to the left-hand object.

_Copy-assignment_ occurs when an existing object is assigned a new value from another existing object.

## Conversion Constructors

Conversion constructors allow for the implicit conversion **from** other types to the class type. Only one such implicit conversion is possible; it isn't possible to chain multiple such conversions.

Such conversion constructors can be suppressed using the `explicit` keyword, which effectively only allows the direct form of initialization:

``` cpp
explicit A(std::string &str) : internal(str) {};
```

However, the `explicit` keyword still allows one to use an explicit conversion using a `static_cast`:

**VERIFY**: When defining a copy constructor in the above manner, it forces the compiler to always copy the string instead of being able to use move semantics. Instead, prefer to pass by value and then moving ([source](https://news.ycombinator.com/item?id=6398924)):

``` cpp
explicit A::A(std::string str) : internal(std::move(str)) {}
```

## Conversion Operators

Where as [conversion constructors](#conversion-constructors) provide a way of converting another type to the class type, conversion operators provide a way of converting the class type to another type. They are defined using the `operator` keyword followed by the type it converts to.

``` cpp
struct A {
  operator bool () const { return B; }
};
```

However, creating a `bool` conversion operator can cause unexpected results such as in the following:

``` cpp
int i = 42;
cin << i;
```

The above code is legal even though `<<` isn't defined for `cin` which is of type `istream`. The reason it's legal is that `cin` gets converted to `bool`, which then gets promoted to an `int`, after which the operation becomes a simple left-shift operation.

For this reason, conversion operators can be defined as explicit. A conversion operator that is defined as explicit won't be performed implicitly and instead it must be performed explicitly through the use of `static_cast`. The only exception to this is when the expression would be used for boolean logic.

``` cpp
struct A {
  explicit operator bool () const { return B; }
};
```

## Conversion Ambiguity

It's pretty easy to get into a situation where it becomes ambiguous as to how a type is being converted.

In general:

* don't define mutually converting classes
* avoid conversions to built-in arithmetic types. If this is necessary, then:
    * don't define overloaded versions of operators that take arithmetic types since the conversion will handle it
    * don't define a conversion for more than one arithmetic type

However, it's probably best to try to completely avoid conversion functions with the exception of explicit conversions to `bool` and others that are very obvious.

### Mutual Conversions

One way is to create a conversion constructor to a type that itself defines a conversion operator to the original type.

For example, given:

``` cpp
struct B;

struct A {
  A() = default;
  A(const B&);
};

struct B {
  operator A() const;
};
```

Both `A` and `B` define mutual conversions. `A` defines a conversion constructor that converts `B` to `A`, and `B` itself defines a conversion operator that converts from `B` to `A`. Therefore, the last line in the following code is ambiguous:

``` cpp
A f(const A&);
B b;
A a = f(b);
```

Because the conversion operation is ambiguous to the compiler, an error is emitted. Instead, it would have to be explicitly qualified:

``` cpp
A a1 = f(b.operator A()); // use B's conversion operator
A a2 = f(A(b));           // use A's conversion constructor
```

To avoid ambiguity, one should not define classes with mutual conversions.

### Redundant Built-In Conversions

Another way is to define multiple conversions to or from types that themselves are related by conversions.

For example, given:

``` cpp
struct A {
  A(int = 0);
  A(double);
  operator int () const;
  operator double () const;
};
```

Due to implicit integer promotion, the two conversions to and from `int` and `double` become ambiguous to the compiler:

``` cpp
void f2(long double);
A a;
f2(a);    // operator int () or operator double ()

long lg;
A a2(lg); // A(int) or A(double)
```

The calls above are ambiguous because `long -> double` and `long -> int` both have the same rank in terms of integral promotion. If instead the parameter had been of type `short` then the promotion of `short -> int` would have had a higher rank than `short -> double` and so that conversion would have been chosen by the compiler.

For this reason, one should not define more than one conversion to or from an arithmetic type.

## Delete

Functions can be specified as **deleted** which prevents the compiler from generating code for them. This can be helpful for preventing copying of a specific type:

``` cpp
struct NoCopy {
  NoCopy(const NoCopy&) = delete;
  NoCopy &operator=(const NoCopy&) = delete;
};
```

The compiler sometimes defines copy-control members, which it would have otherwise synthesized, as **deleted** for the following reasons:

* **destructor**: if a member has a deleted or inaccessible destructor, e.g. `private`
* **copy constructor**: if a member has a deleted or inaccessible copy constructor _or_ if a member has a deleted or inaccessible destructor
* **copy-assignment operator**: if a member has a deleted or inaccessible copy-assignment operator _or_ if the class has a `const` or reference member
* **default constructor**: if a member has a deleted or inaccessible destructor _or_ has a reference member without an in-class initializer _or_ has a `const` member whose type has no explicit default constructor and the member has no in-class initializer

## Swapping

Classes that allocate resources might want to define a `swap` inline friend function that simply swaps pointers around. This is useful for classes that allocate resources, and can be re-used in copy and move operations.

``` cpp
struct A {
  friend void swap(A&, A&);
private:
  SomeType *B;
};

inline void swap(A &lhs, A &rhs) {
  using std::swap;
  swap(lhs.B, rhs.B);
}
```

It's _very_ important to recognize that the `swap` function used isn't explicitly qualified to be from the `std` namespace. Instead, the `swap` function from the `std` namespace is brought into the scope for purposes of name resolution.

**Not** explicitly qualifying the function allows a type-specific `swap` function to be used in the event that one is defined, which would be much more efficient than using the `std` function which simply creates a temporary swap value.

One use of the `swap` function is to implement the assignment operator:

``` cpp
A& A::operator=(A rhs) {
  swap(*this, rhs);
  return *this;
}
```

It's important to note that this implementation passes the right-hand side by value and not by reference. This is done so that after the type internals are swapped, the right-hand side's copy's destructor is run and the resources are freed. This handles self-assignment gracefully.

## Inheritance

Constructors, copy and move operations, and assignment operations all have to handle initializing not only their members but also those of the base class. This is usually accomplished by delegating that work to the equivalent operation from the base class.

_However_, a destructor is always only in charge of destroying only its own members. The base class destructor is implicitly invoked after the completion of the derived class destructor.

_Name lookup_ is affected by inheritance and virtual functions. Given a call `p->mem()` or `p.mem()`:

1. determine the static type of `p`
2. look for `mem` in the class that corresponds to the static type of `p`. If it's not found, continue the lookup up the inheritance hierarchy. Error if not found.
3. perform normal type checking (&sect; 6.1 p. 203) to see if the call is legal
4. if it's legal, generate code depending on whether the call is virtual:
    1. **virtual**: if the call is made through a reference or pointer, then generate code to determine at run-time which version to run based on the dynamic type of `p`
    2. **otherwise**: if the call isn't virtual or made through a reference or pointer, then generate a normal function call

Inheritance can be prevented by a class using the `final` directive:

``` cpp
class A final {};
```

This directive can also be used on specific member functions:

``` cpp
struct A {
  void Perform() final;
};
```

An abstract base class is one that contains a pure abstract method, which is one that _must_ be implemented by children.

``` cpp
class T {
  virtual void func() = 0;
};
```

### Constructors

Constructors of derived classes can't directly initialize base-class members. Instead, initialization is delegated to the base-class constructor:

``` cpp
B(const std::string& str, int num, char ltr) :
  A(str, num), ltr_(ltr) {}
```

If the base-class is not initialized in this manner, then the base-class is default initialized.

### Inherited Constructors

It's possible to "inherit" constructors from the base class:

``` cpp
struct B : public A {
  using A::A;
};
```

The `using` directive causes `B` to "inherit" _each_ of the constructors from `A` except:

1. the default, copy, and move constructors
2. those which have the same parameter lists as one of the constructors already defined in the derived class

Despite the first exception above, the inherited constructors aren't considered to be "user defined" and so the compiler can still synthesize the default, copy, and move constructors if allowed.

The inherited constructors have the exact same properties as defined in the base class, including accessibility, `explicit`, and `constexpr`.

### Copy and Move Operations

If a derived class defines a copy or move operation, then it is responsible for copy or moving the entire object including base-class members. This is accomplished similar to what a regular does by delegating the work to the equivalent constructor in the base class.

### Copy-Assignment Operator

As with the constructor and copy/move operations, the copy-assignment operator can delegate its work to the copy-assignment operator of the base class:

``` cpp
B& B::operator=(const B& rhs) {
  // delegate to base copy
  A::operator=(rhs);

  // assign members of derived class
  return *this;
}
```

### Destructors

Base classes that intend to be derived from should define their constructors as `virtual`, so that correct destructor is run through dynamic dispatch based on the dynamic type of the object being destroyed, instead of the static type.

This has an implication with move semantics. If a destructor is defined, even as `default`, then no move operations are synthesized for that class. This issue percolates throughout the inheritance hierarchy, since classes don't synthesize operations that the base class doesn't define.

For this reason, the base class usually explicitly defines---even if as `default`---all of the operations it requires. First the virtual destructor for the aforementioned reasons, then move operations for the aforementioned reasons, and then the copy operations since they would otherwise not be synthesized since the move operations are explicitly defined.

# Move Semantics

C++11 introduced _move semantics_ which simply refers to recognizing the notion of moving objects instead of only being able to copy them. With this introduction came _rvalue-references_ which designate an object as being "moveable," usually because it's about to be destroyed anyways.

A simple explanation for the act of "moving" is that of a string class with an underlying `char` array. If there is an instance **A** that needs to be replicated into instance **B**, it can be done by copying **A** into **B** using a copy constructor which would make a copy of the underlying array. However, if **A** was going to be destroyed shortly after, then the copy would have been unnecessary. Instead of copying the array from **A**, it could simply _steal_ its pointer.

## rvalue-references

_rvalue-references_ are simply references that can _only_ be bound to rvalues. rvalues are either temporary objects or literals, both of which are ephemeral over the course of evaluating an expression. It then follows naturally that an object bound to an rvalue-reference has no "owner", and more importantly that the object is _about to be destroyed_, **so code is free to steal its contents**. rvalue-references are simply a way of "tagging" such objects, to be able to write functions that apply specifically to objects that are about to be destroyed, i.e. a move constructor.

Aside from binding rvalue-references to rvalues, it is possible to derive an rvalue-reference from an lvalue through the use of `static_cast`. Such a cast has been implemented as the function `std::move` in order to be more semantic:

``` cpp
Object &&ref = std::move(instance);
```

However, deriving an rvalue-reference from an lvalue is seen as a promise that the lvalue will no longer be used other than to assign or destroy it, as the actual value of the lvalue is not well defined or guaranteed.

### Reference Collapsing

rvalue-references to template parameters have special rules. For example, given the definition:

``` cpp
template <typename T> void func(T&&);
```

If an lvalue `int` is passed to the function, a language rule states that the template parameter `T` will be deduced as being an lvalue-refernece, `int&`. This poses a problem, since the function parameter's type ends up being an lvalue-reference to an rvalue-reference, `int& &&`. A reference to a reference, of any type, can't usually be created but an **exception** is made for template parameters.

Template parameters that are deduced as being references to references undergo a process that is referred to as _reference collapsing_, the rules of which are as follows:

* `X& &`, `X& &&`, `X&& &` &rarr; `X&`
* `X&& &&` &rarr; `X&&`

Basically, all reference-to-reference instances collapse to lvalue-references, unless an actual rvalue-reference was what the template parameter `T` was deduced to.

The consequence of this is that function parameters that are an rvalue-reference to a template parameter type can match _any_ type.

This is the mechanism behind the `std::move` function, which is defined by the standard as:

``` cpp
template <typename T>
typename remove_reference<T>::type&& move(T&& t) {
  return static_cast<typename remove_reference<T>::type&&>(t);
}
```

This has the effect that rvalues are passed through as-is. Instead, when an lvalue is passed to `std::move`, the templated function is instantiated as follows:

1. `T` type deduces to `string&`
2. `remove_reference` is instantiated with `string&`
3. `remove_reference<string&>::type` is `string`
4. return type of `move` is therefore `string&&`
5. function parameter instantiates as `string& &&` which collapses to `string&`

The above instantiation procedure yields the following function signature:

``` cpp
string&& move(string &str);
```

The actual `static_cast` is what yields and returns an rvalue-reference.

### Type-Matching

An rvalue-reference can be converted to a `const` reference. This means that if a class defines copy constructor but not a move constructor and as a result the compiler [defines the move constructor as deleted](#move-operation-synthesis), rvalue-references will type match with `const` references and as a result, rvalue-reference arguments will use the copy constructor seamlessly.

### Reference Qualifiers

It's usually the case that member functions can be called on objects regardless of whether they're lvalues or rvalues. However, this can lead to unexpected usage of objects such as the following:

``` cpp
s1 + s2 = "wow!";
```

C++11 allows for the explicit restriction on the usage of a member function based on the lvalue/rvalue property of the calling object using a _reference qualifier_, which is similar to a `const` qualifier in that it appears at the end of the parameter list but *_after_* the `const` qualifier, and must appear in both the declaration and definition of the function.

Two possible reference qualifiers exist:

1. `&` can only be called from an lvalue
2. `&&` can only be called from an rvalue

**Note**: If a function has a reference qualifier, than _all_ of the same functions require a reference qualifier.

``` cpp
struct A {
  A& operator=(const A&) &;
};

A& A::operator=(const A &rhs) & {
  return *this;
}
```

## Move Constructors

Because rvalue-references serve as a sort of "tag" on an object that's about to be destroyed, functions can overload implementations specifically for such objects. An example of this would be a move constructor:

``` cpp
A::A(A &&moveFrom) noexcept :
  firstMember(moveFrom.firstMember),
  secondMember(moveFrom.secondMember) {
  moveFrom.firstMember = moveFrom.secondMember = nullptr;
  }
```

It's important to leave the moved-from object in a destructible state.

## Move-Assignment Operator

This is similar to the move constructor:

``` cpp
A& A::operator=(A&& rhs) noexcept {
  if (this != &rhs) {
    delete firstMember;
    firstMember = rhs.firstMember;
    rhs.firstMember = nullptr;
  }

  return *this;
}
```

An interesting thing to note is that the move-assignment operator can be defined in terms of the copy-assignment operator if a move constructor is defined:

``` cpp
struct A {
  A(A &&other) noexcept : B(other.B) { other.B = nullptr; }
  A& operator=(A rhs) {
    swap (*this, rhs);
    return *this;
  }
};
```

In this case, if an rvalue-reference is used with the assignment operator, then the `rhs` variable is created using the move-constructor which simply allows `rhs` to steal the `B` pointer from the rvalue. Once inside the assignment operator function body, the current instance steals the `B` pointer from the `rhs` copy. The `rhs` copy is automatically destroyed when it goes out of scope.

## Synthesis

Unlike the copy operations that are _always_ synthesized if they're not otherwise defined or deleted, the compiler _only_ synthesizes move operations if the class doesn't define any copy operations and if every non-static data member is moveable. Moveable members include built-in types and those that define a move operation.

If a class defines move operations, the respective copy operation will be defined as deleted and must be defined explicitly.

If a default implementation is explicitly requested with the `default` directive, but the compiler can't define one due to the following reasons, then it will be defined as `deleted`:

* the class has a member that defines its own copy constructor but not a move constructor _or_ if the class has a member that doesn't define its own copy operations _and_ for which the compiler is unable to synthesize a move constructor. The same applies for move-assignment.
* the class has a member whose respective move operation is deleted or inaccessible
* the destructor is deleted or inaccessible
* the class has a `const` or reference member

## Exception Guarantees

Some classes make guarantees about what occurs in the event that exceptions are thrown. For example, `std::vector` guarantees that if an exception occurs during `push_back`, the original `vector` would be left unchanged. In the event that the `push_back` would have had to reallocate space, if the `vector` decided to use the move constructor to move the objects to the new space and an exception were thrown at some point, the original `vector` would be left in an inconsistent state, with some of its elements having been moved to the new allocation of memory.

For this reason, such classes use copy constructors unless they are guaranteed that a type's move constructor doesn't throw exceptions. This guarantee is specified using the `noexcept` declaration on a function definition as shown above.

# Miscellaneous

`static_assert` is a compile-time assertion.

The **type_traits** header defines a variety of type trait queries.

The `auto` keyword allows for type-deduction and should be preferred in the following circumstances:

* when an expression would otherwise be repeated on both sides
* lambdas, though can also use `std::function`
* iterators and other long type names

The `decltype` operator can deduce and "return" the type of the argument to be used to declare something else such as a variable of a function. The rules for what gets returned depends on the expression:

* **identifier** (name of object or function) or **class member access**, _yieds_ type of identifier or class member access
* **parenthesized identifier** becomes an lvalue expresion, _yields_ lvalue reference to type of expression
* **function call**, _yields_ return type of the function
* **rvalue**, _yields_ rvalue reference to type of expression
* **lvalue**, _yields_ lvalue reference to type of expression

The suffix-return syntax is useful when the return type is deduced from information---such as the function arguments---and has to appear after the function argument list so that the arguments are "in scope":

``` cpp
template <class T, class U>
auto add(T x, U y) -> decltype(x + y) {
  return x + y;
}
```

Suffix-return syntax can also be useful in class methods in classes with nested types. Given the following class:

``` cpp
struct LL {
  struct Link {};
  Link *erase(Link *p);
};
```

Given the following declaration:

``` cpp
LL::Link *LL::erase(Link *p) {};
```

Using suffix-return syntax, after the compiler reads `LL::erase` it enters the class scope of `LL`, making it unnecessary to fully qualify the `Link` type that's nested within `LL`:

``` cpp
auto LL::erase(Link *p) -> Link * {};
```

The `std::function` type is a generalized type that "matches" any kind of function-like type, such as an actual function pointer, lambdas, function objects, etc.

## User-Defined Literals

User-defined literals can easily be created:

``` cpp
Out operator "" _intlit(int literal);
Out operator "" _strlit(const char * literal);
Out someVar = 1234_intlit;
Out otherVar = "testing"_strlit;
```

## Enumerations

_Scoped enumerations_ can be created to avoid symbol clashing and enumerations' underlying type can be specified explicitly:

``` cpp
enum class EventType : uint8_t { STATUS, LOG, ERROR };

EventType type = EventType::STATUS;
```

# Standard Template Library

## Initializer Lists

The `std::initializer_list` type is a lightweight proxy object wrapping an array of objects of type `const T`. For this reason, it's normal and expected to pass it around by-value, since it's already essentially a pointer to the underlying array.

## integral_constant

The `std::integral_constant<T, T v>` type from `type_traits` takes an integral type and a constant value for it. Two typedefs for these exist which are `true_type` (i.e. `std::integral_constant<bool, true>`) and `false_type`. That can be used to refine the selection of a function overload.

``` cpp
// integrals
template<typename T>
void foo_impl(T val, true_type);

// floats
template<typename T>
void foo_impl(T val, false_type);

// Use is_integral to select the appropriate overload.
template<typename T>
void foo(T val) {
 foo_impl(val, std::is_integral<T>());
}
```

## Array

The `std::array<N, T>` type is a wrapper around regular `T[N]` arrays. It provides typical STL collection functionality such as iterators and copy and assignment operators. It has no user-provided constructors, no base classes, no virtual member functions, no in-class initializers, and only contains a regular public array. This means that `std::array` is an aggregate type, which allows it to be initialized like a regular array, via aggregate-initialization:

``` cpp
std::array<3, int> numbers = {1, 2, 3};
```

## Pairs

An `std::pair` is essentially a tuple with two components, accessible via members `first` and `second`. It is used to denote, for example, key-value pairs in maps.

Pairs can be constructed using list-initialization when the type can be inferred or explicitly with the helper free-function `std:make_pair`:

``` cpp
auto keyval = std::make_pair(42, "test");
```

When emplacing a new key-value pair into a map, it can become ambiguous as to which parameters correspond to which constructor: either the key's or the value's.

``` cpp
std::map<std::string, std::complex<float>> map;

// "key" should be the parameter for the string constructor
// 3.4, 7.8 should be the parameters for the complex constructor
map.emplace("key", 3.4, 7.8);
```

To disambiguate these situations, there is `std::piecewise_construct` which can forward the respective pair components as a tuple:

``` cpp
map.emplace(std::piecewise_construct,
            // used to construct the first pair component
            std::forward_as_tuple("key"),
            // used to construct the second pair component
            std::forward_as_tuple(3.4, 7.8));
```

## Tuples

The `std::tuple` type is similar to tuples in other languages. `get<index>(tuple)` retrieves the value at a given index. Tuples can easily be created with the `make_tuple` function.

Note that tuples cannot be copy-list-initialized because the corresponding constructor is marked explicit in order to prevent a single value from being implicitly converted to a tuple.

``` cpp
std::tuple<int, double> wrong = {3, 3.14}; // error
std::tuple<int, double> correct{3, 3.14};  // ok
```

As in other languages, tuples can be "unpacked" into multiple values using the `tie` function:

``` cpp
tie(num, std::ignore, letter) = make_tuple(10, 4.23, 'a');
```

The `std::tie` function works by essentially creating an ephemeral tuple of references and copy-assigning the source tuple to the ephemeral tuple, causing those values to be set via reference.

``` cpp
std::tuple<int, string> source(3, "test");

int first;
std::string second;

std::make_tuple(std::ref(first), std::ref(second)) = source;

EXPECT_EQ(3, first);
EXPECT_EQ("test", second);
```

Tuples implement comparison operators to perform lexicographical comparisons, this makes it possible to use `tie` to perform lexicographical comparisons of multiple fields:

``` cpp
struct Record {
 std::string name;
 unsigned int floor;
 double weight;

 // First compare names to each other, and if equal, then floor, finally weight.
 friend bool operator<(const Record& l, const Record& r) {
  return std::tie(l.name, l.floor, l.weight)
       < std::tie(r.name, r.floor, r.weight);
 }
};
```

If a class defines a constructor that takes an `std::initializer_list` then that constructor takes precedence when using initializer list construction. Initializer lists cause an error if a construction would narrow a type.

## Unordered Containers

Custom hashers and key-equality functions can be used on a given unordered container.

``` cpp
struct S {
  int data;
};

auto hash_func = [](const T& t) {
    return hash<int>()(t.data);
};

auto equality = [](const T& lhs, const T &rhs) -> bool {
    return lhs.data == rhs.data;
};

unordered_map<T, string,
              decltype(hash_func),
              decltype(equality)> m(S{10}, hash_func, equality);
```

## Bitsets

Bitsets can be instantiated from an integer source or a string. They are parameterized by their capacity, i.e. they are not dynamic: their capacity must be stated up-front. For a dynamic bitset there exists an `std::vector<bool>` specialization.

The indexing operator `operator[]` is overloaded to test a bit, and there is also a method `test` to do the same.

There are `all`, `any`, and `none` methods, as well as `count` to see how many bits are set.

Bits can be turned on (set) via the index `operator[]` or via the `set` method, which can set all bits if no parameters are given, a specific bit, or can set a specific bit to an explicit value. The `reset` method does the opposite, turning off bits. The `flip` method negates bits.

## Strings

A constructor overload exists for creating a string consisting of a certain number of a single character.

Arbitrary strings or characters can be inserted anywhere into the string via the `insert` method, likewise they can be removed with `erase`.

Strings can be appended to via `push_back` (there's also `pop_back`) or `append` and `operator+=`, but an `std::stringstream` is preferred if there's going to be many append calls.

The `substr` function takes a starting position and a _count_, not one-past last, but a _count_, and returns the substring denoted by the range [begin, begin + count].

The `replace` function can replace a given range in the string with another string.

The `find` method searches for the occurrence of a needle in the string, optionally starting from a given position, and returns the starting position of the first match, or `std::string::npos` if there was none.

There is also `rfind` which does the same but starting from the right of the string.

The `find_first_of` method is like the algorithm of the same name: it looks for the first occurrence of one of the characters in the input string. There is also `find_first_not_of`, as well as `find_last_of` and `find_last_not_of` which search from right-to-left.

## Numeric Limits

The `limits` header contains numeric limits, such as `std::numeric_limits<int>::max()` for the largest representable number with the `int` type.

## Smart Pointers

Both `unique_ptr` and `shared_ptr` are explicit (or contextually, i.e. in an if-condition) convertible operators to `bool`, denoting whether the managed pointer is `nullptr` or not.

### unique_ptr

A `unique_ptr` can relinquish ownership of the managed object with the `release` method.

One `unique_ptr` can transfer ownership to another `unique_ptr` via move construction or assignment.

### shared_ptr

Each `shared_ptr` has an associated _control block_ which consists of:

* managed object or a pointer to it
* reference count
* weak reference count
* type-erased deleter
* type-erased allocator

Depending on the way a `shared_ptr` is constructed, there can be a potential for memory leaks. Imaging the following code:

``` cpp
processWidget(shared_ptr<Widget>(new Widget), dangerous());
```

The compiler can generate the above code as:

1. allocate `Widget`
2. call `dangerous`
3. construct `shared_ptr` from `Widget`

This can be a problem if the call to `dangerous` in step 2 throws an exception, because the `shared_ptr` didn't have a chance to take ownership of the `Widget`. If it had taken ownership, the exception would've destroyed the `shared_ptr` which also would've deallocated `Widget`. Otherwise the memory leaks.

For this reason, it's preferable to use `std::make_shared` to construct `shared_ptr`s. It's also faster because it is usually able to perform a single allocation for both the managed object and the `shared_ptr`'s control block, whereas when the managed object is explicitly allocated, the `shared_ptr` has no choice but to perform another allocation for the control block.

``` cpp
processWidget(make_shared<Widget>(), dangerous());
```

This way, the compiler now has two things that it may rearrange:

* call `make_shared`
* call `dangerous`

Regardless of their arrangement, there is no possibility for memory leaks.

However, the use of `std::make_shared` prevents specifying a custom deleter. For that, the regular constructor is necessary:

``` cpp
shared_ptr<c_type>(new_c_type(), free_c_type);
```

A `shared_ptr` can change the object it manages by calling its `reset` method with a pointer to the new object to manage. If the `shared_ptr` was the last owner of the previously managed object, its deleter is called.

``` cpp
shared_ptr<string> thing;
thing.reset(new std::string("other"));
```

The raw pointer that a `shared_ptr` manages can be accessed without relinquishing ownership via the `get` method.

The "alias constructor" of `shared_ptr` is one that accepts a raw pointer to manage and whose lifetime is tied to another `shared_ptr`.

``` cpp
struct X {
  int a;
};

shared_ptr<X> px(new X);
shared_ptr<X> pi(px, &px->a);

EXPECT_EQ(pi.get(), &px->a);
```

There are casting functions equivalent to `static_cast` et al which operate on `shared_ptr`:

* `static_pointer_cast`
* `dynamic_pointer_cast`
* `const_pointer_cast`

Sometimes it's necessary to be able to create a type which should be able to give `shared_ptr`s of itself (i.e. of its `this` pointer) to other types.

For example, this can be useful or even necessary when creating a tree where the nodes have pointers to their parents. If the children are wrapped in `shared_ptr`, then the parent pointers must also be `shared_ptr` so that all paths from which the pointers may be referenced go through a `shared_ptr`. However, this poses a problem because it would entail giving a child a copy of the parent's `this` pointer for it to store in its `shared_ptr` of its parent. What's really needed is a `shared_ptr` of the `this` pointer.

This can be achieved by having the class derive from `std::enable_shared_from_this<T>`, where `T` is the type in question. This inherits a method `shared_from_this` which returns a `shared_ptr` of the `this` pointer.

However, something has to have created a `shared_ptr` of the `this` pointer, i.e. of the class. For this reason, there is usually a factory method which constructs `shared_ptr`s of the class.

``` cpp
class Widget : public std::enable_shared_from_this<Widget> {
 public:
  template<typename... Ts>
  static std::shared_ptr<Widget> create(Ts&&... params);
};
```

To prevent reference-cycles, the `std::weak_ptr` represents a pointer that can share but not own a resource. In order to be used, it must first attempt to elevate itself to a `shared_ptr` via the `lock` method.

``` cpp
shared_ptr<Thing> shared = make_shared<Thing>();
weak_ptr<Thing> weak(shared);

if (shared_ptr<Thing> locked = weak.lock()) {
  // object still exists
} else {
  // object no longer exists
}
```

## Callables

The type `std::function` can store a type-erased pointer to any callable: function, lambda, bind expression, function object, pointer to member function, pointer to data member.

``` cpp
void print_int(int n) { cout << n << endl; }

std::function<void(int)> display_int = print_int;

print_int(7);
```

If the callable is a member function, then the target object must be explicitly passed when invoked.

``` cpp
const Class obj;

std::function<void(const Class&, int)> print_num = &Class::show_num;

print_num(obj, 4);
```

The `std::bind` function can wrap a function with certain parameters pre-bound. Parameters that aren't intended to be bound must pass-through using a placeholder in `std::placeholders`. It's much easier and less error-prone to use lambdas. For example, arguments to the `bind` call are copied or moved unless explicitly placed in a reference wrapper such as `std::ref` or `std::cref`.

``` cpp
void func(int a, int b, int &c) {
  return a - b * c;
}

// func(8, 4, 2) == 8

// pre-bind first and second parameter to 0 and 1
auto reverse_params = std::bind(func, 0, 1, std::placeholders::_3);

// func(8, 4, 2) == -2
```

The `functional` header contains a variety of function objects for primitive operations, such as `std::plus<T>` which is a function object for `operator+(T a, T b)`.

There are also `std::not1` and `std::not2` functions which negate the result of a unary or binary predicate, respectively.

## Iterators

The free-functions `std::begin` and `std::end` are specialized to return the respective iterators for different types. For example, usually one would just use the methods of the same name on actual collection objects. However, regular arrays don't have methods, and so these free-functions have specializations to do the correct thing for arrays.

Additionally, if the parameters are const-qualified, they return constant iterators, like the equivalent `cbegin` and `cend` methods.

The `distance` function can be used to get the number of elements between first and last.

The `next` and `prev` function returns a new iterator that is advanced forward or backward respectively by a given number of steps, or a single step by default.

The `advance` function is similar but it modifies the passed iterator, and it can take a signed integer so that a negative step steps backward.

The functions `std::back_inserter` and `std::front_inserter` take a collection and return iterator adapters whose assignment operator automatically calls `push_back` or `push_front` on the collection.

``` cpp
std::vector<int> v{1, 2, 3}, expect{1, 2, 3, 0, 0, 0};

std::fill_n(std::back_inserter(v), 3, 0);

EXPECT_EQ(expect, v);
```

Function `std::inserter` is a generalization of the above functions. It takes a collection and an iterator position. Assigning to the iterator (dereferenced or not) calls the corresponding `insert` method at that position.

``` cpp
std::vector<int> v{1, 2, 3}, expect{1, 0, 0, 2, 3};

std::fill_n(std::inserter(v, std::next(v.begin())), 2, 0);

EXPECT_EQ(expect, v);
```

The `std::make_reverse_iterator` function returns an iterator adapter that reverses the direction of the iterator. However, with C++14 and above it's simpler to call the `rbegin` and `rend` methods.

The `std::make_move_iterator` function returns an iterator adapter that overloads its deference operator to convert the returned value into an rvalue, allowing the underlying value to be moved from. Note that there is also an algorithm `std::move` that can be used.

``` cpp
std::vector<std::string> s{"one", "two", "three"};
std::vector<std::string> v(std::make_move_iterator(s.begin()),
                           std::make_move_iterator(s.end()));

std::vector<std::string> s_expect{"", "", ""};
std::vector<std::string> v_expect{"one", "two", "three"};

EXPECT_EQ(s_expect, s);
EXPECT_EQ(v_expect, v);
```

The `std::ostream_iterator` and `std::istream_iterator` iterator types are iterators which write to an `ostream` or read from an `istream`, respectively. These can be used, for example, to read a sequence of delimited numbers from standard input, or write a sequence of numbers to standard output. In the case of `std::istream_iterator`, the end-point iterator can be constructed by passing no parameters.

``` cpp
// Read the input stream into a vector.
std::istringstream input("1 2 3 4 5");
std::vector<int> v, v_expect{1, 2, 3, 4, 5};

std::copy(
  std::istream_iterator<int>(input),
  std::istream_iterator<int>(),
  std::back_inserter(v)
);

EXPECT_EQ(v_expect, v);

// Write the vector to an output stream (could be std::cout)
std::ostringstream output;

std::copy(v.begin(), v.end(),
          std::ostream_iterator<int>(output, " "));

// Note trailing space.
EXPECT_EQ("1 2 3 4 5 ", output.str());
```

## Collection Exception-Safety

Most functions and collections which are able to perform moves only do so if the moves are `noexcept`, otherwise copies are performed.

## Algorithms

There are a couple of conventions that STL algorithms follow:

* most algorithms take at least one iterator range specifying the range of operation, where the range is exclusive on its end-point, i.e. [begin, end).

    Some algorithms take two ranges, and sometimes the end-point of the second can be omitted, which automatically assumes the same distance as the first range.

* algorithms whose names end in `_n` take a beginning iterator position and a count of elements to affect from that point onward, instead of the regular [begin, end) iterator range.

* algorithms whose names end in `_if` take a predicate instead of a value to compare against.

* algorithms whose names end in `_until` return an iterator to one-past the last "qualifying" element. For example, `is_sorted_until` returns one-past the last sorted element.

* algorithms whose names end in `_backward` perform their operation from right-to-left. For example,  `copy_backward` copies to to the last source element to the last destination iterator position, then the penultimate, etc. **Don't** confuse this with the operation being performed in reverse. The elements are still in the same order; they were simply copied from right-to-left.

* algorithms whose names contain `_copy` perform their operation to elements as they are being copied into another range. Conversely, their counterparts which _don't_ have `_copy` in their name operate in-place, modifying the elements of the range.

* algorithms that take a comparison function expect a function which returns `true` if the first parameter is _less_ than the second, and `false` otherwise. Comparison functions must not modify the parameters.

    Equality can be checked with such a comparison function by ensuring that it doesn't yield true for either one with respect to the other, i.e. a and b are equal if `!cmp(a, b) && !cmp(b, a)`, e.g. if `!(a < b) && !(b < a)`.

* algorithms that concern "order" or "equality" accept an optional comparison function

* algorithms usually return one-past the last element that was operated on.

The `exchange` function replaces the value of an object with that of another and returns the old value.

The `all_of`, `any_of`, and `none_of` functions specify whether the elements of a given iterator range satisfy the given predicate for all, any, or none of the elements respectively.

``` cpp
vector<int> vec{0, 2, 4, 6};

bool all_even = std::all_of(vec.begin(), vec.end(), [](auto i) {
                  return i % 2 == 0;
                });

EXPECT_TRUE(all_even);
```

The `for_each` function applies a function to each element in the range, potentially mutating the element. It returns the provided function object, allowing for the accumulation of a result.

``` cpp
vector<int> vec{1, 2, 3}, expect{2, 3, 4};

std::for_each(vec.begin(), vec.end(), [](int &n) { ++n; });

EXPECT_EQ(expect, vec);
```

The `count` function counts all elements in the range which equals a given value, whereas `count_if` counts those which satisfy a given predicate.

``` cpp
vector<int> vec{1, 2, 3};

int twos = count(vec.begin(), vec.end(), 2);
int odds = count_if(vec.begin(), vec.end(),
                    [](int i) { return i % 2 == 0; });

EXPECT_EQ(1, twos);
EXPECT_EQ(2, odds);
```

The `mismatch` function takes two iterator ranges and finds and returns the first mismatching positions as determined by equality or a given predicate.

``` cpp
vector<int> a{1, 2, 3, 4, 5};
vector<int> b{1, 2, 3, 5, 6};

auto pos = std::mismatch(a.begin(), a.end(), b.begin(), b.end());

EXPECT_EQ(&a[3], &*pos.first);
EXPECT_EQ(a[3], *(pos.first));
EXPECT_EQ(b[3], *(pos.second));
```

The `equal` function checks if two iterator ranges are equivalent by equality or predicate.

``` cpp
std::string s = "radar";

// Compare forward range to backward range
bool is_palindrome = std::equal(s.begin(), (s.begin() + s.size() / 2),
                                s.rbegin());

EXPECT_TRUE(is_palindrome);
```

The `find` function gets the iterator position of the first element equal to the given value, whereas the `find_if` and `find_if_not` functions find the first element that does or does not satisfy the given predicate, respectively.

``` cpp
vector<int> v{1, 2, 3};

auto two_position = std::find(v.begin(), v.end(), 2);
auto even_position = std::find_if(v.begin(), v.end(),
                                  [](const int &i) {
                                    return i % 2 == 0;
                                  });

EXPECT_EQ(&v[1], &*two_position);
EXPECT_EQ(&v[1], &*even_position);
```

The `find_end` function gets the iterator position of beginning of the last occurrence of the subsequence denoted by the second iterator range within the sequence denoted by the first iterator range.

``` cpp
vector<int> v{1, 2, 3, 4, 1, 2, 3, 4};
vector<int> needle{1, 2, 3};

auto pos = std::find_end(v.begin(), v.end(), seq.begin(), seq.end());

EXPECT_EQ(&v[4], &*pos);
```

The `find_first_of` function gets the iterator position of the first element found that's one of the elements in the second range.

``` cpp
vector<int> v{0, 2, 3, 25, 5};
set<int> els{3, 19, 10, 2};

auto pos = std::find_first_of(v.begin(), v.end(), els.begin(), els.end());

EXPECT_EQ(&v[2], &*pos);
```

The `adjacent_find` method finds the position of the first two consecutive elements that are equal to each other or both satisfy a predicate.

``` cpp
vector<int> v{0, 1, 2, 3, 40, 40, 41, 41, 5};

auto pos = std::adjacent_find(v.begin(), v.end());
EXPECT_EQ(&v[4], &*pos);

// If it's sorted, each pair should satisfy std::less<int>()(left, right)
// Conversely, if any pair satisfies std::greater<int>(left, right), then
// that is the first pair of elements that is unsorted.
auto unsorted = std::adjacent_find(v.begin(), v.end(), std::greater<int>());
EXPECT_EQ(&v[7], &*pos);
```

The `search` function finds the second range within the first range using equality or a given predicate.

``` cpp
string s = "one two three";
string needle = "two";

auto pos = std::search(s.begin(), s.end(), needle.begin(), needle.end());

EXPECT_EQ(&s[4], &*pos);
```

The `search_n` function finds the a sequence of `n` elements of `value`, e.g. three 4's, within the first range using equality or a given predicate.

``` cpp
vector<int> v{1, 2, 3, 3, 3, 3, 4};

auto pos = std::search_n(v.begin(), v.end(), 4, 3);

EXPECT_EQ(&v[2], &*pos);
```

The `copy` function copies elements from the given range into the range beginning at a given iterator position. The `copy_if` does the same only if the element satisfies a given predicate.

``` cpp
vector<int> v{1, 2, 3};
vector<int> odds, odds_expect{1, 3};

std::copy_if(v.begin(), v.end(), std::back_inserter(odds),
             [](auto i) { return i % 2 != 0; });

EXPECT_EQ(odds_expect, odds);
```

The `copy_n` function copies `n` elements from the range starting at the first parameter to the range starting at the third parameter.

``` cpp
vector<int> a{1, 2, 3, 4};
vector<int> b{5};

std::copy_n(a.begin(), 3, std::back_inserter(b));

EXPECT_EQ({5, 1, 2, 3}, b);
```

The `max_element` and `min_element` find the position of the largest or smallest element in the range, respectively. A comparison function can be passed.

``` cpp
vector<int> v{3, 1, 4, 5, 9};

auto pos = std::min_element(v.begin(), v.end());

EXPECT_EQ(1, *pos);
```

The `minmax_element` function simultaneously finds the smallest and largest element in the range.

``` cpp
vector<int> v{3, 9, 1, 4, 2, 5, 9};

auto min_max = std::minmax_element(v.begin(), v.end());

EXPECT_EQ(1, *min_max.first);
EXPECT_EQ(9, *min_max.second);
```

There is also a `minmax` function which operates on two operands or an initializer list. This can be used to simultaneously find the lesser and greater of two numbers, such as for randomly setting some bounds:

``` cpp
std::vector<int> v{3, 1, 4, 1, 5, 9, 2, 6};
std::srand(std::time(0));

std::pair<int, int> bounds = std::minmax(std::rand() % v.size(),
                                         std::rand() % v.size());
```

The `swap_ranges` function swaps elements from the first range with the corresponding elements of the second range, and returns one-past the last swapped element of the _second_ range.

``` cpp
vector<int> v{1, 2, 3};
list<int> l{4, 5, 6};

// Swap the first two elements of v with first two elements of l
std::swap_ranges(v.begin(), v.begin() + 2, l.begin());

EXPECT_EQ({4, 5, 3}, v);
EXPECT_EQ({1, 2, 6}, l);
```

The `swap` function swaps two parameters with each other. It's also overloaded for arrays. It's important to note that it should usually be called in an unqualified manner. That is, use a using-declaration to bring the `std::swap` definitions into scope, but call it as `swap` and not `std::swap`. This enables additional, perhaps more specialized overloads of `swap` to be used when appropriate.

``` cpp
void Function() {
  using std::swap;

  int a = 1, b = 2;

  swap(a, b);
}
```

The `iota` function can be used to fill a range with sequentially incremented values, starting with the given value.

``` cpp
vector<int> v(3);
std::iota(v.begin(), v.end(), 1);

EXPECT_EQ({1, 2, 3}, v);
```

The `copy_backward` function copies elements right-to-left from the given range into the range ending with the third parameter. Note that the elements are _not_ copied in reverse, that is, the order of the elements is preserved. Instead, this function copies starting from the right end, which is why the end iterator is provided.

``` cpp
vector<int> source{1, 2, 3};
vector<int> destination(4);

std::copy_backward(source.begin(), source.end(), destination.end());

EXPECT_EQ({0, 1, 2, 3}, destination);
```

The `fill` function sets every element in the range to a given value. There is also `fill_n`.

``` cpp
vector<int> v(3);

std::fill(v.begin(), v.end(), 7);

EXPECT_E1({7, 7, 7}, v);
```

The `generate` function assigns each element in the range the value generated by the provided function, which takes no arguments. There is also `generate_n`.

``` cpp
vector<int> v(3);
int n = 1;

std::generate(v.begin(), v.end(), [&n]() { return n++; });

EXPECT_EQ({1, 2, 3}, v);
```

The `move` function moves elements from the range into the range beginning with the third parameter.

``` cpp
vector<thread> ths;
ths.emplace_back(func, arg);
ths.emplace_back(func, arg);

vector<thread> dest(2);

// Could just dest = move(ths) in this case, but w/e
std::move(ths.begin(), ths.end(), dest.begin());
```

The `move_backward` function is equivalent to `copy_backward` but moves its elements instead of copying them. That is, it moves elements from the range into the range ending with the third parameter, moving the last element first, then the penultimate one, and so on.

``` cpp
vector<string> src{"one", "two", "three"};
vector<string> dest(4, "");

std::move_backward(src.begin(), src.end(), dest.end());

EXPECT_EQ({"", "one", "two", "three"}, dest);
```

The `remove_copy` function copies elements from the range into the beginning of the range denoted by the third parameter, skipping any elements equal to the provided value or satisfying a given predicate (with `remove_copy_if`). In other words, this is the opposite of `copy` and `copy_if`.

``` cpp
vector<int> a{1, 1, 2, 3};
vector<int> not_one(2, 0), odds(2, 0);

std::remove_copy(a.begin(), a.end(), not_one.begin(), 1);

EXPECT_EQ({2, 3}, not_one);

std::remove_copy(a.begin(), a.end(), odds.begin(),
                 [](int i) { return i % 2 == 0 });

EXPECT_EQ({1, 3}, odds);
```

The `replace` function replaces the elements in the range that match the given value or satisfy the predicate (with `replace_if`) with another value.

``` cpp
vector<int> v{1, 1, 2, 2, 3};

// Replace even numbers with 0.
std::replace_if(v.begin(), v.end(),
                [](int i) { return i % 2 == 0; },
                0);

EXPECT_EQ({1, 1, 0, 0, 3}, v);
```

The `replace_copy` function copies elements from the range into the range beginning at the third parameter, mapping those that match the given value or satisfy a predicate (with `replace_copy_if`) to another value.

``` cpp
vector<int> v{1, 1, 3, 4, 5};
vector<int> b(5);

// Copy v into b, mapping values of 1 → 0
std::replace_copy(v.begin(), v.end(), b.begin(), 1, 0);

EXPECT_EQ({0, 0, 3, 4, 5}, b);
```

The `iter_swap` function simply swaps the elements pointed to by the iterators.

``` cpp
vector<int> v{1, 2, 3};

// Swap the 1 and the 3
std::iter_swap(v.begin(), v.end() - 1);

EXPECT_EQ({3, 2, 1}, v);

// Can also explicitly just dereference iterators and call std::swap
swap(*v.begin(), *(v.end() - 1));
```

The `reverse` function reverses the order of the elements in the range.

``` cpp
vector<int> v{1, 2, 3};

std::reverse(v.begin(), v.end());

EXPECT_EQ({3, 2, 1}, v);
```

The `reverse_copy` function copies elements from the range into another range beginning with the third parameter in reverse order.

``` cpp
vector<int> v{1, 2, 3};
vector<int> d(3);

std::reverse_copy(v.begin(), v.end(), d.begin());

EXPECT_EQ({3, 2, 1}, v);
```

The `rotate` function rotates all elements in the range to the left such that the middle parameter becomes the first element in the range. Note that this function breaks convention in that there is a parameter (the "new-left") in between the [begin, end) iterator pair parameters. As per convention, `rotate_copy` does the same but copies the result into another range.

``` cpp
vector<int> v{1, 2, 3, 4};

// Rotate v to the left so that the 3 (v[2]) become the first element.
std::rotate(v.begin(), v.begin() + 2, v.end());

EXPECT_EQ({3, 4, 1, 2}, v);
```

The `random_shuffle` function shuffles all elements in the range given a random number generator.

``` cpp
vector<int> v{1, 2, 3};
random_device rd;
mt19937 g(rd());

std::shuffle(v.begin(), v.end(), g);

// e.g. v = {3, 1, 2}
```

The `unique` function removes all _consecutive duplicates_ (and thus expects a sorted input) from the range, returning one-past the new logical end of the range. Consecutive duplicates are checked by equality or a given predicate. There is also `unique_copy`.

Note that the elements are _not_ physically removed from the container, so this call is usually followed by a call to the `erase` method of the collection with the iterator returned by `unique`.

``` cpp
vector<int> v{1, 1, 1, 2, 2, 3, 4};

auto last = std::unique(v.begin(), v.end());

// v = {1, 2, 3, 4, x, x, x} where x = indeterminate

v.erase(last, v.end());

EXPECT_EQ({1, 2, 3, 4}, v);
```

The `is_partitioned` function checks if all elements in the range are partitioned based on the given predicate, so that all elements that satisfy the predicate come before all of those that don't.

``` cpp
vector<int> v{1, 1, 0, 0};

bool is_parted = std::is_partitioned(v.begin(), v.end(),
                                     [](int i) { return i == 1; });
```

The `partition` function partitions the elements of a range so that all elements that satisfy the predicate come before all of those that don't. There is also `partition_copy`.

``` cpp
vector<int> v{1, 2, 3, 4, 5, 6};
auto partition_func = [](int i) { return i % 2 == 0; };

auto it = std::partition(v.begin(), v.end(), partition_func);

bool is_parted = std::is_partitioned(v.begin(), v.end(),
                                     partition_func;
```

The `stable_partition` function is a stable version of the `partition` function, so that the relative order of equal elements is preserved.

The `partition_point` function returns one-past the end of the first partition, i.e. the first element that doesn't satisfy the predicate.

``` cpp
vector<int> v{2, 4, 1, 3};
auto pos = std::partition_point(v.begin(), v.end(),
                                [](int i) { return i % 2 == 0; });
```

The `is_sorted` function checks if the range is sorted in ascending order or given a comparison function.

``` cpp
vector<int> v{1, 2, 3};

bool sorted = std::is_sorted(v.begin(), v.end()));
```

The `is_sorted_until` function returns one-past the last sorted element, i.e. the first element that is not sorted.

The `sort` function sorts the range in ascending order or based on a given comparison function. The `stable_sort` variant preserves the relative order of equal elements.

The `partial_sort` function rearranges the elements of the range so that the [begin, middle) contains the elements of the sorted order of the entire array, i.e. the first (middle - begin) smallest elements. There is also `partial_sort_copy` which only copies enough elements that fit in the destination.

``` cpp
vector<int> v{5, 7, 4, 2, 8, 6, 1, 9, 0, 3};

std::partial_sort(v.begin(), v.begin() + 3, v.end());

// v = {0, 1, 2, 7, 8, 6, 5, 9, 4, 3}
//      |-----|
//      sorted
```

The `nth_element` function selects the $n^\text {th}$ element from the sorted order of the range, i.e. the $n^\text {th}$-order statistic. A comparison function can be specified.

This is like Quicksort's partitioning, so the range is modified so that the element pointed to by `nth` becomes the `nth` element of the sorted order of the range. All elements to the left of the iterator are less than or equal to that element and all elements to the right of the iterator are greater than that element.

``` cpp
vector<int> v{5, 6, 4, 3, 2, 6, 7, 9, 3};
int mid = v.size() / 2;

std::nth_element(v.begin(), v.begin() + mid, v.end());
// median is v[mid] = 5

std::nth_element(v.begin(), v.end() - 1, v.end());
// max is v.back()
```

The `lower_bound` function returns an iterator to the first element in the partially or fully sorted range that is greater than or equal to the value or comparison function. There is also an `upper_bound` function.

``` cpp
vector<int> v{1, 2, 3, 4, 4, 5, 6};

auto lower = std::lower_bound(v.begin(), v.end(), 4);

// lower = idx 3
```

The `equal_range` function returns an iterator range (pair) of the sub-range of elements in the input range that contains values equivalent to the provided value based on an optional comparison function.

``` cpp
vector<int> v{1, 1, 2, 2, 2, 3, 4};

auto range = equal_range(v.begin(), v.end(), 2);

bool all_twos = std::all_of(range.first, range.second,
                            [](int i) { return i == 2; });

EXPECT_TRUE(all_twos);
```

The `binary_search` function does a membership check by performing a binary search for the value in the partially or fully sorted range using an optional comparison function.

``` cpp
vector<int> v{1, 3, 4, 5, 9};

bool found_four = std::binary_search(v.begin(), v.end(), 4);
```

The `merge` function merges two sorted ranges together into a destination.

``` cpp
vector<int> a{1, 3, 5}, b{2, 4, 6};
vector<int> dest(6);

std::merge(a.begin(), a.end(),
           b.begin(), b.end(),
           dest.begin());

// dest = {1, 2, 3, 4, 5, 6}
```

The `inplace_merge` function is an in-place variant of the `merge` function, merging two sorted ranges denoted by [begin, middle) and [middle, end) into a single sorted range, in-place.

``` cpp
vector<int> v{1, 3, 5, 2, 4, 6};

std::inplace_merge(v.begin(), v.begin() + 3, v.end());

// v = {1, 2, 3, 4, 5, 6}
```

The `includes` function checks if every element of the second sorted range is found within the first sorted range, reads as "first range includes second."

``` cpp
vector<int> v{1, 2, 3, 4, 5, 6};
vector<int> a{3, 4, 5}, b{1, 6};

// true
std::includes(v.begin(), v.end(), a.begin(), a.end());

// true
std::includes(v.begin(), v.end(), b.begin(), b.end());
```

The `set_difference` function performs a set difference operation on the two ranges and outputs the result into the destination iterator.

``` cpp
vector<int> a{1, 2, 3, 4, 5};
vector<int> b{2, 4};
vector<int> difference(3);

std::set_difference(a.begin(), a.end(),
                    b.begin(), b.end(),
                    difference.begin());

// difference = {1, 3, 5};
```

There is also `set_intersection`:

``` cpp
vector<int> a{1, 3, 5};
vector<int> b{2, 4, 6};
vector<int> intersection(6);

std::set_intersection(a.begin(), a.end(),
                      b.begin(), b.end(),
                      intersection.begin());

// intersection = {1, 2, 3, 4, 5, 6};
```

There is also `set_union`:

``` cpp
vector<int> a{1, 3, 5, 5};
vector<int> b{1, 2, 2, 4};
vector<int> union(6);

std::set_union(a.begin(), a.end(),
               b.begin(), b.end(),
               union.begin());

// union = {1, 2, 3, 4, 5};
```

The `set_symmetric_difference` function computes the set of elements that are in either of the sets but _not_ in both (not in their intersection).

``` cpp
vector<int> a{1, 2, 3};
vector<int> b{3, 4};
vector<int> symmetric_difference(3);

std::set_union(a.begin(), a.end(),
               b.begin(), b.end(),
               symmetric_difference.begin());

// symmetric_difference = {1, 2, 4};
// not 3 because it's in both
```

The `make_heap` function constructs a max-heap from the elements in the range, i.e. it's a "heapify" operation. A heap with a different order can be created using an optional comparison function, for example with `std::greater<in>()` a min-heap can be created.

``` cpp
vector<int> v{3, 1, 4, 1, 5, 9};

std::make_heap(v.begin(), v.end());

// v = {9, 5, 4, 1, 1, 3}
```

The `push_heap` function is used to _logically push_ the last element in the range onto the logical heap. This means that the element must already be present in the range, added for example via something like `push_back`.

``` cpp
vector<int> v{3, 1, 4, 1, 5, 9};

std::make_heap(v.begin(), v.end());

// physically push
v.push_back(6);
// v = {9, 5, 4, 1, 1, 3, 6}

// logically push
std::push_heap(v.begin(), v.end());
// v = {9, 5, 6, 1, 1, 3, 4}
```

The `pop_heap` function is used to _logically pop_ the top element from the heap by swapping the top element of the logical heap with the last element in the heap, then re-heapifying to preserve heap-order. If an optional comparison function was used with `make_heap` or `push_heap`, it should also be used here.

``` cpp
vector<int> v{3, 1, 4, 1, 5, 9};

std::make_heap(v.begin(), v.end());

int largest = v.front();

// logically remove
std::pop_heap(v.begin(), v.end());

// physically remove
v.pop_back(); // actually remove
```

The `sort_heap` function essentially performs a heap sort, that is, it sorts the elements into the heap in ascending order or given a comparison function, destroying the logical heap.

The `is_heap` function checks to see if the range is in heap-order given some optional comparison function. There is an `is_heap_until` function which returns one-past the last heap-ordered element.

The `lexicographical_compare` function checks to see if the first range is lexicographically less than the second range.

``` cpp
vector<int> a{1, 2, 2}, b{1, 2, 3};

std::lexicographical_compare(a.begin(), a.end(),
                             b.begin(), b.end());
// a < b = true
```

The `is_permutation` function checks to see if the first range is a permutation of the second range.

``` cpp
vector<int> a{1, 2, 3, 4, 5}, b{3, 5, 4, 1, 2};

std::is_permutation(a.begin(), a.end(),
                    b.begin(), b.end());
// true
```

The `next_permutation` function rearranges the elements of the range into the lexicographically-next permutation and returns true. If a next permutation doesn't exist, it wraps around and produces the first permutation (i.e sorted order) and returns false. There is also `prev_permutation` which does the opposite.

``` cpp
string s = "aba";

// produce first permutation
std::sort(s.begin(), s.end());

do { cout << s << endl; }
while (next_permutation(s.begin(), s.end()));

// aab, aba, baa
```

The `accumulate` function reduces the elements in the range given an initial value and an optional binary function which produces the reduction of two elements. By default, the binary function is addition, so `accumulate` produces the sum of the elements.

``` cpp
vector<int> v{1, 2, 3, 4, 5};

int sum = std::accumulate(v.begin(), v.end(), 0);
// sum = 15

int product = std::accumulate(v.begin(), v.end(), 1,
                              [](int prod, int next) {
                                return prod * next;
                              });
// product = 120

// can also be
int product = std::accumulate(v.begin(), v.end(), 1, std::multiplies<int>());
```

The `inner_product` function computes the sum of the pair-wise products of the elements of the two ranges, which is the _dot product_. Custom sum and product functions can be provided, in which case the "product" function is applied in a pair-wise manner to the elements of the two ranges, and the "sum" function is applied to those results.

This is like an `transform`/`for_each` of pair-wise elements with the "product" function and an `accumulate` of those results with the "sum" function.

``` cpp
vector<int> a{0, 1, 2, 3, 4};
vector<int> b{5, 4, 2, 3, 1};

int dot_product = std::inner_product(a.begin, a.end(), b.begin(), 0);
// dot_product = 21
```

The `adjacent_difference` function computes the difference of each element in the range and its predecessor, writing each element into the output iterator. Since the first element doesn't have a predecessor, the predecessor is treated as 0.

``` cpp
vector<int> v{2, 4, 6, 8};

// v = {2, 2, 2, 2};
std::adjacent_difference(v.begin(), v.end(), v.begin());

v = {1, 1, 1, 1};

// v = {1, 1, 2, 3}
.adjacent_difference(v.begin(), v.end() - 1,
                     v.begin() + 1,
                     [](int a, int b) { return a + b });
```

The `partial_sum` function successively computes the sums of increasing sub-ranges of the input range and copies each sum into the output iterator. A custom sum function can be provided. Specifically, the result is such that:

$$ \text {dest}[i] = \sum_0^i src[i] $$

This can be useful for example to compute the [maximal sub-array](https://en.wikipedia.org/wiki/Maximum_subarray_problem).

``` cpp
vector<int> v{2, 2, 2, 2};
vector<int> dest(4);

std::partial_sum(v.begin(), v.end(), dest.begin());

for (std::size_t i = 0; i < dest.size(); ++i) {
  std::cout << "sum of sub-range [0, " << i << "] = " << dest[i] << '\n';
}

// dest = {2, 4, 6, 8}
```

The `transform` function applies a given unary function to each element in the array, or a given binary function to each pair of elements in two ranges, and writes each result to the output iterator.

``` cpp
string s("hello");

std::transform(s.begin(), s.end(), s.begin(), ::toupper);

// s = "HELLO"
```

The `remove` function rearranges the elements of the range so that those equal to a given value or satisfying a given predicate are moved to the end of the range, allowing them to easily be erased from their container.

``` cpp
vector<int> v{1, 2, 1, 3, 1, 4};

auto new_end = std::remove(v.begin(), v.end(), 1);

v.erase(new_end, v.end());
```

## Random

Random engines are a stateful source of randomness, and random distributions use a random engine to generate random numbers distributed over a range.

The `shuffle` method can be used to shuffle a range based on a given random number generator.

``` cpp
vector<int> v{1, 2, 3, 4, 5};
std::default_random_engine engine;

std::shuffle(v.begin(), v.end(), engine);
```

Random numbers can be generated with, for example, `uniform_int_distribution`. The constructor takes the inclusive bounds.

``` cpp
std::default_random_engine engine;
std::uniform_int_distribution<int> distribution(10, 20);

for (int i = 0; i < 5; ++i)
  cout << distribution(engine) << endl;
```

## Concurrency

Threads are represented by `std::thread<F, Args...>` and they run on a separate thread. These can be instantiated with a lambda:

``` cpp
thread print([]() {
  std::cout << "other thread" << std::endl;
});
```

Threads can be `join`ed and `detach`ed.

The `std::mutex` type represents a mutex, which can be `lock`ed and `unlock`ed.

An `std::lock_guard` is a type that provides RAII ownership of an `std::mutex`, so that the lock is automatically unlocked when the `lock_guard` is destroyed.

``` cpp
std::mutex global_mutex;

void func() {
  std::lock_guard<mutex> lock(global_mutex);

  // unlocked here
}
```

An `std::unique_lock` is similar except that it may not necessarily be associated with a mutex, and locking and unlocking can be done explicitly, essentially it simply guarantees that if the mutex is locked when the `unique_lock` is destroyed, it unlocks it.

Ownership of the associated lock can be `release`ed.

The `std::lock` function can lock an arbitrary number of locks (passed as parameters) in such a way that deadlocks are avoided. If an exception occurs, all so-far locked mutexes are unlocked.

An `std::condition_variable` represents a condition variable which can be used to notify others of being ready via `notify_one` and `notify_all`, and others to wait on the condition variable via `wait`. One of the overloads of `wait` can accept a lambda which is used to test the condition, in order to guard against spurious wake-ups, in which case the wait is repeated.

``` cpp
std::mutex m;
std::condition_variable cv;

bool ready = false;

// thread 1 runs first
std::unique_lock<std::mutex> lock(m);

// notify waiting threads
cv.notify_one();

// thread 2 runs after, and waits on the condition variable
// to guard against spurious wake-ups, a lambda is run to ensure
// that the actual condition holds. if it doesn't, it waits again
std::unique_lock<std::mutex> lock(m);
cv.wait(lock, [] { return ready; });
```

An `std::promise<T>` is the push/write-end of the promise-future communication channel. It can be used to store a value that is later acquired asynchronously via an `std::future` created by the promise via `get_future`. A promise is made ready by writing a value to it via `set_value`.

A value can be obtained from the future via `get`, which blocks until a result is received, then returns that value. There's also `wait` that simply waits until the value is received, but doesn't actually retrieve it.

The `std::async` function wraps a function and calls it asynchronously, possibly on another thread, and returns a future representing the result.

The `std::packaged_task<R(Args...)>` type is similar in that it wraps any callable so that it can be invoked asynchronously and its return value obtained via a future. It does this by overloading the call operator so that when the function returns, its value is written to a promise.

The future is obtained via `get_future`. The `packaged_task` can then be run on a separate thread, for example.

``` cpp
int fib(int n) {
 if (n < 3) return 1;
 else return fib(n - 1) + fib(n - 2);
}

packaged_task<int(int)> task(&fib);
auto result = task.get_future();

thread t(std::move(task), 40);
int answer = result.get();

t.join();
```

Atomic types are represented by `std::atomic<T>` and they can be either integral types or pointers. Their constructor takes their initial value.

Atomic replacement of the value is done via `store`, and atomic reading can be done with `load`, which is also aliased to a conversion operator for the underlying type.

The `exchange` method atomically exchanges the current value with another, and returns the old value.

Similarly, there are `fetch_` methods such as `fetch_add` which atomically adds an operand to the atomic variable and returns the original value. The increment and decrement operators are also overloaded to perform atomic increments and decrements.

# Resources

* [Three Optimization Tips for C++](https://www.facebook.com/notes/facebook-engineering/three-optimization-tips-for-c/10151361643253920)
