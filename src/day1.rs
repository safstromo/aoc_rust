use std::fs::File;
use std::io::{BufReader, Error};
use std::io::prelude::*;


fn day1_part1() -> std::io::Result<i32> {
    let elfs = all_elves()?;

    Ok(elfs.into_iter().max().unwrap())
}

fn day1_part2() -> std::io::Result<i32> {
    let mut elfs = all_elves().unwrap();
    elfs.sort();
    elfs.reverse();
    let mut result = 0;
    for i in 0..3 {
        result += elfs.get(i).unwrap();
    }
    Ok(result)
}

fn all_elves() -> Result<Vec<i32>, Error> {
    let file = File::open("src/input/day1.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;
    let elf: Vec<&str> = input.split("\n\n")
        .collect();

    let mut elfs: Vec<i32> = vec![];

    for x in elf {
        elfs.push(add(x));
    }

    Ok(elfs)
}

fn add(string: &str) -> i32 {
    let numbers_as_strings: Vec<&str> = string.lines().collect();
    let mut result = 0;
    for x in numbers_as_strings {
        result += x.parse::<i32>().unwrap()
    }
    result
}


mod tests {
    use crate::day1::{day1_part1, day1_part2};

    #[test]
    fn day1_test() {
        assert_eq!(day1_part1().unwrap(), 75501)
    }

    #[test]
    fn day1_part2_test() {
        assert_eq!(day1_part2().unwrap(), 215594)
    }
}
