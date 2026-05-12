fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
    folder.sort();

    let mut res = Vec::<String>::new();
    for f in folder.iter() {
        if let Some(last) = res.last() && f.starts_with(&format!("{}/", last)) {
            continue;
        }

        res.push(f.to_string());
    }

    res
}

pub fn main() {
    let folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{:?}", remove_subfolders(folder));
}
