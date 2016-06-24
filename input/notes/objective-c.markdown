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

Objective-C has `nil` to represent a pointer to no object. Sending a message to `nil` has no effect and it is completely legal. However, sending a message to an object that doesn't implement that method _does_ crash the program.

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

However, as of Xcode 4.4 this is no longer necessary as the compiler does this for you. It's still necessary to do if the compiler ends up not generating any of the methods, for example if a property is marked `readonly` but the getter is manually overridden, then the `_variable` won't be defined, in which case it's necessary to explicitly `@synthesize` to define it or declare it manually in the `@interface`.

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

# Collections

Sets are represented by `NSSet` and `NSMutableSet`.

Collection methods which test for equality contain variants containing the word `Identical` which test if the objects are the same object by testing the pointers for equality, e.g. `indexOfObject:` vs `indexOfObjectIdenticalTo:`.

Dictionaries are represented by `NSDictionary` and `NSMutableDictionary`. Dictionaries can be created from literal syntax `@{…}`. A dictionary can be keyed using subscript notation, for example:

``` objective-c
NSDictionary *ages = @{
  @"John": @20,
  @"Jane": @21,
};

NSNumber johnAge = ages[@"John"];
```

Mutable arrays can be sorted using:

``` objective-c
- (void)sortUsingDescriptors:(NSArray *)sortDescriptors;
```

A sort descriptor is an object of type `NSSortDescriptor` which specifies a property of the sorted element---any instance variable or the result of any method of the object---and whether to sort it in ascending or descending order. `sortUsingDescriptors:` takes an array of sort descriptors so that in the event of equality, the next descriptor is used. For example, to sort by the property `lastName` in ascending order, the following descriptor may be used:

``` objective-c
NSSortDescriptor *lastAscending = [NSSortDescriptor sortDescriptorWithKey:@"lastName"
                                                                ascending:YES];
```

