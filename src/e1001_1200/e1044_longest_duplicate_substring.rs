fn longest_dup_substring(s: String) -> String {
    let s = s.as_bytes();
    let n = s.len();
    let mut sa = (0..n).collect::<Vec<_>>();
    let mut rank = s.iter().map(|&v| v as i32).collect::<Vec<_>>();
    let mut temp = vec![0; n];

    let mut k = 1;
    while k < n {
        sa.sort_by(|&i, &j| {
            if rank[i] != rank[j] {
                rank[i].cmp(&rank[j])
            } else {
                let a = if i+k < n { rank[i+k] } else { -1 };
                let b = if j+k < n { rank[j+k] } else { -1 };
                a.cmp(&b)
            }
        });

        for i in 1..n {
            let prev = sa[i-1];
            let curr = sa[i];
            let prev_pair = (rank[prev], if prev+k < n { rank[prev+k] } else { -1 });
            let curr_pair = (rank[curr], if curr+k < n { rank[curr+k] } else { -1 });
            temp[curr] = temp[prev] + (prev_pair != curr_pair) as i32;
        }

        rank.copy_from_slice(&temp);
        k <<= 1;
    }

    let mut res = (0,0);
    for i in 1..n {
        let j = sa[i-1];
        let i = sa[i];
        let mut k = 0;
        while i+k < n && j+k < n && s[i+k] == s[j+k] {
            k += 1;
        }
        if k>res.1 {
            res = (i, k);
        }
    }

    if res.1 == 0 { String::new() } else { String::from_utf8(s[res.0..res.0+res.1].to_vec()).unwrap() }
}

pub fn main() {
    let s = "aaaaa".to_string();
    println!("{}", longest_dup_substring(s));
}
