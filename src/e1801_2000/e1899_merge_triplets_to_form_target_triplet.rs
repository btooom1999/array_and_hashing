fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut res = vec![0, 0, 0];
    for triplet in triplets {
        let (x, y, z) = (triplet[0] ,triplet[1], triplet[2]);
        if x > target[0] || y > target[1] || z > target[2] {
            continue;
        }

        res[0] = res[0].max(x);
        res[1] = res[1].max(y);
        res[2] = res[2].max(z);
    }

    res == target
}

pub fn main() {
    let triplets = [[2,5,3],[1,8,4],[1,7,5]].into_iter().map(Vec::from).collect();
    let target = [2,7,5].to_vec();
    println!("{}", merge_triplets(triplets, target));
}
