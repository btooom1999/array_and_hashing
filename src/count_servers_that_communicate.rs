use std::collections::{HashMap, HashSet};

fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut data = HashMap::new();
    let mut result = HashSet::new();
    for (sr, row) in grid.iter().enumerate() {
        for (sc, &col) in row.iter().enumerate() {
            if col == 1 {

                let sr_key = format!("sr{}", sr);
                let sc_key = format!("sc{}", sc);
                if let Some(&data) = data.get(&sr_key) {
                    result.insert(data);
                    result.insert((sr, sc));
                } else {
                    data.insert(sr_key, (sr, sc));
                }

                if let Some(&data) = data.get(&sc_key) {
                    result.insert(data);
                    result.insert((sr, sc));
                } else {
                    data.insert(sc_key, (sr, sc));
                }
            }
        }
    }

    result.len() as i32
}

pub fn main() {
    let grid = [[1,0],[0,1]].into_iter().map(Vec::from).collect();
    println!("{}", count_servers(grid));
}
