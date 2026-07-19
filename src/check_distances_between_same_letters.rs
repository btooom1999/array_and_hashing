fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut indexes = [None; 26];
    let mut res = [0; 26];
    for (j, c) in s.chars().enumerate() {
        let k = (c as u8 - b'a') as usize;
        if let Some(i) = indexes[k] {
            res[k] = (j-i) as i32;
        }
        indexes[k] = Some(j+1);
    }

    (0..26).all(|i| res[i] == distance[i] || (res[i] == 0 && indexes[i].is_none()))
}

pub fn main() {
    let s = "abaccb".to_string();
    let distance = [1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
    println!("{}", check_distances(s, distance));
}
