
fn wonderful_substrings(word: String) -> i64 {
    let mut bit = 0;
    let mut hashmap = [0; 1024];
    hashmap[0] = 1;
    let mut res = 0;
    for c in word.as_bytes() {
        let k = (c - b'a') as usize;
        bit ^= 1 << k;
        res += hashmap[bit];
        for i in 0..10 {
            res += hashmap[bit ^ (1 << i)];
        }

        hashmap[bit] += 1;
    }


    res
}

pub fn main() {
    let word = "aabb".to_string();
    println!("{}", wonderful_substrings(word));
}
