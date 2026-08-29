fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut res = vec![pref[0]];

    let mut curr = res[0];
    for num in pref.into_iter().skip(1) {
        let val = curr ^ num;
        res.push(val);
        curr ^= val;
    }

    res
}

pub fn main() {
    let pref = [5,2,0,3,1].to_vec();
    println!("{:?}", find_array(pref));
}
