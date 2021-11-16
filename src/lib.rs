//! A map that creates a random handle on insertion to use when retrieving.

use hashers::null::PassThroughHasher;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{HashMap, Iter, IterMut};
use std::hash::BuildHasherDefault;

/// A map that creates a random handle on insertion to use when retrieving.
///
/// The raison d'être for this map is:
///
/// - You want to put something in a map, but you have no key. Means you do
///   not want to use a [`HashMap`
///   ](https://doc.rust-lang.org/std/collections/struct.HashMap.html) or
///   [`BTreeMap`
///   ](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).
///
/// - You want to forget the details of what you put in to later retrieve it
///   with a simple handle, and you are not interested in how many equal
///   objects you insert. Means you do not want to use a [`HashSet`
///   ](https://doc.rust-lang.org/std/collections/struct.HashSet.html) or
///   multiset.
///
/// - You want a persistent handle to refer to the item you put in the map.
///   Means you do not want to use a `Vec`.
///
/// The implementation uses a `HashMap` that does not actually hash. The
/// contained `HashMap` can be [borrowed](#method.as_hash_map), so all
/// nonmuting [`HashMap`
/// ](https://doc.rust-lang.org/std/collections/struct.HashMap.html) functions
/// are at your disposal.
///
/// ### Example:
/// ```
/// use rand_map::RandMap;
///
/// let mut map: RandMap<String> = RandMap::new();
/// let foo = map.insert("foo".to_string());
/// let bar = map.insert("bar".to_string());
/// assert_ne!(foo, bar);
/// map.clear();
/// assert!(map.as_hash_map().is_empty());
/// assert!(map.get(foo).is_none());
/// let foo = map.insert("foo".to_string());
/// let bar = map.insert("bar".to_string());
/// assert_eq!(map.len(), 2);
/// assert_eq!(map.get(foo), Some(&"foo".to_string()));
/// assert_eq!(map.get(bar).unwrap(), "bar");
/// for (key, value) in map.as_hash_map() {
///     if *key == foo {
///         assert_eq!(value, "foo");
///     }
/// }
/// for (key, mut value) in map.iter_mut() {
///     value.push_str("_more");
/// }
/// let mutref = map.get_mut(bar);
/// assert!(mutref.is_some());
/// mutref.unwrap().push_str("_and_more");
/// assert_eq!(map.remove(foo).unwrap(), "foo_more");
/// assert_eq!(map.get(bar).unwrap(), "bar_more_and_more");
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
pub struct RandMap<V>(HashMap<u64, V, BuildHasherDefault<PassThroughHasher>>);

impl<V> RandMap<V> {
    /// Creates an empty map.
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    /// Borrow the contained [`HashMap`
    /// ](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
    #[inline]
    pub fn as_hash_map(
        &self,
    ) -> &HashMap<u64, V, BuildHasherDefault<PassThroughHasher>> {
        &self.0
    }

    /// Clears the map.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Retrieves a reference to a `V` using the handle created by [`insert()`
    /// ](#method.insert).
    #[inline]
    pub fn get(&self, handle: u64) -> Option<&V> {
        self.0.get(&handle)
    }

    /// Retrieves a mutable reference to a `V` using the handle created by
    /// [`insert()`](#method.insert).
    #[inline]
    pub fn get_mut(&mut self, handle: u64) -> Option<&mut V> {
        self.0.get_mut(&handle)
    }

    /// Insert a `V` and get a handle for retrieval.
    ///
    pub fn insert(&mut self, value: V) -> u64 {
        use rand::{thread_rng, Rng};
        let key: u64 = thread_rng().gen();
        self.0.insert(key, value);
        key
    }

    /// Equivalent to `as_hash_map().iter()`.
    // TODO: make it (u64, &V)
    #[inline]
    pub fn iter(&self) -> Iter<u64, V> {
        self.0.iter()
    }

    /// The iterator element type is `(&u64, &mut V)`.
    // TODO: make it (u64, &mut V)
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<u64, V> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Remove and return the value with handle `handle`, or `None` if not
    /// found.
    #[inline]
    pub fn remove(&mut self, handle: u64) -> Option<V> {
        self.0.remove(&handle)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map: RandMap<String> = RandMap::new();
        let foo = map.insert("foo".to_string());
        let bar = map.insert("bar".to_string());
        assert_ne!(foo, bar);
    }
}
*/
