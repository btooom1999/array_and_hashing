fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
    tokens.sort();

    let mut power = power;
    let mut score = 0;
    let mut l = 0;
    let mut r = tokens.len() as i32 - 1;

    while l <= r && power >= tokens[l as usize] {
        while l <= r && tokens[l as usize] <= power {
            power -= tokens[l as usize];
            score += 1;
            l += 1;
        }

        if l < r && score > 0 && power + tokens[r as usize] >= tokens[l as usize] {
            power += tokens[r as usize];
            score -= 1;
            r -= 1;
        }
    }

    score
}

pub fn main() {
    let tokens = vec![26];
    let power = 51;
    println!("{}", bag_of_tokens_score(tokens, power));
}
