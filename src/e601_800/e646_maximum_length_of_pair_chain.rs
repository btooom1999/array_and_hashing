fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_by(|a,b | a[0].cmp(&b[0]));

    let n = pairs.len();
    let mut dp = vec![0; n];
    dp[n-1] = 1;

    let mut res = 0;
    for i in (0..n-1).rev() {
        for j in i..n {
            if pairs[i][1] < pairs[j][0] {
                dp[i] = dp[i].max(1+dp[j]);
            }

            res = res.max(dp[i]);
        }
    }

    res
}

pub fn main() {
    // let pairs = [[1,2],[7,8],[4,5]].into_iter().map(Vec::from).collect::<Vec<_>>();
    let pairs = [[-10,-8],[8,9],[-5,0],[6,10],[-6,-4],[1,7],[9,10],[-4,7]].into_iter().map(Vec::from).collect();
    println!("{}", find_longest_chain(pairs));
}
