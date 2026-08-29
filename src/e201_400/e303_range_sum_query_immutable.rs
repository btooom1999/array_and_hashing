struct NumArray(Vec<i32>);

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self(nums)
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.0[left as usize..(right + 1) as usize].iter().sum()
    }
}

pub fn main() {
    let num_array = NumArray(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", num_array.sum_range(0, 2));
    println!("{}", num_array.sum_range(2, 5));
    println!("{}", num_array.sum_range(0, 5));
}
