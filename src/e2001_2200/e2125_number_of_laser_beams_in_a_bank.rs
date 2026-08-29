fn number_of_beams(bank: Vec<String>) -> i32 {
    let fold = |v: String| -> i32 {
        v.chars().fold(0, |acc, c| acc + (c == '1') as i32)
    };

    let bank = bank.into_iter().map(fold).collect::<Vec<_>>();
    let mut at = None;
    let mut res = 0;
    for (i, &count) in bank.iter().enumerate() {
        if count > 0 {
            res += count * at.map_or(0, |at| bank[at]);
            at = Some(i);
        }
    }

    res
}

pub fn main() {
    let bank = ["011001","000000","010100","001000"].into_iter().map(String::from).collect();
    println!("{}", number_of_beams(bank));
}
