fn max_candies(mut status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
    let n = candies.len();
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    for b in &initial_boxes {
        let b = *b as usize;
        if status[b] == 1 {
            queue.push_back(b);
        }
        visited[b] = true;
    }

    let mut res = 0;
    while let Some(b) = queue.pop_front() {
        res += candies[b];
        for k in &keys[b] {
            let k = *k as usize;
            if status[k] == 0 && visited[k] {
                queue.push_back(k);
            }
            status[k] = 1;

        }
        for b in &contained_boxes[b] {
            let b = *b as usize;
            if status[b] == 1 {
                queue.push_back(b);
            }
            visited[b] = true;
        }
    }

    res
}

pub fn main() {
    let status = [1,0,1,0].to_vec();
    let candies = [7,5,4,100].to_vec();
    let keys = vec![vec![],vec![],vec![1],vec![]];
    let contained_boxes = vec![vec![1,2],vec![3],vec![],vec![]];
    let initial_boxes = [0].to_vec();
    println!("{}", max_candies(status, candies, keys, contained_boxes, initial_boxes));
}
