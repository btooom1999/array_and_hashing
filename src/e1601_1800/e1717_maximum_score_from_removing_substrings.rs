fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut a = [0;3];
    let mut b = [0;3];
    let mut res = 0;

    for c in s.chars() {
        match c {
            'a' => {
                a[0] += 1;
                if b[0] > 0 {
                    b[2] += y;
                    b[0] -= 1;
                } else {
                    b[1] += 1;
                }
            }
            'b' => {
                b[0] += 1;
                if a[0] > 0 {
                    a[2] += x;
                    a[0] -= 1;
                } else {
                    a[1] += 1;
                }
            }
            _ => {
                a[2] += a[0].min(a[1]) * y;
                b[2] += b[0].min(b[1]) * x;
                res += a[2].max(b[2]);
                a = [0;3];
                b = [0;3];
            }
        }
    }

    res + (a[0].min(a[1]) * y + a[2]).max(b[0].min(b[1]) * x + b[2])
}

pub fn main() {
    let s = "abaabbaaxbbaabb".to_string();
    // let s = "abaabbaa".to_string();
    // let s = "bbaabb".to_string();
    let x = 4;
    let y = 5;
    println!("{}", maximum_gain(s, x, y));
}
