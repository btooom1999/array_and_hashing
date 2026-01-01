fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();

    let mut count1 = 0;
    let mut count2 = 0;
    let mut candidate1 = -1;
    let mut candidate2 = -1;

    for num in &nums {
        if candidate1 == *num {
            count1 += 1;
        } else if candidate2 == *num {
            count2 += 1;
        } else if count1 == 0 {
            count1 = 1;
            candidate1 = *num;
        } else if count2 == 0 {
            count2 = 1;
            candidate2 = *num;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
    }

    println!("{} {}", candidate1, candidate2);

    let mut count1 = 0;
    let mut count2 = 0;

    for num in &nums {
        if candidate1 == *num {
            count1 += 1;
        } else if candidate2 == *num {
            count2 += 1;
        }
    }

    if count1 > nums.len() / 3 {
        res.push(candidate1);
    }

    if count2 > nums.len() / 3 {
        res.push(candidate2);
    }

    res
}

pub fn main() {
    let nums = vec![2, 1, 1, 3, 1, 4, 5, 6];
    println!("{:?}", majority_element(nums));
}
