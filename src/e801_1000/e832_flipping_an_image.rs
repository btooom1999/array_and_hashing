fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for pixels in image.iter_mut() {
        pixels.reverse();
        for num in pixels {
            *num ^= 1;
        }
    }

    image
}

pub fn main() {
    let image = [[1,1,0],[1,0,1],[0,0,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", flip_and_invert_image(image));
}
