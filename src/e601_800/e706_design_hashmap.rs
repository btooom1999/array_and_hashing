struct MyHashMap(Vec<i32>);

impl MyHashMap {
    fn new() -> Self {
        Self(vec![-1; 1_000_001])
    }

    fn put(&mut self, key: i32, value: i32) {
        self.0[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.0[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.0[key as usize] = -1;
    }
}

pub fn main() {
    let mut my_hashmap = MyHashMap::new();
    my_hashmap.put(1, 1); // The map is now [[1,1]]
    my_hashmap.put(2, 2); // The map is now [[1,1], [2,2]]
    my_hashmap.get(1); // return 1, The map is now [[1,1], [2,2]]
    my_hashmap.get(3); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
    my_hashmap.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
    my_hashmap.get(2); // return 1, The map is now [[1,1], [2,1]]
    my_hashmap.remove(2); // remove the mapping for 2, The map is now [[1,1]]
    my_hashmap.get(2); // return -1 (i.e., not found), The map is now [[1,1]]
}
