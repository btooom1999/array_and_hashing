fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        let val = numbers[l] + numbers[r];
        if val == target {
            return vec![l as i32 + 1, r as i32 + 1];
        } else if val > target {
            r -= 1;
        } else {
            l += 1;
        }
    }

    vec![-1, -1]
}

pub fn main() {
    let numbers = vec![2,7,11,15];
    let target = 9;
    println!("{:?}", two_sum(numbers, target));
}
