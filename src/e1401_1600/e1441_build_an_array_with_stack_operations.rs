fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut stack = Vec::new();
    let mut res = Vec::new();
    let mut i = 0;
    for num in 1..=n {
        if stack == target {
            return res;
        }

        res.push("Push".to_string());
        if num < target[i] {
            res.push("Pop".to_string());
        } else {
            stack.push(num);
            i += 1;
        }

    }

    res
}

pub fn main() {
    let target = [1,3].to_vec();
    let n = 3;
    println!("{:?}", build_array(target, n));
}
