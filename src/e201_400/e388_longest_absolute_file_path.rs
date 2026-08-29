fn recursion(
    condition: (usize, usize),
    input: &[u8],
) -> i32 {
    if !input.contains(&b'.') {
        return 0;
    }

    if !input.contains(&10) {
        return input.len() as i32;
    }

    let mut current = (0, 0);
    let mut stack = Vec::new();
    let mut i = 0;
    for j in 0..input.len() {
        if input[j] == 10 || input[j] == 9 {
            if input[j] == 10 {
                current.0 += 1;
            } else {
                current.1 += 1;
            }
        } else if input[j].is_ascii_lowercase() || input[j].is_ascii_whitespace() {
            if condition == current {
                stack.push((i, j - current.0 - current.1));
                i = j;
            }

            current = (0, 0);
        }
    }

    let mut max = 0;
    stack.push((i, input.len()));
    while stack.len() > 1 {
        let (i, j) = stack.pop().unwrap();
        max = max.max((stack[0].1 - stack[0].0) as i32 + 1 + recursion((condition.0, condition.1+1), &input[i..j]));
    }

    max
}

fn length_longest_path(input: String) -> i32 {
    let mut max = 0;
    let input = input.as_bytes();
    let n = input.len();
    let mut i = 0;
    for j in 0..input.len() {
        if j+1 < n && input[j] == 10 && input[j+1] != 9 {
            max = max.max(recursion((1,1), &input[i..j]));
            i = j+1;
        }
    }

    max.max(recursion((1,1), &input[i..n]))
}

pub fn main() {
    // let input = "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string();
    // let input = "file1.txt\nfile2.txt\nlongfile.txt".to_string();
    // let input = "a\n\tb.txt\na2\n\tb2.txt".to_string();
    let input = "a\n\tb.txt\n\n\n\n\n\n\n\na2\n\tb2.txt".to_string();
    println!("{}", length_longest_path(input));
}
