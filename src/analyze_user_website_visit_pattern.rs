use std::collections::{HashMap, HashSet};

fn most_visited_pattern(
    username: Vec<String>,
    timestamp: Vec<i32>,
    website: Vec<String>,
) -> Vec<String> {
    let mut infos = username
        .iter()
        .zip(website.iter())
        .zip(timestamp.iter())
        .map(|((u, w), t)| (u.to_owned(), w.to_owned(), t.to_owned()))
        .collect::<Vec<_>>();
    infos.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)));

    let (mut start, mut candidate) = (0, None);
    let mut hashmap = HashMap::<String, HashSet<Vec<String>>>::new();
    for (i, (u, w, _)) in infos.iter().enumerate() {
        if candidate.as_ref().is_none_or(|v| v != u) {
            candidate = Some(u.to_owned());
            start = i;
            continue;
        }

        if i - start >= 2 {
            let mut s = start;
            while i - s >= 2 {
                for j in (s + 1)..i {
                    hashmap.entry(u.to_owned()).or_default().insert(vec![
                        infos[s].1.clone(),
                        infos[j].1.clone(),
                        infos[i].1.clone(),
                    ]);
                }
                s += 1;
            }
        }
    }

    let mut res = vec![];
    let mut max = -1;
    let mut results = HashMap::new();
    for hs in hashmap.values() {
        for k in hs {
            let val = results.entry(k.clone()).or_default();
            *val += 1;
            if max < *val {
                res = k.clone();
                max = *val;
            } else if max == *val {
                res = std::cmp::min(res, k.clone());
            }
        }
    }

    res
}

pub fn main() {
    let username: Vec<String> = vec![
        "him".to_string(),
        "mxcmo".to_string(),
        "jejuvvtye".to_string(),
        "wphmqzn".to_string(),
        "uwlblbrkqv".to_string(),
        "flntc".to_string(),
        "esdtyvfs".to_string(),
        "nig".to_string(),
        "jejuvvtye".to_string(),
        "nig".to_string(),
        "mxcmo".to_string(),
        "flntc".to_string(),
        "nig".to_string(),
        "jejuvvtye".to_string(),
        "odmspeq".to_string(),
        "jiufvjy".to_string(),
        "esdtyvfs".to_string(),
        "mfieoxff".to_string(),
        "nig".to_string(),
        "flntc".to_string(),
        "mxcmo".to_string(),
        "qxbrmo".to_string(),
    ];
    let timestamp: Vec<i32> = vec![
        113355592, 304993712, 80831183, 751306572, 34485202, 414560488, 667775008, 951168362,
        794457022, 813255204, 922111713, 127547164, 906590066, 685654550, 430221607, 699844334,
        358754380, 301537469, 561750506, 612256123, 396990840, 60109482,
    ];
    let website: Vec<String> = vec![
        "k".to_string(),
        "o".to_string(),
        "o".to_string(),
        "nxpvmh".to_string(),
        "dssdnkv".to_string(),
        "kiuorlwdcw".to_string(),
        "twwginujc".to_string(),
        "evenodb".to_string(),
        "qqlw".to_string(),
        "mhpzoaiw".to_string(),
        "jukowcnnaz".to_string(),
        "m".to_string(),
        "ep".to_string(),
        "qn".to_string(),
        "wxeffbcy".to_string(),
        "ggwzd".to_string(),
        "tawp".to_string(),
        "gxm".to_string(),
        "pop".to_string(),
        "xipfkhac".to_string(),
        "weiujzjcy".to_string(),
        "x".to_string(),
    ];
    println!("{:?}", most_visited_pattern(username, timestamp, website));
}
