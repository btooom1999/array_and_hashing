fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut res1 = Vec::new();
    let mut res2 = Vec::new();
    let mut res3 = Vec::new();

    for num in nums {
        if num < pivot {
            res1.push(num);
        } else if num == pivot {
            res2.push(num);
        } else {
            res3.push(num);
        }
    }

    [res1, res2, res3].concat()
}

pub fn main() {
    let nums = vec![9,12,5,10,14,3,10];
    let pivot = 10;
    println!("{:?}", pivot_array(nums, pivot));
}
