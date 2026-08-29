use std::collections::HashMap;

fn prison_after_n_days(mut cells: Vec<i32>, mut n: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    while n > 0 {
        if let Some(&prev_n) = seen.get(&cells) {
            let cycle = prev_n - n;
            n = (n-1) % cycle + 1;
        } else {
            seen.insert(cells.clone(), n);
        }

        let mut next = vec![0;8];
        for i in 1..7 {
            next[i] = if cells[i-1] == cells[i+1] { 1 } else { 0 }
        }
        cells = next;
        n -= 1;
    }

    cells
}

pub fn main() {
    let cells = [1,0,0,1,0,0,1,0].to_vec();
    let n = 1000000000;
    println!("{:?}", prison_after_n_days(cells, n));
}
