const MOD: i64 = 1_000_000_007;
fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut powers = vec![];
    let mut i = 0;
    while n > 0 {
        if n & 1 == 1 {
            powers.push(1 << i);
        }
        i += 1;
        n >>= 1;
    }

    let mut res = Vec::new();
    for query in queries {
        let mut val = 1;
        for i in query[0]..=query[1] {
            val = val * powers[i as usize] % MOD;
        }

        res.push(val as i32);
    }

    res
}

pub fn main() {
    let n = 15;
    let queries = [[0,1], [2,2], [0,3]].into_iter().map(Vec::from).collect();
    println!("{:?}", product_queries(n, queries));
}
