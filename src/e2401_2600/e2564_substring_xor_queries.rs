use std::collections::HashMap;

fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let s = s.as_bytes();
    let n = s.len();
    let mut hashmap = HashMap::new();

    for i in 0..n {
        let mut bit = (s[i] == b'1') as i32;
        hashmap.entry(bit).or_insert([i as i32, i as i32]);
        if s[i] != b'0' {
            for j in i+1..n.min(i+32) {
                bit = (bit << 1) ^ (s[j] == b'1') as i32;
                hashmap.entry(bit).or_insert([i as i32, j as i32]);
            }
        }
    }

    let mut res = Vec::new();
    for query in queries {
        if let Some(&data) = hashmap.get(&(query[0] ^ query[1])) {
            res.push(data.to_vec());
        } else {
            res.push(vec![-1, -1]);
        }
    }

    res
}

pub fn main() {
    let s = "010111010101000000001011001111111101010101010011011001010001110101111010000100101011011".to_string();
    let queries = [[3, 3338]].into_iter().map(Vec::from).collect();
    println!("{:?}", substring_xor_queries(s, queries));
}
