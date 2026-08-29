fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut l = 1;
    let mut r = *quantities.iter().max().unwrap();

    while l <= r {
        let m = (l + r) / 2;
        let mut n = n;
        for &quantity in &quantities {
            n -= (quantity + m - 1) / m;
            if n < 0 {
                break;
            }
        }

        if n < 0 {
            l = m + 1;
        } else {
            r = m - 1 ;
        }
    }

    l
}

pub fn main() {
    let n = 6;
    let quantities = [11,6].to_vec();
    println!("{}", minimized_maximum(n, quantities));
}
