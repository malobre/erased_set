//! This crate provides a [`StaticTypeMap`] which allows you to store a single instance of any type
//! that implement [`Any`].
//!
//! # Examples
//!
//! ```
//! # use static_type_map::StaticTypeMap;
//! let mut type_map = StaticTypeMap::new();
//! type_map.insert(10u8);
//! type_map.insert(20u16);
//! type_map.insert(true);
//! type_map.insert("a");
//!
//! assert!(type_map.contains::<bool>());
//!
//! assert_eq!(type_map.get::<&str>(), Some(&"a"));
//!
//! if let Some(previous_value) = type_map.insert(50u8) {
//!     assert_eq!(previous_value, 10u8);
//! }
//!
//! type_map.remove::<u16>();
//!
//! assert_eq!(type_map.len(), 3);
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
extern crate core;
#[cfg(feature = "no_std")]
use alloc::boxed::Box;
#[cfg(feature = "no_std")]
use hashbrown::HashMap;

#[cfg(not(feature = "no_std"))]
use std::collections::HashMap;

use core::any::{Any, TypeId};

macro_rules! define_bounded_map {
    ($(#[$attr:meta])* $name:ident, $bound:tt $(+ $others:tt)*) => {
        $(#[$attr])*
        pub struct $name(HashMap<TypeId, Box<dyn $bound $(+ $others)*>>);

        impl Default for $name {
            fn default() -> Self {
                Self(HashMap::new())
            }
        }

        impl $name {
            #[doc = concat!("Creates an empty [`", stringify!($name), "`].")]
            ///
            /// The map is initially created with a capacity of 0, so it will not allocate
            /// until it is first inserted into.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let type_map = ", stringify!($name), "::new();")]
            /// ```
            #[must_use]
            pub fn new() -> Self {
                Self { ..Self::default() }
            }

            #[doc = concat!("Creates an empty [`", stringify!($name), "`] with the specified capacity.")]
            ///
            /// The map will be able to hold at least `capacity` types without reallocating.
            /// If `capacity` is 0, the static type map will not allocate.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let type_map = ", stringify!($name), "::with_capacity(10);")]
            /// ```
            #[must_use]
            pub fn with_capacity(capacity: usize) -> Self {
                Self(HashMap::with_capacity(capacity))
            }

            /// Returns the number of types the map can hold without reallocating.
            ///
            /// This number is a lower bound; the map might be able to hold more, but it is
            /// guaranteed to be able to hold at least so many.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let type_map = ", stringify!($name), "::with_capacity(100);")]
            /// assert!(type_map.capacity() >= 100);
            /// ```
            #[must_use]
            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }

            /// Returns `true` if the map contains no instances of any type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let type_map = ", stringify!($name), "::new();")]
            /// assert!(type_map.is_empty());
            /// ```
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns the number of types in the map.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// assert_eq!(type_map.len(), 0);
            /// type_map.insert("a");
            /// assert_eq!(type_map.len(), 1);
            /// ```
            #[must_use]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Clears the map. Keep allocated memory for reuse.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// type_map.insert("a");
            /// type_map.clear();
            /// assert!(type_map.is_empty());
            /// ```
            pub fn clear(&mut self) {
                self.0.clear();
            }

            /// Reserves capacity for at least `additional` more types to be inserted in the map. The
            /// collection may reserve more space to avoid frequent reallocations.
            ///
            /// # Panics
            ///
            /// Panics if the new allocation size overflows [`usize`].
            ///
            /// # Examples
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// assert_eq!(type_map.capacity(), 0);
            /// type_map.reserve(10);
            /// assert!(type_map.capacity() >= 10);
            /// ```
            pub fn reserve(&mut self, additional: usize) {
                self.0.reserve(additional);
            }

            /// Shrinks the capacity of the map as much as possible. It will drop down as much as possible
            /// while mainting the internal rules and possibly leaving some space in accordance with the
            /// resize policy.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::with_capacity(100);")]
            /// assert!(type_map.capacity() >= 0);
            /// type_map.insert("a");
            /// type_map.insert(true);
            /// assert!(type_map.capacity() >= 2);
            /// ```
            pub fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit();
            }

            /// Returns `true` if the map contains an instance of `T`.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// type_map.insert("a");
            /// assert!(type_map.contains::<&str>());
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
            /// If the map does not have an instance of `T`, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// type_map.insert("a");
            /// assert_eq!(type_map.get::<&str>(), Some(&"a"));
            /// assert_eq!(type_map.get::<bool>(), None);
            /// ```
            #[must_use]
            pub fn get<T>(&self) -> Option<&T>
            where
                T: $bound $(+ $others)*,
            {
                self.0
                    .get(&TypeId::of::<T>())
                    .and_then(|any| (&**any as &(dyn Any)).downcast_ref())
            }

            /// Returns a mutable reference to an instance of `T`.
            ///
            /// If the map does not have an instance of `T`, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// type_map.insert("a");
            /// if let Some(x) = type_map.get_mut::<&str>() {
            ///     *x = "b";
            /// }
            /// assert_eq!(type_map.get::<&str>(), Some(&"b"));
            /// ```
            #[must_use]
            pub fn get_mut<T>(&mut self) -> Option<&mut T>
            where
                T: $bound $(+ $others)*,
            {
                self.0
                    .get_mut(&TypeId::of::<T>())
                    .and_then(|any| (&mut **any as &mut (dyn Any)).downcast_mut())
            }

            /// Insert an instance of type `T` into the map.
            ///
            /// If the map did not have this type present, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// assert_eq!(type_map.insert("a"), None);
            /// assert_eq!(type_map.insert("b"), Some("a"));
            /// ```
            pub fn insert<T>(&mut self, t: T) -> Option<T>
            where
                T: $bound $(+ $others)*,
            {
                self.0
                    .insert(TypeId::of::<T>(), Box::new(t))
                    .and_then(|any| (any as Box<dyn Any>).downcast().ok())
                    .map(|concrete_type| *concrete_type)
            }

            /// Remove and return an instance of type `T` from the map.
            ///
            /// If the map did not have this type present, [`None`] is returned.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use static_type_map::", stringify!($name), ";")]
            #[doc = concat!("let mut type_map = ", stringify!($name), "::new();")]
            /// type_map.insert("a");
            /// assert_eq!(type_map.remove::<&str>(), Some("a"));
            /// ```
            pub fn remove<T>(&mut self) -> Option<T>
            where
                T: $bound $(+ $others)*,
            {
                self.0
                    .remove(&TypeId::of::<T>())
                    .and_then(|any| (any as Box<dyn Any>).downcast().ok())
                    .map(|concrete_type| *concrete_type)
            }
        }
    }
}

define_bounded_map!(StaticTypeMap, Any);

define_bounded_map!(
    /// Like [`StaticTypeMap`] but with a [`Send`] bound.
    SendStaticTypeMap,
    Any + Send
);

define_bounded_map!(
    /// Like [`StaticTypeMap`] but with a [`Send`] + [`Sync`] bound.
    SendSyncStaticTypeMap,
    Any + Send + Sync
);
