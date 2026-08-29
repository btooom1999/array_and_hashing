fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    let mut count = 0;
    let mut hashmap = std::collections::HashMap::<_, Vec<char>>::new();
    let mut pairs = 0;
    let mut three = false;

    let n = ranks.len();
    for i in 0..n {
        count += (suits[i] == suits[0]) as usize;
        let vec = hashmap.entry(ranks[i]).or_default();
        vec.push(suits[i]);
        if vec.len() == 2 { pairs += 1; }
        if vec.len() == 3 { three = true; }
    }

    if count == n {
        return "Flush".to_string();
    }

    if three {
        return "Three of a Kind".to_string();
    }

    if pairs > 0 {
        return "Pair".to_string();
    }

    "High Card".to_string()
}

pub fn main() {
    let ranks = [13,2,3,1,9].to_vec();
    let suits = ["a","a","a","a","a"].into_iter().map(|v| v.chars().next().unwrap()).collect();
    println!("{}", best_hand(ranks, suits));
}
