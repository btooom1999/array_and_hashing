fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let mut result = vec![0; grid.len()];
    for team in grid {
        for (i, stronger) in team.into_iter().enumerate() {
            if stronger == 1 {
                result[i] -= 1;
            }
        }
    }

    let mut res = 0;
    for i in 1..result.len() {
        if result[i] > result[res] {
            res = i;
        }
    }

    res as i32
}

pub fn main() {
    let grid = [[0,1],[0,0]].into_iter().map(Vec::from).collect();
    println!("{}", find_champion(grid));
}
