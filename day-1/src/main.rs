use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Could not read file");

    let calories: Vec<&str> = contents.split("\n\n").collect();

    let mut calorie_count: Vec<u32> = Vec::new();

    for calorie in calories {
        let single_calories: u32 = calorie.split("\n").map(|x| match x.parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        }).collect::<Vec<u32>>().iter().sum();

        calorie_count.push(single_calories);
    }

    let  Some((i , max)) = calorie_count.iter().enumerate().max_by(|&(_, item)| item);

    println!("Largest calorie count: {max}");    
}
