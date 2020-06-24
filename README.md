`StaticTypeMap`, a type map for types that impl `Any`
=====================================================

This crates provides a `StaticTypeMap` which is a wrapper over `HashMap<TypeId, Box<dyn Any>>`.
It allows you to store a single instance of all types that implement
[`Any`](https://doc.rust-lang.org/std/any/trait.Any.html).
