use std::iter::FromIterator;
use indexmap::IndexSet;
use indexmap::set::{IntoIter, Iter};
use unicase::UniCase;

type Key = UniCase<String>;

#[derive(Debug, Default, Clone)]
pub struct UniCaseIndexSet {
    inner: IndexSet<Key>,
}

impl PartialEq for UniCaseIndexSet {
    fn eq(&self, other: &UniCaseIndexSet) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|key| other.contains(key.as_ref()))
    }
}

impl<K> Extend<K> for UniCaseIndexSet
where
    K: Into<Key>,
{
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        // Transform the keys into UniCases.
        let iter = iter.into_iter().map(|k| k.into());
        self.inner.extend(iter);
    }
}

impl<K> FromIterator<K> for UniCaseIndexSet
where
    K: Into<Key>,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

impl<'a> IntoIterator for &'a UniCaseIndexSet {
    type Item = &'a Key;
    type IntoIter = Iter<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for UniCaseIndexSet {
    type Item = Key;
    type IntoIter = IntoIter<Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl UniCaseIndexSet {
    /// Creates a new UniCaseBTreeSet with the default
    /// hasher and capacity.
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl UniCaseIndexSet {
    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the map contains a value for the specified key.
    /// The key may be a String, str or UniCase value.
    pub fn contains<K: Into<Key>>(&self, k: K) -> bool {
        let key = k.into();
        self.inner.contains(&key)
    }

    /// Returns a reference to the value corresponding to the key.
    /// The key may be a String, str or UniCase value.
    pub fn get<K: Into<Key>>(&self, k: K) -> Option<&Key> {
        let key = k.into();
        self.inner.get(&key)
    }

    // Adds a value to the set.
    // Returns whether the value was newly inserted. That is:
    // If the set did not previously contain an equal value, true is returned.
    // If the set already contained an equal value, false is returned, and the entry is not updated.
    pub fn insert<K: Into<Key>>(&mut self, k: K) -> bool {
        let key = k.into();
        self.inner.insert(key)
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is (&'a UniCase<String>, &'a V).
    pub fn iter(&self) -> Iter<Key> {
        self.inner.iter()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    /// The key may be a String, str or UniCase value.
    pub fn remove<K: Into<Key>>(&mut self, k: K) -> bool {
        let key = k.into();
        self.inner.remove(&key)
    }

    /// Retains only the elements specified by the predicate.
    /// In other words, remove all pairs (k, v) such that f(&k,&mut v) returns false.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Key) -> bool,
    {
        self.inner.retain(f);
    }
}

#[cfg(test)]
mod tests {
    use super::UniCaseIndexSet;
    use unicase::UniCase;

    #[test]
    fn new() {
        let map = UniCaseIndexSet::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn clear() {
        let mut map = UniCaseIndexSet::new();
        assert_eq!(map.len(), 0);
        map.insert("A");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());

        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn contains_str() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        assert!(map.contains("A"));
        assert!(map.contains("a"));
        assert!(!map.contains("B"));
        assert!(!map.contains("Å"));
    }

    #[test]
    fn contains_string() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        assert!(map.contains("A".to_string()));
        assert!(map.contains("a".to_string()));
        assert!(!map.contains("B".to_string()));
        assert!(!map.contains("Å".to_string()));
    }

    #[test]
    fn get_str() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        assert_eq!(map.get("A"), Some(&UniCase::new("A".to_string())));
        assert_eq!(map.get("a"), Some(&UniCase::new("a".to_string())));
        assert!(map.get("B").is_none());
        assert!(map.get("Å").is_none());
    }

    #[test]
    fn get_string() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        assert_eq!(map.get("A".to_string()).unwrap(), &UniCase::new("a".to_string()));
    }

    #[test]
    fn get_unicase() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        // Won't work with plain &str, which is annoying.
        let uc = UniCase::new("a".to_string());
        assert_eq!(map.get(uc).unwrap(), &UniCase::new("A".to_string()));
    }

    #[test]
    fn get_key_value() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        let result = map.get("a");
        assert_eq!(result, Some(&UniCase::new("a".to_string())));
    }

    #[test]
    fn insert_str() {
        let mut map = UniCaseIndexSet::new();
        let result = map.insert("A");
        assert_eq!(result, true);
        let result = map.insert("B");
        assert_eq!(result, true);
        let result = map.insert("A");
        assert_eq!(result, false);

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn insert_string() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());
        map.insert("B".to_string());
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut map = UniCaseIndexSet::new();
        assert!(map.is_empty());
        map.insert("A");
        assert!(!map.is_empty());
    }

    #[test]
    fn iter() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());
        map.insert("B".to_string());

        let mut elems: Vec<_> = map.iter().map(|v| v.clone()).collect();
        elems.sort();
        assert_eq!(elems, vec![UniCase::new("a".to_string()), UniCase::new("b".to_string())]);
    }

    #[test]
    fn keys() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());
        map.insert("B".to_string());

        let mut keys: Vec<_> = map.iter().collect();
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
        let mut map = UniCaseIndexSet::new();
        assert_eq!(map.len(), 0);
        map.insert("A".to_string());
        assert_eq!(map.len(), 1);
        map.insert("B".to_string());
        assert_eq!(map.len(), 2);
        map.clear();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn remove() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());
        map.insert("B".to_string());
        assert_eq!(map.remove("b"), true);
        assert_eq!(map.remove("b"), false);
    }

    #[test]
    fn remove_entry() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());
        map.insert("B".to_string());
        assert_eq!(map.remove("b"), true);
        assert_eq!(map.remove("b"), false);
    }

    #[test]
    fn partial_eq() {
        let mut map1 = UniCaseIndexSet::new();
        map1.insert("A".to_string());
        map1.insert("B".to_string());
        map1.insert("C".to_string());

        let mut map2 = UniCaseIndexSet::new();
        map2.insert("C".to_string());
        map2.insert("B".to_string());
        map2.insert("A".to_string());

        assert_eq!(map1, map2);
    }

    #[test]
    fn extend() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A".to_string());

        let v = vec!["A", "B", "C"];

        map.extend(v);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), Some(&UniCase::new("a".to_string())));
    }

    #[test]
    fn into_iterator_impls() {
        let mut map = UniCaseIndexSet::new();
        map.insert("A");
        map.insert("B");

        // These should all compile.
        for _ in &map {}
        for _ in map {}
    }

    #[test]
    fn from_iterator() {
        let v = vec!["A", "B", "C"];

        let _map: UniCaseIndexSet = v.into_iter().collect();
    }
}
