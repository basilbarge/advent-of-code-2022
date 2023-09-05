use std::collections::HashSet;

fn main() {
    let input = include_str!("./input-test.txt");

    let rucksacks: Vec<&str> = input.lines().collect();
    
    let mut rucksack_by_compartment: Vec<(&str, &str)> = Vec::new();
    
    for rucksack in &rucksacks {
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
        bytes.push(calculate_bytes(matched));
    }

    let mut sum: u32 = 0;
    for byte in bytes {
        sum += calculate_priority_from_byte(byte);
    }

    println!("Priority sum is {sum}");

    let mut elf_group: Vec<(&str, &str, &str)> = Vec::new();

    for (i, rucksack) in rucksacks.iter().enumerate().step_by(3) {
        elf_group.push((rucksack, rucksacks[i + 1], rucksacks[i + 2]));
    }

    let mut matches: Vec<&str> = Vec::new();

    for rucksack in &elf_group {
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

    for matched in &matches {
        println!("First matched: {matched}");
    }

    let mut final_matches: Vec<&str> = Vec::new();

    for rucksack in &elf_group {
        for matched in &matches {
            for item in rucksack.2.chars() {
                match matched.matches(item).last() {
                    Some(matched_item) => {
                        println!("Second pass: {matched_item}");
                        final_matches.push(matched_item);
                        break;
                    },
                    None => continue,
                }
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();

    for matched in &final_matches {
        bytes.push(calculate_bytes(matched));
    }

    let mut sum: u32 = 0;
    for byte in bytes {
        sum += calculate_priority_from_byte(byte);
    }

    println!("Badges sum is {sum}");

}

fn calculate_bytes(string_to_calc: &str) -> u8 {
    let mut sum: u8 = 0;
    for byte in string_to_calc.bytes() {
        sum += byte; 
    }

    sum
}

fn calculate_priority_from_byte(byte: u8) -> u32 {
    if byte >= 65 && byte <= 90 {
        (u32::from_be_bytes([0, 0, 0, byte]) % 48) + 10
    } else {
        u32::from_be_bytes([0, 0, 0, byte]) % 96
    }
}
