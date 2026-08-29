fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
    let mut total = 0;
    if start_pos[0] <= home_pos[0] {
        for i in start_pos[0]+1..=home_pos[0] {
            total += row_costs[i as usize];
        }
    } else {
        for i in home_pos[0]..start_pos[0] {
            total += row_costs[i as usize];
        }
    }

    if start_pos[1] <= home_pos[1] {
        for i in start_pos[1]+1..=home_pos[1] {
            total += col_costs[i as usize];
        }
    } else {
        for i in home_pos[1]..start_pos[1] {
            total += col_costs[i as usize];
        }
    }

    total
}

pub fn main() {
    let start_pos = [1, 0].to_vec();
    let home_pos = [2, 3].to_vec();
    let row_costs = [5, 4, 3].to_vec();
    let col_costs = [8, 2, 6, 7].to_vec();
    println!("{}", min_cost(start_pos, home_pos, row_costs, col_costs));
}
