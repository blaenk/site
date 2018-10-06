+++
title = "GraphQL"
date = 2018-05-24

[note]
kind = "technology"
+++

GraphQL is a query language for APIs. It can be used to describe queries and mutations which do exactly what the consumer wants, in a way that mirrors the way the data is returned by the API, in a way that removes the need for multiple round-trip requests.

<nav id="toc"></nav>

# Queries

Queries have the same shape as their result.

``` graphql
{
  hero {
    name
  }
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2"
    }
  }
}
```

Sub-selection of fields can be made in queries:

``` graphql
{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "friends": [
        {
          "name": "Luke Skywalker"
        },
        {
          "name": "Han Solo"
        },
        {
          "name": "Leia Organa"
        }
      ]
    }
  }
}
```

## Arguments

Fields can take arguments, even scalar fields. Arguments themselves may be of any type.

``` graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```

``` json
{
  "data": {
    "human": {
      "name": "Luke Skywalker",
      "height": 5.6430448
    }
  }
}
```

## Aliases

Aliases can be used to query the same field with different arguments:

``` graphql
{
  empireHero: hero(episode: EMPIRE) {
    name
  }
  jediHero: hero(episode: JEDI) {
    name
  }
}
```

``` json
{
  "data": {
    "empireHero": {
      "name": "Luke Skywalker"
    },
    "jediHero": {
      "name": "R2-D2"
    }
  }
}
```

## Fragments

Fragments can name a set of fields to allow them to be reused within queries:

``` graphql
{
  leftComparison: hero(episode: EMPIRE) {
    ...comparisonFields
  }
  rightComparison: hero(episode: JEDI) {
    ...comparisonFields
  }
}

fragment comparisonFields on Character {
  name
  appearsIn
  friends {
    name
  }
}
```

``` json
{
  "data": {
    "leftComparison": {
      "name": "Luke Skywalker",
      "appearsIn": [
        "NEWHOPE",
        "EMPIRE",
        "JEDI"
      ],
      "friends": [
        {
          "name": "Han Solo"
        },
        {
          "name": "Leia Organa"
        },
        {
          "name": "C-3PO"
        },
        {
          "name": "R2-D2"
        }
      ]
    },
    "rightComparison": {
      "name": "R2-D2",
      "appearsIn": [
        "NEWHOPE",
        "EMPIRE",
        "JEDI"
      ],
      "friends": [
        {
          "name": "Luke Skywalker"
        },
        {
          "name": "Han Solo"
        },
        {
          "name": "Leia Organa"
        }
      ]
    }
  }
}
```

## Operation Name

Previous syntax omits the `query` keyword specifying the operation type and the query name specifying the operation name. Operation types can be a query, mutation, or subscription.

``` graphql
query HeroNameAndFriends {
  hero {
    name
    friends {
      name
    }
  }
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "friends": [
        {
          "name": "Luke Skywalker"
        },
        {
          "name": "Han Solo"
        },
        {
          "name": "Leia Organa"
        }
      ]
    }
  }
}
```

## Variables

Queries can be given arguments, preventing the need to interpolate parameters into query strings, similar to prepared statements in SQL. Variables can be optional or required, and must be scalars, enums, or input types. Default values can be assigned after the type declaration.

``` graphql
query HeroNameAndFriends($episode: Episode = "JEDI") {
  hero(episode: $episode) {
    name
    friends {
      name
    }
  }
}
```

``` json
{
  "episode": "JEDI"
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "friends": [
        {
          "name": "Luke Skywalker"
        },
        {
          "name": "Han Solo"
        },
        {
          "name": "Leia Organa"
        }
      ]
    }
  }
}
```

## Directives

Directives can be attached to a filed or fragment inclusion to affect the execution of the query. Core GraphGL defines two directives:

* `@include(if: Boolean)` - only include this field if the argument is true
* `@skip(if: Boolean)` - skip this field if the argument is true

# Mutations

By convention, operations that cause writes should be sent explicitly via a mutation. The main distinction between queries and mutations beyond the name is that query fields are executed in parallel, whereas mutation fields are run in series. For example, if two `incrementCredits` mutations are sent in one request, the first one is guaranteed to finish before the second begins.

``` graphql
mutation CreateReviewForEpisode($ep: Episode!, $review: ReviewInput!) {
  createReview(episode: $ep, review: $review) {
    stars
    commentary
  }
}
```

``` json
{
  "ep": "JEDI",
  "review": {
    "stars": 5,
    "commentary": "This is a great movie!"
  }
}
```

