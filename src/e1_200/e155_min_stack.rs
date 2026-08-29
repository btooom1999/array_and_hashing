#[derive(Default)]
struct MinStack {
    data: Vec<i32>,
    min_data: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);

        if self.min_data.is_empty() || val <= *self.min_data.last().unwrap() {
            self.min_data.push(val);
        } else {
            self.min_data.push(*self.min_data.last().unwrap());
        }
    }

    fn pop(&mut self) {
        self.data.pop();
        self.min_data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap_or(&-1)
    }

    fn get_min(&self) -> i32 {
        *self.min_data.last().unwrap_or(&-1)
    }
}

pub fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    println!("{}", min_stack.get_min());
    min_stack.pop();
    min_stack.pop();
    println!("{}", min_stack.get_min());
}
