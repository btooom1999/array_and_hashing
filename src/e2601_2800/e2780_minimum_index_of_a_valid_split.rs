#[inline]
fn find_majority_element(vec: &Vec<i32>) -> (i32, i32) {
    let mut count = 0;
    let mut candidate = -1;
    for num in vec {
        if count == 0 {
            candidate = *num;
        }

        if candidate == *num {
            count += 1;
        } else {
            count -= 1;
        }
    }

    count = 0;
    for num in vec {
        if candidate == *num {
            count += 1;
        }
    }

    (count, candidate)
}

fn minimum_index(nums: Vec<i32>) -> i32 {
    let (max_count, candidate) = find_majority_element(&nums);
    let mut count = 0;
    for (i, num) in nums.iter().enumerate() {
        if candidate == *num {
            count += 1;
            if count * 2 > i as i32 + 1 && (max_count - count) * 2 > (nums.len() - 1 - i) as i32 {
                return i as i32;
            }
        }
    }

    -1
}

pub fn main() {
    // let nums = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
    let nums = vec![1, 2, 2, 2];
    println!("{}", minimum_index(nums));
}
