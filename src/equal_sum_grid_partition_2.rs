fn check(
    left: (i64, usize, usize, usize, usize),
    right: (i64, usize, usize, usize, usize),
    current: &[i32],
    hashmap: &[i32],
    grid: &Vec<Vec<i64>>,
) -> bool {
    let remaining = (left.0 - right.0).abs();
    if remaining >= 100_000 {
        return false;
    }
    if left.0 > right.0 && current[remaining as usize] > 0 {
        if (left.1 == left.2 || left.3 == left.4) && grid[left.1][left.3] != remaining && grid[left.2][left.4] != remaining {
            return false;
        }
        return true;
    }

    if left.0 < right.0 && hashmap[remaining as usize] > 0 {
        if (right.1 == right.2 || right.3 == right.4) && grid[right.1][right.3] != remaining && grid[right.2][right.4] != remaining {
            return false;
        }

        return true;
    }

    false
}

fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let mut total = 0;
    let (m, n) = (grid.len(), grid[0].len());
    let mut hashmap = [0; 100_001];
    hashmap[0] = 1;
    let grid = grid
        .into_iter()
        .map(|v| v
            .into_iter()
            .map(|num| {
                total += num as i64;
                hashmap[num as usize] += 1;
                num as i64
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut current = [0; 100_001];
    current[0] = 1;

    let mut left = 0;
    for j in 0..n {
        for i in 0..m {
            current[grid[i][j] as usize] += 1;
            hashmap[grid[i][j] as usize] -= 1;
            left += grid[i][j];
        }

        if total - left == left {
            return true;
        }

        if check((left, 0, m-1, 0, j), (total-left, 0, m-1, j+1, n-1), &current, &hashmap, &grid) {
            return true;
        }
    }

    (hashmap, current) = (current, hashmap);
    left = 0;
    for i in 0..m {
        for j in 0..n {
            current[grid[i][j] as usize] += 1;
            hashmap[grid[i][j] as usize] -= 1;
            left += grid[i][j];
        }

        if total - left == left {
            return true;
        }

        if check((left, 0, i, 0, n-1), (total-left, i+1, m-1, 0, n-1), &current, &hashmap, &grid) {
            return true;
        }
    }

    false
}

pub fn main() {
    // let grid = [[4],[3],[4],[4],[4]].into_iter().map(Vec::from).collect();
    // let grid = [[1,4],[2,3]].into_iter().map(Vec::from).collect();
    let grid = [[253,10,10]].into_iter().map(Vec::from).collect();
    println!("{}", can_partition_grid(grid));
}
