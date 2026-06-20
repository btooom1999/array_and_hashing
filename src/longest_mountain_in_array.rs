fn longest_mountain(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n < 3 {
        return 0;
    }

    let mut up = 0;
    let mut down = 0;
    let mut res = 0;

    for i in 1..n {
        if arr[i] > arr[i-1] {
            if down > 0 { up = 0; }
            down = 0;
            up += 1;
        } else if arr[i] < arr[i-1] {
            down += 1;
            if up > 0 { res = res.max(up+down+1); }
        } else {
            up = 0;
            down = 0;
        }
    }

    res
}

pub fn main() {
    // let arr = [2,1,4,7,3,2,5].to_vec();
    let arr = [0,2,0,2,1,2,3,4,4,1].to_vec();
    println!("{}", longest_mountain(arr));
}
