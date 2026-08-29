struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut cur_span = 1;
        while let Some(&(value, span)) = self.stack.last() {
            if value <= price {
                self.stack.pop();
                cur_span += span;
            } else {
                break;
            }
        }

        self.stack.push((price, cur_span));

        cur_span
    }
}

pub fn main() {
    let mut stock_spanner = StockSpanner::new();
    println!("{}", stock_spanner.next(100));
    println!("{}", stock_spanner.next(80));
    println!("{}", stock_spanner.next(60));
    println!("{}", stock_spanner.next(70));
    println!("{}", stock_spanner.next(60));
    println!("{}", stock_spanner.next(75));
    println!("{}", stock_spanner.next(85));
}
