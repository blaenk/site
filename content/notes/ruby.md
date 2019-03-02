+++
title = "Ruby"
date = 2017-04-18

[note]
kind = "language"
+++

Ruby is a dynamic interpreted language. It gained widespread adoption for web development following the release of [Ruby on Rails](/notes/ruby-on-rails/). Early on its dynamic metaprogramming facilities were abused, as was evident in Ruby on Rails.

<nav id="toc"></nav>

The `rbconfig` package contains constants with information about the local Ruby installation, particularly in the `RbConfig::CONFIG` hash.

| Term        | Contents                                  |
|:------------| :-----------------------------------------|
| rubylibdir  | standard library                          |
| bindir      | CLI tools                                 |
| archdir     | arcitecture-specific extensions/libraries |
| sitedir     | own/third-party extensions/libraries      |
| vendordir   | third-party extensions/libraries          |
| sitelibdir  | own extensions (Ruby)                     |
| sitearchdir | own extensions \(C)                       |

# Loading Files

Ruby's load path is accessible through the global variable `$:`.

The `load` method looks in the file system for its argument, given a relative or absolute path, or in the Ruby load path. The `load` method loads the file even if it has already been previously loaded, with subsequent loads overwriting existing definitions.

The `require` method does _not_ re-load the same file multiple times. Unlike `load`, the `require` method doesn't treat the current directory as being part of the load path.

``` ruby
# This doesn't work.
require 'some_file.rb'

# This does.
require './some_file.rb'

# Or add current directory to load path.
$: << '.'
require 'some_file.rb'
```

There's also a `require_relative` method which assumes a path relative to the file from which it is called.

A specific Ruby gem can be required by using the `gem` method which takes a version as the second argument.

# Ruby Index

The `ri` command can be used to lookup documentation in the terminal using the method description convention, i.e. `String#upcase`, whereas a class method would use `::`.

# Naming Conventions

The interpreter considers a bareword identifier to potentially be a local variable, keyword, or method call:

1. if it's a keyword, treat it as such
2. if an equal sign `=` follows, it's local variable being assigned too
3. assume a method call

By convention, methods are referred to by class name and method name separated by a hash `#`, while class methods are separated by a double colon `::`.

## Variable Naming

The Ruby variable naming convention is to use snake case as opposed to camel case.

Local variables start with lowercase characters or an underscore and cannot start with an uppercase character.

Instance variables follow local variable conventions but start with an at-sign `@`.

Class variables follow instance variable conventions but start with _two_ at-signs `@@`.

Global variables start with a dollar sign `$` and can consist of characters such as `:`, `/`, etc., unlike regular variables.

Constants begin with an uppercase letter.

## Method Naming

Methods follow local variable conventions but they can optionally end with characters `?`, `!`, and `=`.

Methods that end in equal signs `=` can be called without the parentheses:

``` ruby
def obj.name=(value)
  @name = value
end

obj.name = "yes"
```

Note that this is purely cosmetic/syntax sugar. Evaluation semantics are still those of assignment, i.e. the expression evaluates to the right-hand side, not whatever the method evaluates to.

When a method returns multiple values, they are automatically wrapped up in an array.

``` ruby
def method(a, b, c)
  return a, b, c
end

assert(method(1, 2, 3) == [1, 2, 3])
```

Variable method arguments can be collected into an array specified by a parameter preceded by an asterisk `*`:

``` ruby
def variable_args(first, *rest)
  # `rest` is an array
end
```

Variable arguments are sponged up into that parameter as needed to make the method call valid for all other parameters.

It's possible to give arguments default values:

``` ruby
def default_args(first, second=2)
  first + second
end

default_args(1) #=> 3
default_args(1, 3) #=> 4
```

Variable arguments can't be placed to the left of default arguments, as it would be ambiguous whether or not a default argument should take on one of the arguments.

# Truthiness

