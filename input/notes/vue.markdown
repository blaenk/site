---
title = "Vue.js"
published = "August 26, 2017"
excerpt = "The Vue.js Library"
comments = false
---

## Vue Instance

The `Vue` function can be used to create a Vue instance, which by convention is often named `vm` for "ViewModel." The `Vue` function takes an options object.

Upon creation, all properties found in its `data` object are registered with the reactivity system. Only the `data` properties that existed at instantiation time are reactive.

``` javascript
const data = { a: 1 };

const vm = new Vue({ data });

vm.a === data.a; // => true

vm.a = 2;
data.a // => 2

data.a = 3;
vm.a // => 3
```

Various built-in `Vue` [instance properties and methods] are prefixed with `$`.

[instance properties and methods]: https://vuejs.org/v2/api/#Instance-Properties

``` javascript
vm.$data === data; // => true
```

## Lifecycle Hooks

It's possible to define lifecycle hooks just [as in React]. Each hook has `this` set to the Vue instance invoking it, so avoid using arrow functions as that overrides it with the parent context. Possible hooks include `created`, `mounted`, `updated`, and `destroyed`.

[as in React]: /notes/react/#lifecycle

``` javascript
new Vue({
  data: { a: 1 },
  created() {
    console.log(`a is: ${this.a}`);
  },
});

// => 'a is: 1'
```

Here's a diagram detailing a Vue instance's lifecycle:

<img src="https://vuejs.org/images/lifecycle.png" />

## Templates

Vue's templates remind me a bit of Angular's. A common misconception is that this is just "plain HTML," often mentioned as a counterpoint to React's JSX. However, Vue's templates—although possible to define as HTML—are nonetheless compiled to JavaScript as Virtual DOM render functions, similar to JSX.

Text interpolation can be accomplished through Mustache-style curly braces. These interpolations will update whenever the value being interpolated changes. The `v-once` directive can be used to suppress that behavior.

``` html
<span>Reactive message: {{ message }}</span>

<span v-once>This will not update: {{ message }}</span>
```

The above interpolations escape HTML, but raw HTML can be inserted via the `v-html` directive, which takes as argument the property containing the HTML, and causes the contents of the tag it is placed on to take on that HTML content.

``` html
<div v-html="rawHTMLProperty"></div>
```

Mustaches cannot be placed within HTML attributes; instead, the `v-bind` directive should be used to "bind" a value to an attribute. Boolean attributes such as `disabled` are configured to remove themselves if the value is falsy.

``` html
<!-- `id` attribute set to the value of `idProperty`. -->
<div v-bind:id="idProperty"></div>

<!-- if `isButtonDisabled` is falsy, remove `disabled` attribute. -->
<button v-bind:disabled="isButtonDisabled">Button</button>
```

Mustaches and attributes can contain arbitrary JavaScript expressions, but like JSX, each may contain a _single_ expression.

``` html
{{ number + 1 }}
{{ ok ? 'YES' : 'NO' }}

<div v-bind:id="'list-' + id"></div>
```

### Directives

Directives are attributes with a `v-` prefix. Directives may take arguments, denoted by a colon `:` after the directive name.

``` html
<!-- argument is `href` -->
<a v-bind:href="url"></a>
```

Modifiers are denoted by a dot `.` prefix, and may be stacked.

``` html
<!-- modifier is `prevent` -->
<form v-on:submit.prevent="onSubmit"></form>
```

The shorthand for `v-bind` is the colon `:` by itself.

``` html
<a v-bind:href="url"></a>
<a :href="url"></a>
```

The shorthand for `v-on` is the at symbol `@`.

``` html
<a v-on:click="clicked"></a>
<a @click="clicked"></a>
```

### Class and Style Bindings

The `v-bind:class` directive can accept an object where keys are class names that will be part of the final `class` attribute if the corresponding value is truthy. The object can be specified inline or as a reference to a property. A computed property may be most natural to use for this, since dependencies are automatically registered.

