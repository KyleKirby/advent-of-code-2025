
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_nth_highest_voltage(batteries: &Vec<u32>, start: &mut usize, nth_digit: usize) -> u32 {
    let mut highest_joltage: u32 = batteries[*start];
    let mut highest_index: usize = *start;

    for i in *start..batteries.len()-nth_digit {
        // get the highest joltage with at least n batteries remaining to select from
        if batteries[i] > highest_joltage {
            highest_joltage = batteries[i];
            highest_index = i;
        }
    }

    *start = highest_index + 1;
    return highest_joltage;
}

fn get_max_joltage(battery_bank: &String, num_batteries: usize) -> u64 {
    // get the two batteries with the largest joltage
    let batteries: Vec<_> = battery_bank.chars().map(|c| c.to_digit(10).expect("should be a digit")).collect();

    let mut selected_batteries = vec![];
    let mut last_index: usize = 0;

    for i in 1..=num_batteries {
        selected_batteries.push(get_nth_highest_voltage(&batteries, &mut last_index, num_batteries - i));
    }
    
    let mut joltage_str: String = "".to_string();

    for i in 0..num_batteries {
        joltage_str.push_str(&selected_batteries[i].to_string());
    }


    match joltage_str.to_string().parse::<u64>() {
        Ok(n) => {
            return n;
        },
        Err(e) =>  {
            println!("{}", e);
            return 0;
        },
    }
}

fn part1() {

    println!("!===============================================================================");
    println!("part 1");

    let mut total_joltage: u64 = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{}", line);
            total_joltage += get_max_joltage(&line, 2);
        }
    }

    println!("total_joltage {}", total_joltage);

}

fn part2() {

    println!("!===============================================================================");
    println!("part 2");

    let mut total_joltage: u64 = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{}", line);
            total_joltage += get_max_joltage(&line, 12);
        }
    }

    println!("total_joltage {}", total_joltage);

}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    //part1();
    part2();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
