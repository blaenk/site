+++
title = "TypeScript"
date = 2018-04-29

[note]
kind = "language"
+++

# Basic Types

They're all lowercase. There is `boolean`, `number`, and `string`.

Arrays are expressed as `Array<T>` or with the syntax sugar `T[]`.

Tuple types are expressed as `[T, U]` and constructed with the same bracket syntax. Tuple members are accessed via index as with arrays. Any indices outside of the known members are given a union type of all of the types of the known members. For example, if a tuple contains two members known to be a string and a number, then indices above `1` get the union type `string | number`.

Enumerations are expressed as similar to C++. Members can be given explicit values. An enumeration's name can be obtained from a raw value by indexing into the enumeration type.

``` typescript
enum Color { Red, Green, Blue }
enum Color { Red = 1, Green, Blue }
enum Color { Red = 1, Green = 2, Blue = 3}

let colorName: string = Color[2]; // "Green"
```

The `any` type can be used to opt-out of type-checking. A value of any type can be assigned to a variable of type `any`.

The `void` type represents the absence of any type, such as the return type of a function that doesn't return anything. Only `undefined` or `null` can be assigned to a variable of type `void`.

The values `undefined` and `null` have their own respective types with the same name. By default they are subtypes of all other types, so `null` and `undefined` can be assigned to any other type. However, the suggested `--strictNullChecks` flag ensures that they can only be assigned to variables of their respective types, requiring an explicit union type enumerating `null` or `undefined` to be able to do the same.

The `never` type represents the types of values that never occur, such as for a function that always throws an exception, or one that never returns [^rust_never]. The `never` type is a subtype of every type.

[^rust_never]: Similar to Rust's `!` type.

# Type Assertions

The compiler can be forced to treat a value as being of a certain type. There are two ways to express it, with the angle-bracket syntax not being usable in JSX.

``` typescript
let someValue: any = "this is a string";

let length1: number = (<string>someValue).length;
let length2: number = (someValue as string).length;
```

# Variable Declarations

`let` declarations essentially create a new scope per iteration when used in a for loop, obviating the need to use an IIFE for captured per-iteration variables.

``` typescript
// old
for (var i = 0; i < 10; i++) {
    (function(i) {
        setTimeout(function() { console.log(i); }, 100 * i);
    })(i);
}

// new
for (let i = 0; i < 10 ; i++) {
    setTimeout(function() { console.log(i); }, 100 * i);
}
```

# Interfaces

Interfaces facilitate structural subtyping [^structural_subtyping] by specifying that a variable must conform to at least the structure of the specified interface. This can be done in an ad-hoc manner or explicitly by specifying a named interface.

[^structural_subtyping]: As found in Go with its interfaces, for example.

``` typescript
function printLabel(labelledObj: { label: string }) {
    console.log(labelledObj.label);
}

let myObj = {size: 10, label: "Size 10 Object"};
printLabel(myObj);
```

With an explicit name:

``` typescript
interface LabelledValue {
    label: string;
}

function printLabel(labelledObj: LabelledValue) {
    console.log(labelledObj.label);
}

let myObj = {size: 10, label: "Size 10 Object"};
printLabel(myObj);
```

Interfaces may specify optional properties by adding a question mark `?` suffix.

``` typescript
interface SquareConfig {
    color?: string;
    width?: number;
}
```

The modification of certain properties can be restricted to creation-time by marking them `readonly`.

``` typescript
interface Point {
    readonly x: number;
    readonly y: number;
}
```

The `ReadonlyArray<T>` type works like an array without its mutating methods.

Object literals have excess property checking, so that if they have any properties that the target type doesn't have when being assigned to a variable of the target type, an error is emitted. This can be circumvented by explicitly using a type assertion.

``` typescript
let mySquare = createSquare({ width: 100, opacity: 0.5 } as SquareConfig);
```

Interfaces can also describe functions by specifying a call signature with no name.

