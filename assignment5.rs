use std::collections::HashMap;

// Define a trait SortByKey
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement the SortByKey trait for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut sorted_pairs: Vec<_> = self.drain().collect();
        sorted_pairs.sort_by_key(|(k, _)| k.clone());
        self.extend(sorted_pairs);
    }
}

fn main() {
    // Create a new instance of HashMap
    let mut my_map: HashMap<i32, &str> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "three");
    my_map.insert(1, "one");
    my_map.insert(2, "two");

    println!("Original map: {:?}", my_map);

    // Sort the map by keys using the SortByKey trait
    my_map.sort_by_key();

    println!("Sorted map: {:?}", my_map);
}
