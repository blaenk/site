---
title = "Objective-C"
published = "June 21, 2016"
comments = false
---

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
