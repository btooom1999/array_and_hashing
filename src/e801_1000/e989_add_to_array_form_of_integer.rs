fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let mut excess = 0;
    while !num.is_empty() || k > 0 || excess == 1 {
        let a = num.pop().unwrap_or(0);
        let b = k % 10;

        res.push((a+b+excess)%10);

        excess = (a+b+excess)/10;
        k /= 10;
    }

    res.reverse();
    res
}

pub fn main() {
    let num = [1,2,0,0].to_vec();
    let k = 34;
    println!("{:?}", add_to_array_form(num, k));
}
