const DIRECTIONS: [(i32, i32); 8] = [(1,0), (-1,0), (0,1), (0,-1), (1,1), (1,-1), (-1,1), (-1,-1)];
fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (img.len(), img[0].len());
    let mut res = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut total = 1;
            let mut sum = img[i][j];
            for direct in DIRECTIONS {
                let i = i as i32 + direct.0;
                let j = j as i32 + direct.1;

                if i < 0 || j < 0 || i == m as i32 || j == n as i32 {
                    continue;
                }
                total += 1;
                sum += img[i as usize][j as usize];
            }
            res[i][j] = sum / total;
        }
    }

    res
}

pub fn main() {
    let img = [[100,200,100],[200,50,200],[100,200,100]].into_iter().map(Vec::from).collect();
    println!("{:?}", image_smoother(img));
}