A separate, direct `class` attribute may also be specified and will be part of the resulting `class` attribute unconditionally.

``` html
<div v-bind:class="{ active: isActive }"></div>

<div v-bind:class="someObject"></div>
```

The `v-bind:class` directive can also accept an array of class names. Going further, objects may be embedded within an array of classes to mix conditional and unconditional classes.

``` html
<div v-bind:calss"[unconditionalClass, { conditionalClass: isActive }]"></div>
```

When a `class` attribute is used on a custom component, it ends up being applied to that component's root element, appended if that element itself contains a `class` attribute.

Much like `v-bind:class`, `v-bind:style` also accepts an object of style properties, referred to as a "style object". It also accepts an array of style objects that are merged together with later conflicting styles overriding earlier ones.

Vue automatically applies vendor prefixes to CSS property names that require them.

### Conditional Rendering

The `v-if` directive can be used to conditionally render an element. An immediately-following, sibling element can optionally be provided with a corresponding `v-else` directive, with the expected effect.

``` html
<div v-if="isVisible">Visible</div>
<div v-else></div>
```

The `<template>` element can be used as an invisible wrapper to group multiple same-level elements in order to, for example, toggle all of their visibility at once.

There is also a `v-else-if` directive that has the expected effect.

The `v-show` directive can be used to conditionally _hide_ an element. Whereas `v-if` affects whether or not the element is rendered _at all_, `v-show` simply toggles the element's `display` property. For this reason, it wouldn't work on a `<template>` element since it has no corresponding element in the output DOM on which to apply the `display` property.

Since `v-if` affects whether or not the element is rendered at all, if it's _not_ rendered, it will avoid creating the element and with it everything that that may entail, such as creating event listeners and child components. On the other hand, since `v-show` only affects the resulting element's _visibility_, the element and everything it contains is fully created.

