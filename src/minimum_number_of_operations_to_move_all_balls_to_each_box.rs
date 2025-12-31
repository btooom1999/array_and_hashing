fn min_operations(boxes: String) -> Vec<i32> {
    let mut res = vec![0; boxes.len()];
    for i in 0..boxes.len() {
        for (j, c) in boxes.chars().enumerate() {
            if c != '0' {
                res[i] += (j as i32 - i as i32).abs();
            }
        }
    }

    res
}

pub fn main() {
    let boxes = "110".to_string();
    println!("{:?}", min_operations(boxes));
}
