fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let n = n as usize;
    let mut stack = Vec::new();
    let mut res = vec![0; n];
    for log in &logs {
        let mut data = log.split(":");
        let (id, status, timestamp) = (data.next().unwrap().parse::<usize>().unwrap(), data.next().unwrap(), data.next().unwrap().parse::<i32>().unwrap());
        if stack.last().is_some_and(|&(_, prev_status, _, _)| prev_status != status) {
            let (_, _, prev_timestamp, skip) = stack.pop().unwrap();
            res[id] = timestamp - prev_timestamp - skip + 1;

            if let Some(last) = stack.last_mut() {
                last.3 += timestamp - prev_timestamp + 1;
            }
        } else {
            stack.push((id, status, timestamp, 0));
        }
    }

    res
}

pub fn main() {
    let n = 2;
    let logs = ["0:start:0","1:start:2","1:end:5","0:end:6"].map(String::from).to_vec();
    println!("{:?}", exclusive_time(n, logs));
}
