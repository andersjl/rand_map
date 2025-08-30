A map that creates a random handle on insertion to use when retrieving.

The raison d'être for this map is:

- You want to put something in a map, but you have no key. Means you do
  not want to use a [`HashMap`
  ](https://doc.rust-lang.org/std/collections/struct.HashMap.html) or
  [`BTreeMap`
  ](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).

- You want to forget the details of what you put in to later retrieve it
  with a simple handle, and you are not interested in how many equal
  objects you insert. Means you do not want to use a [`HashSet`
  ](https://doc.rust-lang.org/std/collections/struct.HashSet.html) or
  [`HashMultiSet`
  ](https://docs.rs/multiset/latest/multiset/struct.HashMultiSet.html).

- You want a persistent handle to refer to the item you put in the map.
  Means you do not want to use a `Vec`.

The implementation uses a `HashMap` that does not actually hash. The
contained `HashMap` can be borrowed (`.as_hash_map()`), so all `HashMap`
functions that do not change the map are at your disposal.

