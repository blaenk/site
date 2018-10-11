+++
title = "C++"
date = 2013-09-10

[note]
kind = "language"
+++

A lot of people really dislike C++ because it's a very complex language that often catches one by surprise. Despite this, C++ is undisputed when it comes to striking a balance between abstraction and speed. Those that need to use it for these reasons generally take one of two approaches, while the rest completely dismiss it as an option to begin with.

The first consists of restricting its usage to a specific subset of the language specification; for example, exceptions are generally avoided.

The other approach, perhaps an extreme, is for people to become "language lawyers," poring over the language specification for every minute detail.

I guess I became a language lawyer myself after [asking a question] on StackOverflow that was given the [language lawyer tag].

[asking a question]: https://stackoverflow.com/questions/33849718/how-is-this-lambda-with-an-empty-capture-list-able-to-refer-to-reaching-scope-na
[language lawyer tag]: https://stackoverflow.com/review/suggested-edits/15111866

In general, though, I try to take a pragmatic approach. I do appreciate C++'s advantage in striking a balance between speed and abstraction, I do limit my use of it to a certain subset of the language, and I do try to learn as much about the language short of actually reading the specification to lower the probability that the language may catch me off guard.

To that end, these are non-exhaustive notes about C++---particularly the trickier bits---including C++11 and C++14 changes.

