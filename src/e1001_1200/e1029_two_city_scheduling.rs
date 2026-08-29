fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut costs = costs.into_iter().map(|v| (v[0]-v[1], v[0], v[1])).collect::<Vec<_>>();
    costs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut res = 0;
    let n = costs.len();
    for i in 0..n/2 {
        res += costs[i].1;
        res += costs[n-1-i].2;
    }

    res
}

pub fn main() {
    // let costs = [[10,20],[30,200],[400,50],[30,20]].into_iter().map(Vec::from).collect();
    let costs = [[70,311],[74,927],[732,711],[126,583],[857,118],[97,928],[975,843],[175,221],[284,929],[816,602],[689,863],[721,888]].into_iter().map(Vec::from).collect::<Vec<_>>();
    println!("{}", two_city_sched_cost(costs));
}