The only false objects are the `nil` object and `false`. Everything else, including zero and empty strings, are truthy.

# Objects

Aside from generic objects of class `Object` there are even more basic objects of class `BasicObject`.

Every object has a unique identifier which can be obtained via `Object#object_id`.

The `Object#respond_to?` method can be used to determine if an object responds to a message.

The `Object#send` method can be used to send a message to an object. It's aliased to `__send__` to reduce the chance of naming collisions. The `send` method can also send messages that invoke private methods, whereas the `Object#public_send` method only invokes publicly accessible methods.

## Primitives

Most objects are referred to via references except for the following primitives:

* integers
* symbols
* `true`
* `false`
* `nil`

Note that symbols may contain spaces if they're surrounded with double quotes and preceded by a colon: `:"with spaces"`.

## References

Objects can be duplicated using the `dup` method. An object can be frozen with the `freeze` method, which prevents it from mutating. The `clone` method is like `dup` in that it duplicates the object, but it also keeps the object frozen if it already was frozen. Freezing is shallow, such that for example, object elements of an array can still be mutated even if the array itself can't.

## Comparison

Object comparison through `==` by default only compares pointers. By convention, `==` may be redefined by `equal?` is left alone to allow comparing pointers.

It's easy to enable object comparison by mixing in the `Comparable` module and defining the `<=>` (spaceship) method. All other comparison methods are defined in terms of it.
# Classes

Classes are named with constants, which begin with an uppercase letter.

Methods defined for a particular instance/object are _singleton methods_.

``` ruby
str = "this is a test"

def str.say_it
  puts self
end

str.say_it #=> "this is a test"
```

Singleton methods defined on a class object are called _class methods_. Class methods are accessible through the class object, but not through individual instances.

Classes can be reopened to make additions or changes:

``` ruby
class Test
  # some code
end

# later on
class Test
  def new_method
    puts "this is a new method"
  end
end
```

Methods named `initialize` are constructors which are invoked via the `new` class method.

Attributes are not a language construct, but rather they are a high level concept referring to a configuration of methods and instance variables.

The `attr_reader` class method creates getter for an instance variable of the same name. The `attr_writer` class method creates a setter for an instance variable of the same name. The `attr_accessor` class method creates both a getter and a setter. There is also an `attr` class method that creates a reader and takes a second argument denoting whether to also create writer.

Shortcut (compound) assignment operators are expanded into the non-compound equivalent, so that `+=` can be used on any object that has a `+` method defined, since `x += y` is expanded into `x = x.+(y)`.

The `private` method takes a list of methods to make private, or if none are provided, acts as a switch so that all subsequent methods are treated as private unless `public` or `protected` is called.

A private method is one that can't be called _with_ an explicit receiver. Whether a private method can be called or not is determined by the object that is `self` at the time it is called. It means that an instance of that class can send the message to itself, which is enforced by Ruby by forbidding an explicit receiver on that method, such that the only time that the method can be called without a receiver is when a private method can be called.

However, setter methods cannot be called without omitting the receiver, so it would not be possible to call a private method, but Ruby makes an exception for private setter methods, allowing the receiver to be specified as long as it is `self`.

Top-level methods are stored as private instance methods of the `Object` class. Since they're private methods, they must be called without an explicit receiver, and since they're methods of `Object`, they can be called from anywhere because `Object` lies in the method lookup path of every class except for `BasicObject`.

``` ruby
def talk
  puts "hello"
end

# Equivalent
class Object
  private

  def talk
    puts "hello"
  end
end
```

A protected method can be called as long as `self` is an instance of the same class or an ancestor or descendant.

Subclasses inherit the method-access rules of their superclasses, but they can set up new rules which take precedence.

# Inheritance

The syntax for inheritance uses a less-than sign `<`:

``` ruby
class Parent
end

class Child < Parent
end
```

# Object Hierarchy

