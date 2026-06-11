fn bulb_switch(n: i32) -> i32 {
    n.isqrt()
}

pub fn main() {
    let n = 3;
    println!("{}", bulb_switch(n));
}
