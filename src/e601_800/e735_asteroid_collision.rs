fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    for ast in asteroids {
        if ast.is_positive() {
            stack.push(ast);
            continue;
        }

        let mut exploded = false;
        while let Some(&val) = stack.last() {
            if val.is_negative() {
                break;
            }

            if val <= -ast {
                stack.pop();
                if val == -ast {
                    exploded = true;
                    break;
                }
            } else {
                exploded = true;
                break;
            }
        }

        if !exploded {
            stack.push(ast);
        }
    }

    stack
}

pub fn main() {
    let asteroids = vec![-2, -1, 1, 2];
    println!("{:?}", asteroid_collision(asteroids));
}
