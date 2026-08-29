fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort();

    let mut low = 1;
    let mut high = *position.last().unwrap();

    while low <= high {
        let mid = (high + low) / 2;
        let mut balls = 0;
        let mut basket = position[0];
        for &num in &position {
            if basket <= num {
                balls += 1;
                basket = num + mid;
            }

            if balls == m {
                break;
            }
        }

        if balls == m {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    high
}

pub fn main() {
    // let position = [5,4,3,2,1,1000000000].to_vec();
    // let m = 2;
    let position = [1,2,3,4,7,10].to_vec();
    let m = 3;

    println!("{}", max_distance(position, m));
}
