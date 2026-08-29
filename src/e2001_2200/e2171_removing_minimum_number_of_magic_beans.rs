fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort_unstable();
    let n = beans.len();
    let total = beans.iter().map(|&x| x as i64).sum::<i64>();
    let mut keep = 0;
    for i in 0..n {
        keep = keep.max(beans[i] as i64 * (n-i) as i64);
    }

    total - keep
}

pub fn main() {
    let beans = [4,1,6,5].to_vec();
    println!("{}", minimum_removal(beans));
}
