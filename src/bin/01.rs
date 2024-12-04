use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let (first_column, second_column) = reader
            .lines()
            .flatten()
            // Each line is two numbers tab delimited
            //.map(|s| s.chars().join("").split_whitespace())
            .map(|s| {
                let parts: Vec<String> = s
                    .split_whitespace()
                    .map(|s| s.to_string()) // Convert &str to String
                    .collect();

                (parts[0].clone(), parts[1].clone()) // This creates a tuple of the two parts
            })
            .fold(
                (Vec::new(), Vec::new()),
                |(mut first_column, mut second_column), (first, second)| {
                    first_column.push(first.parse::<i32>().unwrap());
                    second_column.push(second.parse::<i32>().unwrap());
                    (first_column, second_column)
                },
            );

        let mut first_column_sorted = first_column.to_vec();
        first_column_sorted.sort();

        let mut second_column_sorted = second_column.to_vec();
        second_column_sorted.sort();

        let mut overall_total = 0;
        for n in 0..(second_column_sorted.len()) {
            let total = first_column_sorted[n] - second_column_sorted[n];
            overall_total += total.abs();
        }

        Ok(overall_total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
