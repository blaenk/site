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
