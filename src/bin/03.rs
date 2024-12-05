use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");

    // fn part1<R: BufRead>(reader: R) -> Result<u64> {
    //     let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    //     let rows = reader.lines().flatten();

    //     let mut overall_total = 0;

    //     for row in rows {
    //         for (_, [val1, val2]) in re.captures_iter(&row).map(|c| c.extract()) {
    //             overall_total += val1.parse::<u64>()? * val2.parse::<u64>()?;
    //         }
    //     }

    //     Ok(overall_total)
    // }

    // // TODO: Set the expected answer for the test input
    // assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let re = Regex::new(r"mul\((\d*),(\d*)\)|don't\(\)|do\(\)").unwrap();

        let rows = reader.lines().flatten();

        let mut overall_total = 0;
        let mut enabled = true;

        for row in rows {
            for capture in re.captures_iter(&row) {
                if capture.get(0).unwrap().as_str() == "don't()" {
                    enabled = false;
                } else if capture.get(0).unwrap().as_str() == "do()" {
                    enabled = true;
                } else if enabled {
                    // This must be a mul() capture
                    let val1 = capture.get(1).unwrap().as_str();
                    let val2 = capture.get(2).unwrap().as_str();
                    let product = val1.parse::<u64>()? * val2.parse::<u64>()?;
                    overall_total += product;
                }
            }
        }

        Ok(overall_total)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
