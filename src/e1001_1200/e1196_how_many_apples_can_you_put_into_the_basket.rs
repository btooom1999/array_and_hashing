fn max_number_of_apples(mut weight: Vec<i32>) -> i32 {
    weight.sort();

    let mut max = 5000;
    let mut count = 0;
    for w in weight {
        max -= w;
        if max < 0 {
            return count;
        }

        count += 1;
    }

    count
}

pub fn main() {
    // let weight = [900, 950, 800, 1000, 700, 800].to_vec();
    let weight = [100, 200, 150, 1000].to_vec();
    println!("{}", max_number_of_apples(weight))
}
