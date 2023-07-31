fn main() {
    let input = include_str!("./input.txt");

    let rucksacks: Vec<&str> = input.lines().collect();
    
    let mut rucksack_by_compartment: Vec<(&str, &str)> = Vec::new();
    
    for rucksack in rucksacks {
        rucksack_by_compartment.push(rucksack.split_at(rucksack.len() / 2));
    }

    let mut matches: Vec<&str> = Vec::new();

    for rucksack in rucksack_by_compartment {
        for item in rucksack.0.chars() {
            match rucksack.1.matches(item).last() {
                Some(matched_item) => {
                    matches.push(matched_item);
                    break;
                },
                None => continue,
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();

    for matched in &matches {
        let mut sum: u8 = 0;
        for byte in matched.bytes() {
            sum += byte; 
        }

        bytes.push(sum);
    }

    let mut sum: u32 = 0;
    for byte in bytes {
        if byte >= 65 && byte <= 90 {
            sum += (u32::from_be_bytes([0, 0, 0, byte]) % 48) + 10;
        } else if byte >= 97 && byte <= 122 {
            sum += u32::from_be_bytes([0, 0, 0, byte]) % 96;
        }
    }

    println!("Priority sum is {sum}");

}
