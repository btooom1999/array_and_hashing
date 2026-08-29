const MOD: u64 = 1_000_000_007;

fn concatenated_binary(n: i32) -> i32 {
    let mut res = 0;

    for i in 1..=n {
        let amount = 32 - i.leading_zeros();
        res = ((res << amount) % MOD + i as u64) % MOD;
    }

    res as _
}

pub fn main() {
    let n = 12;
    println!("{}", concatenated_binary(n));
}


