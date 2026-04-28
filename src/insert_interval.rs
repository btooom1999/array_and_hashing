fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();
    let (x, y) = (new_interval[0], new_interval[1]);

    intervals.push(vec![x,y]);
    intervals.sort();

    for pair in intervals {
        let (a, b) = (pair[0], pair[1]);
        if let Some(last) = res.last_mut() && (last[1] >= a || last[1] >= b) {
            last[1] = last[1].max(b);
            continue;
        }

        if a <= x {
            if b < x {
                res.push(vec![a, b]);
            } else {
                res.push(vec![a, b.max(y)]);
            }
        } else if b >= y {
            if a > y {
                res.push(vec![a, b]);
            } else {
                res.push(vec![a.min(x), b]);
            }
        }
    }

    res
}

pub fn main() {
    let intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]].into_iter().map(Vec::from).collect();
    let new_interval = [4,8].to_vec();
    println!("{:?}", insert(intervals, new_interval));
}
