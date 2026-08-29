fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut data = Vec::new();
    for i in 0..dist.len() {
        data.push((dist[i] as f32 / speed[i] as f32).ceil() as i32);
    }

    data.sort();
    let mut res = 0;
    for (minute, item) in data.into_iter().enumerate() {
        if minute as i32 >= item {
            break;
        }

        res += 1;
    }

    res
}

pub fn main() {
    let dist = [4,3,3,3,4].to_vec();
    let speed = [1,1,1,1,4].to_vec();
    println!("{}", eliminate_maximum(dist, speed));
}
