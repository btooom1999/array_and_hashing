fn primes(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n+1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=n.isqrt() {
        if primes[i] {
            for i in (i*i..=n).step_by(i) {
                primes[i] = false;
            }
        }
    }

    primes
}

fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = Vec::new();
    let primes = primes(n);
    for i in 2..=n/2 {
        if primes[i] && primes[n-i] {
            res.push(vec![i as i32, (n-i) as i32]);
        }
    }

    res
}

pub fn main() {
    let n = 6;
    println!("{:?}", find_prime_pairs(n));
}
