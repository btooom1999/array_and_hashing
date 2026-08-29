fn clumsy(n: i32) -> i32 {
    let mut stack = Vec::from([n]);
    let operations = ['*', '/', '+', '-'];
    for (i, num) in (1..n).rev().enumerate() {
        let operation = operations[i%4];
        if operation == '*' || operation == '/' {
            let last = stack.pop().unwrap();
            if operation == '*' {
                stack.push(last * num);
            } else {
                stack.push(last / num);
            }
        } else {
            stack.push(num);
        }
    }

    let mut res = 0;
    for i in 0..stack.len() {
        if i > 0 && i % 2 == 0 {
            res -= stack[i];
        } else {
            res += stack[i];
        }
    }

    res
}

pub fn main() {
    let n = 10;
    println!("{}", clumsy(n));
}
