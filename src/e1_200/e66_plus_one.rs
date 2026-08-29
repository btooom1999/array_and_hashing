use std::collections::VecDeque;

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = VecDeque::new();
    let mut excess = 0;
    let last = digits.len() - 1;
    for (i, num) in digits.iter().enumerate().rev() {
        let val = if last == i { num + 1 } else { num + excess };
        if val == 10 {
            excess = 1;
            res.push_front(0);
        } else {
            excess = 0;
            res.push_front(val);
        }
    }

    if excess == 1 {
        res.push_front(1);
    }

    res.into_iter().collect::<Vec<_>>()
}

pub fn main() {
    let digits = vec![1,0,0,0];
    println!("{:?}", plus_one(digits));
}

