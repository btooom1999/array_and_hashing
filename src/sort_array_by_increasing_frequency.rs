use std::collections::HashMap;

fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, i32>::new();
    for num in &nums {
        *hashmap.entry(*num).or_default() += 1;
    }

    let mut res = hashmap.into_iter().collect::<Vec<_>>();
    res.sort_by(|(a1, a2), (b1, b2)| a2.cmp(b2).then(b1.cmp(a1)));

    res.iter().fold(Vec::new(), |mut vec, x| {
        vec.append(&mut vec![x.0; x.1 as usize]);
        vec
    })
}

pub fn main() {
    let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
    println!("{:?}", frequency_sort(nums));
}
