fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }

    let mut i = 1;
    let mut increasing = true;
    let mut set = vec![vec![]; num_rows+1];

    for c in s.chars() {
        set[i].push(c.to_string());

        if increasing {
            i += 1;
        } else {
            i -= 1;
        }

        if i == 1 {
            increasing = true;
        } else if i == num_rows {
            increasing = false;
        }
    }

    set.into_iter().flatten().collect::<Vec<_>>().join("")
}

pub fn main() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    println!("{}", convert(s, num_rows));
}
