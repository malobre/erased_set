//! This crate provides a `StaticTypeMap` which allows you to store a single instance of any type
//! that implement [`Any`]
//!

use ::std::any::{Any, TypeId};
use ::std::collections::HashMap;

/// A wrapper over `HashMap<TypeId, Box<dyn Any>>`.
///
/// # Examples
///
/// ```
/// use ::static_type_map::StaticTypeMap;
/// let mut type_map = StaticTypeMap::new();
/// type_map.insert(10u8);
/// type_map.insert(20u16);
/// type_map.insert(true);
/// type_map.insert("a");
///
/// if type_map.contains::<&str>() {
///     println!("We have an `&str` stored, its value is: {}.", type_map.get::<&str>().unwrap());
/// }
///
/// if let Some(previous_value) = type_map.insert(50u8) {
///     println!("We had a `u8` before inserting a new one, its value was {}.", previous_value);
/// }
///
/// type_map.remove::<bool>();
///
/// println!("Our `StaticTypeMap` contains {} types.", type_map.len());
/// ```
pub struct StaticTypeMap(HashMap<TypeId, Box<dyn Any>>);

impl StaticTypeMap {
    /// Creates an empty `StaticTypeMap`
    ///
    /// The static type map is initially created with a capacity of 0, so it will not allocate
    /// until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Creates an empty `StaticTypeMap`
    ///
    /// The static type map will be able to hold at least `capacity` types without reallocating.
    /// If `capacity` is 0, the static type map will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of types the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `StaticTypeMap` might be able to hold more, but it is
    /// guaranteed to be able to hold at least so many.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::with_capacity(100);
    /// assert!(type_map.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns `true` if the map contains no instances of any type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// assert!(type_map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of types in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// assert_eq!(type_map.len(), 0);
    /// type_map.insert("a");
    /// assert_eq!(type_map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Clears the map. Keep allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// type_map.insert("a");
    /// type_map.clear();
    /// assert!(type_map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns `true` if the map contains an instance of `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// type_map.insert("a");
    /// assert!(type_map.contains::<&str>());
    /// ```
    pub fn contains<T>(&self) -> bool
    where
        T: Any,
    {
        self.0.contains_key(&TypeId::of::<T>())
    }

    /// Returns a reference to the instance of `T`.
    ///
    /// If the map does not have an instance of `T`, [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// type_map.insert("a");
    /// assert_eq!(type_map.get::<&str>(), Some(&"a"));
    /// assert_eq!(type_map.get::<bool>(), None);
    /// ```
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref())
    }

    /// Returns a mutable reference to the instance of `T`.
    ///
    /// If the map does not have an instance of `T`, [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// type_map.insert("a");
    /// if let Some(x) = type_map.get_mut::<&str>() {
    ///     *x = "b";
    /// }
    /// assert_eq!(type_map.get::<&str>(), Some(&"b"));
    /// ```
    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        self.0
            .get_mut(&TypeId::of::<T>())
            .and_then(|any| any.downcast_mut())
    }

    /// Insert an instance of type `T` into the map.
    ///
    /// If the map did not have this type present, [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// assert_eq!(type_map.insert("a"), None);
    /// assert_eq!(type_map.insert("b"), Some("a"));
    /// ```
    pub fn insert<T>(&mut self, t: T) -> Option<T>
    where
        T: Any,
    {
        self.0
            .insert(TypeId::of::<T>(), Box::new(t))
            .and_then(|any| any.downcast().ok())
            .map(|concrete_type| *concrete_type)
    }

    /// Remove and return the instance of type `T` from the map.
    ///
    /// If the map did not have this type present, [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// type_map.insert("a");
    /// assert_eq!(type_map.remove::<&str>(), Some("a"));
    /// ```
    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Any,
    {
        self.0.remove(&TypeId::of::<T>())
            .and_then(|any| any.downcast().ok())
            .map(|concrete_type| *concrete_type)
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
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::new();
    /// assert_eq!(type_map.capacity(), 0);
    /// type_map.reserve(10);
    /// assert!(type_map.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Shrinks the capacity of the map as much as possible. It will drop down as much as possible
    /// while mainting the internal rules and possibly leaving some space in accordance with the
    /// resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use ::static_type_map::StaticTypeMap;
    /// let mut type_map: StaticTypeMap = StaticTypeMap::with_capacity(100);
    /// assert!(type_map.capacity() >= 0);
    /// type_map.insert("a");
    /// type_map.insert(true);
    /// assert!(type_map.capacity() >= 2);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
}

impl Default for StaticTypeMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
