fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
    if y >= x {
        return y-x;
    }

    let mut hashset = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::from([(x, 0)]);
    hashset.insert(x);
    while let Some((num, count)) = queue.pop_front() {
        if num == y { return count; }
        if !hashset.contains(&(num+1)) {
            hashset.insert(num+1);
            queue.push_back((num+1, count+1));
        }
        if num % 11 == 0 && !hashset.contains(&(num/11)) {
            hashset.insert(num/11);
            queue.push_back((num/11, count+1));
        }
        if num % 5 == 0 && !hashset.contains(&(num/5)) {
            hashset.insert(num/5);
            queue.push_back((num/5, count+1));
        }
        if num > 1 && !hashset.contains(&(num-1)) {
            hashset.insert(num-1);
            queue.push_back((num-1, count+1));
        }
    }

    unreachable!()
}

pub fn main() {
    let x = 54;
    let y = 2;
    println!("{}", minimum_operations_to_make_equal(x, y));
}
