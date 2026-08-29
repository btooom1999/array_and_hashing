fn dfs(
    from: usize,
    to: usize,
    memo: &mut Vec<Vec<i32>>,
    map: &Vec<Vec<usize>>,
) -> bool {
    if memo[from][to] != -1 {
        return memo[from][to] == 1;
    }

    for next_from in map[from].clone() {
        if next_from == to || dfs(next_from, to, memo, map) {
            memo[from][to] = 1;
            return true;
        }
    }

    memo[from][to] = 0;
    false
}

fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = num_courses as usize;
    let mut map = vec![vec![]; n];

    let mut memo = vec![vec![-1; n]; n];
    for courses in &prerequisites {
        map[courses[0] as usize].push(courses[1] as usize);
        memo[courses[0] as usize][courses[1] as usize] = 1;
    }

    let mut res = vec![false; queries.len()];
    for i in 0..queries.len() {
        let (from, to) = (queries[i][0] as usize, queries[i][1] as usize);
        res[i] = dfs(from, to, &mut memo, &map);
    }

    res
}

pub fn main() {
    // let num_courses = 2;
    // let prerequisites = [[1,0].to_vec()].to_vec();
    // let queries = [[0,1].to_vec(),[1,0].to_vec()].to_vec();
    let num_courses = 4;
    let prerequisites = [[1,3],[1,0],[2,3],[0,2]].into_iter().map(Vec::from).collect();
    let queries = [[1,0],[1,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", check_if_prerequisite(num_courses, prerequisites, queries));
}
