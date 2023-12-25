use std::iter::FromIterator;
use std::ops::Index;
use indexmap::IndexMap;
use indexmap::map::{Entry, IntoIter, Iter, IterMut, Keys, Values, ValuesMut};
use unicase::UniCase;

type Key = UniCase<String>;

#[derive(Debug, Default, Clone)]
pub struct UniCaseIndexMap<V> {
    inner: IndexMap<Key, V>,
}

impl<V> Eq for UniCaseIndexMap<V> where V: Eq {}

impl<V> PartialEq for UniCaseIndexMap<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &UniCaseIndexMap<V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(&***key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Extend<(K, V)> for UniCaseIndexMap<V>
where
    K: Into<Key>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        // Transform the keys into UniCases.
        let iter = iter.into_iter().map(|(k, v)| (k.into(), v));
        self.inner.extend(iter);
    }
}

impl<'a, K, V> Extend<(K, &'a V)> for UniCaseIndexMap<V>
where
    K: Into<Key>,
    V: Copy,
{
    fn extend<T: IntoIterator<Item = (K, &'a V)>>(&mut self, iter: T) {
        // Transform the keys into UniCases and copy the values.
        let iter = iter.into_iter().map(|(k, v)| (k.into(), *v));
        self.inner.extend(iter);
    }
}

impl<K, V> FromIterator<(K, V)> for UniCaseIndexMap<V>
where
    K: Into<Key>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

impl<'a, V> IntoIterator for &'a UniCaseIndexMap<V> {
    type Item = (&'a Key, &'a V);
    type IntoIter = Iter<'a, Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut UniCaseIndexMap<V> {
    type Item = (&'a Key, &'a mut V);
    type IntoIter = IterMut<'a, Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V> IntoIterator for UniCaseIndexMap<V> {
    type Item = (Key, V);
    type IntoIter = IntoIter<Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V> Index<K> for UniCaseIndexMap<V>
where
    K: Into<Key>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        let key = index.into();
        self.inner.index(&key)
    }
}

impl<V> UniCaseIndexMap<V> {
    /// Creates a new UniCaseBTreeMap with the default
    /// hasher and capacity.
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<V> UniCaseIndexMap<V> {
    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the map contains a value for the specified key.
    /// The key may be a String, str or UniCase value.
    pub fn contains_key<K: Into<Key>>(&self, k: K) -> bool {
        let key = k.into();
        self.inner.contains_key(&key)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<K: Into<Key>>(&mut self, k: K) -> Entry<'_, Key, V> {
        let key = k.into();
        self.inner.entry(key)
    }

    /// Returns a reference to the value corresponding to the key.
    /// The key may be a String, str or UniCase value.
    pub fn get<K: Into<Key>>(&self, k: K) -> Option<&V> {
        let key = k.into();
        self.inner.get(&key)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    /// The key may be a String, str or UniCase value.
    pub fn get_key_value<K: Into<Key>>(&self, k: K) -> Option<(&Key, &V)> {
        let key = k.into();
        self.inner.get_key_value(&key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    /// The key may be a String, str or UniCase value.
    pub fn get_mut<K: Into<Key>>(&mut self, k: K) -> Option<&mut V> {
        let key = k.into();
        self.inner.get_mut(&key)
    }

    /// Inserts a key-value pair into the map.
    /// If the map did not have this key present, None is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated, though; this matters for types that can be == without being identical.
    /// See the module-level documentation of [BTreeMap](https://doc.rust-lang.org/std/collections/index.html#insert-and-complex-keys)
    // for more.
    pub fn insert<K: Into<Key>>(&mut self, k: K, v: V) -> Option<V> {
        let key = k.into();
        self.inner.insert(key, v)
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is (&'a UniCase<String>, &'a V).
    pub fn iter(&self) -> Iter<Key, V> {
        self.inner.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values. The iterator element type is
    /// (&'a UniCase<String>, &'a mut V).
    pub fn iter_mut(&mut self) -> IterMut<Key, V> {
        self.inner.iter_mut()
    }

    /// An iterator visiting all keys in arbitrary order. The iterator element type is &'a UniCase<String>.
    pub fn keys(&self) -> Keys<Key, V> {
        self.inner.keys()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    /// The key may be a String, str or UniCase value.
    pub fn remove<K: Into<Key>>(&mut self, k: K) -> Option<V> {
        let key = k.into();
        self.inner.remove(&key)
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    /// The key may be a String, str or UniCase value.
    pub fn remove_entry<K: Into<Key>>(&mut self, k: K) -> Option<(Key, V)> {
        let key = k.into();
        self.inner.remove_entry(&key)
    }

    /// Retains only the elements specified by the predicate.
    /// In other words, remove all pairs (k, v) such that f(&k,&mut v) returns false.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Key, &mut V) -> bool,
    {
        self.inner.retain(f);
    }

    /// An iterator visiting all values in arbitrary order. The iterator element type is &'a V.
    pub fn values(&self) -> Values<Key, V> {
        self.inner.values()
    }

    /// An iterator visiting all values mutably in arbitrary order. The iterator element type is &'a mut V.
    pub fn values_mut(&mut self) -> ValuesMut<Key, V> {
        self.inner.values_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::UniCaseIndexMap;
    use unicase::UniCase;

    #[test]
    fn new() {
        let map = UniCaseIndexMap::<u8>::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn clear() {
        let mut map = UniCaseIndexMap::<u8>::new();
        assert_eq!(map.len(), 0);
        map.insert("A", 1);
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());

        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn contains_key_str() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        assert!(map.contains_key("A"));
        assert!(map.contains_key("a"));
        assert!(!map.contains_key("B"));
        assert!(!map.contains_key("Å"));
    }

    #[test]
    fn contains_key_string() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        assert!(map.contains_key("A".to_string()));
        assert!(map.contains_key("a".to_string()));
        assert!(!map.contains_key("B".to_string()));
        assert!(!map.contains_key("Å".to_string()));
    }

    #[test]
    fn entry() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        let entry = map.entry("A");
        assert_eq!(entry.key(), &UniCase::new("A".to_string()));
    }

    #[test]
    fn get_str() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        assert_eq!(map.get("A").unwrap(), &1);
        assert_eq!(map.get("a").unwrap(), &1);
        assert!(map.get("B").is_none());
        assert!(map.get("Å").is_none());
    }

    #[test]
    fn get_string() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        assert_eq!(map.get("A".to_string()).unwrap(), &1);
    }

    #[test]
    fn get_unicase() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        // Won't work with plain &str, which is annoying.
        let uc = UniCase::new("a".to_string());
        assert_eq!(map.get(uc).unwrap(), &1);
    }

    #[test]
    fn get_key_value() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        let result = map.get_key_value("a");
        assert_eq!(result.unwrap().0, &UniCase::new("a".to_string()));
        assert_eq!(result.unwrap().1, &1);
    }

    #[test]
    fn get_mut() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        assert_eq!(map.get_mut("a"), Some(&mut 1));
        assert!(map.get_mut("C").is_none());
    }

    #[test]
    fn insert_str() {
        let mut map = UniCaseIndexMap::<u8>::new();
        let result = map.insert("A", 1);
        assert!(result.is_none());
        let result = map.insert("B", 2);
        assert!(result.is_none());
        let result = map.insert("A", 20);
        assert_eq!(result, Some(1));

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn insert_string() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut map = UniCaseIndexMap::<u8>::new();
        assert!(map.is_empty());
        map.insert("A", 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn iter() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);

        let mut elems: Vec<_> = map.iter().map(|(_, v)| v.clone()).collect();
        elems.sort();
        assert_eq!(elems, vec![1, 2]);
    }

    #[test]
    fn iter_mut() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);

        for (_, v) in map.iter_mut() {
            *v += 10;
        }

        let mut elems: Vec<_> = map.iter().map(|(_, v)| v.clone()).collect();
        elems.sort();
        assert_eq!(elems, vec![11, 12]);
    }

