use std::collections::HashMap;

fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut hashmap = Vec::with_capacity(names.len());
    for (i, (name, height)) in names.iter().zip(heights.iter()).enumerate() {
        hashmap.push((name.to_string(), *height));
    }

    hashmap.sort_by(|(_, a), (_, b)| b.cmp(a));

    hashmap
        .into_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>()
}

pub fn main() {
    let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights = vec![180, 165, 170];
    println!("{:?}", sort_people(names, heights));
}
