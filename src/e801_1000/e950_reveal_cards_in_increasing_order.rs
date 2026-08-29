use std::collections::VecDeque;

fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    deck.sort();

    let mut res = VecDeque::from([deck[deck.len()-1]]);

    for i in (0..deck.len()-1).rev() {
        let adjacent = res.pop_back().unwrap();
        res.push_front(adjacent);
        res.push_front(deck[i]);
    }

    res.into()
}

pub fn main() {
    let deck = [17,13,11,2,3,5,7].to_vec();
    println!("{:?}", deck_revealed_increasing(deck));
}
