fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min = (i32::MAX, -1);
    let mut prev_min = (i32::MAX, -1);
    let mut max = (i32::MIN, -1);
    let mut prev_max = (i32::MIN, -1);

    for (i, array) in arrays.iter().enumerate() {
        if let (Some(first), Some(last)) = (array.first(), array.last()) {
            if max.0 <= *last {
                prev_max = max;
                max = (*last, i as i32);
            } else if prev_max.0 <= *last {
                prev_max = (*last, i as i32);
            }

            if min.0 >= *first {
                prev_min = min;
                min = (*first, i as i32);
            } else if prev_min.0 >= *first {
                prev_min = (*first, i as i32);
            }
        }
    }

    if max.1 != min.1 {
        max.0 - min.0
    } else {
        std::cmp::max(max.0 - prev_min.0, prev_max.0 - min.0)
    }
}

pub fn main() {
    let arrays = vec![vec![1, 5], vec![3, 4]];
    println!("{}", max_distance(arrays));
}
