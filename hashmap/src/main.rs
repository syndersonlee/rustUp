use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //field_name, field_value는 만료

    let mut scores_rev = HashMap::new();

    scores_rev.insert(String::from("Blue"), 10);
    scores_rev.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    

    let mut scores_overwrites = HashMap::new();
    scores_overwrites.insert(String::from("Blue"), 10);
    scores_overwrites.entry(String::from("Yellow")).or_insert(50);
    scores_overwrites.entry(String::from("Blue")).or_insert(50);


    println!("{:?}", scores_overwrites);
}
