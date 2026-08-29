fn gcd_of_strings(str1: String, str2: String) -> String {
    fn gcd(
        max: &[u8],
        min: &[u8],
        i: usize,
        j: usize,
    ) -> String {
        if i == max.len() {
            if j == min.len() {
                return String::from_utf8(min.to_vec()).unwrap();
            }
            return gcd(max, &min[j..], 0, 0);
        }
        if j == min.len() {
            return gcd(&max[i..], min, 0, 0);
        }

        if max[i] != min[j] {
            return String::new();
        }

        gcd(max, min, i+1, j+1)
    }

    let (str1, str2) = if str1.len() >= str2.len() { (str1, str2) } else { (str2, str1) };
    gcd(str1.as_bytes(), str2.as_bytes(), 0, 0)
}

pub fn main() {
    let str1 = "NLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGM".to_string();
    let str2 = "NLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGM".to_string();
    println!("{}", gcd_of_strings(str1, str2));
}
