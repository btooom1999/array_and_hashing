fn dfs(
    cookies: &[i32],
    k: usize,
    i: usize,
    distribution: &mut Vec<i32>,
) -> i32 {
    if i == cookies.len() {
        return *distribution.iter().max().unwrap();
    }

    let mut res = i32::MAX;
    for j in 0..k {
        distribution[j] += cookies[i];
        res = res.min(dfs(cookies, k, i+1, distribution));
        distribution[j] -= cookies[i];
    }

    res
}

fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    dfs(&cookies, k, 0, &mut vec![0; k])
}

pub fn main() {
    let cookies = [8,15,10,20,8].to_vec();
    let k = 2;
    println!("{}", distribute_cookies(cookies, k));
}
