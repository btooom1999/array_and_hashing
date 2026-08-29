fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    let mut max = 0;
    let mut res = Vec::new();
    for i in (0..heights.len()).rev() {
        if heights[i] > max {
            res.push(i as i32);
        }
        max = max.max(heights[i]);
    }

    res.reverse();
    res
}

pub fn main() {
    let heights = [4,2,3,2,1].to_vec();
    println!("{:?}", find_buildings(heights));
}
