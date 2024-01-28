# Chapter 8 - Common Collections

- Common collections are built-in data structures that are used to store collections of related data on the heap (variable size)
- The most common collections are:
    - Vectors: store a variable number of values next to each other
    - Strings: store a collection of characters
    - Hash Maps: store a key with an associated value
- The correct way to access elements in a vector is by using the `get` method, which returns an `Option<&T>` (None if the index is out of bounds)
- A vecror can only store values of the same type
- The `enum` type allows you to define a type by enumerating its possible variants
- To access a value in a enum you can use the `match` expression

- Strings
 - Can be represented as a collection of bytes, a collection of scalar values (chars), or a collection of grapheme clusters
 - Grapheme clusters are what a person would call a letter

 - Hash Maps
 - A hash map allows you to associate a value with a particular key
- to retrieve a value from a hash map, you can use the `get` method, which returns an `Option<&V>`
