fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let n = encoded.len();
    let x = (1..=n+1).fold(0, |acc, i| acc ^ i as i32 ^ if i % 2 == 1 { *encoded.get(i).unwrap_or(&0) } else { 0 });
    let mut res = vec![x];
    for &num in &encoded {
        res.push(res[res.len()-1] ^ num);
    }

    res
}

pub fn main() {
    // let encoded = [3,1].to_vec();
    let encoded = [6,5,4,6].to_vec();
    println!("{:?}", decode(encoded));
}
