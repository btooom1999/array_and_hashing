use std::collections::HashMap;

fn find_lonely_pixels(picture: Vec<Vec<char>>) -> i32 {
    let mut blacks = Vec::new();
    let mut hashmap = HashMap::<String, i32>::new();
    for (i, vec) in picture.iter().enumerate() {
        for (j, p) in vec.iter().enumerate() {
            if *p == 'B' {
                blacks.push((i, j));
                *hashmap.entry(format!("r{}", j)).or_default() += 1;
                *hashmap.entry(format!("c{}", i)).or_default() += 1;
            }
        }
    }

    blacks.into_iter().fold(0, |count, (i, j)| {
        if let (Some(val1), Some(val2)) = (
            hashmap.get(&format!("r{}", j)),
            hashmap.get(&format!("c{}", i)),
        ) && *val1 == 1
            && *val2 == 1
        {
            count + 1
        } else {
            count
        }
    })
}

pub fn main() {
    let picture: Vec<Vec<char>> = vec![
        vec!['W', 'W', 'B'],
        vec!['W', 'B', 'W'],
        vec!['B', 'W', 'W'],
    ];

    println!("{}", find_lonely_pixels(picture))
}
