use std::collections::{HashMap, HashSet, VecDeque};

fn dfs(
    src: String,
    target: String,
    map: &HashMap<String, Vec<(String, f64)>>
) -> f64 {
    if !map.contains_key(&src) || !map.contains_key(&target) {
        return -1f64;
    }

    let mut queue = VecDeque::from([(src, 1f64)]);
    let mut visited = HashSet::new();

    while let Some((str1, res1)) = queue.pop_front() {
        if str1 == target {
            return res1;
        }

        for (str2, res2) in map.get(&str1).unwrap().clone() {
            if visited.contains(&str2) {
                continue;
            }

            visited.insert(str2.clone());
            queue.push_back((str2, res1 * res2));
        }
    }

    -1f64
}

fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut map = HashMap::<String, Vec<(String, f64)>>::new();
    for (i, equations) in equations.iter().enumerate() {
        map.entry(equations[0].clone()).or_default().push((equations[1].clone(), values[i]));
        map.entry(equations[1].clone()).or_default().push((equations[0].clone(), 1f64 / values[i]));
    }

    queries.into_iter().map(|v| dfs(v[0].clone(), v[1].clone(), &map)).collect()
}

pub fn main() {
    let equations = [["a","b"],["b","c"]].into_iter().map(|v| v.into_iter().map(String::from).collect()).collect();
    let values = [2.0,3.0].to_vec();
    let queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]].into_iter().map(|v| v.into_iter().map(String::from).collect()).collect();
    println!("{:?}", calc_equation(equations, values, queries));
}
