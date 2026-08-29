fn get_sum(nums: Vec<i32>) -> i64 {
    let mut manacher = vec![];
    manacher.push(-2);
    for num in nums {
        manacher.push(-1);
        manacher.push(num);
    }
    manacher.push(-1);
    manacher.push(-3);

    let n = manacher.len();
    let mut prefix = vec![0; n];
    for i in 1..n-1 {
        prefix[i] = prefix[i-1] + manacher[i].max(0) as i64;
    }

    let mut p = vec![0; n];
    let (mut r, mut c) = (0, 0);
    let mut max = 0;
    for i in 1..n-1 {
        if r > i {
            let i_mirror = 2*c-i;
            p[i] = p[i_mirror].min(r-i);
        }

        while manacher[i-1-p[i]] == manacher[i+1+p[i]] {
            p[i] += 1;
        }

        if i+p[i] > r {
            c = i;
            r = i+p[i];
        }

        if p[i] > 0 {
            max = max.max(prefix[i+p[i]] - prefix[i-1-p[i]]);
        }
    }

    max
}

pub fn main() {
    let nums = [7,1,2,1,7,3,4,3,4].to_vec();
    println!("{}", get_sum(nums));
}
