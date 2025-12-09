
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/* return true if the number is comprised of one repeating sequence */
fn has_repeating_sequence_i64(n: i64) -> bool {
    let num_string = n.to_string();
    let num_string_len = num_string.len();
    //println!("check {}", num_string);
        
    let substring_end = num_string_len/2;
    let sub_num_string = &num_string[0..substring_end];
    //println!("consider {}", sub_num_string);

    // find repeating occurences of this substring
    let possible_match = &num_string[substring_end..num_string_len];
    //println!("match with {}", possible_match);

    if sub_num_string == possible_match {
        //println!("found repeating sequence {}", sub_num_string);
        return true;
    }
        
    return false;
}

fn sum_part1_invalid_ids_in_range(bottom: i64, top: i64) -> i64 {
    let mut sum: i64 = 0;

    for n in bottom..=top {
        if has_repeating_sequence_i64(n) {
            //println!("invalid ID {}", n);
            sum += n;
        }
    }

    return sum;
}

// sum invalid IDs (IDs comprised of one repeating sequence)
fn part1() {
    let mut invalid_id_sum: i64 = 0;

    println!("!===============================================================================");
    println!("part 1");

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{}", line);

            // split ID ranges

            let ranges: Vec< _> = line.split(",").collect();

            for range in ranges {
                //println!("{}", range);

                let bottom: i64;

                let range_ends: Vec< _> = range.split("-").collect();

                match range_ends[0].to_string().parse::<i64>() {
                    Ok(n) => {
                        bottom = n;
                    },
                    Err(e) =>  {
                        println!("{}", e);
                        return;
                    },
                }

                match range_ends[1].to_string().parse::<i64>() {
                    Ok(top) => {
                        invalid_id_sum += sum_part1_invalid_ids_in_range(bottom, top);
                        //println!("invalid_id_sum {}", invalid_id_sum)
                    },
                    Err(e) => println!("{}", e),
                }
            }


            
        }
    }

    println!("invalid_id_sum {}", invalid_id_sum);

}

fn string_is_repeating_sequence(string: &str, sequence: &str) -> bool {
    let possible_reoccurences: usize = string.len() / sequence.len() - 1;
    //println!("possible_reoccurences {}", possible_reoccurences);

    if string.len() % sequence.len() != 0 {
        return false;
    }

    for n in 1..=possible_reoccurences {
        let next_sequence: &str = &string[sequence.len()*n..sequence.len()+sequence.len()*n];
        //println!("compare {} {}", sequence, next_sequence);
        if sequence != next_sequence {
            return false;
        }
    }
    return true;
}

/* return true if the number is comprised of repeating sequence(s) of digits */
fn has_repeating_sequences_i64(n: i64) -> bool {
    let num_string = n.to_string();
    let num_string_len = num_string.len();
    //println!("check {}", num_string);
    for substring_end in 1..=num_string_len/2 {
        let sub_num_string = &num_string[0..substring_end];

        if string_is_repeating_sequence(&num_string, &sub_num_string) {
            return true;
        }
    }
    return false;
}

fn sum_part2_invalid_ids_in_range(bottom: i64, top: i64) -> i64 {
    let mut sum: i64 = 0;

    for n in bottom..=top {
        if has_repeating_sequences_i64(n) {
            //println!("invalid ID {}", n);
            sum += n;
        }
    }

    return sum;
}

// sum invalid IDs (IDs comprised of repeating sequences)
fn part2() {
    let mut invalid_id_sum: i64 = 0;

    println!("!===============================================================================");
    println!("part 2");

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //println!("{}", line);

            // split ID ranges

            let ranges: Vec< _> = line.split(",").collect();

            for range in ranges {
                //println!("{}", range);

                let bottom: i64;

                let range_ends: Vec< _> = range.split("-").collect();

                match range_ends[0].to_string().parse::<i64>() {
                    Ok(n) => {
                        bottom = n;
                    },
                    Err(e) =>  {
                        println!("{}", e);
                        return;
                    },
                }

                match range_ends[1].to_string().parse::<i64>() {
                    Ok(top) => {
                        invalid_id_sum += sum_part2_invalid_ids_in_range(bottom, top);
                        //println!("invalid_id_sum {}", invalid_id_sum)
                    },
                    Err(e) => println!("{}", e),
                }
            }


            
        }
    }

    println!("invalid_id_sum {}", invalid_id_sum);

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
