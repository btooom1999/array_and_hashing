fn dfs(num: i32, limit: i32, vec: &mut Vec<i32>) {
    if num > limit {
        return;
    }

    let max = if num < 10 { num } else { num + 9 };
    let mut num = num;
    while num <= max && num <= limit {
        vec.push(num);
        dfs(num*10, limit, vec);
        num += 1;
    }
}

fn lexical_order(n: i32) -> Vec<i32> {
    let mut vec = Vec::new();

    for i in 1..=std::cmp::min(9, n) {
        dfs(i, n, &mut vec);
    }

    vec
}

pub fn main() {
    let n = 13;
    println!("{:?}", lexical_order(n));
}
