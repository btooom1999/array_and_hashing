fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut map = vec![0i64; n];

    for road in roads {
        map[road[0] as usize] += 1;
        map[road[1] as usize] += 1;
    }

    map.sort_unstable();
    let mut res = 0;
    for (point, level) in map.into_iter().enumerate() {
        res += level * (point + 1) as i64;
    }

    res
}

pub fn main() {
    let n = 25;
    let roads = [[0,18],[18,1],[1,19],[21,20],[20,7],[7,8],[3,22],[22,13],[13,1],[13,9],[9,16],[16,7],[24,17],[17,3],[3,21],[22,6],[6,11],[11,7],[23,4],[4,15],[15,12],[5,7],[7,21],[21,1]].map(Vec::from).into_iter().collect();
    println!("{:?}", maximum_importance(n, roads));
}