``` json
{
  "data": {
    "createReview": {
      "stars": 5,
      "commentary": "This is a great movie!"
    }
  }
}
```

## Inline Fragments

Inline fragments are used to access the data of an underlying concrete type of a returned interface or union type. Named fragments can also be used like this.

Below, the `hero` field may return a `Character` interface of type `Human` or `Droid` depending on the `episode` argument. With direct selection, only fields that exist on the `Character` interface, such as `name`. A type condition of form `... on Type` is used with an inline fragment to access fields from a concrete type. For example, below the `primaryFunction` field will only be executed if the `Character` is a `Droid` type.

``` graphql
query HeroForEpisode($ep: Episode!) {
  hero(episode: $ep) {
    name
    ... on Droid {
      primaryFunction
    }
    ... on Human {
      height
    }
  }
}
```

``` json
{
  "ep": "JEDI"
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "primaryFunction": "Astromech"
    }
  }
}
```

The `__typename` meta field can be used to get the name of the object type at that point. It is particularly useful for returned union types.

``` graphql
{
  search(text: "an") {
    __typename
    ... on Human {
      name
    }
    ... on Droid {
      name
    }
    ... on Starship {
      name
    }
  }
}
```

``` json
{
  "data": {
    "search": [
      {
        "__typename": "Human",
        "name": "Han Solo"
      },
      {
        "__typename": "Human",
        "name": "Leia Organa"
      },
      {
        "__typename": "Starship",
        "name": "TIE Advanced x1"
      }
    ]
  }
}
```

# Schema

Queries are validated and executed against the schema.

## Objects

An object type may look like this. A non-nullable type is represented with an exclamation mark `!` suffix. Arrays are represented with bracket `[]` syntax.

``` graphql
type Character {
  name: String!
  appearsIn: [Episode]!
}
```

Every field in an object can have arguments which are always named and may have a default value.

``` graphql
type Starship {
  id: ID!
  name: String!
  length(unit: LengthUnit = METER): Float
}
```

## Query and Mutation Types

The schema contains two special types `Query` and `Mutation` which are regular object types which define the entry point of every GraphQL query and mutation. The fields contained within them define the root query and mutation fields.

They are only special in being entry points into the schema, but are otherwise the same as any other object type. They can be named anything else, but are named `Query` and `Mutation` by convention.

``` graphql
schema {
  query: Query
  mutation: Mutation
}

type Query {
  hero(episode: Episode): Character
  droid(id: ID!): Droid
}
```

``` graphql
query {
  hero {
    name
  }
  droid(id: "2000") {
    name
  }
}
```

## Scalars

The default scalar types include:

| Type | Definition |
| :--  | :--        |
| `Int` | 32-bit signed integer |
| `Float` | signed double-precision floating-point value |
| `String` | UTF-8 string |
| `Boolean` | true or false |
| `ID` | serialized as `String`, but not intended to be human-readable |

Most GraphQL implementations have a way to create custom scalar types. The implementation then defines how the type should be serialized, deserialized, and validated.

``` graphql
scalar Date
```

## Enumerations

In GraphQL enumerations are a set of pre-defined values. It's up to the implementation to determine how they're represented.

## Type Modifiers

A type can be marked as non-null by adding the exclamation mark `!` suffix. During validation, if the server gets a null value for a non-null field, it will trigger a GraphQL execution error. It can also be applied to a field argument.

``` graphql
myField: [String!]

myField: null // valid
myField: [] // valid
myField: ['a', 'b'] // valid
myField: ['a', null, 'b'] // error

myField: [String]!

myField: null // error
myField: [] // valid
myField: ['a', 'b'] // valid
myField: ['a', null, 'b'] // valid
```

## Interfaces

An interface is an abstract type that includes a certain set of fields that a type must include to satisfy it, but they may also include additional fields.

``` graphql
interface Character {
  id: ID!
  name: String!
  friends: [Character]
  appearsIn: [Episode]!
}

type Human implements Character {
  id: ID!
  name: String!
  friends: [Character]
  appearsIn: [Episode]!
  starships: [Starship]
  totalCredits: Int
}

type Droid implements Character {
  id: ID!
  name: String!
  friends: [Character]
  appearsIn: [Episode]!
  primaryFunction: String
}
```

``` graphql
query HeroForEpisode($ep: Episode!) {
  hero(episode: $ep) {
    name
    ... on Droid {
      primaryFunction
    }
  }
}
```

