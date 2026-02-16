use std::collections::HashSet;

fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..fruits.len() {
        for j in 0..baskets.len() {
            if fruits[i] <= baskets[j] {
                baskets[j] = -1;
                res += 1;
                break;
            }
        }
    }

    fruits.len() as i32 - res
}

pub fn main() {
    let fruits = [4,2,5].to_vec();
    let baskets = [3,5,4].to_vec();
    println!("{}", num_of_unplaced_fruits(fruits, baskets));
}
