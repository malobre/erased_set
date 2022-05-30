//! # 🦀 `ErasedSet`
//!
//! You may be looking for:
//!
//! - [Git repository](https://github.com/malobre/erased_set)
//! - [Crates.io](https://crates.io/crates/erased_set)
//!
//! ---
//!
//! This crate provides a new collection: the [`ErasedSet`].
//! It allows storing different types in a single set (as long as they implement [`Any`]).
//!
//! [`Any`]: ::core::any::Any
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
//!
//! ## `no_std` support
//!
//! This crate is `no_std` compatible, however it still requires `alloc`.

#![no_std]

extern crate alloc;

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
// This macro is not currently public because trait objects for multiple traits are not currently
// supported, see <https://github.com/rust-lang/rfcs/issues/2035> for more details.
macro_rules! impl_erased_set {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident: Any $(+ $bounds:tt)*;
    ) => {
        $(#[$attr])*
        $vis struct $name(
            ::alloc::collections::BTreeMap<
                ::core::any::TypeId,
                ::alloc::boxed::Box<dyn ::core::any::Any $(+ $bounds)*>
            >
        );

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
                Self(::alloc::collections::BTreeMap::new())
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
                T: ::core::any::Any,
            {
                self.0.contains_key(&::core::any::TypeId::of::<T>())
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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

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
    /// This set can store a single instance of any type that implements [`Any`](::core::any::Any).
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
