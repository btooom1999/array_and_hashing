struct MyHashSet(Vec<bool>);

impl MyHashSet {
    fn new() -> Self {
        Self(vec![false; 1000001])
    }

    fn add(&mut self, key: i32) {
        self.0[(key) as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.0[(key) as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.0[(key) as usize]
    }
}

pub fn main() {
    let mut my_hashset = MyHashSet::new();
    my_hashset.add(1); // set = [1]
    my_hashset.add(2); // set = [1, 2]
    my_hashset.contains(1); // return True
    my_hashset.contains(3); // return False, (not found)
    my_hashset.add(2); // set = [1, 2]
    my_hashset.contains(2); // return True
    my_hashset.remove(2); // set = [1]
    my_hashset.contains(2); // return False, (already removed)
}
