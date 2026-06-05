fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by_key(|p| p[0]);

    let mut max = 0;
    for i in 1..points.len() {
        max = max.max(points[i][0]-points[i-1][0]);
    }
    max
}

pub fn main() {
    let points = [[8,7],[9,9],[7,4],[9,7]].into_iter().map(Vec::from).collect();
    println!("{}", max_width_of_vertical_area(points));
}
