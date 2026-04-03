fn smallest_value(mut n: i32) -> i32 {
    while n != 1 {
        let old_n = n;

        let mut temp = 0;
        while n > 1 {
            let mut p = 2;
            while p*p <= n && n % p != 0 {
                p += 1;
            }

            if n % p == 0 {
                temp += p;
                n /= p;
            } else {
                temp += n;
                n = 1;
            }
        }

        if temp == old_n {
            return old_n;
        }

        n = temp;
    }

    n
}

pub fn main() {
    let n = 15;
    println!("{}", smallest_value(n));
}