For C++11 in particular, some compilers are faster than others at adopting the new feature set. [Visual Studio](http://msdn.microsoft.com/en-us/library/vstudio/hh567368%28v=vs.120%29.aspx) is particularly behind the rest, while [Clang](http://clang.llvm.org/cxx_status.html) and [GCC](http://gcc.gnu.org/projects/cxx0x.html) seem to be very quick on adopting the new features.

<nav id="toc"></nav>

# Type Aliasing

Type aliases are similar to `typedef`s but they can also be used with template names.

``` cpp
using flags = std::ios_base::fmtflags;
using func = void(*)(int,int);

template <typename T>
using ptr = T*;

// int* = ptr<int>

template <typename T>
using B = Blah<T>;

// B<int> == Blah<int>

template <typename T>
struct Container {
  using value_type = T;
};

typename Container::value_type n;
```

A _dependent name_ is one that depends on a type parameter. For example, in the following code, `ty` is a dependent name because its lookup depends on the type of template argument `T`, such that it can't be looked up until the template argument is known. For example, `T::C` may end up being a type for one `T` while it may be a static integer member for another `T`.

``` cpp
template <typename T>
struct S {
  T::C ty;  // ambiguous whether it's a type or value
};
```

A dependent name is assumed to not name a type unless the name is qualified with the `typename` specifier. A `typename` specifier isn't necessary in certain simple cases, such as `T *ptr`. The `typename` specifier is only allowed for qualified names, unqualified names are assumed to be types.

``` cpp
template <typename T>
struct S {
  typename T::C ty;
};

typename std::vector<T>::const_iterator it = v.begin();
```

A name that is not a member of the current instantiation and is dependent on a template argument is not considered a type unless it's marked with the `typename` specifier _or_ it was defined as a type name, such as via a `typedef` or a `using` alias.

The same applies to templates. Consider the following [^dependent_name]:

[^dependent_name]: Taken from [this great StackOverflow answer](http://stackoverflow.com/a/613132/101090).

``` cpp
boost::function<int()> f;
```

Consider the following implementation of `boost::function`:

``` cpp
namespace boost {
  int function = 0;
}
```

In this case, the original code would end up being parsed as `boost::function` which is zero, compared to `int()` which is zero, compared to `f`.

In order to explicitly specify that a name is a template, the `template` specifier can be used:

``` cpp
T::template foo<X>();

s.template foo<X>();

this->template foo<X>();

typename T::template iterator<int>::value_type v;
```

# Lookup

The non-right side of the scope resolution operator during unqualified or qualified lookup only considers namespaces, class types, enumerations, or templates whose specializations are types.

``` cpp
struct A {
  static int n;
};

int main() {
  int A;
  A::n = 42; // OK: unqualified lookup ignores the variable
  A b;       // error: unqualified lookup of A finds the variable A
}
```

Argument-dependent lookup (ADL) makes it possible to use operators defined in a different namespace. For example, ADL finds the correct definition of the stream insertion operator because its argument is in the `std` namespace.

``` cpp
std::operator<<(std::ostream&, const char *);

operator<<(std::cout, "Test\n");
```

ADL is not considered if the lookup set produced by unqualified lookup contains any of:

* class member declaration
* block-scope function declaration
* any non-function & non-function template declaration

ADL checks each function call argument and template function template argument to determine the associated set of namespaces and classes it will add to the lookup.

ADL adds an associated set of namespaces and classes for every argument in a function call of type:

1. fundamental type: empty
2. class type, including unions: add:
    1. the class
    2. its direct/indirect base classes
    3. enclosing class (if any)
    4. enclosing namespaces
3. template specialization: add:
    1. the types of all template arguments for type template parameters
    2. classes and namespaces in which template template arguments are members
4. enumeration: class and namespace it's defined in
5. pointer to `T` or array of `T`: ADL set of `T`
6. function: ADL set of parameters and return type
7. pointer-to-member function `F` of class `X`: ADL set of parameter types, return type, and class `X`
8. type pointer-to-data member `T` of class `X`: ADL set of member type `T` and class `X`
9. name or address-of expression for overloaded function or function template: ADL set of every function in overload set
    1. if named by template name and arguments: ADL set of type template arguments and template template arguments

A condensed summary of ADL specifies that the associated-namespaces are:

* class member: the class itself, its base classes, and enclosing namespaces
* namespace member: enclosing namespaces
* built-in type: none

Due to ADL, non-member functions and operators defined in the same namespace as a class are considered part of the public interface of that class.

ADL only applies during unqualified lookup of function names in function-call expressions.

ADL can find a friend function that is defined entirely within a class or class template even if it was never declared at namespace level.

``` cpp
template <typename T>
struct number {
 number(int);

 friend number gcd(number x, number y) {
   return 0;
 };
};

void g() {
 number<double> a(3), b(4);
 a = gcd(a,b);
}
```

A function call to a function template with explicitly-specified template arguments requires a declaration of the template found by ordinary lookup, such as via a using-declaration:

``` cpp
namespace N1 {
  struct S {};

  template<int X>
  void f(S);
}

namespace N2 {
  template<class T>
  void f(T t);
}

void g(N1::S s) {
  f<3>(s);     // syntax error: unqualified lookup finds no `f`
  N2::f<3>(s); // error: wrong `f`, no ADL because qualified

  using N2::f;
  f<3>(s); // ok: unqualified lookup finds N2::f,
           // ADL kicks in and finds N1::f
}
```

When a name appears to the right of the scope resolution operator `::`, it is looked up via qualified lookup. Otherwise it is looked up using unqualified lookup. Note that this means that the `std` in `std::cout` will be looked up with unqualified lookup, while `cout` via qualified lookup.

For example, given the following:

``` cpp
namespace A {
  struct X;
  struct Y;

  void f(int);
  void g(X);
}

namespace B {
  void f(int i) {
    f(i); // 1
  }

  void g(A::X x) {
    g(x); // 2
  }

  void h(A::Y y) {
    h(y); // 3
  }
}
```

The following results are observed:

1. endless recursion because ADL is not used since `int` is a fundamental type, and unqualified lookup finds `B::f`.
2. ambiguity error because unqualified lookup finds `B::q` and ADL finds `A::g`.
3. endless recursion because ADL finds no `A::h` and unqualified lookup finds `B::h`.

Given the following expression:

``` cpp
std::cout << std::endl;
```

Each of the names are looked up as follows:

* `std`: unqualified lookup finds the namespace in `<iostream>`
* `cout`: qualified lookup finds the variable declaration in `std`
* `endl`: qualified lookup finds the function template declaration in `std`
* `operator<<`: ADL finds multiple function template declarations in `std`

## Overload Resolution

Overload resolution ranks candidate functions as:

1. exact match: no conversions
2. promotions: integral promotion or `float` to `double`
3. standard conversions: `int` ↔ `double`, `Derived*` to `Base*`, `T*` to `void*`, `int` to `unsigned`
4. user-defined conversions: `double` to `complex<double>`
5. ellipsis: `...` in a function declaration, e.g. `printf`

If more than one match is found at the highest level where a match is found, the call is rejected as ambiguous.

If a function and a template specialization are equally good matches for overload resolution, the function is preferred.

Overload resolution only considers the functions of a single scope. Use a using-declaration to bring declarations into scope.

``` cpp
void f(int);

void g() {
  void f(double);
  f(1); // calls f(double)
}
```

Overloading a function on both integral and pointer types should be avoided because calling the function with `0` will call the integral overload, not the pointer-type overload, unless the caller _always_ uses `nullptr` instead of `0`.

Overloading functions for which an overload taking a forwarding reference exists is discouraged because the forwarding reference overload will be very greedy, inducing perfect matches in situations where conversions would have matched more appropriate overloads.

Passing overloaded functions can lead to ambiguity errors. This can be resolved by explicitly selecting the desired overload by casting the function to the desired prototype.

# constexpr

`constexpr` functions yield compile-time constants when called with compile-time constants, otherwise they return runtime values if called with runtime values.

Constructors and member functions can be `constexpr`.

`constexpr` forms part of the type's interface. That is, adding for example I/O for debugging purposes would no longer permit `constexpr`, which could break a lot of existing code.

# decltype

| Input          | Deduction                |
| :----          | :--------                |
| identifier     | type                     |
| `(identifier)` | lvalue reference of type |
| `function()`   | return type              |
| rvalue         | rvalue reference type    |
| lvalue         | lvalue reference type    |

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
LL::Link *LL::erase(Link *p) {}
```

Using suffix-return syntax, after the compiler reads `LL::erase` it enters the class scope of `LL`, making it unnecessary to fully qualify the `Link` type that's nested within `LL`:

``` cpp
auto LL::erase(Link *p) -> Link * {}
```

# auto

The `auto` keyword allows for type-deduction and should be preferred in the following circumstances:

* when an expression would otherwise be repeated on both sides
* lambdas, though can also use `std::function`
* iterators and other long type names

`auto` type deduction rules are the same as that of Template Argument Deduction, except when initialized with a braced-init-list in which case the type is deduced to be `std::iitializer_list`.

`auto` variable type deduction rules follow those of Template Argument Deduction (TAD), that is, `auto&&` is deduced as an lvalue reference or rvalue reference.

Note that the use of `auto` in a function's trailing return type does _not_ perform automatic type deduction.

When `auto` is used as the function return type, TAD is used on the `return` statement's operand. Note that if an lvalue reference is returned, TAD will ignore/remove the reference. To return an lvalue reference, `decltype` deduction rules must be used instead:

``` cpp
template<typename C, typename I>
auto authAndAccess(C& c, I i) {
  authenticate();
  return c[i]; // returns Element not Element&
}

template<typename C, typename I>
decltype(auto) authAndAccess(C& c, I i) {
  authenticate();
  return c[i];
}
```

When `decltype(auto)` is used for variable type deduction, the `auto` keyword is replaced with the expression of the initializer, then regular `decltype` deduction rules take place.

When `decltype(auto)` is used as a function's return type, the `auto` keyword is replaced with the `return` statement's operand, then regular `decltype` deduction rules take place.

Lambda parameters can be declared `auto`.

When `auto` is used on a `new` expression, the type is deduced from the initializer.

``` cpp
auto c = new auto('x');
```

Sometimes it's necessary to use the _explicitly-typed initializer_ idiom to guide `auto` type deduction. For example, `vector<bool>::operator[]` returns a proxy class `vector<bool>::reference` as an implementation detail which implicitly converts to `bool`. However, if using `auto` type deduction the type will be declared as `vector<bool>::reference` instead of the desired `bool`.

``` cpp
auto isSet = bool_vector[3]; // doesn't deduce to bool
auto isSet = static_cast<bool>(bool_vector[3]);
```

# new

Calling `delete` on a `nullptr` does not call any destructors or deallocators.

Creating a dynamic array with `new` has the restriction that only the first dimension can be dynamic; all the other dimensions must be an integral constant expression.

``` cpp
int n = 42;
double a[n][5]; // Error

auto p1 = new double[n][5]; // OK
auto p2 = new double[5][n]; // Error
```

Placement-`new` can be used to construct objects in storage that has already been allocated. Objects can be destroyed without deallocating their storage by explicitly calling their destructors.

``` cpp
char* ptr = new char[sizeof(T)];
T* tptr = new(ptr) T; // construct in ptr storage

tptr->~T();   // destruct
delete[] ptr; // deallocate
```

# Type Conversions

The result of a cast expression is:

* an lvalue if `new_type` is an lvalue reference type or rvalue reference to function type
* xvalue if `new_type` is rvalue reference to object type
* prvalue otherwise

## Pointer Decay

Array-to-pointer decay for multidimensional arrays only converts the array to a pointer to the _first_ element, row, or plane. The pointer type has to be the type of the first element, row, or plane.

``` cpp
int a[2];
int *p1 = a;         // first element

int b[2][3];
// int *err = b;     // can't convert int (*)[3][4] to int*
int (*p2)[3] = b;    // first 3-element row

int c[2][3][4];
int (*p3)[3][4] = c; // first 3x4 element plane
```

A function name automatically converts to a pointer to that function.

``` cpp
void f(int);
void (*p1)(int) = &f;
void (*p2)(int) = f;
```

For a pointer-to-member, only the type of the member matters, not which _specific member_ is being poitned to. Pointer-to-members can also point to member functions. Members are dereferenced with the syntax `.*` or `->*`:

``` cpp
struct C {
 int m;
 void f(int n) {};
};

int C::* data_member = &C::m;
void (C::* member_function)(int) = &C::f;

C c   = {7};  c.*data_member;
C *cp = &c;   cp->*data_member;

(c.*member_function)(1);
```

A pointer-to-member of a base class can be implicitly converted to a pointer to the same member of a derived class.

A pointer-to-member of derived class can be used on the base class by converting it to a pointer-to-member of base class via `static_cast`. It's undefined behavior if the base class doesn't contain the member.

``` cpp
struct Base {};
struct Derived : Base { int m; };

int Derived::* dp = &Derived::m;
int Base::* bp = static_cast<int Base::*>(dp);
```

The type of a pointer-to-member can itself be a pointer-to-member.

``` cpp
struct A {
  int m;
  int A::* const p;
};

int A::* const A::* p = &A::p;
const A a = {1, &A::m};

a.*(a.*p);

int A::* const* p2 = &a.p;
a.* *p2;
```

Pointer-to-member functions can be used as callbacks or function objects using `std::mem_fn` or `std::bind`:

``` cpp
std::vector<std::string> v = {"a", "ab", "abc"};
std::vector<std::size_t> l;

std::transform(v.begin(), v.end(), std::back_inserter(l),
               std::mem_fn(&std::string::size));
```

## C-Style Casts

C-Style casts are discouraged. C++-style explicit casts such as `static_cast` are preferred instead. A C-Style cast expression in C++ is interpreted as the first of the following which satisfy the requirements of the respective cast:

1. `const_cast`
2. `static_cast` with extensions, i.e. a pointer or reference to a derived class is allowed to be cast to a pointer or reference to the base class
3. `static_cast` with extensions followed by `const_cast`
4. `reinterpret_cast`
5. `reinterpret_cast` followed by `const_cast`

As is evident above, the exact behavior of a C-Style cast can vary widely, which is why it's preferred to use explicit C++ casts.

## Functional Casts

A functional cast expression is a single-word type name followed by a single expression in parentheses. It's exactly equivalent to the corresponding C-style cast expression, i.e.

``` cpp
int(45) == (int)45
```

## static_cast

The sequence that `static_cast` follows is:

1. if `new_type` can be direct-initialized from the expression, then return a `new_type` temporary
2. if `new_type` is pointer or reference to `D` and expression is pointer or reference to its base `B`, then perform a unsafe downcast
3. if `new_type` is rvalue reference, then return an xvalue referring to expression
4. if `new_type` is `void`, then discard the value of the expression after evaluation
5. given the existence of a standard conversion sequence from `new_type` to expression type, it can perform the inverse of the conversion
6. it can perform explicit conversion of lvalue-to-rvalue, array-to-pointer, or function-to-pointer
7. a scoped enumeration can be converted to integer or floating-point
8. an integer, floating-point, or enumeration type can be converted to any enumeration. It is undefined behavior if the value is out of the range of target enumeration type
9. a pointer-to-member of class `D` can be unsafe upcast to pointer-to-member of base class `B`. Note that there is no check to ensure that the member actually exists in base class.
10. prvalue of type pointer to `void` can be converted to pointer to any type

`static_cast` can be used to disambiguate function overloads by performing a function-to-pointer conversion with a specific type:

``` cpp
// choose std::toupper(int)
static_cast<int(*)(int)>(std::toupper);
```

`static_cast` can be used to perform an unsafe downcast:

``` cpp
D d;
B& b_ref = d;

D& downcast = static_cast<D&>(b_ref);
```

## const_cast

`const_cast` can be used to cast away `const`-ness or volatility (`volatile`). The sequence consists of:

1. two possibly multi-level pointers to the _same type_ may be converted between each other regardless of cv-qualifiers at each level
2. lvalue may be converted to lvalue reference or rvalue reference of the same type of any cv-qualification
3. rvalue may be converted to rvalue reference of the same type of any cv-qualification
4. null pointer value may be converted to null pointer value of `new_type`

It is undefined behavior to use `const_cast` to remove `const`-ness or volatility from a pointer or reference and using it to either write to an object declared `cosnt` or access an object declared `volatile`.

## dynamic_cast

The behavior of `dynamic_cast` in the event of failure depends on whether `new_type` is a pointer type or a reference type. If it's a pointer type, `dynamic_cast` returns a `nullptr`. If it's a reference type, `dynamic_cast` throws an `std::bad_cast` exception.

The sequence of `dynamic_cast` consists of:

1. can add `const`-ness
2. `dynamic_cast<T>(nullptr)` → `nullptr` of `T`
3. `dynamic_cast<Base*>(Derived*)` → `Base*`
4. `dynamic_cast<void*>(Polymorphic*)` → `DynamicType*`
5. `dynamic_cast<Derived*>(Base*)` → `Derived*`, i.e. _downcast_, if the dynamic type of `Base` IS-A `Derived`
6. `dynamic_cast<OtherBase*>(Base*)` → `OtherBase*`, i.e. _sidecast_, if dynamic type of `Base` IS-A `OtherBase`
7. if not 5 or 6:
    1. `dynamic_cast<T*>(expr)` → `nullptr`
    2. `dynamic_cast<T&>(expr)` → `throw std::bad_cast`
8. if within a constructor or destructor, and the expression refers to an object currently under construction or destruction, that object is the dynamic type. It is undefined behavior if `new_type` is not a pointer or reference to the constructor or destructor's own class or one of its bases

The _static type_ of an expression is the one that results from compile-time analysis, it doesn't change during run-time.

The _dynamic type_ of an expression is the type of the most derived polymorphic object.

An incomplete type is one of:

* `void`
* a class type that has been declared (e.g. forward declaration) but not defined
* an array of unknown size
* an array of elements of incomplete type
* an enumeration type from point of declaration until the underlying type is determined

## reinterpret_cast

A `reinterpret_cast` is purely a compiler directive to treat a sequence of bits of the expression as if it had the type `new_type`.

Given the cast:

``` cpp
reinterpret_cast<AliasedType>(DynamicType)
```

The strict aliasing rule says that the cast always succeeds, but the resulting pointer can only be used if one of the following holds, otherwise it is undefined behavior:

* `AliasedType` is the (possibly cv-qualified) `DynamicType`
* `AliasedType` and `DynamicType` are both pointers to same type `T`
* `AliasedType` is a base class of `DynamicType`
* `AliasedType` is `char` or `unsigned char`
* `AliasedType` is the signed or unsigned variant of `DynamicType`
* `AliasedType` is an aggregate or `union` type which holds one of the types as element or member

## Signed-to-Unsigned

When a signed value is assigned to an unsigned variable, the underlying bit representation **is not altered**. Instead, the signed value is simply treated literally as if it were an unsigned value.

If the signed value is negative, then it is likely represented at the bit-level in [Two's Complement](http://en.wikipedia.org/wiki/Two%27s_complement). For example, given:

``` cpp
uint8_t var = -1;
```

The value `-1` is encoded by first representing it as a positive number:

<div>$$ 0000\ 0001 $$</div>

The digits are then flipped, so that 1s become 0s and vice versa:

<div>$$ 1111\ 1110 $$</div>

Finally, the value is incremented by 1 to arrive at the Two's Complement representation of `-1`:

<div>$$ 1111\ 1111 $$</div>

When this value is assigned to an unsigned integer, the value is simply interpreted as if it were unsigned to begin with. Therefore, this value is interpreted as being `255`.

## Integer Promotion

In general, operands are converted to the same type of the widest operand type in the expression. Loss of precision is avoided, so this also means that when integral and floating-point values are mixed, they're all converted to floating-point values.

**Integer promotion** concerns converting small integral types to larger integral types.

`bool`, `char`, `signed char`, `unsigned char`, `short`, `unsigned short` are promoted to `int` if all possible values fit within an `int`. Otherwise, they are promoted to `unsigned int`.

Larger types are promoted to the smallest type of `int`, `unsigned int`, `long`, `unsigned long`, `long long`, or `unsigned long long` which fits the value.

## Mixing Unsigned Types

If the types still don't match but the signs match, then the type of the smaller value is promoted to the type of the larger value.

If the signs don't match and the type of the unsigned operand is the same as or larger than that of the signed operand, then the signed operand is converted to unsigned as described in [Signed-to-Unsigned](#signed-to-unsigned), which most likely yields unexpected behavior.

If the signs don't match and the type of the unsigned operand is smaller than that of the signed operand, the **result is machine-dependent**. If all values in the unsigned type fit in the larger signed type, it's converted to the larger signed type. Otherwise, the signed operand is converted to the unsigned type as described in [Signed-to-Unsigned](#signed-to-unsigned), which most likely yields unexpected behavior.

## Negative Modulus

The modulus operation `%` simply calculates the remainder of the left expression divided by the right expression. There is confusion when it comes to modulus operations with negative operands, which as far as I know isn't clearly defined mathematically. For example, the operation `-1 % 256`.

The equation generally used to calculate the modulus is:

<div>$$ \text{mod}(a, n) = a - \lfloor a / n \rfloor * n $$</div>

The operation `-1 % 256` yields the result `255` with this implementation. This is the result yielded in languages such as Python and Ruby.

C and C++ uses the same equation as the above, **but** the division operation has an additional restriction when used with negative operands:

<div>$$ \text{div}(-a, n) = \text{div}(a, -n) = -(a/n) $$</div>

With these definitions, the division of `-1 / 256` in the above equation becomes `-(1 / 256)`. The result of `1 / 256` is zero due to truncation. The negation of this result is still zero, so the result of the modulus operation is simply `-1`, which is **very different** from the result of `256` yielded above without these restrictions.

Given the above restriction on the division operation with negative operands, the definition of the modulus operation with negative operands can be simplified to:

<div>
$$
\begin{align}
  \text{mod}(\phantom {-} a, -n) &= \phantom {-} \text{mod}(a, n) \\
  \text{mod}(-a, \phantom {-} n) &= -\text{mod}(a, n)
\end{align}
$$
</div>

# Exceptions

A _function-try-block_ is a way of wrapping an exception handler around a function body.

``` cpp
int func(int n) try {
  ++n;
  throw n;
} catch (...) {
  assert(n == 4);
  return n;
}
```

A catch-all handler can be specified with three dots `...` as the catch parameter.

If a catch handler doesn't match the exception thrown in the corresponding try-block, then the exception is rethrown to the containing try-block, or `std::terminate` is called if there is none.

A catch handler can rethrow the caught exception explicitly to propagate it up the call stack by using the empty `throw;` statement.

A `noexcept` specification specifies that a function throws exceptions if the expression argument evaluates to `true`. If missing, it is assumed to be `noexcept(false)`, meaning that the function _may_ throw exceptions, whereas `noexcept` on its own is equivalent to `noexcept(true)`.

A `noexcept` specification is part of the function type, so it can be used for function parameters that are function pointers to function that don't throw, or to create type aliases for pointers to functions that don't throw.

``` cpp
void f() noexcept { … }

// func is noexcept is T()
// constructor is also noexcept
template<class T>
void foo() noexcept(noexcept(T())) { … }
```

If a function declaration is given a `noexcept` specification, _all_ other overloads must have the _same_ `noexcept` specification.

The `noexcept` operator performs a compile-time check and returns `true` if the expression argument is declared to not throw exceptions. It is usually used in a `noexcept` specifier to encode the possibility for example that a function may throw if a function it uses may throw for a given parameter.

Destructors are implicitly `noexcept` unless the class contains a member whose destructor is explicitly `noexcept(false)`.

A function-try-block can also be used around a constructor as a way to catch exceptions during initialization within a member initializer list. It begins before the function body and includes the member initializer list. Every catch must terminate by throwing an exception, otherwise an implicit rethrow occurs at the end of a catch clause scope.

``` cpp
class A {
  A() try : x(0), y(0) {
    // succeeded
  } catch (std::exception &e) {
    // failed
  }
};
```

A function-try-block can also be used around a destructor. The catch clauses may perform explicit `return`s, otherwise an implicit rethrow is occurs at the end of a catch clause scope.

Some classes make guarantees about what occurs in the event that exceptions are thrown. For example, `std::vector` guarantees that if an exception occurs during `push_back`, the original `vector` would be left unchanged. In the event that the `push_back` would have had to reallocate space, if the `vector` decided to use the move constructor to move the objects to the new space and an exception were thrown at some point, the original `vector` would be left in an inconsistent state, with some of its elements having been moved to the new allocation of memory.

For this reason, such classes use copy constructors unless they are guaranteed that a type's move constructor doesn't throw exceptions. This guarantee is specified using the `noexcept` declaration on a function definition as shown above.

# Lambdas

A lambda expression is conceptually transformed into an unnamed prvalue temporary object of a unique, unnamed, non-union, non-aggregate type that overloads its function-call operator, with one data member for each captured variable.

Lambda captures by copy can't be modified if the `mutable` keyword is missing after the parameter list. The `mutable` keyword allows the lambda body to modify the parameters captured by the copy. Conversely, a lambda _with_ the `mutable` keyword essentially removes the `const`-qualifier for the converse effect.

Conceptually, a lambda without a `mutable` keyword has the effect of adding a `const`-qualifier to the function-call operator's declaration, thereby preventing the lambda body from mutating the captured variables.

If the lambda's return type is omitted, it is assumed to be `auto`.

Conceptually, a lambda with a parameter type of `auto` has the effect of making the function-call operator a template function with the corresponding parameter as a template parameter.

If the lambda's parameter list is omitted, it is assumed to be empty:

``` cpp
[capture list] { body }
```

| Captures  | Effect                        |
|:----------|:------------------------------|
| `[a, &b]` | `a` by-value, `b` by-referece |
| `[this]`  | `this` by-value               |
| `[&]`     | default all by-reference      |
| `[=]`     | default all by-value          |
| `[]`      | capture nothing               |

If a lambda capture list contains a capture-default specifier, other captures can't use the same capture type. That is, if by-value is specified as the capture default, any other listed captures must be by-reference.

Default capture specifiers are discouraged because they can lead to implicit and unexpected captures, which can lead to dangling references.

Note that by-value captures in lambdas can still cause dangling references if pointers are captured. Likewise, by-value default captures can implicitly capture the `this` pointer when a data member of method is accessed within the lambda. That can be avoided by using a generalized capture to create a local copy.

``` cpp
[age = this->age](int years) {
  return age + years;
};
```

`static` variables are essentially captured by-reference, even given a by-value default capture mode. Use a generalized capture initializer to explicitly perform a copy.

A lambda capture with an initializer has the effect of declaring and explicitly capturing a variable with type `auto`. This is useful for capturing move-only types:

``` cpp
[move_only_obj = std::move(move_only_obj)](const int param) {
  …
}
```

It's also possible to capture by reference:

``` cpp
[&my_ref = some_lvalue]() {
  …
}
```

A lambda capture initializer can also be used to inject values into the lambda:

``` cpp
[pw = make_unique<Widget>("test")]() {
  // use pw
};
```

# Enumerations

_Scoped enumerations_ can be created to avoid symbol clashing and enumerations' underlying type can be specified explicitly:

``` cpp
enum class EventType : uint8_t { STATUS, LOG, ERROR };

EventType type = EventType::STATUS;
```

Scoped enumerations can be forward-declared because the underlying type is _always_ known because it is either explicitly specified or the default is `int`. Unscoped enumerations can only be forward-declared if the underlying type is explicitly specified in the forward-declaration.

Unscoped enumerations introduce their enumerators into the enclosing scope. If an unscoped enumeration is defined in a class, the enumerators are accessible with the member access operators.

``` cpp
struct X {
  enum Direction { Left, Right };
};

X x;
X *p = &x;

// enumerator `Left` is accessible via
X::Direction::Left
X::Left
x.Left
p->Left
```

If the underlying type is not explicitly specified, then the type will either be `int` or the largest integral type that can represent all values.

Enumerators of unscoped enumerations are implicitly convertible to integral types, while those of scoped enumerations must perform an explicit `static_cast`.

``` cpp
// unscoped enumeration, implicit conversion
enum Color { RED, GREEN = 20, BLUE };

Color r = BLUE;

int n = r;  // n == 21

// scoped enumeration, explicit conversion
enum class Color { RED, GREEN = 20, BLUE };

Color r = Color::BLUE;

int n = static_cast<int>(r);  // n = 21
```

Each _enumerator_, i.e. a possible enumeration value, can be associated with a value of a `constexpr`. If an enumerator doesn't have an initializer, it takes on the value of the previous enumerator plus 1, or zero if it's the first.

Initializers can refer to previous enumerators.

``` cpp
enum Foo {
  A,
  B,
  C = 10,
  D,
  E = 1,
  F,
  G = F + C
};

// A = 0
// B = 1
// C = 10
// D = 11
// E = 1
// F = 2
// G = 12
```

# Temporaries

The lifetime of a temporary is extended to match the lifetime of a reference bound to it, except when:

* A temporary is bound to a return value of a function. In this case, the temporary is destroyed immediately at the end of the `return` expression, yielding a dangling reference.

* A temporary is bound to a reference parameter in a function call. In this case, the temporary exists until the end of the full expression containing the function call. If the function returns a reference to the temporary, it becomes a dangling reference.

* A temporary is bound to a reference in a `new`-expression initializer. In this case, the temporary exists until the end of the full expression containing the `new`-expression. If the initialized object outlives the full expression, its reference member becomes a dangling reference.

# Value Categories

An lvalue ("left value") expression is one that has identity and cannot be moved from. It designates a function or object. Note that the _name_ of a variable or function in scope, even if the variable type is rvalue reference, is itself an lvalue.

An xvalue ("expiring value") expression is one that has identity and can be moved from, i.e. it's "expiring". It's usually near the end of its lifetime, e.g. a function-returned rvalue reference.

A glvalue ("generalized lvalue") expression is one that has identity but may or may not be moved from, i.e. it's _either_ an lvalue _or_ an xvalue.

A prvalue ("pure rvalue") expression is one that has no identity and cannot be moved from, i.e. it's an rvalue and is not an xvalue, e.g. a function-returned value that is not a reference.

An rvalue ("right value") expression is one that can be moved from, but may or may not have identity, i.e. it's _either_ a prvalue _or_ an xvalue.

| Category   | Has Identity   | Can Move From   |
| :--------- | :------------: | :-------------: |
| lvalue     | ✓              | ❌               |
| xvalue     | ✓              | ✓               |
| glvalue    | ✓              | ?               |
| prvalue    | ❌              | ❌               |
| rvalue     | ?              | ✓               |

When a glvalue appears where a prvalue is expected, the glvalue is converted to a prvalue.

An lvalue transformation is one of:

* lvalue-to-rvalue conversion
* array-to-pointer conversion
* function-to-pointer conversion

An lvalue-to-rvalue conversion is one in which a prvalue temporary object is copy-constructed from a glvalue.

# Static Variables

Local `static` variables are initialized the first time control passes through their declaration. Subsequent passes skip the declaration.

If the initialization of local static variable throws an exception, the variable isn't considered to be initialized, and so initialization is attempted again on the next pass.

Static local variable destructors run at program exit as long as they were ever successfully initialized.

Static members can be declared in the class definition and defined outside of it.

``` cpp
struct S {
  static int X;  // declaration
};

int S::X = 3;  // definition
```

# Templates

All special member functions can be templates except for copy-constructors and destructors.

## Parameter Packs

Parameter packs can be used to allow a function to accept an arbitrary number of parameters with potentially differing types.

``` cpp
template <class ...Us>
void f(Us... pargs) {
  …
}

template <class ...Ts>
void g(Ts... args) {
 f(&args...); // into &a1, &a2, etc
}
```

In order to process each parameter, possibly a different way based on its type, the recursive variadic template function pattern may be used. For example, in the following code, the second function is called as long as there is more than one remaining parameter. As soon as there is only one remaining parameter, the first function is called which only prints that parameter, terminating the recursion.

``` cpp
void print(T t) {
  std::cout << t;
}

template <typename T, typename ...Targs>
void print(T t, Targs... args) {
 std::cout << t;
 print(args...);
}
```

It's similar to the following contrived Haskell code:

``` haskell
print [x] = putStrLn $ show x

print (x:xs) = do putStrLn $ show x
                  print xs
```

The size of a parameter pack can be obtained with the `sizeof...` operator.

``` cpp
template <typename... Ts>
constexpr auto make_array(Ts&&... ts)
 -> std::array<std::common_type_t<Ts...>, sizeof...(ts)> {
    return { std::forward<Ts>(ts)... };
}

std::array<int, 3> a = make_array(1, 2, 3);
```

## Template Specialization

A partial template specialization is one where some but not all of the template parameters are specialized in a template specialization.

Members of partial specializations aren't related to members of the primary template.

``` cpp
template<class T, int I>
struct A {
  void f();
};

template<class T, int I>
void A<T, I>::f() { }

template<class T>
struct A<T, 2> {
  void f();
};

A<char, 0> a0;
a0.f();  // ok: f() definition in primary template A<T, I>

A<char, 2> a2;
a2.f();  // error: no f() definition in partial spec A<T, 2>
```

When an enclosing class template is fully specialized, all of the partial specializations of member templates are ignored for the given specialization of the enclosing class.

A full class template specialization can change the base class.

``` cpp
template<typename T>
struct is_void : std::false_type {};

// full specialization for T = void
template<>
struct is_void<void> : std::true_type {};
```

A member of a class template specialization doesn't require an explicit template argument list.

``` cpp
template <typename T>
struct A {
  struct B { … };

  template <class U>
  struct C { … };
};

template <>
struct A<int> {
  void f(int);
};

// no template <>
void A<int>::f(int) { … }
```

A member or member template of a class template can be explicitly specialized even if it is defined in the class template definition.

``` cpp
template <typename T>
struct A {
    void h(T) { … }
};

template <>
void A<int>::h(int) { … }
```

A nested class member template cannot be specialized if its enclosing class is not specialized.

``` cpp
template <class T1>
class A {
  template <class T2>
  class B {
    template <class T3>
    void mf1(T3);

    void mf2(); // non-template member
  };
};

// error:
//   member template B<double> is specialized
//   so its enclosing class A must be specialized
template <class Y>
template <>
void A<Y>::B<double>::mf2() { … }
```

## Variable Templates

A variable template can be used to define a variable with different values based on the type.

``` cpp
template <class T>
constexpr T pi = T(3.1415926535897932385);

// specialization
template <>
constexpr int pi<int> = 3;

template <class T>
T circular_area(T r) {
  return pi<T> * r * r;
}
```

When used in a class scope, variable templates declare a static data member template.

## Function Templates

When the template argument list (even if empty) is omitted, overload resolution examines both template and non-template overloads.

## Template Template Parameters

Template template parameters make it possible to accept a template as a template argument [^HKT]. It can be read as "template parameter that itself is a template."

[^HKT]: This reminds me of Higher-Kinded Types

``` cpp
template <template <typename>
          class Container,
          typename Element>
class Thing {
  Container<Element> things;
};

Thing<std::vector, int> thing;
```

## Integral Template Parameters

These are used for example with `std::array` to specify the dimension.

``` cpp
template <int N>
struct S {
  int a[N];
};
```

## Explicit Template Instantiation

It's possible to explicitly instantiate a class and all of its members for the provided template arguments.

``` cpp
// instantiated Vector<int> definition
template class Vector<int>;

template <typename T>
void f(T s) {
  std::cout << s << '\n';
}

template void f<double>(double);
template void f<>(char);
template void f(int);
```

It's possible to signal that a given template instantiation is explicitly instantiated in another compilation unit, so that the current one should not instantiate it either implicitly or explicitly.

``` cpp
extern template Vector<int>;

extern template void f<double>(double);
```

An explicit template specialization must be declared in the same namespace as the primary template, after its definition, before the first use that would cause an implicit instantiaton.

An explicit template specialization of a function template is `inline` only if it's declared with the `inline` specifier, regardless of whether the primary template is `inline` or not.

An explicit template specialization of a function template cannot be a friend declaration.

An explicit template specialization of a function template cannot contain default function arguments.

## Implicit Template Instantiation

Implicit class template instantiation occurs when a completely defined type of a class template is needed. For example, when an object of that type is instantiated, but not when a pointer to that type is constructed. The same applies to members of class type, i.e. they're only instantiated if they're used.

Implicit function template instantiation occurs when code requires the function definition to exist and it hasn't been explicitly instantiated.

## Member Templates

A member function template cannot be virtual.

A member function template in a derived class cannot override a virtual member function from the base class, i.e. it may exist alongside one that does override.

``` cpp
class Base {
  virtual void f(int);
};

struct Derived : Base {
  // does not override B::f
  template <class T> void f(T);

  // override can call the template
  void f(int i) override {
    f<>(i);
  }
};
```

A member function template of special member function _does not_ prevent implicit generation of the corresponding member function.

Given a conflict between a template member function and a non-template member function, the non-template member function is chosen unless an explicit template argument list is supplied.

``` cpp
template <typename T>
struct A {
  void f(int);

  template <typename T2>
  void f(T2);
};

A<char> ac;
ac.f('c'); // template function f<char>(int)
ac.f(1);   // non-template function f(int)
ac.f<>(1); // template function f<int>(int)
```

A user-defined conversion function can be a template:

``` cpp
struct A {
  template <typename T>
  operator T*(); // conversion to pointer to any type
};

// out-of-class definition
template <typename T>
A::operator T*() {
  return nullptr;
}

// explicit specialization for char*
template <>
A::operator char*() {
  return nullptr;
}
```

Nested templates, such as a function template inside of a class template, all need to be specified in the same order when defining a function that depends on multiple types.

``` cpp
template <typename T>
struct String {
  template <typename S>
  int compare(const S&);
};

template <typename T>
template <typename S>
int String<T>::compare(const S& s) {
  …
}
```

## Template Argument Deduction

| Parameter | Argument | Deduction |
| :-------- | :------- | :-------- |
| `T&&`     | `A&`     | `A&`      |
| `T&&`     | `A&&`    | `A&&`     |

``` cpp
template<typename T>
void f(T&& param);
int x = 27;
f(x); // T = int&, param = int&

template<typename T>
void f(T&& param);
int&& x = 27;
f(x); // T = int, param = int&&

template<typename T>
void f(T&& param);
int y = 27;
const int& x = y;
f(x); // T = const int&, param = const int&

template <typename T>
void f(T param);
int x = 27;
f(x); // T = int, param = int

template <typename T>
void f(T param);
const int x = 27;
f(x); // T = int, param = int

template <typename T>
void f(T param);
int y = 27;
const int& x = y;
f(x); // T = int, param = int

template <typename T>
void f(T& param);
int x = 27;
f(x); // T = int, param = int&

template <typename T>
void f(T& param);
const int x = 27;
f(x); // T = const int, const int&

template <typename T>
void f(T& param);
int y = 27;
const int& x = y;
f(x); // T = const int, const int&

template <typename T>
void f(T param);
int[] array;
f(array); // array-to-pointer decay

template <typename T>
void f(T& param);
int[] array;
f(array); // reference to array
```

TAD for forwarding references follow normal TAD rules unless the argument is an lvalue, in which case the deduced type parameter and the parameter type are both lvalue references to the same type. A pointer-to-`const` remains pointer-to-`const` because the `const`-ness that would be ignored is that of the pointer, not what the pointer points to.

``` cpp
template<typename T>
void f(T param);

const char * const ptr = "test";
f(ptr); // T = const char *, param = cost char *

template<typename T>
void f(T param);

const char name[] = "abc";
f(name); // T = const char *, param = const char *

template<typename T>
void f(T& param);

const char name[] = "abc";
f(name); // T = const char[4], param = const char (&)[4]
```

For non-pointer, non-reference template parameters, TAD ignores the reference, `const`, or `volatile` components. The reason that TAD ignores the `const`-qualifier of a by-value template parameter is that, just because the argument can't be modified doesn't mean that a copy of the same type can't be.

TAD can be used to determine the size of an array.

``` cpp
template<typename N, std::size_t N>
constexpr std::size_t array_size(T (&)[N]) noexcept {
  return N;
}
```

TAD for no-forwarding reference parameters treat lvalue refereces as non-references, that is, given `int&`, it deduces `int`.

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

Forwarding references can be used to forward parameters exactly the same way they were passed.

``` cpp
template<class... Args>
Object emplace(Args&&... args) {
  return Object(forward<Args>(args)...);
}
```

This can also be done with lambdas by using `decltype`.

``` cpp
[](auto&&... params) {
  return Object(forward<decltype(params)>(params)...);
};
```

### Reference Collapsing

rvalue-references to template parameters have special rules. For example, given the definition:

``` cpp
template <typename T> void func(T&&);
```

If an lvalue `int` is passed to the function, a language rule states that the template parameter `T` will be deduced as being an lvalue-refernece, `int&`. This poses a problem, since the function parameter's type ends up being an lvalue-reference to an rvalue-reference, `int& &&`. A reference to a reference, of any type, can't usually be created but an **exception** is made for template parameters.

Template parameters that are deduced as being references to references undergo a process that is referred to as _reference collapsing_, the rules of which are as follows:

| Input    | Output  |
| :------- | :-----  |
| `X& &`   | `X&`    |
| `X& &&`  | `X&`    |
| `X&& &`  | `X&`    |
| `X&& &&` | `X&&`   |

Basically, if both references (the template parameter and the deduced type) are rvalue references, collapse to an rvalue reference. Otherwise, collapse to an lvalue reference.

Reference collapsing occurs in the following contexts:

1. template instantiation
2. auto variables
3. typedefs, using aliases
4. decltype

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

Perfect forwarding can fail with arguments that are:

* braced-init-lists
* null pointers 0 and NULL
* declaration-only integral `const` `static` data members
* template and overloaded function names
* bitfields

### Type-Matching

An rvalue-reference can be converted to a `const` reference. This means that if a class defines copy constructor but not a move constructor and as a result the compiler [defines the move constructor as deleted](#move-operation-synthesis), rvalue-references will type match with `const` references and as a result, rvalue-reference arguments will use the copy constructor seamlessly.

If an rvalue reference is bound to a temporary, it has the effect of extending the lifetime of the temporary while remaining modifiable, unlike `const`-lvalue references to temporaries.

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

# Classes

It's a good thing to remember that the _only_ distinction between a `class` type and a `struct` type is that `struct` has by default public visibility and `class` has default private visibility. That's all!

Members can be defined `mutable` so that they can be modified even in a `const` class or `const` member function. This should usually only be done when the member doesn't affect the externally-visible state of the class.

`const` member functions should be thread safe because they convey the idea of reading, not writing, so `const` member functions that perform mutations (e.g. to mutable members) should employ some means of synchronization.

Functions defined entirely within a class, struct, or union, are implicitly inline.

## Rule of Five

The copy constructor, move constructor, copy-assignment operator, move-assignment operator, and destructor should be thought of as a unit: if one needs to be defined, then the rest should be defined as well.

* if a class needs a destructor, it likely also needs a copy-assignment operator and copy constructor
* if a class needs a copy constructor, it likely so needs a copy-assignment operator, **and vice versa**

## Rule of Zero

This [recent rule] is unlike the [other two] in that it instead says that classes that contain custom destructors, copy/move constructors, or copy/move assignment operators should deal _exclusively_ with ownership, i.e. encapsulating a so called _ownership policy_ which handles the allocation and deallocation of a particular resource (via RAII). All other classes should **not have** custom destructors, copy/move constructors, or copy/move assignment operators.

[recent rule]: http://flamingdangerzone.com/cxx11/2012/08/15/rule-of-zero.html
[other two]: http://en.cppreference.com/w/cpp/language/rule_of_three

This rule is enforceable out-of-the-box in C++11 through the use of smart pointers such as `shared_ptr` and `unique_ptr` along with custom deleters when necessary.

## Class Initialization

Classes are initialized as follows:

1. virtual base classes in depth-first, left-to-right order
2. direct base classes in left-to-right order
3. default member initializers top-to-bottom
4. constructor initializer lists in top-to-bottom member definition order
5. constructor body initialization

### Member Initialization

The order of initializing member variables is:

1. default member initialization
2. constructor initializer lists in top-to-bottom member definition order
3. constructor body initialization

Constructor initializer lists initialize member variables. If a member variable is missing from the initializer list it is default initialized. Members that are `const` or references must be initialized in the constructor initializer lists. Members in a constructor initializer list are initialized in the order in which they are defined in the class definition.

It is considered best practice to use default member initializers for member variables, opting for constructor initializer lists for edge cases, and for constructor initialization in the worst case.

If a member has a default member initializer and also appears in a constructor's member initializer list, the default member initializer is ignored. The default member initializer can be thought of as the initializer to use if the member would otherwise be default-initialized.

Value initialization occurs when:

* in an array initialization, fewer declarations appear than the size of the array
* defining a local static object without an initializer
* explicitly requesting value initialization by writing expressions of the form `T()` where `T` is the name of the type

Member functions defined inside the class definition are inlined.

### List-Initialization

A _narrowing conversion_ is:

* from floating-point to integral type
* from `long double` to `double` or `float`, unless source is a `constexpr` and there is no overflow
* from `double` to `float`, unless source is a `constexpr` and there is no overflow
* from integral type to floating-point type, unless source is a `constexpr` whose value can be represented _exactly_ in the target type
* from integral or unscoped enumeration to an integral type that cannot represent all values of the original, unless source is a `constexpr` whose value can be represented exactly in the target type

A _braced-init-list_ is _not_ an expression, and by extension it has no type. For this reason, it can't be used as an argument to `decltype()`. Since it has no type, template argument deduction _cannot_ deduce a type that matches a braced-init-list.

The following is ill-formed because the braced-init-list has no type and thus a type cannot be deduced, so a function template cannot be instantiated.

``` cpp
template <class T>
void f(T);

f({1, 2, 3});
```

The `auto` keyword makes an exception in that it deduces any braced-init-list as an `std::initializer_list`.

The list-initialization sequence consists of:

1. If `T` is a class type and there's a single element that IS-A `T`, then initialize from that element.
2. If `T` is a character array and there's a single element that is a string literal, then initialize from the string literal
3. If `T` is an aggregate type, then perform [aggregate-initialization](#aggregate-initialization).
4. If `T` is a class type with a default constructor and the braced-init-list is empty, then perform [value-initialization](#value-initialization)
5. If `T` is an `std::initializer_list`, then initialize from the temporary rvalue `std::initializer_list`.
6. If `T` is a class type then overload resolution chooses between its constructors:
    1. those that only take an `std::initializer_list` (despite default parameters)
    2. those with parameters matching the braced-init-list elements as arguments, barring narrowing conversions.
7. If there's a single element of type `E` and `T` is not a reference type, or _is_ a reference type _and_ IS-A `E`, then initialize from the element barring narrowing conversions
8. If `T` is an lvalue `const` reference or rvalue reference, then bind to the rvalue reference of the list-initialize temporary
9. If the braced-init-list is empty, then perform [value-initialization](#value-initialization).

### Aggregate-Initialization

An aggregate is an object is either an array or a class type that has:

* no `private` or `protected` members
* no user-provided constructors, though it's allowed to explicitly mark them `default` or `delete`
* no base classes
* no virtual member functions
* no default member initializers

Aggregate-initialization occurs whenever an aggregate type is initialized.

The aggregate-initialization sequence consists of:

1. Copy-initialize each array element or member
2. If the braced-init-list's size is greater than the number of members or empty, then the remaining members are initialized by their default member initializers, or if there are none, using [value-initialization](#value-initialization).

    A consequence of this is that it's possible to initialize only the first column of a 2D array, for example, such that the following two arrays are equivalent:

    ``` cpp
    int a[2][2] = {{1}, {2}};
    int b[2][2] = {{1, 0}, {2, 0}};
    ```

3. Braces around nested subaggregate initializer lists may be omitted, i.e. the following two initializations are equivalent:

    ``` cpp
    int a[2][2] = {1, 2, 3, 4};
    int b[2][2] = {{1, 2}, {3, 4}};
    ```

### Constant-Initialization

Constant-initialization occurs after zero-initialization of static and thread-local objects, but before _all other_ initializations.

### Copy-Initialization

Copy-initialization only considers non-explicit constructors and non-explicit user-defined conversion functions.

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

Given a target type `T` and initializing expression `E`, the copy-initialization sequence consists of:

1. If `T` is a class type and `E` IS-A `T`, overload resolution chooses the best converting constructor
2. If either:

    * `T` is a class type and `E` is of a different type
    * `T` is not a class type but `E` is a class type

    Then overload resolution chooses the best user-defined conversion, the result of which is used for direct-initialization
3. If `T` nor `E` are class types, standard conversions are used

### Copy-List-Initialization

Copy-list-initialization, like copy-initialization, only considers non-explicit constructors.

Copy-list-initialization occurs when:

* initialization of a named variable with a braced-init-list _after_ an equals sign `=`:

    ``` cpp
    T object = {arg, …};
    ```

* in a function call expression with a braced-init-list as an argument which is used to list-initialize the function parameter:

    ``` cpp
    function({arg, …});
    ```

* in a `return` statement with a braced-init-list which is used to list-initialize the returned object:

    ``` cpp
    return {arg, …};
    ```

* in a subscript expression with a user-defined `operator[]` which list-initializes the parameter of the overloaded operator:

    ``` cpp
    object[{arg, …}];
    ```

* in an assignment expression which list-initializes the parameter of the overloaded `operator=`:

    ``` cpp
    object = {arg, …};
    ```

* in a functional cast expression which uses a braced-init-list to copy-list-initialize a constructor's _parameter_:

    ``` cpp
    U({arg, …});
    ```

* in a data member initializer of a non-static data member that uses an equals sign `=`:

    ``` cpp
    class A {
      T member = {arg, …};
    };
    ```

### Direct-Initialization

Direct-initialization considers all constructors.

Direct-initialization occurs when:

* Non-empty parenthesized initialization

    ``` cpp
    T obj(arg1, arg2, …);
    ```

* List-initialization

    ``` cpp
    T obj { arg1, arg2, … };
    ```

* Functional cast

    ``` cpp
    T(other);
    ```

* Static cast

    ``` cpp
    static_cast<T>(other);
    ```

* Base or member constructor initializer list

    ``` cpp
    T::T() : member(arg1, arg2, …) { … };
    ```

* By-copy closure captures

    ``` cpp
    [arg]() { … }
    ```

The direct-initialization sequence consists of:

* If `T` is a class type, overloaded resolution determines the best constructor
* Otherwise if `T` is a non-class type, use standard conversions

### Direct-List-Initialization

Direct-List-Initialization occurs when:

* Initializing a named variable with a braced-init-list:

    ``` cpp
    T object{arg, …};
    ```

* Initializing an unnamed temporary with a braced-init-list:

    ``` cpp
    T{arg, …};
    ```

* Initialization of dynamic object with a braced-init-list:

    ``` cpp
    new T{arg, …};
    ```

* Default member initializer for non-static data member that doesn't use an equals sign `=`:

    ``` cpp
    class A {
      T member{arg, …};
    };
    ```

* Member initialization when using a braced-init-list in a member initializer list:

    ``` cpp
    class A {
      A() : T{arg, …} {
        …
      }
    };
    ```

### Default-Initialization

Default-initialization occurs when no parentheses or empty parentheses are used with a constructor, or when a base class or member is omitted from a constructor initializer list and there is no default member initializer.

Given a target type `T`, the default-initialization sequence consists of:

* If `T` is a class type, overload resolution determines the best (default) constructor.
* If `T` is an array type, every element is default-initialized.
* Otherwise objects with automatic storage and their subobjects are initialized to _indeterminate values_

### Value-Initialization

Value-initialization occurs when:

* Nameless temporary object created with empty parentheses of braces

    ``` cpp
    T();
    T{};
    T *t = new T();
    T *t = new T{};
    ```

* Named temporary object with empty braces

    ``` cpp
    T object{};
    ```

* Member initializer with empty parentheses or braces

    ``` cpp
    T::T() : member() { … }
    T::T() : member{} { … }
    ```

The value-initialization sequence consists of:

* If `T` is a class type with either:

    * no default constructor
    * a user-provided default constructor
    * deleted default constructor

    Then the object is [default-initialized](#default-initialization).

* If `T` is a class type with a default constructor that's _neither_ user-provided _nor_ deleted (i.e. it may be defaulted or implicitly-defined), then the object is [zero-initialized](#zero-initialization) and then if it has a non-trivial default constructor it is also [default-initialized](#default-initialization).

* If `T` is an array type, each element is value-initialized

* Otherwise the object is [zero-initialized](#zero-initialization).

### Zero-Initialization

Zero-initialization occurs when:

* For every named variable with static or thread-local storage duration, _before any other initialization_:

    ``` cpp
    static T object;
    ```

* As part of the value-initialization sequence for non-class types of members of value-initialized class types that have no constructors:

    ``` cpp
    int();
    ```

* Character array is initialized with string literal that is too short, so remainder of the array is zero-initialized:

    ``` cpp
    char array[5] = "";
    ```

The zero-initialization sequence consists of:

| Type                  | Effect                                                         |
| :-------------------- | :------------------------------------------------------------- |
| scalar                | integral constant zero explicitly converted to `T`             |
| non-union class type  | base classes & members zero initialized; constructors ignored  |
| union type            | first named member is zero-initialized                         |
| array                 | each element is zero-initialized                               |
| reference             | nothing                                                        |

## Access Specifiers

Access specifiers are used for class members in class definitions and on base classes when inheriting.

Class member access specifiers specify the _external visibility_ of members, including deriving classes. That is, if a member is given `private` visibility, not even a deriving class will be able to access that member, unless it's a `friend`. The default member access is `public` for `struct`s and `private` for `class`es.

| Specifier   | Accessible from                                        |
| :---------- | :----------------------------------------------------- |
| `public`    | anywhere                                               |
| `protected` | members and friends of the class and direct inheritors |
| `private`   | members and friends of the class                       |

Inheritance access specifiers specify the _external visibility_ of _inherited members_, which means this affects classes that derive from the derived class. The default inheritance access specifier is `public` for `struct`s and `private` for `class`es.

| Specifier   | Effect                                                      |
| :--------   | :---------------------------------------------------------- |
| `public`    | inherited members retain their member access                |
| `protected` | inherited `public` members become `protected`               |
| `private`   | inherited `public` and `protected` members become `private` |

In effect, member access specifiers specify what and how members are externally visible, whereas inheritance access specifiers are a way to specify what and how inherited members _which are accessible to the derived class_ are externally visible.

For example, consider base `A`, inheritor `B: A`, and `C: B`:

``` cpp
class A { int x; };
class B : A {};
class C : B {};
```

If `A` marks member `x` as `private`, then `B` nor `C` will be able to access it. However, if `A` were to specify a `public` accessor method for `x` then `B` would be able to access it that way. It makes sense that `A` should be able to guard access to its internal state so that inheritors don't come to rely on it or even corrupt it.

``` cpp
class A { private: int x; };

// can't access x
class B : A {};
class C : B {};
```

If `A` marks member `x` as `public` but `B` inherits `A` as `private`, i.e. `B: private A`, then `C` won't be able to access `x`. In effect, `B` has "closed off" access to `x`. This can be useful if `B` has added some behavior around `A`'s internal state that it doesn't want inheritors to mess up.

``` cpp
class A { public: int x; };
class B : private A {};

// can't access x
class C : B {};
```

Note that a `protected` member is only externally accessible from derived classes. If a derived class defines a method which accepts a parameter of the base class, the `protected` member won't be accessible through that parameter even though it's in a method of the derived class:

``` cpp
class A { protected: int x; };

class B : public A {
 void func(B&);  // can access A::x via B
 void func(A&);  // can't access A::x
};
```

A name that is private according to unqualified name lookup may still be accessible through qualified name lookup.

Accessibility for names of virtual functions is checked at the call point using the static type of the expression. Access of the final overrider is ignored.

## Base Classes

A base class should explicitly mark `default` all operations that it requires:

1. virtual destructor
2. move operations (won't be implicitly defined since destructor defined)
2. copy operations (won't be implicitly defined since move operations defined)

## Virtual Base Classes

A virtual base class is one that is included only once for every time it is inherited as a virtual base class in the hierarchy.

``` cpp
class T : public virtual B {};
```

## Default Constructors

The best practice is to always define a default constructor if any other constructors are defined.

Default constructors are synthesized only if all of the following criteria are met:

1. no other constructors are defined
2. all of the members of built-in or compound type have default member initializers
3. all members of class type have default constructors

If other constructors are defined but otherwise all other criteria is met for synthesizing a default constructor, the default constructor can be constructed using the `= default` directive:

``` cpp
class A {
  A() = default;
  A(int a, int b);
};
```

<!-- TODO this isn't specific to default constructors -->

Class members can be initialized inside the class definition. These initializers are known as _default member initializers_. Default member initializers must be defined either using the `=` assignment operator or list initialization syntax `{}`.

Constructors can _delegate_ their constructing to other constructors inside the constructor initializer list. In this case, the delegated constructor must be the only initializer in the member initializer list.

``` cpp
struct S {
  int m;

  S(int x) : m(x) { … }

  S(string s)
    : S(std::stoi(s)) {
    …
  }
};
```

Virtual functions can be explicitly overridden in derived classes using the `override` trailing keyword.

Class methods or entire classes can be defined `final` which prevents their overriding or deriving, respectively.

A default-constructor is trivial if:

* it performs no action
* it isn't user-provided
* there are no virtual bases or virtual member functions
* there are no members with default member initializers
* every direct base and member of class type has a trivial default-constructor

A default-constructor is implicitly-declared if:

* there are no user-defined constructors of _any kind_; OR
* user forces the declaration via `default`

An implicitly-declared default-constructor is deleted if:

* there's a reference member without a default member initializer
* there's a `const` member without a default-member initializer or user-defined default constructor
* there's a member without a default member initializer and a deleted or inaccessible default-constructor
* there's a direct base with a deleted or inaccessible default-constructor or destructor

An implicitly-declared, default-constructor is implicitly-defined if it's not deleted. The effect of the implicitly-defined default-constructor is the same as a constructor with an empty initializer list and body, i.e. it calls the default-constructors of bases and members (unless the member has a default member initializer).

## Destructors

Destructors do whatever work must be done to free resources used by an object, e.g. file handles. While in constructors the members are initialized before the constructor body runs, a destructor body's body executes first and then the members are destroyed afterward, in the reverse order of declaration in the class definition.

A destructor can be called directly on an object, but it is undefined behavior to do so more than once. For this reason, directly calling a destructor on a local object would yield undefined behavior when the destructor is automatically and implicitly called again at the end of the scope.

A destructor is trivial if:

* it isn't user provided
* it isn't virtual (nor is the base class destructor virtual)
* every direct base and class-type member has a trivial destructor

A destructor is implicitly-declared if there is no user-defined destructor provided.

An implicitly-declared destructor is deleted if:

* there's a direct base or member with a deleted or inaccessible destructor
* there's an implicitly-declared virtual destructor and a deleted or inaccessible `operator delete()`

An implicitly-declared destructor is implicitly-defined if it's not deleted or trivial. An implicitly-defined destructor simply has an empty body.

If one deletes `this`, then `this` and every pointer to the object becomes invalid, and no member function may be called.

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

The compiler can perform [copy elision](http://en.wikipedia.org/wiki/Copy_elision) to avoid unnecessary copies, short of using actual move semantics. Multiple copy elisions can be chained. Copy elision may occur when:

* _Named Return Value Optimization_ (NRVO): a class type is returned by-value and is the same type as function return type and isn't a function parameter object. It is constructed directly into the function return value.
* A non-reference, nameless temporary that would be moved/copied into an object of same type, is instead constructed directly into the storage of the object, known as _Return Value Optimization_ (RVO) when in a `return` context.

It is unportable to rely on the side-effects of copy/move constructors and destructors because some compilers don't perform copy elision in every situation where it is allowed, such as in debug mode.

Even if copy elision isn't performed, the return statement will attempt to use the move constructor to initialize the by-value return object, only copying if that fails.

A copy-constructor is implicitly-declared if there are no user-defined copy-constructors or the user forced it via `default`. The type of the copy constructor is either:

* `T::T(const T&)` if all direct bases and members have copy-constructors with `const`-reference or `const`-volatile
* `T::T(T&)` otherwise

An implicitly-declared copy-constructor is deleted if there exists:

* a direct base or member with a deleted or inaccessible copy-constructor
* a direct base with a deleted or inaccessible destructor
* a user-defined move-constructor or move-assignment operator
* an rvalue-reference member

An implicitly-declared copy-constructor is implicitly-defined if it's not deleted or trivial. The effect of an implicitly-defined copy-constructor is equivalent to a full member-wise copy of the bases and members using direct-initialization.

A copy-constructor is trivial if it performs a bytewise copy of the object and:

* it is not user-provided
* there are no virtual bases or virtual member functions
* every base and member has a trivial copy-constructor
* there are no `volatile` members

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

A copy-assignment operator is trivial if it creates an object copy as if by `std::memmove`, and:

* isn't user-provided
* there are no virtual bases or virtual member functions
* every direct base and class-type member has a trivial copy-assignment operator

A copy-assignment operator is implicitly-declared if there are no user-defined copy-assignment operators or the user forced it via `default`. The type of an implicitly-declared copy-assignment operator is:

* `T& T::operator=(const T&)` if each base and member has a copy-assignment operator with a parameter type of one of the following: `B`, `const B&`, `const volatile B&`
* `T& T::operator=(T&)` otherwise

An implicitly-declared copy-assignment operator is deleted if:

* user-declared move constructor or move-assignment operator
* there's a non-class type `const` member
* there's a reference member
* there's a member or direct base with a deleted or inaccessible copy-assignment operator

An implicitly-declared copy-assignment operator is implicitly-defined if it's not deleted or trivial. The effect of an implicitly-defined copy-assignment operator is simply to perform a member-wise copy-assignment of the bases and members.

## Move Constructors

Because rvalue-references serve as a sort of "tag" on an object that's about to be destroyed, functions can overload implementations specifically for such objects. In effect, a move constructor is used when overload resolution selects the move constructor---which is considered a converting constructor---because the argument is an rvalue expression.

An example of this would be a move constructor:

``` cpp
A::A(A &&moveFrom) noexcept :
  firstMember(moveFrom.firstMember),
  secondMember(moveFrom.secondMember) {
  moveFrom.firstMember = moveFrom.secondMember = nullptr;
  }
```

It's important to leave the moved-from object in a destructible state.

A move-constructor is implicitly-declared if there are no user-defined move-constructors and there are no user-declared:

* copy-constructors
* copy-assignment operators
* move-assignment operators
* destructors

An implicitly-declared move-constructor is deleted if there's:

* a direct base or member with a deleted or inaccessible move-constructor
* a direct base with a deleted or inaccessible destructor

A move-constructor is trivial if it performs a simple copy, same as a trivial copy constructor, and it is:

* not user-provided
* there are no virtual bases or virtual member functions
* every direct base and member has a trivial move-constructor
* there are no `volatile` members

An implicitly-declared move-constructor is implicitly-defined if it's not deleted or trivial. The effect of an implicitly-defined move-constructor is to perform a full member-wise move of object bases and members.

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

A move-assignment operator is implicitly-declared if there are no user-defined move-assignment operators or user-forced move-assignment operator via `default`. Also, there are no user-declared:

* copy constructors
* move constructors
* copy-assignment operators
* destructors

An implicitly-declared move-assignment operator is deleted if:

* there is a `const` or reference member
* there is a base or member with a `delete`d or inaccessible move-assignment operator

A move-assignment operator is trivial if it performs a simple copy, same as the trivial copy constructor, and:

* it isn't user-provided
* there are no virtual bases or virtual member functions
* every direct base and class-type member has a trivial move-assignment operator

An implicitly-declared move-assignment operator is implicitly-defined if it's neither deleted nor trivial. In this case, the implicitly-defined move-assignment operator simply performs a full member-wise move-assignment of direct bases and members.

### Synthesis

Unlike the copy operations that are _always_ synthesized if they're not otherwise defined or deleted, the compiler _only_ synthesizes move operations if the class doesn't define any copy operations and if every non-static data member is moveable. Moveable members include built-in types and those that define a move operation.

If a class defines move operations, the respective copy operation will be defined as deleted and must be defined explicitly.

If a default implementation is explicitly requested with the `default` directive, but the compiler can't define one due to the following reasons, then it will be defined as `deleted`:

* the class has a member that defines its own copy constructor but not a move constructor _or_ if the class has a member that doesn't define its own copy operations _and_ for which the compiler is unable to synthesize a move constructor. The same applies for move-assignment.
* the class has a member whose respective move operation is deleted or inaccessible
* the destructor is deleted or inaccessible
* the class has a `const` or reference member

## Conversions

<!--
TODO
perhaps create a parent Conversions section and under it
put the converting constructor and conversion operators sections
also merge the implicit conversions section
-->

The sequence order of implicit conversions is:

1. 0 or 1 standard conversion sequence
2. 0 or 1 user-defined conversion
3. 0 or 1 standard conversion sequence

The sequence of a standard conversion is:

1. 0 or 1 lvalue transformation
2. 0 or 1 numeric promotion or conversion
3. 0 or 1 function pointer conversion
4. 0 or 1 qualification adjustment

A user-defined conversion consists of:

1. 0 or 1 non-explicit, single-argument constructor or non-explicit conversion function call

An _explicit_ `bool` conversion operator can be used in an implicit conversion sequence when it's used in the context of:

* `if`, `while`, `for` conditions
* logical operators `!`, `&&`, `||`
* ternary operator `:?`
* `static_assert`
* `noexcept`

An expression `E` is implicitly convertible to `T` when an object of `T` can be copy-initialized with expression `E`.

The conversion ranks of primitive types are as follows. Note that the unsigned counterparts have equal rank:

1. `bool`
2. `char`
3. `short`
4. `int`
5. `long`
6. `long long`

Arithmetic conversions produce a common type by:

* if either operand is a scope `enum`, other must be same type
* if either operand is `long double`, other to `long double`
* if either operand is `double`, other to `double`
* if either operand is `float`, other to `float`
* if operand has integer type (`bool`, `char`, unscoped `enum` promoted to):
    * if both are signed or unsigned: operand with lesser conversion rank converted to operand with greater conversion rank
    * unsigned operand conversion rank ≤ signed operand conversion rank: signed operand converted to unsigned operand type
    * if signed operand's type can represent all values of unsigned operand: unsigned operand converted to signed operand's type
    * else: both operands are converted to unsigned counterpart of signed operand's type

### Converting Constructors

Converting constructors allow for the implicit conversion _from_ other types to the class type. Only one such implicit conversion is possible; it isn't possible to chain multiple such conversions. Examples of converting constructors are implicitly-declared or user-defined, non-explicit copy and move constructors.

To prevent a converting constructor from being used to perform an implicit conversion, the function can be marked `explicit`, in which case the constructor is _no longer_ considered a converting constructor.

``` cpp
explicit A(std::string &str) : internal(str) {}
```

Explicit conversion functions are only considered when performing [direct-initialization](#direct-initialization), whereas converting constructors are considered during [copy-initialization](#copy-initialization) as part of the user-defined conversion sequence.

Explicit conversion functions can be used via an explicit `static_cast` or with direct-initialization.

### Conversion Operators

Whereas [converting constructors](#converting-constructors) provide a way of converting another type to the class type, conversion operators provide a way of converting the class type to another type. They are defined using the `operator` keyword followed by the type it converts to.

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

### Conversion Ambiguity

It's pretty easy to get into a situation where it becomes ambiguous as to how a type is being converted.

In general:

* don't define mutually converting classes
* avoid conversions to built-in arithmetic types. If this is necessary, then:
    * don't define overloaded versions of operators that take arithmetic types since the conversion will handle it
    * don't define a conversion for more than one arithmetic type

However, it's probably best to try to completely avoid conversion functions with the exception of explicit conversions to `bool` and others that are very obvious.

#### Mutual Conversions

One way is to create a converting constructor to a type that itself defines a conversion operator to the original type.

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

Both `A` and `B` define mutual conversions. `A` defines a converting constructor that converts `B` to `A`, and `B` itself defines a conversion operator that converts from `B` to `A`. Therefore, the last line in the following code is ambiguous:

``` cpp
A f(const A&);
B b;
A a = f(b);
```

Because the conversion operation is ambiguous to the compiler, an error is emitted. Instead, it would have to be explicitly qualified:

``` cpp
A a1 = f(b.operator A()); // use B's converting operator
A a2 = f(A(b));           // use A's converting constructor
```

To avoid ambiguity, one should not define classes with mutual conversions.

#### Redundant Built-In Conversions

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
* **default constructor**: if a member has a deleted or inaccessible destructor _or_ has a reference member without a default member initializer _or_ has a `const` member whose type has no explicit default constructor and the member has no default member initializer

Any function can be marked `delete`, not just special member functions. This can be used to prevent certain inadvertent conversions from taking place, since overload resolution will automatically select it and the compiler will emit an error since it's deleted:

``` cpp
bool isLucky(int number);      // arg _must_ be int
bool isLucky(char) = delete;   // not char
bool isLucky(bool) = delete;   // not bool
bool isLucky(double) = delete; // not float or double

isLucky('a')  // error: call to deleted function
isLucky(true) // same
isLucky(3.5f) // same
```

`delete` can also be used to prevent certain template instantiations. This is accomplished by specializing the instantiation and deleting it.

``` cpp
// shouldn't be instantiated for use with
// void* or char*
template<typename T>
void processPointer(T *ptr);

template<>
void processPointer<void>(void*) = delete;

template<>
void processPointer<const char>(const char *) = delete;
```

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

### Virtual Functions

An _abstract base class_ is one that contains a pure abstract method, which is one that _must_ be implemented by children.

``` cpp
class T {
  virtual void func() = 0;
};
```

Sometimes a class needs to be abstract but there are no other functions available to declare as pure virtual. In this case, the destructor may be explicitly defined as pure virtual. However, it would need also require a definition, since all base class destructors are always called when a derived class is destroyed.

``` cpp
struct T {
  virtual ~T() = 0;
};

T::~T() {}
```

A _polymorphic class_ is one that declares or inherits at least one virtual function.

Function templates _cannot_ be declared `virtual`.

Virtual functions can be bypassed by using qualified name lookup:

``` cpp
Derived derived;
Base &base = derived;

// call Base::func not Derived::func
br.Base::func();
```

Note that virtual functions can't use return type deduction.

A function with the same name but different parameter list as a base class virtual function does _not_ override the base virtual function of the same name. Instead it shadows it during unqualified name lookup, unless it's called through a pointer or reference of the base type.

An overriding function can differ in its return type _only if_:

* both types are single-level pointers or references to classes
* the base virtual function's return type is a direct or indirect base class of the override's return type
* the override's return type is equally or less cv-qualified than the base return type

Virtual destructors are _automatically_ overridden in derived classes.

When a virtual function is called directly or indirectly in a constructor or destructor---including the construction or destruction of members---and the object to which the call applies is the object being constructed or destroyed, the function that is effectively called is the final overriding function in the constructor or destructor's class, that is, dynamic dispatch doesn't propagate down the inheritance hierarchy as usual. The more-derived classes don't exist yet during construction or destruction.

In a class with multiple bases, construction of one subobject restricts polymorphism to its class and its bases.

In a class with multiple bases, during the construction of a base subobject, obtaining a pointer or reference to a separate base subobject and calling a virtual function on it is undefined behavior. In practice, the virtual call is attempted using the current branch's class virtual table.

The choice of which virtual method to call is often made via _virtual tables_. A virtual table is constructed for each class containing or overriding virtual methods. Each class is given a virtual table pointer as a hidden member which points to that class' virtual table. Each virtual table entry contains the address of the appropriate, potentially-overriding implementation of the virtual method for that class. All type-compatible classes have virtual tables with the same layout, this enables a base class pointer to execute overridden methods.

A virtual method invocation on the class' first method can effectively look like this, given that `d` is the class:

``` cpp
(*((*d)[0]))(d)
```

1. `addr = (*d)[0]`

    Dereference the virtual table pointer to access the table, then get the address of the first method within it.

2. `(*addr)(d)`

    Invoke the method, passing the correct value of `this` as the first parameter.

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

Base classes that intend to be derived from should define their constructors as `virtual`, so that correct destructor is run through dynamic dispatch based on the dynamic type of the object being destroyed, instead of the static type. Otherwise destroying a polymorphic class with no virtual destructor is undefined behavior.

This has an implication with move semantics. If a destructor is defined, even as `default`, then no move operations are synthesized for that class. This issue percolates throughout the inheritance hierarchy, since classes don't synthesize operations that the base class doesn't define.

For this reason, the base class usually explicitly defines---even if as `default`---all of the operations it requires. First the virtual destructor for the aforementioned reasons, then move operations for the aforementioned reasons, and then the copy operations since they would otherwise not be synthesized since the move operations are explicitly defined.

``` cpp
class Base {
 public:
  virtual ~Widget() = default;

  Base(Base&&) = default;
  Base& operator=(Base&&) = default;

  Base(const Base&) = default;
  Base& operator=(const Base&) = default;
};
```

## Operator Overloading

Overloaded operators can be called in a function notation syntax.

``` cpp
std::string str = "A";

str.operator+=("B");        // str += "B"
operator<<(std::cout, str); // std::cout << str;
```

A _function object_ is one that overloads the function call operator `operator()`.

In order to distinguish pre-increment from post-increment operators, the post-increment operator overloads take a dummy integer parameter.

The canonical pre-increment operator implementation increments and returns a reference to the newly incremented value.

``` cpp
struct X {
  X& operator++() {
    this->value += 1;
    return *this;
  }
};
```

The canonical post-increment operator implementation copies the original value and increments the actual value, then returns the copy of the previous value. Often the actual increment can be delegated to the pre-increment operator.

``` cpp
struct X {
  X operator++(int) {
    X tmp(*this);     // copy
    this->value += 1; // increment
    return tmp;       // return original value
  }
};
```

This makes it obvious what the effect of `*it++` is. First it copies `it`, then increments the original, then dereferences the copy of `it`.

Instead of dealing with the complexity of overloading the subscript operator to enable multidimensional array access, since it would entail returning intermediary window/view objects, it is common to overload `operator()` to return a reference to the correct value, e.g. `matrix(i, j, k) = x`.

## Size of Every Object

The size of every object must be at least 1 byte in order to ensure that each object gets a distinct address. However, the _empty base optimization_ allows for empty base class subobjects to be zero-sized, since their address can be derived from the 1 byte allocated for the container class.

``` cpp
struct Base {};
struct Derived : Base { int i; };

assert(sizeof(Base) == 1);
assert(sizeof(Derived) == sizeof(int));
```

Notice that the size of `Derived` is just the size of its data member `i`, and not the size of `i` _plus_ the size of `Base`.

If one of the empty base classes is also the type or base of the type of the first member of the derived class, the empty base optimization _cannot_ occur, since the two base subobjects of the same type are required to have different addresses within the object representation of the derived class, since they are separate objects.

# Namespaces

C++17 allows specifying namespace paths in a condensed form:

``` cpp
// pre-C++17
namespace A {
  namespace B {
    namespace C {
      …
    }
  }
}

// C++17
namespace A::B::C { … }
```

Inline namespaces treat their members as members of their enclosing namespaces, in a transitive manner.

``` cpp
inline namespace { … }

// std::literals and its member namesapces are inline
// makes visible:
// - std::literals::string_literals::operator""s
// - std::literals::chrono_literals::operator""s
using namespace std::literals;
```

Anonymous namespaces (aka unnamed namespaces) are treated as a namespace with a unique name followed by a `using` directive so that its members are accessible from the enclosing namespace. Any name declared within has internal linkage.

``` cpp
namespace A {
  namespace { int i; }  // A::(unique)::i
  // implicit `using namespace (unique);`

  i++;  // OK: A::(unique)::i++
}

A::i++;  // OK: A::(unique)::i++
```

A `using`-declaration brings a namespace member into the current scope. Extensions of the namespace made after the point of declaration aren't visible.

``` cpp
using std::string;
```

A `using`-declaration can be used in a class definition to introduce a base class member (either a variable or function) in the derived class, with possibly a _different_ accessibility specifier. In effect this allows for more fine-grained tuning of the inherited members' accessibility. Note that this _does not_ apply to [inherited constructors](#inherited-constructors).

When a member function is introduced, all functions with that name are introduced, not just a particular overload. The derived class functions of the same prototype shadow or override the base class functions.

``` cpp
struct B {
  virtual void f() {}
  void g(char) {}

protected:
  int m;
};

struct D : B {
  using B::m;  // D::m is now public
  using B::f;

  void f() {}  // D::f() overrides B::f()

  using B::g;

  void g(int) {}  // D::g(int) hides B::g(char)
}
```

A `using`-declaration makes visible _all members_ of a namespace as if they were declared in the nearest enclosing scope which contains _both_ the `using`-directive _and_ the namespace, i.e. the lowest common ancestor namespace of the `using`-directive and the namespace specified in the directive.

Its effect is transitive, so that nominating a namespace that itself contains `using`-directives acts as if those directives were done within the enclosing namespace.

Extensions of the namespace made after the point of nomination are visible, unlike with `using`-declarations.

A namespace alias can be used to define alternate names for a namespace.

``` cpp
namespace A {
  namespace B {
    namespace C {
      int i = 0;
    }
  }
}

namespace ABC = A::B::C;

assert(ABC::i == 0);
```

# Raw Strings

There are raw string literals. They can contain an optional prefix to disambiguate from the contents.

``` cpp
const char *s = R"(the raw string)";

const char *s = R"delim(
escape codes \n parenthesis)
)delim";
```

# Unicode Strings

String literal prefixes can be used to specify the string's encoding and underlying character type.

| Prefix | Type               |
|:-------|:-------------------|
| `u8`   | `const char[]`     |
| `u`    | `const char16_t[]` |
| `U`    | `const char32_t[]` |
| `L`    | `const wchar_t[]`  |

# Static Assertions

`static_assert` is a compile-time assertion. It takes a `constexpr` that is contextually convertible to `bool`. An optional string can be displayed if the assertion fails.

# Attributes

Attribute specifiers are a standardization of syntax used for implementation-defined language extensions.

``` cpp
[[probably(true)]] if (blah)

