fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = points.len();
    if n < 3 {
        return n as i32;
    }

    for i in 0..n-2 {
        let (x0, y0) = (points[i][0], points[i][1]);
        for j in i+1..n-1 {
            let (x1, y1) = (points[j][0], points[j][1]);
            let mut count = 0;
            for k in j+1..n {
                let (x2, y2) = (points[k][0], points[k][1]);
                if (x0*y1+x1*y2+x2*y0) - (y0*x1+y1*x2+y2*x0) == 0 {
                    count += 1;
                }
            }

            res = res.max(count+2);
        }
    }

    res
}

pub fn main() {
    let points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]].into_iter().map(Vec::from).collect();
    // let points = [[1,1],[2,3]].into_iter().map(Vec::from).collect();
    println!("{}", max_points(points));
}
