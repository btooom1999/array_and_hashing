fn compress(chars: &mut [char]) -> i32 {
    let mut l = 0;
    let mut duplicate_idx = None as Option<usize>;
    let mut total = 0;
    while l < chars.len() {
        let next_val = chars[l];
        let mut r = l;
        let mut count = 0;
        let old_duplicate_idx = duplicate_idx.unwrap_or(l);
        while r < chars.len() && next_val == chars[r] {
            count += 1;
            if let Some(idx) = duplicate_idx {
                (chars[idx], chars[r]) = (chars[r], chars[idx]);
                duplicate_idx = Some(idx + 1);
            }
            r += 1;
        }

        total += 1;
        if count > 1 {
            let count = count.to_string();
            for (i, c) in count.chars().enumerate() {
                chars[old_duplicate_idx + i + 1] = c;
                total += 1;
            }
            if let Some(next_val) = chars.get(old_duplicate_idx + count.len() + 1) && *next_val == chars[old_duplicate_idx] {
                duplicate_idx = Some(old_duplicate_idx + count.len() + 1);
            }
        }

        l = r;

    }

    println!("{:?}", chars);

    total
}

pub fn main() {
    // let mut chars = ["a","a","b","b"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>();
    // let mut chars = ["a"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>();
    let mut chars = ["b","b","b","a","a","a","a","b","b","b","c","c","d","d"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>();

    println!("{}", compress(&mut chars));
}
