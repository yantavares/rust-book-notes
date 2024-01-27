# Chapter 8 - Common Collections

- Common collections are built-in data structures that are used to store collections of related data on the heap (variable size)
- The most common collections are:
    - Vectors: store a variable number of values next to each other
    - Strings: store a collection of characters
    - Hash Maps: store a key with an associated value
- The correct way to access elements in a vector is by using the `get` method, which returns an `Option<&T>` (None if the index is out of bounds)
