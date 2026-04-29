fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    let mut data1 = [0f32;2];
    let mut data2 = [0f32;2];

    for (i, time) in event1.into_iter().enumerate() {
        let mut total = 0f32;
        let mut divide = 1f32;
        for time in time.split(":") {
            let mut sum = 0f32;
            for c in time.as_bytes() {
                let num= (c-b'0') as f32;
                sum = (sum * 10f32) + num;
            }

            total += sum / divide;
            divide = 100f32;
        }

        data1[i] = total;
    }

    for (i, time) in event2.into_iter().enumerate() {
        let mut total = 0f32;
        let mut divide = 1f32;
        for time in time.split(":") {
            let mut sum = 0f32;
            for c in time.as_bytes() {
                let num= (c-b'0') as f32;
                sum = (sum * 10f32) + num;
            }

            total += sum / divide;
            divide = 100f32;
        }

        data2[i] = total;
    }

    if data1[1] < data2[0] || data1[0] > data2[1] || data2[1] < data1[0] || data2[0] > data1[1] {
        return false;
    }

    true
}

pub fn main() {
    let event1 = ["01:15","02:00"].into_iter().map(String::from).collect();
    let event2 = ["02:00","03:00"].into_iter().map(String::from).collect();
    println!("{}", have_conflict(event1, event2));
}
