fn winning_player(x: i32, y: i32) -> String {
    if (x.min(y/4)) % 2 == 0 { "Bob".to_string() } else { "Alice".to_string() }
}

pub fn main() {
    let x = 4;
    let y = 7;
    println!("{}", winning_player(x, y));
}
