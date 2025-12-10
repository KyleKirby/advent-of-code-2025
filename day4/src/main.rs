
use std::env;

#[path = "../../utils/src/puzzle_map.rs"] mod puzzle_map;
use puzzle_map::PuzzleMap;
use puzzle_map::Coordinates;

fn part1() {

    println!("!===============================================================================");
    println!("part 1");

    let mut area_map: PuzzleMap = PuzzleMap::new();

    area_map.get_from_file(env::args());

    area_map.print();

    area_map.parse_paper_roll_map();

    area_map.find_accessible_paper_rolls();

    println!("Accessible paper rolls: {}", area_map.get_forklift_access_point_count());
}

fn part2() {

    println!("!===============================================================================");
    println!("part 2");

    let mut area_map: PuzzleMap = PuzzleMap::new();

    area_map.get_from_file(env::args());

    area_map.print();

    area_map.parse_paper_roll_map();
    
    println!("Removed paper rolls: {}", area_map.find_all_accessible_paper_rolls_with_removal());
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    //part1();
    part2();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
