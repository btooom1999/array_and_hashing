use std::collections::HashMap;

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut hashmap = HashMap::<String, String>::with_capacity(paths.len());
    for path in &paths {
        hashmap.insert(path[0].clone(), path[1].clone());
    }

    let mut end_path = paths[0][1].clone();
    while let Some(val) = hashmap.get(&end_path) {
        end_path = val.to_string();
    }

    end_path
}

pub fn main() {
    let paths = vec![
        vec!["London".to_string(), "New York".to_string()],
        vec!["New York".to_string(), "Lima".to_string()],
        vec!["Lima".to_string(), "Sao Paulo".to_string()],
    ];
    println!("{}", dest_city(paths));
}
