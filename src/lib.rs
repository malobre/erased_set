//! # 🦀 `ErasedSet`
//!
//! You may be looking for:
//!
//! - [Git repository](https://github.com/malobre/erased_set)
//! - [Crates.io](https://crates.io/crates/erased_set)
//!
//! ---
//!
//! This crate provides a new collection: The [`ErasedSet`].
//!
//! It can store any type `T: Any`.
//!
//! ## Example
//!
//! ```
//! # #[derive(Debug, PartialEq)]
//! # struct ClickEvent(u32, u32);
//! # #[derive(Debug, PartialEq)]
//! # struct KeyDownEvent(char);
//! #
//! use erased_set::ErasedSet;
//!
//! let mut set = ErasedSet::new();
//! set.insert(ClickEvent(128, 256));
//! set.insert(KeyDownEvent('z'));
//!
//! assert_eq!(set.get::<ClickEvent>(), Some(&ClickEvent(128, 256)));
//!
//! assert_eq!(set.insert(KeyDownEvent('e')), Some(KeyDownEvent('z')));
//!
//! set.remove::<ClickEvent>();
//!
//! assert_eq!(set.len(), 1);
//! ```
//!
//! ## Features
//!
//! | name        | default ? | description               |
//! | ----------- | --------- | ------------------------- |
//! | `send`      | yes       | Enables [`ErasedSendSet`] |
//! | `sync`      | yes       | Enables [`ErasedSyncSet`] |
//! | `hashbrown` | no        | Enables `no_std` support  |

#![cfg_attr(feature = "hashbrown", no_std)]

#[cfg(feature = "hashbrown")]
extern crate alloc;
#[cfg(feature = "hashbrown")]
extern crate core;
#[cfg(feature = "hashbrown")]
use alloc::boxed::Box;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use core::any::{Any, TypeId};

