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
    let mut myHashSet = MyHashSet::new();
    myHashSet.add(1); // set = [1]
    myHashSet.add(2); // set = [1, 2]
    myHashSet.contains(1); // return True
    myHashSet.contains(3); // return False, (not found)
    myHashSet.add(2); // set = [1, 2]
    myHashSet.contains(2); // return True
    myHashSet.remove(2); // set = [1]
    myHashSet.contains(2); // return False, (already removed)
}
