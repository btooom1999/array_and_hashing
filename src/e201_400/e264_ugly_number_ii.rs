use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

fn nth_ugly_number(mut n: i32) -> i32 {
    let mut hashset = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(1));

    while let Some(Reverse(num)) = heap.pop() {
        n -= 1;
        if n == 0 {
            return num;
        }

        hashset.remove(&num);

        if let Some(num) = num.checked_mul(2) && !hashset.contains(&num) {
            heap.push(Reverse(num));
            hashset.insert(num);
        }

        if let Some(num) = num.checked_mul(3) && !hashset.contains(&num) {
            heap.push(Reverse(num));
            hashset.insert(num);
        }

        if let Some(num) = num.checked_mul(5) && !hashset.contains(&num) {
            heap.push(Reverse(num));
            hashset.insert(num);
        }
    }

    0
}
