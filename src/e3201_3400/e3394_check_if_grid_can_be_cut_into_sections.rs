fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    rectangles.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut res = 0;
    let mut max = rectangles[0][2];
    for i in 1..n {
        if max <= rectangles[i][0] {
            max = rectangles[i][2];
            res += 1;
        } else {
            max = max.max(rectangles[i][2]);
        }

        if res >= 2 {
            return true;
        }
    }

    rectangles.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut res = 0;
    let mut max = rectangles[0][3];
    for i in 1..n {
        if max <= rectangles[i][1] {
            max = rectangles[i][3];
            res += 1;
        } else {
            max = max.max(rectangles[i][3]);
        }

        if res >= 2 {
            return true;
        }
    }

    false
}

pub fn main() {
    let n = 6;
    let rectangles = [[0,0,3,3],[3,0,4,3],[4,0,6,3],[0,3,3,6],[3,3,4,6],[4,3,6,6]].into_iter().map(Vec::from).collect();
    println!("{:?}", check_valid_cuts(n, rectangles));
}
