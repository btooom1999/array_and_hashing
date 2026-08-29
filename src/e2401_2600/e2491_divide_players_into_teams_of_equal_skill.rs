fn divide_players(mut skill: Vec<i32>) -> i64 {
    skill.sort();

    let mut res = 0;
    let mut i = 0;
    let mut j = skill.len() - 1;
    let condition = skill[i] as i64 + skill[j] as i64;
    while i < j {
        if skill[i] as i64 + skill[j] as i64 != condition  {
            return -1;
        }
        res += skill[i] as i64 * skill[j] as i64;
        i += 1;
        j -= 1;
    }

    res
}

pub fn main() {
    let skill = vec![3,2,5,1,3,4];
    println!("{}", divide_players(skill));
}
