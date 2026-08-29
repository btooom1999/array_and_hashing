fn cal_points(operations: Vec<String>) -> i32 {
    let mut res = Vec::new();
    for op in operations {
        if let Ok(num) = op.parse::<i32>() {
            res.push(num);
            continue;
        }

        match op.as_str() {
            "C" => {
                res.pop();
            }
            "D" => {
                let n = res.len();
                let val = res[n-1];
                res.push(val * 2);
            }
            "+" => {
                let n = res.len();
                let val2 = res[n-1];
                let val1 = res[n-2];
                res.push(val1 + val2);
            }
            _ => unreachable!()
        }
    }

    res.iter().sum::<i32>()
}

pub fn main() {
    let ops = ["5","2","C","D","+"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", cal_points(ops));
}
