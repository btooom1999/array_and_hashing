const DIRECTIONS: [(i32, i32); 8] = [(1,0), (-1,0), (0,1), (0,-1), (1,1), (1,-1), (-1,-1), (-1,1)];

fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    if m < 3 || n < 3 {
        return 0;
    }

    let mut res = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            if grid[i][j] > 0 && grid[i][j] < 10 {
                let mut nums = [false; 10];
                let mut sum_rows = [0; 3];
                let mut sum_cols = [0; 3];
                nums[0] = true;
                sum_rows[i%3] += grid[i][j];
                sum_cols[j%3] += grid[i][j];
                nums[grid[i][j] as usize] = true;
                for direct in DIRECTIONS {
                    let i = (i as i32 + direct.0) as usize;
                    let j = (j as i32 + direct.1) as usize;
                    if grid[i][j] > 9 || nums[grid[i][j] as usize] { break; }
                    nums[grid[i][j] as usize] = true;
                    sum_rows[i%3] += grid[i][j];
                    sum_cols[j%3] += grid[i][j];
                }

                if nums.iter().all(|&v| v)
                && sum_rows[0] == sum_rows[1] && sum_rows[1] == sum_rows[2]
                && sum_cols[0] == sum_cols[1] && sum_cols[1] == sum_cols[2]
                && grid[i-1][j-1] + grid[i][j] + grid[i+1][j+1] == 15
                && grid[i-1][j+1] + grid[i][j] + grid[i+1][j-1] == 15 {
                    res += 1;
                }
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]].into_iter().map(Vec::from).collect();
    println!("{}", num_magic_squares_inside(grid));
}
