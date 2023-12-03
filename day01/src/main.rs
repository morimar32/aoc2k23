use regex::{Match, Regex};
use std::fs;

fn main() {
    //part1();
    part2();
    //validate();
}

fn validate() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut total = 0;
    for line in lines {
        let orig = get_digits_or_names(line);
        let newline = sanitize_line(&line);
        let simple = get_digits(&newline);
        if orig != simple {
            println!("[{}] - {} vs {}", &line, &orig, &simple);
        }
    }
}
fn part1() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut total = 0;
    for line in lines {
        let val = get_digits(&line);
        total += val
    }
    println!("{}", total);
}

fn part2() {
    let data = fs::read_to_string("input.txt").unwrap();
    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut total = 0;
    for line in lines {
        let val = get_digits_or_names(&line);
        println!("[{}] -> {}", &line, &val);
        total += val
    }
    println!("{}", total);
}

pub fn sanitize_line(line: &str) -> String {
    let newline = line
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    return newline;
}
pub fn get_digits(line: &str) -> (u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for letter in line.chars() {
        if letter.is_numeric() {
            first = letter.to_digit(10).unwrap();
            break;
        }
    }
    for letter in line.chars().rev() {
        if letter.is_numeric() {
            last = letter.to_digit(10).unwrap();
            break;
        }
    }
    //println!("{}{}", first, last);
    return (first * 10) + last;
}

pub fn get_digits_or_names(line: &str) -> (u32) {
    let mut first: u32 = 0;
    let re =
        Regex::new("(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let revre =
        Regex::new("(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|9|8|7|6|5|4|3|2|1)").unwrap();

    let backwards: String = line.chars().rev().collect();
    let m = re.find_iter(line).collect::<Vec<Match>>();
    //let r: Vec<Match> = re.find_iter(line).collect::<Vec<Match>>().into_iter().rev().collect();
    let r = revre.find_iter(&backwards).collect::<Vec<Match>>();

    let first = get_first_from_vec(&m);
    let last = get_first_from_vec(&r);

    //println!("{}{}", first, last);
    return (first * 10) + last;
}

fn get_first_from_vec(matches: &Vec<Match>) -> (u32) {
    let mut first: u32 = 0;
    for i in matches {
        let s = i.as_str();
        if s.len() == 1 {
            first = s.chars().nth(0).unwrap().to_digit(10).unwrap();
        } else {
            match s {
                "one" | "eno" => first = 1,
                "two" | "owt" => first = 2,
                "three" | "eerht" => first = 3,
                "four" | "ruof" => first = 4,
                "five" | "evif" => first = 5,
                "six" | "xis" => first = 6,
                "seven" | "neves" => first = 7,
                "eight" | "thgie" => first = 8,
                "nine" | "enin" => first = 9,
                _ => println!("something went wrong"),
            }
        }
        //println!("----{} -> {}", i.as_str(), first);
        break;
    }
    return first;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits_or_names() {
        assert_eq!(get_digits_or_names("two1nine"), 29);
        assert_eq!(get_digits_or_names("eightwothree"), 83);
        assert_eq!(get_digits_or_names("abcone2threexyz"), 13);
        assert_eq!(get_digits_or_names("xtwone3four"), 24);
        assert_eq!(get_digits_or_names("4nineeightseven2"), 42);
        assert_eq!(get_digits_or_names("zoneight234"), 14);
        assert_eq!(get_digits_or_names("7pqrstsixteen"), 76);
        assert_eq!(get_digits_or_names("bobonebob"), 11);
        assert_eq!(get_digits_or_names("98two"), 92);
        assert_eq!(get_digits_or_names("vjchzt7btthreesix1tcngpbtzsfmvsx"), 71);

        assert_eq!(get_digits_or_names("12"), 12);
        assert_eq!(get_digits_or_names("1two"), 12);
        assert_eq!(get_digits_or_names("one2"), 12);
        assert_eq!(get_digits_or_names("onetwo"), 12);

        assert_eq!(get_digits_or_names("142"), 12);
        assert_eq!(get_digits_or_names("14two"), 12);
        assert_eq!(get_digits_or_names("one42"), 12);
        assert_eq!(get_digits_or_names("one4two"), 12);

        assert_eq!(get_digits_or_names("1four2"), 12);
        assert_eq!(get_digits_or_names("1fourtwo"), 12);
        assert_eq!(get_digits_or_names("onefour2"), 12);
        assert_eq!(get_digits_or_names("onefourtwo"), 12);

        assert_eq!(get_digits_or_names("bla12"), 12);
        assert_eq!(get_digits_or_names("bla1two"), 12);
        assert_eq!(get_digits_or_names("blaone2"), 12);
        assert_eq!(get_digits_or_names("blaonetwo"), 12);
        assert_eq!(get_digits_or_names("bla142"), 12);
        assert_eq!(get_digits_or_names("bla14two"), 12);
        assert_eq!(get_digits_or_names("blaone42"), 12);
        assert_eq!(get_digits_or_names("blaone4two"), 12);
        assert_eq!(get_digits_or_names("bla1four2"), 12);
        assert_eq!(get_digits_or_names("bla1fourtwo"), 12);
        assert_eq!(get_digits_or_names("blaonefour2"), 12);
        assert_eq!(get_digits_or_names("blaonefourtwo"), 12);

        assert_eq!(get_digits_or_names("12bla"), 12);
        assert_eq!(get_digits_or_names("1twobla"), 12);
        assert_eq!(get_digits_or_names("one2bla"), 12);
        assert_eq!(get_digits_or_names("onetwobla"), 12);
        assert_eq!(get_digits_or_names("142bla"), 12);
        assert_eq!(get_digits_or_names("14twobla"), 12);
        assert_eq!(get_digits_or_names("one42bla"), 12);
        assert_eq!(get_digits_or_names("one4twobla"), 12);
        assert_eq!(get_digits_or_names("1four2bla"), 12);
        assert_eq!(get_digits_or_names("1fourtwobla"), 12);
        assert_eq!(get_digits_or_names("onefour2bla"), 12);
        assert_eq!(get_digits_or_names("onefourtwobla"), 12);

        assert_eq!(get_digits_or_names("nineskg12rlgmpbbdlxmk9twonel"), 91);
    }

    #[test]
    fn test_get_digits_or_names_combined_start() {
        assert_eq!(get_digits_or_names("rrtwonedzthree6four"), 24);
    }

    #[test]
    fn test_get_digits_or_names_combined_end() {
        assert_eq!(get_digits_or_names("rrtwonedzthree6fourtwonezed"), 21);
    }
}