`Object` is a generic object class which contains many introspection methods. The `BasicObject` class is even more bare-bones, and the `Object` class derives from it.

The `Kernel` module is mixed into `Object` and defines most of Ruby's fundamental methods.

Class objects are the only object with the power to spawn new instances. Every class, such as `Object`, is an instance of the `Class` class.

This can be illustrated explicitly by instantiating a class object via `Class.new` and using that class object to create instances of that class:

``` ruby
class MyClass
end

instance = MyClass.new

# equivalent to

my_class = Class.new
instance_of_my_class = my_class.new
```

Instance methods can be defined for `Class.new`-created classes by providing a code block to the constructor.

``` ruby
my_class = Class.new do
  def hello
    puts "Hello!"
  end
end
```

Note the curiosity that `Class` is itself an instance of `Class`, and `Object` is a class and `Class` is an object. Basically, every object has an internal record of what class it's an instance of, and that of `Class` points to itself. Simply remember that classes are objects/instances of the `Class` class.

Class methods can be invoked by using the dot operator on the class name or by using it without an explicit receiver at the top-level of a class definition, where `self` is implicitly the class object.

Class objects have their own method, state, and identity, and don't share any of these things with instances of itself.

# Value of self

When no explicit receiver is named, `self` is implied. In a class definition body, `self` is the class object itself.

If a method and local variable are given the same name, and the bare identifier (sans `self`) is used, the variable takes precedence, but the method can be forced by explicitly specifying `self` or explicitly specifying an argument list. Note, however, that when the method name ends with an equal sign `=`, the `self` can't be omitted, because Ruby always interprets `ident = val` as an assignment to a local variable.

In the top-level, `self` is `main`, a built-in top-level default object. `main` cannot be referred to directly within a program, since Ruby interprets it as a regular variable or method name. It can be accessed by assigning `self` to a variable at the top-level.

In a class or module definition, `self` is the class or module object.

In top-level method definitions, `self` is whatever object is `self` when the method is called. Top-level methods are available as private methods to all objects.

In instance method definitions in a class, `self` is the instance responding to the method.

In instance method definitions in a module, `self` is an individual object extended by the module or an instance of the class that mixes in the module.

In singleton method definitions, `self` is the object on which the singleton method is defined.

Instance variables belong to whatever object is the current object `self` at that point in the program.

# Variables

Constants declared inside of a class can be referred to from within instance or class methods, as well as externally through a double colon `::`, such as `Math::PI`.

In fact, constants have a kind of global visibility or reachability, in that as long as the path to the constant is known, it can be accessed.

Constants are identified relative to the point of execution. If the constant being referred to is ambiguous or shadowed by a local constant, the correct one can specified by using the absolute path to the constant, which begins with a double colon `::`.

It's possible to reassign to constants, except that doing so emits a warning when running with the `-w` command-line argument. This is permitted in part because program files can be reloaded, so then many file loading operations would fail if they included constant assignments and this weren't permitted.

Class variables are shared between a class and instances of that class, without being visible to other objects. More specifically, class variables are class-hierarchy-scoped variables, meaning that class variables are shared between the class, its instances, and its subclasses and their instances.

An alternative to class variables is to create an instance variable belonging to the class object and wrapping it in accessor methods. This has the added benefit that subclass objects get their own version of the pseudo-class variable.

``` ruby
class Test
  def self.total_count
    @total_count ||= 0
  end

  def self.total_count=(n)
    @total_count = n
  end

  def some_method
    self.class.total_count += 1
  end
end

# Accessible through: Test.total_count
```

# Modules

Modules aren't instantiated, but they can be _mixed in_ to classes or objects to add to them their functionality, using the `include` or `prepend` method, which causes instances of that class to have access to instance methods defined in the module.

``` ruby
module MyModule
  def hello
    puts "Hello"
  end
end

class MyClass
  include MyModule
end

c = MyClass.new
c.hello
```