Collections can be filtered given a predicate of type `NSPredicate`. The filtering is done in-place on an `NSMutableArray` via `filterUsingPredicate:` whereas a copy is created for `NSArray` via `filteredArrayUsingPredicate:`. The predicate can be constructed [from a string](https://developer.apple.com/library/ios/documentation/Cocoa/Conceptual/Predicates/AdditionalChapters/Introduction.html) representing the condition:

``` objective-c
NSPredicate *pred = [NSPredicate predicateWithFormat:@"person.age > 18"];
NSArray *adults = [people filteredArrayUsingPredicate:pred];
```

An `NSNumber` is essentially a boxed number type which is used to wrap numbers so that they can be stored in collections such as `NSDictionary`. They can be constructed using `NSNumber` literals such as `@2`.

The `NSValue` type can be used to box/wrap arbitrary types such as structs.

It's not possible to insert `nil` into a collection. In order to represent a "hole" in a collection, the `NSNull` class can be used.

# Enumerations

The `NS_ENUM()` preprocessor macro can be used to specify the enumeration's backing data type and name.

``` objective-c
typedef NS_ENUM(int, Color) {
  ColorRed,
  ColorBlue,
  ColorGreen
};
```

# NSError

Some methods may fail for any reason. By convention, the error is returned through a parameter which is a pointer to a pointer to an `NSError`. That is, the parameter type is `NSError **`, but the caller can simply create a `NSError *` and return a pointer to it via `&error`. Methods that can take an `NSError` pointer parameter always return a value indicating whether or not there was an error.

``` objective-c
NSError *error = nil;

BOOL success = [obj someArgument:@"test" error:&error];

if (!success) {
  NSLog(@"failed: %@", [error localizedDescription]);
}
```

# NSData

The `NSData` class represents a buffer of bytes. The data can be written to a file using `writeToFile:options:error`. The option `NSDataWritingAtomic` ensures an atomic write operation.

It's possible to obtain the standard path for a given task by using the `NSSearchPathForDirectoriesInDomains` method. For example, to get the desktop directory:

``` objective-c
NSArray *desktops =
  NSSearchPathForDirectoriesInDomains(NSDesktopDirectory, NSUserDomainMask, YES);

NSString *desktopPath = desktops[0];
```

# Callbacks

Callbacks in Objective-C can take on four forms:

1. _Target-action_: Specify an object (target) and a message to send it (action).
2. _Helper objects_: Specify objects which do the required work. These objects are also known as _delegates_ or _data sources_.
3. _Notifications_: An object subscribes to the notification center for a particular kind of notification.
4. _Blocks_: Essentially a lambda meant to run when the event is triggered.

Events happen within the context of a run loop of type `NSRunLoop`.

The potential for strong reference cycles is high in most callback schemes. For example, an object may have a pointer to the object that will call it back, and that object contains a pointer to the object so that it _can_ call it back. To mitigate this:

* Notifications don't own their observers. Observers remove themselves from the notification center in their `dealloc` method via `removeObserver:`.

    ``` objective-c
    - (void)dealloc {
      [[NSNotificationCenter defaultCenter] removeObserver:self];
    }
    ```

* Objects don't own their delegates/data sources. Delegates/data sources remove themselves in their `dealloc` method.

    ``` objective-c
    - (void)dealloc {
      [delegator setDelegate:nil];
    }
    ```

* Objects don't own their targets. Target objects should remove themselves in their `dealloc` method.

    ``` objective-c
    - (void)dealloc {
      [button setTarget:nil];
    }
    ```

## Selectors

Each method name that the compiler encounters is given a unique number known as a _selector_ which is used to perform method lookup. The `@selector(…)` directive is replaced by the compiler with the selector for the given method.

## Target-action

Action methods are the methods invoked by a target-action combination, and they always take a single argument consisting of the object that sent the message.

Timers of type `NSTimer` use the target-action pattern to specify what message to send to what object every time the timer triggers. For example, the timer below will send the message `actionMethod` to `someObject` every 2 seconds.

``` objective-c
NSTimer *timer =
  [NSTimer scheduledTimerWithTimeInterval:2.0
                                   target:someObject
                                 selector:@selector(actionMethod:)
                                 userInfo:nil
                                  repeats:YES];
```

Unused variable warnings can be explicitly silenced by using the `__unused` keyword as a prefix to the type:

``` objective-c
__unused NSString *name = @"John";
```

When sending one callback to one object, Apple uses target-action.

## Helper Objects

Helper objects implement methods used to do different kinds of work. For example, asynchronous usage of `NSURLConnection` requires a helper object which defines methods to do work in response to new data, authentication, handle failure, etc.

For example, in the code below, the connection is configured to use object `logger` as its delegate, which defines methods which are invoked in response to specific events, such as `connectionDidFinishLoading:`.

``` objective-c
NSURLConnection *conn =
  [[NSURLConnection alloc] initWithRequest:request
                                  delegate:logger
                          startImmediately:YES];
```

The methods expected by an `NSURLConnection` are defined in a protocol---a list of method declarations. A protocol is implemented by specifying its name in angle brackets following the superclass in the `@interface` line:

``` objective-c
@interface MYLogger : NSObject <NSURLConnectionDelegate, NSURLConnectionDataDelegate>
{
  …
}
```

When sending various callbacks to one object, Apple uses a helper object with a protocol.

## Notifications

Various objects can subscribe to certain notifications using the notification center `NSNotificationCenter`. For example, the code below subscribes the `logger` object to a notification for when the time zone is changed, and it's configured to invoke the `zoneChange:` method. It's also possible to specify that only notifications sent from a particular object are to be considered.


``` objective-c
[[NSNotificationCenter defaultCenter]
  addObserver:logger
     selector:@selector(zoneChange:)
         name:NSSystemTimeZoneDidChangeNotification
       object:nil];
```

# Blocks

Blocks are essentially lambdas. They are prefixed by a caret `^` followed by optional parameter list and, followed by braces surrounding the body.

``` objective-c
^{
  NSLog(@"No parameters.");
}

^(int a, int b){
 return a + b;
}
```

Blocks can be stored in a variable, in which case their type must be explicitly typed. Block types look like function pointer types except that they use `^` to denote a block rather than `*` to denote a pointer.

``` objective-c
void (^devowelizer)(id, NSUInteger, BOOL *);
```

Anonymous blocks are ones that are passed directly to a method without first giving them a name by storing them in a variable.

_External variables_ are those that are captured by the block from the outer scopes. Primitive variables are copied as local variables within the block. Pointers are kept as strong references to ensure that they live at least as long as the block itself.

Since pointers are captured as strong references, it's easy to inadvertently create a strong reference cycle. This can happen implicitly when an instance variable is used directly within a block, because directly accessing an instance variable `_var` gets translated to `self->_var` by the compiler.

The cycle can be broken by first creating a `__weak` pointer to `self`, then using that within the block. However, this would mean that `self` could be deallocated while the block is executing. To prevent that, a strong reference to the `__weak` pointer can be created:

``` objective-c
__weak MYPerson *weakSelf = self;

block = ^{
  MYPerson *innerSelf = weakSelf;
  NSLog(@"Person: %@", innerSelf);
};
```

Instance variables should be accessed through the `innerSelf` to avoid implicitly and inadvertently capturing `self`.

Variables captured by a block are constant within the block. In order to modify an external variable within a block, it must be declared as an external variable by using the `__block` keyword prefix:

``` objective-c
__block int count = 0;

void (^incrementBlock)() = ^{ count++ };
```

# Protocols

A _protocol_ is a list of method declarations which may be required or optional. Any object that wants to conform to a protocol must implement the required methods.

``` objective-c
@protocol UITableViewDataSource <NSObject>

@required

- (NSInteger)tableView:(UITableView *)tv numberOfRowsInSection:(NSInteger)section;

…

@optional

- (NSInteger)numberOfSectionsInTableView:(UITableView *)tv;

…

@end
```

It's then possible to specify the type of any object that conforms to a particular protocol:

``` objective-c
@property (nonatomic, assign) id<UITableViewDataSource> dataSource;
```

In order to make a class conform to a protocol, the protocol name must be mentioned in the `@interface`:

``` objective-c
@interface SomeViewController : UIViewController <UITableViewDataSource>

…

@end
```

To avoid crashing the program when attempting to call an optional method that an object didn't implement, it's necessary to first check if the object did implement the method by using `NSObject`'s `respondsToSelector:` which returns a `BOOL` indicating if the object does respond to that message.

``` objective-c
if ([_dataSource respondsToSelector:@selector(numberOfSectionsInTableView:)]) {
  // call it here
} else {
  // don't call it; use some default value
}
```

# Property Lists

A property list (aka P-list) is an XML file format which is used to serialize Objective-C objects such as `NSArray`, `NSDictionary`, `NSString`, `NSData`, `NSDate`, and `NSNumber`.

Writing a property list can be achieved by using `NSMutableArray`'s `writeToFile:atomically:`. It can then be read in by using `arrayWithContentsOfFile`.

# iOS Applications

GUI applications start a run loop which listens for events.

The _company identifier_ is used to generate a _bundle identifier_ which is used to uniquely identify each app in the App Store.

Every iOS application has an app delegate which is a subclass of `UIResponder` which conforms to `UIApplicationDelegate`. The app delegate has a property of type `UIWindow` which fills the screen of the iOS application. Other controls can be added onto the window.

When an iOS application launches, an instance of `UIApplication` is created to control the application state and liaison with the operating system. An instance of the application delegate serves as the `UIApplication`'s delegate. For example, the `application:didFinishLaunchingWithOptions` method is invoked when the application becomes ready.

A control is created by specifying the a `CGRect` to specify its frame (positions and dimensions) and then it's added to the `UIWindow` as a sub-view.

``` objective-c
- (BOOL)application:(UIApplication *)application
        didFinishLaunchingWithOptions:(NSDictionary *)launchOptions
{
  // CGRect is (x, y, width, height)

  CGRect winFrame [[UIScreen mainScreen] bounds];
  UIWindow *theWindow = [[UIWindow alloc] initWithFrame:winFrame];
  self.window = theWindow;

  CGRect buttonFrame = CGRectMake(228, 40, 72, 31);
  self.button = [UIButton buttonWithType:UIButtonTypeRoundedRect];
  self.button.frame = buttonFrame;

  [self.button setTitle:@"Click me" forState:UIControlStateNormal];

  [self.window addSubview:self.button];
  [self.window makeKeyAndVisible];

  return YES;
}
```

A `UIButton` uses the target-action callback pattern to send a message when it receives certain events. For example, to send the `addTask:` message to the application delegate:

``` objective-c
[self.button addTarget:self
                action:@selector(addTask:)
      forControlEvents:UIControlEvenTouchUpInside];
```

The [App Distribution Guide](https://developer.apple.com/library/ios/documentation/IDEs/Conceptual/AppDistributionGuide/Introduction/Introduction.html) details Apple's process of distributing an application.

When using the Interface Builder, the `IBOutlet` keyword is actually an empty `#define` which only serves to tell Xcode that the pointer to the object will be set by Interface Builder and not manually in code. The `IBAction` keyword is actually an alias for `void` that tells Xcode that a method is an action method that will be used in a target-action pair configured in Interface Builder and not manually in code.

Interface Builder files `.xib` are XML files representing the UI.

# Initializers

When a class does not override the `init` method, `NSObject`'s `init` method will be called which zeroes out all of the subclass' instance variables.

When overriding the `init` method, the first line should call `super`'s `init`, then the return value should be checked to determine if a valid object was returned.

The return type of `init` should be `instancetype`, a keyword represents the type of an instance to which the method belongs. This is preferred over returning an explicit type such as `MYPerson *` so that subclasses can continue to use the same initializer to initialize objects of the subclass type. Previously this was done by returning `id`, but `instancetype` is preferred because it more narrowly restricts the type to the type of the receiver.

It's necessary to set `self` to the value returned by `super`'s `init` because a superclass `init` may have deallocated `self` and allocated a new object. Then it's necessary to check that the returned value is not `nil`, in which case we can forego further initialization and simply return `nil`.

``` objective-c
- (instancetype)init
{
  // initialize superclass object, e.g. NSObject
  self = [super init];

  // above may return nil
  if (self) {
    _instance_variable = 3;
  }

  return self;
}
```

It's also common to do the three things in one step: initializing the superclass object, assigning it to `self`, and checking its value:

``` objective-c
- (instancetype)init
{
  if (self = [super init]) {
    _instance_variable = 3;
  }

  return self;
}
```

It's also possible to create initializers which take arguments in the same manner.

In initializers it's common to simply use instance variables directly rather than through accessors because it is more semantically correct, since accessors may assume that the object is already initialized. However, using accessors usually works fine.

It's often necessary to override superclass initializers to account for the subclass' additional variables, otherwise users may initialize a subclass using a superclass initializer which will not initialize the rest of the subclass variables, leaving the object in an inconsistent state.

A _designated initializer_ is a single initializer in a class that all other initializers in the class directly or indirectly call and rely upon for complete initialization of the object.

If the subclass' designated initializer has a different name than the designated initializer of its superclass, the superclass' designated initializer should be overridden to call the new subclass' designated initializer.

When there is no good default initialization of an object, `init` can be overridden to raise an exception that will crash the program:

``` objective-c
- (instancetype)init
{
  [NSException raise:@"MYNoDefaultUserPass"
              format:@"Use initWithUser:Pass:, not init"];
}
```

# Properties

The `@property` directive accepts a list of attributes that affects how the accessors are created.

The `readwrite` and `readonly` attributes specify whether to create both setters and getters or just getters, respectively.

The `assign` attribute is the default attribute for non-object types. It simply assigns the passed value to the property.

The `strong` attribute is the default for object pointers. It ensures that a strong reference is kept to the passed object, releasing ownership of the old object.

The `weak` attribute represents a weak reference to the passed object. IF the pointed-to object is deallocated, the property is set to `nil`.

The `unsafe_unretained` attribute acts like the `weak` attribute _except_ that the property is _not_ automatically set to `nil` when the pointed-to object is deallocated.

The `copy` attribute creates a copy of an object and then makes the pointer point to the copy. This is commonly used with objects that have mutable subclasses. This ensures that if an `NSMutableString` is passed, an immutable `NSString` copy will be created and pointed to, and if an immutable `NSString` is passed, no copy is actually performed because `NSString` overrides `copyWithZone:`, called by `copy`, to just return the pointer to itself.

The `atomic` attribute ensures that the setters are atomic. This is the default attribute but is rarely needed, so most properties should be explicitly marked `nonatomic`.

# Key-Value Coding

Key-Value Coding allows reading and setting a property using its name, for example the following two are equivalent:

``` objective-c
[person setName:@"John"];
[person setValue:@"John" forKey:@"name"];
```

The `setValue:forKey:` method is defined in `NSObject` and looks for a setter method, property, or instance variable of the given name.

Similarly, the `valueForKey:` method can be used to read a value.

Key-value coding is used by general frameworks that need to read or push data to one's own custom objects, since it provides a uniform way of accessing that information regardless of the object's layout and capabilities.

It's possible to use key-value coding to set primitive types by for example using `NSNumber`.

``` objective-c
[person setAge:[NSNumber numberWithInt:18] forKey:@"age"];
```

A _key path_ is a way to use dot notation to specify a property in a given object hierarchy. For example:

``` objective-c
NSString *schoolName = [person valueForKeyPath:@"class.school.name"];
```

There is also `setValue:forKeyPath:`.

# Key-Value Observing

Key-value observing (KVO) is a way of subscribing to notifications that are emitted when an object's property changes. KVO enables Cocoa bindings and Core Data.

KVO is accomplished by using `NSObject`'s `addObserver:forKeyPath:options:context:` to specify an observer object which implements a method called when the property changes.

``` objective-c
@implementation MYObserver

- (void)observeValueForKeyPath:(NSString *)keyPath
                      ofObject:(id)object
                        change:(NSDictionary *)change
                       context:(void *)context
{
  NSString *oldValue = [change objectForKey:NSKeyValueChangeOldKey];
  NSString *newValue = [change objectForKey:NSKeyValueChangeNewKey];

  NSLog(@"Property %@ on object %@ changed: %@ → %@",
        keyPath, object, oldValue, newValue);
}

@end
```

``` objective-c
__unused MYObserver *observer = [[MYObserver alloc] init];

[person addObserver:observer
         forKeyPath:@"name"
            options:NSKeyValueObservingOptionNew | NSKeyValueObservingOptionOld
            context:nil];
```

Since a superclass may itself use KVO, it may be necessary for the subclass to determine if the notification was meant for it or its superclass, in which case it would need to be forwarded to the superclass. This differentiation can be accomplished by passing a class-unique pointer to the `context:` parameter. This class-unique pointer is often accomplished by creating a static variable.

``` objective-c
// pass as context:&contextForKVO
static int contextForKVO;

@implementation MYObserver

- (void)observeValueForKeyPath:(NSString *)keyPath
                      ofObject:(id)object
                        change:(NSDictionary *)change
                       context:(void *)context
{
  if (context != &contextForKVO) {
    // notification is for superclass
    [super observeValueforkeypath:keyPath
                         ofObject:object
                           change:change
                          context:context];
  } else {
    // notification is for us
  }
}

@end
```

If the accessor method is not used, the notification will have to be sent explicitly by using `NSObject`'s `willChangeValueForKey:` and `didChangeValueForKey:`.

It's possible to specify that a given property is changed whenever another one is changed. For example, if an `ageString` property is recomputed whenever the `age` property is changed, KVO can be configured to send notifications whenever `age` changes. This is accomplished by implementing a method named `keyPathsForValuesAffecting` concatenated with the name of the property in camel case. This method returns an `NSSet` of properties that, when changed, lead to a change in the named property:

``` objective-c
+ (NSSet *)keyPathsForValuesAffectingAgeString
{
  return [NSSet setWithObject:@"age"];
}
```

