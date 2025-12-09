
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// count number of times the dial lands on 0
fn part1() {
    let mut zero_count: u32 = 0;
    let mut dial: i32 = 50;
    let dial_max: i32 = 100; // dial can contain values between 0-99

    println!("!===============================================================================");
    println!("part 1");

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{} {}", line, line.chars().nth(0).unwrap());

            let turn_amount = line[1..].to_string();

            match turn_amount.parse::<i32>() {
                Ok(n) => {
                    if line.chars().nth(0).unwrap() == 'R' {
                        dial += n;
                    } else {
                        dial -= n;
                    }
                },
                Err(e) => println!("{}", e),
            }

            //println!("dial before modulo {}", dial);

            dial = dial.rem_euclid(dial_max);

            //println!("dial after modulo {}", dial);

            if dial == 0 {
                zero_count += 1;
            }

        }
    }

    println!("{:?}", zero_count);
}

// count number of time dial lands on 0 or passes 0
fn part2() {
    let mut zero_count: i32 = 0;
    let mut dial: i32 = 50;
    let dial_max: i32 = 100; // dial can contain values between 0-99

    println!("!===============================================================================");
    println!("part 2");

    //println!("dial        {}", dial);

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{} {}", dial, line);

            let turn_amount = line[1..].to_string();
            let mut dial_turn: i32 = dial;

            match turn_amount.parse::<i32>() {
                Ok(n) => {
                    if line.chars().nth(0).unwrap() == 'R' {
                        dial_turn += n;
                    } else {
                        dial_turn -= n;
                    }
                },
                Err(e) => println!("{}", e),
            }

            //println!("dial turn   {}", dial_turn);

            
            /*
            if the dial after turn is 0 then we landed on zero and we increment

            if the dial value was non-zero before turn and negative after turn then we went left and passed 0

            if the absolute dial 'value' before modulo is less than dial_max (100) then division will result in 0

            if the absolute dial 'value' before modulo is greater than dial_max (100) then the result is the number of times we passed 0
             */
            if dial_turn == 0 {
                zero_count += 1;
            } else if dial_turn < 0 && dial > 0{
                zero_count += 1;
            }

            zero_count += dial_turn.abs() / dial_max;

            dial = dial_turn.rem_euclid(dial_max);

            //println!("dial modulo {}", dial);

            //println!("zero_count  {}", zero_count);


        }
    }

    println!("{:?}", zero_count);
}

fn main() {
    part1();
    part2();
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
