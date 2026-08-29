fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort();

    let mut res = vec![vec![]; 33];
    for num in arr {
        res[num.count_ones() as usize].push(num);
    }

    res.into_iter().flatten().collect()
}

pub fn main() {
    let arr = [0,1,2,3,4,5,6,7,8].to_vec();
    println!("{:?}", sort_by_bits(arr));
}