``` json
{
  "ep": "JEDI"
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "primaryFunction": "Astromech"
    }
  }
}
```

## Unions

The members of a union type need to be concrete object types. Unions can't be created from interfaces or other unions.

``` graphql
union SearchResult = Human | Droid | Starship
```

Inline fragments can be used to query any of the union members.

``` graphql
{
  search(text: "an") {
    ... on Human {
      name
      height
    }
    ... on Droid {
      name
      primaryFunction
    }
    ... on Starship {
      name
      length
    }
  }
}
```

``` json
{
  "data": {
    "search": [
      {
        "name": "Han Solo",
        "height": 1.8
      },
      {
        "name": "Leia Organa",
        "height": 1.5
      },
      {
        "name": "TIE Advanced x1",
        "length": 9.2
      }
    ]
  }
}
```

## Input Types

Input types look the same as regular object types but use `input` instead of `type`. The fields of an input type can refer to other input types, but can't mix input and output types, nor can they have arguments on their fields.

According to [the standard](https://facebook.github.io/graphql/October2016/#sec-Input-Objects), input types are necessary because object types may contain fields that express circular references or references to interfaces and unions, which aren't appropriate as an input argument.

Input types are used as query parameters, whereas output types are obtained after query execution.

Naming input types with an `Input` suffix is a common convention due to the common practice of having regular output object types and a corresponding input type.

``` graphql
input ReviewInput {
  stars: Int!
  commentary: String
}

mutation CreateReviewForEpisode($ep: Episode!, $review: ReviewInput!) {
  createReview(episode: $ep, review: $review) {
    stars
    commentary
  }
}
```

``` json
{
  "ep": "JEDI",
  "review": {
    "stars": 5,
    "commentary": "This is a great movie!"
  }
}
```

``` json
{
  "data": {
    "createReview": {
      "stars": 5,
      "commentary": "This is a great movie!"
    }
  }
}
```

# Execution

Each field on each type is backed by a function called a resolver. When a field is executed, the resolver is called to produce the next value. Execution stops when the query reaches the leaves of the query corresponding to scalar values.

At the top level of a GraphQL server schema is a type known as the Root type or Query type that represents all possible entry points into the API.

## Resolvers

The resolver function for a `human` field on the `Query` type may look like this. The `obj` parameter refers to the previous/parent object, which may not be used on the root type. The `args` parameter contains the arguments provided to the field in the query. The `context` value is provided to every resolver and holds contextual information such as the currently logged in user or database connection.

``` javascript
Query: {
  human(obj, args, context) {
    return context.db.loadHumanByID(args.id).then(
      userData => new Human(userData)
    )
  }
}
```

The resolver function for a `Human` object's `name` field may look like this. In this case, the `obj` argument is the `new Human` returned from the previous `Human` field. GraphQL libraries may assume missing resolvers to simply be properties of the same name that should be read and returned, as is the case with the `name` field resolver.

``` javascript
Human: {
  name(obj, args, context) {
    return obj.name
  }
}
```

The type system may use scalar coercion to convert values returned by a resolver function into something that matches the API, such as numbers used internally being coerced into enumeration values.

GraphQL waits for all promises concurrently before continuing, even those found in a list.

As each scalar field is resolved, it's placed into a key-value map with the field name or alias as the key and the resolved value as the value, in a bottom-up fashion back up to the root Query type.

# Introspection

The `__schema` field is always available on the root type of a `Query` and can be queried to obtain the schema of an API. The `types { name }` query includes the defined types, the built-in scalars, and the types that are part of the introspection system.

``` graphql
{
  __schema {
    types {
      name
    }
  }
}
```

``` json
{
  "data": {
    "__schema": {
      "types": [
        {
          "name": "Query"
        },
        {
          "name": "Episode"
        },
        {
          "name": "Character"
        },
        {
          "name": "ID"
        },
        {
          "name": "String"
        },
        {
          "name": "Int"
        },
        {
          "name": "FriendsConnection"
        },
        {
          "name": "FriendsEdge"
        },
        {
          "name": "PageInfo"
        },
        {
          "name": "Boolean"
        },
        {
          "name": "Review"
        },
        {
          "name": "SearchResult"
        },
        {
          "name": "Human"
        },
        {
          "name": "LengthUnit"
        },
        {
          "name": "Float"
        },
        {
          "name": "Starship"
        },
        {
          "name": "Droid"
        },
        {
          "name": "Mutation"
        },
        {
          "name": "ReviewInput"
        },
        {
          "name": "__Schema"
        },
        {
          "name": "__Type"
        },
        {
          "name": "__TypeKind"
        },
        {
          "name": "__Field"
        },
        {
          "name": "__InputValue"
        },
        {
          "name": "__EnumValue"
        },
        {
          "name": "__Directive"
        },
        {
          "name": "__DirectiveLocation"
        }
      ]
    }
  }
}
```

The different types of queries can be obtained with the `queryType` field.

``` graphql
{
  __schema {
    queryType {
      name
    }
  }
}
```

A specific type can be queried:

``` graphql
{
  __type(name: "Droid") {
    name
    kind
  }
}
```

Fields can be queried with the `fields` field.

``` graphql
{
  __type(name: "Droid") {
    name
    fields {
      name
      type {
        name
        kind
      }
    }
  }
}
```

A list kind is the `LIST` wrapper type. The `ofType` field can be used to obtain the wrapped type.

``` graphql
{
  __type(name: "Droid") {
    name
    fields {
      name
      type {
        name
        kind
        ofType {
          name
          kind
        }
      }
    }
  }
}
```

A type's documentation can be obtained with the `description` field.

``` graphql
{
  __type(name: "Droid") {
    name
    description
  }
}
```

``` json
{
  "data": {
    "__type": {
      "name": "Droid",
      "description": "An autonomous mechanical character in the Star Wars universe"
    }
  }
}
```

# Best Practices

## Versioning

GraphQL doesn't really need to be versioned because clients only request what they request, and new types and fields without breaking existing clients.

## Batching

Naive resolvers may incur many redundant queries. A common optimization is to debounce and batch them into single requests by using something like [DataLoader] or [join-monster].

[DataLoader]: https://github.com/facebook/dataloader
[join-monster]: https://github.com/stems/join-monster

## Pagination

A suggested way to implement pagination is to use cursors at an additional level of indirection representing the edge between a type and its list of associated items. The edge would return the item itself within a `node` field as well as a `cursor` field. The plural field can contain cumulative information such as `totalCount`, as well as a `pageInfo` field which can contain information about the `startCursor` and `endCursor` on that page, as well as whether it `hasNextPage` to avoid the final query.

This is known as a [Relay Cursor Connection].

[Relay Cursor Connection]: https://facebook.github.io/relay/graphql/connections.htm

``` graphql
{
  hero {
    name
    friendsConnection(first:2 after:"Y3Vyc29yMQ==") {
      totalCount
      edges {
        node {
          name
        }
        cursor
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }
}
```

``` json
{
  "data": {
    "hero": {
      "name": "R2-D2",
      "friendsConnection": {
        "totalCount": 3,
        "edges": [
          {
            "node": {
              "name": "Han Solo"
            },
            "cursor": "Y3Vyc29yMg=="
          },
          {
            "node": {
              "name": "Leia Organa"
            },
            "cursor": "Y3Vyc29yMw=="
          }
        ],
        "pageInfo": {
          "endCursor": "Y3Vyc29yMw==",
          "hasNextPage": false
        }
      }
    }
  }
}
```

# GraphQL.js

A minimal GraphQL servers looks like this.

``` javascript
var { graphql, buildSchema } = require('graphql');

// Construct a schema, using GraphQL schema language
var schema = buildSchema(`
  type Query {
    hello: String
  }
`);

// The root provides a resolver function for each API endpoint
var root = {
  hello: () => {
    return 'Hello world!';
  },
};

// Run the GraphQL query '{ hello }' and print out the response
graphql(schema, '{ hello }', root).then((response) => {
  console.log(response);
});
```

It's possible and straightforward to define an object type resolver as an instance of an ES6 class such that its instance methods and properties are resolvers.

``` graphql
type RandomDie {
  numSides: Int!
  rollOnce: Int!
  roll(numRolls: Int!): [Int]
}

type Query {
  getDie(numSides: Int): RandomDie
}
```

``` javascript
class RandomDie {
  constructor(numSides) {
    this.numSides = numSides;
  }

  rollOnce() {
    return 1 + Math.floor(Math.random() * this.numSides);
  }

  roll({numRolls}) {
    var output = [];
    for (var i = 0; i < numRolls; i++) {
      output.push(this.rollOnce());
    }
    return output;
  }
}

var root = {
  getDie: function ({numSides}) {
    return new RandomDie(numSides || 6);
  }
}
```
