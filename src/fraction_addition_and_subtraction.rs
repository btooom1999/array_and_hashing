fn dfs(expression: &str) -> (i32, i32) {
    if expression.is_empty() {
        return (0, 0);
    }

    if let Some(i) = expression.find('+') {
        let left = dfs(&expression[..i]);
        let right = dfs(&expression[i+1..]);
        if left.1 == 0 {
            return right;
        }
        if right.1 == 0 {
            return left;
        }

        if left.1 == right.1 {
            return (left.0 + right.0, left.1);
        }

        return (left.0 * right.1 + right.0 * left.1, left.1 * right.1);
    }

    let mut splitted = expression.split("/");
    let num1 = splitted.next().unwrap().parse::<i32>().unwrap();
    let num2 = splitted.next().unwrap().parse::<i32>().unwrap();
    (num1, num2)
}

fn fraction_addition(expression: String) -> String {
    let expression = expression.replace("-", "+-");
    let (a, b) = dfs(&expression);
    let gcd = |mut a: i32, mut b: i32| -> i32 {
        while b != 0 {
            (a, b) = (b, a%b);
        }

        a
    };

    let gcd = gcd(a.abs(), b.abs());
    format!("{}/{}", a/gcd, b/gcd)
}

pub fn main() {
    let expression = "-1/2+1/2+1/3".to_string();
    println!("{}", fraction_addition(expression));
}