``` typescript
interface SearchFunc {
    (source: string, subString: string): boolean;
}

let mySearch: SearchFunc = function(source: string, subString: string) {
    let result = source.search(subString);
    return result > -1;
};

// Inferred parameter types.
let inferredSearch: SearchFunc = function(src, sub) {
    let result = src.search(sub);
    return result > -1;
};
```

Interfaces can represent indexable types using an index signature describing the type that can be used to index and the type that is returned by indexing. Only `string` or `number` can be used to index, and both can be supported, but then the type returned from the numeric indexer must be a subtype of the type returned by the string indexer, since JavaScript automatically converts numeric indices into string indices (e.g. number 100 to string "100").

``` typescript
interface StringArray {
    [index: number]: string;
}
```

Since JavaScript `obj.prop` also means `obj["prop"]`, index signatures also enforce that _all_ properties match their return type.

``` typescript
interface NumberDictionary {
    [index: string]: number;
    length: number;    // ok, length is a number
    name: string;      // error, the type of 'name' is not a subtype of the indexer
}
```

Index signatures can be used to allow any kind of property.

``` typescript
interface SquareConfig {
    color?: string;
    width?: number;
    [propName: string]: any;
}
```

Index signatures can be marked `readonly` to prevent assignment to their indices.

It's possible to enforce that a class type implements a given interface using the `implements` syntax.

``` typescript
interface ClockInterface {
    currentTime: Date;
    setTime(d: Date);
}

class Clock implements ClockInterface {
    currentTime: Date;
    setTime(d: Date) {
        this.currentTime = d;
    }
    constructor(h: number, m: number) { }
}
```

Classes have two types: the type of the static side, as well as the type of the instance side. This means, for example, that a class constructor can't be expressed in an interface, because only the instance side of the class is checked, and the constructor is on the static side.

Interfaces can extend (inherit) from one or more other interfaces or classes.

``` typescript
interface Shape {
    color: string;
}

interface Square extends Shape {
    sideLength: number;
}

let square = <Square>{};
square.color = "blue";
square.sideLength = 10;
```

Interfaces can describe hybrid types that act as both functions and objects with additional properties.

``` typescript
interface Counter {
    (start: number): string;
    interval: number;
    reset(): void;
}
```

Interfaces can extend classes, which acts as if the interface had declared all of the member of the class without their implementations, including private and protected members. When the class contains private or protected members, this usually means that only that class or a subclass of it can implement the interface type.

Parameter properties allow the declaration and initialization of a property without needing to do it separately in the class body and constructor. A parameter property is only declared inside the constructor's parameter list. It's accomplished by prefixing a constructor parameter with an accessibility modifier and/or `readonly`.

``` typescript
class Octopus {
    readonly numberOfLegs: number = 8;

    // Creates and initializes property `name`.
    constructor(readonly name: string) {
    }
}
```

Accessors are defined similar to in JavaScript, by prefixing a method with `get` or `set`.

Accessors with a `get` but no `set` are inferred to be `readonly`.

``` typescript
class Employee {
    private _fullName: string;

    get fullName(): string {
        return this._fullName;
    }

    set fullName(newName: string) {
        if (passcode && passcode == "secret passcode") {
            this._fullName = newName;
        }
        else {
            console.log("Error: Unauthorized update of employee!");
        }
    }
}

let employee = new Employee();
employee.fullName = "Bob Smith";

if (employee.fullName) {
    console.log(employee.fullName);
}
```

Static properties are achieved by prefixing the property with `static`.

Abstract classes cannot be instantiated directly and are defined by prefixing the class with `abstract`. Unlike interfaces, abstract classes may define implementations. Individual methods can marked abstract enforce that derivations must implement them.

The type of a particular class constructor function can be obtained through the `typeof T` construct.

``` typescript
class Greeter {
    static standardGreeting = "Hello, there";
    greeting: string;
    greet() {
        if (this.greeting) {
            return "Hello, " + this.greeting;
        }
        else {
            return Greeter.standardGreeting;
        }
    }
}

let greeter1: Greeter;
greeter1 = new Greeter();
// greeter1.greet() == "Hello, there"

let greeterMaker: typeof Greeter = Greeter;
greeterMaker.standardGreeting = "Hey there!";

let greeter2: Greeter = new greeterMaker();
// greete2.greet() == "Hey there!"
```

