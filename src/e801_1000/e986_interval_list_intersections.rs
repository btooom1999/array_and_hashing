fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();
    while i < first_list.len() && j < second_list.len() {
        let begin = first_list[i][0].max(second_list[j][0]);
        let end = first_list[i][1].min(second_list[j][1]);

        if begin <= end {
            res.push(vec![begin,end]);
        }

        if first_list[i][1] <= second_list[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    res
}

pub fn main() {
    let first_list = [[0,2],[5,10],[13,23],[24,25]].into_iter().map(Vec::from).collect();
    let second_list = [[1,5],[8,12],[15,24],[25,26]].into_iter().map(Vec::from).collect();
    println!("{:?}", interval_intersection(first_list, second_list));
}
