---
title = "Objective-C"
published = "June 21, 2016"
comments = false
---

<toc/>

# Types

The `BOOL` type is an alias for an integer with variants `YES` and `NO` set to `1` and `0` respectively. Since these are integers, it's not advised to explicitly compare to `YES` because an affirmative value can be any non-zero value.

Explicit integer types can be used such as `UInt32` for an unsigned 32-bit integer or `SInt16` for a signed 16-bit integer.

The `NSInteger` and `NSUInteger` types are aliases for the integer width of the system, e.g. 32-bit on 32-bit systems.

# Objects

Instead of invoking a method on an object, a message is sent to it:

``` objective-c
NSDate *now = [NSDate date];
double seconds = [now timeIntervalSince1970];
```

Naturally, sent messages are expressions which can be embedded where expressions of that type are expected:

``` objective-c
double seconds = [[NSDate date] timeIntervalSince1970];
```

_Class methods_ are when a message is sent to a class such as to `NSDate`, whereas _instance methods_ are sent to a particular instance, such as to `now`.

Method arguments are named when passed to a method.

``` objective-c
[now dateByAddingTimeInterval:100000]
```

A method's name is often described by the concatenated names of its parameters:

``` objective-c
// ordinalityOfUnit:inUnit:forDate:
[cal ordinalityOfUnit:NSDayCalendarUnit
               inUnit:NSMonthCalendarUnit
              forDate:now]
```

A method's declaration starts with a `-` for instance methods and `+` for class methods, followed by the parameters and their types, e.g.

``` objective-c
- (BOOL)isEqualToString:(NSString *)other
```

The `alloc` class method handles the allocation of the memory for an object of the class type. It returns a pointer to the allocated memory, but it is _not initialized_. The memory must be initialized with the `init` instance method.

``` objective-c
NSDate *now = [[NSDate alloc] init];
```

Convenience methods are those which perform an allocation-initialization sequence in a conveniently expected manner. For example, since `NSDate`'s `init` method initializes the `NSDate` to the current date and time, the `date` class method is considered a convenience method since it's more convenient than an explicit `alloc`-`init` chain.

Objective-C has `nil` to represent a pointer to no object. Sending a message to `nil` has no effect and it is completely legal.

The `id` type is a pointer to an Objective-C object:

``` objective-c
id delegate = [some delegate];
```

# Automatic Reference Counting

Previously it was necessary to perform manual reference counting by invoking reference-manipulating methods such as `release` or `retain`. _Automatic Reference Counting_ (ARC) injects similar constructs automatically into the code.

# NSString

`NSString` is a high-level immutable string type. `NSString` literals are string literals prefixed with the at sign `@`.

``` objective-c
NSString *name = @"John";
```

`NSString`s can contain Unicode characters by escaping them with `\u`.

It's possible to create an `NSString` from a given format:

``` objective-c
NSString *dateString = [NSString stringWithFormat:@"Today is %@", [NSDate date]];
```

The `length` method is used to retrieve the string length. Strings can be compared with `isEqualToString:`.

``` objective-c
if ([name isEqualToString:@"John"]) {
  NSLog(@"They're equal");
}
```

# NSArray

An `NSArray` is an immutable array that holds pointers to other objects. `NSArray` literals can be created with the syntax `@[…]`.

``` objective-c
NSArray *names = @[@"John", @"Jane"];
```

Before `NSArray` literal syntax was introduced, the class method `arrayWithObjects:` was used to specify the list of objects with which to initialize the array, _terminated_ by `nil`.

Elements of the array are accessed as with any other array, using subscript notation:

``` objective-c
NSString *john = names[0];
```
Before subscripting was introduced, the method `objectAtIndex:` was used to access a particular element.

The size of the array can be obtained with the `count` method.

`NSArray`s can be iterated over using the `for-in` syntax known as _fast enumeration_.

``` objective-c
for (NSString *name in names) {
  NSLog(@"%@ is in the array");
}
```

# NSMutableArray

`NSMutableArray` is a subclass of `NSArray` that is mutable. The `addObject:` method can be used to push an object onto the array, whereas `insertObject:atIndex:` can insert an object at a particular location in the array.

