fn is_palindrome(x: i32) -> bool {
    let mut reversed_num = 0;
    let mut temp_x = x;
    while temp_x > 0 {
        let exceed = temp_x % 10;
        reversed_num = reversed_num * 10 + exceed;
        temp_x /= 10;
    }

    reversed_num == x
}

pub fn main() {

}
