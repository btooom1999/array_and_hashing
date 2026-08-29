fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    tiles.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut prefix = vec![(0,0); tiles.len() * 2 + 1];
    for (i, tile) in tiles.iter().enumerate() {
        let x = (i + 1) * 2 - 1;
        prefix[x] = (tile[0], prefix[x-1].1 + 1);
        prefix[x+1] = (tile[1], tile[1] - tile[0] + prefix[x].1);
    }

    let mut res = 0;
    for (l, (num, amount)) in prefix.iter().enumerate().skip(1) {
        let mut l = l;
        let mut r = prefix.len() - 1;
        let target = num + carpet_len - 1;
        while l < r {
            let m = (l + r) / 2;

            if prefix[m].0 == target {
                l = m;
                break;
            } else if prefix[m].0 < target {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if prefix[l].0 > target {
            if l % 2 == 0 {
                res = res.max(prefix[l-1].1 - amount + 1 + target - prefix[l-1].0);
            } else {
                res = res.max(prefix[l-1].1 - amount + 1);
            }
        } else {
            res = res.max(prefix[l].1 - amount + 1);
        }
    }

    res
}

pub fn main() {
    // let tiles = [[1,5],[10,11],[12,18],[20,25],[30,32]].iter().map(Vec::from).collect();
    // let carpet_len = 10;
    let tiles = vec![vec![1,6], vec![7,9]];
    let carpet_len = 5;
    println!("{}", maximum_white_tiles(tiles, carpet_len));
}


