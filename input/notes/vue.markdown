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
    firstName(val) {
      this.fullName = val + ' ' + this.lastName
    },
    lastName(val) {
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
    fullName() {
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

Vue also _cannot_ detect property addition or deletion, which is why all data properties must be defined upfront, even if to empty values. This also is why Vue cannot allow dynamically adding new root-level reactive properties. This _is_ possible to do on existing objects via `Vue.set()`:

``` javascript
Vue.set(vm.someObject, 'newProperty', someValue);

// Equivalent (in instance):
this.$set(this.someObject, 'newProperty', someValue);
```

When doing mass assignment via `Object.assign`, do so in an immutable fashion as in Redux, replacing the original property, instead of directly mutating the object.

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
<input v-on:keyup.alt.67="clear">

<!-- Ctrl + Click -->
<div v-on:click.ctrl="doSomething">Do something</div>
```

### Custom Events

Every Vue instance implements an events interface so that it can listen to an event via `$on(eventName)` and trigger an event via `$emit(eventName)`.

A parent component can listen to events emitted from a child via the `v-on` directive on that child component.

For example, the below example shows that the child component emits an event whenever it is clicked, which the parent component listens on to maintain a total count.

``` html
<div id="counter-event-example">
  <p>{{ total }}</p>
  <button-counter v-on:increment="incrementTotal"></button-counter>
  <button-counter v-on:increment="incrementTotal"></button-counter>
</div>
```

``` javascript
Vue.component('button-counter', {
  template: '<button v-on:click="incrementCounter">{{ counter }}</button>',
  data() {
    return {
      counter: 0,
    };
  },
  methods: {
    incrementCounter() {
      this.counter += 1;
      this.$emit('increment');
    },
  },
});

new Vue({
  el: '#counter-event-example',
  data: { total: 0 },
  methods: {
    incrementTotal() {
      this.total += 1;
    },
  },
});
```

A parent component can listen for a native event on a child component's root element by using the `.native` modifier for `v-on`.

``` html
<my-component v-on:click.native="doTheThing"></my-component>
```

It is possible to declare a prop to be a two-way binding by using the `.sync` modifier on `v-bind`. This actually has the effect of adding a `v-on` directive listening for the `update` event on the specified property, in which case it updates the property.

``` html
<!-- Declare prop `foo` to be a two-way binding to parent's `bar` property. -->
<comp v-bin:foo.sync="bar"></comp>

<!-- Expands to this.
     An explicit listener for the update event affecting the `foo` prop,
     which has the effect of setting parent's `bar` property to the new
     given value. -->
<comp v-bin:foo="bar" v-on:update:foo="val => bar = val"></comp>
```

To cause this update to occur, the child must explicitly emit the event rather than directly mutating the property.

``` javascript
// wrong
this.foo = newValue;

// correct
this.$emit('update:foo', newValue);
```

## Form Bindings

The `v-model` directive can be used to create two-way data bindings on form `<input>` and `<textarea>` elements. These bindings automatically use the correct way to update the element based on its type.

The `v-model` directive is essentially sugar for binding the variable to the input's value with `v-bind` and updating the variable whenever there is new input.

``` html
<input v-model="something">

<!-- Expands to -->
<input
  v-bind:value="something"
  v-on:input="something = $event.target.value">
```

Therefore, for a component to work with `v-model`, it should accept a `value` prop and emit an `input` event with the new value when appropriate.

It's possible to specify something other than `value` as the prop that is bound to and `input` as the event that is emitted by using the `model` option.

``` javascript
Vue.component('my-checkbox', {
  model: {
    // v-bind to prop `checked`
    prop: 'checked',

    // listen v-on `change`
    event: 'change',
  },
  props: {
    checked: Boolean,

    // now `value` can be used for something else
    value: String,
  },
});
```

``` html
<my-checkbox v-model="foo" value="some value"></my-checkbox>

<!-- Expands to -->
<my-checkbox
  v-bind:checked="foo"
  v-on:change="val => { foo = val }"
  value="some value">
</my-checkbox>
```

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

## Components

A component can be registered _globally_ with the `Vue.component()` method which takes a tag name and options object. The W3C rules for custom tag names are that they are all lowercase and must contain a hyphen; while a good practice, Vue does not enforce these rules.

``` html
<div id="example">
  <my-component></my-component>
</div>
```

``` javascript
Vue.component('my-component', {
  template: '<div>A custom component!</div>',
});

// create a root instance
new Vue({
  el: '#example',
});
```

A component can be registered locally for a particular Vue instance by specifying it in its `components` property.

``` javascript
var Child = {
  template: '<div>A custom component!</div>',
};

new Vue({
  components: {
    // only available in parent's template
    'my-component': Child,
  },
});
```

Since objects are passed by reference, the `data` property of a component should be defined as a function that returns an object, rather than as an object itself, so as to prevent having multiple instances of the component mutating the same data object.

The preferred method of component communication is "props down, events up," meaning that parent components should pass data down to child components via props, and child components may communicate with parents by emitting events that propagate up the component hierarchy.

<img src="https://vuejs.org/images/props-events.png" />

Since each component has its own isolated scope, parents can't directly access a child's data. Instead, parents may pass data to a child component via `props`. Each component explicitly declares which `props` it accepts via the `props` option property.

``` javascript
Vue.component('child', {
  props: ['message'],
  template: '<span>{{ message }}</span>',
});
```

``` html
<child message="hello!"></child>
```

Props can be dynamically bound to data on the parent with the `v-bind` directive.

``` html
<div>
  <input v-model="parentMsg">
  <br>
  <child v-bind:my-message="parentMsg"></child>
</div>
```

Prop values are interpreted as strings, unless they are applied to `v-bind` in which case they are interpreted as a JavaScript expression:

``` html
<!-- this passes down a plain string "1" -->
<comp some-prop="1"></comp>

<!-- this passes down an actual number -->
<comp v-bind:some-prop="1"></comp>
```

Mutating the parent's state is highly discouraged and may occur by inadvertently mutating an object or array passed via a prop, since both are passed by reference.

If the purpose of a prop is to serve as an initial value for an internal data property, it should be prefixed with the word `initial` and used to set the initial value of that data property:

``` javascript
props: ['initialCounter'],
data() {
  return { counter: this.initialCounter }
},
```

If the purpose of a prop is to read a raw value which the component uses to compute another, create a computed property for the target value:

``` javascript
props: ['size'],
computed: {
  normalizedSize() {
    return this.size.trim().toLowerCase();
  }
},
```

If a parent passes an attribute that is not a declared prop to a child component, the attribute is applied to the child's root element, replacing the root element's corresponding attribute if any is found. However, if the attribute in question is a `class` or `style` attribute, the values are merged with the corresponding attributes on the root element, instead of replacing them.

### Functional Components

A _functional component_ is a stateless component marked with the option property `functional`. It has no instance, it simply consists of a `render` function that takes `props` and `children` as arguments and returns one or more VNodes (as an array), unlike stateful components that can only return a single root node. It can also specify an optional `props` property, although all attributes found on the component node are implicitly extracted as props.

The `context` object passed to the render function has many properties:

* `props` object
* `children` array of VNodes
* `slots` function returning a slots object
* `data` the data object passed to the component
* `parent` reference to parent component
* `listeners` object of parent-registered event listeners (alias of `data.on`)
* `injections` containing the resolved injections

Note that functional components don't show up in Vue devtools since they lack a persistent instance.

``` javascript
Vue.component('wrap-with-tag', {
  functional: true,
  props: ['tag'],
  render (createElement, context) {
    return createElement(context.props.tag, null, context.children);
  },
});
```

``` html
<wrap-with-tag tag="div">hello</wrap-with-tag>

<!-- Produces -->
<div>hello</div>
```

### Validation

It's possible to define validation criteria for props by using an object instead of an array of prop names, where each key is the name of the prop and value is some validation criteria. That criteria can be a constructor function such as `String`, `Number`, or a custom one. An array of these can be used to specify multiple allowed types.

An object can be passed where the `type` property specifies the allowed types, `required` property declares that the prop must be specified, `default` property specifies a value or function yielding a default value if none is given, and `validator` property specifies a function that validates the actual value.

``` javascript
Vue.component('example', {
  props: {
    // basic type check (`null` means accept any type)
    propA: Number,

    // multiple possible types
    propB: [String, Number],

    // a required string
    propC: {
      type: String,
      required: true,
    },

    // a number with default value
    propD: {
      type: Number,
      default: 100,
    },

    // object/array defaults should be returned from a
    // factory function
    propE: {
      type: Object,
      default() {
        return { message: 'hello' };
      },
    },

    // custom validator function
    propF: {
      validator(value) {
        return value > 10;
      },
    },
  },
});
```

### Content Distribution

It's possible to parent content with a component's own template via a process called _content distribution_. Content placed within a child component by the parent is discarded unless the child component template contains at least one `<slot>` outlet.

If there is only one, without any attributes, then the entire content fragment is inserted at that position, replacing the slot itself. Anything inside of a `<slot>` tag is considered _fallback content_ which is rendered only if the hosting element is empty and has no content to be inserted.

``` html
<!-- Child -->
<div>
  <h2>I'm the child title</h2>
  <slot>
    This will only be displayed if there is no content
    to be distributed.
  </slot>
</div>

<!-- Parent -->
<div>
  <h1>I'm the parent title</h1>
  <my-component>
    <p>This is some original content</p>
    <p>This is some more original content</p>
  </my-component>
</div>

<!-- Rendered -->
<div>
  <h1>I'm the parent title</h1>
  <div>
    <h2>I'm the child title</h2>
    <p>This is some original content</p>
    <p>This is some more original content</p>
  </div>
</div>
```

It's also possible to name slots via `<slot>`'s `name` attribute in order to specify how content should be distributed. The hosting element declares that a given element will be distributed to a `<slot>` with a given `name` attribute by specifying that name via a `slot` attribute on an arbitrary element.

An unnamed, _default slot_ may coexist as a catch-all for any unmatched content. If there is no default slot, unmatched content is discarded.

``` html
<!-- Child -->
<div class="container">
  <header>
    <slot name="header"></slot>
  </header>
  <main>
    <slot></slot>
  </main>
  <footer>
    <slot name="footer"></slot>
  </footer>
</div>

<!-- Parent -->
<app-layout>
  <h1 slot="header">Here might be a page title</h1>
  <p>A paragraph for the main content.</p>
  <p>And another one.</p>
  <p slot="footer">Here's some contact info</p>
</app-layout>

<!-- Rendered -->
<div class="container">
  <header>
    <h1>Here might be a page title</h1>
  </header>
  <main>
    <p>A paragraph for the main content.</p>
    <p>And another one.</p>
  </main>
  <footer>
    <p>Here's some contact info</p>
  </footer>
</div>
```

_Scoped slots_ allow a child to expose data that the parent can use when filling that slot. To do this, the child simply passes the data via attributes on the `<slot>` tag, binding if necessary. The parent must then fill that slot using a `<template>` tag with a `scope` attribute naming an object that will be created and populated with the exposed data, then the data becomes available within that `<template>` as a property on that object.

``` html
<!-- Child -->
<div class="child">
  <slot text="hello from child"></slot>
</div>

<!-- Parent -->
<div class="parent">
  <child>
    <template scope="props">
      <span>hello from parent</span>
      <span>{{ props.text }}</span>
    </template>
  </child>
</div>

<!-- Rendered -->
<div class="parent">
  <div class="child">
    <span>hello from parent</span>
    <span>hello from child</span>
  </div>
</div>
```

This can be used to allow a parent scope to customized the way that a child component renders list items.

``` html
<!-- Child -->
<ul>
  <slot name="item"
    v-for="item in items"
    v-bind:text="item.text">
    <!-- fallback content -->
    <li>Default: {{ item.text }}</li>
  </slot>
</ul>

<!-- Parent -->
<my-awesome-list v-bind:items="items">
  <!-- scoped slot can be named too -->
  <template slot="item" scope="props">
    <li class="my-fancy-item">{{ props.text }}</li>
  </template>
</my-awesome-list>
```

It's possible to dynamically switch between multiple components mounted at the same point by using the reserved `<component>` element and dynamically binding its `is` attribute.

``` javascript
var vm = new Vue({
  el: '#example',
  data: { currentView: 'home' },
  components: {
    home: { … },
    posts: { … },
    archive: { … },
  },
});
```

``` html
<component v-bind:is="currentView">
  <!-- component changes when vm.currentView changes! -->
</component>
```

It's also possible to bind to component objects directly:

``` javascript
var Home = { template: '<p>Welcome home!</p>' };

var vm = new Vue({
  el: '#example',
  data: { currentView: Home }
});
```

Components can be marked to be kept in memory in order to preserve their state and avoid re-rendering by wrapping it in a `<keep-alive>` element.

``` html
<keep-alive>
  <component :is="currentView">
    <!-- inactive, switched-from components will be cached! -->
  </component>
</keep-alive>
```

It's possible to name a particular element and access it directly by using the `ref` attribute on that element and accessing it via the `$refs` instance property. If it's used together with `v-for`, the ref will be an array.

``` html
<div id="parent">
  <user-profile ref="profile"></user-profile>
</div>
```

``` javascript
var parent = new Vue({ el: '#parent' });

// access child component instance
var child = parent.$refs.profile;
```

It's possible to define a component as a factory function that is resolved asynchronously via a callback or  by returning a promise.

``` javascript
Vue.component('async-example', function (resolve, reject) {
  setTimeout(function () {
    // Pass the component definition to the resolve callback
    resolve({ template: '<div>I am async!</div>' });
  }, 1000);
});

Vue.component('async-promise-example', () => somePromise());
```

There is also a more advanced method of specifying asynchronous components by passing an options object that can specify the `component` as a promise, a component to use while it is `loading`, a `delay` before showing that `loading` component, a component to use in case of an `error`, and a `timeout` after which to show the `error` component.

``` javascript
const AsyncComponent = () => ({
  component: getSomeComponent(),
  loading: LoadingComp,
  error: ErrorComp,
  delay: 200,
  timeout: 3000,
});
```

Components can be recursive by specifying the `name` option for them to refer to themselves recursively. The `name` is automatically set when the component is registered globally.

Tools like Webpack may have difficulty importing cyclically-dependent components. For example, consider a `Folder` component that has a child component `FolderContents`, which itself may have a `Folder` component within it. Webpack would see that `Folder` imports `FolderContents`, but `FolderContents` needs to import `Folder`—a cyclic dependency. This can be remedied by deferring the import, usually on the `beforeCreate` lifecycle hook.

``` javascript
beforeCreate() {
  this.$options.components.FolderContents = require('./folder-contents').default;
}
```

The `inline-template` attribute can be applied to a child component so that its inner content is used as the child component's template (in the child's scope), rather than treating it as distributed content.

It's possible to ensure that static HTML is only evaluated once by applying the `v-once` attribute to the element.

## Transitions

The `<transition>` wrapper component can be used to add entering/leaving transitions to the contents for conditional rendering, conditional display, dynamic components, and component root nodes.

When a wrapped element is inserted or removed, Vue detects whether the element has CSS transitions or animations applied, and if so it adds or removes the CSS transition classes at the appropriate timings, and any JavaScript hooks are executed. Otherwise the DOM insertion/removal operations are executed on the next browser animation frame.

The possible transition classes include:

1. `v-enter`: Starting to enter, before insertion until one frame after the element is inserted.
2. `v-enter-active`: Applied during the entire entering phase, before insertion until transition finishes.
3. `v-enter-to`: Ending enter, one frame after insertion (right after `v-enter`) until transition finishes.
4. `v-leave`: Starting to leave, as soon as transition is triggered until one frame.
5. `v-leave-active`: Applied during the entire leaving phase, as soon as transition is triggered until transition finishes.
6. `v-leave-to`: Ending leave, one frame after transition is triggered (right after `v-leave` is removed) until transition finishes.

<img src="https://vuejs.org/images/transition.png" />

The classes that are added or removed are prefixed by the `<transition>`'s `name` attribute and the `v-` prefixed is removed, unless there is no `name` attribute. A `<transition>` with `name="my-transition"` becomes `my-transition-enter`.

The classes used can be overridden with attributes on the `<transition>` tag which are named after the class in question with a `-class` suffix, such as `enter-active-class`.

Vue registers event listeners for the animation end on `transitionend` or `animatinoend` depending on the type used. If both are animation types are used, it is necessary to declare to Vue which one to care about by using the `type` attribute on the `<transition>` tag with a value of `animation` or `transition`.

Vue automatically detects when a transition has finished by using the `transitionend` or `animationend` events. However, an explicit duration can be specified with the `duration` attribute, set to either an integer for milliseconds or an object with `enter` and/or `leave` properties specifying the enter or leave duration, respectively.

It's possible to register JavaScript hooks to run at different stages of the transition by using `v-on` with the name of the stage as an argument, e.g. `v-on:after-enter`. These can be used to create JavaScript animations. Each hook is passed the root element within the `<transition>`, while the `enter` and `leave` hooks are also passed a callback that should be invoked to signal the end of the phase.

If the `appear` attribute is present in the `<transition>`, the entering and leaving transitions are used when the element is rendered or is removed, or separate ones can be specified with an `appear-` prefix, such as `appear-active-class`. This can also be done for JavaScript hooks, such as `v-on:after-appear`.

Here's a complete example:

``` html
<div id="demo">
  <button v-on:click="show = !show">
    Toggle
  </button>
  <transition name="fade">
    <p v-if="show">hello</p>
  </transition>
</div>
```

``` javascript
new Vue({
  el: '#demo',
  data: { show: true },
});
```

``` css
.fade-enter-active, .fade-leave-active {
  transition: opacity .5s
}

.fade-enter, .fade-leave-to {
  opacity: 0
}
```

It's possible to use `<transition>` to transition between raw elements, such as between a `v-if` and its `v-else`, but care should be taken to add `key` attributes to each in the event that they each use the same tag.

``` html
<transition>
  <button v-if="isEditing" key="save">
    Save
  </button>
  <button v-else key="edit">
    Edit
  </button>
</transition>
```

In fact, that can be condensed further by binding the `key` attribute and using a ternary expression for the content:

``` html
<transition>
  <button v-bind:key="isEditing">
    {{ isEditing ? 'Save' : 'Edit' }}
  </button>
</transition>
```

Transitioning between custom components simply entails the use of a dynamic `<component>`.

By default, the entering and leaving transitions are simultaneous, meaning that if transitioning between two elements, one will begin entering at the same time that the other begins leaving. Other _transition modes_ include `in-out` where the current element transitions out until the new element has transitioned in, and `out-in` where the current element transitions out and _then_ the new element transitions in. These modes are specified via the `mode` attribute on the `<transition>` element.

The `<transition-group>` element can be used to render multiple items. Unlike `<transition>`, it renders a physical element which is `<span>` by default (for example, for a list it can be `<ul>`), and every element _must_ have a unique `key` attribute. The `<transition-group>` element also adds a `v-move` class to elements that are changing positions. This can be used in conjunction with a CSS `transform` transition so that Vue automatically uses [FLIP animations].

[FLIP animations]: https://aerotwist.com/blog/flip-your-animations/

## Mixins

Mixins are simply objects that contain component options which can be mixed into a component via its `mixins` property:

``` javascript
const mixin = {
  created() {
    this.hello();
  },
  methods: {
    hello() {
      console.log('hello from mixin!');
    },
  },
}

const Component = Vue.extend({
  mixins: [mixin],
});

new Component() // => "hello from mixin!"
```

Mixed-in hooks are called _before_ the components hooks. Otherwise-conflicting options are sensibly mixed, except when merging properties that expect object values, such as `methods`, in which case the component's corresponding property overrides the mixin's. When merging custom options, the existing value is overwritten unless a custom merge strategy is defined for that option.

``` javascript
Vue.config.optionMergeStrategies.myOption = function (toVal, fromVal) {
  // return mergedVal
}

// Or just use `methods`'s strategy.
Vue.config.optionMergeStrategies.myOption = Vue.config.optionMergeStrategies.methods;
```

Although discouraged, it's also possible inject mixins globally via `Vue.mixin()`.

## Custom Directives

Custom directives can be defined globally via the `Vue.directive()` method.

``` javascript
// Define a `v-focus` directive.
Vue.directive('focus', {
  // When the bound element is inserted into the DOM…
  inserted(el) {
    el.focus();
  }
})
```

Custom directives may also be defined locally within a component via the `directives` property.

``` javascript
directives: { focus: { … } }
```

``` html
<input v-focus>
```

A directive can hook into different points:

* `bind`: When the directive is first bound to the element.
* `inserted`: When the element is inserted into its parent node (just parent node presence—not necessarily in-document).
* `update`: When component's VNode has updated, possibly before its children VNodes have. The binding's old and new values can be checked to avoid unnecessary updates.
* `componentUpdated`: When component's VNode and the VNodes of its children have updated.
* `unbind`: When the directive is unbound from the element.

Each hook is passed three arguments:

1. The `element` the directive is bound to
2. The `binding` object.
3. The `vnode` (virtual node) of the element.

The `update` and `componentUpdated` hooks are also passed the `oldVnode`, the previous virtual node.

The `binding` object contains the following properties:

* `name` of the directive without the `v-` prefix
* `value` passed to the directive, specifically, the expression's value
* `oldValue` whether or not it changed (for `update` and `componentUpdated`)
* `expression` passed to the directive as a string
* `arg` passed to the directive, if any
* `modifiers`: an object mapping the provided modifiers to `true`

There exists a shorthand for registering a custom directive that only executes on `bind` and `update`:

``` javascript
Vue.directive('color-swatch', (el, binding) => {
  el.style.backgroundColor = binding.value;
});
```

## Render Functions

It's possible to write components that have explicit render functions instead of implicit ones via an associated templates.

``` javascript
Vue.component('anchored-heading', {
  render: function (createElement) {
    return createElement(
      'h' + this.level,   // tag name
      this.$slots.default // array of children
    );
  },
  props: {
    level: {
      type: Number,
      required: true,
    },
  },
});
```

The `createElement` function takes up to three arguments:

1. The tag name, component options, or function returning either.
2. An optional attributes object.
3. The text string or array of children VNodes.

A _virtual node_ is essentially a node description which. A _virtual DOM_ is a tree of VNodes. Each VNode must be unique.

When defining event handlers directly within a component's `on` data object property, modifiers can be leveraged by using certain prefixes on the handler's name:

| Modifiers       | Prefix |
|-----------------|--------|
| `.passive`      | `&`    |
| `.capture`      | `!`    |
| `.once`         | `~`    |
| `.capture.once` | `~!`   |

``` javascript
on: {
  '!click': this.captureMode,
  '~keyup': this.doOnce,
  `~!mouseover`: this.doOnceInCaptureMode,
},
```

## Plugins

There are different types of plugins:

1. Some add some global methods or properties
2. Some add one or more global assets (directives, filters, transitions)
3. Some add some component options via a global mixin
4. Some add some Vue instance methods via `Vue`'s prototype
5. Some act as libraries with an API of their own

A plugin should expose an `install` method which is called with the `Vue` constructor as the first argument and possibly other options.

``` javascript
MyPlugin.install = function (Vue, options) {
  // add global method or property (1)
  Vue.myGlobalMethod = function () { … };

  // or add a global asset (2)
  Vue.directive('my-directive', {
    bind (el, binding, vnode, oldVnode) { … },
  });

  // or inject some component options (3)
  Vue.mixin({ created() { … } });

  // or add an instance method (4)
  Vue.prototype.$myMethod = function (methodOptions) { … },
};
```

A plugin is used via `Vue.use()` with an optional options argument. This automatically guards against loading the same plugin more than once.

``` javascript
// calls `MyPlugin.install(Vue)`
Vue.use(MyPlugin)

// with options:
Vue.use(MyPlugin, { someOption: true })
```

## Filters

Filters can be used in mustache interpolations and `v-bind` expressions. Filters can be chained.

``` html
<!-- in mustaches -->
{{ message | capitalize }}

<!-- in v-bind -->
<div v-bind:id="rawId | formatId"></div>
```

Custom filters can be defined in the component's `filters` property, which is an object mapping filter names to functions that handle them. Each filter is passed the expression's value.

``` javascript
filters: {
  capitalize(value) {
    if (!value) return '';

    value = value.toString();

    return value.charAt(0).toUpperCase() + value.slice(1);
  },
},
```

Since filters are just functions, they can be defined to take additional arguments, however, the filtered value is always the first argument.

## Reactivity

Given a passed-in `data` option, Vue automatically traverses the object and converts each property into getters and setters via `Object.defineProperty`, allowing Vue to inject behavior such as threading dependency-tracking and change-notification when properties are accessed or modified. This is why vue-devtools may be more useful for inspecting properties, since browser log functions will output the getters and setters themselves.

Each component instance has a watcher instance that records as dependencies any properties that are touched during the component's render. Then when a dependency's setter is triggered it notifies the watcher which then causes a re-render.

<img src="https://vuejs.org/images/data.png" />

Vue debounces multiple watcher triggers within the same event loop iteration, flushing the queue of changes on the next tick. It's possible to do some work until after Vue has performed the DOM updates by registering a callback to run on the next tick via `Vue.nextTick()`.

``` html
<div id="example">{{ message }}</div>
```

``` javascript
const vm = new Vue({
  el: '#example',
  data: { message: '123' },
});

vm.message = 'new message';
vm.$el.textContent === 'new message'; // => false

Vue.nextTick(function () {
  vm.$el.textContent === 'new message'; // => true
});

// Or within an instance:
this.$nextTick(function () {
  console.log(this.$el.textContent);
});
```

## Error Handling

The `Vue.config.errorHandler` property can be set to a function that will receive any emitted errors.

## Vuex

Like Redux, Vue has a notion of a store. A store in Vue is reactive and is mutated by committing mutations.

``` javascript
import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex);

const store = new Vuex.Store({
  state: { count: 0 },
  mutations: {
    increment(state) {
      state.count++
    },
  },
});

// …
store.commit('increment');
store.state.count // => 1
```

Besides accessing a global Vuex store within a component, Vuex allows for the injection of the store into all child components of a given component by simply specifying the store to inject as the `store` component option. This makes the store available via the `this.$store` instance property.

``` javascript
const app = new Vue({
  el: '#app',
  store,
  components: { Counter },
  template: `
    <div class="app">
      <counter></counter>
    </div>
  `,
});

const Counter = {
  template: `<div>{{ count }}</div>`,
  computed: {
    count() {
      return this.$store.state.count;
    },
  },
};
```

Store state is usually wrapped in a component's computed property, so that the property is recomputed automatically when the state changes. The `mapState()` function helps with this repetitive task. If passed an array, it creates `computed` properties for each state property.

``` javascript
computed: mapState({
  count: (state) => state.count,

  // String 'count' is the same as `(state) => state.count`
  countAlias: 'count',

  // Use a normal function in order to access local state with `this`.
  countPlusLocalState(state) {
    return state.count + this.localCount;
  },
});

// Map this.count to store.state.count.
computed: mapState(['count']);
```

The object spread operator can be used to mix Vuex' `mapState()` with local computed properties.

``` javascript
computed: {
  localComputed() { … },
  ...mapState({ … }),
},
```

### Getters

The store can define _getters_ which are essentially computed properties defined on the store itself. Like computed properties, these only re-evaluate when any dependencies have changed. Getters are passed the state as the first argument and all getters as the second argument.

``` javascript
const store = new Vuex.Store({
  state: {
    todos: [{ id: 1, text: '...', done: true }],
  },
  getters: {
    doneTodos: (state, getters) => state.todos.filter((todo) => todo.done),
  },
});
```

Getters can be used within components by accessing them directly.

``` javascript
computed: {
  doneTodosCount() {
    return this.$store.getters.doneTodosCount;
  },
},
```

A getter can return a closure in order to take arbitrary arguments.

``` javascript
getters: {
  getTodoById: (state, getters) => (id) => {
    return state.todos.find((todo) => todo.id === id);
  },
},

store.getters.getTodoById(1); // => { id: 1, text: '...', done: true }
```

Like the `mapSetters()` helper function, the `mapGetters()` function can be used to map local `computed` properties to a store's getters by passing an array with the name of the store's getter, or an object mapping the desired local computed property name to the name of the store's getter.

``` javascript
computed: {
  ...mapGetters(['doneTodosCount']),

  // or
  ...mapGetters({ doneCount: 'doneTodosCount' }),
},
```

### Mutations

State in a store can only be changed by committing a mutation. Each mutation has a `type` named by a string and a `handler` function which actually performs the state modifications. The handler function is passed the state object as the first argument and an optional additional `payload` argument.

Mutations _must_ be synchronous.

``` javascript
const store = new Vuex.Store({
  state: {
    count: 1
  },
  mutations: {
    increment(state) {
      state.count++;
    },
  },
});

// invoke as
store.commit('increment');
```

When a mutation needs additional arguments, it should be done by passing an object as the `payload`.

``` javascript
mutations: {
  increment(state, payload) {
    state.count += payload.amount;
  },
},

// invoke as
store.commit('increment', { amount: 10 });
```

It's also possible to commit a mutation with a single object argument which specifies the mutation via a `type` property. This is known as _object-style_ commit.

``` javascript
store.commit({ type: 'increment', amount: 10 });
```

Since Vuex state is reactive it has the same [reactivity caveats]. All fields should be defined upfront, and any new properties on a nested object should either use `Vue.set()` or should result in a new nested object that replaces the original via `Object.assign()` or the spread operator.

[reactivity caveats]: #reactivity-caveats

Components can commit mutations by using `this.$store.commit()` directly or by using the `mapMutations()` helper function which maps component `methods` to store mutations either by a simple array mapping names one-to-one or with an object which can be used to specify the local name. This works whether or not the mutation takes a payload.

``` javascript
methods: {
  ...mapMutations(['increment', 'incrementBy']),

  // Map `this.add()` to `this.$store.commit('increment')`.
  ...mapMutations({ add: 'increment' }),
},
```

### Actions

Actions are possibly-asynchronous functions that commit mutations. Actions are passed a `context` object which exposes the `commit()` function, the `state`, and the `getters`.

Whereas mutations are triggered via `commit()`, actions are triggered via `dispatch()`.

Like mutations, actions support payload format and object-style dispatch.

``` javascript
actions: {
  increment(context) {
    context.commit('increment')
  },
},
```

Components can dispatch actions by directly accessing `this.$store.dispatch()` or by using the `mapActions()` helper function to map component `methods` to dispatch calls, much like `mapMutations()`.

``` javascript
methods: {
  ...mapActions(['increment', 'incrementBy']),

  // Map `this.increment()` to `this.$store.commit('increment)`
  ...mapActions({ add: 'increment' })
}
```

An action can return a promise, which is also returned by `dispatch()`.

``` javascript
actions: {
  asyncAction({ commit }) {
    commit('someMutation');

    return Promise.resolve();
  },
  otherAsyncAction({ dispatch, commit }) {
    return dispatch('asyncAction').then(() => commit('otherMutation'));
  },
},
```

### Modules

A store can be divided into separate `modules`, each with its own state, mutations, actions, getters, and even other modules within it.

``` javascript
const moduleA = {
  state: { … },
  mutations: { … },
  actions: { … },
  getters: { … },
};

const moduleB = {
  state: { … },
  mutations: { … },
  actions: { … },
};

const store = new Vuex.Store({
  modules: {
    a: moduleA,
    b: moduleB,
  },
});

store.state.a; // -> `moduleA`'s state
store.state.b; // -> `moduleB`'s state
```

A module's mutations and getters are passed the _module's_ local state. Module actions get the _module's_ local state via `context.state`, but the store's root state is also accessible via `context.rootState`. Module getters can access the root state via the third argument.

Although a module's state is nested under a state property named after the module, actions, mutations, and getters are registered under the global namespace by default, allowing multiple modules to react to the same mutation or action type (when this happens, asynchronous actions yield a Promise that resolves when _all_ triggered handlers have resolved).

This can be avoided by marking the module as `namespaced`, then all getters, actions, and mutations are namespaced under the module name with a forward slash `/` separating namespace components.

``` javascript
modules: {
  account: {
    namespaced: true,

    // module state is already nested
    state: { … },

    // -> getters['account/isAdmin']
    getters: { isAdmin () { … } },

    // -> dispatch('account/login')
    actions: { login () { … } },

    // -> commit('account/login')
    mutations: { login () { … } },

    // nested modules
    modules: {
      // inherits the namespace from parent module
      myPage: {
        state: { … },

        // -> getters['account/profile']
        getters: { profile () { … } },
      },

      // further nest the namespace
      posts: {
        namespaced: true,

        state: { … },

        // -> getters['account/posts/popular']
        getters: { popular () { … } },
      },
    },
  },
},
```

Namespaced getters and actions receive _localized_ `getters`, `dispatch`, and `commit`, that is, ones that act upon the namespaced module itself. Global state and getters can be accessed via `rootState` and `rootGetters` which are passed as third and fourth arguments to getter functions and exposed via the `context` object in action functions. Given a localized `dispatch` or `commit` function, global actions or commits can be invoked by passing `{ root: true }` as the third argument.

``` javascript
modules: {
  foo: {
    namespaced: true,

    getters: {
      someOtherGetter: (state) => { … },

      someGetter(state, getters, rootState, rootGetters) {
        // -> 'foo/someOtherGetter'
        getters.someOtherGetter;

        // -> 'someOtherGetter'
        rootGetters.someOtherGetter;
      },
    },

    actions: {
      someAction({ dispatch, commit, getters, rootGetters }) {
        // -> 'foo/someGetter'
        getters.someGetter;

        // -> 'someGetter'
        rootGetters.someGetter;

        // -> 'foo/someOtherAction'
        dispatch('someOtherAction')

        // -> 'someOtherAction'
        dispatch('someOtherAction', null, { root: true })

        // -> 'foo/someMutation'
        commit('someMutation')

        // -> 'someMutation'
        commit('someMutation', null, { root: true })
      },
      someOtherAction(ctx, payload) { … }
    },
  },
},
```

The helper functions `mapState()`, `mapGetters()`, `mapActions()`, and `mapMutations()` each can take a namespace string as the first argument so that all bindings are done in that module's context.

``` javascript
computed: {
  ...mapState({
    a: (state) => state.some.nested.module.a,
    b: (state) => state.some.nested.module.b,
  }),
},

// or
computed: {
  ...mapState('some/nested/module', {
    a: (state) => state.a,
    b: (state) => state.b,
  }),
},

methods: {
  ...mapActions([
    'some/nested/module/foo',
    'some/nested/module/bar',
  ]),
},

// or
methods: {
  ...mapActions('some/nested/module', [
    'foo',
    'bar',
  ]),
},
```

In fact, namespaced helpers can be created with `createNamespacedHelpers()`, which returns an object with each helper method already bound to the given namespace.

``` javascript
import { createNamespacedHelpers } from 'vuex';

const { mapState, mapActions } = createNamespacedHelpers('some/nested/module');

```

Modules can be dynamically registered via the store's `registerModule()` method, which takes the module name or an array that includes its namespace components, and the module itself. A dynamically registered module can be unregistered dynamically with the store's `unregisterModule()` method.

Modules can be made safely reusable by ensuring that the `state` is a function that returns the state object, similar to a component's `data` property.

Plugins can be registered which can subscribe to mutations.

``` javascript
// Called on store initialization.
const myPlugin = (store) => {
  // Called after every mutation.
  store.subscribe((mutation, state) => {
    // The mutation is object-style: `{ type, payload }`.
  });
}

const store = new Vuex.Store({
  plugins: [myPlugin],
  …,
});
```

Plugins can commit mutations, allowing them to be used to sync a data source with the store.

Strict mode causes an error to be thrown when state mutation occurs outside of mutation handlers, and can be enabled on a store by setting its `strict` property.

Using a Vuex store state value as a form input model with `v-model` is incorrect since that would make Vue mutate the state directly instead of through mutations. Instead, a two-way computed property should be created to wrap around the state.

``` html
<input v-model="message">
```

``` javascript
computed: {
  message: {
    get() {
      return this.$store.state.obj.message;
    },
    set(value) {
      this.$store.commit('updateMessage', value);
    },
  },
},
```

The focus on unit testing a Vuex store should be on mutations and actions. Testing mutations simply entails importing them and passing them a mock state object. Testing actions can leverage [inject-loader] in order to mock API calls.

[inject-loader]: https://github.com/plasticine/inject-loader
