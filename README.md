# `StaticTypeMap`, a map of types

![Build, Test & Check Formatting](https://github.com/malobre/static_type_map/workflows/Build,%20Test%20&%20Check%20Formatting/badge.svg?branch=master)

[Documentation](https://docs.rs/static_type_map/) |
[Crates.io](https://crates.io/crates/static_type_map)

This crates provides a
[`StaticTypeMap`](https://docs.rs/static_type_map/static_type_map/struct.StaticTypeMap.html)
which allows you to store a single instance of any `'static` value as long as it
implements [`Any`](https://doc.rust-lang.org/stable/std/any/trait.Any.html).

The crate is `no_std` compatible using the `hasbrown` feature, (`alloc` is still
required).

## Features

| name        | default ? | description                                                                                                          |
| ----------- | --------- | -------------------------------------------------------------------------------------------------------------------- |
| `send`      | yes       | Enables [`SendStaticTypeMap`](https://docs.rs/static_type_map/static_type_map/struct.SendStaticTypeMap.html)         |
| `sync`      | yes       | Enables [`SendSyncStaticTypeMap`](https://docs.rs/static_type_map/static_type_map/struct.SendSyncStaticTypeMap.html) |
| `hashbrown` | no        | Enables `no_std` support                                                                                             |

## Example

```rust
use static_type_map::StaticTypeMap;
use some_crate::{ EventA, EventB, EventC };

pub fn main() {
    let mut events = StaticTypeMap::new();

    events.insert(Vec<EventA>::new());
    events.insert(Vec<EventB>::new());

    events.get_mut::<Vec<EventA>>().push(EventA::new());
    events.get_mut::<Vec<_>>().push(EventA::new());

    events.get_mut::<Vec<_>>().push(EventB::new());

    assert_eq!(events.get::<Vec<EventA>>().len(), 2);
    assert_eq!(events.get::<Vec<EventB>>().len(), 1);
    assert_eq!(events.get::<Vec<EventC>>(), None);
}
```
