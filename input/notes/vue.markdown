---
title = "Vue.js"
published = "August 26, 2017"
excerpt = "The Vue.js Library"
comments = false
---

## Vue Instance

The `Vue` function can be used to create a Vue instance, which by convention is often named `vm` for "view model." The `Vue` function takes an options object.

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

Modifiers are denoted by a dot `.` prefix.

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
