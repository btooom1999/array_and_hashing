fn minimize_xor(num1: i32, mut num2: i32) -> i32 {
    let mut set = 0;
    while num2 > 0 {
        if 1 & num2 == 1 {
            set += 1;
        }

        num2 >>= 1;
    }

    let mut minimal = [0; 32];
    for i in (0..32).rev() {
        if set > 0 && (num1 >> i) & 1 == 1 {
            minimal[i as usize] = 1;
            set -= 1;
        }
    }

    for i in 0..32 {
        if set == 0 {
            break;
        }

        if minimal[i] == 0 {
            minimal[i] = 1;
            set -= 1;
        }
    }

    minimal.reverse();
    minimal.into_iter().fold(0, |acc, bit| (acc << 1) | bit)
}

pub fn main() {
    let num1 = 65;
    let num2 = 84;
    println!("{}", minimize_xor(num1, num2));
}
