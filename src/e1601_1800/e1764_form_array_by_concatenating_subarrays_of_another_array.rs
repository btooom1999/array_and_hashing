fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let mut k = 0;
    let n = nums.len();
    'outer: for group in groups {
        let m = group.len();
        let mut lps = vec![0; n-k];

        if n-k < m { return false; }

        for i in 0..m {
            if nums[i+k] != group[i] { break; }
            lps[0] += 1;
        }

        if lps[0] == m {
            k += m;
            continue 'outer;
        } else {
            let mut i = 1;
            lps[0] = 0;
            let mut len = 0;
            while i+k < n {
                if nums[i+k] == group[len] {
                    len += 1;
                    lps[i] = len;
                    i += 1;
                } else if len == 0 {
                    i += 1;
                } else {
                    len = lps[len-1];
                }

                if len == m {
                    k += i;
                    continue 'outer;
                }
            }
        }

        return false;
    }

    true
}

pub fn main() {
    let groups = vec![vec![1,-1,-1], vec![3,-2,0]];
    let nums = vec![1,-1,0,1,-1,-1,3,-2,0];
    println!("{}", can_choose(groups, nums));
}