    #[test]
    fn keys() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);

        let mut keys: Vec<_> = map.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                &UniCase::new("A".to_string()),
                &UniCase::new("B".to_string())
            ]
        );
    }

    #[test]
    fn len() {
        let mut map = UniCaseIndexMap::<u8>::new();
        assert_eq!(map.len(), 0);
        map.insert("A".to_string(), 1);
        assert_eq!(map.len(), 1);
        map.insert("B".to_string(), 2);
        assert_eq!(map.len(), 2);
        map.clear();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn remove() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        assert_eq!(map.remove("b"), Some(2));
        assert_eq!(map.remove("b"), None);
    }

    #[test]
    fn remove_entry() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        assert_eq!(map.remove("b"), Some(2));
        assert_eq!(map.remove("b"), None);
    }

    #[test]
    fn retain() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        map.insert("C".to_string(), 1);

        map.retain(|_, v| v == &1);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("A"), Some(&1));
        assert_eq!(map.get("B"), None);
        assert_eq!(map.get("C"), Some(&1));
    }

    #[test]
    fn values() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        map.insert("C".to_string(), 1);

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        assert_eq!(values, vec![1, 1, 2]);
    }

    #[test]
    fn values_mut() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);
        map.insert("B".to_string(), 2);
        map.insert("C".to_string(), 1);

        for v in map.values_mut() {
            *v += 10;
        }

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        assert_eq!(values, vec![11, 11, 12]);
    }

    #[test]
    fn partial_eq() {
        let mut map1 = UniCaseIndexMap::<u8>::new();
        map1.insert("A".to_string(), 1);
        map1.insert("B".to_string(), 2);
        map1.insert("C".to_string(), 3);

        let mut map2 = UniCaseIndexMap::<u8>::new();
        map2.insert("C".to_string(), 3);
        map2.insert("B".to_string(), 2);
        map2.insert("A".to_string(), 1);

        assert_eq!(map1, map2);
    }

    #[test]
    fn extend() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A".to_string(), 1);

        let v = vec![("A", 2), ("B", 3), ("C", 4)];

        map.extend(v);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), Some(&2));
    }

    #[test]
    fn index() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        map.insert("B", 2);

        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
    }

    #[test]
    fn into_iterator_impls() {
        let mut map = UniCaseIndexMap::<u8>::new();
        map.insert("A", 1);
        map.insert("B", 2);

        // These should all compile.
        for _ in &map {}
        for _ in &mut map {}
        for _ in map {}
    }

    #[test]
    fn from_iterator() {
        let v = vec![("A", 2), ("B", 3), ("C", 4)];

        let _map: UniCaseIndexMap<u8> = v.into_iter().collect();
    }
}
