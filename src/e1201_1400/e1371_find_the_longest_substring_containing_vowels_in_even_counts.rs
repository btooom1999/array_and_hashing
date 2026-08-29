fn find_the_longest_substring(s: String) -> i32 {
    let mut bits = [0; 32];
    bits[0] = 1;
    bits[(b'e'-b'a') as usize] = 2;
    bits[(b'i'-b'a') as usize] = 4;
    bits[(b'u'-b'a') as usize] = 8;
    bits[(b'o'-b'a') as usize] = 16;

    fn find_excess(mask: i32) -> usize {
        let mut max = 0;
        for i in [1,2,4,8,16] {
            if mask & i == i {
                max |= i;
            }
        }

        max as usize
    }

    let s = s.into_bytes();
    let mut res = 0;
    let mut mask = 0;
    let mut hashmap = [-1; 32];
    hashmap[0] = 0;

    for (j, &b) in s.iter().enumerate() {
        let j = j as i32;
        mask ^= bits[(b-b'a') as usize];

        let excess = find_excess(mask);
        if hashmap[excess] > -1 {
            res = res.max(j+1-hashmap[excess]);
        } else {
            hashmap[excess] = j+1;
        }
    }

    res
}

pub fn main() {
    let s = "eleetminicoworoep".to_string();
    println!("{}", find_the_longest_substring(s));
}
