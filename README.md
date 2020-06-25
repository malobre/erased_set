`StaticTypeMap`, a type map for types that impl `Any`
=====================================================
![Build, Test & Check Formatting](https://github.com/malobre/static_type_map/workflows/Build,%20Test%20&%20Check%20Formatting/badge.svg?branch=master)

[Documentation](https://docs.rs/static_type_map/) | [Crates.io](https://crates.io/crates/static_type_map)

This crates provides a `StaticTypeMap` which is a wrapper over `HashMap<TypeId, Box<dyn Any>>`.
It allows you to store a single instance of all types that implement
[`Any`](https://doc.rust-lang.org/std/any/trait.Any.html).
