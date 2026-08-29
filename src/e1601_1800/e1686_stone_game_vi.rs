use std::collections::BinaryHeap;

fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    for i in 0..alice_values.len() {
        heap.push((alice_values[i]+bob_values[i], i));
    }

    let (mut a, mut b) = (0, 0);
    let mut i = 0;
    while let Some((_, idx)) = heap.pop() {
        if i % 2 == 0 {
            a += alice_values[idx];
        } else {
            b += bob_values[idx];
        }
        i += 1;
    }

    if a > b {
        return 1;
    }

    if a == b {
        return 0;
    }

    -1
}

pub fn main() {
    // let alice_values = [1,2].to_vec();
    // let bob_values = [3,1].to_vec();
    let alice_values = [40,76,27,31,40,12,57,10,88,72,85,5,28,25,61,82,16,63,50,90,20,55,63].to_vec();
    let bob_values = [74,5,37,21,29,59,94,25,31,10,86,31,99,45,77,91,44,73,83,67,55,12,35].to_vec();
    println!("{}", stone_game_vi(alice_values, bob_values));
}
