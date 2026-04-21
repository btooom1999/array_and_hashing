fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut fives = 0;
    let mut tens = 0;

    for bill in bills {
        if bill == 5 {
            fives += 1;
        } else if bill == 10 {
            if fives == 0 { return false; }

            fives -= 1;
            tens += 1;
        } else if fives == 0 {
            return false;
        } else if tens == 0 {
            if fives < 3 { return false; }

            fives -= 3;
        } else {
            tens -= 1;
            fives -= 1;
        }
    }

    true
}

pub fn main() {
    let bills = [5,5,5,10,20].to_vec();
    println!("{}", lemonade_change(bills));
}
