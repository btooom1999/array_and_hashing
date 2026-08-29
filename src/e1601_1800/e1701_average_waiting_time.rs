fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut sum = 0;
    let mut time = 0;
    for customer in &customers {
        time = std::cmp::max(time, customer[0] as i64) + customer[1] as i64;
        sum += time - customer[0] as i64;
    }

    sum as f64 / customers.len() as f64
}

pub fn main() {
    // let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
    let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
    println!("{}", average_waiting_time(customers));
}