# Classes

Constructors in derived classes must call `super()` to execute the base class' constructor. This must be done before any property on `this` is accessed in a constructor.

In TypeScript, members are `public` by default. Members marked `protected` can be accessed within deriving classes, but not externally. If a constructor is marked `protected`, then it can't be instantiated outside of the class, but it can be extended.

TypeScript has a structural type system, so that two different types are compatible if the types of all of their members are compatible. However, an exception is made for `private` and `protected` members, in which case they must originate from the same declaration, such as by deriving from the same base.

Class properties can be marked `readonly` so that they must be initialized at the site of their declaration or within the constructor.

Since classes define types, they can be used wherever interfaces may be used. For example, an interface can extend a class.

``` typescript
class Point {
    x: number;
    y: number;
}

interface Point3d extends Point {
    z: number;
}

let point3d: Point3d = {x: 1, y: 2, z: 3};
```

# Functions

Functions are statically typed. The return type can usually be inferred.

``` typescript
function add(x: number, y: number): number {
    return x + y;
}

let myAdd = function(x: number, y: number): number { return x + y; };
```

Function types are expressed similar to ES6 arrow functions. The parameter names in a function type don't need to match those in the actual function value. Unlike with function definitions, the return type cannot be omitted and inferred, so if there is no return value, it must be set to `void`.

``` typescript
let myAdd: (x: number, y: number) => number =
    function(x: number, y: number): number { return x + y; };
```

When TypeScript knows the full function type, the type of an assigned function expression can be inferred through contextual typing.

TypeScript expects each parameter to be passed to a function. Passing too few or too many parameters, or of the wrong type, is an error.

Parameters can be made optional by adding a question mark `?` suffix to their name. Optional parameters must follow required parameters.

Default-initialized parameters are specified by setting them equal to their default value. Those that come after all required parameters are also treated as optional parameters, but if they don't come after all required parameters, then users must explicitly pass `undefined` to get the default value.

Since a default-initialized parameter's default value doesn't affect its type, it will have the same type as a similar function where the same parameter is optional.

``` typescript
function buildName(firstName: string, lastName = "Smith") {
    return firstName + " " + lastName;
}

// Same type: (firstName: string, lastName: string) => string
function buildName(firstName: string, lastName?: string) {}
function buildName(firstName: string, lastName = "Smith") {}
```

Rest arguments can be gathered using the ellipsis `...` syntax.

``` typescript
function buildName(firstName: string, ...restOfName: string[]) {
    return firstName + " " + restOfName.join(" ");
}
```

It's possible to constraint the type of `this` within a function by including it as the first parameter of a function. By extension, this means it can be typed as `void` to prevent its use within a standalone function.

``` typescript
function f(this: void) {
    // make sure `this` is unusable in this standalone function
}
```

Typing `this` can be used to ensure that callbacks aren't bound to an object. The type of a method can type `this` to be `void` so that it can then type-check with such a callback interface, but then `this` would not be usable withing the method. Alternatively an arrow function can be defined to capture individual properties within it.

``` typescript
// Expect `onclick` function to not have `this` bound
interface UIElement {
    addClickListener(onclick: (this: void, e: Event) => void): void;
}

class Handler {
    info: string;

    // Can change `this` type to `void` to type-check.
    // Then `this` cannot be used within function.
    onClickBad(this: Handler, e: Event) {
        // oops, used this here. using this callback would crash at runtime
        this.info = e.message;
    }

    // Works
    onClickGood(this: void, e: Event) {
        // can't use this here because it's of type void!
        console.log('clicked!');
    }

    // Works
    onClickArrow = (e: Event) => { this.info = e.message }
}

let h = new Handler();
uiElement.addClickListener(h.onClickBad); // error!
uiElement.addClickListener(h.onClickGood); // error!
uiElement.addClickListener(h.onClickArrow); // error!
```

Functions can be overloaded by specifying the declarations of more specific overloads before the general, untyped definition, which checks for the actual type of the arguments if necessary.

