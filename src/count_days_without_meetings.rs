fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut max = meetings[0][1];
    let mut day = meetings[0][0];
    for i in 1..meetings.len() {
        let (a, b) = (meetings[i][0], meetings[i][1]);
        if max >= a {
            max = max.max(b);
        } else {
            days -= max-day+1;
            max = b;
            day = a;
        }
    }
    days-(max-day+1)
}

pub fn main() {
    let days = 4;
    let meetings = [[2,3],[1,2],[2,3],[2,4],[1,2],[1,3]].into_iter().map(Vec::from).collect();
    println!("{}", count_days(days, meetings));
}
