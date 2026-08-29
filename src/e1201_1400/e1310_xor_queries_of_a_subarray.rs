fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix = vec![0; arr.len()+1];
    for i in 0..arr.len() {
        prefix[i+1] = arr[i] ^ prefix[i];
    }

    let mut res = vec![0; queries.len()];
    for (i, query) in queries.iter().enumerate() {
        res[i] = prefix[(query[1]+1) as usize] ^ prefix[query[0] as usize];
    }

    res
}

pub fn main() {
    let arr = [1,3,4,8].to_vec();
    let queries = [[0,1],[1,2],[0,3],[3,3]].into_iter().map(Vec::from).collect();
    println!("{:?}", xor_queries(arr, queries));
}
