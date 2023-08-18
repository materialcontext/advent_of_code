use std::cmp::Ordering;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::collections::{HashSet, HashMap};

/* ============ CONVENIENCE FUNCTIONS ============ */
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn xmas() { // DONE
    let bars: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas, my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            println!("{}", bars[j]);
        }
    }
}

fn aoc_01() { // DONE
    // calaculate the toal sums of calories carried by each elf
    // values are line-by-line with a blank line separating elves
    // records are kept in a records struct that implements an update_records method
    // there's probably a better way to do this with an array

    struct Records {
        first: u32,
        second: u32,
        third: u32
    }

    impl Records {
        fn update_records(&mut self, test_val: u32) {
            match &test_val.cmp(&self.first) {
                Ordering::Greater | Ordering::Equal => {
                    self.third = self.second;
                    self.second = self.first;
                    self.first = test_val;
                },
                Ordering::Less => {
                    self.update_second(test_val)
                }
            }
        }

        fn update_second(&mut self, test_val: u32) {
            match &test_val.cmp(&self.second) {
                Ordering::Greater | Ordering::Equal => {
                    self.third = self.second;
                    self.second = test_val;
                },
                Ordering::Less => {
                    self.update_third(test_val)
                }
            }
        }

        fn update_third(&mut self, test_val: u32) {
            match &test_val.cmp(&self.third) {
                Ordering::Greater | Ordering::Equal => {
                    self.third = test_val
                },
                _ => ()
            }
        }
    }

    let mut records: Records = Records{first:0, second:0, third:0};

    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_01_input.txt") {
        let mut curr_total = 0;

        for line in lines {
            if let Ok(ln) = line {
                if ln.is_empty() {
                    records.update_records(curr_total);
                    curr_total = 0;
                } else {
                    let val = ln.trim().parse::<u32>().expect("Not a valid number");
                    curr_total += val;
                }
            }
        }
        println!("The elf with the most calories has {} calories.", records.first);
        println!("The top three elves are carrying a total of {} calories.", records.first + records.second + records.third);
    }

}

fn aoc_02() { // DONE
    // calculate the final score from of a list of rock paper scissors matches 
    // (rock = 1, paper = 2, scissors =3); (loss = 0, draw = 3, win = 6)
    let mut total: i32 = 0;

    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_02_input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                // final value of any given match as described by the original prompt
                match ln.trim() {
                    "A X" => {total += 3; continue;},
                    "A Y" => {total += 4; continue;},
                    "A Z" => {total += 8; continue;},
                    "B X" => {total += 1; continue;},
                    "B Y" => {total += 5; continue;},
                    "B Z" => {total += 9; continue;},
                    "C X" => {total += 2; continue;},
                    "C Y" => {total += 6; continue;},
                    "C Z" => {total += 7; continue;},
                    _ => {print!("Uh oh! Invalid match data."); break;}
                }
            }
        }
    }
    println!("{}", total)
}

fn aoc_04() { //DONE
    // given two ranges, count all of the subsets and partial overlaps
    let mut count_subsets: i32 = 0;
    let mut count_overlap: i32 = 0;

    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_04_input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                // split the input string into a vector of rnage markers [start, end, start, end] and change them into i32
                let zones: Vec<_> = ln.split(|c| c == ',' || c == '-').map(|c| c.parse::<i32>().unwrap()).collect();
                // create two hashsets for the two ranges
                let zone_a:HashSet<i32> = HashSet::from_iter(zones[0]..zones[1]+1);
                let zone_b:HashSet<i32> = HashSet::from_iter(zones[2]..zones[3]+1);
                if zone_a.is_subset(&zone_b) || zone_b.is_subset(&zone_a) {
                    // if one range is a subset of the other, increment the subset counter
                    count_subsets += 1;
                }
                // create a union of the two hashsets
                // if it is smaller than the two counted separately, increment the overlap counter
                let mut union = zone_a.clone();
                union.extend(&zone_b);
                if union.len() < (zone_a.len() + zone_b.len()) {
                    count_overlap += 1;
                }

            }
        }
    }
    println!("{}, {}", count_subsets, count_overlap);
}

