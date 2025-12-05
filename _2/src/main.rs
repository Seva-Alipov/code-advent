use std::fs;

pub fn repeats(num: usize) -> bool { //PART 2
    let mut num_cpy = num;
    let subnum: usize = num_cpy % 10;
    let mut single_repeated = true;
    let mut repeats = 0;
    while num_cpy > 0 {
        repeats += 1;
        if num_cpy % 10 != subnum {
            single_repeated = false;
            break;
        }
        num_cpy /= 10;
    }
    if num < 10usize.pow(repeats-1) {single_repeated = false;}
    if repeats < 2 {single_repeated = false;}

    if single_repeated {return true;}

    num_cpy = num;
    let subnum = num_cpy % 100;
    let mut double_repeated = true;
    repeats = 0;
    while num_cpy > 0 {
        repeats += 1;
        if num_cpy % 100 != subnum {
            double_repeated = false;
            break;
        }
        num_cpy /= 100;
    }
    if num < 100usize.pow(repeats-1) {double_repeated = false;}
    if repeats < 2 {double_repeated = false;}

    if double_repeated {return true;}

    num_cpy = num;
    let subnum = num_cpy % 1000;
    let mut triple_repeated = true;
    repeats = 0;
    while num_cpy > 0 {
        repeats += 1;
        if num_cpy % 1000 != subnum {
            triple_repeated = false;
            break;
        }
        num_cpy /= 1000;
    }
    if num < 1000usize.pow(repeats-1) {triple_repeated = false};
    if repeats < 2 {double_repeated = false;}

    if triple_repeated {return true;}

    num_cpy = num;
    let subnum = num_cpy % 10000;
    let mut quad_repeated = true;
    repeats = 0;
    while num_cpy > 0 {
        repeats += 1;
        if num_cpy % 10000 != subnum {
            quad_repeated = false;
            break;
        }
        num_cpy /= 10000;
    }
    if num < 10000usize.pow(repeats-1) {quad_repeated = false};
    if repeats < 2 {quad_repeated = false;}

    if quad_repeated {return true;}

    num_cpy = num;
    let subnum = num_cpy % 100000;
    let mut pent_repeated = true;
    repeats = 0;
    while num_cpy > 0 {
        repeats += 1;
        if num_cpy % 100000 != subnum {
            pent_repeated = false;
            break;
        }
        num_cpy /= 100000;
    }
    if num < 100000usize.pow(repeats-1) {pent_repeated = false;}
    if repeats < 2 {pent_repeated = false;}

    pent_repeated
}

pub fn repeats_twice(str: String) -> bool { //PART 1
    if str.len() % 2 != 0 {return false;}

    str[0..str.len()/2] == str[str.len()/2..str.len()]
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
            if repeats(i) {password_2 += i;}
        }

    }

    println!("The answer to part 1 is: {password}");
    println!("The answer to part 2 is: {password_2}")
}
