#[inline]
fn convert(z: Vec<usize>, left: usize) -> Vec<usize> {
    let mut z = z[left+1..].to_vec();
    let mut flag = false;
    for i in (0..z.len()).rev() {
        if flag {
            if z[i] == left { continue; }
            z[i] = z[i+1]+1;
        } else if z[i] == left {
            flag = true;
        }
    }

    z
}

fn z_algorithm(s: &[u8], p: &[u8]) -> Vec<usize> {
    let temp = format!("{}#{}", String::from_utf8(p.to_vec()).unwrap(), String::from_utf8(s.to_vec()).unwrap());
    let temp = temp.as_bytes();
    let n = temp.len();

    let mut z = vec![0; n];
    let mut left = 0;
    let mut right = 0;
    for k in 1..n {
        let k1 = k - left;
        if k > right || z[k1] > right - k {
            left = k;
            right = right.max(k);

            while right < n && temp[right] == temp[right-left] {
                right += 1;
            }

            z[k] = right - left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }
    }

    convert(z, p.len())
}

fn shortest_matching_substring(s: String, p: String) -> i32 {
    let mut splitted = p.split('*');
    let a = splitted.next().unwrap_or("");
    let b = splitted.next().unwrap_or("");
    let c = splitted.next().unwrap_or("");
    let (n1, n2, n3) = (a.len(), b.len(), c.len());

    let z_a = z_algorithm(s.as_bytes(), a.as_bytes());
    let z_b = z_algorithm(s.as_bytes(), b.as_bytes());
    let z_c = z_algorithm(s.as_bytes(), c.as_bytes());
    println!("{:?}", z_a);
    println!("{:?}", z_b);
    println!("{:?}", z_c);

    let s = s.as_bytes();
    let mut res = i32::MAX;
    for i in 0..s.len() {
        let x = z_a[i];
        let y = *z_b.get(i+z_a[i]).unwrap_or(&0);
        let z = *z_c.get(i+x+y).unwrap_or(&0);
        if x>=n1 && y>=n2 && z>=n3 {
            res = res.min((x+y+z) as i32);
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let s = "aaaaaaaa".to_string();
    let p = "a*aaaaaaa*".to_string();
    println!("{}", shortest_matching_substring(s, p));
}
