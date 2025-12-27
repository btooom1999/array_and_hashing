fn time_required_to_buy(mut tickets: Vec<i32>, mut k: i32) -> i32 {
    let mut t = 0;
    for (i, num) in tickets.iter().enumerate() {
        if i as i32 <= k {
            t += std::cmp::min(tickets[i], tickets[k as usize]);
        } else {
            t += std::cmp::min(tickets[i], tickets[k as usize] - 1);
        }
    }

    t
}

pub fn main() {
    // let tickets = vec![2, 3, 2];
    let tickets = vec![5, 1, 1, 1];
    let k = 0;
    println!("{}", time_required_to_buy(tickets, k));
}
