fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..code.len() {
        let i = i as i32;
        let mut sum = 0;
        if k < 0 {
            for j in i+k..i {
                let x = if j < 0 { (code.len() as i32 + j) as usize } else { j as usize };
                sum += code[x];
            }
        } else if k > 0 {
            for j in i+1..i+k+1 {
                let x = if j >= code.len() as i32 { j as usize - code.len() } else { j as usize };
                sum += code[x];
            }
        }

        res.push(sum);
    }

    res
}

pub fn main() {
    let code = [5,7,1,4].to_vec();
    let k = -3;
    println!("{:?}", decrypt(code, k));
}
