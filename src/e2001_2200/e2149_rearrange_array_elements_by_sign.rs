fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut n = 0;
    let mut i = 0;
    let mut j = 0;
    let mut need = '+';

    while n < nums.len() {
        match need {
            '+' => {
                while nums[i] < 0 {
                    i += 1;
                }
                res.push(nums[i]);
                need = '-';
                n += 1;
                i += 1;
            }
            '-' => {
                while nums[j] > 0 {
                    j += 1;
                }
                res.push(nums[j]);
                need = '+';
                n += 1;
                j += 1;
            }
            _ => unreachable!()
        }
    }

    res
}

pub fn main() {
    let nums = vec![3,1,-2,-5,2,-4];
    println!("{:?}", rearrange_array(nums));
}
