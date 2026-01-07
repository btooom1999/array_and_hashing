const N: i32 = 10_000;
struct KthLargest {
    k: i32,
    data: Vec<i32>
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut data = vec![0; (N * 2 + 1) as usize];
        for num in &nums {
            data[(num + N) as usize] += 1;
        }

        Self { k, data }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.data[(val + N) as usize] += 1;

        let mut count = self.k;
        for (i, k) in self.data.iter().enumerate().rev() {
            count -= k;
            if count <= 0 {
                return i as i32 - N;
            }
        }

        -1
    }
}

pub fn main() {
    let mut kthLargest = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
    println!("{}", kthLargest.add(2)); // return 7
    println!("{}", kthLargest.add(10)); // return 7
    println!("{}", kthLargest.add(9)); // return 7
    println!("{}", kthLargest.add(9)); // return 8
}