That means that `v-show` has a higher initial render cost but cheaper toggle cost, whereas `v-if` has a lower initial render cost (if it's not rendered) but a higher toggle cost (having to recreate the elements each time).

Prefer to use `v-show` for content that is likely to be toggled often, and `v-if` otherwise.

### List Rendering

The `v-for` directive can be used to replicate the element it's applied to for each element in an array, object, or integer range. It's possible to replicate multiple elements by wrapping them in the `<template>` tag, as with `v-if`.

``` html
<ul>
  <li v-for="item in items">
    {{ item.message }}
  </li>
</ul>
```

It's possible to enumerate the indices of the array by binding a second variable:

``` html
<li v-for="(item, index) in items">
```

The `v-for` enumeration syntax may use `in` or `of` as a delimiter.

The `v-for` directive may also iterate over an object's property _values_. However, as with array index enumeration, the key can be enumerated as well by binding a second variable. In fact, an index can also be bound by providing yet another third variable.

``` html
<div v-for="value in object">{{ value }}</div>

<div v-for="(value, key) in object">{{ key }}: {{ value }}</div>

<div v-for="(value, key, index) in object">
  {{ index }}. {{ key }}: {{ value }}
</div>
```

The `v-for` directive can also iterate over a range by supplying an integer instead of an array or object.

``` html
<div>
  <span v-for="n in 10">{{ n }}</span>
</div>
```

When an element contains both `v-for` and `v-if`, the `v-for` has higher precedence and is evaluated _before_ the `v-if`, meaning that each replicated element will evaluate `v-if` separately.

The `v-for` directive can be applied to custom components, but data must be passed explicitly via `v-bind` for example, since each component has its own isolated scope. It was Vue's design decision to be explicit about the interface rather than implicitly—perhaps inadvertently—tightly couple the component with its parent.

``` html
<my-component
  v-for="(item, index) in items"
  v-bind:item="item"
  v-bind:index="index"
  v-bind:key="item.id">
</my-component>
```

### Element Keys

Similar DOM sub-trees can be differentiated with a `key` attribute, as in React. This ensures that the Virtual DOM diff algorithm wont inadvertently reuse elements that it shouldn't reuse.

It's considered good practice to use a `key` in conjunction with a `v-for` directive to ensure optimal efficiency and avoid surprising behavior.

## Computed Properties

A computed property can be defined for complex expressions, often based on instance properties.

For example, the following should be factored out to a computed property:

``` html
<div id="example">
  {{ message.split('').reverse().join('') }}
</div>
```

Defining a computed property has the effect of defining a getter function for a property with the provided name.

``` javascript
const vm = Vue({
  el: '#example',
  data: { message: 'Hello' },
  computed: {
    reversedMessage() {
      return this.message.split('').reverse().join('');
    },
  },
})
```

Since this defines a property, it can be accessed like any other:

``` html
<div id="example">
  {{ reversedMessage }}
</div>
```

Vue is aware of a computed property's data dependencies, so that when those dependencies are updated, so are any bindings that themselves depend on the computed property.

Unlike methods, computed properties are cached based on their dependencies, so that they are only re-evaluated if any of the dependencies have changed, otherwise it serves the cached value.

For that same reason, including any global side-effect code such as `Date.now()` may not have the intended effect, because the value will not change if _only_ that expression changed, since it's not a reactive dependency. Such code could be better expressed as a method.

Watchers may appear to be equivalent to computed properties. For example, the following intends to maintain a `fullName` property updated whenever either the `firstName` _or_ the `lastName` changes.

``` javascript
var vm = new Vue({
  data: {
    firstName: 'John',
    lastName: 'Doe',
    fullName: 'John Doe'
  },
  watch: {
    firstName: function (val) {
      this.fullName = val + ' ' + this.lastName
    },
    lastName: function (val) {
      this.fullName = this.firstName + ' ' + val
    },
  },
});
```
However, the above is much more verbose and imperative compared to the declarative, computed property implementation. With a computed property implementation, Vue already knows to only recompute the value when the dependencies change, and the dependencies are automatically detected. This makes the code less error-prone and more flexible, such as in the event that the dependencies change.

``` javascript
var vm = new Vue({
  data: {
    firstName: 'John',
    lastName: 'Doe'
  },
  computed: {
    fullName: function () {
      return this.firstName + ' ' + this.lastName
    },
  },
})
```

A setter can also be provided for a computed property. For example, the setter below will ensure that if the value of the computed property is set, the underlying dependencies are updated accordingly.

``` javascript
var vm = new Vue({
  data: {
    firstName: 'John',
    lastName: 'Doe'
  },
  computed: {
    get() {
      return this.firstName + ' ' + this.lastName;
    },
    set(newValue) {
      const names = newValue.split(' ');

      this.firstName = names[0];
      this.lastName = names[names.length - 1];
    },
  },
})
```

## Watchers

Watchers are a more general construct compared to computed properties. A function can be registered to "watch" a given property, and execute when it is changed. This is accomplished by registering a function for a given property which accepts the new value of that property.

Watchers are usually used in asynchronous or expensive computational contexts.

``` javascript
const vm = new Vue({
  data: { someProperty: 1 },
  watch: {
    someProperty(newValue) {
      …
    }
  }
})
```

## Reactivity Caveats

Vue wraps observed arrays' mutation methods so that they trigger view updates. In particular, the wrapped methods are:

* `push()`
* `pop()`
* `shift()`
* `unshift()`
* `splice()`
* `sort()`
* `reverse()`

Since objects are also observed, it's also possible to simply replace an array property.

Vue _cannot_ detect direct element setting via the indexing operator.

``` javascript
// wrong
vm.items[index] = value;

// correct
Vue.set(vm.items, index, value);
```

Vue also _cannot_ detect the direct modification of the array length.

``` javascript
// wrong
vm.items.length = newLength;

// correct
vm.items.splice(newLength);
```

Vue also _cannot_ detect property addition or deletion. This is why Vue cannot allow dynamically adding new root-level reactive properties. This _is_ possible to do on existing objects via `Vue.set`:

``` javascript
Vue.set(vm.someObject, 'newProperty', someValue);

// Equivalent (in instance):
this.$set(this.someObject, 'newProperty', someValue);
```

When doing mass assignment via `Object.assign`, do so in pure, immutable fashion as in Redux, replacing the original property, instead of overwriting it.

``` javascript
// wrong
Object.assign(this.userProfile, {
  age: 27,
  favoriteColor: 'Vue Green'
});

// correct
this.userProfile = Object.assign({}, this.userProfile, {
  age: 27,
  favoriteColor: 'Vue Green'
});
```

When wanting to show filtered or sorted results, it's best to create a computed property with those filtered or sorted items. If this would end up being too expensive, it's also possible to just define a method that does this.

## Event Handling

The `v-on` directive can be used to listen to DOM events and register methods to be invoked in response to them. Invoked methods are passed the event object. The directive can take as argument the event to listen for.

Any event listeners registered with `v-on` are automatically removed when the ViewModel is destroyed.

The directive can take modifiers which affect the event, such as `.stop` to invoke `event.stopPropagation()`. Note that the code is generated in the order specified by the modifiers. Possible modifiers include:

| Modifier   | Description                       |
|------------|-----------------------------------|
| `.stop`    | `e.stopPropagation()`             |
| `.prevent` | `e.preventDefault()`              |
| `.capture` | Handle before inner element       |
| `.self`    | Only trigger if is `event.target` |
| `.once`    | Trigger at most once              |

The `v-on` directive also has modifiers for filtering for specific keycodes when listening for keyboard events by defining modifiers with aliases for common keycodes, such as `.enter`.

``` html
<!-- Explicit write enter's keycode -->
<input v-on:keyup.13="submit"></input>

<!-- Or use the alias -->
<input v-on:keyup.enter="submit"></input>
```

It's possible to define new key modifier aliases using the `Vue.config.keyCodes` object.

``` javascript
Vue.config.keycodes.f1 = 11;
```

Existing key modifiers are:

* `.enter`
* `.tab`
* `.delete` (also captures backspace)
* `.esc`
* `.space`
* `.up`
* `.down`
* `.left`
* `.right`

There are also modifiers for key modifiers:

* `.ctrl`
* `.alt`
* `.shift`
* `.meta`

There are also modifiers for mouse buttons:

* `.left`
* `.right`
* `.middle`

``` html
<!-- Alt + C -->
<input @keyup.alt.67="clear">

<!-- Ctrl + Click -->
<div @click.ctrl="doSomething">Do something</div>
```

# Form Bindings

The `v-model` directive can be used to create two-way data bindings on form `<input>` and `<textarea>` elements. These bindings automatically use the correct way to update the element based on its type.

Note that `v-model` ignores the initial `value`, `checked`, or `selected` attributes. More simply, Vue treats the instance data as the single source of truth, so all initial values should be specified in the `data` option.

``` html
<input v-model="message" placeholder="edit me">
<p>Message is: {{ message }}</p>
```

If multiple checkboxes bind to the same array, the effect is that for each checkbox that is checked, its `value` attribute is inserted into the array.

It's possible to set an element's value to something other than a string or boolean by using the `v-bind` directive with a `:value`, `:true-value`, or `:false-value` argument. This sets the `v-model` to the specified value.

``` html
<input
  type="checkbox"
  v-model="toggle"
  v-bind:true-value="a"
  v-bind:false-value="b">

<!-- For a radio button -->
<input type="radio" v-model="pick" v-bind:value="a">
```

``` javascript
// when checked:
vm.toggle === vm.a

// when unchecked:
vm.toggle === vm.b
```

The `.lazy` modifier makes the data synchronize on each `change` event instead of on each `input` event.

The `.number` modifier specifies to automatically convert the input string to a number.

The `.trim` modifier automatically trims the input string.
