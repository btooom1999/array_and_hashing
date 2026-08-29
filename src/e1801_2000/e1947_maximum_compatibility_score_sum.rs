fn calc_point(n: usize, num1: i32, num2: i32) -> i32 {
    let mut count = 0;
    for i in 0..n {
        if num1 >> i & 1 == num2 >> i & 1 {
            count += 1;
        }
    }

    count
}
fn dfs(
    m: usize,
    n: usize,
    i: usize,
    students: &Vec<i32>,
    mentors: &Vec<i32>,
    visited: &mut Vec<bool>,
) -> i32 {
    if i == m {
        return 0;
    }

    let mut res = 0;
    for j in 0..m {
        if !visited[j] {
            visited[j] = true;
            res = res.max(calc_point(n, mentors[i], students[j]) + dfs(m, n, i+1, students, mentors, visited));
            visited[j] = false;
        }
    }

    res
}

fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (students.len(), students[0].len());
    let students = students
        .into_iter()
        .map(|v| v
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, num)| acc ^ (num << (n-i-1)))
        )
        .collect::<Vec<_>>();

    let mentors = mentors
        .into_iter()
        .map(|v| v
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, num)| acc ^ (num << (n-i-1)))
        )
        .collect::<Vec<_>>();

    dfs(m, n, 0, &students, &mentors, &mut vec![false; m])
}

pub fn main() {
    let students = [[1,1,0],[1,0,1],[0,0,1]].into_iter().map(Vec::from).collect();
    let mentors = [[1,0,0],[0,0,1],[1,1,0]].into_iter().map(Vec::from).collect();
    println!("{}", max_compatibility_sum(students, mentors));
}
