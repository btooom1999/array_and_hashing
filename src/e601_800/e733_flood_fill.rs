fn dfs(
    changable_color: i32,
    color: i32,
    sr: usize,
    sc: usize,
    m: usize,
    n: usize,
    image: &mut Vec<Vec<i32>>,
) {
    image[sr][sc] = color;

    if sr > 0 && image[sr-1][sc] == changable_color && changable_color != color {
        dfs(changable_color, color, sr-1, sc, m, n, image);
    }

    if sr+1 < m && image[sr+1][sc] == changable_color && changable_color != color {
        dfs(changable_color, color, sr+1, sc, m, n, image);
    }

    if sc > 0 && image[sr][sc-1] == changable_color && changable_color != color {
        dfs(changable_color, color, sr, sc-1, m, n, image);
    }

    if sc+1 < n && image[sr][sc+1] == changable_color && changable_color != color {
        dfs(changable_color, color, sr, sc+1, m, n, image);
    }
}

fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let (sr, sc) = (sr as usize, sc as usize);
    dfs(image[sr][sc], color, sr, sc, image.len(), image[0].len(), &mut image);

    image
}

pub fn main() {
    let image = [[1,1,1],[1,1,0],[1,0,1]].into_iter().map(Vec::from).collect();
    let sr = 1;
    let sc = 1;
    let color = 2;
    println!("{:?}", flood_fill(image, sr, sc, color))
}
