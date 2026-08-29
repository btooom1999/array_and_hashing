fn is_valid_serialization(preorder: String) -> bool {
    let preorder = preorder.split(",").collect::<Vec<_>>();
    let n = preorder.len();
    let mut stack = 0;
    let mut i = 0;
    while i < n {
        let node = preorder[i];
        if node == "#" {
            if stack == 0 { break; }
            stack -= 1;
            i += 1;
        } else {
            if i+1 == n || i+2 == n { return false; }
            if preorder[i+1] == "#" {
                i += 2;
            } else {
                stack += 1;
                i += 1;
            }
        }
    }

    i == n-1
}

pub fn main() {
    // let preorder = "9,9,9,19,#,9,#,#,#,9,#,69,#,#,#".to_string();
    let preorder = "9,9,#,9,#,#,9,9,#,#,9,9,#,#,79,#,#".to_string();
    println!("{}", is_valid_serialization(preorder));
}
