fn string_shift(mut s: String, shifts: Vec<Vec<i32>>) -> String {
    for shift in &shifts {
        let (direction, amount) = (shift[0], shift[1] % s.len() as i32);
        let (left, right) = if direction == 0 {
            s.split_at(amount as usize)
        } else {
            s.split_at(s.len() - amount as usize)
        };

        s = format!("{}{}", right, left);
    }

    s.clone()
}

pub fn main() {
    let s = "abcdefg".to_string();
    let shift = vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]];
    println!("{}", string_shift(s, shift));
}
