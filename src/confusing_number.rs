use std::collections::HashSet;

fn confusing_number(n: i32) -> bool {
    let hashset = HashSet::<i32>::from([0, 1, 6, 8, 9]);

    let mut rotated_n = 0;
    let mut temp_n = n;
    while temp_n > 0 {
        let val = temp_n % 10;
        if hashset.contains(&val) {
            rotated_n = rotated_n * 10 + val;
        } else {
            return false;
        }

        temp_n /= 10;
    }

    rotated_n != n
}

pub fn main() {
    let n = 609;
    println!("{}", confusing_number(n));
}

#[cfg(test)]
mod tests {
    use super::confusing_number;

    #[test]
    fn test_confusing_number_11() {
        assert!(!confusing_number(11));
    }

    #[test]
    fn test_confusing_number_10() {
        assert!(confusing_number(10));
    }

    #[test]
    fn test_confusing_number_69() {
        assert!(confusing_number(69));
    }

    #[test]
    fn test_confusing_number_818() {
        assert!(!confusing_number(818));
    }

    #[test]
    fn test_confusing_number_2() {
        assert!(!confusing_number(2));
    }

    #[test]
    fn test_confusing_number_609() {
        assert!(confusing_number(609));
    }
}
