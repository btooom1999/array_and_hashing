fn dfs(
    course: usize,
    map: &mut Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    added: &mut Vec<bool>,
    result: &mut Vec<i32>,
) -> bool {
    if visited[course] {
        result.clear();
        return false;
    }

    if map[course].is_empty() {
        if !added[course] {
            added[course] = true;
            result.push(course as i32);
        }
        return true;
    }

    visited[course] = true;
    for course in map[course].clone() {
        if !dfs(course, map, visited, added, result) {
            return false;
        }
    }

    if !added[course] {
        result.push(course as i32);
        added[course] = true;
    }
    visited[course] = false;
    map[course].clear();
    true
}

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut map = vec![vec![]; n];

    for courses in &prerequisites {
        map[courses[0] as usize].push(courses[1] as usize);
    }

    let mut visited = vec![false; n];
    let mut added = vec![false; n];
    let mut res = Vec::new();
    for i in (0..n).rev() {
        if !dfs(i, &mut map, &mut visited, &mut added, &mut res) {
            return res;
        }
    }

    res
}

pub fn main() {
    let num_courses = 4;
    let prerequisites = [[1,0],[2,0],[3,1],[3,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_order(num_courses, prerequisites));
}
