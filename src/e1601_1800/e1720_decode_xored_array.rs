fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut res = vec![-1; encoded.len()+1];
    res[0] = first;
    for i in 0..encoded.len() {
        res[i+1] = encoded[i] ^ res[i];
    }

    res
}

pub fn main() {
    let encoded = [1,2,3].to_vec();
    let first = 1;
    println!("{:?}", decode(encoded, first));
}