The difference between inheriting from a class and mixing in a module is that more than one module can be mixed in. Multiple behaviors can be defined in separate modules and mixed in at will. Following this, most class names are nouns, whereas modules tend to be adjectives.

The difference between `include` and `prepend` is that `prepend` causes the object to look in that module before it looks in the class.

The `Class` class is a subclass of the `Module` class, meaning that every class object is also a module object. This also means that modules are the more basic structure, and classes are a specialization.

Modules are sometimes used to introduce a new namespace. Classes defined within a module are accessed with the double-colon `::` constant lookup token syntax, since classes are constants after all.

# Method Lookup

Objects follow a lookup hierarchy to find methods, starting with the class, the inheritance hierarchy, and finally any singleton methods.

Modules are searched in reverse order of inclusion, i.e. the most recently mixed-in module is searched first. Re-including a module doesn't do anything, and so doesn't affect the inclusion order.

Classes can call instance methods of `Class`, and `new` is one such method. Since `Class` derives from the class `Module`, class objects have access to `Module`'s instance methods, which includes the `attr_accessor` family of methods.

If an object's method-lookup path includes more than one method with the same name, the first one to be encountered is the one that is executed.

In summary, object method lookup searches the following:

1. prepended modules, in reverse order of prepending
2. its class
3. included modules, in reverse order of inclusion
4. modules prepended to the superclass
5. its class' superclass
6. modules included in the superclass
7. repeat 4-6 up to `Object` and `BasicObject`

The `super` keyword can be used inside the body of a method definition to jump to the next-highest definition in the lookup path. When called without arguments, it forwards the arguments that were passed to the method from which it is called. When called with an explicit empty argument list, it sends no arguments to the higher-up method even if some were passed to the current method. When called with explicit, specific arguments, it sends those arguments to the higher-up method.

When a method isn't found, a method named `method_missing` defined in the `Kernel` module is invoked on the object, which can be overridden to define custom behavior. The first argument is the name of the missing method as a symbol, and the rest of the arguments are the arguments that were passed to the method.

Singleton methods reside in an object's singleton class.

# Control Structures

Code blocks can be written in curly braces `{}` or enclosed with the keywords `do` and `end`.

The `next` keyword skips the current iteration of a loop, similar to `continue` in other languages.

## Conditionals

`if` and `case` expressions evaluate to their chosen branch, and if they don't succeed anywhere then they return `nil`.

An `if` clause can be used on a single line in the form `if … then … end`

Conditional modifiers are placed after a statement.

``` ruby
puts "Big number" if x > 100
```

The allocation of new variables happens when the parser sees assignment to a new variable, even if it's within a conditional block. This means that a variable may be brought into existence even if the conditional block that assigns a value to it is never executed.

``` ruby
if false
  x = 1
end

p x #=> nil
p y #=> Fatal error: y is unknown
```

A `case` statement's `when` clause works by delegating to the argument's case equality method `===` ("threequal" operator), so that `case a when b` is expanded to `a.===(b)`.

A `case` statement's `when` clause can have more than one match separated by commas `,` that act like a Boolean OR operator.

``` ruby
case answer
when "y", "yes"
  puts "Confirmed"
  exit
end
```

A `case` statement can omit an argument expression in order to behave similar to an `if` statement.

``` ruby
case
when a == b
  …
when c == d
  …
end
```

## Loops

`until` is to `while` as `unless` is to `if`.

There are `while` and `until` modifiers, but they don't behave as post-positioned (do-while) loops.

``` ruby
n = n + 1 until n == 10

# Won't execute, because `until` is treated as if at the beginning.
a += 1 until true
```

# Iterators

An iterator is a method that expects a code block, which it can then execute through the `yield` keyword. A code block isn't an argument, it's part of the method's syntax. The code block can take parameters and the `yield` keyword can pass arguments to it. The code block can return a value back to the yielding function, which is set as the result of the `yield` statement.