fn aoc_05() { // PART 1 DONE
    struct Ship {
        cargo: Vec<Vec<u8>>,
    }

    impl Ship {
        fn crane(&mut self, source: usize, destination: usize, depth: usize) {
            let items = self.unpack(source, depth);
            for item in items {
                self.pack(destination, item);
            }
        }

        fn pack(&mut self, destination: usize, item: u8) {
            self.cargo[destination].push(item);
        }

        fn unpack(&mut self, source: usize, depth: usize) -> Vec<u8> {
            let end = self.cargo[source].len();
            self.cargo[source].drain((end - depth)..end).collect()
        }

        fn reverse(&mut self) {
            for i in 0..self.cargo.len() {
                self.cargo[i].reverse();
            }
        }
    }
    fn parse_move(line: &str) -> Vec<usize> {
        // map all the numbers in the line to an array of move orders as usize
        let r = Regex::new(r"(\d+)").unwrap();
        let matches: Vec<usize>= r.find_iter(line)
            .map(|c| c.as_str().parse::<usize>().unwrap()).collect();
        matches
    }

    // create a ship with an empty cargo hold and 9 stacks
    let width: usize = 9;
    let mut ship = Ship {cargo: vec![Vec::<u8>::new(); width]};

    // read the file
    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_05_input.txt") {
        // track the current line so we know when to switch from packing to repacking
        let mut curr_line = 1;
        for line in lines {
            if let Ok(ln) = line {
                // inital sort
                match &curr_line.cmp(&width) {
                    Ordering::Less => {
                        //initialize the line position & convert the line to a bytes array
                        let mut i: usize = 1;
                        let row = ln.as_bytes();
                        // each stack is 4 chars long with spaces so a 9 stack cargo has 36 width
                        while i < 36 {
                            // calculate the destination stack from the position of the element in the line for each origin stack
                            let destination: usize = (i + 3) / 4;
                            // extract the element as bytes array
                            let item = row[i];
                            if !(item == 32) {
                                // if the item is no a blank character, pack it
                                ship.pack(destination - 1, item);
                            }
                            i += 4;
                        }
                    },
                    Ordering::Equal => {
                        // the stacks need to be reversed to match the source file for processing
                        ship.reverse();
                    },
                    _ => ()
                }
                // start moving
                match &curr_line.cmp(&10) {
                    Ordering::Greater => {
                        // get the move orders and then repack
                        let moves = parse_move(&ln);
                        ship.crane(moves[1] - 1, moves[2] - 1, moves[0])
                    },
                    _ => ()   
                }
            }
            // go to next line
            curr_line += 1;
        }
    }
    let mut output = Vec::<char>::new();
    for stack in ship.cargo {
        let item = stack[stack.len() - 1];
        output.push(item as char);
    }
    println!("{:?}", output)
}

fn aoc_06() {
    fn unique(input: &[u8], size: usize) -> bool{
        let mut sorted = input.to_vec();
        sorted.sort();
        for i in 0..size - 1 {
            if sorted[i] == sorted[i + 1] { return false }
        }
        true
    }

    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_06_input.txt") {
        for line in lines {
            let ln = line.unwrap();
            let chars = ln.as_bytes();
            let mut found = false;
            for i in 0..chars.len() {
                if unique(&chars[i..(i+4)], 4) && found == false {
                    println!("{} {} {} {} {}", i+4, chars[i] as char, chars[i+1] as char, chars[i+2] as char, chars[i+3] as char);
                    found = true;
                }
                if unique(&chars[i..(i+14)], 14) {
                    println!("{}", i+14);
                    break;
                }
            }

        }
    }
}

fn aoc_07() {
    let mut totals: Vec<u32> = Vec::new();
    let mut stack: Vec<u32> = Vec::new();
    stack.push(0);

    let mut hundo = 0;

    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_07_input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                let cd = Regex::new(r"\$ cd ([\w\.]+)").unwrap().captures(&ln);
                match cd {
                    None => {},
                    Some(_) => {
                        let dir = cd.unwrap().get(1).unwrap().as_str();
                        if dir == ".." {
                            // Exit current directory.  Save its total size.
                            let total = stack.pop().unwrap();
                            totals.push(total);
                            // Add that total size to its parent (if any)
                            if let Some(top) = stack.last_mut() {
                                *top += total;
                            }
                            continue;
                        } else {
                            // Entering a new directory; don't care about the name.
                            // Initialize the size (so far) to 0.
                            stack.push(0);
                            continue;
                        }
                    }
                }
                let word = ln.split(' ').next().unwrap();
                if let Ok(v) = word.parse::<u32>() {
                    // Add the size of this file to the current directory
                    *stack.last_mut().unwrap() += v;
                }
            }
        }
    }

    // Pop any directories still on the stack
    while let Some(v) = stack.pop() {
        // Add that total size to its parent (if any)
        if let Some(top) = stack.last_mut() {
            *top += v;
        }

        totals.push(v);
    }

    for i in &totals {
        if i < &100000 {
            hundo += i
        }
    }
    println!("{:?}", totals);
    println!("{}", hundo);

    let needed = 30000000 - (70000000 - totals.last().unwrap());
    let mut to_delete: Vec<&u32> = totals.iter().filter(|val| val >= &&needed).collect();
    to_delete.sort();
    let out = to_delete.first().unwrap();

    println!("{} {}", needed, out);
    

}

fn main() {
    // xmas();
    // aoc_01();
    // aoc_02();
    // aoc_04();
    // aoc_05();
    // aoc_06();
    aoc_07();
}
