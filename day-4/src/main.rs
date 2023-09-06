fn main() {
    let pairs: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut ids: Vec<Vec<&str>> = Vec::new();

    for pair in &pairs {
        ids.push(pair.split(",").collect());     
    }

    for id in ids {
        for i in id {
        println!("{i}");
        }
    }


}
