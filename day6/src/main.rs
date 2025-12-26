
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn part1() {

    println!("!===============================================================================");
    println!("part 1");

    let mut numbers: Vec<Vec<i64>> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line_split: Vec<_> = line.split_whitespace().collect();
            println!("{:?}", line_split);


            if line_split[0].chars().nth(0).unwrap() == '*' || line_split[0].chars().nth(0).unwrap() == '+' {
                println!("start parsing operators");

                let mut grand_total: i64 = 0;

                for (i,operator) in line_split.iter().enumerate() {
                    if *operator == "*" {
                        //println!("*");
                        let mut sub_total: i64 = 1;

                        for line_numbers in &numbers {
                            //println!("{}", line_numbers[i]);
                            sub_total *= line_numbers[i];
                        }

                        grand_total += sub_total;
                    } else if *operator == "+" {
                        //println!("+");
                        let mut sub_total: i64 = 0;

                        for line_numbers in &numbers {
                            //println!("{}", line_numbers[i]);
                            sub_total += line_numbers[i];
                        }
                        
                        grand_total += sub_total;
                    }
                }

                //println!("{:?}", numbers);
                //println!("{:?}", line_split);
                println!("grand_total = {}", grand_total);
        } else {
                // parse numbers
                let mut line_numbers: Vec<i64> = Vec::new();
                for str_num in line_split {
                    line_numbers.push(str_num.parse::<i64>().unwrap());
                }
                numbers.push(line_numbers);
            }
        }
    }
}

fn part2() {

    println!("!===============================================================================");
    println!("part 2");

    let mut all_chars: Vec<Vec<char>> = Vec::new();
    let mut grand_total: i64 = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {

            if line.chars().nth(0).unwrap() == '*' || line.chars().nth(0).unwrap() == '+' {
                // this is the last line in the input which has the operators

                let line_chars: Vec<char> = line.chars().collect();

                let mut last_char_index = line_chars.len() - 1; // column index which contains the right-most character for the next calculation

                println!("{:?}", line_chars);

                for i in (0..line_chars.len()).rev() {
                    // read characters right-to-left, find the next operator

                    if line_chars[i] != ' ' {
                        // not a space, must be an operator

                        let mut num_strings: Vec<String> = Vec::new();
                        // populate vector of strings which will contain each number for the calculation
                        for _ in 0..=last_char_index-i {
                            num_strings.push(String::new());
                        }

                        // each number is read in top-to-bottom, right-to-left
                        for col in (i..=last_char_index).rev() {
                            // step backwards from last_char_index and parse the numbers in each column
                            for row_chars in &all_chars {
                                if row_chars[col] != ' ' {
                                    num_strings[last_char_index - col].push(row_chars[col]);
                                }
                            }
                        }

                        //println!("{:?}", num_strings);

                        let mut numbers: Vec<i64> = Vec::new();

                        for num_string in num_strings {
                            numbers.push(num_string.parse::<i64>().unwrap());
                        }

                        let mut sub_total: i64;

                        // apply the given operation for this set of numbers
                        if line_chars[i] == '+' {
                            // addition
                            sub_total = 0;
                            for n in numbers {
                                sub_total += n;
                            }
                        } else {
                            // multiplication
                            sub_total = 1;
                            for n in numbers {
                                sub_total *= n;
                            }
                        }

                        grand_total += sub_total;

                        if i == 0 {
                            // break here to avoid underflow on the next line
                            break;
                        }
                        last_char_index = i - 2;
                    }

                }
            } else {
                // numbers
                println!("{:?}", line.chars().collect::<Vec<_>>());
                all_chars.push(line.chars().collect());
            }
        }
    }

    println!("grand_total = {}", grand_total);
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
