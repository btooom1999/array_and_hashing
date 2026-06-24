fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
    if start == goal {
        return 0;
    }

    let mut hashset = [false; 1001];
    let mut queue = std::collections::VecDeque::from([(start, 0)]);
    while let Some(x) = queue.pop_front(){
        for &num in &nums {
            for num in [x.0 + num, x.0 - num, x.0 ^ num] {
                if num == goal {
                    return x.1+1;
                }
                if num < 0 || num > 1000 || hashset[num as usize] {
                    continue;
                }
                hashset[num as usize] = true;
                queue.push_back((num, x.1+1));
            }
        }
    }

    -1
}

pub fn main() {
    let nums = [2,4,12].to_vec();
    let start = 2;
    let goal = 12;
    println!("{}", minimum_operations(nums, start, goal));
}
