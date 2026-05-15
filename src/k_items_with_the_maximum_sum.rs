fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, mut k: i32) -> i32 {
    let mut res = 0;
    for (amount, num) in [(num_ones, 1), (num_zeros, 0), (num_neg_ones, -1)] {
        let amount = amount.min(k);
        res += num * amount;
        k -= amount;
    }

    res
}

pub fn main() {
    let num_ones = 3;
    let num_zeros = 2;
    let num_neg_ones = 0;
    let k = 2;
    println!("{}", k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k));
}
