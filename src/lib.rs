pub struct UniqueVec<T> {
    items: Vec<T>
}

impl<T:PartialEq> UniqueVec<T> {
    /// Creates a new, empty UniqueVec.
    pub fn new() -> Self {
        UniqueVec { items: Vec::new() }
    }
    
    /// Adds an item if it's not already in the collection.
    pub fn add(&mut self, item: T) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    /// Removes an item if it exists.
    pub fn remove(&mut self, item: &T) {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
        }
    }

    /// Checks if the item is in the collection.
    pub fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }

    /// Returns all items as a slice.
    pub fn get_all(&self) -> &[T] {
        &self.items
    }
}

#[cfg(test)]
    mod tests {
    use super::*; // This brings UniqueVec into scope

    #[test]
    fn test_add_and_get_all() {
        let mut uv = UniqueVec::new();
        uv.add(1);
        uv.add(2);
        uv.add(1); // duplicate

        let items = uv.get_all();
        assert_eq!(items, &[1, 2]); // no duplicate
    }

    #[test]
    fn test_remove() {
        let mut uv = UniqueVec::new();
        uv.add("apple");
        uv.add("banana");
        uv.remove(&"apple");

        assert_eq!(uv.contains(&"apple"), false);
        assert_eq!(uv.contains(&"banana"), true);
    }

    #[test]
    fn test_contains() {
        let mut uv = UniqueVec::new();
        uv.add(42);
        assert!(uv.contains(&42));
        assert!(!uv.contains(&99));
    }
}
