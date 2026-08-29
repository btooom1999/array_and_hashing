fn max_product(s: String) -> i64 {
    let s = format!("${s}%");
    let s = s.as_bytes();
    let n = s.len();
    let mut p = vec![0; n];

    let (mut r, mut c) = (0, 0);
    let mut prefix = vec![1;n+1];
    let mut suffix = vec![1;n+1];
    for i in 1..n-1 {
        if r>i {
            let i_mirror = 2*c-i;
            p[i] = p[i_mirror].min(r-i);
        }

        prefix[i+p[i]] = prefix[i+p[i]].max(p[i]*2+1);
        suffix[i-p[i]] = suffix[i-p[i]].max(p[i]*2+1);
        while s[i+1+p[i]] == s[i-1-p[i]] {
            prefix[i+1+p[i]] = (p[i]+1)*2+1;
            suffix[i-1-p[i]] = (p[i]+1)*2+1;
            p[i] += 1;
        }

        if i+p[i] > r {
            c = i;
            r = i+p[i];
        }
    }

    for i in 1..n-1 {
        prefix[i] = prefix[i].max(prefix[i-1]);
        suffix[n-i] = suffix[n-i].max(suffix[n-i+1]);
    }

    let mut res = 0;
    for i in 1..n-2 {
        res = res.max(prefix[i] as i64 * suffix[i+1] as i64);
    }

    res
}

pub fn main() {
    let s = "aaaaaa".to_string();
    println!("{}", max_product(s));
}
