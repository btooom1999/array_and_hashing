fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    items.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut max_items = vec![0; items.len()];
    for (i, item) in items.iter().enumerate() {
        max_items[i] = std::cmp::max(item[1], max_items[std::cmp::max(i, 1)-1]);
    }

    let mut res = vec![0; queries.len()];
    for (i, &price) in queries.iter().enumerate() {
        let mut l = 0;
        let mut r = items.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if items[m][0] > price {
                r = m;
            } else {
                l = m + 1;
            }
        }

        let mut l = l as i32;
        if items[l as usize][0] > price {
            l -= 1;
        }

        if l >= 0 {
            res[i] = max_items[l as usize];
        }

    }

    res
}

pub fn main() {
    let items = [[193,732],[781,962],[864,954],[749,627],[136,746],[478,548],[640,908],[210,799],[567,715],[914,388],[487,853],[533,554],[247,919],[958,150],[193,523],[176,656],[395,469],[763,821],[542,946],[701,676]].into_iter().map(Vec::from).collect::<Vec<_>>();
    let queries = [885,1445,1580,1309,205,1788,1214,1404,572,1170,989,265,153,151,1479,1180,875,276,1584].to_vec();
    println!("{:?}", maximum_beauty(items, queries));
}
