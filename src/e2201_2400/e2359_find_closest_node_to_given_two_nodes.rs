use std::collections::HashSet;

fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    let mut idx1 = node1;
    let mut idx2 = node2;
    let mut hashset1 = HashSet::new();
    let mut hashset2 = HashSet::new();

    while idx1 != idx2 {
        let val1 = edges[idx1 as usize];
        let val2 = edges[idx2 as usize];

        if hashset1.contains(&idx2) && hashset2.contains(&idx1) {
            return idx1.min(idx2);
        }
        if hashset1.contains(&idx2) {
            return idx2;
        }
        if hashset2.contains(&idx1) {
            return idx1;
        }
        if (val1 == -1 && hashset2.contains(&val2))
        || (val2 == -1 && hashset1.contains(&val1))
        || (hashset1.contains(&val1) && hashset2.contains(&val2)) {
            return -1;
        }
        if val1 == idx2 && val2 == idx1 {
            return idx1.min(idx2);
        }
        if val1 == idx2 {
            return idx2;
        }
        if val2 == idx1 {
            return idx1;
        }
        if val1 == val2 {
            return val1;
        }
        if val1 != -1 {
            hashset1.insert(idx1);
            idx1 = val1;
        }
        if val2 != -1 {
            hashset2.insert(idx2);
            idx2 = val2;
        }
    }

    idx1
}

pub fn main() {
    let edges = [2,0,0].to_vec();
    // let edges = [-1,-1].to_vec();
    let node1 = 2;
    let node2 = 0;
    println!("{}", closest_meeting_node(edges, node1, node2));
}
