fn dfs(
    bottom: &[u8],
    need: &mut Vec<u8>,
    hashmap: &Vec<Vec<Vec<u8>>>,
) -> bool {
    if bottom.len() == 1 {
        return if !need.is_empty() { dfs(need, &mut vec![], hashmap) } else { true };
    }

    for &b in &hashmap[(bottom[0]-b'A') as usize][(bottom[1]-b'A') as usize] {
        need.push(b);
        if dfs(&bottom[1..], need, hashmap) {
            return true;
        }
        need.pop();
    }

    false
}

fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let mut hashmap = vec![vec![vec![]; 26]; 26];

    for s in allowed {
        let s = s.as_bytes();
        hashmap[(s[0]-b'A') as usize][(s[1]-b'A') as usize].push(s[2]);
    }

    dfs(bottom.as_bytes(), &mut vec![], &hashmap)
}

pub fn main() {
    // let bottom = "BCD".to_string();
    // let allowed = ["BCC","CDE","CEA","FFF"].into_iter().map(String::from).collect();
    let bottom = "AAAA".to_string();
    let allowed = ["AAB","AAC","BCD","BBE","DEF"].into_iter().map(String::from).collect();
    println!("{}", pyramid_transition(bottom, allowed));
}
