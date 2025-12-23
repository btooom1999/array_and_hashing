use std::collections::HashSet;

fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut hashset = HashSet::with_capacity(emails.len());

    for e in emails.into_iter() {
        let strs = e.split('@').collect::<Vec<_>>();
        let [local_name, domain_name] = strs.as_slice().try_into().unwrap();
        let mut local_name = local_name.replace(".", "");
        if let Some(end) = local_name.find('+') {
            local_name = local_name.get(..end).unwrap().to_string();
        }
        hashset.insert(format!("{}@{}", local_name, domain_name));
    }

    hashset.len() as i32
}

pub fn main() {
    let emails = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    println!("{}", num_unique_emails(emails));
}
