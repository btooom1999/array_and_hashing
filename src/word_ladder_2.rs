use std::collections::{HashMap, VecDeque};

fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let n = word_list.len();
    let check_transformation_once = |from: String, to: String| -> bool {
        let mut count = 0;
        let from = from.as_bytes();
        let to = to.as_bytes();
        for i in 0..from.len() {
            if from[i] != to[i] {
                count += 1;
                if count == 2 { return false; }
            }
        }

        count == 1
    };

    let mut visited = vec![usize::MAX; n];
    if let Some(i) = word_list.iter().position(|v| *v == begin_word) {
        visited[i] = 0;
    }

    let mut memo = HashMap::<_, Vec<Vec<usize>>>::new();
    let mut q = VecDeque::from([
        (begin_word.clone(), 0, vec![])
    ]);

    let mut shortest = (word_list.len(), vec![]);
    while let Some((w, count, mut path)) = q.pop_front() {
        if w == end_word && count <= shortest.0 {
            shortest = (count, path.clone());
            continue;
        }

        for i in 0..n {
            if visited[i] > count && check_transformation_once(w.clone(), word_list[i].clone()) {
                if visited[i] == count+1 {
                    memo.entry((i, count+1)).or_default().push(path.clone());
                } else {
                    visited[i] = count+1;
                    path.push(i);
                    q.push_back((word_list[i].clone(), count+1, path.clone()));
                    path.pop();
                }
            }
        }
    }

    if shortest.1.is_empty() { return vec![]; }

    let mut res = Vec::new();
    const DEFAULT: Vec<Vec<usize>> = vec![];
    res.push(shortest.1.clone());
    let n = shortest.1.len();
    let mut i = n;
    while i > 0 {
        for k in 0..res.len() {
            let row = res[k].clone();
            for segment in memo.get(&(row[i-1], i)).unwrap_or(&DEFAULT) {
                let chained = segment.iter().chain(row[i-1..].iter()).cloned().collect::<Vec<_>>();
                res.push(chained);
            }
        }
        i -= 1;
    }

    res
        .into_iter()
        .map(|v| {
            let n = v.len()+1;
            let mut data = vec![String::new(); n];
            data[0] = begin_word.clone();
            for (i, idx) in v.into_iter().enumerate() {
                data[i+1] = word_list[idx].clone();
            }

            data
        })
        .collect()
}

pub fn main() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = ["hot","dot","dog","lot","log","cog"].into_iter().map(String::from).collect();
    // let begin_word = "qa".to_string();
    // let end_word = "sq".to_string();
    // let word_list = ["si","go","se","cm","so","ph","mt","db","mb","sb","kr","ln","tm","le","av","sm","ar","ci","ca","br","ti","ba","to","ra","fa","yo","ow","sn","ya","cr","po","fe","ho","ma","re","or","rn","au","ur","rh","sr","tc","lt","lo","as","fr","nb","yb","if","pb","ge","th","pm","rb","sh","co","ga","li","ha","hz","no","bi","di","hi","qa","pi","os","uh","wm","an","me","mo","na","la","st","er","sc","ne","mn","mi","am","ex","pt","io","be","fm","ta","tb","ni","mr","pa","he","lr","sq","ye"]
    //     .into_iter()
    //     .map(String::from)
    //     .collect();
    println!("{:?}", find_ladders(begin_word, end_word, word_list));
}
