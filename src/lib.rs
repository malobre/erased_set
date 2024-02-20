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
        $vis struct $name {
            #[doc(hidden)]
            inner: ::alloc::collections::BTreeMap<
                ::core::any::TypeId,
                ::alloc::boxed::Box<dyn ::core::any::Any $(+ $bounds)*>,
            >,
            #[doc(hidden)]
            #[cfg(debug_assertions)]
            debug_type_names: ::alloc::collections::BTreeMap<
                ::core::any::TypeId,
                &'static str
            >,
        }

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
                Self {
                    inner: ::alloc::collections::BTreeMap::new(),
                    #[cfg(debug_assertions)]
                    debug_type_names: ::alloc::collections::BTreeMap::new(),
                }
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
                self.inner.is_empty()
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
                self.inner.len()
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
                self.inner.clear();
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
                self.inner.contains_key(&::core::any::TypeId::of::<T>())
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

                self.inner
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
            pub fn get_or_insert<T>(&mut self, value: T) -> &T
            where
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

                #[cfg(debug_assertions)]
                self.debug_type_names.insert(TypeId::of::<T>(), core::any::type_name::<T>());

                let boxed_any: &Box<dyn Any $(+ $bounds)*> = self
                    .inner
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
            pub fn get_or_insert_with<T>(&mut self, f: impl FnOnce() -> T) -> &T
            where
                T: ::core::any::Any $(+ $bounds)*,
            {
                use ::core::any::{Any, TypeId};
                use ::alloc::boxed::Box;

                #[cfg(debug_assertions)]
                self.debug_type_names.insert(TypeId::of::<T>(), core::any::type_name::<T>());

                let boxed_any: &Box<dyn Any $(+ $bounds)*> = self
                    .inner
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

                self.inner
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

                #[cfg(debug_assertions)]
                self.debug_type_names.insert(TypeId::of::<T>(), core::any::type_name::<T>());

                self.inner
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

                #[cfg(debug_assertions)]
                self.debug_type_names.remove(&TypeId::of::<T>());

                self.inner
                    .remove(&TypeId::of::<T>())
                    .map(|boxed_any: Box<dyn Any $(+ $bounds)*>| {
                        // Sanity check
                        debug_assert!(boxed_any.as_ref().is::<T>());

                        let ptr = Box::into_raw(boxed_any).cast::<T>();

                        unsafe { *Box::from_raw(ptr) }
                    })
            }

            /// Gets an iterator over the [`TypeId`](::core::any::TypeId)s of stored elements, in arbitrary order.
            pub fn type_ids(&self) -> impl Iterator<Item = &::core::any::TypeId> {
                self.inner.keys()
            }

            /// Gets an iterator over the names of the stored types, in arbitrary order.
            #[cfg(debug_assertions)]
            pub fn debug_type_names(&self) -> impl Iterator<Item = &'static str> + '_ {
                assert!(self.inner.keys().eq(self.debug_type_names.keys()));

                self.debug_type_names.values().map(|&name: &&'static str| name)
            }
        }
    }
}

impl_erased_set! {
    /// A set of erased types.
    ///
    /// This set can store a single instance of any type that implements [`Any`](::core::any::Any).
    ///
    /// ## Example
    ///
    /// ```
    /// # #[derive(Debug, PartialEq)]
    /// # struct ClickEvent(u32, u32);
    /// # #[derive(Debug, PartialEq)]
    /// # struct KeyDownEvent(char);
    /// #
    /// use erased_set::ErasedSet;
    ///
    /// let mut set = ErasedSet::new();
    /// set.insert(ClickEvent(128, 256));
    /// set.insert(KeyDownEvent('z'));
    ///
    /// assert_eq!(set.get::<ClickEvent>(), Some(&ClickEvent(128, 256)));
    ///
    /// assert_eq!(set.insert(KeyDownEvent('e')), Some(KeyDownEvent('z')));
    ///
    /// set.remove::<ClickEvent>();
    ///
    /// assert_eq!(set.len(), 1);
    /// ```
    #[derive(Default)]
    pub struct ErasedSet: Any;
}

impl core::fmt::Debug for ErasedSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
            .entries(
                #[cfg(debug_assertions)]
                self.debug_type_names(),
                #[cfg(not(debug_assertions))]
                self.type_ids(),
            )
            .finish()
    }
}

#[cfg(feature = "send")]
impl_erased_set! {
    /// Like [`ErasedSet`] but with a [`Send`] bound.
    ///
    /// ## Example
    ///
    /// ```
    /// # #[derive(Debug, PartialEq)]
    /// # struct ClickEvent(u32, u32);
    /// # #[derive(Debug, PartialEq)]
    /// # struct KeyDownEvent(char);
    /// #
    /// use erased_set::ErasedSendSet;
    ///
    /// let mut set = ErasedSendSet::new();
    /// set.insert(ClickEvent(128, 256));
    /// set.insert(KeyDownEvent('z'));
    ///
    /// assert_eq!(set.get::<ClickEvent>(), Some(&ClickEvent(128, 256)));
    ///
    /// assert_eq!(set.insert(KeyDownEvent('e')), Some(KeyDownEvent('z')));
    ///
    /// set.remove::<ClickEvent>();
    ///
    /// assert_eq!(set.len(), 1);
    /// ```
    #[derive(Default)]
    pub struct ErasedSendSet: Any + Send;
}

#[cfg(feature = "send")]
impl core::fmt::Debug for ErasedSendSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
            .entries(
                #[cfg(debug_assertions)]
                self.debug_type_names(),
                #[cfg(not(debug_assertions))]
                self.type_ids(),
            )
            .finish()
    }
}

#[cfg(feature = "sync")]
impl_erased_set! {
    /// Like [`ErasedSet`] but with a [`Send`] + [`Sync`] bound.
    ///
    /// ## Example
    ///
    /// ```
    /// # #[derive(Debug, PartialEq)]
    /// # struct ClickEvent(u32, u32);
    /// # #[derive(Debug, PartialEq)]
    /// # struct KeyDownEvent(char);
    /// #
    /// use erased_set::ErasedSyncSet;
    ///
    /// let mut set = ErasedSyncSet::new();
    /// set.insert(ClickEvent(128, 256));
    /// set.insert(KeyDownEvent('z'));
    ///
    /// assert_eq!(set.get::<ClickEvent>(), Some(&ClickEvent(128, 256)));
    ///
    /// assert_eq!(set.insert(KeyDownEvent('e')), Some(KeyDownEvent('z')));
    ///
    /// set.remove::<ClickEvent>();
    ///
    /// assert_eq!(set.len(), 1);
    /// ```
    #[derive(Default)]
    pub struct ErasedSyncSet: Any + Send + Sync;
}

#[cfg(feature = "sync")]
impl core::fmt::Debug for ErasedSyncSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
            .entries(
                #[cfg(debug_assertions)]
                self.debug_type_names(),
                #[cfg(not(debug_assertions))]
                self.type_ids(),
            )
            .finish()
    }
}
