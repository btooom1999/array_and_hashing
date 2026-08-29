fn dfs(
    city: usize,
    dp: &mut Vec<i32>,
    map: &Vec<Vec<usize>>,
) -> i32 {
    if dp[city] > -1 {
        return dp[city];
    }

    let mut min = i32::MAX;
    for city in map[city].clone() {
        min = min.min(1 + dfs(city, dp, map));
    }

    dp[city] = min;
    min
}

fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut map = vec![vec![]; n];
    for i in 0..n {
        map[i].push(i+1);
    }

    let mut res = Vec::new();
    for cities in queries {
        let mut dp = vec![-1; n];
        dp[n-1] = 0;

        map[cities[0] as usize].push(cities[1] as usize);
        res.push(dfs(0, &mut dp, &map));
    }

    res
}

pub fn main() {
    let n = 5;
    let queries = [[2,4],[0,2],[0,4]].into_iter().map(Vec::from).collect();
    println!("{:?}", shortest_distance_after_queries(n, queries));
}
