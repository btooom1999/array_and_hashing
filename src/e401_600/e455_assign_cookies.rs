fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();

    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < g.len() && j < s.len() {
        if g[i] <= s[j] {
            i += 1;
            res += 1;
        }
        j += 1;
    }

    res
}

pub fn main() {
    let g = vec![1,3,2];
    let s = vec![1,1];
    println!("{}", find_content_children(g, s));
}
