use std::collections::HashMap;

fn least_bricks(mut wall: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = HashMap::<i64, i32>::new();
    for vec in &wall {
        let mut sum: i64 = 0;
        for (i, num) in vec.iter().enumerate().take(vec.len() - 1) {
            sum += *num as i64;
            *hashmap.entry(sum).or_default() += 1;
        }
    }

    wall.len() as i32 - hashmap.into_values().max().unwrap_or(0)
}
pub fn main() {
    // let wall = vec![
    //     vec![1, 2, 2, 1],
    //     vec![3, 1, 2],
    //     vec![1, 3, 2],
    //     vec![2, 4],
    //     vec![3, 1, 2],
    //     vec![1, 3, 1, 1],
    // ];
    let wall = vec![vec![2147483647, 2147483647, 2147483647, 2147483647]];

    // let wall = vec![vec![100_000_000], vec![100_000_000], vec![100_000_000]];

    println!("{}", least_bricks(wall));
}
