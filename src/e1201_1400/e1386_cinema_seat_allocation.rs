fn max_number_of_families(n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
    reserved_seats.push(vec![1_000_000_001, 0]);
    reserved_seats.push(vec![n, 0]);
    reserved_seats.sort_by_key(|v| v[0]);

    let mut count = 0;
    let mut current = 0;
    let mut bit = i32::MAX;
    let n = reserved_seats.len();
    for (i, reserved_seat) in reserved_seats.into_iter().enumerate() {
        if reserved_seat[0] != current {
            if i != n-1 { count += (reserved_seat[0]-current-1)*2; }
            for seat in [0b111100, 0b11110000, 0b1111000000] {
                if bit & seat == 0 {
                    count += 1;
                    bit |= seat;
                }
            }

            bit = 0;
            current = reserved_seat[0];
        }
        bit |= 1 << reserved_seat[1];
    }

    count
}

pub fn main() {
    let n = 3;
    let reserved_seats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]].into_iter().map(Vec::from).collect();
    println!("{}", max_number_of_families(n, reserved_seats));
}
