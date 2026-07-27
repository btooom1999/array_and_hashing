const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let mut trees = Vec::new();
    let (m, n) = (forest.len(), forest[0].len());
    for i in 0..m {
        for j in 0..n {
            if forest[i][j] > 1 {
                trees.push((forest[i][j], i, j));
            }
        }
    }

    trees.push((0, 0, 0));
    trees.sort_by_key(|v| v.0);
    // println!("{:?}", trees);

    let mut res = 0;
    for k in 1..trees.len() {
        let current = trees[k-1];

        let mut queue = std::collections::VecDeque::from([(current.1, current.2, 0)]);
        let mut visited = vec![vec![false; n]; m];
        visited[current.1][current.2] = true;
        let mut is_fail = true;

        while let Some((i, j, step)) = queue.pop_front() {
            if i == trees[k].1 && j == trees[k].2 {
                res += step;
                is_fail = false;
                break;
            }


            for direct in DIRECTIONS {
                let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if i < m && j < n && !visited[i][j] && forest[i][j] > 0 {
                    queue.push_back((i, j, step+1));
                    visited[i][j] = true;
                }
            }
        }

        if is_fail {
            return -1;
        }
    }

    res
}

pub fn main() {
    let forest = [[0,2,8],[0,0,4],[7,6,5]].into_iter().map(Vec::from).collect();
    println!("{}", cut_off_tree(forest));
}
