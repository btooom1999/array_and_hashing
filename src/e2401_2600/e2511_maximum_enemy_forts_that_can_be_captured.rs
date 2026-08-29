fn capture_forts(forts: Vec<i32>) -> i32 {
    let n = forts.len();
    let mut res = 0;
    let (mut l_count_captured, mut l_flag) = (0, false);
    let (mut r_count_captured, mut r_flag) = (0, false);
    for i in 0..n {
        // left
        if forts[i] == 1 {
            l_count_captured = 0;
            l_flag = true;
        }

        if l_flag && forts[i] == 0 {
            l_count_captured += 1;
        }

        if l_flag && forts[i] == -1 {
            l_flag = false;
            res = res.max(l_count_captured);
        }

        // right
        if forts[n-i-1] == 1 {
            r_count_captured = 0;
            r_flag = true;
        }

        if r_flag && forts[n-i-1] == 0 {
            r_count_captured += 1;
        }

        if r_flag && forts[n-i-1] == -1 {
            r_flag = false;
            res = res.max(r_count_captured);
        }
    }

    res
}

pub fn main() {
    let forts = [1,0,0,-1,0,0,0,0,1].to_vec();
    println!("{}", capture_forts(forts));
}
