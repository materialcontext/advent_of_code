use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                    "A X" => {total += 4; println!("{} {}", ln, total ); continue;},
                    "A Y" => {total += 8; println!("{} {}", ln, total ); continue;},
                    "A Z" => {total += 3; println!("{} {}", ln, total ); continue;},
                    "B X" => {total += 1; println!("{} {}", ln, total ); continue;},
                    "B Y" => {total += 5; println!("{} {}", ln, total ); continue;},
                    "B Z" => {total += 9; println!("{} {}", ln, total ); continue;},
                    "C X" => {total += 7; println!("{} {}", ln, total ); continue;},
                    "C Y" => {total += 2; println!("{} {}", ln, total ); continue;},
                    "C Z" => {total += 6; println!("{} {}", ln, total ); continue;},
                    _ => {print!("Uh oh! Invalid match data."); break;}
                }
            }
        }
        println!("{}", total)
    }
}

fn aoc_03() { // DONE
    fn get_char_val(char: char) -> u32 {
        if char.is_ascii_lowercase() { return char as u32 - 96 };
        if char.is_ascii_uppercase() { return char as u32 - 38 };
        panic!("This is not a valid ascii character!")
    }

    let mut result: u32 = 0;
    
    if let Ok(lines) = read_lines("D:\\rust\\apps\\advent_of_code\\src\\aoc_2022_03_input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                let half_len = ln.len()/2;
                let mut char_vec_a: Vec<char> = ln[..half_len].chars().collect(); // these work
                let char_vec_b: Vec<char> = ln[half_len..].chars().collect(); // these work

                'outer: while char_vec_a.len() > 0 {
                    let char_a = char_vec_a.pop().unwrap();
                    for i in 0..half_len {
                        let char_b = char_vec_b[i];
                        if char_a == char_b { result += get_char_val(char_a); break 'outer; }
                    }
                }
            }
        }
    }
    println!("The total value of all incorrect characters is {}.", result);
}
fn main() {
    // xmas();
    aoc_01();
    // aoc_02();
    // aoc_03();
}
