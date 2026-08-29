fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
    let n = watched_videos.len();

    let mut visited = vec![false; n];
    visited[id as usize] = true;

    let mut hashmap = std::collections::HashMap::<_, i32>::new();
    let mut queue = std::collections::VecDeque::from([(id as usize, 0)]);
    while let Some((id, l)) = queue.pop_front() {
        if level == l {
            for watched_video in &watched_videos[id] {
                *hashmap.entry(watched_video.clone()).or_default() += 1;
            }
        }

        for friend in &friends[id] {
            let friend = *friend as usize;
            if !visited[friend] {
                queue.push_back((friend, l+1));
                visited[friend] = true;
            }
        }
    }

    let mut hashmap = hashmap.into_iter().collect::<Vec<_>>();
    hashmap.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    hashmap.into_iter().map(|v| v.0).collect()
}

pub fn main() {
    let watched_videos = [vec!["A","B"],vec!["C", "A"],vec!["B","C", "A"],vec!["D"]]
        .into_iter()
        .map(|v| v.into_iter().map(String::from).collect())
        .collect();
    let friends = [[1,2],[0,3],[0,3],[1,2]].into_iter().map(Vec::from).collect();
    let id = 0;
    let level = 1;
    println!("{:?}", watched_videos_by_friends(watched_videos, friends, id, level))
}
