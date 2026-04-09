fn capitalize_title(title: String) -> String {
    title
        .split_whitespace()
        .map(|str| {
            if str.len() <= 2 {
                str.to_lowercase()
            } else {
                let mut chars = str.chars();
                let first = chars.next().unwrap().to_uppercase();
                let rest = chars.as_str().to_lowercase();
                format!("{}{}", first, rest)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn main() {
    let title = "capiTalIze tHe titLe".to_string();
    println!("{}", capitalize_title(title));
}
