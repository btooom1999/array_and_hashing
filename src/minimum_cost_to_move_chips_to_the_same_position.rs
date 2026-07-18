fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let (mut even, mut odd) = (0, 0);
    for pos in position {
        if pos % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    even.min(odd)
}

pub fn main() {
    let position = [2,2,2,3,3].to_vec();
    println!("{}", min_cost_to_move_chips(position));
}
