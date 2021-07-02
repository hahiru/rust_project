fn main() {
   use std::collections::HashMap;

   //let mut score = HashMap::new();

   //score.insert(String::from("blue"), 1);
   //score.insert("yellow", 2);

    //println!("{:?}", score);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

//    for (team, score) in teams.iter().zip(initial_scores.iter()) {
//    	println!("{}, {}", team, score)
//    }

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}
