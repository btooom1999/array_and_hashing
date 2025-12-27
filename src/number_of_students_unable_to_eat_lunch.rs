fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
    let mut i = students.len() as i32;
    while i > 0 {
        if students[0] == sandwiches[0] {
            students.remove(0);
            sandwiches.remove(0);
            i = students.len() as i32;
        } else {
            students.rotate_left(1);
            i -= 1;
        }
    }

    students.len() as i32
}

pub fn main() {
    let students = vec![1, 1, 1, 0, 0, 1];
    let sandwiches = vec![1, 0, 0, 0, 1, 1];
    println!("{}", count_students(students, sandwiches));
}
