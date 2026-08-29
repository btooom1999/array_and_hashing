fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();

    let mut l = 0;
    let mut r = people.len() as i32 - 1;
    let mut res = 0;

    while l <= r {
        if people[l as usize] + people[r as usize] <= limit {
            res += 1;
            l += 1;
            r -= 1;
        } else {
            if people[r as usize] <= limit {
                res += 1;
            }
            r -= 1;
        }
    }

    res
}

pub fn main() {
    let people = vec![5,1,4,2];
    let limit = 6;
    println!("{}", num_rescue_boats(people, limit));
}
