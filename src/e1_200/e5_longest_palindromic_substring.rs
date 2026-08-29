fn longest_palindrome(s: String) -> String {
    let mut t = vec![];
    t.push(b'$');
    for &b in s.as_bytes() {
        t.push(b'#');
        t.push(b);
    }
    t.push(b'#');
    t.push(b'%');

    let n = t.len();
    let mut p = vec![0; n];
    let (mut r, mut c) = (0, 0);
    for i in 1..n-1 {
        if r > i {
            let i_mirror = 2*c-i;
            p[i] = p[i_mirror].min(r-i);
        }

        while t[i+1+p[i]] == t[i-1-p[i]] {
            p[i] += 1;
        }

        if i+p[i] > r {
            c = i;
            r = i+p[i];
        }
    }

    let mut res = (0, 0);
    for i in 1..n-1 {
        if p[i] > res.0 {
            res = (p[i], i);
        }
    }

    let start = (res.1 - res.0) / 2;
    s[start..start+res.0].to_string()
}

pub fn main() {
    let s = "babab".to_string();
    println!("{}", longest_palindrome(s));
}
