fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort();

    let total = potions.len() as i32;
    let mut res = Vec::new();

    for spell in &spells {
        let mut l = 0;
        let mut r = total - 1;
        while l <= r {
            let m = (l + r) / 2;
            if *spell as i64 * potions[m as usize] as i64 >= success {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        res.push(total - l);
    }

    res
}

pub fn main() {
    let spells = [1,2,3,4,5,6,7].to_vec();
    let potions = [1,2,3,4,5,6,7].to_vec();
    let success = 25;
    println!("{:?}", successful_pairs(spells, potions, success));
}
