use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn handle_increasing_row(values: &[i32]) -> Result<()> {
        // Note: Result is from anyhow
        if values.is_empty() {
            return Ok(());
        }

        for n in 1..values.len() {
            if values[n - 1] >= values[n] {
                return Err(anyhow!("Row is not increasing")); // Use anyhow! macro
            }

            let diff = (values[n] - values[n - 1]).abs();
            if diff < 1 || diff > 3 {
                return Err(anyhow!("Value is unsafe")); // Use anyhow! macro
            }
        }

        Ok(())
    }

    fn handle_decreasing_row(values: &[i32]) -> Result<()> {
        if values.is_empty() {
            return Ok(());
        }

        for n in 1..(values.len()) {
            if values[n - 1] < values[n] {
                return Err(anyhow!("Row is not decreasing"));
            }

            let diff = (values[n - 1] - values[n]).abs();

            if diff > 3 || diff < 1 {
                return Err(anyhow!("Value is unsafe"));
            }
        }

        Ok(())
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let rows = reader.lines().flatten().map(|s| {
            let parts: Vec<i32> = s
                .split_whitespace()
                .map(|s| s.to_string().parse::<i32>().unwrap()) // Convert &str to String
                .collect();

            parts
        });

        let mut overall_total = 0;

        for row in rows {
            if row[0] > row[1] {
                match handle_decreasing_row(&row) {
                    Ok(_) => overall_total += 1,
                    Err(err) => println!("{} {:?}", err, row),
                }
            } else {
                match handle_increasing_row(&row) {
                    Ok(_) => overall_total += 1,
                    Err(err) => println!("{} {:?}", err, row),
                }
            }
        }

        Ok(overall_total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");

    // fn part2<R: BufRead>(reader: R) -> Result<u128> {
    //     let (first_column, second_column) = reader
    //         .lines()
    //         .flatten()
    //         // Each line is two numbers tab delimited
    //         //.map(|s| s.chars().join("").split_whitespace())
    //         .map(|s| {
    //             let parts: Vec<String> = s
    //                 .split_whitespace()
    //                 .map(|s| s.to_string()) // Convert &str to String
    //                 .collect();

    //             (parts[0].clone(), parts[1].clone()) // This creates a tuple of the two parts
    //         })
    //         .fold(
    //             (Vec::new(), Vec::new()),
    //             |(mut first_column, mut second_column), (first, second)| {
    //                 first_column.push(first.parse::<i32>().unwrap());
    //                 second_column.push(second.parse::<i32>().unwrap());
    //                 (first_column, second_column)
    //             },
    //         );

    //     let mut overall_total: u128 = 0;
    //     let mut second_column_mapped = HashMap::new();

    //     for ele in second_column {
    //         if !second_column_mapped.contains_key(&ele) {
    //             second_column_mapped.insert(ele, 0);
    //         }

    //         let value = second_column_mapped.get(&ele).unwrap();

    //         second_column_mapped.insert(ele, value + 1);
    //     }

    //     for ele in first_column {
    //         if second_column_mapped.contains_key(&ele) {
    //             let element_count = second_column_mapped.get(&ele).unwrap();

    //             if *element_count > 0 {
    //                 overall_total +=
    //                     u128::try_from(ele).unwrap() * u128::try_from(*element_count).unwrap();
    //             }
    //         }
    //     }

    //     Ok(overall_total)
    // }

    // assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    // endregion

    Ok(())
}
