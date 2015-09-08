---
title = "Java"
published = "July 6, 2014"
excerpt = "A recap of a traditional language"
comments = false
---

Java is considered by many to be a very boring language, relegated to insurance applications---or the enterprise in general. Compared to many other languages, it's dry and monotone. It has a reputation of being over-engineered, being home to the classic `FactoryFactory` joke [^over_engineered]. In the past, programs written in Java were generally regarded as being slow and bloated resource hogs.

[^over_engineered]: For example, [`AbstractSingletonProxyFactoryBean`](http://docs.spring.io/spring/docs/2.5.x/api/org/springframework/aop/framework/AbstractSingletonProxyFactoryBean.html) from the Spring framework.

When Java was introduced in the 90s, it was hailed as being very portable, giving rise to the famous slogan "Write Once, Run Anywhere." Java runtimes became available on various operating systems, cell phones, and even gained traction on websites as Java applets.

Java applets quickly became a very popular attack vector for malicious entities, leading public perception of Java as being insecure, ultimately leading Mozilla to adopt a stance of having Java disabled by default in Firefox---though this was eventually reverted due to public outcry.

Despite this negative perception of Java by many, I've always held a certain respect for it. Throughout all this, real Java applications have and continue to be created, seemingly impervious to this image. Where many are creating virtual machines from scratch, Java has a very robust JVM which is the added experience of the many years that Java has been in existence.

*[JVM]: Java Virtual Machine

These notes are a recap of Java and also cover Java 8.

<toc/>

# JVM

Java is an interpreted language. Java code is compiled to highly optimized bytecode which is run by the JVM. HotSpot provides a Just-in-Time (JIT) compiler for Java bytecode. However, this doesn't mean that the entire Java program is compiled into executable code, which would be too complicated since it requires many run-time checks that can only be performed at run-time. Instead, the JIT compiler compiles parts of the program as it sees fit.

*[JIT]: Just-in-Time

JDK 8 adds the concept of compact profiles which contain a subset of the Java API. Profiles go from 1 to 3, where `compact1` is the smallest profile. When compiling a program, the profile can be specified to determine if the program conforms to the subset specified by the profile.

``` bash
# ensure Program only uses compact1 subset
$ javac -profile compact1 Program
```

# Primitives

Java doesn't support unsigned integers. The `double` floating-point type is more commonly used than `float` because all of the standard library's math functions use doubles. The `char` can hold unsigned 16-bit values and uses UTF-16.

Underscores can be written within integer or floating-point literals to make them more readable.

Automatic type conversions only take place if the two types are compatible and the destination is larger than the source type, a widening conversion. Manual conversions can be performed using casts, the same as C-style casts.

When different types are present in the same expression, Java enforces type promotion rules. The `char`, `byte`, and `short` values are promoted to `int`. If `long`, `float`, or `double` values are present in the expression, then the entire expression becomes of that type.

Integer types are always signed in Java. Bitwise right-shift operations therefore shift the sign bit into the high-order bit. This is not always preferable, and so the unsigned right shift operator `>>>` exists to shift a zero in the high-order bit regardless of sign.

``` java
// 11111111 11111111 11111111 11111111
int a = -1;

// 11111111 11111111 11111111 11111111
a >> 24;

// 00000000 00000000 00000000 11111111
a >>> 24;
```

The `strictfp` modifier can be applied to a class, method, or interface to ensure that floating-point calculations perform truncations of certain intermediate values during a computation, as in previous JVM versions.

Array type syntax can place the `[]` in one of two locations. The latter is better for methods returning arrays and for declaring multiple arrays in one line.

``` java
int a[] = new int[3];
int[] a = new int[3];

// three arrays
int[] nums, nums2, nums3;
```

## Primitive Wrappers

Type wrappers are classes that wrap primitive types, such as `Character` which wraps `char`. All numeric type wrappers such as `Integer` and `Float` inherit from abstract class `Number` [^NSNumber] which provides conversion methods for all numeric types.

[^NSNumber]: Reminds me of Objective-C's `NSNumber`.

Encapsulating a primitive in an object is referred to as _boxing_, and the reverse is called _unboxing_. _Autoboxing_ and _auto-unboxing_ refers to the automatic wrapping and unwrapping of primitive values. JDK 5 added support for autoboxing and auto-unboxing, which works whenever a primitive type must be converted to an object, such as when passed as parameters to methods or when used in expressions.

``` java
Integer i = 100; // autoboxed

Integer ib = 1;
++ib; // auto-unboxed, incremented, re-boxed
```

There is no way to refer to the same instance of a primitive value. Primitive values can be wrapped into objects using the primitive wrappers---for use in collection classes which only work with objects, for example.

`Number` is an abstract class that is derived by specific numeric type wrappers like `Integer` and `Double`. It provides methods for retrieving a given value in any other type format, e.g. `doubleValue`.

Wrappers can be constructed given the actual primitive value or a string representation of it.

Specific wrapper types also include static methods for parsing strings into primitive types, such as `Float.parseFloat("3.14")`.

Each of these wrapper types include certain constants such as `MIN_VALUE` and `MAX_VALUE`.

The `Double` and `Float` methods `isInfinite` and `isNaN` can be used to test if the values are either of those special values.

The `Char` methods `forDigit` and `digit` can convert a number to a character and vice versa, respectively.

## Supplemental Characters

Java `char`s can only hold 16 bits, which means that a single `char` is unable to represent supplemental characters, those characters which are larger than `0xFFFF` and thus would require 32 bits to represent. Java resolves this issue by using two `char`s to represent a supplemental character: a _high surrogate_ and a _low surrogate_.

Various `Character` methods provide overloads that accept an `int`, which is 32 bits and therefore large enough to hold even a supplemental character.

The method `codePointAt` returns an `int` containing a particular code point of the provided character sequence location. The method `toCodePoint` is similar except that it returns the code point of the provided surrogate pair provided a high and low surrogate character argument.

The `toChars` method performs the reverse operation, taking a code point and returning an array of characters, which may be two elements in length if it's a supplemental character.
# Control Structures

Switch statements in Java can operate on expressions of type `byte`, `short`, `int`, `char`, enumerations, or `String`. Case statements don't break automatically, and so the `break` keyword must be used.

Labeled break statements can specify exactly which block to break to, causing execution to jump to the _end_ of the specified _enclosing_ block. Blocks can be given names by prefixing the `{` character with a label in the form of `thelabel:`.

The `continue` keyword also supports this functionality, in effect specifying which outter-loop to `continue`.

``` java
redundant: for (int i = 0; i < 4; i++) {
  break redundant;
}

outter: {
  for (int i = 1; i < 4; i++) {
    for (int j = 1; j < 4; j++) {
      for (int k = 1; k < 4; k++) {
        // break out of _all_ loops
        if (somecondition)
          break outter;
      }
    }
  }
} // execution jumps here
```

Assertions aren't exactly control structures but they're in this section anyway. The `assert` keyword takes a condition which is optionally followed by a colon and an expression that is converted to a string and displayed if the assertion fails.

``` java
assert divisor != 0: "attempted to divide by zero";
```

Assertions aren't run by default due to the performance impact, but can be enabled by supplying the `-ea` option to the `java` interpreter, or disabled with the `-da` option. Assertions can be enabled or disabled at the package level by specifying the package and following it by three periods.

``` bash
# enable all assertions
$ java -ea Program

# only enable assertions from Core package
$ java -ea:Core... Program
```

# Classes

Something to remember is that variables of class type hold references to `new` instantiated objects of that type, not the objects themselves.

Classes can have finalizer methods which can be used to free resources and are run just prior to being garbage collected.

Static methods can only directly call other static methods and access static data. Static variables that require computation for initialization can use static blocks which are executed exactly once.

``` java
static int a = 3;
static int b;

static {
  System.out.println("static block initialized");
  b = a * 4;
}
```

Fields can be `final`, which makes them immutable. Final fields can be initialized via a value given at declaration or within a constructor.

Fields can be `transient`, which means that they should not be persisted when the object is stored.

Variable-length arguments are specified by threep periods and makes the arguments available as an array.

``` java
void printArgs(int ... v) {
  for (int a : v) {
    System.out.println(a);
  }
}

printArgs(1, 2, 3);
```

The `instanceof` operator can test to see if an instance is of a given type or _can be_ cast into a given type, yielding a boolean value.

the `native` keyword can be used to mark a method as native via the Java Native Interface (JNI), in which case a method shoudldn't be provided. The library that contains the definition of the function should be linked using the `System.loadLibrary` method, particularly within a `static` block to ensure that this only occurs once.

*[JNI]: Java Native Interface

``` java
class Test {
  public native void test();

  static {
    System.loadLibrary("NativeDemo");
  }
}
```

The code should be compiled normally, then the `javah` program should be run on the result to produce a header file that must be included in the implementation of the native method.

``` bash
# produces Test.h
$ javah -jni Test
```

This generated header specifies the expected prototype of the native method which should be used for its implementation.

``` c
#include <jni.h>
#include "Test.h"
#include <stdio.h>

JNIEXPORT void JNICALL Java_Test_test(JNIEnv *env, jobject obj) {
  printf("successfully called\n");
}
```

# Inheritance

Inheritance is expressed with the `extends` keyword. Private members in the superclass can't be accessed by the child class. The superclass can be accessed via the `super` keyword. A superclass initializer can be called with `super` as well, which must be the first statement in a subclass constructor.

``` java
class A {
  int a;
}

class B extends A {
  int b;
}
```

The `abstract` keyword can be used to denote that subclasses must override a method, and this property bubbles up to the class, so that a class with an abstract method must itself be declared abstract. Even if a class is abstract, it may contain concrete method implementations. Abstract classes cannot be instantiated, though they may be used to create references in order to leverage run-time polymorphism.

``` java
abstract class A {
  abstract void callme();
}

class B extends A {
  void callme() {
    System.out.println("called");
  }
}
```

The `final` keyword can be used to prevent a method from being overridden in subclasses. Such methods can be inlined by the compiler because it knows that they will not be overridden and thus doesn't need to resolve the call dynamically at run-time.

Further, the `final` keyword can be used to prevent inheriting from a particular class at all.

All classes are subclasses of `Object`, so that a reference of type `Object` can refer to any other class.

Overloaded constructors can call other constructors by using the `this` keyword as a method, but if this is done then it must be the first statement within the constructor. Calling overloaded constructors in this manner imposes a performance impact due to the calla nd return mechanism used when the second constructor is invoked, so this mechanism shouldn't be used simply for the sake of cleaner code.

There are two restrictions with calling other constructors. The first is that an instance variable of the constructor's class can't be used in a call to another constructor (i.e. passing it as an argument). The second is that superclass constructor delegation and same-class constructor delegation can't be used in the same constructor, since each has the requirement of being the first statement in the constructor.

# Generics

Java doesn't actually create different versions of parameterized classes or methods, unlike C++ template instantiations. Instead it performs _type erasure_ so all generic type information is substituted by necessary type casts.

With type erasure, all generic type information is erased at compile-time, replacing type parameters with their bound type---`Object` if no explicit bound is specified. Appropriate casts are then inserted to maintain compatibility with the types specified by the type arguments, a compatibility which the compiler also enforces.

It's possible for methods overridden in subclasses to mismatch the type erasure of the superclass method definition. In this case, the compiler inserts a bridge method that has the same type erasure as the superclass which then calls the method that has the type erasure specified by the override.

In the following example, `getOb` in `T1<String>` results in a return type of `Object` due to type erasure, so the override isn't actually an override since the return types don't match. For this reason, the compiler would insert a method of the same name with the same return type as `T1` which itself would call the `T2` "override".

In this example the only difference between the two methods with the same name is the return type, which is not a valid overload and would normally yield a compiler error, but it's handled automatically and correctly by the JVM.

``` java
class T1<T> {
  T ob;

  T getOb() {
    return ob;
  }
}

class T2 extends T1<String> {
  String getOb() {
    System.out.println("called String override");
    return ob;
  }
}
```

It's also possible for type erasure to lead to ambiguity errors, where two distinct declarations resolve to the same erased type. This can be fixed by placing strict type bounds or making the method names distinct.

``` java
class T<X, Y> {
  X obj1;
  Y obj2;

  // both resolve to void set(Object o)
  void set(X o) { ... }
  void set(Y o) { ... }
}
```

Ambiguity errors can also be caused when there are more than one type parameters with no restriction on them being distinct, incompatible types.

Generics only work with reference types, so that a primitive type such as `int` can't be a type argument to a type parameter, instead necessitating a boxed type such as `Integer`.

It's not possible to create an instance of a type parameter, since the compiler won't know what type to actually create. By extension, it's not possible to instantiate arrays whose element type is a type parameter or a specific generic type as well. However, it is possible to create arrays of references to a generic type via a wildcard. It's also not possible to create generic exception classes.

``` java
class T<A> {
  A obj;

  T() {
    obj = new A(); // error
    T vals[] = new T[10]; // error
    T<Integer> vals[] = new T<Integer>[10]; // error
    T<?> vals[] = new T<?>[10]; // fine
  }
}
```

It's also not possible to define static members that use type parameters declared by the enclosing class. There can, however, be generic static methods that define their own type parameters.

Generic classes are defined in the following form.

``` java
class Name<TypeParameter> {}
```

Bounded types allow the specification of an upper or lower bound on the expected type. An upper bound specifies the superclass from which the type argument must derive and is accomplished using the `extends` keyword. It's possible to use an interface as a bound, in which case the passed type must implement the given interface. A combination of type and interface(s) may be provided separated by ampersands `&`, but the type must come first.

``` java
// upper bound: type argument must be or extend SuperClass
<T extends SuperClass>

// type argument must be or extend SuperClass
//   and extend Interface1 and Interface2
<T extends SuperClass & Interface1 & Interface2>
```

It's possible to specify a wildcard type parameter with the question mark `?`, which represents an unknown type. This would match any generic type regardless of its type argument, so that `Test<A>` and `Test<B>` would match `Test<?>`.

``` java
// this can take in a Stats<Integer> and Stats<Double>
boolean sameAvg(Stats<?> obj) {
  return average() == obj.average();
}
```

Wildcards can also be bounded with an upper or lower bound with the `extends` and `super` keywords respectively. In both cases, the provided bound type is eligible for satisfying the bound.

``` java
// upper bound
<? extends SuperClass>

// lower bound: type must be superclass of SubClass
//   or SubClass itself
<? super SubClass>
```

Generic methods can be defined within non-generic classes. In this case, the type parameter list precedes the return type. If type inference fails in inferring the types, they may be explicitly provided before the method name.

``` java
class Generic {
  static <T, V> boolean method(T a, V b) { ... }
}

Generic.<Integer, Double>method(2, 3.0);
```

Constructors can also be generic even if their classes aren't, in which case the type parameter list also precedes the constructor name.

``` java
class DoubleContainer {
  private double val;

  <T extends Number> DoubleContainer(T arg) {
    val = arg.doubleValue();
  }
}
```

Interfaces can also be made generic, in which case their declaration syntax is identical to that of a generic class. In most cases, if a class implements a generic interface then the class itself must be generic in order to pass the type parameter to the interface's type parameter list.

In order to preserve backwards compatibility, Java allows a generic class to be used without any type arguments, in which case it's referred to as a _raw type_. Type casts, which would normally be substituted automatically during type erasure, must be explicitly included to type check. However, if the type cast fails at run-time, it yields a run-time error.

Due to the danger imposed by raw types, the Java compiler displays unchecked warnings when raw types are used in ways that may break type safety.

``` java
class Type<T> {
  T ob;
}

Type raw = new Gen(new Double(3.0));
double d = (Double)raw.ob;
int i = (Integer)raw.ob; // run-time error
```

Generic classes may inherit from generic and non-generic classes. It's also possible to inherit from a specific generic type (e.g. `T<String>`).

The `instanceof` operator can be used on generic classes, but since generic type information is not available at run-time, a wildcard must be used to check.

``` java
Type<Integer> t = new Type<Integer>(3);

assert t instanceof Type<?>;
```

It's possible to cast an instance of a generic class into another if the type arguments are the same and the classes are compatible (related).

the diamond operator `<>` can be used to instantiate a generic class and infer the type arguments from the types passed to the constructor.

``` java
Type<Integer, String> ob = new Type<>(3, "string");
```

# Packages

Packages serve as containers for classes and serve a similar purpose to namespaces in C++, in particular they help avoid name collisions.

Packages are created by specifying a `package` declaration at the beginning of a source file, which has the effect of putting all classes declared within that file to belong to the package.

``` java
package MyPackage;
package Some.Hierarchy.Here;
```

Multiple source files may contain the same `package` declaration, allowing packages to be spread across many source files. Packages map to directories on the file system.

The default access specification is that, if a class member doesn't have an explicit access specification, it is visible to subclasses and other classes in the same package. Specifying a member as `protected` makes it accessible outside of the package but only to subclasses of the class to which they are a member.

The following table specifies whether a class member with a particular access modifier is accessible by other package components.

|Class Member Accessible By     |Public    |Protected    |No Modifier    |Private|
|:-------------                 |:-------- |:----------- |:------------- |:---------|
|Same Class                     |Yes       |Yes          |Yes            |Yes|
|Same Package SubClass          |Yes       |Yes          |Yes            |No|
|Same Package Non-SubClass      |Yes       |Yes          |Yes            |No|
|Different Package SubClass     |Yes       |Yes          |No             |No|
|Different Package Non-SubClass |Yes       |No           |No             |No|

Packages can be imported using the `imported` keyword in order to avoid having to fully qualify package contents. The import statement may import either a classname or the `*` to import all classes.

``` java
import java.util.Date;
import java.io.*;
```

If the `import` keyword is followed by the `static` keyword then only static members are imported, avoiding the need to fully qualify them. A wildcard is also possible with static imports.

``` java
import static java.lang.Math.sqrt;
import static java.lang.Math.pow;

// or
import static java.lang.Math.*;
```

# Interfaces

Classes must implement the complete set of methods specified in an interface in order to fully implement that interface. Interfaces must be declared as either `public` or use the default access level, while nested interfaces may be declared as `public`, `private`, or `protected`.

``` java
interface Callback {
  void callback(int param);
}
```

Classes specify that they implement a particular interface by using the `implements` keyword followed by a list of interfaces that it implements. Methods that implement an interface _must_ be declared `public`.

``` java
class Client implements Callback {
  public void callback(int p) {
    System.out.println("callback called with " + p);
  }
}
```

As with subclasses, it's possible to create references of interface types that point to objects that implement the interface, such that method calls resolve to those implemented by the object.

``` java
Callback c = new Client();
c.callback(42);
// callback called with 42
```

If a class doesn't fully implement the methods required by the interface it claims to implement, then that class must be declared as `abstract`.

``` java
abstract class Incomplete implements Callback {
  int a, b;
}
```

Variables may also be declared within interface declarations, but they are implicitly `final` and `static` such that they cannot be changed by the implementing class.

Interfaces may inherit from each other, such that the derived interface requires all methods in its parent interfaces to be implemented as well as its own.

``` java
interface A {
  void meth();
}

interface B extends A {
  void meth2();
}

class SomeClass implements B {
  public void meth() { /* ... */ }
  public void meth2() { /* ... */ }
}
```

JDK 8 makes it possible to provide default implementations of methods. Such implementations are referred to as _default methods_ or _extension methods_. Default methods are specified by prefixing the method implementation with the `default` keyword.

*[JDK]: Java Development Kit

Class implementations take priority over interface default implementations. If a class implements two interface with the same default method, the method must be overridden to disambiguate the call.

If an interface inherits from another and both define a common default method, the sub-interface's version takes precedence. However, the sub-interface can refer to the super-interface's default implementation by using the `super` keyword, as in `Interface.super.method()`.

``` java
public interface SomeInterface {
  int getNumber();

  default String getString() {
    return "default";
  }
}
```

JDK 8 also added the ability to define static methods in interfaces which can only be called off of the interface name, since static interface methods aren't inherited by an implementing class or a subinterface.

``` java
interface SomeInterface {
  static int getDefaultNumber() {
    return 0;
  }
}

int defNum = SomeInterface.getDefaultNumber();
```

# Exceptions

The `try` block is used to enclose code that may potentially throw an exception. These can be nested so that an exception thrown within an inner one bubbles outwards until it is caught.

The `catch` statement essentially works like a pattern matching in functional languages, where the match succeeds if the actual exception type is a subclass of or _is_ the type specified within the parentheses. This is the manner in which the type of error is determined, in order to appropriately handle it.

``` java
try {}

catch (ExceptionType e) {}

finally {}
```

Exception types are subclasses of the built-in class `Throwable`. Under `Throwable` there are two subclasses: `Exception` which is for exceptional conditions that programs should catch, and `Error` which is for exceptions that aren't expected to be caught under normal circumstances. In particular, exceptions of type `Error` are used for errors pertaining to the Java run-time environment and are usually created in response to serious failures that usually can't be handled by the program.

The `throw` statement is used to throw instances of exception types, particularly of type `Throwable` or a subclass of it. Execution immediately stops after the `throw` statement and jumps to wherever the exception is caught, bubbling out of enclosing `try` blocks until a handler is found or the run-time catches it.

``` java
try {
  throw new NullPointerException("demo");
} catch (NullPointerException e) {
  System.out.println("caught " + e);
}
```

If a method is capable of throwing an exception that it doesn't handle, then it must be marked with the `throws` keyword to inform callers that they should put it within a `try` block. The `throws` keyword is placed after the parameter list and includes a list of exception types that may be thrown. Failing to do this prevents the program from compiling.

``` java
class Throws {
  static void throwOne() throws IllegalAccessException {
    throw new IllegalAccessException("demo");
  }
}

try {
  throwOne();
} catch (IllegalAccessException e) {
  // ...
}
```

The `finally` block is used to define code that must be run regardless of whether or not an exception was thrown, even in the event that an exception is thrown but not handled.

Chained exceptions allow associating one exception with another. This is facilitated by two constructors on `Throwable`, one which takes the other exception instance and another that takes a message as well as the instance. The `getCause` method can then yield the exception instance that was the cause of the current exception. The `initCause` method allows associating another exception with the current exception after it has been created.

Multi-catch allows two or more exceptions to be caught by the same `catch` clause. This is useful if two ore more exception handlers use the same exact code despite responding to different exceptions. To facilitate this, the exception types are separated by `|` and the exception parameter is implicitly `final`.

``` java
catch (ArithmeticException | ArrayIndexOutOfBoundsException e) {}
```

More precise rethrow refers to the restriction of the type of exceptions that can be rethrown to only those checked exceptions that the associated `try` block throws that aren't handled by a preceding `catch` clause and are a subtype or supertype of the parameter. For this restriction to be enforced, the `catch` parameter must be treated as or be explicitly declared as `final`.

When working with resources in pre-JDK 7 environments, it's necessary to leverage exception handling to make sure that resources don't leak if exceptions are thrown.

``` java
try {
  FileInputStream fin = new FileInputStream("test");
  // ...
} catch (Exception e) {
  // ...
} finally {
  // dispose of the resource if it was created
  try {
    if (fin != null) fin.close();
  }
  catch (IOException e) { /* error closing file */ }
}
```

Note that it's important to check that the stream isn't `null` before attempting to invoke the `close` method---in case the exception was thrown before the object was instantiated---in order to avoid a null pointer exception.

JDK 7 introduced _try-with-resources_ which allows initializing a resource within a `try` statement that should be automatically closed if the body ends, whether it threw or not. This can only be used on those resources that implement the `AutoCloseable` interface which defines a `close` method. This allows `catch` clauses to be used for more meaningful reasons.

Multiple resources can be defined within the same `try` statement by separating their declarations with semicolons.

Something to note is that the resource declared in the `try` statement is implicitly `final`, so that the resource can't be assigned to after it has been created.

``` java
try (FileInputStream fin = new FileInputStream("test")) {
  // work with fin
} catch (FileNotFoundException e) {
  // handle meaningful exception
}
```

Normally when an exception occurs after another exception leads to the `finally` block, the original exception is lost in favor of the new exception. With try-with-resources, the new exception is supressed and can be accessed using the `getSuppressed` method of the original exception.

# Multithreading

The `Runnable` interface represents a unit of executable code and consists of a `run` method. The `Runnable` object's `run` method can be executed in a separate thread by instantiating a `Thread` and passing it a reference to the `Runnable`. This can be accomplished by using a particular `Thread` constructor which takes the reference, then calling the `Thread`'s `start` method.

It's also possible extend the `Thread` class and override its `run` method to more directly specify code that should be run in a separate thread.

The `join` method from `Thread` can be used to join one thread to another, i.e. wait for another thread to finish.

A monitor is an object that's used as a mutually exclusive lock, and every object has its own implicit monitor that's automatically "entered," i.e. acquired, when a `synchronized` method is called, so that only one thread may enter a `synchronized` method at a time on a given object.

``` java
class Synchronized {
  synchronized void raceCall(int arg) {
    // ...
  }
}
```

There is also a `synchronized` statement which can be used to synchronize sections of code, which can be useful when one doesn't have control over the methods of a class. The statement takes a reference to the object to use as the monitor and contains code that should be synchronized for it.

``` java
synchronized(obj) {
  // ...
}
```

Java threads can communicate with each other via the `Object` methods `wait`, `notify`, and `notifyAll`. The `wait` method causes the calling thread to relinquish the monitor and sleep until another thread enters the monitor and calls `notify` or `notifyAll`. The `notify` method wakes up a thread that called `wait` on the same object, and the `notifyAll` method wakes up all threads that did so with one gaining access at random.

Despite calling `wait`, a thread may be woken up for no apparent reason (e.g. interrupted by a signal), which is why it's advised to put the `wait` call within a loop that checks the overall condition that is being waited upon.

``` java
while (condition) {
  try {
    wait();
  } catch (InterruptedException e) {
    // ...
  }
}
```

The functionality for suspending, resuming, and stopping threads must be implemented manually, usually in the form of a loop that checks a flag which represents the user's request. A suspend method can't be provided by the standard library because it could end up suspending a thread before it relinquishes its locks, leading to deadlocks. A stop method can't be provided either because it could leave data in an inconsistent state if it's stopped abruptly.

Normally in multithreaded programs when two or more threads share the same variable they store thread-local copies and update the "master copy" at certain points in execution, such as when `synchronized` methods are entered. Specifying the variable as `volatile` tells the compiler that it must always use the master copy of the varaible, or to always keep the local copies synchronized with the master copy.

The `ThreadGroup` class can be used to create a group of threads, which is useful when wanting to manage a group of threads as a single unit. Threads are added to the thread group by providing a reference to it as an argument in the `Thread` constructor. Operations can be performed on each of the threads in a group by enumerating them using the `enumerate` method on `ThreadGroup`.

The `ThreadLocal` class can be used to create thread-local data, and an `InheritableThreadLocal` can be used to allow them to be inherited.

# Enumerations

In Java, enumerations define class types that implicitly inherit from the `Enum` class, meaning that they may define constructors, methods, and instance variables. Despite this, they may not explicitly inherit or be inherited from. The `Enum` class is defined as:

``` java
class Enum<E extends Enum<E>>
```

Enumeration constants are implicitly `static` and `final`. Each enumeration constant is an object of its enumeration type, and each enumeration constant has its own copy of instance variables.

When defining a constructor, it may be called once for each enumeration constant that is specified by providing the parameters in parentheses after each constant.

``` java
enum Colors {
  Red(3), Green(2), Blue(1)

  private int number;

  Colors(int n) { number = n; }
  int getNumber() { return number; }
}
```

An enumeration constant's position, i.e. its _ordinal value_, can be retrieved by calling the `ordinal` method, and it can be compared against another enumeration constant's ordinal using the `compareTo` method. The `equals` method can be used to test if two enumeration constants are the same. Since enumeration constants are objects of their enumeration type, they can also be compared using the reference equality operator `==`.

# Annotations

Annotations provide metadata about code that can be used by development tools. Annotations are created through a special kind of interface that consists solely of method declarations for which Java provides implementations. All annotations implicitly extend the `Annotation` interface, so that `Annotation` is a super-interface of all annotations.

``` java
@interface Annot {
  String str();
  int val();
}

@Annot(str = "Example", val = 100)
public static void method() {}
```

Annotations could be used on declarations of any type, including classes, methods, fields, parameters, enumeration constants, and even other annotations. Annotations are applied by giving values to the annotation members.

Annotations members can be given default values by following the member line with the `default` keyword and the value to give it, such as:

``` java
int val() default 3;
```

Annotation retention policies refer to how long the annotation is retained. Regardless of the policy, annotations on local variable declarations are not retained in `.class` files.

|Policy    |Lifetime|
|:-------  |:---------|
|`SOURCE`  |source code|
|`CLASS`   |`.class` files|
|`RUNTIME` |`.class` files; available at runtime|

Annotation retention policies are specified using the `@Retention` annotation.

``` java
@Retention(Retention.Policy.RUNTIME)
@interface Annot {
  String str();
  int val();
}
```

Annotations with `RUNTIME` retention policies can be obtained at run-time via reflection. First, a `Class` object must be obtained that represents the class whose annotations we want to obtain, which is usually done with `getClass` or the `class` member. Next, it's necessary to obtain an object that represents the item for which we want to obtain annotations, e.g. `getMethod`. Once one of these objects is obtained, the actual annotation may be obtained with `getAnnotation` which can then be queried for the values of its members.

``` java
@Retention(RetentionPolicy.RUNTIME)
@interface MyAnno {
  String str();
  int val();
}

class Meta {
  @MyAnno(str = "test", val = 3)
  public static void myMeth(String str, int i) {
    Class<?> c = Meta.class;
    Method m = c.getMethod("myMeth", String.class, int.class);
    MyAnno anno = m.getAnnotation(MyAnno.class);

    System.out.println("str: " + anno.str() + ", val: " + anno.val());
  }
}
```

Alternatively, the `getAnnotations` method on a given item, such as `Method`, yields all annotations associated with the item with a `RUNTIME` retention. This method is defined by the `AnnotatedElement` interface, which defines many other annotation introspection methods.

Marker annotations don't have any members, so that their only purpose is to _mark_ the items to which they're applied, which can then be checked using the method `isAnnotationPresent`. Parentheses are optional with marker annotations.

Single-member annotations are those that only contain one member. These annotations can leverage a short-hand syntax if the member's name is `value`, in which case the value of the single member is the only thing within the parentheses.

It's also possible to use this short-hand if there are other members but they have default values.

``` java
@interface MySingle {
  int value();
}

@MySingle(100)
class Single {}
```

There are a variety of built-in annotations but some are used more than others.

The `@Target` annotation specifies the types of items to which the annotation may be applied by supplying possible targets as defined by the `ElementType` enumeration. If more than one target is specified, it must be specified in a comma-separated manner within braces, as in array initialization syntax.

``` java
@Target({ElementType.FIELD, ElementType.LOCAL_VARIABLE})
@interface Whatever {}
```

|Constant          |Applicable To|
|:---------        |:--------------|
|`ANNOTATION_TYPE` |another annotation|
|`CONSTRUCTOR`     |constructor|
|`FIELD`           |field|
|`LOCAL_VARIABLE`  |local variable|
|`METHOD`          |method|
|`PACKAGE`         |package|
|`PARAMETER`       |parameter|
|`TYPE`            |class, interface, enumeration|
|`TYPE_PARAMETER`  |type parameter (JDK 8)|
|`TYPE_USE`        |type use (JDK 8)|

The `@Inherited` annotation can only be applied to annotations being applied to class declarations, causing the annotation of a superclass to be inherited by a subclass. That is, if a subclass is searched for a given annotation and it's not found, its superclass is searched.

The `@Override` annotation can only be used on methods in order to declare that the method to which it's applied must be overriding a method from a superclass, yielding a compile-time error if this isn't the case.

The `@Deprecated` annotation is used to mark a declaration obsolete.

The `@FunctionalInterface` is a marker annotation added by JDK 8 that indicates that the annotated interface is a functional interface. However, any interface with exactly one abstract method is by definition a functional interface, so that this annotation is purely informational, aside from yielding compile-time errors if the constraint isn't satisfied.

Beginning with JDK 8, annotations can also be placed in most cases in which a type is used---such as return types, the type of `this`, a type cast, and so on---in which case they're referred to as type annotations. These annotations are mainly used for external tools to enforce stricter checks than the Java compiler may perform.

To annotate the type of `this`, known as the _receiver_, JDK 8 allows explicitly declaring `this` as the first parameter of a method in which case it should take on the type of the class the method belongs to.

``` java
int myMeth(@TypeAnno SomeClass this, int i, int j) {}
```

When annotating return types, it's not possible to annotate a return type of `void`.

JDK 8 added support for so called _repeating annotations_ which are annotations that can be repeated on the same element. The annotation that is intended to be repeatable must be annotated with the `@Repeatable` annotation which specifies the annotation's container type, that is, another annotation for which its `value` field is an array of the repeatable annotation type.

These repeated annotations can then be retrieved using `getAnnotation` to retrieve the container type.

Alternatively, it's more straightforward to use the `getAnnotationsByType` method.

``` java
@Retention(RetentionPolicy.RUNTIME)
@Repeatable(MyRepeatedAnnos.class)
@interface MyAnno {
  String str() default "test";
  int val() default 3;
}

@Retention(RetentionPolicy.RUNTIME)
@interface RepeatedAnnos {
  MyAnno[] value();
}

@MyAnno(str = "first", val = 1)
@MyAnno(str = "second", val = 2)
class Annotated {
  Annotation container = Annotated.class.getAnnotation(RepeatedAnnos.class);
  MyAnno[] annos = container.value();

  // or
  MyAnno[] annos = Annotated.class.getAnnotationsByType(MyAnno.class);

  for (Annotation a : annos)
    System.out.println(a);
}
```

# Lambdas

A functional interface is an interface that contains only one abstract method. This means that the interface can contain other methods so long as they have default implementations. The functional interface's method specifies the target type, and lambda expressions can only be specified in a context in which a target type is defined.

When a lambda expression does occur in a target type context, an instance of a class is automatically created that implements the functional interface. The parameters and return type of the lambda expression must match those of the abstract method's, and any exceptions thrown by the lambda must be acceptable to the method.

``` java
interface Test {
  double getValue();
}

class Demo {
  Test t = () -> 2.0;
  System.out.println("value: " + t.getValue());
}
```

If a lambda expression has only one parameter, it's not necesary to surround the parameters with parentheses. If it's necessary to explicitly declare the type of a parameter, all of them must be specified---all or nothing. If multiple statements are required within a lambda, they simply need to be surrounded with braces as with a method body and a return statement must be given.

Lambdas may only use local variables from their enclosing scope if they're effectively final, that is, their value doesn't change after they're first assigned. As a result, lambdas can't modify local variables from their enclosing scope. However, it _may_ use and modify instance varaibles from its invoking class.

Method references can refer to methods without executing them. Static method references can be obtained using the `::` separator introduced in JDK 8. A method reference can then be used anywhere in which it is compatible with the target type.

``` java
ClassName::staticMethod;
```

It's also possible to obtain references to instance methods of a specific object with the same syntax.

``` java
objRef::methodName;
```

It's also possible to obtain a reference to an instance method that can be used on any object. In this case, the first parameter of the functional interface should be of the type of the invoking object and the second should be the parameter(s) specified by the method.

``` java
interface Func {
  boolean func(ClassName a, int b);
}

ClassName::instanceMethod;
```

If the class is generic, then the type parameter is specified after the `::` separator.

``` java
ClassName::<Type, OtherType>instanceMethod;
```

It's also possible to reference constructors. If the class is generic, then the type parameters are provided as mentioned above. Constructor references for arrays can also be created. A functional interface for a constructor references to arrays should contain a method that takes an integer parameter to refer to an array constructor.

``` java
ClassName::new;

ClassName[]::new; // arrays
```

A superclass version of a method may be referred to with the `super` keyword.

``` java
super::methodName;
```

JDK 8 contains predefined functional interfaces in <span class="path">java.util.function</span>.

# Strings

String objects are automatically created from string literals, which means that string literals may be used as if they were String objects themselves. When working with regions, the end index is one-past the last affected index, as with C++ iterators.

Java automatically converts data to strings using the `String`'s static method `valueOf`, which is overloaded for all primitive types and `Object`. For other objects, `valueOf` calls the object's `toString` method.

The `equals` and `equalsIgnoreCase` methods can be used to determine if a string is equal to another. The `regionMatches` method can be used to determine if separate regions of two different strings match. The `startsWith` and `endsWith` methods can be used to determine if a string ends or begins with another string. The `Comparable` interface's `compareTo` and `compareToIgnoreCase` methods can be used to get a less, equal, or greater than result with respect to another string.

The `indexOf` and `lastIndexOf` methods can be used to obtain the index where the first occurrence of a character or string begins. There are overloads which take a starting point as well, which can simplify getting all the positions of all of the occurrences.

Strings are immutable, so operations that appear to modify them simply return new copies of the resulting strings. The `substring` method can be used to extract a copy of a region of a string given a starting index and optionally en ending index. The `replace` method can replace all occurrences of a character with another. An overload exists which replaces character sequences. The `replaceAll` method can replace any substring that matches the given regex with the specified string.

JDK 8 adds a static `join` method that can join a number of strings with a given string. Conversely, the `split` method can split a string based on a regex string.

The `toLowerCase` and `toUpperCase` methods can be used to convert an entire string to upper or lower case characters.

## StringBuffer

The `StringBuffer` class represents a growable, thread-safe mutable string. JDK 5 added `StringBuilder` which is similar but not thread-safe, making it inadvertently faster.

Constructors exist for creating one with a given capacity size or to build one from an existing from an existing string plus an additional 16 characters in capacity. The default constructor only reserves 16 characters for its capacity.

It's possible to ensure a certain capacity is available with the `ensureCapacity` method which is given the minimum size that the buffer should have. The `setLength` method can be used to either extend the string by adding null characters or to truncate the string.

`StringBuffer` provides a `setCharAt` method that can modify a character at the provided position. The `append` method can concatenate strings to the buffer while returning the updated buffer, allowing calls to this method to be chained. The `insert` method can insert a given string at the specified index. The `reverse` method can reverse the string. The `delete` and `deleteCharAt` methods can remove a region of the string or a single character respectively. The `replace` method can replace a region of the string with another string, even if it differs in length.

# java.lang

## Runtime

The abstract class `Process` represents an executing program and is derived by objects created by `exec` in `Runtime` or `start` in `ProcessBuilder`.

The `Runtime` class represents the Java Virtual Machine's run-time environment. A reference to the current run-time's `Runtime` instance can be retrieved using `Runtime.getRuntime()`. `Runtime` defines methods such as `exec` for executing processes, in which case it returns an object of type `Process` that describes the process.

`Runtime` provides methods such as `gc` to manually initiate garbage collection, or `runFinalization` to initiate the `finalize` methods of unused but not yet garbage collected objects.

There is also `exit` for halting execution of the program.

## Process

The `Process` class can be used to control a running process. The `destroy` method can be used to kill the process. The `waitFor` method waits until the process finishes, whereas `exitValue` does the same but yields the process' exit value. Access to the input and output streams are available via the `getOutputStream` and `getInputStream` methods.

The `ProcessBuilder` class provides even more control over a process, allowing for example to set the working directory. The constructors accept either a variable argument list of strings or a `List<String>`. The `start` method is used to actually start execution of the process.

There is also a static class `ProcessBuilder.Redirect` which has methods `to`, `from`, and `appendTo` which can be used to redirect the input or output streams to or from a given file. The `type` method returns a value of the enumeration type `ProcessBuilder.Redirect.Type` describing the type of redirection, which can be `APPEND`, `INHERIT`, `PIPE`, and `WRITE`.

## System

The `System` class provides a variety of static methods and variables, such as the input, output, and error streams in `System.{in,out,err}`.

There is also---for some reason---a method for copying arrays, `arrayCopy`, which takes a reference to the array, its starting index, the same for the other array, and a size.

## Object

The `Object` class defines a method `clone` which generates a duplicate copy of the object, but only if the class implements the `Cloneable` interface. The `Cloneable` interface defines no members and is instead used to signal to the system that it is safe to create a bitwise copy of a particular type of object.

Since it may sometimes be dangerous to perform bitwise copies of certain classes, it may be useful to override `clone`.

## Class

The `Class` class represents the run-time state of a class or interface, and objects of this type are created automatically when classes are loaded. A reference to an object's `Class` instance can be retrieved using the `getClass` method defined on `Object`. A `Class` instance can also be retrieved using `forName` static method. The `Class` method provides a variety of Run-time Type Information methods such as `getName`, `getSuperClass`, and so on.

This class is a good real-world example of the use of wildcards as well as lower and upper bounds.

``` java
Class<?> forName(String name)
Class<? super T> getSuperClass()
<A extends Annotation> A getAnnotation(Class<A> annoType)
```

The `ClassLoader` abstract class defines how classes are loaded, enabling one to alter the way classes are loaded by the JVM by simply deriving from it.

## Package

The `Package` class provides information associated with a package.

## Comparable

The `Comparable` interface represents objects that can be compared, and provides a single method `compareTo` which should return 0 if the values are equal, a negative number if the invoking object is lower, or a positive number if the invoking object is greater.

## Appendable

The `Appendable` interface signifies that a character or character sequences can be appended to an object by means of its `append` method.

## Iterable

The `Iterable` interface signifies that an object can be used in a for-each loop. It provides an `iterator` method yielding an `Iterator` of the object. JDK 8 also provides two default methods `forEach` and `splititerator`. The `forEach` method takes functional interface `Consumer` and applies it to each element yielded by the iterator.

## AutoCloseable

The `AutoCloseable` interface signifies that the object can be used with the try-with-resources statement which provides automatic resource management by means of its `close` method.

# Collections

The Java Collections Framework provides a variety of collections and interfaces for working with them.

## Collection Interfaces

|Collection     | Purpose                                                          |
|:--------------|:-----------------------------------------------------------------|
|`Collection`   | work with groups of objects                                      |
|`List`         | extends `Collection` to handle sequences of objects              |
|`Queue`        | extends `Collection` to handle lists with removal only from head |
|`Deque`        | extends `Queue` to handle double-ended queue                     |
|`Set`          | extends `Collection` to handle sets                              |
|`SortedSet`    | extends `Set` to handle sorted sets                              |
|`NavigableSet` | extends `SortedSet` to add closest-match retrieval               |

### Collection

The `Collection<E>` interface represents a generic collection of elements, and it extends the `Iterable` interface so that all collections are inherently compatible with for-each loops. The methods of `Collection` may throw a variety of exceptions:

| Exception                       | Cause                                           |
| :----------                     | :------                                         |
| `UnsupportedOperationException` | attempting to mutate immutable collection       |
| `ClassCastException`            | adding incompatible object                      |
| `NullPointerException`          | storing a `null` when `null`s aren't allowed    |
| `IllegalStateException`         | adding element to fixed-length, full collection |

The `add` method adds an object to a collection, returning a boolean indicating whether the object was added or if it already existed and duplicates are not allowed. The `addAll` method adds all of the objects from another collection.

The `remove` and `removeAll` methods are analogous to the `add` methods. `clear` removes all elements. The JDK 8 `removeIf` method removes all elements that satisfy the provided predicate. The `retainAll` method removes all elements except those in the provided collection.

The `size` method provides the number of elements in the collection.

The `contains` method returns `true` is the object is present in the collection. There is also a `containsAll` method similar to `addAll`.

The `equals` method provides equality checking, whether it's value or reference equality is up to the implementer. The `isEmpty` method checks if the collection is empty.

The `toArray` methods can return an array of the elements in the collection. The first overload returns an array of `Object` whereas the second takes an array parameter into which the elements are written if they fit, otherwise an array is returned. If they did fit and the array was larger than the amount of elements, the element after the last collection element is set to `null`.

### List

The `List` interface extends `Collection` to provide behavior for a sequence of elements, which can be inserted or accessed by their zero-based index position. The methods here may throw `IndexOutOfBoundsException` if an invalid index is used. Overloads are provided for the `add` methods that take an index argument to specify where to insert the element. The overloads without the index parameter are changed in `List` to insert the elements at the end of the sequence.

The `get` method takes an index argument and retrieves the element at that index. Conversely, the `set` method takes an index parameter and an element with which to replace the element at that index.

The `indexOf` and `lastIndexOf` methods can be used to find an element in the sequence and retrieve its index.

A sub-list of the sequence can be obtained using the `subList` method and specifying beginning and end indices.

The `sort` method can sort a `List` using a provided `Comparator`.

### Set

The `Set` interface extends `Collection` and adds behavior that doesn't allow duplicate elements. It doesn't actually provide any additional methods aside from providing this behavior. The `add` method returns `false` if the element already existed within the collection.

There are no union, intersection, or difference methods but they may be emulated using other methods. Union can be performed using `addAll`, intersection with `retainAll`, and difference with `removeAll`.

### SortedSet

The `SortedSet` interface extends `Set` to add the behavior of a set sorted in ascending order. It provides methods such as `first` and `last` for getting the first and last elements. A sorted subset can be obtained using `subSet` and specifying start and end indices, or there are the `headSet` and `tailSet` methods that obtain a subset starting with the first element or a subset that ends the set---respectively---up to a certain end index..

### NavigableSet

The `NavigableSet` interface extends `SortedSet` and provides closest match element retrieval. For example, the `lower` method will find the largest element that is smaller than the provided object, whereas the `floor` method will find the smallest element that is smaller than or equal to the provided object. There are also `higher` and `ceil` analogs to those methods.

There are also methods that behave like priority queues, such as `pollFirst` which returns the first element---which will be the smallest element since the set is sorted in ascending order---and removes it from the set. There is also a `pollLast` analog.

### Queue

The `Queue` interface extends `Collection` and adds behavior for a FIFO structure. Elements can only be removed from the head of the queue via the methods `poll` and `remove`, where the first returns `null` if the queue is empty and `remove` throws an exception. The methods `peek` and `element` are analogous to `poll` and `remove` respectively, but _don't_ remove the element from the queue. An addition to the queue can be attempted via the `offer` method, which may fail if the queue is of fixed-size and is full, in which case it returns `false`.

### Deque

The `Deque` interface extends `Queue` to add behavior for a double-ended queue so that a queue can function as a FIFO (queue) as well as a LIFO (stack) thanks to methods `push` and `pop`. The `descendingIterator` returns an iterator that iterates over the elements in reverse. There are also `addFirst` and `addLast` methods that are similar to `offer` except they throw `IllegalStateException` if the queue is of fixed-size and full.

### RandomAccess

The `RandomAccess` interface has no members and simply signifies that that the collection supports efficient random access.

## Collection Classes

### ArrayList

The `ArrayList` class is similar to `vector` in C++ in that it represents an array that will grow as required. It can be constructed either as an empty array list, from an arbitrary collection, or an empty array list with a reserved capacity. Capacity can be reserved after construction using the `ensureCapacity` method. The `trimToSize` method can be used to shrink the array to the minimum size required to store all of the elements.

The `toArray` method from `Collection` can be used to yield an array from an `ArrayList`:

``` java
ArrayList<Integer> arrayList = new ArrayList<Integer>();
Integer array[] = new Integer[arrayList.size()];

// fills array with elements if it's large enough,
// otherwise it returns a new array that is large enough
array = arrayList.toArray(array);
```

### LinkedList

The `LinkedList` class extends `AbstractSequentialList` and implements `List`, `Deque`, and `Queue`, and provides linked-list behavior. The `add` method would be used to insert elements at a particular location with minimal performance cost.

### HashSet

The `HashSet` class extends `AbstractSet` and implements `Set`, and provides hash table behavior. Two of the constructors accept a capacity argument (default 16), with one of them accepting a load capacity argument (default 0.75) known as _fill ratio_, which determines how full the hash set can be before it is grown, and as such it must be a value between 0.0 and 1.0. In other words, the hash set is grown when:

$$ \text{# of elements} \gt \text{capacity} \cdot \text{fill ratio} $$

### LinkedHashSet

The `LinkedHashSet` class extends `HashSet` and adds no members of its own aside from adding the behavior that it maintains a linked list of elements in order of insertion.

### TreeSet

The `TreeSet` class extends `AbstractSet` and implements `NavigableSet` and represents a tree-backed ascending-order sorted set. This class is great when storing many sorted elements that must be accessed quickly. One of the constructors accepts a `Comparator` to use for sorting the elements. Another constructor can build the `TreeSet` from another `SortedSet`.

### PriorityQueue

The `PriorityQueue` class extends `AbstractQueue` and implements `Queue` and provides priority queue behavior. One of the JDK 8 constructors accepts a `Comparator` used to order the elements, which is also possible via another non-JDK 8 constructor that takes a capacity and `Comparator`. A reference to the `Comparator` can be obtained using the `comparator` method, which returns `null` if the default ascending order is used.

Note that manually iterating over a `PriorityQueue` yields an undefined order, so `offer` and `poll` should be used instead.

### ArrayDeque

The `ArrayDeque` class extends `AbstractCollection` and implements `Deque` and can be used as a growable stack.

### EnumSet

The `EnumSet` class extends `AbstractSet` and implements `Set` and is used for enumerations, as enforced by its signature which forces all elements to be of the same enumeration type:

``` java
class EnumSet<E extends Enum<E>>
```

It provides no constructors and instead has static factory methods. The `allOf` method creates an `EnumSet` of all possible enumerations of a given type represented by a `Class` object. The `complementOf` method creates a set of all enumerations not present in the given set. More generally, the `of` method accepts an arbitrary amount of enumerations and constructs an `EnumSet` from it, providing overloads for efficiency. The `range` method creates a set from the given range of enumerations.

## Iterator

The `Iterator` interface encapsulates the act of iterating over a collection. It provides methods `hasNext` and `next` to both test if a value remains and to obtain that value. The `remove` method can be used to remove the current element from the collection being iterated over, but may throw `IllegalStateException` if the call was not preceded by `next` or if the collection is read-only. The JDK 8 method `forEachRemaining` takes a `Consumer` and applies it to each remaining element in the iterator.

The `ListIterator` interface extends `Iterator` and adds bidirectional iteration as well as modification of elements. It's accessible from collections that implement `List`. The `add` method inserts an element before the element that will be returned by the next call to `next`. It also provides bidirectional equivalents to the methods in `Iterator` such as `hasPrevious` and `previous`.

The `nextIndex` and `previousIndex` methods return the index of the next or previous element respectively. If there is no such element, it returns the size of the list in the case of `nextIndex` or -1 in the case of `previousIndex`, i.e. one past the last element or one before the first element.

The `set` method can be used to replace the current element, which is the element last returned by `next` or `previous`.

## Spliterator

JDK 8 introduces a new kind of iterator known as a _spliterator_, represented with the `Spliterator` interface. Spliterators provide support for parallel iteration, but also provide many more facilities than regular iterators making them useful in non-parallel contexts.

Iterating with a `Spliterator` is done using the `tryAdvance` which applies a `Consumer` to the next element, returning `false` if there is no element remaining. The `forEachRemaining` method does the same thing but for every element remaining.

The fact that `tryAdvance` returns `false` when no elements remain means that it can be used in a while loop very easily, though the same can be done using `forEachRemaining`:

``` java
while (spliterator.tryAdvance((e) -> System.out.println(e)));

// order
spliterator.forEachRemaining((e) -> System.out.println(e));
```

The spliterator can be split further using the `trySplit` method which yields a new spliterator that iterates over a portion of the sequence and the invoking spliterator iterates over the other portion, or it returns `null` if it's not possible to split further.

Spliterators can contain characteristics which are retrieved using the `characteristics` method to retrieve them all or the `hasCharacteristics` method to test for an individual characteristic. Characteristics are defined as static integer fields on `Spliterator`, such as `SORTED` and `IMMUTABLE`.

## Maps

Maps represents associations between keys and values. They **do not** implement `Iterable` and so the pairs cannot be iterated over.

### Map Interfaces

| Collection      | Purpose                                          |
| :-------------- | :----------------------------------------------- |
| `Map`           | maps keys to values                              |
| `Map.Entry`     | describes a key-value pair                       |
| `SortedMap`     | extends `Map` to put keys in ascending order     |
| `NavigableMap`  | extends `SortedMap` for closest-match retrieval  |

#### Map

The `Map` interface embodies behavior for key-value stores. The two fundamental methods provided are `get` and `put` which are used to retrieve and insert into the map. The `remove` method takes a key and removes the entry associated with it, returning it.

Though maps aren't collections since they don't implement `Collection`, they do provide collection views for their keys and values via the `keySet` and `values` methods respectively, or over the key-value pairs using the `entrySet` method. Each of the collection views is backed by the map, so changing values through the view changes the values in the map as well.

There are some `compute` variant methods that facilitate the use of a map as a cache, such as `computeIfAbsent`, which takes a function and returns the value associated with the key, and if it doesn't exist, computes the value using the function, stores it in the map, and then returns the computed value [^rust_entry].

[^rust_entry]: This is very much like the Rust [Entry API](http://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html).

The `containsKey` and `containsValue` methods can be used to test the presence of a key or value respectively. The `equals` method can be used to check if another map contains the same entries.

The JDK 8 method `forEach` can be used to apply an action on each entry in the map.

The JDK 8 method `merge` takes a key and value and inserts it into the map if the key didn't already exist, otherwise it computes a new value given the old value.

The `putAll` method puts all the entries from another map into the invoking map.

#### SortedMap

The `SortedMap` interface extends `Map` and ensures that entries are stored in ascending order. It provides very efficient sub-map manipulations via the `headMap`, `tailMap`, and `subMap` methods. The first and last key can be obtained using `firstKey` and `lastKey` respectively.

#### NavigableMap

The `NavigableMap` interface extends `SortedMap` and provides closest-match retrieval of key(s).

#### Map.Entry

The `Map.Entry` interface represents a map entry, i.e. a key-value pair. It provides methods for getting the key and value via `getKey` and `getValue` respectively as well as setting the value using `setValue`.

``` java
Set<Map.Entry<String, String>> set = map.entrySet();

for (Map.Entry<String, String> entry : set) {
  System.out.println(entry.getKey() + ": " + entry.getValue());
}
```

### Map Classes

The `WeakHashMap` class uses weak keys so that the entry can be garbage collected when the key is otherwise unused.

#### HashMap

The `HashMap` class extends `AbstractMap` and implements the `Map` interface and represents a map backed by a hash table. It provides constructors similar to `HashSet`'s, such as those that allow to set the capacity and load capacity (fill ratio).

#### TreeMap

The `TreeMap` class extends `AbstractMap` and implements `NavigableMap`. It's like `TreeSet` in that it can store entries in sorted order allowing for efficient retrieval. It has similar methods to `TreeSort` such as one that takes a `Comparator`.

#### LinkedHashMap

The `LinkedHashMap` class extends `HashMap` and is the map analog to `LinkedHashSet` in that it maintains a linked list of entries in the order in which they were inserted. This means that iterating over a collection view of the map yields elements in insertion order.

One of the constructors takes an `order` parameter after the capacity and load capacity which specifies whether the linked list should store elements in insertion order or by last access order.

It also provides a single additional method aside from those defined by `HashMap` and that is `removeEldestEntry`. This function is called internally after calling `put` or `putAll` and is used to determine whether or not to remove the oldest entry in the map. For that purpose, it returns `false` by default but can be overridden to provide different behavior, such as a fixed-size LRU cache:

``` java
protected boolean removeEldestEntry(Map.Entry<K, V> entry) {
    return this.size() > self.MAX_SIZE;
}
```

#### EnumMap

The `EnumMap` extends `AbstractMap` and implements `Map`. It specifically takes enumerations for keys.

``` java
class EnumMap<K extends Enum<K>, V>
```

## Comparators

The `Comparator` interface represents an arbitrary comparison between two values. Prior to JDK 8 it defined two methods `compare` and `equals`. JDK 8 adds many more methods as default and static interface methods.

The default method `reverse` returns a comparator that is the reverse of the invoking comparator.

The static methods `naturalOrder` and `reverseOrder` provide comparators for the natural ordering and the reverse of it respectively.

The static methods `nullsFirst` and `nullsLast` adapts an existing comparator so that can handle null values and considers them to be first or last in the order respectively. If the comparator passed is `null`, then all non-`null` values are considered equivalent.

The default method `thenComparing` returns a comparator that chains a comparator in the event that the invoking comparator considers two values to be equivalent. Two additional overloads accept a function for selecting the next comparison key to compare as well as the comparator to use. There are also specialized versions for primitives such as `thenComparingInt`.

The static method `comparing` takes a function to select a comparison key and returns a comparator that compares based on that key. The second overload accepts an arbitrary comparator and adapts it accordingly. As with `thenComparing`, there are specialized versions of `comparing` for primitives, such as `comparingInt`.

Since `Comparator` only requires one method to be implemented---the reset being default or static methods---it is possible to use a lambda to instantiate a comparator.

## Collection Algorithms

The `Collections` class provides a variety of algorithms as static methods.

The `checkedCollection` family of methods returns a run-time type-safe collection view which provides run-time checks to ensure that compatible objects are inserted into the collection, throwing `ClassCastException` if the check fails. There are `checkedSet`, `checkedList`, `checkedMap`, etc.

Thread-safe (synchronized) copies of collections can be obtained using the `synchronized` family of methods such as `synchronizedList`. Iteration over synchronized collections must be performed within `synchronized` blocks.

The `unmodifiable` family of methods such as `unmodifiableSet` provides an immutable view over a collection. `Collections` provides three static methods that yield immutable collections: `EMPTY_SET`, `EMPTY_LIST`, and `EMPTY_MAP`.

The `asLifoQueue` provides a LIFO (stack) view of a `Deque` as a `Queue`.

The `binarySearch` method takes a list and a search value and performs a binary search on the list, returning the index of the match or a negative value if none was found.

The `disjoint` method checks if the two collections have no elements in common.

The `emptyIterator` method yields an empty iterator.

The `fill` method takes a list and an object and replaces each element in the list with that object.

The `frequency` method takes a collection and an object and counts the number of occurrences in it.

The `indexOfSublist` method takes two lists and returns the index of the beginning of the first match or -1 if none was found. There is also `lastIndexOfSubList`.

The `max` and `min` methods return the maximum and minimum element in the collection based on natural order, respectively. Overloads exist that accept a custom comparator.

The `replaceAll` method replaces all occurrences of one value with another in a given list.

The `reverse` method reverses a list.

The `reverseOrder` method returns a comparator that is the reverse of the one that is passed, or the reverse of the natural order if none is given.

The `rotate` method reverse a list by a given number of places to the right, where a negative number rotates to the left.

The `shuffle` method shuffles the elements in a list given a `Random` seed, or an arbitrary seed if none is given.

The `sort` method sorts a list given a comparator, or uses natural order if none is given.

The `swap` method swaps two elements of a list at the given indices.

## Arrays

The `asList` method returns a `List` backed by the invoking array, so that modifying one modifies the other.

The `binarySearch` method performs binary search on a sorted array for a given value, returning the index if found or a negative value if not found. One overload accepts a custom comparator, while others allow specifying a sub-range.

The `copyOf` method returns a copy of the array up to a certain size. If the size is shorter then the copy is truncated, and if it's larger it is padded with zeros for numeric arrays, `null`s for object arrays, and `false` for boolean arrays. The `copyOfRange` method is similar except it allows specifying a sub-range to copy by providing a start and end index.

The `equals` method tests if two arrays are equivalent. The `deepEquals` array does the same for arrays that may contain other arrays.

The `fill` method assigns a value to all elements in the array, with an overload accepting a sub-range to fill.

The `sort` method sorts an array into ascending order, or one of the overloads accepts a custom comparator. Other overloads allow specifying a sub-range.

### Arrays in JDK 8

JDK 8 adds a variety of new methods.

The `parallelSort` method which performs a sort in parallel and then merges the results, which provides similar overloads to `sort`.

The `spliterator` method returns a spliterator of an entire array, with an overload accepting a sub-range to iterate over.

The `stream` method yields a `Stream` for use with the JDK 8 `Stream` interface.

The `setAll` and `parallelSetAll` methods assign values to all elements based on the result of applying a provided generator function on a given element.

The `parallelPrefix` method performs an operation on all previous elements for each element [^haskell_scan]. So that if the operation is addition, each element will be the sum of all elements prior to it.

[^haskell_scan]: This sounds a lot like Haskell's [`scan`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Prelude.html#g:16) family of functions.

The `toString` and `hashCode` methods work on arrays as well as `deepToString` and `deepHashCode` variants.

## Legacy Collections

The `Enumeration` interface is a legacy version of `Iterator` and some classes still use it, such as `SequenceInputStream`. It defines two methods: `hasMoreElements` and `nextElement`, where the former must return `true` so long as there are still more elements to process and `nextElement` must return the next element if there is one, or throw `NoSuchElementException` when enumeration is complete.

The `Vector` class is a legacy version of `ArrayList`.

The `Stack` class is a LIFO subclass of `Vector`.

The `Dictionary` abstract class is a legacy version of `Map`.

The `Hashtable` class is a legacy version of `HashMap`.

The `Properties` class is a subclass of `Hashtable` and is still used for Java system related functionality, such as `System.getProperties`. It can retrieve values using `getProperty` and set properties with `setProperty`. It's possible to specify a default value to return if no value is associated with a given key by giving the `getProperty` a second argument. Alternatively, a constructor can take another `Properties` object to use as default properties. The `store` and `load` methods can be used to serialize and deserialize the properties to to a stream.

# java.util

## BitSet

The `BitSet` class is a set where each member is an individual bit. It's backed by an array which grows dynamically, though one of the constructors can set the initial size.

The `set` method can be used to set a bit at a particular index, with overloads for explicitly setting the value instead of the default of `true` as well as setting a range. The `get` method returns a `boolean` representing the state of a bit at a particular index, with an overload accepting a range and returning a new `BitSet` containing only that range.

The `cardinality` method returns the number of set bits. The `clear` method zeros all of the bits, with overloads for a specific index or range. The `flip` method flips an individual bit.

It provides a variety of methods such as `and`, `andNot`, `or`, and `xor` which takes another bitset and performs that operation on it with the result overwriting the invoking object.

The `intersects` method returns `true` if at least one of the same bits in the invoking `BitSet` and the argument are set.

The `length` method returns the number of bits needed to hold the invoking `BitSet`, based on the last set bit.

The `nextClearBit` and `nextSetBit` methods find the next clear or set bit from the provided index, returning -1 if none is found. There are also `previousClearBit` and `previousSetBit` methods.

## Optional

JDK 8 introduces an `Optional` type that is similar to Haskell's `Maybe` and Scala & Rust's `Option` in that it represents the possibility of a value. Previous to JDK 8 objects would be set to `null` for this purpose, which would lead to null pointer exceptions if unchecked prior to their use.

There are no constructors for `Optional`, instead there are static methods for creating `Optional`s such as `of` which takes a value that must not be `null` or `ofNullable` which takes a value that may be `null` in which case it returns an empty `Optional`.

The `get` method is used for unwrapping the `Optional`, but may throw `NoSuchElementException` if the `Optional` is empty.

The `isPresent` method may be used to check if the `Optional` is not empty, with an overload accepting a `Consumer` function which is applied to the contained value if the `Optional` is not empty.

The `orElse` method returns the contained value or a provided default value if the `Optional` is empty, like Rust's `unwrap_or`. The `orElseGet` function is similar except it accepts a `Supplier` which is invoked to obtain the default value to return if the invoking `Optional` is empty, like Rust's `unwrap_or_else`. The `orElseThrow` method returns the value or throws an exception generated by the provided `Supplier`.

The `filter` method applies a predicate to the contained value and returns an empty `Optional` if the predicate fails, or the original `Optional` otherwise. It's essentially mapping a conditional identity function over the value.

The `flatMap` method applies a given function to the contained value if any and returns a new `Optional` of the result, similar to Rust's `and_then` or Haskell's bind `>>=`.

The `map` method applies a given function to the contained value to another value.

There are also specialized variants of the `Optional` class for primitives which are `OptionalDouble`, `OptionalInt`, and `OptionalLong` which have methods such as `getAsDouble` instead of regular `get`, and don't support `filter`, `ofNullable`, `map`, or `flatMap`.

## Random

The `Random` class is a pseudorandom number generator. A variety of different kinds of numbers can be extracted from `Random` via different methods such as `nextBoolean`, `nextBytes`, `nextInt`, and so on. The `nextBytes` method in particular takes an array and fills it with the randomly generated values. The `nextInt` method has an overload that accepts an upper bound so that numbers are generated within the range $[0, n)$.

The seed can be passed to one of the constructor overloads or reset after the fact with the `setSeed` method.

With the stream API the methods `doubles`, `ints`, and `longs` return a reference to a stream of the appropriate type.

## Observable

The `Observable` class can be derived so that other classes can register interest in the class so that they are notified of any changes to objects of the class. To accomplish this, the derived class must call `setChanged` whenever the object is changed, and then it must notify observers of the change using `notifyObservers`, which causes the `update` method to be called on the observing objects. An overload of `notifyObservers` exists which accepts an arbitrary object which is passed as the second argument to the observing objects' `update` method, otherwise `null` is passed.

Observers can be added using the `addObserver` method, whereas the `deleteObserver` method does the opposite, with a `deleteObservers` variant removing all observers.

The `clearChanged` method can be used to reset the status to "unchanged."

Observer objects must implement the `Observer` interface which defines a method `update` which takes a reference to the `Observerable` object and an optional second object which may be passed through the `notifyObservers` overload.

## Formatter

The `Formatter` class provides format conversions for displaying numbers and other values as strings. By default, `Formatter` builds the result in a `StringBuilder`. The `format` methods can be used for actually formatting strings, with one overload taking a `Locale` as the first parameter. The `toString` method yields a `String` of the output. A `Formatter` should be closed with the `close` method when it's no longer needed, so that underlying resources can be freed. The `System.out.printf` method is a convenient way to leverage `Formatter` without explicitly creating one.

## Scanner

The `Scanner` class is the reverse of the `Formatter` class in that it reads formatted input, deserializing it. It can be created from a `String`, `InputStream`, `File`, or any `Readable` or `ReadableByteChannel`. `Scanner` tokenizes the input using regular expressions, providing built-in patterns for primitive types such as integers.


The `Scanner` is typically used to read formatted input from standard in:

``` java
Scanner scanner = new Scanner(System.in);
```

The general flow of using a scanner is to determine the next token's type of data using the `hasNext` family of methods, such as `hasNextInt`, which is then consumed using the corresponding `next` family of methods, such as `nextInt`. This is repeated as needed, then closed when no longer needed with `close`.

The general `hasNext` method checks if there's another token of _any_ type left in the input. There are also overloads that accept a regular expression to match the next token with. There are corresponding versions of these functions for the `next` family of functions, with a no-parameter `next` method yielding the next token as a `String`. There is also `nextLine` for consuming the entire next line of input.

If the `next` method is called and the next token type doesn't match, `InputMismatchException` is thrown. If instead there is no more data in the input, `NoSuchElementException` is thrown.

`Scanner` implements `AutoCloseable` so it can be used with try-with-resources.

A common pattern is to loop on `hasNext` and then within the loop check for specific types of data.

It's possible to set custom delimiters using the `useDelimiter` method, whereas the current delimiter can be obtained using `delimiter`.

The `findInLine` method looks for a match given a pattern so that and returns the matched string if found, advancing the input to the point past the match. The more general `findWithinHorizon` method is similar but it accepts a maximum character count to look forward in.

The `skip` method is used to look for a specific pattern and advances the input stream to the point past the match, if any is found.

## java.util.function

The `java.util.function` package contains various functional interfaces which can be used by lambda expressions.

`Consumer` variants refer to functions that accept arguments of different types, whereas `Function` variants also produce a result. `Operator` variants are like a `Function` except the parameters and result are all the same type. `Predicate`s accept potentially different types of parameters and returns a `boolean` result. `Supplier` variants take no parameters and provide a result value. `Bi`-prefix variants take two parameters. `To`-prefix variants represent a function that returns a value of the type following the `To` prefix, e.g. `ToDoubleBiFunction`.

There are also specialized primitive prefix variants, such as `DoubleConsumer`.

# java.io

## File

A `File` represents a file system file and can be constructed using a `String` path to a directory, with another overload taking a second `String` path to a file within the directory, while another overload takes a `URI`.

There are `getParent` and `getName` for retrieving the directory and file name components of the path represented by a `File`. The `exists` method yields a `boolean` indicating whether a file at that location exists. The `isFile` method can be used to detect if it's a regular file or a directory.

There is a `renameTo` utility method that takes another `File` instance representing the target to rename the invoking `File` to. There is also a `delete` method to remove a regular file or empty directory from the file system, as well as a `deleteOnExit` method.

The `toPath` method returns a `Path` object of the path to the invoking `File`'s path.

If the `File` is a directory, the `list` method will return a `String` array of file names of the contents of the directory. An overload of the `list` method takes a `FilenameFilter` object which filters the returned list of files to ones that match a particular file name pattern. The `FilenameFilter` interface contains a single method taking a reference to the directory's `File` and the file in question's file name as a string, and must return a `boolean` indicating whether or not the file satisfies the filter.

Alternatively, there is a `listFiles` method that yields `File` instances instead which is otherwise identical to the `list` method, except one of the overloads can take a `FileFilter` which is identical to `FilenameFilter` aside from the fact that it operates on `File` objects instead of `String` file names.

Creating directories is possible with the `mkdir` and `mkdirs` method, where the second one creates all necessary directories in the path, like the `mkdir -p` command.

## I/O Streams

I/O is performed in Java through the stream abstraction which is split into two types: byte streams reserved for binary data and character streams reserved for internationalizable Unicode text (and are sometimes more efficient than byte streams).

Byte streams consist of two hierarchies with the following abstract classes at the top: `InputStream` and `OutputStream`. Character streams are similar, with `Reader` and `Writer` being at the top. Each of these sets of classes define `read` and `write` methods respectively.

The `System` class defines three static, predefined stream variables `in`, `out`, and `err` where `in` is an `InputStream` while `out` and `err` are `PrintStream` types.

For example, to read input from the keyboard, the `InputStream` can be wrapped by `InputStreamReader` to convert bytes to characters, then wrapped in `BufferedReader` to support a buffered input stream.

``` java
BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
```

Likewise, to print characters to the terminal it's preferred to use a `PrintWriter` which can be created by wrapping the `PrintStream`.

``` java
PrintWriter pw = new PrintWriter(System.out, true);
```

## Byte Streams

### InputStream

The `InputStream` abstract class represents streaming byte input.

The `read` method reads the next byte or -1 if it's there's no more input, with a variant that takes a byte array to write the contents into, returning the number of bytes read or -1 if there's no more input. The final overload of `read` takes a byte array, an offset into it, and the maximum number of bytes to read, returning the same thing as the second overload.

The `mark` method places a "bookmark" at the current position in the input stream which will remain valid until the provided number of bytes have been read. The `reset` method then resets the input pointer to the set mark. The `markSupported` method must be consulted before attempting to use these methods, as some streams don't support them. The `skip` method skips the provided number of bytes of input, returning the number of bytes that were actually skipped. The `

The `FileInputStream` class is an `InputStream` for reading bytes from a file. It can be created from a `String` file path or an existing `File` object.

The `ByteArrayInputStream` class is more general in that it uses a byte array as the input source, with one of the constructors taking an offset into the array and a number of bytes to use as input.

### OutputStream

The `OutputStream` abstract class represents streaming byte output. The `available` method returns the number of bytes available for reading.

The `write` method writes a single byte to the output stream, with an overload taking a byte array to write to the output stream. The final overload takes a byte array, an offset into it, and the number of bytes to write.

The `FileOutputStream` class is an `OutputStream` for writing bytes to a file. It can be constructed the same way that a `FileInputStream` can be, with overloads taking an additional parameter indicating whether to append to the file. The file backing the `FileOutputStream` doesn't have to already exist, in which case it's created automatically.

The `ByteArrayOutputStream` class is analogous to the `ByteArrayInputStream` class, except that it uses a byte array as the destination, with one of the constructors specifying the size of the output buffer, which is dynamically grown otherwise.

### Filtered Byte Streams

Filtered streams are adapters around other streams that provide additional behavior. For example, `FilterInputStream` simply extends `InputStream` and overrides all methods of `InputStream` to versions that pass the requests to the wrapped input stream. Arbitrary stream adapters can be created by deriving from `FilterInputStream`, for example, and constructing the adapter from an existing `InputStream`.

Concretely, a `LoggingStream` can be created that derives from `FilterInputStream` and overrides the `read` methods with logging messages. A `LoggingStream` would then be constructed from an existing `InputStream` like `FileInputStream`, so that the `FileInputStream` can continue to be used as expected while triggering the logging messages from the `LoggingStream` adapter.

### Buffered Byte Streams

Buffered streams extend a filtered stream and attach a memory buffer to back the I/O stream, allowing operations on more than one byte at a time, which improves performance and facilitates skipping, marking, and resetting the stream.

The `BufferedInputStream` class for example wraps any `InputStream` into a buffered stream, with one of the overloads taking a buffer size parameter, which should generally be the the size of a memory page or disk block. There is also a `BufferedOutputStream`.

### PushbackInputStream

The `PushbackInputStream` allows peeking at the next byte on the input stream without consuming it [^rust_peekable]. In fact, the amount of peekable data can be specified in one of the constructors by passing the size of the peekable buffer.

Peeking is performed by reading data from the input normally with the `read` method and then *explicitly* "pushing back" the data that should be pushed back using the `unread` method, which takes an integer whose lower byte is sent back to the input stream. There are also overloads that mirror the overloads available for `read`, in particular one that takes an array of bytes and another that takes an array of bytes, an offset into it, and a number of bytes to send back.

[^rust_peekable]: Much like Rust's [`Peekable`](http://doc.rust-lang.org/std/iter/struct.Peekable.html) iterator adapter.

Note that `PushbackInputStream` invalidates the `mark` and `reset` methods.

### SequenceInputStream

The `SequenceInputStream` concatenates multiple `InputStream`s into one and is constructed from either two `InputStream` objects or from an `Enumeration` of `InputStream`s. Closing a `SequenceInputStream` closes all unclosed streams that constitute it.

### PrintStream

The `PrintStream` class is a stream that facilitates printing data, and is what's used when accessing `System.out`. It's constructed from an existing `OutputStream` and can take a parameter determining whether to turn on auto flushing of the stream. Auto flushing is performed whenever a newline is printed, when a byte array is written, or when `println` is called. There is also an overload that accepts a string representing the character set to use for the stream. A `PrintStream` can also be created from a `File` and a `String` path.

The `PrintStream` class also defines the `printf` method which leverages the `Formatter` for outputting formatted strings.

### DataStream

The `DataOutputStream` and `DataInputStream` which are specifically for writing and reading primitive data to or from a stream. They define multiple methods for writing and reading specific types of primitive data, such as `writeDouble`.

### RandomAccessFile

The `RandomAccessFile` class represents a file that can be accessed randomly, which means that the position in the file can be moved around. It's constructed from a `File` or `String` file path and takes a `String` parameter specifying the access policy to use with the file, such as `"r"` or `"rw"`. The `"s"` specifier means that all changes to the file or its metadata are made immediately, whereas `"d"` does the same but only when the file's data is changed.

The `seek` method can be used to move the current position of the file pointer given a byte position.

The `setLength` method can be used to truncate or lengthen a file, where the added portion is undefined.

## Character Streams

The `Reader` and `Writer` abstract classes are analogs to the `InputStream` and `OutputStream` byte stream abstract classes, except that they are instead used for handling Unicode characters.

### Reader

The `Reader` abstract class represents streaming character input. The `read` method returns an integer representation of the next available character. There are also overloads for reading into a `char[]`, as well as writing into a provided `CharBuffer`. There is also an abstract `read` method that takes a `char[], an offset into it, and a number of characters to read into it. The `ready` method returns true if the next read would not block.

### Writer

The `Writer` abstract class represents streaming character output. The `append` method appends a single `char` to the output stream, with overloads accepting a `CharSequence` along or with a range. The `write` method writes a single character to the output stream, with overloads for writing a `char[]` and another for specifying an offset into it and a number of characters to write. There are also simpler overloads for writing a `String`, as well as a substring of a `String`.

### FileReader

The `FileReader` class extends `Reader` and can be constructed from a `String` file path or a `File` object.

### FileWriter

The `FileWriter` class extends `Writer` and can be constructed from a `String` file path or `File object along with a parameter specifying whether it should be open in append mode.

### CharArrayReader

The `CharArrayReader` class represents an input stream that is backed by a character array, like the character equivalent of `ByteArrayInputStream`. It can be constructed from a `char[]`, with one of the overloads accepting an offset into it and a number of characters to use.

### CharArrayWriter

The `CharArrayWriter` class represents an output stream that is backed by a character array. One of the constructors has a parameter specifying the size of the backing buffer.

### BufferedReader

The `BufferedReader` class is an analog to the `BufferedInputStream` class for character streams: it backs an existing stream with a buffer. JDK 8 adds the method `lines` for accessing individual lines in the stream.

### BufferedWriter

The `BufferedWriter` class is an analog to the `BufferedOutputStream` class for character streams.

### PushbackReader

The `PushbackReader` class is an analog of the `PushbackInputStream` class for character streams.

### PrintWriter

The `PrintWriter` class is an analog of the `PrintStream` class for character streams.

## Console

The `Console` class added in JDK 6 is for reading from and writing to a console. Most of its functionality is available through `System.in` and `System.out`. It has no constructors and instead a reference to the associated `Console` can be obtained using the `System.console()` static method, which returns `null` if no console is associated.

The `readPassword` method is useful for reading input without echoing it to the console.

## Serialization

Serialization is writing an object to a byte stream. Serialization in Java correctly handles references and cyclic references. All objects referenced in an object being serialized are automatically serialized as well, and this is correctly handled at the point of deserialization.

Objects can be serializable by implementing the `Serializable` interface, which contains no members and is only used to signify that the class and its subclasses may be serialized.

Member variables declared as `transient` are not serialized.

The `Externalizable` interface can be used for customizing parts of the serialization process to enable for example compression and/or encryption of the serialized data. It defines methods for reading and writing the data: `readExternal` and `writeExternal`, which take an input byte stream and output byte stream respectively.

The `ObjectOutput` interface extends `DataOutput` and represents object serialization. It contains a `writeObject` method which is used for serializing an object to the stream. There are also general `write` methods found in output streams.

The `ObjectOutputStream` class extends `OutputStream` and implements `ObjectOutput` and is actually responsible for writing objects to a stream. It can be constructed from a general `OutputStream` which is the stream to which the object is written. It contains a variety of `write` methods such as one that takes a `byte[]`, one that takes a `byte[]` and offset and length, as well as variants for all of the primitive data types as well as a regular `Object`.

The `ObjectInput` interface extends `DataInput` and represents object deserialization. The `readObject` method is used for deserializing an object.

The `ObjectInputStream` class extends `InputStream` and implements `ObjectInput` and is responsible for reading objects from a stream. It can be constructed from the stream from which to read the object. Like `ObjectOutputStream`, it contains a variety of `read` methods.

The general process of serialization is to create a backing stream such as a `FileOutputStream` and wrap it in an `ObjectOutputStream`, then invoking `writeObject` on it to serialize a particular object. Deserialization is achieved by doing the reverse: creating a `FileInputStream`, wrapping it in an `ObjectInputStream`, and invoking `readObject`.

# NIO

NIO (new I/O) is built on buffers and channels. Buffers hold data while channels represent open connections to an I/O device such as a file. Channels read to and from buffers.

## Buffers

NIO buffers are subclasses of the `Buffer` class which represents buffers with a current position, limit, and capacity. The limit is the index past the last valid location of data in the buffer. Subclasses of `Buffer` include `ByteBuffer` and buffers specialized for primitive data, as well as `MappedByteBuffer` which extends `ByteBuffer` and maps a file to a buffer [^mmap_file].

[^mmap_file]: Is this like [`mmap`](http://man7.org/linux/man-pages/man2/mmap.2.html)?

Buffers provide `put` and `get` methods for reading and writing to a buffer. The `allocate` method can be used to allocate a buffer manually, or an existing array can be used to back a Buffer using the `wrap` method. A sequence of a buffer can be created with `slice`.

There is a `mark` method and as well as a `reset` method that resets the position to the last set `mark`.

A `rewind` method sets the position to the beginning of the buffer, which is necessary when writing to a buffer and then wanting to read from it from the beginning.

Alternatively, the `flip` method sets the position to the beginning of the buffer and sets the limit to the previous position, which is convenient for writing to the buffer, flipping, then reading from it so that only the written portion is read.

## Channels

NIO channels represent an open connection to an I/O device. All channels implement the `Channel` interface. A channel can be obtained from an object that supports channels by calling `getChannel` on it. The actual type of channel returned differs based on the source object. This is supported by:

* `DatagramSocket`
* `FileInputStream`
* `FileOutputStream`
* `RandomAccessFile`
* `ServerSocket`
* `Socket`

Alternatively, they can be created manually by calling static methods on the source objects, such as `Files.newByteChannel`, and providing it the `Path` to the file.

## Charsets

A _charset_ defines how bytes are mapped to characters. A sequence of characters are encoded into bytes using an _encoder_. A sequence of bytes is decoded into characters using a _decoder_. Charsets, encoders, and decoders are available in the `java.nio.charset` package.

## Selectors

A _selector_ provides key-based, non-blocking, multiplexed I/O. Selectors are used for performing I/O through multiple channels. Selectors are available in `java.nio.channels`.

## Path

The `Path` interface encapsulates a path to a file and it implements `Watchable`, `Iterable<Path>`, and `Comparable<Path>`. The `Watchable` interface represents an object that can be monitored for changes.

The `getName` method can access a specific component of a path given an index. The number of components in the path can be obtained using `getNameCount`. A `String` representation of the path can be obtained with `toString`.

The `Paths` class provides methods for retrieving a concrete class that implements the `Path` interface. The `get` method that takes a `String` path to a file, optionally followed by individual variable-argument list components. Another overload of `get` accepts a URI.

### File System Traversal

NIO provides better facilities for performing information about a file and its path. It's possible to read a directory's contents using a directory stream which can be obtained with `newDirectoryStream` on `Files` with the `Path` to the directory. This returns a `DirectoryStream<Path>` which implements `Iterable<Path>`, allowing a regular for-each loop to iterate over the directory contents, however, the iterator can only be obtained once over the lifetime of the directory stream.

``` java
DirectoryStream<Path> stream = Files.newDirectoryStream(Paths.get("/home"));

for (Path entry : stream) {
  System.out.printf("> %s\n", entry);
}
```

An overload of `newDirectoryStream` takes a `String` parameter representing a wildcard pattern with which to filter entries. Another overload takes a `DirectoryStream.Filter` instead of a wildcard, which has an `accept` method which specifies whether the file is accepted based on the `Path`, allowing filtering based on the file's attributes, for example.

The `walkFileTree` static method on `Paths` enables the recursive enumeration of a directory's contents. It takes a `Path` to the root to begin enumerating at and a `FileVisitor` object.

The `FileVisitor` interface represents how files are visited in a directory tree. It provides a series of pre and post-visiting hooks for directories, a visit hook for files, and finally a `visitFileFailed` hook. For the file and directory visiting hooks, a `Path` of the file or directory is passed as well as the file attributes. The `visitFileFailed` method is passed the `Path` to the file that failed to be visited as well as the `IOException` that was thrown.

Each of the `FileVisitor` methods returns a `FileVisitResult` enumeration which can be any of the following values. Note that `SKIP_SIBLINGS` and `SKIP_SUBTREE` must only be returned from `preVisitDirectory` and have the effect of preventing the call to `postVisitDirectory`.

| Value | Meaning |
|:------|:--------|
| `CONTINUE` | continue visiting |
| `SKIP_SIBLINGS` | skip directory and siblings |
| `SKIP_SUBTREE` | skip directory and children |
| `TERMINATE` | stop visiting |

It's much more common and convenient to extend the `SimpleFileVisitor` class which implements `FileVisitor` so that only select behavior needs to be overridden.

``` java
class TestVisitor extends SimpleFileVisitor<Path> {
  public FileVisitResult visitFile(Path path, BasicFileAttributes attrs)
    throws IOException {
    System.out.println(path);
    return FileVisitResult.CONTINUE;
  }
}

Files.walkFileTree(Paths.get("/home"), new TestVisitor());
```

## Files

The `Files` class provides a variety of static methods for performing actions on a file specified as a `Path`. JDK 8 adds methods `list`, `walk`, `lines`, and `find` which each return a `Stream` object.

The `delete` method on `Files` for example takes a `Path` to a file to be deleted. There are also many other utility methods such as `copy`.

## OpenOption

The `OpenOption` interface is used for specifying how a file should be opened [^rust_openoptions] and is implemented by the `StandardOpenOption` class which defines an enumeration containing for example `CREATE_NEW` for creating a file only if it doesn't already exist.

[^rust_openoptions]: Probably the inspiration for Rust's [`OpenOptions`](http://doc.rust-lang.org/std/fs/struct.OpenOptions.html).

## File Attributes

Attributes such as wetter a file is a directory, a file's size, and so on are represented by a variety of interfaces in `java.nio.file.attribute` with the top interface being `BasicFileAttributes` which encapsulates common file attributes via methods such as `creationTime`, `isDirectory`, `lastModifiedTime`, and so on.

Platform specific file attributes are represented by interfaces that derive from `BasicFileAttributes` such as `DosFileAttributes` for FAT file systems, such as `isSystem`, and `PosixFileAttributes` for POSIX file attributes, such as `permissions`.

File attributes for a particular file can be obtained using the `readAttributes` static method on `Files` which takes a `Path`, a `Class` representing the attribute type e.g. `BasicFileAttributes.class`, and optional `LinkOption` which specify whether to follow symbolic links.

Another way to obtain file attributes is by calling the `getFileAttributeView` static method on `Files`.

However, there are already some dedicated static methods on `Files` for accessing specific file attributes, such as `isWritable` and `exists`.

## Channel-based I/O

### Reading from a Channel

Reading a file using a channel can be done in various ways. One way is to obtain a channel via `Files.newByteChannel` which returns a `SeekableByteChannel` object such as `FileChannel`. Then a buffer must be created for use by the channel either by wrapping an existing array or allocating one with `ByteBuffer.allocate`, passing it the size of the buffer.

Once there is a channel to the file and a buffer for use by the channel, the `read` method can be called on the channel with a reference to the buffer, which returns the number of bytes actually read or -1 on EOF.

``` java
Path file = Paths.get("file.txt");
SeekableByteChannel channel = Files.newByteChannel(file);
ByteBuffer buffer = ByteBuffer.allocate(128);

int count = channel.read(buffer);

if (count != -1) {
  // reset position of buffer for reading
  buffer.rewind();

  System.out.print((char)buffer.get());
}
```

### Reading from a Memory Map

Another way to read a file is to map it to a buffer directly, so that the entire contents of the file are in the buffer. This is done by calling `map` on the channel. The `map` method takes a map mode argument which can be `MapMode.READ_ONLY`, `mapmode.READ_WRITE`, and `mapmode.PRIVATE`, where `PRIVATE` causes a copy of the file to be made so that changes don't affect the backing file. The second and third parameters are the offset into the file to begin mapping and the length to map.

``` java
Path file = Paths.get("file.txt");
SeekableByteChannel channel = Files.newByteChannel(file);

// file size
int size = channel.size();

MappedByteBuffer fileMap = channel.map(FileChannel.MapMode.READ_ONLY, 0, size);

// print first byte
System.out.println((char)fileMap.get())
```

### Writing to a Channel

There are also many ways to write to a file using channels. The first way is the reverse of reading from a channel. Data is written to a buffer and then the buffer is passed to the channel's `write` method.

One difference is that an `OpenOption` must be provided to the `newByteChannel` method, specifically `StandardOpenOption.WRITE` as well as `StandardOpenOption.CREATE` in order to create the file if it didn't already exist.

As before, the buffer should be `rewind` after writing to it so that the position is at the beginning when writing it to the channel. Alternatively, the `flip` method could also be called in this case.

Also note that the writing data to the file in this way overwrites existing data, and doesn't outright replace the entire file, since the `StandardOpenOption.TRUNCATE_EXISTING` option is not being used.

``` java
Path file = Paths.get("file.txt");
SeekableByteChannel channel =
    Files.newByteChannel(file,
                         StandardOpenOption.WRITE,
                         StandardOpenOption.CREATE);

ByteBuffer buffer = ByteBuffer.allocate(128);

// write a 'C' to the buffer
buffer.put((byte)'C');

// reset position of buffer for writing
buffer.rewind();

channel.write(buffer);
```

### Writing to a Memory Map

It's also possible to memory map a file for writing purposes in the same way as was [previously covered](#reading-from-a-memory-map), except that the `MapMode.READ_WRITE` option needs to be used.

``` java
Path file = Paths.get("file.txt");
SeekableByteChannel channel =
    Files.newByteChannel(file,
                         StandardOpenOption.READ,
                         StandardOpenOption.WRITE,
                         StandardOpenOption.CREATE);

String contents = "this is a test";
byte[] data = contents.getBytes();

MappedByteBuffer fileMap = channel.map(FileChannel.MapMode.READ_WRITE, 0, data.length);

fileMap.put(data);
```

## Stream-based I/O

`Files` provides static methods `newInputStream` and `newOutputStream` for obtaining streams connected to a file specified by a `Path`. Since it's a regular stream, it can be wrapped in other streams such as the `BufferedInputStream`.

``` java
// writing
OutputStream stream = Files.newOutputStream(Paths.get("file.txt"));

stream.write((byte)'C');

// reading
InputStream stream = Files.newInputStream(Paths.get("file.txt"));
int b;

b = stream.read();
if (b != -1) System.out.print((char)b)
```

# Networking

The `InetAddress` class represents a numerical IPv4/IPv6 address or domain name. It has no constructors, only factory methods, including `getLocalHost`, `getByName`, and `getAllByName` which resolves a host name, and `getByAddress` which takes an IPv4 or IPv6 address.

The `URL` class represents a Uniform Resource Locator and provides methods for accessing information about the URL, as well as an `openConnection` method that opens a connection to the URL and returns a `URLConnection` object to represent the connection.

The `URLConnection` can be used to obtain information about a resource pointed to by a `URL`, such as `getContentLength` and `getHeaderFields`. The `getInputStream` method returns an `InputStream` that can be used to obtain the resource pointed to by the URL.

The `HttpURLConnection` class extends `URLConnection` and is specifically for HTTP connections and can it can be obtained by casting the result of `URL`'s `openConnection`. This class provides additional methods such as `getRequestMethod` and `setRequestMethod`.

The `URI` class represents a Uniform Resource Locator which is a more general form of a URL, which also describes how to access the resource.

The TCP `ServerSocket` class represents a listener socket, whereas `Socket` is a general socket that can be used by clients. The input and output streams of a `Socket` can be accessed with `getInputStream` and `getOutputStream`.

There also UDP sockets available via `DatagramSocket` which creates a local UDP socket. It has methods `send` and `receive` which send and receive a `DatagramPacket`. A `DatagramPacket` is constructed from an existing `byte[]` and optionally a target `InetAddress` and port.

# Concurrency

## Synchronization

Synchronizers are used for synchronizing interactions between threads.

### Semaphore

Semaphores control a shared resource using a counter, so that access is allowed if the counter is greater than zero, but disallowed if it's zero, in which case it blocks until it's no longer zero. It's useful to think of a semaphore as representing a fixed number of permits for accessing the resource.

A `Semaphore` can be constructed by specifying the resource count to give it. An optional `boolean` parameter may be specified to indicate that threads should be given access to the resource in the order that they requested it.

The `acquire` method is used for actually attempting to acquire the resource, with an optional count argument specifying how many resources to request. Conversely, the `release` method does the same in reverse, relinquishing the resource.

### CountDownLatch

The `CountDownLatch` class can be used for waiting until a number of events have occurred. It's constructed by specifying the number of events that should be waited on.

The `await` method is called to wait on the latch until all events have occurred, with an overload accepting a time out and returning `false` if the time out was triggered.

The `countDown` method actually decrements the count associated with the latch.

### CyclicBarrier

The `CyclicBarrier` class represents a traditional barrier which enforces that all participating threads must reach the barrier before they're allowed to continue execution past it. It's constructed by specifying the number of participating threads, with an overload accepting an arbitrary `Runnable` to execute after the last thread reaches the barrier but before they all resume execution.

Threads signal that they have reached the barrier by calling `await` on the barrier---which blocks until all other threads reach the barrier---with an overload accepting a time out. The `await` method returns an integer count of the number of _other_ participating threads aside from the current one.

### Exchanger

The `Exchanger` class is used for exchanged data between two threads. It waits until both communicating threads call `exchange` with the data to be sent as an argument, and then exchanges the data by returning it from the method call on the receiving thread. An overload of `exchange` accepts a time out.

``` java
Exchanger<String> exchanger;

// thread 1
String receivedFrom2 = exchanger.exchange("send to 2");

// thread 2
String receivedFrom1 = exchanger.exchange("send to 1");

receivedFrom2 == "send to 1"
receivedFrom1 == "send to 2"
```

### Phaser

The `Phaser` class can be used for synchronizing threads that represent various phases of a process. It's similar to `CyclicBarrier` except that it supports multiple phases. It's constructed by optionally providing the number of participating parties. Parties can register themselves for the next phase by calling the `register` method. A party signals that it has completed a phase by calling `arrive` or `arriveAndAwaitAdvance`.

The `arrive` method returns the current phase number or a negative number if the phaser was terminated, but **does not** block execution, whereas `arriveAndAwaitAdvance` does and returns the _next_ phase number.

The `arriveAndDeregister` method signals arrival and deregisters itself without waiting for the phase to complete.

The current phase number can also be retrieved using `getPhase`.

The `Phaser` class can be extended and the `onAdvance` method overridden to hook into the point between phases. It takes the current phase number and the number of parties and returns whether the phaser should be terminated as a `boolean`. This is useful for capping the number of phases that should be allowed.

It's also possible to construct trees of phasers using a constructor overload that takes a parent phaser.

## Executors

An executor initiates and controls the execution of threads. The `Executor` interface defines an `execute` method that takes a `Runnable` which it then executes. The `ExecutorService` interface extends `Executor` and adds methods to control the execution of threads, such as `shutdown`, as well as methods that run threads which return results. The `ScheduledExecutorService` further extends `ExecutorService` to add scheduling capabilities.

The `ThreadPoolExecutor` class implements `ExecutorService` and provides a pool of threads for running `Runnable`s.

The `ScheduledThreadPoolExecutor` class implements `ScheduledExecutorService` and provides a scheduled thread pool.

The `ForkJoinPool` class implements `ExecutorService` and is used by the Fork/Join Framework.

Thread pools are typically created via static methods on the `Executors` utility class. The `newCachedThreadPool` method returns a thread pool that adds threads if needed but reuses threads when possible. The `newFixedThreadPool` method creates a fixed-size thread pool, where `newScheduledThreadPool` does the same but supports scheduling.

## Callable

The `Callable` interface represents a thread that returns a value to the invoking thread. It's a generic interface parameterized on the return value type and defines a single method `call` which returns the value.

A `Callable` is executed by an `ExecutorService`'s `submit` method, which returns an object of type `Future`.

## Future

The `Future` interface represents a value returned by a `Callable` at some future time. It is also a generic interface parameterized on the return value type. The `get` method is used for actually accessing the value, blocking until it becomes available if it isn't already, with one of the overloads accepting a time out.

``` java
class ProvideFive implements Callable<Integer> {
  public Integer call() {
    return 5;
  }
}

ExecutorService executor = Executors.newFixedThreadPool(2);
Future<Integer> five = executor.submit(new ProvideFive());

int fiveInteger = five.get();
```

## TimeUnit

Various methods in the concurrency API accept optional time outs which are generally provided in the form of a `long` parameter specifying _how many_ and a `TimeUnit` enumeration value specifying the time unit. However, there is **no guarantee** that the system is capable of any of these granularity levels. The possible time units are:

* `DAYS`
* `HOURS`
* `MINUTES`
* `SECONDS`
* `MICROSECONDS`
* `MILLISECONDS`
* `NANOSECONDS`

The `TimeUnit` enumeration also provides methods for converting between units, such as the `convert` method that takes a source quantity and `TimeUnit` and converts it to the invoking enumeration, for example:

``` java
1 == TimeUnit.HOURS.convert(60, TimeUnit.MINUTES)
```

There are also specific methods for converting the invoking enumeration into a specific `TimeUnit`, such as `toDays`.

The `sleep` method pauses execution for a given delay in the `TimeUnit` of the invoking enumeration.

The `timedJoin` method paused the given thread for the given delay in the `TimeUnit` of the invoking enumeration, whereas `timedWait` waits for the given thread up to a given time out in the `TimeUnit` of the invoking enumeration.

## Concurrent Collections

There are a variety of concurrent collections. Most are equivalent to the regular collections framework classes aside from the fact that they provide concurrency support.

```
ArrayBlockingQueue
Concurrent{
  HashMap,
  Linked{Deque, Queue},
  SkipList{Map, Set},
}
CopyOnWriteArray{List, Set}
DelayQueue
LinkedBlocking{Deque, Queue}
PriorityBlockingQueue
SynchronousQueue
```

## Locks

The `java.util.concurrent.locks` package provides actual locks via the `Lock` interface which represents acquiring and releasing a resource via methods `lock`, `tryLock`, and `unlock`. The `lock` method waits until the lock is released by other threads, whereas the `tryLock` method tries to acquire the lock without waiting, returning a boolean indicating whether or not the lock was acquired, with an overload accepting a time out.

The `ReentrantLock` class implements a reentrant, or recursive, lock which can be acquired by the same thread more than once and which must be released the same number of times in order to fully release the lock.

There is also a `ReadWriteLock` class that keeps separate locks for read and write access, enabling multiple readers to exist whenever there aren't any writers.

### Condition Variables

The `newCondition` method on `Lock` returns a `Condition` object representing a condition variable which can be waited on using `await` and signaled using `signal` or `signalAll`.

Condition variables are useful for representing a lock that waits for a condition to change. Acquiring the lock and then using a busy loop to check if the condition is true wouldn't work since the lock couldn't be acquired by another thread to make the condition true. Alternatively, looping and acquiring the lock, checking if it's true, and if not sleeping for some time before repeating would work but it would be difficult to determine the best amount of time to sleep.

A condition variable supports a wait and notify operation. Waiting entails the following operations:

1. atomically:
    1. release associated lock
    2. move thread to condition variable's wait queue
    3. sleep thread
2. when notified: re-acquire lock
3. return

In the aforementioned scenario, the lock is acquired to check if the condition is now true, and if not, a condition variable associated with the lock is waited on using `await`, which releases the lock and puts the thread to sleep until the condition variable is signaled. Another thread might then acquire the lock in order to change the condition to true, then it signals all waiting threads using `signal` or `signalAll` and releases the lock.

A call to a condition variable's `await` is generally placed inside a loop that checks the actual condition, so that upon wake up the thread first checks to ensure that the condition didn't change since the point at which it was notified/woken up and when it actually resumed execution, which is known as a _spurious wakeup_.

## Atomic Operations

The `java.util.concurrent.atomic` package provides atomic primitive data types such as `AtomicInteger` which have methods such as `compareAndSet`.

JDK 8 also introduces four classes for lock-free cumulative operations: `DoubleAccumulator`, `DoubleAdder`, `LongAccumulator`, and `LongAdder`.

## Fork/Join Framework

The Fork/Join framework simplifies the creation and use of threads while automatically utilizing multiple processors.

The `ForkJoinTask<V>` abstract class represents a task managed by a `ForkJoinPool`. Whereas `Thread` represents a thread of execution, `ForkJoinTask` is a lightweight abstraction of a task which is executed by threads managed by a thread pool in `ForkJoinPool`.

The two primary methods provided by `ForkJoinTask` are `fork` and `join`. The `fork` method submits the invoking task for asynchronous execution. The `join` method waits until the invoking task finishes and returns its value. The `invoke` method combines `fork` and `join` into a single call. The `invokeAll` method can take an arbitrary amount of `ForkJoinTask`s.

The `RecursiveAction` abstract class represents a task that doesn't return a result. It can be extended and the `compute` method overridden to define the task's _computational_ portion.

The `RecursiveTask<V>` abstract class represents a task that returns a result. It also defines an abstract `compute` method that should be overridden to define the task's computational portion.

The `ForkJoinPool` class manages the execution of `ForkJoinTask`s. JDK 8 provides two ways to acquire a pool: creating one using the `ForkJoinPool` constructor or use what the common pool, which is a static `ForkJoinPool` that is globally available. The default constructor automatically scales to the number of processors in the system, whereas another one allows explicitly setting the size.

A reference to the common pool can be obtained with the `commonPool` static method on `ForkJoinPool`. It has the default level of parallelism which scales to the amount of execution units on the system.

The `invoke` method is used to execute a task on the pool, returning the value that it returns. The `execute` method is used for asynchronously submitting a task for execution.

When the `invoke` or `fork` methods are called on a task from outside its computational portion, the common pool is automatically used to perform the operation.

`ForkJoinPool` uses a work-stealing queue to manage execution of its threads. Each thread maintains a queue of tasks, and if a thread's queue is empty, it steals a task from another thread's queue.

`ForkJoinPool` uses daemon threads, which automatically terminate when all user threads are terminated, so there is no need to explicitly shut down the pool, though it can be done explicitly with the `shutdown` method.

The `cancel` method on `ForkJoinTask` can be used to cancel a running task, and returns whether or not the task was successfully canceled.

The `reinitialize` method on `ForkJoinTask` reinitializes the state of the task so taht it can be re-run.

A `Runnable` or `Callable` can be converted into a `ForkJoinTask` by calling the `adapt` method on `ForkJoinTask`.

`ForkJoinTask` objects should generally not use synchronized methods, blocks, or other primitives, though `Phaser` is compatible. It's also preferable to avoid blocking or I/O in general.

# Stream API

Streams are essentially conduits for data, sourced for example by arrays or collections. The `BaseStream` interface provides the basic functionality available in all streams. It is generic on the type `T` of elements in the stream as well as the type of the stream `S` that implements `BaseStream`.

``` java
interface BaseStream<T, S extends BaseStream<T, S>>
```

An `onClose` method returns a stream that runs the provided `Runnable` when the stream is closed. The `parallel` method returns a parallel stream based on the invoking stream, whereas the `sequential` method returns a sequential stream based on the invoking stream. The `spliterator` method returns a spliterator to the stream, while `iterator` returns a regular iterator.

The `Stream` interface derives from `BaseStream`. The `count` method returns the number of elements in the stream.

The `filter` method takes a `Predicate` and returns a stream that only produces the elements that satisfy the predicate. The `forEach` method takes a `Consumer` which it applies to each element in the stream. The `map` function takes a `Function` and transforms each element in the stream with it.

``` java
stream.filter((n) -> (n % 2) == 0)
      .forEach((n) -> System.out.printf("%d is an even number", n));
```

Streams are lazy until a terminal operation is performed, such as `collect`.

Streams can be obtained through a variety of ways, such as from a collection by using the `stream` method or a parallel stream with `parallelStream`. A stream can be obtained from an array by using the `Arrays.stream` method.

The `reduce` method takes a `BinaryOperator` to reduce a stream into a single value.

``` java
int sum = numbers.stream().reduce(0, Integer::sum);
```

The `collect` method accepts a `Collector` which collects elements into a collection which the `collect` method then returns. The `Collector` interface is parameterized by the type `T` of the element in the stream, the internal accumulated type `A`, and the result type `R`.

The `Collectors` class provides static methods for obtaining `Collector` objects for lists and sets via `Collectors.toList` and `Collectors.toSet`.

Another overload of `collect` takes a `Supplier` for constructing the target collection type, a `BiConsumer` for adding an element to the collection, and a `BiConsumer` for combining two partial results.

``` java
LinkedList<Integer> list =
  numbers.collect(
    () -> new LinkedList<>(),
    (list, element) -> list.add(element),
    (listA, listB) -> listA.addAll(listB))
```

Note that the above could be simplified by passing method or constructor references:

``` java
LinkedList<Integer> list =
  numbers.collect(
    LinkedList::new,
    LinkedList::add,
    LinkedList::addAll)
```

## Parallel Streams

A parallel stream can be obtained using the `parallelStream` method on supported types such as those that implement `Collection`, and one can also be created from a regular sequential stream by using the `parallel` method on a stream type.

Operations on parallel streams must be stateless, non-interfering (not modify the data source), and associative.

Parallel streams can leverage an overload of `reduce` that accepts a combiner `BinaryOperator` that specifies how partial results from parallel computations are to be combined.

In the example below, partial results would be the weights, in which case they are combined by simply adding them. However, if no separate combining function was provided, the accumulator function would be used, which would in effect add one weight to a weight of another weight.

``` java
int weightsSum =
  numbers.stream()
         .reduce(0, (sum, b) -> sum + b.getWeight(), Integer::sum);
```

A parallel stream can be switched back to a sequential stream with the `sequential` method.

It's possible to optimize a parallel stream by allowing it to be unordered by using the `unordered` method to yield an unordered stream, instead of forcing it to preserve the original order.

The `forEach` method may not preserve order on a parallel stream even if the stream is not unordered, for that there is `forEachOrdered`.

# Regular Expressions

A `Pattern` is constructed using the `compile` static method. The `Pattern` can then be used to match against a sequence by obtaining a `Matcher` built from the `Pattern` via the `matcher` method on `Pattern` which takes a `CharSequence` of the string to match on.

The `matches` method on `Matcher` returns a boolean indicating whether the input matches the pattern, whereas `find` checks if any subsequence of the input matches the pattern.

A string containing the last matching sequence can be obtained using `group`, and the index within the input of the beginning and end of the match can be obtained using `start` and `end`. This means that `find` can be used to find the next match, then calling these methods will yield the information about them.

The `replaceAll` method takes a `String` to replace all matches with in the input string, then returns the replaced string.

``` java
Pattern pattern = Pattern.compile("java");
Matcher matcher = pattern.matcher("cpp java go");

true == matcher.find();
```

There's also a `split` method that takes an input stream and splits it into a return `String[]`.

There's also a convenience static method `matches` on `Pattern` that takes a a string pattern and an input string to attempt to match on, and returns whether there was a match. There's also a `matches` method on `String` which takes a string pattern.

# javadoc

Documentation comments are possible using the `/** */` delimiters, which are processed by the `javadoc` program to produce documentation.

| Tag | Meaning |
|:----|:--------|
| `@author` | code author |
| `{@code}` | codeblock |
| `@deprecated` | deprecation marker |
| `@{docRoot}` | specify root path of current docs |
| `@exception` | specify exception thrown by function |
| `{@inheritDoc}` | inherit comment from superclass |
| `{@link}` | link to another topic |
| `{@linkplain}` | link to another topic in plain font |
| `{@literal}` | span code |
| `@param` | document parameter |
| `@return` | document return value |
| `@see` | refer to another topic via link |
| `@serial` | document serializable field |
| `@serialData` | document data written by `writeObject` or `writeExternal` |
| `@serialField` | document `ObjectStreamField` component |
| `@since` | specify release of introduction or change |
| `@throws` | same as `@exception` |
| `{@value}` | display value of static field |
| `@version` | specify version of class |

The `@exception` tag takes two parameters: the exception name and the reason for why it is thrown.

The `@link` tag takes two parameters: the link and the text to use for the link. The `@linkPlain` and `@see` tags are the same.

``` java
{@link pkg.class#member text}
```

The `@param` tag is like `@exception`.

The `@return` tag only requires an explanation, which should include the return type.

Files generally begin with a header documentation comment, and documentation markers follow throughout.

``` java
/**
 * This class is for things.
 * @author Jorge Israel Peña
 * @version 1.0
*/

public class Thing {
  /**
  * This does a thing.
  * @param num The value passed to the method.
  * @exception IOException Just because.
  * @return num The same num.
  * @see AbstractSingletonProxyFactoryBean
  */
  public double method(double num) throws IOException {
    return num;
  }
}
```
