use std::collections::HashMap;

fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, mut num_wanted: i32, use_limit: i32) -> i32 {
    let mut used = HashMap::<i32, i32>::new();
    let mut merged = (0..values.len()).map(|i| (values[i], labels[i])).collect::<Vec<_>>();
    merged.sort_unstable();

    let mut res = 0;
    while num_wanted > 0 && let Some((value, label)) = merged.pop() {
        *used.entry(label).or_default() += 1;

        if used[&label] <= use_limit {
            res += value;
            num_wanted -= 1;
        }
    }

    res
}

pub fn main() {
    let values = [5,4,3,2,1].to_vec();
    let labels = [1,1,2,2,3].to_vec();
    let num_wanted = 3;
    let use_limit = 1;
    println!("{}", largest_vals_from_labels(values, labels, num_wanted, use_limit));
}
