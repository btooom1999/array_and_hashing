fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let n = position.len();
    let mut vec = Vec::with_capacity(n);
    let mut fleets = Vec::<f32>::new();
    for i in 0..n {
        vec.push((position[i], speed[i]));
    }

    vec.sort_by(|&a, &b| b.0.cmp(&a.0));
    for (position, speed) in vec {
        let time = (target - position) as f32 / speed as f32;
        if fleets.last().is_none_or(|v| time > *v) {
            fleets.push(time)
        }
    }

    fleets.len() as i32
}

pub fn main() {
    let target = 10;
    let position = vec![8, 3, 7, 4, 6, 5];
    let speed = vec![4, 4, 4, 4, 4, 4];
    println!("{}", car_fleet(target, position, speed));
}
