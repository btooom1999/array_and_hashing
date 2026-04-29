fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort();

    let mut current = points[0].clone();
    let mut res = 1;
    for i in 1..points.len() {
        if points[i][0] > current[1] {
            current = points[i].clone();
            res += 1;
        } else {
            current[0] = current[0].max(points[i][0]);
            current[1] = current[1].min(points[i][1]);
        }
    }

    res
}

pub fn main() {
    let points = [[10,16],[2,8],[1,6],[7,12]].into_iter().map(Vec::from).collect();
    println!("{}", find_min_arrow_shots(points));
}
