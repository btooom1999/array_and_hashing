fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut res = Vec::new();
    for token in &tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let val2 = res.pop().unwrap();
                let val1 = res.pop().unwrap();
                if token == "+" {
                    res.push(val1 + val2);
                } else if token == "-" {
                    res.push(val1 - val2);
                } else if token == "*" {
                    res.push(val1 * val2);
                } else {
                    res.push(val1 / val2);
                }
            }
            _ => {
                res.push(token.parse::<i32>().unwrap());
            }
        }
    }

    res[0]
}

pub fn main() {
    let tokens = ["2","1","+","3","*"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", eval_rpn(tokens));
}
