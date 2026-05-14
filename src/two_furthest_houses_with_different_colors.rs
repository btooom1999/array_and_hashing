fn max_distance(colors: Vec<i32>) -> i32 {
    let mut res = 0;
    let first_color = colors[0];
    let n = colors.len();

    for i in 1..n {
        if colors[i] != first_color {
            res = res.max(n-i-1);
            res = res.max(i);
        }
    }

    res as i32
}

pub fn main() {
    let colors = [1,1,1,6,1,1,1].to_vec();
    println!("{}", max_distance(colors));
}
