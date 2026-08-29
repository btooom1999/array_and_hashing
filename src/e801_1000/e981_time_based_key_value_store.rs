use std::collections::HashMap;

#[derive(Debug)]
struct TimeMap(HashMap<String, Vec<(String, i32)>>);

impl TimeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let data = self.0.entry(key).or_default();
        let mut stack = Vec::new();

        while let Some(val) = data.pop() {
            if val.1 > timestamp {
                stack.push(val);
            } else if val.1 < timestamp {
                data.push(val);
                break;
            }
        }

        data.push((value, timestamp ));
        data.extend(stack);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(timestamps) = self.0.get(&key) {
            let mut l = 0;
            let mut r = timestamps.len() - 1;

            while l < r {
                let m = (l + r) / 2;
                if timestamps[m].1 == timestamp {
                    return timestamps[m].0.clone();
                } else if timestamps[m].1 > timestamp {
                    r = m;
                } else {
                    l = m + 1;
                }
            }

            if timestamps[l].1 > timestamp {
                if l == 0 {
                    return String::new();
                }

                timestamps.get(l-1).unwrap().0.clone()
            } else {
                timestamps[l].0.clone()
            }
        } else {
            String::new()
        }
    }
}

pub fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 3);  // store the key "foo" and value "bar" along with timestamp = 1.
    println!("{}", time_map.get("foo".to_string(), 1));         // return "bar"
    println!("{}", time_map.get("foo".to_string(), 3));         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
    time_map.set("foo".to_string(), "bar2".to_string(), 4); // store the key "foo" and value "bar2" along with timestamp = 4.
    println!("{}", time_map.get("foo".to_string(), 4));         // return "bar2"
    println!("{}", time_map.get("foo".to_string(), 5));         // return "bar2"
}
