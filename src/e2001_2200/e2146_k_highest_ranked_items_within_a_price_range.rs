const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn highest_ranked_k_items(mut grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
    let (start_i, start_j) = (start[0] as usize, start[1] as usize);

    let mut heap = std::collections::BinaryHeap::new();
    if grid[start_i][start_j] >= pricing[0] && grid[start_i][start_j] <= pricing[1] {
        heap.push((0, grid[start_i][start_j], start_i, start_j));
    }

    let mut queue = std::collections::VecDeque::from([(start_i, start_j, 1)]);
    grid[start_i][start_j] = -1;

    while let Some((i, j, distance)) = queue.pop_front() {
        if heap.len() > k { heap.pop(); }
        for direct in DIRECTIONS {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < m && j < n && grid[i][j] > 0 {
                if grid[i][j] >= pricing[0] && grid[i][j] <= pricing[1] {
                    heap.push((distance, grid[i][j], i, j));
                }
                queue.push_back((i, j, distance+1));
                grid[i][j] = -1;
            }
        }
    }

    let mut res = Vec::new();
    while let Some((_, _, i, j)) = heap.pop() {
        res.push(vec![i as i32, j as i32]);
    }

    res.reverse();
    res
}

pub fn main() {
    let grid = [[1,2,0,1],[1,3,3,1],[0,2,5,1]].into_iter().map(Vec::from).collect();
    let pricing = [2,3].to_vec();
    let start = [2,3].to_vec();
    let k = 2;
    println!("{:?}", highest_ranked_k_items(grid, pricing, start, k));
}