The convenience class method `array` creates an empty array, just like an `alloc`-`init` chain would.

It's not possible to add or remove elements within a fast enumeration loop because the iterators would become invalidated. Instead use a regular for loop.

# Classes

By convention classes are defined in pairs of header (.h) and implementation (.m) files. The header contains the interface that begins with the `@interface` keyword which states the class name and its base classes, as well as its member variables. Method declarations are placed after and outside of the braces. The end of the interface is marked with `@end`.

Due to the lack of namespaces, by convention classes and types are prefixed by some namespace initials preferably 3 or more letters in length, for example the `NS` in `NSString` stands for [NeXTSTEP](https://en.wikipedia.org/wiki/NeXTSTEP).

By convention member variables are prefixed with an underscore `_`, and getters take on the name of the variable without the underscore prefix and setters are prefixed with `set`.

``` objective-c
@interface MYPerson : NSObject
{
  int _age;
  NSString *_name;
}

- (int)age;
- (void)setAge:(int)a;

@end
```

By convention, accessor methods are used within instance methods rather than accessing the instance variables directly.

The implementation file (.m) includes the header file by using the `#import` directive which among other things prevents double inclusions. The beginning of the method implementations is marked with `@implementation` and the end of the implementation is marked with `@end`.

``` objective-c
#import "MYPerson.h"

@implementation MYPerson

- (int)age
{
  return _age;
}

- (void)setAge:(int)a
{
  _age = a;
}

@end
```

Like `this` in C++, the `self` keyword is an implicit local variable in instance methods which points to the object running the method.

Rather than manually defining accessor methods, it's possible to define properties for which the compiler automatically defines accessor methods. A property is marked with `@property` and it takes an argument list specifying the property's attributes (e.g. atomic or nonatomic), followed by the type and name of the property. Properties are declared within the class' `@interface`.

``` objective-c
@interface MYPerson : NSObject

@property (nonatomic) int age;
@property (nonatomic, copy) NSString *name;

{
}

@end
```

Property attributes can specify whether the property is `atomic` or `nonatomic`, or whether to _only_ generate a getter but not a setter via `readonly`. For example, marking it `readonly` will cause the compiler to generate a getter method but no setter method. The `readwrite` permission is the default. The `copy` attribute specifies whether the property should be copied when set or retrieved via a getter. This should be used whenever declaring a property of a pointer to an object.

Previously it was then necessary to place a corresponding `@synthesize` directive in the `@implementation` to actually direct the compiler to generate the accessor methods. The `@synthesize` directive specified the name that would be publicly available in the accessor methods and the name to use internally.

``` objective-c
@implementation MYPerson

@synthesize age = _age;
@synthesize name = _name;

@end
```

However, as of XCode 4.4 this is no longer necessary as the compiler does this for you.

Properties are also usable using dot notation as in C++ and Java. However, rather than simply accessing a field in a struct as in C++, dot notation translates to a message sent to the corresponding accessor method.

``` objective-c
NSString *theName = person.name;
```

# Inheritance

The superclass that a class inherits from is specified in the `@interface` line and it doesn't need to be repeated in the `@implementation` line.

A method is overridden by simply redefining its implementation; it's _not_ possible to redefine its name, return type, or argument types.

The `super` keyword is an implicit local variable within instance methods that refers to the object of the superclass from which the class inherits.

`NSObject` contains an instance variable named `isa` which points at the class that created it. For example, `MYPerson`'s `isa` variable would point to `MYPerson`. When a message is sent to an object, it checks the `isa` pointer to see if the class it points to defines such a method, if not, the search continues up the inheritance hierarchy. If the search reaches the top (`NSObject`) and the method is still not found, an error is emitted specifying that an "unrecognized selector sent to instance." This method lookup is what facilitates method overriding.

The `%@` token in format strings passed to `NSLog` for example cause a `description` message to be sent to the target object. `NSObject` defines this method which simply returns the object's address formatted as a string. It can be overridden in a subclass to provide more useful information.

``` objective-c
@implementation MYPerson

- (NSString *)description
{
  return [NSString stringWithFormat:@"<Person %@>", self.name];
}

@end
```
