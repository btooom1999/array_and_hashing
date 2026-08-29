fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 1..mountain.len()-1 {
        if mountain[i] > mountain[i-1] && mountain[i] > mountain[i+1] {
            res.push(i as i32);
        }
    }

    res
}

pub fn main() {
    let mountain = [1,4,3,8,5].to_vec();
    println!("{:?}", find_peaks(mountain));
}
