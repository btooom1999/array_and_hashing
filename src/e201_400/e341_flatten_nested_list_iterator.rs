use std::collections::VecDeque;

enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    stack: VecDeque<NestedInteger>,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self { stack: nested_list.into() }
    }

    fn next(&mut self) -> i32 {
        if Self::has_next(self) &&let NestedInteger::Int(num) = self.stack.pop_front().unwrap() {
            return num;
        }

        unreachable!()
    }

    fn has_next(&mut self) -> bool {
        while let Some(front) = self.stack.front() {
            match front {
                NestedInteger::Int(_) => { return true; },
                NestedInteger::List(_) => {
                    if let NestedInteger::List(inner) = self.stack.pop_front().unwrap() {
                        for item in inner.into_iter().rev() {
                            self.stack.push_front(item);
                        }
                    }
                },
            };
        }

        false
    }
}

pub fn main() {

}
