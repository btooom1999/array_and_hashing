fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
    special.sort_unstable();

    let n = special.len();
    let mut max = (special[0]-bottom).max(top-special[n-1]);
    for i in 1..n {
        max = max.max(special[i]-special[i-1]-1);
    }

    max
}

pub fn main() {
    let bottom = 2;
    let top = 9;
    let special = [4,6].to_vec();
    println!("{}", max_consecutive(bottom, top, special));
}
