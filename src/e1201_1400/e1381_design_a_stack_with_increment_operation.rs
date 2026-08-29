struct CustomStack {
    nums: Vec<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self { nums: Vec::with_capacity(max_size as usize) }
    }

    fn push(&mut self, x: i32) {
        if self.nums.capacity() != self.nums.len() {
            self.nums.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.nums.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..k as usize {
            self.nums[i] += val;
        }
    }
}

pub fn main() {

}
