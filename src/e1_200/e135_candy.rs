fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];
    for i in 0..n {
        if i>0 && ratings[i] > ratings[i-1] {
            candies[i] = candies[i].max(1 + candies[i-1]);
        }
    }

    for i in (0..n).rev() {
        if i+1<n && ratings[i] > ratings[i+1] {
            candies[i] = candies[i].max(1 + candies[i+1]);
        }
    }

    candies.into_iter().sum()
}

pub fn main() {
    let ratings = [60, 80, 100, 100, 100, 100, 100].to_vec();
    println!("{}", candy(ratings));
}

