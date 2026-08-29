fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut prefix = vec![0; s.len()+1];
    for (i, c) in s.chars().enumerate() {
        prefix[i+1] = prefix[i] ^ (1 << (c as u8 - b'a'));
    }

    let mut res = vec![false; queries.len()];
    for (i, query) in queries.into_iter().enumerate() {
        let (x, y, k) = (query[0] as usize, query[1] as usize, query[2]);
        let num = prefix[y+1] ^ prefix[x];
        let mut count = 0;
        for i in 0..32 {
            if num & 1 << i > 0 {
                count += 1;
            }
        }

        let amount = count / 2;
        res[i] = amount <= k;
    }

    res
}

pub fn main() {
    let s = "abcda".to_string();
    let queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", can_make_pali_queries(s, queries));
}
