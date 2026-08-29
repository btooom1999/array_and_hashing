fn maximum_bob_points(mut num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
    let mut max = (0,0);
    for i in (1..1<<12).rev() {
        let mut arrows = 0;
        let mut points = 0;
        for bit in 0..12 {
            if i & 1 << bit != 0 {
                points += bit;
                arrows += alice_arrows[bit]+1;
            }
        }
        if arrows <= num_arrows && points > max.0 {
            max = (points, i);
        }
    }


    let mut res = (0..12).map(|i| if max.1 >> i & 1 == 1 {
        num_arrows -= alice_arrows[i]+1;
        alice_arrows[i]+1
    } else { 0 }).collect::<Vec<_>>();
    res[11] += num_arrows;
    res
}

pub fn main() {
    let num_arrows = 9;
    let alice_arrows = [1,1,0,1,0,0,2,1,0,1,2,0].to_vec();
    println!("{:?}", maximum_bob_points(num_arrows, alice_arrows));
}
