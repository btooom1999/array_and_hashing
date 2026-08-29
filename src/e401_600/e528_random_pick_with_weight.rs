#[derive(Debug)]
struct Solution {
    prefix: Vec<i32>
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        for &item in &w {
            prefix.push(item + prefix.last().unwrap_or(&0));
        }

        Self { prefix }
    }

    fn pick_index(&self) -> i32 {
        let target = rand::random_range(1..=*self.prefix.last().unwrap());
        let mut l = 0;
        let mut r = self.prefix.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if self.prefix[m] >= target {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l as i32
    }
}

pub fn main() {
    let solution = Solution::new(vec![1,3]);
    println!("{}", solution.pick_index());
}
