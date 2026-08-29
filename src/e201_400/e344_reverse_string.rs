fn reverse_string(s: &mut [char]) {
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        (s[l], s[r]) = (s[r], s[l]);

        l += 1;
        r -= 1;
    }
}

pub fn main() {
    let mut s = ['h','e','l','l','o'];
    reverse_string(&mut s);
    println!("{:?}", s);
}
