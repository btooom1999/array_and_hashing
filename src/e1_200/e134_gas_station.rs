fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut min = 0;
    let mut visited = vec![false; n];
    visited[min] = true;

    loop {
        let mut tank = gas[min];
        for _ in 0..n {
            tank -= cost[min];
            min = (min + 1) % n;
            if tank < 0 {
                if visited[min] {
                    return -1;
                }

                visited[min] = true;
                break;
            }
            tank += gas[min];
        }

        if tank >= 0 {
            return min as i32;
        }
    }
}

pub fn main() {
    let gas = [1,2,3,4,5].to_vec();
    let cost = [3,4,5,1,2].to_vec();
    println!("{}", can_complete_circuit(gas, cost));
}
