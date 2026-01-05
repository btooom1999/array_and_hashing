use std::collections::HashMap;

fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut hashmap = HashMap::<String, i64>::new();
    for rec in &rectangles {
        hashmap
            .entry((rec[0] as f64 / rec[1] as f64).to_string())
            .and_modify(|v| *v += 1)
            .or_default();
    }

    hashmap
        .values()
        .fold(0i64, |mut sum, v| sum + (v * (v + 1)) / 2)
}

pub fn main() {
    let rectangles = vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]];
    println!("{}", interchangeable_rectangles(rectangles));
}
