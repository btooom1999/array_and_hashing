fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.into_iter().map(|v| v.into_iter().sum::<i32>()).max().unwrap()
}

pub fn main() {
    let accounts = [[1,5],[7,3],[3,5]].into_iter().map(Vec::from).collect();
    println!("{}", maximum_wealth(accounts));
}