``` typescript
function pickCard(x: {suit: string; card: number; }[]): number;
function pickCard(x: number): {suit: string; card: number; };
function pickCard(x): any {
    // Check to see if we're working with an object/array
    // if so, they gave us the deck and we'll pick the card
    if (typeof x == "object") {
        let pickedCard = Math.floor(Math.random() * x.length);
        return pickedCard;
    }
    // Otherwise just let them pick the card
    else if (typeof x == "number") {
        let pickedSuit = Math.floor(x / 13);
        return { suit: suits[pickedSuit], card: x % 13 };
    }
}
```

# Generics

Generic functions are specified with type variables.

``` typescript
function identity<T>(arg: T): T {
    return arg;
}

let myIdentity: <T>(arg: T) => T = identity;

// Object literal type
let myIdentity: {<T>(arg: T): T} = identity;

// Generic interface
interface GenericIdentityFn {
    <T>(arg: T): T;
}

let myIdentity: GenericIdentityFn = identity;

// Parameterized generic interface
interface GenericIdentityFn<T> {
    (arg: T): T;
}

let myIdentity: GenericIdentityFn<number> = identity;
```

Generics classes are only generic over their instance side rather than their static side, so static members cannot use the class' type parameter.

``` typescript
// Generic class
class GenericNumber<T> {
    zeroValue: T;
    add: (x: T, y: T) => T;
}

let myGenericNumber = new GenericNumber<number>();
myGenericNumber.zeroValue = 0;
myGenericNumber.add = function(x, y) { return x + y; };

let stringNumeric = new GenericNumber<string>();
stringNumeric.zeroValue = "";
stringNumeric.add = function(x, y) { return x + y; };
```

When creating factories using generics, the class type needs to be referred to by their constructor functions.

``` typescript
function create<T>(c: {new(): T; }): T {
    return new c();
}
```

Type variables can be constrained by an interface with the `extends` syntax.

``` typescript
interface Lengthwise {
    length: number;
}

function loggingIdentity<T extends Lengthwise>(arg: T): T {
    console.log(arg.length);  // Now we know it has a .length property, so no more error
    return arg;
}
```

One type parameter can be constrained by another.

``` typescript
function getProperty<T, K extends keyof T>(obj: T, key: K) {
    return obj[key];
}

let x = { a: 1, b: 2, c: 3, d: 4 };

getProperty(x, "a"); // okay
getProperty(x, "m"); // error: Argument of type 'm' isn't assignable to 'a' | 'b' | 'c' | 'd'.
```

# Enumerations

Numerical enumerations are similar to those in other languages. They can mix in computed and constant members. Those without initializers need to be first or need to come after constant-initialized members.

``` typescript
enum Direction {
    Up = 1,
    Down,
    Left,
    Right = getComputedValue(),
}
```

Numeric enum members can get a reverse mapping from enum values to enum names by indexing the enum type with the enum member.

``` typescript
enum Enum {
    A
}

let a = Enum.A;
let nameOfA = Enum[a]; // "A"
```

String enumerations are similar. Each member can be constant-initialized or initialized with another string enum member.

``` typescript
enum Direction {
    Up = "UP",
    Down = "DOWN",
    Left = "LEFT",
    Right = "RIGHT",
}
```

Heterogenous enums can technically have a mix of string and numeric members.

When all members in an enum have literal enum values, the enum type becomes a union of each of its members, and the enum members become types.

Enums are actual objects at runtime, so they can be used in locations that expect an enum so long as the correct property is accessed.

``` typescript
enum E {
    X, Y, Z
}

function f(obj: { X: number }) {
    return obj.X;
}

// Works, since 'E' has a property named 'X' which is a number.
f(E);
```

Constant enums only use constant enum expressions which are inlined at the use sites and completely removed during compilation.

``` typescript
const enum Directions {
    Up,
    Down,
    Left,
    Right
}
```

Ambient enums can be used to describe the shape of existing enum types.

``` typescript
declare enum Enum {
    A = 1,
    B,
    C = 2
}
```
