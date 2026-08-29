fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let n = flips.len();
    let mut max = 0;
    let mut res = 0;
    for i in 0..n {
        max = max.max(flips[i] as usize);
        if n-i-1 == n-max {
            res += 1;
        }
    }

    res
}

pub fn main() {
    // let flips = [3,2,4,1,5].to_vec();
    let flips = [1,2,3,4,5,6,18,8,30,10,11,12,13,14,17,16,15,7,19,20,41,22,23,24,33,26,27,25,29,9,31,32,28,34,35,36,37,38,39,40,21,42].to_vec();
    println!("{}", num_times_all_blue(flips));
}
