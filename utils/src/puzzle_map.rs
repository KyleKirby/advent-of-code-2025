use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::env::Args;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{}", self.row, self.col)
    }
}

impl std::ops::Sub<&Coordinates> for &Coordinates {
    type Output = Slope;

    fn sub (self, coord: &Coordinates) -> Self::Output {
        Slope {x: self.col as i32 - coord.col as i32, y: self.row as i32 - coord.row as i32 }
    }

    
}

impl std::ops::Sub<&Slope> for &Coordinates {
    type Output = Coordinates;

    fn sub (self, slope: &Slope) -> Self::Output {
        if self.row as i32 - slope.y < 0 {
            return Coordinates { row: usize::MAX, col: 0 }
        }
        if self.col as i32 - slope.x < 0 {
            return Coordinates { row: 0, col: usize::MAX }
        }
    
        Coordinates { row: (self.row as i32 - slope.y) as usize, col: (self.col as i32 - slope.x) as usize }
    }
}

impl std::ops::Add<&Slope> for &Coordinates {
    type Output = Coordinates;

    fn add (self, slope: &Slope) -> Self::Output {
        if self.row as i32 + slope.y < 0 {
            return Coordinates { row: usize::MAX, col: 0 }
        }
        if self.col as i32 + slope.x < 0 {
            return Coordinates { row: 0, col: usize::MAX }
        }
    
        Coordinates { row: (self.row as i32 + slope.y) as usize, col: (self.col as i32 + slope.x) as usize }
    }
}

impl Coordinates {
    #[inline]
    #[must_use]
    pub fn new(y: usize, x: usize) -> Coordinates {
        Coordinates { row: y, col: x }
    }

    #[allow(dead_code)]
    pub fn up(&self, increment: usize) -> Option<Coordinates> {
        if self.row < increment {
            return None;
        }

        return Some(Coordinates::new(self.row - increment, self.col));
    }

    #[allow(dead_code)]
    pub fn down(&self, increment: usize) -> Option<Coordinates> {
        Some(Coordinates::new(self.row + increment, self.col))
    }

    #[allow(dead_code)]
    pub fn left(&self, increment: usize) -> Option<Coordinates> {
        if self.col < increment {
            return None;
        }

        return Some(Coordinates::new(self.row, self.col - increment));
    }

    #[allow(dead_code)]
    pub fn right(&self, increment: usize) -> Option<Coordinates> {
        Some(Coordinates::new(self.row, self.col + increment))
    }

    // diagonals

    #[allow(dead_code)]
    pub fn up_left(&self, increment: usize) -> Option<Coordinates> {
        if self.row < increment {
            return None;
        }

        if self.col < increment {
            return None;
        }

        return Some(Coordinates::new(self.row - increment, self.col - increment));
    }

    #[allow(dead_code)]
    pub fn up_right(&self, increment: usize) -> Option<Coordinates> {
        if self.row < increment {
            return None;
        }

        return Some(Coordinates::new(self.row - increment, self.col + increment));
    }

    #[allow(dead_code)]
    pub fn down_left(&self, increment: usize) -> Option<Coordinates> {
        if self.col < increment {
            return None;
        }

        return Some(Coordinates::new(self.row + increment, self.col - increment));
    }

    #[allow(dead_code)]
    pub fn down_right(&self, increment: usize) -> Option<Coordinates> {
        return Some(Coordinates::new(self.row + increment, self.col + increment));
    }

}


#[derive(Clone)]
pub struct Slope {
    y: i32,
    x: i32,
}

impl std::ops::Mul<i32> for &Slope {
    type Output = Slope;

    fn mul (self, multiplier: i32) -> Self::Output {
        Slope { x: self.x * multiplier, y: self.y * multiplier }
    }
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{}", self.x, self.y)
    }
}

type PuzzleMapRow = Vec<char>;
type PuzzleMapData = Vec<Vec<char>>;
type PuzzleHashMap = HashMap<char, Vec<Coordinates>>;
type PuzzleSet     = HashSet<Coordinates>;

pub struct PuzzleMap {
    pub area_map: PuzzleMapData,
    paper_rolls_set: PuzzleSet,
    accessible_paper_rolls_set: PuzzleSet,
}

#[allow(dead_code)]
fn insert_map_coords(hash_map: &mut PuzzleHashMap, key: &char, value: &Coordinates) {
    if hash_map.contains_key(key) {
        hash_map.get_mut(key).unwrap().push(Coordinates{row:value.row, col:value.col});
    } else {
        hash_map.insert(*key, vec![]);
        hash_map.get_mut(key).unwrap().push(Coordinates{row:value.row, col:value.col});
    }
}

impl std::ops::Index<usize> for PuzzleMap {
    type Output = PuzzleMapRow;

    fn index(&self, row: usize) -> &PuzzleMapRow {
        &self.area_map[row]
    }
}

impl std::ops::Index<&Coordinates> for PuzzleMap {
    type Output = char;

    fn index(&self, coord: &Coordinates) -> &char {
        &self.area_map[coord.row][coord.col]
    }
}

impl PuzzleMap {

    #[inline]
    #[must_use]
    pub fn new() -> PuzzleMap {
        PuzzleMap { area_map: vec![], paper_rolls_set: PuzzleSet::new(), accessible_paper_rolls_set: PuzzleSet::new()}
    }

