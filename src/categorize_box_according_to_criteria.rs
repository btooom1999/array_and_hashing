fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let is_bulky = length >= 10000 || width >= 10000 || height >= 10000 || length.saturating_mul(width).saturating_mul(height) >= 1_000_000_000;
    let is_heavy = mass >= 100;
    if is_heavy && is_bulky {
        return "Both".to_string();
    }

    if is_heavy {
        return "Heavy".to_string();
    }

    if is_bulky {
        return "Bulky".to_string();
    }

    "Neither".to_string()
}

pub fn main() {
    let length = 1000;
    let width = 35;
    let height = 700;
    let mass = 300;
    println!("{}", categorize_box(length, width, height, mass));
}
