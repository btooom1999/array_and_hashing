fn flip_lights(n: i32, presses: i32) -> i32 {
    if presses == 0 { 1 }
    else if n == 1 { 2 }
    else if n == 2 && presses == 1 { 3 }
    else if n == 2 || presses == 1 { 4 }
    else if presses == 2 { 7 }
    else { 8 }
}

pub fn main() {
    let n = 2;
    let presses = 1;
    println!("{}", flip_lights(n, presses));
}
