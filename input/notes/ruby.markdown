---
title = "Ruby"
published = "April 18, 2017"
excerpt = "The Ruby Programming Language"
comments = false
---

<toc />

The `rbconfig` package contains constants with information about the local Ruby installation, particularly in the `RbConfig::CONFIG` hash.

| Term        | Contents                                  |
|-------------+-------------------------------------------|
| rubylibdir  | standard library                          |
| bindir      | CLI tools                                 |
| archdir     | arcitecture-specific extensions/libraries |
| sitedir     | own/third-party extensions/libraries      |
| vendordir   | third-party extensions/libraries          |
| sitelibdir  | own extensions (Ruby)                     |
| sitearchdir | own extensions (C)                        |

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

# Method Lookup

Objects follow a lookup hierarchy to find methods, starting with the class, the inheritance hierarchy, and finally any singleton methods.

Classes can call instance methods of `Class`, and `new` is one such method. Since `Class` derives from the class `Module`, class objects have access to `Module`'s instance methods, which includes the `attr_accessor` family of methods.

# Value of self

When no explicit receiver is named, `self` is implied. In a class definition body, `self` is the class object itself.
