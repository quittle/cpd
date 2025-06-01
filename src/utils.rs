use std::collections::HashMap;
use std::hash::Hash;

pub trait VecExt<T> {
    /// Removes the first occurrence of `target` from the vector, returning it if found.
    fn remove_first_match<F>(&mut self, predicate: F) -> Option<T>
    where
        F: FnMut(&T) -> bool;

    fn require_mut(&mut self, index: usize) -> &mut T;
}

impl<T> VecExt<T> for Vec<T> {
    fn remove_first_match<F>(&mut self, predicate: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        let index = self.iter().position(predicate)?;

        Some(self.remove(index))
    }

    fn require_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

pub trait HashMapExt<K, V>
where
    K: Hash + Eq,
{
    /// Returns a mutable reference to the value corresponding to the key, or inserts the default value if the key does not exist.
    fn require_mut(&mut self, key: &K) -> &mut V;
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn require_mut(&mut self, key: &K) -> &mut V {
        self.get_mut(key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_first_match() {
        let mut vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.remove_first_match(|&x| x == 3), Some(3));
        assert_eq!(vec, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_require_mut() {
        let mut vec = vec![1, 2, 3];
        let member = vec.require_mut(1);
        assert_eq!(*member, 2);
        *member = 9;
        assert_eq!(vec[1], 9);

        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("entry", 1);
        let member = map.require_mut(&"entry");
        assert_eq!(*member, 1);
        *member = 9;
        assert_eq!(map[&"entry"], 9);
    }
}
