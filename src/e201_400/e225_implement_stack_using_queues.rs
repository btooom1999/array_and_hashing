use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {

    fn new() -> Self {
        Self { q1: VecDeque::new(), q2: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.q1.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while self.q1.len() != 1 && let Some(val) = self.q1.pop_front() {
            self.q2.push_back(val);
        }

        let last_item = self.q1.pop_front().unwrap_or(-1);

        std::mem::swap(&mut self.q1, &mut self.q2);

        last_item
    }

    fn top(&mut self) -> i32 {
        while self.q1.len() != 1 && let Some(val) = self.q1.pop_front() {
            self.q2.push_back(val);
        }

        let last_item = self.q1.pop_front().unwrap_or(-1);
        self.q2.push_back(last_item);

        std::mem::swap(&mut self.q1, &mut self.q2);

        last_item
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

pub fn main() {
    let mut my_stack = MyStack::new();
    my_stack.push(1);
    my_stack.push(2);
    println!("{}", my_stack.pop());
    my_stack.push(5);
    my_stack.push(6);
    println!("{}", my_stack.top());
}
