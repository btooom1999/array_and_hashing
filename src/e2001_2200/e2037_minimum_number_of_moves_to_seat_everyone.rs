fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort();
    students.sort();

    let mut res = 0;
    for i in 0..seats.len() {
        res += (seats[i] - students[i]).abs();
    }

    res
}

pub fn main() {
    let seats = [3,1,5].to_vec();
    let students = [2,7,4].to_vec();
    println!("{}", min_moves_to_seat(seats, students));
}
