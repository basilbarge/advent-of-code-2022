fn main() {
    let input = include_str!("./input.txt");

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

    let mut elf_group: Vec<(&str, &str, &str)> = Vec::new();

    for (i, rucksack) in rucksacks.iter().enumerate().step_by(3) {
        elf_group.push((rucksack, rucksacks[i + 1], rucksacks[i + 2]));
    }

    let mut first_matches:Matches<'_, char> = 'a';

    for rucksack in &elf_group {
        for item in rucksack.0.chars() {
            first_matches = rucksack.1.matches(item);
        }
    }
    
    let mut final_match = Vec::new();

    for rucksack in elf_group {
        for item_match in first_matches {
            match rucksack.2.matches(item_match).last() {
                Some(matched_item) => {
                    final_match.push(matched_item);
                    break;
                },
                None => continue,
            }
        }
    }

}
