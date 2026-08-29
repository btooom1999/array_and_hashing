use std::cmp::Ordering;

fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    products.sort();

    let mut res = Vec::new();
    let mut target = String::new();
    let mut l = 0;
    for (i, &byte) in search_word.into_bytes().iter().enumerate() {
        target.push(byte as char);

        let mut r = products.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            let val = &products[m][0..std::cmp::min(i+1, products[m].len())];
            match val.cmp(&target) {
                Ordering::Greater | Ordering::Equal => { r = m }
                Ordering::Less => { l = m + 1; }
            }
        }

        let mut words = Vec::new();
        for j in l..std::cmp::min(l+3, products.len()) {
            let val = &products[j][0..std::cmp::min(i+1, products[j].len())];
            if val != target {
                break;
            }
            words.push(products[j].clone());
        }

        res.push(words);
    }

    res
}

pub fn main() {
    let products = ["mobile","mouse","moneypot","monitor","mousepad"].into_iter().map(String::from).collect::<Vec<_>>();
    let search_word = "mouse".to_string();
    println!("{:?}", suggested_products(products, search_word));
}