[[omp::parallel()]]
for (int i = 0; …; i++) { … }
```

The `[[noreturn]]` attribute specifies that a function does not return [^diverging_function], as such it only applies to functions.

[^diverging_function]: This seems similar to a diverging function in Rust which returns `-> !`.

``` cpp
[[ noreturn ]] void f() {
  throw "error";
}
```

The `[[deprecated]]` attribute can mark a name or entity as deprecated, with an optional reason via `[[deprecated("reason"]]`.

Invalid escape sequences in string literals can be avoided by separating the string literals:

``` cpp
const char *p = "\xfff";    // error
const char *p = "\xff" "f"; // ok
```

The `alignas` specifier can be used to specify the number of bytes between successive addresses at which objects of a given type can be allocated. It can be applied to a type, variable, or member declaration.

``` cpp
struct alignas(16) sse_t {
  float sse_data[4];
};

alignas(128) char cache_line[128];
```

Class bit fields can be used to specify that a member has a specific size. Unnamed bit fields correspond to unused padding, and more specifically an unnamed bit field of size zero is used to finish the padding so that the next bit field begins at the beginning of its allocation unit.

``` cpp
// total 2 bytes
struct S {
  // byte 1
  uint8_t b1 : 3;
  uint8_t    : 0; // pad remaining 5 bits

  // byte 2
  uint8_t b2 : 6; // new byte
  uint8_t b3 : 2;
};
```

# Source Translation

The source translation phases of C++ are:

1. source bytes mapped to basic source character set or universal character name \u or \U escaped if not possible
2. lines ending in backslashes are joined to their next line
3. source decomposed into comments, whitespace, and preprocessing tokens: header names, identifiers, numbers, character and string literals, operators and punctuators.
each comment is replaced by one space character
4. preprocessor executed, recursively applying steps 1-4 for each `#include`
all preprocessor directives removed by the end of this step
5. convert character and string literals from source character set to execution character set, e.g. UTF-8
6. adjacent string literals are concatenated
7. compilation: translation of tokens into translation unit
8. examine each translation unit to determine required template instantiations, producing instantiation units
9. translation units, instantiation units, and library components needed to satisfy external references are collected into a program image

The relationship between header and source files is that a header should only contain the interface (e.g. class definition). This enables other translation units (source files) to "lightly" include just the header file, knowing that the definition of specific methods will be provided at link-time. On the other hand, the source file includes its associated header at the top and defines its implementation.

# volatile

The `volatile` keyword can be used to specify that the memory backing a variable may be modified "externally," such as due to memory-mapped I/O, and it signals to the compiler that it should not perform any optimizations on the variable that may for example elide certain operations.

``` cpp
volatile int temp_sensor_reading;

// don't elide redundant assignments
int temp = temp_sensor_reading;
process(temp);

temp = temp_sensor_reading;
process(temp);
```

# Performance

As a rule of thumb, different operations can be ranked in terms of their speed [^optimization_tips]:

[^optimization_tips]: [Three Optimization Tips for C++](https://www.facebook.com/notes/facebook-engineering/three-optimization-tips-for-c/10151361643253920)

1. comparisons
2. integer add, subtract, bit operations, shift
3. floating point add, subtract
4. indexed array access
5. integer multiplication
6. floating point multiplication
7. floating point division, remainder
8. integer division, remainder

# Duff's Device

Duff's Device can be used as a method of loop unrolling. It aims to decrease the number of branches. Instead of performing a check on each iteration, it breaks the iteration into chunks. The key is that it starts by jumping to the middle of the loop to process the non-divisible remainder, then continues the loop for each chunk.

For example, to copy 8-byte chunks to destination:

1. compute number of 8-byte chunks in source memory
2. create a do-while loop that copies 8-byte chunks per iteration
3. wrap the loop in a switch statement that switches on the non-divisible remainder
4. label each individual byte copy statement such that jumping to that label copies that many bytes via fall-through
5. continue looping for the remaining 8-byte chunks

``` cpp
uint8_t chunks = (count + 7) / 8;
uint8_t remainder = count % 8;

switch (remainder) {
case 0: do { *to = *from++;
case 7:      *to = *from++;
case 6:      *to = *from++;
case 5:      *to = *from++;
case 4:      *to = *from++;
case 3:      *to = *from++;
case 2:      *to = *from++;
case 1:      *to = *from++;
        } while (--chunks > 0);
}
```

A `switch`-based coroutine creates `static` local variables to hold the coroutine state. A switch statement is used as a jump table to go to the correct behavior for the current state. The disadvantage is that the use of `static` variables means that the function is not re-entrant or thread-safe. A workaround would be to store the state in an extra argument.

``` cpp
int yield_numbers() {
  static int i, state = 0;

  switch (state) {
   case 0: goto LABEL0;
   case 1: goto LABEL1;
  }

  LABEL0: /* start of function */
  for (i = 0; i < 10; i++) {
   state = 1; /* come back to LABEL1 */
   return i;
   LABEL1:; /* resume control after the return */
  }
}
```

# User-Defined Literals

User-defined literals can easily be created:

``` cpp
MyType operator"" _mytype(int literal) {
  return MyType(literal);
}

MyType m = 7_mytype;

Thing operator"" _thing(const char *str) {
 return Thing(str);
}

Thing thing = "test"_thing;

Temp operator"" _deg(long double deg) {
 return Temp(deg);
}

Temp temp = 3.14_deg;
```

# Unions

Unions can have member functions.

By default, special member functions are deleted by default, though they can be defined explicitly.

A maximum of one member may have a default member initializer.

A union's default member access is public.

A union's size is as big as necessary to hold the largest member.

If a union member has a user-defined constructor and destructor, switching to another member requires its explicit destruction and placement new of the new member.

``` cpp
union S {
  std::string str;
  std::vector<int> vec;
  ~S() {}
};

S s = {"test"};
s.str.~basic_string<char>();   // explicit destruction
new (&s.vec) std::vector<int>; // explicit placement new
s.vec.push_back(10);
s.vec.~vector<int>();
```

Anonymous unions can't have:

* member functions
* static data members
* non-public members

The members of anonymous unions are injected into the enclosing scope, and only the maximum of their sizes is allocated for them all.

A union can't:

* have base classes
* have reference members
* have virtual functions
* be used as a base class

A union-like class is any class with at least one anonymous union as a member. All of the anonymous union members are called its variant members.

# Linkage

A name with _internal linkage_ can only be referred to from all scopes in the _current_ translation unit. Items with internal linkage include:

* `static` variables, functions, function templates
* `const` and `constexpr` variables that aren't declared `extern`
* members of an anonymous union
* names declared in unnamed namespaces

A name with _no linkage_ can only be referred to from the scope it is in. Items with no linkage include:

* variables that aren't declared `extern`
* local classes and their member functions
* other names declared at block scope

A name with _external linkage_ can be referred to from scopes in _other_ translation units. Items with external linkage include:

* `namespace`-scope non-`const` variables not declared `static`
* functions not declared `static`
* any variables declared `extern`
* enumerations and enumerators
* names of classes, their member functions, `static` members, nested classes and enumerations, functions first introduced with friend declarations inside class bodies
* names of function templates not declared `static`

# Elaborated Type Specifier

The _elaborated type specifier_ syntax is used to disambiguate a type name from a non-type declaration. It works with `class`, `struct`, `union`, and `enum`. If the type isn't found, it's created as a _forward declaration_.

``` cpp
class T {};
int T;

// disambiguate the class/type from the integer
class T t;
```
# Standard Template Library

## Initializer Lists

The `std::initializer_list` type is a lightweight proxy object wrapping an array of objects of type `const T`. For this reason, it's normal and expected to pass it around by-value, since it's already essentially a pointer to the underlying array.

## integral_constant

The `std::integral_constant<T, T v>` type from `type_traits` takes an integral type and a constant value for it. Two typedefs for these exist which are `true_type` (i.e. `std::integral_constant<bool, true>`) and `false_type`. That can be used to refine the selection of a function overload.

``` cpp
// integrals
template <typename T>
void foo_impl(T val, true_type);

// floats
template <typename T>
void foo_impl(T val, false_type);

// Use is_integral to select the appropriate overload.
template <typename T>
void foo(T val) {
 foo_impl(val, std::is_integral<T>());
}
```

## Array

The `std::array<N, T>` type is a wrapper around regular `T[N]` arrays. It provides typical STL collection functionality such as iterators and copy and assignment operators. It has no user-provided constructors, no base classes, no virtual member functions, no default member initializers, and only contains a regular public array. This means that `std::array` is an aggregate type, which allows it to be initialized like a regular array, via aggregate-initialization:

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

Note that mismatching the type of the pairs can lead to inefficiency. For example, if the `const`-qualifier of the key type is missing, it can lead to a temporary being created and then converted to the appropriate type:

``` cpp
std::unordered_map<std::string, int> m;

for (const std::pair<std::string, int>& pair : m) { … }
```

This can result in the following on each iteration:

1. create temporary of type of `pair`
2. convert `m` pair to the type of the temporary (i.e. sans key `const`)
3. bind `pair` to that temporary
4. destroy the temporary

This is contrary to what the user may expect when iterating by reference.

To avoid possibly introducing these type mismatches and consequent performance penalties, it's safer and easier to just use `auto` type deduction:

``` cpp
for (const auto& pair : m) { … }
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
  template <typename... Ts>
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

* algorithms whose names end in `_backward` perform their operation from right-to-left. For example,  `copy_backward` copies the last source element to the last destination iterator position, then the penultimate, etc. _Don't_ confuse this with the operation being performed in reverse. The elements are still in the same order; they were simply copied from right-to-left.

* algorithms whose names contain `_copy` perform their operation to elements as they are being copied into another range. Conversely, their counterparts which _don't_ have `_copy` in their name operate in-place, modifying the elements of the range.

* algorithms that take a comparison function expect a function which returns `true` if the first parameter is _less_ than the second, and `false` otherwise. Comparison functions must not modify the parameters.

    Equality can be checked with such a comparison function by ensuring that it doesn't yield true for either one with respect to the other, i.e. a and b are equal if `!cmp(a, b) && !cmp(b, a)`, e.g. if `!(a < b) && !(b < a)`.

* algorithms that concern "order" or "equality" accept an optional comparison function

* algorithms usually return one-past the last element that was operated on.

The `exchange` function replaces the value of an object with that of another and returns the old value.

The `swap` function swaps two parameters with each other. It's also overloaded for arrays. It's important to note that it should usually be called in an unqualified manner. That is, use a using-declaration to bring the `std::swap` definitions into scope, but call it as `swap` and not `std::swap`. This enables additional, perhaps more specialized overloads of `swap` to be used when appropriate.

``` cpp
void Function() {
  using std::swap;

  int a = 1, b = 2;

  swap(a, b);
}
```

### Testing

The `all_of`, `any_of`, and `none_of` functions specify whether the elements of a given iterator range satisfy the given predicate for all, any, or none of the elements respectively.

``` cpp
vector<int> vec{0, 2, 4, 6};

bool all_even = std::all_of(vec.begin(), vec.end(), [](auto i) {
                  return i % 2 == 0;
                });

EXPECT_TRUE(all_even);
```

The `equal` function checks if two iterator ranges are equivalent by equality or predicate.

``` cpp
std::string s = "radar";

// Compare forward range to backward range
bool is_palindrome = std::equal(s.begin(), (s.begin() + s.size() / 2),
                                s.rbegin());

EXPECT_TRUE(is_palindrome);
```

The `lexicographical_compare` function checks to see if the first range is lexicographically less than the second range.

``` cpp
vector<int> a{1, 2, 2}, b{1, 2, 3};

std::lexicographical_compare(a.begin(), a.end(),
                             b.begin(), b.end());
// a < b = true
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

### Searching

The `mismatch` function takes two iterator ranges and finds and returns the first mismatching positions as determined by equality or a given predicate.

``` cpp
vector<int> a{1, 2, 3, 4, 5};
vector<int> b{1, 2, 3, 5, 6};

auto pos = std::mismatch(a.begin(), a.end(), b.begin(), b.end());

EXPECT_EQ(&a[3], &*pos.first);
EXPECT_EQ(a[3], *(pos.first));
EXPECT_EQ(b[3], *(pos.second));
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

The `nth_element` function selects the `$n^\text {th}$` element from the sorted order of the range, i.e. the `$n^\text {th}$`-order statistic. A comparison function can be specified.

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

### Reducing

The `count` function counts all elements in the range which equals a given value, whereas `count_if` counts those which satisfy a given predicate.

``` cpp
vector<int> vec{1, 2, 3};

int twos = count(vec.begin(), vec.end(), 2);
int odds = count_if(vec.begin(), vec.end(),
                    [](int i) { return i % 2 == 0; });

EXPECT_EQ(1, twos);
EXPECT_EQ(2, odds);
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

<div>$$ \text {dest}[i] = \sum_0^i \text {src}[i] $$</div>

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

The `transform` function applies a given unary function to each element in the array, or a given binary function to each pair of elements in two ranges, and writes each result to the output iterator. The given function must not modify the elements or invalidate iterators.

``` cpp
string s("hello");

std::transform(s.begin(), s.end(), s.begin(), ::toupper);

// s = "HELLO"
```

### Mutating

The `for_each` function applies a function to each element in the range, potentially mutating the element. It returns the provided function object, allowing for the accumulation of a result.

``` cpp
vector<int> vec{1, 2, 3}, expect{2, 3, 4};

std::for_each(vec.begin(), vec.end(), [](int &n) { ++n; });

EXPECT_EQ(expect, vec);
```

The `iota` function can be used to fill a range with sequentially incremented values, starting with the given value.

``` cpp
vector<int> v(3);
std::iota(v.begin(), v.end(), 1);

EXPECT_EQ({1, 2, 3}, v);
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

The `replace` function replaces the elements in the range that match the given value or satisfy the predicate (with `replace_if`) with another value. There's also `replace_copy`.

``` cpp
vector<int> v{1, 1, 2, 2, 3};

// Replace even numbers with 0.
std::replace_if(v.begin(), v.end(),
                [](int i) { return i % 2 == 0; },
                0);

EXPECT_EQ({1, 1, 0, 0, 3}, v);
```

The `reverse` function reverses the order of the elements in the range. There's also `reverse_copy`.

``` cpp
vector<int> v{1, 2, 3};

std::reverse(v.begin(), v.end());

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

The `remove` function rearranges the elements of the range so that those equal to a given value or satisfying a given predicate are moved to the end of the range, allowing them to easily be erased from their container. There's also `remove_copy`.

``` cpp
vector<int> v{1, 2, 1, 3, 1, 4};

auto new_end = std::remove(v.begin(), v.end(), 1);

v.erase(new_end, v.end());
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

### Sorting

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

### Partitioning

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

### Transferring

The `copy` function copies elements from the given range into the range beginning at a given iterator position. The `copy_if` does the same only if the element satisfies a given predicate. There's also `copy_n` and `copy_backward`.

``` cpp
vector<int> v{1, 2, 3};
vector<int> odds, odds_expect{1, 3};

std::copy_if(v.begin(), v.end(), std::back_inserter(odds),
             [](auto i) { return i % 2 != 0; });

EXPECT_EQ(odds_expect, odds);
```

Note that with `copy_backward` the elements are _not_ copied in reverse, that is, the order of the elements is preserved. Instead, this function copies starting from the right end, which is why the end iterator is provided.

``` cpp
vector<int> source{1, 2, 3};
vector<int> destination(4);

std::copy_backward(source.begin(), source.end(), destination.end());

EXPECT_EQ({0, 1, 2, 3}, destination);
```

The `move` function moves elements from the range into the range beginning with the third parameter. There's also `move_backward`.

``` cpp
vector<thread> ths;
ths.emplace_back(func, arg);
ths.emplace_back(func, arg);

vector<thread> dest(2);

// Could just dest = move(ths) in this case, but w/e
std::move(ths.begin(), ths.end(), dest.begin());
```

### Sets

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

### Heaps

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

### Permutations

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
