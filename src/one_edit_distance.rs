fn is_one_edit_distance(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let s_len = s.len();
    let t_len = t.len();

    if (s_len as i32 - t_len as i32).abs() > 1 {
        return false;
    }

    // s is array with smaller length
    // t is array with larger length
    let (s, t) = if s_len > t_len { (t, s) } else { (s, t) };
    let is_equal_len = s_len == t_len;
    let mut l_s = 0;
    let mut l_t = 0;
    let mut valid = 1;
    while l_s < s.len() {
        if valid < 0 {
            break;
        }
        if s[l_s] != t[l_t] {
            valid -= 1;
            if !is_equal_len {
                l_t += 1;
                continue;
            }
        }
        l_s += 1;
        l_t += 1;
    }

    if !is_equal_len {
        // if valid == 1, having an excess char at last position => RETURN true
        // if valid == 0, having an excess char in range from 0..last-1 => RETURN true
        // [in the cases above]: we need delete it and then two strings are equal
        // if valid < 0, we must execute many steps to make two strings are equal => RETURN false
        valid >= 0
    } else {
        // if valid == 1, both is equal => RETURN false
        // if valid == 0, having an difference between two strings, we need replace one char in either string => RETURN true
        // if valid < 0, we must execute many steps to make two strings are equal => RETURN false
        valid == 0
    }
}

pub fn main() {
    let s = "abc".to_string();
    let t = "acb".to_string();
    println!("{}", is_one_edit_distance(s, t));
}