Blocks have direct access to existing variables, but block parameters shadow existing ones. To ensure that a variable is local to the block, to prevent it from clobbering any existing variables, the block parameter list supports a semicolon-delimited syntax to specify any block-local variable names, known as reserved names.

``` ruby
x = "original"

3.times do |i; x|
  # Doesn't clobber the outer-scope x
  x = i
end
```

`loop` is an iterator that performs an unconditional loop of its code block.

``` ruby
def my_loop
  while true
    yield
  end
end

my_loop { puts "iteration" }
```

# Exceptions

An exception is an instance of `Exception`. Raising an exception causes the program to proceed to a `rescue` clause or terminates if there is none.

Rescuing an exception takes the form `begin … rescue … end`. The `rescue` clause can take an exception class to match on. A method or code block is implicitly surrounded by `begin` and `end`, so `rescue` may be used anywhere, although the rescue scope may be narrowed with an explicit `begin`-`end` block.

``` ruby
begin
  result = 100 / n
rescue ZeroDivisionError
  puts "Divided by zero"
# Catch-all
rescue
  puts "Something went wrong"
end
```

Raising an exception is done with the `raise` keyword and the name of the exception to raise, and an optional second argument for a message string. If only a message string is provided, then Ruby assumes a `RuntimeError`.

``` ruby
def f(x)
  raise ArgumentError, "Need number under 10" unless x < 10
end
```

The `rescue` clause can capture the raised exception object to a variable using the `=>` operator. Exception objects respond to `backtrace`, `message`, and contain other useful information. The `rescue` clause can optionally re-raise the exception that was rescued, which can be accomplished with `raise` without any arguments, in which case the rescued exception is implied.

``` ruby
begin f(20)
rescue ArgumentError => e
  puts e.backtrace
end
```

The `ensure` clause can be used to specify code that executes unconditionally after the block body and any rescue clauses. Note that the following example is contrived, since realistically this same functionality would be accomplished through code blocks.

``` ruby
fh = File.open(path)

begin
  line = fh.gets
rescue SomeError
  raise
ensure
  # Close the handle no matter what
  fh.close
end
```

Custom exception classes can be created by inheriting from `Exception` or a descendant of it.

# Built-Ins

Unary operators `+` and `-` can be implemented for a class by defining methods `+@` and `-@` respectively.

Defining the `!` method provides both unary `!` and keyword `not`.

Conventionally, methods ending in `!` are considered "dangerous" relative to the non-bang method, meaning for example that they permanently modify the receiver. This notation should only be used when there's an equivalent non-bang method.

There are built-in conversion methods beginning with `to_` such as:

* `to_s`: to string
* `to_sym`: to symbol
* `to_a`: to array
* `to_i`: to integer
* `to_f`: to float

When an object is interpolated into a string, its `to_s` method is called.

The `to_a` method provides an array-like representation of an object.

A bare list is a series of identifiers or literal objects separated by commas. It's only valid in certain contexts such as array brackets, such that an array is constructed from the bare list. The star operator `*` more or less unwraps the brackets from the bare list allowing it to be embedded in, for example, another pair of brackets.

``` ruby
array = [1, 2, 3]

assert([*array] == array)

def f(a, b)
end

args = [1, 2]

assert(f(*args) == f(1, 2))
```

Converting strings with no reasonable integer equivalent to integers with `to_i` always results in `0`. If the string begins with digits, the following nondigits are ignored. The `to_f` method is similar. The `Integer` and `Float` methods are stricter versions of the conversion functions.

The `to_str` function should be defined and used when an object needs to become a string, usually because they are string-like, compared to `to_s` which simply provides a string _representation_ of the object. For example, `String#+` uses `to_str` for converting the object to a string for the purpose of string concatenation, if it's defined.

Similarly, objects can act as arrays if they define `to_ary`, to facilitate operations where the object must behave like an array, such as in array concatenation.
