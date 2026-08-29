fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    if n == 0 {
        return true;
    }

    if flowerbed.len() == 1 {
        return flowerbed[0] == 0 && n == 1;
    }

    if flowerbed.len() == 2 {
        return !flowerbed.contains(&1) && n == 1;
    }

    for i in 0..flowerbed.len() {
        if n == 0 {
            return true;
        }

        if i == 0
            && flowerbed[i] == 0
            && let Some(next_val) = flowerbed.get(i + 1)
            && *next_val == 0
        {
            flowerbed[i] = 1;
            n -= 1;
        }

        if i == flowerbed.len() - 1
            && flowerbed[i] == 0
            && let Some(prev_val) = flowerbed.get(i - 1)
            && *prev_val == 0
        {
            flowerbed[i] = 1;
            n -= 1;
        }

        if i > 0
            && i < flowerbed.len()
            && flowerbed[i] == 0
            && let (Some(prev_val), Some(next_val)) = (flowerbed.get(i - 1), flowerbed.get(i + 1))
            && *prev_val == 0
            && *next_val == 0
        {
            flowerbed[i] = 1;
            n -= 1;
        }
    }

    n == 0
}

pub fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    println!("{}", can_place_flowers(flowerbed, n));
}
