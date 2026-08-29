struct MovingAverage(i32, Vec<i32>);

impl MovingAverage {
    fn new(n: i32) -> Self {
        Self(n, Vec::new())
    }

    fn next(&mut self, val: i32) -> f64 {
        self.1.push(val);

        let i = std::cmp::max(0, self.1.len() as i32 - self.0);
        let sum = self.1[i as usize..].iter().sum::<i32>();
        let size = std::cmp::min(self.1.len() as i32, self.0);

        sum as f64 / size as f64
    }
}

pub fn main() {
    let mut moving_average = MovingAverage::new(3);
    println!("{}", moving_average.next(1));
    println!("{}", moving_average.next(10));
    println!("{}", moving_average.next(3));
    println!("{}", moving_average.next(5));
}
