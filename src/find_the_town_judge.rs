fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut count = vec![0i32; (n+1) as usize];
    for people in trust {
        count[people[0] as usize] -= 1;
        count[people[1] as usize] += 1;
    }

    for i in 1..=n {
        if count[i as usize] == n-1 {
            return i;
        }
    }

    -1
}

pub fn main() {
    let n = 2;
    let trust = [[1,2]].into_iter().map(Vec::from).collect::<Vec<_>>();
    println!("{:?}", find_judge(n, trust));
}
