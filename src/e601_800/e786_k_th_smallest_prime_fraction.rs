fn helper(arr: &[i32], target: f64) -> (i32, i32, i32) {
    let mut count = 0;
    let (mut num1, mut num2) = (-1, -1);
    for i in 0..arr.len() {
        let mut j = arr.len()-1;
        while i < j {
            let res = arr[i] as f64 / arr[j] as f64;
            if res <= target {
                if res == target {
                    num1 = arr[i];
                    num2 = arr[j];
                }
                count += 1;
                j -= 1;
            } else {
                break;
            }
        }
    }

    (count, num1, num2)
}

fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut l = 0f64;
    let mut r = 1f64;

    while l < r {
        let m = (l+r)/2f64;

        let res = helper(&arr, m);
        if res.0 >= k {
            if res.0 == k && res.1 != -1 {
                return vec![res.1, res.2];
            }
            r = m;
        } else {
            l = m;
        }
    }

    unreachable!()
}

pub fn main() {
    let arr = [1,2,3,5].to_vec();
    let k = 3;
    println!("{:?}", kth_smallest_prime_fraction(arr, k));
}
