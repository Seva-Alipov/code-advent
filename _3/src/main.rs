use std::fs;

fn add_two_largest_digits(bank: &str) -> String { //PART 1
    let mut left = '0';
    let mut right = '0';

    for joltage in bank.chars(){
        if right > left {
            left = right;
            right = joltage;
        }
        else if joltage > right{
            right = joltage;
        }
    }

    format!("{}{}", left, right)
}

fn add_twelve_largest_digits(bank: &str) -> String { //PART 2
    let batteries: Vec<char> = bank.chars().collect();
    let mut remaining = batteries.len();
    let mut out: Vec<char> = Vec::new();

    for &joltage in &batteries {
        remaining -= 1;

        while let Some(&last) = out.last() {
            if last < joltage && out.len() + remaining >= 12 {
                out.pop();
            }
            else {break;}
        }

        if out.len() < 12 {out.push(joltage);}
    }

    out.into_iter().collect()
}

fn main() {
    let banks = fs::read_to_string(String::from("./batteries.txt")).expect("Could not read file.");

    let mut joltage: usize = 0;
    let mut joltage_override: usize = 0;

    for bank in banks.split_whitespace() {
        joltage += add_two_largest_digits(bank).parse::<usize>().expect("Encountered non-digit character.");
        joltage_override += add_twelve_largest_digits(bank).parse::<usize>().expect("Encountered non-digit character.");
    }

    println!("The total joltage is: {joltage}");
    println!("The joltage after override is: {joltage_override}");
}

