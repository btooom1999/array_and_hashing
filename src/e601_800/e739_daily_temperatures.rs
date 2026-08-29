fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack = Vec::<usize>::new();
    for (i, temp) in temperatures.iter().enumerate().rev() {
        while let Some(last) = stack.last() {
            if *temp >= temperatures[*last] {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(last) = stack.last() {
            res[i] = (*last - i) as i32;
        }
        stack.push(i);
    }

    res
}

pub fn main() {
    let temperatures = vec![73,74,75,71,69,72,76,73];
    println!("{:?}", daily_temperatures(temperatures));
}
