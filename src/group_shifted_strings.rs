use std::collections::HashMap;

fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    if strings.is_empty() {
        return vec![];
    };

    let mut res: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

    for str in &strings {
        if str.len() == 1 {
            res.entry(vec![-1]).or_default().push(str.to_owned());
            continue;
        }

        let nums = str.as_bytes();
        let mut distances = vec![];
        for i in 0..(nums.len() - 1) {
            let mut d = (nums[i + 1] as i32 - nums[i] as i32);
            if d < 0 {
                println!("check {:?}", d);
                d = (nums[i + 1] - b'a' + 1) as i32 + (b'z' - nums[i]) as i32;
            }

            distances.push(d);
        }

        res.entry(distances).or_default().push(str.to_owned());
    }

    for (k, v) in res.iter() {
        println!("{:?} {:?}", k, v);
    }

    res.into_values().collect::<Vec<Vec<_>>>()
}

pub fn main() {
    let strings = vec!["az".to_string(), "yx".to_string()];
    // let strings = vec!["aa".to_string(), "bb".to_string(), "b".to_string()];
    // let strings = vec!["lpt".to_string(), "txb".to_string()];

    println!("{:?}", group_strings(strings));
}