    #[allow(dead_code)]
    pub fn get_from_file(&mut self, args: Args) {

        let mut file_name: &str = "example";

        let cmd_args: Vec<String> = args.collect();

        if cmd_args.len() > 1 {
            file_name = &cmd_args[1];
        }
    

        println!("get area map from file {}", file_name);
    
        if let Ok(lines) = read_lines(file_name.to_string()) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                self.area_map.push(line.chars().collect());
            }
        }
    }

    #[allow(dead_code)]
    pub fn coordinates_are_in_bounds(&self, coords: &Coordinates) -> bool {
        return coords.row < self.area_map.len() && coords.col < self.area_map[0].len();
    }


    #[allow(dead_code)]
    pub fn print(&self) {
        // reset console
        print!("\x1B[2J\x1B[1;1H");

        for row in 0..self.area_map.len() {
            for col in 0..self.area_map[0].len() {
                print!("{} ", self.area_map[row][col]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.area_map.len()
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.area_map[0].len()
    }

    #[allow(dead_code)]
    pub fn parse_paper_roll_map(&mut self) {
        for row in 0..self.area_map.len() {
            for col in 0..self.area_map[row].len() {
                if self.area_map[row][col] == '@' {
                    // candidate access point here
                    self.paper_rolls_set.insert(Coordinates{row:row, col:col});
                }
            }
        }
    }

    fn is_paper_roll_at_coordinate(&self, coord: &Coordinates) -> bool {
        return self.paper_rolls_set.contains(coord);
    }

    #[allow(dead_code)]
    pub fn find_accessible_paper_rolls(&mut self) {
        for candidate_coord in &self.paper_rolls_set {
            // search in 8 surrounding directions and count paper rolls. if there are less than 4 rolls of paper adjacent to this coordinate then it is an access point
            let mut paper_roll_count = 0;

            let up: Option<Coordinates> = candidate_coord.up(1);
            if up != None && self.coordinates_are_in_bounds(&up.unwrap()) && self.is_paper_roll_at_coordinate(&up.unwrap()) {
                paper_roll_count += 1;
            }

            let down: Option<Coordinates> = candidate_coord.down(1);
            if down != None && self.coordinates_are_in_bounds(&down.unwrap()) && self.is_paper_roll_at_coordinate(&down.unwrap()) {
                paper_roll_count += 1;
            }

            let left: Option<Coordinates> = candidate_coord.left(1);
            if left != None && self.coordinates_are_in_bounds(&left.unwrap()) && self.is_paper_roll_at_coordinate(&left.unwrap()) {
                paper_roll_count += 1;
            }

            let right: Option<Coordinates> = candidate_coord.right(1);
            if right != None && self.coordinates_are_in_bounds(&right.unwrap()) && self.is_paper_roll_at_coordinate(&right.unwrap()) {
                paper_roll_count += 1;
            }

            let up_left: Option<Coordinates> = candidate_coord.up_left(1);
            if up_left != None && self.coordinates_are_in_bounds(&up_left.unwrap()) && self.is_paper_roll_at_coordinate(&up_left.unwrap()) {
                paper_roll_count += 1;
            }

            let up_right: Option<Coordinates> = candidate_coord.up_right(1);
            if up_right != None && self.coordinates_are_in_bounds(&up_right.unwrap()) && self.is_paper_roll_at_coordinate(&up_right.unwrap()) {
                paper_roll_count += 1;
            }

            let down_left: Option<Coordinates> = candidate_coord.down_left(1);
            if down_left != None && self.coordinates_are_in_bounds(&down_left.unwrap()) && self.is_paper_roll_at_coordinate(&down_left.unwrap()) {
                paper_roll_count += 1;
            }

            let down_right: Option<Coordinates> = candidate_coord.down_right(1);
            if down_right != None && self.coordinates_are_in_bounds(&down_right.unwrap()) && self.is_paper_roll_at_coordinate(&down_right.unwrap()) {
                paper_roll_count += 1;
            }

            if paper_roll_count < 4 {
                self.accessible_paper_rolls_set.insert(*candidate_coord);
            }

            //println!("{}: {}", candidate_coord, paper_roll_count);


        }
    }

    #[allow(dead_code)]
    pub fn get_forklift_access_point_count(&self) -> usize {
        //println!("{:?}", self.accessible_paper_rolls_set);
        return self.accessible_paper_rolls_set.len();
    }

    fn remove_paper_rolls(&mut self) -> usize {
        for paper_roll_coord in &self.accessible_paper_rolls_set {
            self.area_map[paper_roll_coord.row][paper_roll_coord.col] = 'X';
        }

        let removed_rolls_count = self.accessible_paper_rolls_set.len();

        self.paper_rolls_set = PuzzleSet::new();
        self.accessible_paper_rolls_set = PuzzleSet::new();

        return removed_rolls_count;
    }

    #[allow(dead_code)]
    pub fn find_all_accessible_paper_rolls_with_removal(&mut self) -> usize {
        self.find_accessible_paper_rolls();

        let mut removed_rolls_count: usize = 0;

        while self.get_forklift_access_point_count() > 0 {
            // remove paper rolls then parse the map again and see if there are still accessible paper rolls
            removed_rolls_count += self.remove_paper_rolls();
            self.parse_paper_roll_map();
            self.find_accessible_paper_rolls();
        }

        return removed_rolls_count;
    }

}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
