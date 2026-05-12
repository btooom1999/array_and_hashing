fn maximum_number(mut num: String, change: Vec<i32>) -> String {
    let temp = unsafe { num.as_bytes_mut() };

    let n = temp.len();
    let mut i = 0;

    while i < n && change[(temp[i]-b'0') as usize] <= (temp[i]-b'0') as i32 {
        i += 1;
    }

    while i < n && change[(temp[i]-b'0') as usize] >= (temp[i]-b'0') as i32 {
        temp[i] = change[(temp[i]-b'0') as usize] as u8 + b'0';
        i += 1;
    }

    num
}

pub fn main() {
    let num = "132".to_string();
    let change = [9,8,5,0,3,6,4,2,6,8].to_vec();
    println!("{}", maximum_number(num, change));
}
