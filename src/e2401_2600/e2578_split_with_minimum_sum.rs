fn split_num(mut num: i32) -> i32 {
    let mut heap = std::collections::BinaryHeap::new();
    while num > 0 {
        if num % 10 > 0 {
            heap.push(num % 10);
        }
        num /= 10;
    }

    let mut pow = 1;
    let mut res = 0;
    while !heap.is_empty() {
        let a = heap.pop().unwrap_or(0);
        let b = heap.pop().unwrap_or(0);
        res += pow * (a+b);
        pow *= 10;
    }

    res
}

pub fn main() {
    let num = 4325;
    println!("{}", split_num(num));
}
