fn are_almost_equal(s1: String, s2: String) -> bool {
    let s1 = s1.into_bytes();
    let s2 = s2.into_bytes();
    let n = s1.len();

    for i in 0..n {
        if s1[i] != s2[i] {
            let mut j = i+1;
            while j < n {
                if s1[i] == s2[j] {
                    let mut temp = s2.clone();
                    temp[i] = s2[j];
                    temp[j] = s2[i];

                    if temp == s1 {
                        return true;
                    }
                }
                j += 1;
            }

            if j == n {
                return false;
            }
        }
    }

    true
}

pub fn main() {
    let s1 = "ysmpagrkzsmmzmsssutzgpxrmoylkgemgfcperptsxjcsgojwourhxlhqkxumonfgrczmjvbhwvhpnocz".to_string();
    let s2 = "ysmpagrqzsmmzmsssutzgpxrmoylkgemgfcperptsxjcsgojwourhxlhkkxumonfgrczmjvbhwvhpnocz".to_string();
    println!("{}", are_almost_equal(s1, s2));
}
