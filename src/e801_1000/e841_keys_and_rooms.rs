fn dfs(
    idx: usize,
    rooms: &Vec<Vec<i32>>,
    owned: &mut Vec<i32>,
) -> bool {
    if owned.iter().all(|v| *v > 0) {
        return true;
    }

    for &room in &rooms[idx] {
        if owned[room as usize] == 0 {
            owned[room as usize] += 1;
            if dfs(idx, rooms, owned) {
                return true;
            }
        }
    }

    false
}

fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut owned = vec![0; rooms.len()];
    owned[0] = 1;
    dfs(0, &rooms, &mut owned)
}

pub fn main() {
    let rooms = vec![vec![1,3],vec![3,0,1],vec![2],vec![0]];
    println!("{}", can_visit_all_rooms(rooms));
}
