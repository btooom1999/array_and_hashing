fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
    let num_people = num_people as usize;
    let mut res = vec![0; num_people];
    let mut i = 1;
    while candies > 0 {
        let min = candies.min(i as i32);
        res[(i-1) % num_people] += min;
        candies -= min;
        i += 1;
    }

    res
}

pub fn main() {
    let candies = 1_000_000_000;
    let num_people = 3;
    println!("{:?}", distribute_candies(candies, num_people));
}
