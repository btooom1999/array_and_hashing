fn get_row(row_index: i32) -> Vec<i32> {
    let mut res = vec![1; (row_index + 1) as usize];
    for i in 1..=row_index as usize {
        for j in (1..i).rev() {
            res[j] += res[j - 1];
        }
    }

    res
}

pub fn main() {
    let row_index = 4;
    println!("{:?}", get_row(row_index));
}
