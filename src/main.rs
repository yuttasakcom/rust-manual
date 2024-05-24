use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name);
    if score.is_none() {
        return;
    }
    println!("{}", score.unwrap());
}
