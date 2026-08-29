fn grid_illumination(_n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut lamps = lamps.into_iter().collect::<std::collections::HashSet<_>>();
    let mut memo = std::collections::HashMap::<_, i32>::new();
    for lamp in &lamps {
        let (x, y) = (lamp[0], lamp[1]);
        *memo.entry(format!("c{}", y)).or_default() += 1;
        *memo.entry(format!("r{}", x)).or_default() += 1;
        *memo.entry(format!("ld{}-{}", x-x.min(y), y-x.min(y))).or_default() += 1;
        *memo.entry(format!("rd{}-{}", 0, x+y)).or_default() += 1;
    }

    let mut res = vec![0; queries.len()];
    for (i, query) in queries.into_iter().enumerate() {
        let (x, y) = (query[0], query[1]);
        if memo.contains_key(&format!("c{}", y)) {
            res[i] = 1;
        }
        if memo.contains_key(&format!("r{}", x)) {
            res[i] = 1;
        }
        if memo.contains_key(&format!("ld{}-{}", x-x.min(y), y-x.min(y))) {
            res[i] = 1;
        }
        if memo.contains_key(&format!("rd{}-{}", 0, x+y)) {
            res[i] = 1;
        }

        for dir in [(0,0), (1,0), (-1,0), (0,1), (0,-1), (1,1), (1,-1), (-1,1), (-1,-1)] {
            let x = x+dir.0;
            let y = y+dir.1;
            if lamps.contains(&vec![x,y]) {
                if let Some(val) = memo.get_mut(&format!("c{}", y)) {
                    *val -= 1;
                    if *val == 0 { memo.remove(&format!("c{}", y)); }
                }

                if let Some(val) = memo.get_mut(&format!("r{}", x)) {
                    *val -= 1;
                    if *val == 0 { memo.remove(&format!("r{}", x)); }
                }

                if let Some(val) = memo.get_mut(&format!("ld{}-{}", x-x.min(y), y-x.min(y))) {
                    *val -= 1;
                    if *val == 0 { memo.remove(&format!("ld{}-{}", x-x.min(y), y-x.min(y))); }
                }

                if let Some(val) = memo.get_mut(&format!("rd{}-{}", 0, x+y)) {
                    *val -= 1;
                    if *val == 0 { memo.remove(&format!("rd{}-{}", 0, x+y)); }
                }

                lamps.remove(&vec![x,y]);
            }
        }
    }

    res
}

pub fn main() {
    let n = 100;
    let lamps = [[7,55],[53,61],[2,82],[67,85],[81,75],[38,91],[68,0],[60,43],[40,19],[12,75],[26,2],[24,89],[42,81],[60,58],[77,72],[33,24],[19,93],[7,16],[58,54],[78,57],[97,49],[65,16],[42,75],[90,50],[89,34],[76,97],[58,23],[62,47],[94,28],[88,65],[3,87],[81,10],[12,81],[44,81],[54,92],[90,54],[17,54],[27,82],[48,15],[8,46],[4,99],[15,13],[90,77],[2,87],[18,33],[52,90],[4,95],[57,61],[31,22],[32,8],[49,26],[24,65],[88,55],[88,38],[64,76],[94,76],[59,12],[41,46],[80,28],[38,36],[65,67],[75,37],[56,97],[83,57],[2,4],[44,43],[71,90],[62,40],[79,94],[81,11],[96,34],[38,11],[22,3],[54,96],[78,33],[54,54],[79,98],[1,28],[0,32],[37,11]].into_iter().map(Vec::from).collect();
    let queries = [[24,84],[95,68],[80,35],[31,53],[69,45],[85,29],[87,25],[42,47],[7,59],[99,3],[31,70],[64,62],[44,91],[55,25],[15,52],[95,33],[21,29],[61,34],[93,34],[79,27],[30,86],[52,0],[18,10],[5,1],[40,21],[11,48],[55,94],[22,42],[81,0],[39,43],[5,25],[43,29],[45,47],[83,93],[77,70],[22,63],[30,73],[18,48],[39,88],[91,47]].into_iter().map(Vec::from).collect();
    println!("{:?}", grid_illumination(n, lamps, queries));
}
