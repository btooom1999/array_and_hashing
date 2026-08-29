use std::collections::HashSet;

fn dfs(
    hours: &[i32; 4],
    minutes: &[i32; 6],
    idx: (usize, usize),
    time: (i32, i32),
    turned_on: i32,
    results: &mut HashSet<String>,
) {
    if turned_on == 0 {
        results.insert(format!("{}:{:02}", time.0, time.1));
    }

    for i in idx.0..hours.len() {
        if time.0 + hours[i] > 11 || turned_on == 0 {
            break;
        }

        dfs(hours, minutes, (i+1, idx.1), (time.0+hours[i], time.1), turned_on-1, results);
    }

    for j in idx.1..minutes.len() {
        if time.1 + minutes[j] > 59 || turned_on == 0 {
            break;
        }

        dfs(hours, minutes, (idx.0, j+1), (time.0, time.1+minutes[j]), turned_on-1, results);
    }
}

fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut res = HashSet::new();
    dfs(&[1,2,4,8], &[1,2,4,8,16,32], (0, 0), (0, 0), turned_on, &mut res);
    res.into_iter().collect::<Vec<_>>()
}

pub fn main() {
    let turned_on = 3;
    println!("{:?}", read_binary_watch(turned_on));
}
