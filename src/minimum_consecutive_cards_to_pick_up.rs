use std::collections::HashMap;

fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut res = i32::MAX;
    let mut hashmap = HashMap::new();

    for (j, num) in cards.into_iter().enumerate() {
        if let Some(&i) = hashmap.get(&num) {
            res = res.min((j-i+1) as i32);
        }

        hashmap.insert(num, j);
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let cards = [3,4,2,3,4,7].to_vec();
    println!("{}", minimum_card_pickup(cards));
}
