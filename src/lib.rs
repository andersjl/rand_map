//! A map that creates a random handle on insertion to use when retrieving.

use hashers::null::PassThroughHasher;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::hash_map::{self, HashMap};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;

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
/// use rand_map::{Handle, RandMap};
///
/// let mut map: RandMap<String> = RandMap::new();
/// let foo = map.insert("foo".to_string());
/// let baz = Handle::<String>::from_u64(4711);
/// map.insert_key_value(baz, "baz".to_string());
/// assert_eq!(baz.as_u64(), 4711);
/// map.remove(baz);
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
/// for (key, value) in &map {
///     if key == bar {
///         assert_eq!(value, "bar");
///     }
/// }
/// for (key, mut value) in map.iter_mut() {
///     assert!(vec![foo, bar].contains(&key));
///     value.push_str("_more");
/// }
/// let mutref = map.get_mut(bar);
/// assert!(mutref.is_some());
/// mutref.unwrap().push_str("_and_more");
/// assert_eq!(map.remove(foo).unwrap(), "foo_more");
/// assert_eq!(map.get(bar).unwrap(), "bar_more_and_more");
/// // Note that as_hash_map() returns a HashMap<Handle, _> the methods of
/// // which generally take a key parameter that is &Handle.
/// assert!(map.as_hash_map().contains_key(&bar));
/// assert!(map == map.clone());
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
pub struct RandMap<V>(
    HashMap<Handle<V>, V, BuildHasherDefault<PassThroughHasher>>
);

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
    ) -> &HashMap<Handle<V>, V, BuildHasherDefault<PassThroughHasher>> {
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
    pub fn get(&self, handle: Handle<V>) -> Option<&V> {
        self.0.get(&handle)
    }

    /// Retrieves a mutable reference to a `V` using the handle created by
    /// [`insert()`](#method.insert).
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<V>) -> Option<&mut V> {
        self.0.get_mut(&handle)
    }

    /// Insert a `V` and get a handle for retrieval.
    ///
    pub fn insert(&mut self, value: V) -> Handle<V> {
        use rand::{thread_rng, Rng};
        let key: Handle<V> = thread_rng().gen();
        self.0.insert(key, value);
        key
    }

    /// Insert a key-value pair. Does *not* return the old value for `key`.
    ///
    pub fn insert_key_value(&mut self, key: Handle<V>, value: V) {
        self.0.insert(key, value);
    }

    /// Almost equivalent to `as_hash_map().iter()`, but the iterator element
    /// type is `(Handle<V>, &V)` rather than `(&Handle<V>, &V)`
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter(self.0.iter())
    }

    /// The iterator element type is `(Handle<V>, &mut V)`.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut(self.0.iter_mut())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Remove and return the value with handle `handle`, or `None` if not
    /// found.
    #[inline]
    pub fn remove(&mut self, handle: Handle<V>) -> Option<V> {
        self.0.remove(&handle)
    }
}

/// The implementation uses [`iter()`(struct.RandMap.html#method.iter)
///
impl<'a, V> IntoIterator for &'a RandMap<V> {
    type Item = (Handle<V>, &'a V);
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<V> PartialEq for RandMap<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &RandMap<V>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .all(|(key, val)| other.get(key).map_or(false, |v| *val == *v))
    }
}

/// The type returned by [`RandMap::iter()`](struct.RandMap.html#method.iter).
///
pub struct Iter<'a, V>(hash_map::Iter<'a, Handle<V>, V>);
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (Handle<V>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (*k, v))
    }
}

/// The type returned by [`RandMap::iter_mut()`
/// ](struct.RandMap.html#method.iter_mut).
///
pub struct IterMut<'a, V>(hash_map::IterMut<'a, Handle<V>, V>);
impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (Handle<V>, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (*k, v))
    }
}

/// The handle to a [`RandMap`](Struct.RandMap.html) item is a typed `u64`.
#[derive(Debug)]
pub struct Handle<V>(u64, PhantomData<*const V>);

impl<V> Handle<V> {
    #[inline]
    pub fn from_u64(u: u64) -> Self {
        Self(u, PhantomData)
    }

    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl<V> Clone for Handle<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V> Copy for Handle<V> { }

impl<V> Eq for Handle<V> {}

impl<V> From<u64> for Handle<V> {
    fn from(item: u64) -> Handle<V> {
        Self(item, PhantomData)
    }
}

impl<V> From<Handle<V>> for u64 {
    fn from(item: Handle<V>) -> u64 {
        item.as_u64()
    }
}

impl<V> Hash for Handle<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<V> Ord for Handle<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<V> PartialEq for Handle<V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<V> PartialOrd for Handle<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> rand::distributions::Distribution<Handle<V>>
for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Handle<V> {
        Handle(rng.gen(), PhantomData)
    }
}

