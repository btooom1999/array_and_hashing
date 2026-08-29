use std::{cmp::Reverse, collections::BinaryHeap};

struct SeatManager {
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        let min_heap = (1..=n).map(Reverse).collect::<BinaryHeap<_>>();

        Self { min_heap }
    }

    fn reserve(&mut self) -> i32 {
        self.min_heap.pop().map_or(-1, |val| val.0)
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.min_heap.push(Reverse(seat_number));
    }
}

pub fn main() {

}
