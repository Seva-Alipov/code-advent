use std::fs;

pub fn parse_rolls(input: &String) -> Vec<Vec<char>> { //Parses rolls and pads out edges with '.' Make sure file ends with a newline.
    let mut out: Vec<Vec<char>> = Vec::new();
    out.push(Vec::new());
    out.push(vec!['.']);

    let mut current_line = 1;
    for space in input.chars() {
        match space {
            '\n' => {
                out[current_line].push('.');
                current_line += 1;
                out.push(Vec::new());
                out[current_line].push('.');
            }
            c => {
                out[current_line].push(c);
            }
        }
    }
    out[0] = vec!['.'; out[1].len()];
    out[current_line] = vec!['.'; out[1].len()];

    out
}

pub fn count_forkliftable(rolls: &Vec<Vec<char>>) -> usize { //PART 1, assumes input is padded.
    let mut forkliftable = 0;
    for row in 1..rolls.len()-1 {
        for col in 1..rolls[row].len()-1 {
            if rolls[row][col] == '@' {
                let mut adj = 0;
                if rolls[row-1][col-1] == '@' { adj += 1; }
                if rolls[row-1][col]   == '@' { adj += 1; }
                if rolls[row-1][col+1] == '@' { adj += 1; }
                if rolls[row][col-1]   == '@' { adj += 1; }
                if rolls[row][col+1]   == '@' { adj += 1; }
                if rolls[row+1][col-1] == '@' { adj += 1; }
                if rolls[row+1][col]   == '@' { adj += 1; }
                if rolls[row+1][col+1] == '@' { adj += 1; }

                if adj < 4 {forkliftable+=1;}
            }
        }
    }

    forkliftable
}

pub fn count_remove_forkliftable(rolls: &mut Vec<Vec<char>>) -> usize {
    let mut forkliftable = 0;
    for row in 1..rolls.len()-1 {
        for col in 1..rolls[row].len()-1 {
            if rolls[row][col] == '@' {
                let mut adj = 0;
                if rolls[row-1][col-1] == '@' { adj += 1; }
                if rolls[row-1][col]   == '@' { adj += 1; }
                if rolls[row-1][col+1] == '@' { adj += 1; }
                if rolls[row][col-1]   == '@' { adj += 1; }
                if rolls[row][col+1]   == '@' { adj += 1; }
                if rolls[row+1][col-1] == '@' { adj += 1; }
                if rolls[row+1][col]   == '@' { adj += 1; }
                if rolls[row+1][col+1] == '@' { adj += 1; }

                if adj < 4 {
                    forkliftable+=1;
                    rolls[row][col] = '.';
                }
            }
        }
    }

    forkliftable
}

pub fn recursive_forkliftable(rolls: &Vec<Vec<char>>) -> usize { //PART 2
    let mut local_copy = rolls.clone();

    let mut forkliftable = 0;

    loop {
        let this_iter = count_remove_forkliftable(&mut local_copy);
        if this_iter == 0 {break;}
        forkliftable += this_iter;
    }
    forkliftable
} //I lied, the function isn't actually recursive mwahahaha

fn main() {
    let rolls = fs::read_to_string(String::from("./papers.txt")).expect("Could not read file.");

    let rolls = parse_rolls(&rolls);

    let forkliftable = count_forkliftable(&rolls);
    let forkliftable_rec = recursive_forkliftable(&rolls);

    println!("There are {forkliftable} forkliftable rolls.");
    println!("If we continue, we can remove {forkliftable_rec} rolls.");
}

