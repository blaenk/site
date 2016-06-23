---
title = "Objective-C"
published = "June 21, 2016"
comments = false
---

Although [Swift](/notes/swift) has been released, there is still a considerable iOS/Cocoa ecosystem which is built on Objective-C.

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
  …
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

However, as of XCode 4.4 this is no longer necessary as the compiler does this for you. It's still necessary to do if the compiler ends up not generating any of the methods, for example if a property is marked `readonly` but the getter is manually overridden, then the `_variable` won't be defined, in which case it's necessary to explicitly `@synthesize` to define it or declare it manually in the `@interface`.

Properties are also usable using dot notation as in C++ and Java. However, rather than simply accessing a field in a struct as in C++, dot notation translates to a message sent to the corresponding accessor method.

``` objective-c
NSString *theName = person.name;
```

It's possible to define a property with a different type than a manually defined member variable of the same name. For example, the class below defines and uses a mutable array internally, but publicly exposes an immutable array.

``` objective-c
@interface MYPerson : NSObject
{
  NSMutableArray *_names;
}

@property (nonatomic, copy) NSArray *names;

- (void)addName:(NSString *)n;

@end
```

Notice that the accessor methods are overridden to handle the translation between mutable and immutable arrays. Specifically, the setter sets the internal variable to a mutable copy of the passed immutable array, while the getter returns an immutable copy of the internal immutable array.

``` objective-c
@implementation MYPerson

- (void)setNames:(NSArray *)n
{
  _names = [n mutableCopy];
}

- (NSArray *)names
{
  return [_names copy];
}

- (void)addName:(NSString *)n
{
  if (!_names) {
    _names = [[NSMutableArray alloc] init];
  }

  [_names addObject:n];
}

@end
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

Forward declarations are possible with the `@class` keyword.

When a `@property` is defined on a class without a corresponding manually-defined instance variable, subclasses aren't able to access the synthesized instance variable directly; they must do so via the accessors. For example, given:

``` objective-c
@interface MYPerson : NSObject
{
}

@property (nonatomic) NSMutableArray *friends;
@end

@interface MYEmployee : MYPerson
{
  …
}
@end
```

Then a subclass `MYEmployee` cannot access `_friends` directly; it must do so via an explicit accessor or dot notation.

``` objective-c
@implementation MYEmployee

…

[_friends addObject:@"Bob"];     // Error

[self.friends addObject:@"Bob"]; // Ok
```

# Ownership

An object that contains a pointer to another object is said to own that object. Due to reference counting, the owned object knows how many owners it has through its reference count.

The `dealloc` method is run when an instance of a class is deallocated because it has no owners.

# Class Extensions

Private internal methods, instance variables, and properties should be defined in a _class extension_, which is a set of private declarations that only the class or instances of it can use. A class interface is denoted by a typical `@interface` block with an empty parentheses pair at the end. By convention class extensions are declared in the implementation file, before the `@implementation`.

``` objective-c
#import "MYPerson.h"

@interface MYPerson ()

@property (nonatomic) int somePrivateVariable;

@end

@implementation MYPerson

…

@end
```

An [earlier example](#classes) demonstrated that it's possible to have a manually-defined instance variable differ in the type of a separate property of the same name. However, doing that can be confusing, and instead it's recommended to use a private class extension to define the manually-defined instance variable.

Since the class extension is defined in the implementation file, and subclasses `#import` the header file, subclasses won't have access to the superclass' class extensions.

# Reference Counting

A Strong reference cycle represents a potential for a memory leak, because the garbage collector cannot deallocate either side of the cycle. A strong reference cycle can be weakened with a _weak reference_ which is a pointer that does not imply ownership. This is useful in a parent-child relationship, in which case the child should hold a weak reference to the parent, since the parent is what owns the child.

``` objective-c
@interface TreeNode : NSObject

@property (nonatomic, weak) TreeNode *parent;

@end
```

When the targets of weak pointers are deallocated, the weak pointer is set to `nil`.

Weak points can be explicitly created with the `__weak` keyword:

``` objective-c
__weak MYPerson *parent;
```

Before ARC, manual reference counting was necessary using the `retain` and `release` methods. For example, in a setter, the passed object was `retain`ed to increment its reference count and the previously-held object was `release`d to decrease its reference count, then the pointer was set to the new object:

``` objective-c
- (void)setPerson:(MYPerson *)newPerson
{
  [newPerson retain];
  [_person release];
  _person = newPerson;
}
```

Furthermore, the `dealloc` call had the responsibility of `release`ing all held objects and `dealloc`ating the immediate superclass.

``` objective-c
- (void)dealloc
{
  [_holder release];
  [super dealloc];
}
```

Newly created and returned objects would be marked as `autorelease`, i.e. `release` sometime in the future. For example, the `description` method creates and returns a new `NSString`, so it was marked for `autorelease`:

``` objective-c
- (NSString *)description
{
  NSString *result = [[NSString alloc] initWithFormat:@"Person: %@", [self name]];

  [result autorelease];
  return result;
}
```

Specifically the object was sent the `release` message when the current autorelease pool was drained:

``` objective-c
NSAutoreleasePool *arp = [[NSAutoreleasePool alloc] init];

NSString *desc = [[[MyPerson alloc] init] description];

[arp drain]; // `desc` sent `release` message
```

The syntax sugar `@autoreleasepool` can be used to automatically create an autorelease pool and drain it at the end of the provided block.

``` objective-c
@autoreleasepool {
  NSString *desc = [[[MyPerson alloc] init] description];
} // drained here
```

There are a couple of rules of thumb for manual reference counting:

* Creating an object using a method starting with `alloc`, `new`, or containing `copy` gives you ownership of it. Assume refcount = 1, not in autorelease pool.

* Objects created by any other means are not owned by you. Assume refcount = 1, in autorelease pool.

* Take ownership by `retain`ing it.

* Relinquish ownership by using `release` or `autorelease`.

* Objects exist as long as they have an owner.

This explains why the `NSString` returned by `description` is `autorelease`d: because although it created the object via `alloc`-`init` and thus gained ownership of it, it is giving it away by returning it. Sending it a `release` message would immediately decrement its refcount, thereby deallocating it, so instead it is `autorelease`d.
