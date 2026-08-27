fn dfs(
    map: &Vec<Vec<usize>>,
    parent: usize,
    s: &[u8],
    dfs_str: &mut Vec<u8>,
    skip: usize,
    result: &mut Vec<(usize, usize)>,
) -> usize {
    let mut len = 0;
    let mut new_skip = skip;
    if !map[parent].is_empty() {
        for &child in &map[parent] {
            let l = dfs(map, child, s, dfs_str, new_skip, result);
            new_skip += l;
            len += l;
        }
    }

    result[parent] = (skip*2+1, (skip+len+1)*2+1);
    dfs_str.push(b'#');
    dfs_str.push(s[parent]);
    len+1
}

fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
    let s = s.as_bytes();
    let n = s.len();
    let mut map = vec![vec![]; s.len()];
    for (child, &parent) in parent.iter().enumerate().skip(1) {
        let parent = parent as usize;
        map[parent].push(child);
    }

    let mut dfs_str = Vec::new();
    dfs_str.push(b'$');
    let mut res = vec![(0, 0); n];
    dfs(&map, 0, s, &mut dfs_str, 0, &mut res);
    dfs_str.push(b'#');
    dfs_str.push(b'%');

    let m = dfs_str.len();
    let mut p = vec![0; m];
    let (mut c, mut r) = (0, 0);
    for i in 1..m-1 {
        if r > i {
            let i_mirror = 2*c-i;
            p[i] = p[i_mirror].min(r-i);
        }

        while dfs_str[i-1-p[i]] == dfs_str[i+1+p[i]] {
            p[i] += 1;
        }

        if i+p[i] > r {
            c = i;
            r = i+p[i];
        }
    }

    res.into_iter().map(|(l, r)| p[l+(r-l)/2]*2+1 > r-l).collect()
}

pub fn main() {
    let parent = [-1,0,0,1,1,2].to_vec();
    let s = "aababa".to_string();
    // let parent = [-1,0,0,0,0].to_vec();
    // let s = "aabcb".to_string();
    println!("{:?}", find_answer(parent, s));
}
