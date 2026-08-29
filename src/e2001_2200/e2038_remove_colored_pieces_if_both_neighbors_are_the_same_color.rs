fn winner_of_game(colors: String) -> bool {
    let (mut alice, mut bob) = (0,0);
    let (mut a, mut b) = (0, 0);
    let colors = colors.as_bytes();
    let n = colors.len();
    for i in 1..n-1 {
        if colors[i] == b'A' && colors[i-1] == b'A' && colors[i+1] == b'A' {
            a += 1;
            bob += b;
            b = 0;
        } else if colors[i] == b'B' && colors[i-1] == b'B' && colors[i+1] == b'B' {
            b += 1;
            alice += a;
            a = 0;
        }
    }

    alice += a;
    bob += b;

    alice > bob
}

pub fn main() {
    let colors = "AAABABB".to_string();
    println!("{}", winner_of_game(colors));
}
