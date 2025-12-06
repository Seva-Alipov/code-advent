use std::fs;

pub fn repeats_twice(str: String) -> bool { //PART 1
    str[0..str.len()/2] == str[str.len()/2..str.len()]
}

pub fn repeats(str: String) -> bool { //PART 2
    let len = str.len();
    let mut repeats: [bool; 7] = [true; 7]; //May need to expand this if your puzzles has longer numbers.

    'outer: for substr_len in 1..8 {
        if len % substr_len != 0 || len <= substr_len {
            repeats[substr_len-1] = false;
            continue;
        }

        let substr = &str[0..substr_len];
        for i in 1..len/substr_len {
            if substr != &str[i*substr_len..(i+1)*substr_len] {
                repeats[substr_len-1] = false;
                continue 'outer;
            }
        }
    }

    repeats.contains(&true)
}

fn main() {
    let ranges = fs::read_to_string("./puzzle.txt").expect("Could not read puzzle.");

    let mut password:usize = 0;
    let mut password_2 = 0;

    for range in ranges.split(',') {
        let (from, to) = range.split_once('-')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();

        for i in from..=to {
            if repeats_twice(i.to_string()) {password += i;}
            if repeats(i.to_string()) {password_2 += i;}
        }

    }

    println!("The answer to part 1 is: {password}");
    println!("The answer to part 2 is: {password_2}")
}
