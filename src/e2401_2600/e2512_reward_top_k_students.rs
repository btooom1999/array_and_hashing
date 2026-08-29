use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

#[derive(Debug, PartialEq, Eq)]
struct Student {
    id: i32,
    point: i32,
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.point.cmp(&other.point).then(other.id.cmp(&self.id))
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>, report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let positive_feedback = positive_feedback.into_iter().collect::<HashSet<_>>();
    let negative_feedback= negative_feedback.into_iter().collect::<HashSet<_>>();
    let mut heap = BinaryHeap::new();

    for (i, r) in report.into_iter().enumerate() {
        if r.trim().is_empty() {
            heap.push(Reverse(Student { id: student_id[i], point: 2 }));
        } else {
            let mut point = 0;
            for str in r.trim().split(' ') {
                if positive_feedback.contains(str) {
                    point += 3;
                }
                if negative_feedback.contains(str) {
                    point -= 1;
                }
            }

            heap.push(Reverse(Student { id: student_id[i], point }));
        }

        if heap.len() > k { heap.pop(); }
    }

    let mut res = Vec::new();
    while let Some(Reverse(student)) = heap.pop() {
        res.push(student.id);
    }

    res.reverse();
    res
}

pub fn main() {
    let positive_feedback = ["smart","brilliant","studious"].into_iter().map(String::from).collect();
    let negative_feedback = ["not"].into_iter().map(String::from).collect();
    let report = ["this student is studious","the student is smart"].into_iter().map(String::from).collect();
    let student_id = [1,2].to_vec();
    let k = 2;
    println!("{:?}", top_students(positive_feedback, negative_feedback, report, student_id, k));
}
