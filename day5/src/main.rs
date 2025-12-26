
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line_add_range_to_vec(line: &String, fresh_vec: &mut Vec<(i64,i64)>) {
    let range_ends: Vec< _> = line.split("-").collect();
    let bottom: i64;

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
            fresh_vec.push((bottom, top));
        },
        Err(e) => println!("{}", e),
    }
}

fn i64_is_in_range(i: &i64, bottom: &i64, top: &i64) -> bool {
    return i >= bottom && i <= top;
}

fn merge_ranges(fresh_vec: &Vec<(i64,i64)>, merged_vec: &mut Vec<(i64,i64)>) {
    merged_vec.clear();

    for (bottom, top) in fresh_vec {

        let mut in_range = false;
        for i in 0..merged_vec.len() {
            // extend range
            let (merged_bottom, merged_top) = merged_vec[i];

            if (*bottom == merged_bottom && *top == merged_top) || (*bottom == merged_top && *top == merged_top) || (*bottom == merged_bottom && *top == merged_bottom) {
                in_range = true;
                break;
            }

            //println!("compare {}-{} to {}-{}", *bottom, *top, merged_bottom, merged_top);

            if *bottom > merged_bottom && i64_is_in_range(top, &merged_bottom, &merged_top) {
                in_range = true;
                break;
            }

            if *top < merged_top && i64_is_in_range(bottom, &merged_bottom, &merged_top) {
                in_range = true;
                break;
            }  

            if *bottom < merged_bottom && (i64_is_in_range(top, &merged_bottom, &merged_top) || *top > merged_top) {
                println!("extend bottom {} -> {}", merged_vec[i].0, *bottom);
                merged_vec[i].0 = *bottom;
                in_range = true;
            }

            if *top > merged_top && (i64_is_in_range(bottom, &merged_bottom, &merged_top) || *bottom < merged_bottom) {
                println!("extend top {} -> {}", merged_vec[i].1, *top);
                merged_vec[i].1 = *top;
                in_range = true;
            }

            if in_range {
                break;
            }
        }

        if !in_range {
            merged_vec.push((*bottom, *top));
            //println!("push {}-{}", *bottom, *top);
        }
    }
}

fn _merge_ranges(vec: &mut Vec<(i64,i64)>) {
    let mut merged_vec: Vec<(i64,i64)> = Vec::new();

    // sort by bottom of the range, then merge
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    merge_ranges(&vec, &mut merged_vec);

    *vec = merged_vec.clone();

    // sort by top of the range, then merge
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    merge_ranges(&vec, &mut merged_vec);

    *vec = merged_vec.clone();
}

fn part1() {

    println!("!===============================================================================");
    println!("part 1");

    let mut fresh_vec: Vec<(i64,i64)> = Vec::new();
    let mut parse_ranges = true;
    let mut available_fresh_ingredients = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if line.len() == 0 {
                //println!("blank line");
                parse_ranges = false;
            } else {
                if parse_ranges == true {
                    parse_line_add_range_to_vec(&line, &mut fresh_vec);
                } else {
                    // parse ID and check if ingredient is in a range of fresh ingredient IDs
                    match line.parse::<i64>() {
                        Ok(id) => {
                            for (bottom, top) in &fresh_vec {
                                if id >= *bottom && id <= *top {
                                    available_fresh_ingredients += 1;
                                    println!("{} is available", line);
                                    break;
                                }
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                }
                //println!("{}", line);
            }
        }
    }

    println!("total available = {}", available_fresh_ingredients);
}

fn part2() {
    println!("!===============================================================================");
    println!("part 2");

    let mut fresh_vec: Vec<(i64,i64)> = Vec::new();
    let mut available_fresh_ingredients = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if line.len() == 0 {
                // blank space 

                let mut previous_len = fresh_vec.len();
                _merge_ranges(&mut fresh_vec);

                while previous_len != fresh_vec.len() {
                    previous_len = fresh_vec.len();
                    _merge_ranges(&mut fresh_vec);

                    println!("{} {}", previous_len, fresh_vec.len());
                }

                for (bottom, top) in fresh_vec {
                    available_fresh_ingredients += top - bottom + 1;

                    println!("range {}-{} = {}", bottom, top, top - bottom + 1);
                }
                
                println!("total available = {}", available_fresh_ingredients);
                return;
            } else {
                parse_line_add_range_to_vec(&line, &mut fresh_vec);
                //println!("{}", line);
            }
        }
    }

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
