struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    histories: Vec<(i32, i32, i32, i32, i32)>
}

impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle, histories: Vec::new() }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.histories.push((row1, col1, row2, col2, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.histories
            .iter()
            .rev()
            .find(|v| row >= v.0 && row <= v.2 && col >= v.1 && col <= v.3)
            .map_or_else(
                || self.rectangle[row as usize][col as usize],
                |v| v.4
            )
    }
}