/// Implement an erased set with the specified bounds.
///
/// # Syntax
///
/// ```ignore
/// impl_erased_set! {
///     [pub] struct NAME: Any [+ BOUNDS ...];
/// }
/// ```
///
/// # Example
///
/// ```rust
/// erased_set::impl_erased_set! {
///     /// A set of erased types.
///     #[derive(Debug, Default)]
///     pub struct ErasedSet: Any;
/// }
/// ```
macro_rules! impl_erased_set {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident: Any $(+ $bounds:tt)*;
    ) => {
        $(#[$attr])*
        $vis struct $name(HashMap<TypeId, Box<dyn Any $(+ $bounds)*>>);

        #[allow(rustdoc::private_doc_tests)]
        impl $name {
            #[doc = concat!("Creates an empty [`", stringify!($name), "`].")]
            ///
            /// The set is initially created with a capacity of 0, so it will not allocate
            /// until it is first inserted into.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let set = ", stringify!($name), "::new();")]
            /// ```
            #[must_use]
            pub fn new() -> Self {
                Self(HashMap::new())
            }

            #[doc = concat!("Creates an empty [`", stringify!($name), "`] with the specified capacity.")]
            ///
            /// The set will be able to hold at least `capacity` types without reallocating.
            /// If `capacity` is 0, the set will not allocate.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let set = ", stringify!($name), "::with_capacity(10);")]
            /// ```
            #[must_use]
            pub fn with_capacity(capacity: usize) -> Self {
                Self(HashMap::with_capacity(capacity))
            }

            /// Returns the number of types the set can hold without reallocating.
            ///
            /// This number is a lower bound; the set might be able to hold more, but it is
            /// guaranteed to be able to hold at least so many.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let set = ", stringify!($name), "::with_capacity(100);")]
            /// assert!(set.capacity() >= 100);
            /// ```
            #[must_use]
            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }

            /// Returns `true` if the set contains no instances of any type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let set = ", stringify!($name), "::new();")]
            /// assert!(set.is_empty());
            /// ```
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns the number of types in the set.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// assert_eq!(set.len(), 0);
            /// set.insert("a");
            /// assert_eq!(set.len(), 1);
            /// ```
            #[must_use]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Clears the set. Keep allocated memory for reuse.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// set.insert("a");
            /// set.clear();
            /// assert!(set.is_empty());
            /// ```
            pub fn clear(&mut self) {
                self.0.clear();
            }

            /// Reserves capacity for at least `additional` more types to be inserted in the set. The
            /// collection may reserve more space to avoid frequent reallocations.
            ///
            /// # Panics
            ///
            /// Panics if the new allocation size overflows [`usize`].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// assert_eq!(set.capacity(), 0);
            /// set.reserve(10);
            /// assert!(set.capacity() >= 10);
            /// ```
            pub fn reserve(&mut self, additional: usize) {
                self.0.reserve(additional);
            }

            /// Shrinks the capacity of the set with a lower limit. It will drop
            /// down no lower than the supplied limit while maintaining the internal rules
            /// and possibly leaving some space in accordance with the resize policy.
            ///
            /// If the current capacity is less than the lower limit, this is a no-op.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::with_capacity(100);")]
            /// set.insert(1_u8);
            /// set.insert(1_u16);
            /// assert!(set.capacity() >= 100);
            /// set.shrink_to(10);
            /// assert!(set.capacity() >= 10);
            /// set.shrink_to(0);
            /// assert!(set.capacity() >= 2);
            /// ```
            pub fn shrink_to(&mut self, min_capacity: usize) {
                self.0.shrink_to(min_capacity)
            }

            /// Shrinks the capacity of the set as much as possible. It will drop down as much as possible
            /// while mainting the internal rules and possibly leaving some space in accordance with the
            /// resize policy.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::with_capacity(100);")]
            /// set.insert(1_u8);
            /// set.insert(1_u16);
            /// assert!(set.capacity() >= 100);
            /// set.shrink_to_fit();
            /// assert!(set.capacity() >= 2);
            /// ```
            pub fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit();
            }

            /// Returns `true` if the set contains an instance of `T`.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// set.insert("a");
            /// assert!(set.contains::<&str>());
            /// ```
            #[must_use]
            pub fn contains<T>(&self) -> bool
            where
                T: Any,
            {
                self.0.contains_key(&TypeId::of::<T>())
            }

            /// Returns a reference to an instance of `T`.
            ///
            /// If the set does not have an instance of `T`, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// set.insert("a");
            /// assert_eq!(set.get::<&str>(), Some(&"a"));
            /// assert_eq!(set.get::<bool>(), None);
            /// ```
            #[must_use]
            pub fn get<T>(&self) -> Option<&T>
            where
                T: Any $(+ $bounds)*,
            {
                self.0
                    .get(&TypeId::of::<T>())
                    .map(|boxed_any: &Box<dyn Any $(+ $bounds)*>| {
                        // Sanity check
                        debug_assert!(boxed_any.as_ref().is::<T>());

                        let ptr = (boxed_any.as_ref() as *const dyn Any).cast::<T>();

                        unsafe { &*ptr }
                    })
            }

            /// Inserts the given `value` into the set if it is not present, then
            /// returns a reference to the value in the set.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// assert_eq!(set.get_or_insert("abc"), &"abc");
            /// assert_eq!(set.get_or_insert("def"), &"abc");
            /// ```
            #[must_use]
            pub fn get_or_insert<T>(&mut self, value: T) -> &T
            where
                T: Any $(+ $bounds)*,
            {
                let boxed_any: &Box<dyn Any $(+ $bounds)*> = self
                    .0
                    .entry(TypeId::of::<T>())
                    .or_insert_with(|| Box::new(value));

                // Sanity check
                debug_assert!(boxed_any.as_ref().is::<T>());

                let ptr = (boxed_any.as_ref() as *const dyn Any).cast::<T>();

                unsafe { &*ptr }
            }

            /// Inserts a value computed from `f` into the set if it does not contain
            /// a value of type `T`, then returns a reference to the value in the set.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// assert_eq!(set.get_or_insert_with(|| String::from("abc")), &"abc");
            /// assert_eq!(set.get_or_insert_with(|| String::from("def")), &"abc");
            /// ```
            #[must_use]
            pub fn get_or_insert_with<T>(&mut self, f: impl FnOnce() -> T) -> &T
            where
                T: Any $(+ $bounds)*,
            {
                let boxed_any: &Box<dyn Any $(+ $bounds)*> = self
                    .0
                    .entry(TypeId::of::<T>())
                    .or_insert_with(|| Box::new(f()));

                // Sanity check
                debug_assert!(boxed_any.as_ref().is::<T>());

                let ptr = (boxed_any.as_ref() as *const dyn Any).cast::<T>();

                unsafe { &*ptr }
            }

            /// Returns a mutable reference to an instance of `T`.
            ///
            /// If the set does not have an instance of `T`, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// set.insert("a");
            /// if let Some(x) = set.get_mut::<&str>() {
            ///     *x = "b";
            /// }
            /// assert_eq!(set.get::<&str>(), Some(&"b"));
            /// ```
            #[must_use]
            pub fn get_mut<T>(&mut self) -> Option<&mut T>
            where
                T: Any $(+ $bounds)*,
            {
                self.0
                    .get_mut(&TypeId::of::<T>())
                    .map(|boxed_any: &mut Box<dyn Any $(+ $bounds)*>| {
                        // Sanity check
                        debug_assert!(boxed_any.as_mut().is::<T>());

                        let ptr = (boxed_any.as_mut() as *mut dyn Any).cast::<T>();

                        unsafe { &mut *ptr }
                    })
            }

            /// Insert an instance of type `T` into the set.
            ///
            /// Returns the replaced value or [`None`].
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// assert_eq!(set.insert("a"), None);
            /// assert_eq!(set.insert("b"), Some("a"));
            /// ```
            pub fn insert<T>(&mut self, value: T) -> Option<T>
            where
                T: Any $(+ $bounds)*,
            {
                self.0
                    .insert(TypeId::of::<T>(), Box::new(value))
                    .map(|boxed_any: Box<dyn Any $(+ $bounds)*>| {
                        // Sanity check
                        debug_assert!(boxed_any.as_ref().is::<T>());

                        let ptr = Box::into_raw(boxed_any).cast::<T>();

                        unsafe { *Box::from_raw(ptr) }
                    })
            }

            /// Remove and return an instance of type `T` from the set.
            ///
            /// If the set did not have this type present, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ", module_path!(), "::", stringify!($name), ";")]
            ///
            #[doc = concat!("let mut set = ", stringify!($name), "::new();")]
            /// set.insert("a");
            /// assert_eq!(set.remove::<&str>(), Some("a"));
            /// ```
            pub fn remove<T>(&mut self) -> Option<T>
            where
                T: Any $(+ $bounds)*,
            {
                self.0
                    .remove(&TypeId::of::<T>())
                    .map(|boxed_any: Box<dyn Any $(+ $bounds)*>| {
                        // Sanity check
                        debug_assert!(boxed_any.as_ref().is::<T>());

                        let ptr = Box::into_raw(boxed_any).cast::<T>();

                        unsafe { *Box::from_raw(ptr) }
                    })
            }
        }
    }
}

impl_erased_set! {
    /// A set of erased types.
    ///
    /// This set can store a single instance of any type that implements [`Any`].
    #[derive(Debug, Default)]
    pub struct ErasedSet: Any;
}

#[cfg(feature = "send")]
impl_erased_set! {
    /// Like [`ErasedSet`] but with a [`Send`] bound.
    #[derive(Debug, Default)]
    pub struct ErasedSendSet: Any + Send;
}

#[cfg(feature = "sync")]
impl_erased_set! {
    /// Like [`ErasedSet`] but with a [`Send`] + [`Sync`] bound.
    #[derive(Debug, Default)]
    pub struct ErasedSyncSet: Any + Send + Sync;
}
