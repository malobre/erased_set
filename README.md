`StaticTypeMap`, a type map for types that impl `Any`
=====================================================
![Rust Build, Test &
Publish](https://github.com/malobre/static_type_map/workflows/Rust%20Build,%20Test%20&%20Publish/badge.svg)

This crates provides a `StaticTypeMap` which is a wrapper over `HashMap<TypeId, Box<dyn Any>>`.
It allows you to store a single instance of all types that implement
[`Any`](https://doc.rust-lang.org/std/any/trait.Any.html).
