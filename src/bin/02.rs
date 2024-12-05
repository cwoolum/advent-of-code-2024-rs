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
    println!("\n=== Part 2 ===");

    fn handle_increasing_row_with_safety(values: &[i32], use_safety: bool) -> Result<()> {
        // Note: Result is from anyhow
        if values.is_empty() {
            return Ok(());
        }

        for n in 1..values.len() {
            if values[n - 1] >= values[n] {
                if use_safety {
                    let mut new_vec = values.to_vec();
                    new_vec.remove(n);
                    return handle_increasing_row_with_safety(&new_vec, false);
                }

                return Err(anyhow!("Row is not increasing"));
            }

            let diff = (values[n] - values[n - 1]).abs();
            if diff < 1 || diff > 3 {
                if use_safety {
                    let mut new_vec = values.to_vec();
                    new_vec.remove(n);
                    return handle_increasing_row_with_safety(&new_vec, false);
                }

                return Err(anyhow!("Value is unsafe"));
            }
        }

        Ok(())
    }

    fn handle_decreasing_row_with_safety(values: &[i32], use_safety: bool) -> Result<()> {
        if values.is_empty() {
            return Ok(());
        }

        for n in 1..(values.len()) {
            if values[n - 1] < values[n] {
                if use_safety {
                    let mut new_vec = values.to_vec();
                    new_vec.remove(n);
                    return handle_decreasing_row_with_safety(&new_vec, false);
                }

                return Err(anyhow!("Row is not decreasing"));
            }

            let diff = (values[n - 1] - values[n]).abs();

            if diff > 3 || diff < 1 {
                if use_safety {
                    let mut new_vec = values.to_vec();
                    new_vec.remove(n);
                    return handle_decreasing_row_with_safety(&new_vec, false);
                }

                return Err(anyhow!("Value is unsafe"));
            }
        }

        Ok(())
    }

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
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
                match handle_decreasing_row_with_safety(&row, true) {
                    Ok(_) => overall_total += 1,
                    Err(err) => println!("{} {:?}", err, row),
                }
            } else {
                match handle_increasing_row_with_safety(&row, true) {
                    Ok(_) => overall_total += 1,
                    Err(err) => println!("{} {:?}", err, row),
                }
            }
        }

        Ok(overall_total)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
