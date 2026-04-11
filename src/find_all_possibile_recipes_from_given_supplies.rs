use std::collections::{HashMap, HashSet, VecDeque};

fn dfs(
    recipe: String,
    map: &HashMap<String, Vec<String>>,
    supplies: &mut HashSet<String>,
    visited: &mut HashMap<String, i32>,
) -> bool {
    if !map.contains_key(&recipe) {
        return false;
    }

    if let Some(&num) = visited.get(&recipe) {
        return num == 1;
    }

    visited.insert(recipe.clone(), 0);
    for ingredient in map.get(&recipe).unwrap().clone() {
        if supplies.contains(&ingredient) {
            continue;
        }

        if !dfs(ingredient, map, supplies, visited) {
            return false;
        }
    }

    supplies.insert(recipe.clone());
    visited.insert(recipe, 1);
    true
}

fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
    let mut supplies = supplies.into_iter().collect::<HashSet<String>>();
    let mut map = HashMap::<String, Vec<String>>::new();
    let n = recipes.len();
    for i in 0..n {
        map.insert(recipes[i].clone(), ingredients[i].clone());
    }

    let mut res = Vec::new();
    for i in 0..n {
        if dfs(recipes[i].clone(), &map, &mut supplies, &mut HashMap::new()) {
            res.push(recipes[i].clone());
        }
    }

    res
}

pub fn main() {
    let recipes = ["bread"].into_iter().map(String::from).collect();
    let ingredients = [["yeast","flour"]].into_iter().map(|v| v.into_iter().map(String::from).collect()).collect();
    let supplies = ["yeast","flour","corn"].into_iter().map(String::from).collect();
    println!("{:?}", find_all_recipes(recipes, ingredients, supplies));
}
