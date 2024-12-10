use crate::puzzle_utils::read_puzzle::read_puzzle;
use anyhow::{anyhow, bail, Context, Result};

pub fn run_puzzle_1() -> Result<String> {
    let (mut left, mut right) = parse_input(&read_puzzle("puzzles/day1.txt")?)?;
    run_algorithm_1(&mut left, &mut right).map(|i| i.to_string())
}

pub fn run_puzzle_2() -> Result<String> {
    let (mut left, mut right) = parse_input(&read_puzzle("puzzles/day1.txt")?)?;
    run_algorithm_2(&mut left, &mut right).map(|i| i.to_string())
}

pub fn parse_input(input: &str) -> Result<(Vec<isize>, Vec<isize>)> {
    let results = input
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            let cols = line
                .split_whitespace()
                .enumerate()
                .map(|(record_no, record)| {
                    record.parse::<isize>().with_context(|| {
                        format!(
                            "Parse error at line {}, '{}', invalid record {}",
                            line_no + 1,
                            line,
                            record_no + 1
                        )
                    })
                })
                .collect::<Result<Vec<isize>>>()?;

            cols.get(0)
                .cloned()
                .zip(cols.get(1).cloned())
                .ok_or_else(|| anyhow!("ParseError on line {}, length not 2", line_no + 1))
        })
        .collect::<Result<Vec<(isize, isize)>>>()?;
    Ok(results.into_iter().unzip())
}

fn run_algorithm_1(left: &mut Vec<isize>, right: &mut Vec<isize>) -> Result<isize> {
    if left.len() != right.len() {
        bail!("Both collections must be the same length.")
    }
    if left.is_empty() {
        bail!("left or right may not be empty")
    }
    left.sort();
    right.sort();
    Ok(left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(*b) as isize)
        .sum())
}

fn run_algorithm_2(left: &Vec<isize>, right: &Vec<isize>) -> Result<isize> {
    if left.is_empty() {
        bail!("input empty")
    }
    let mut total: isize = 0;
    for i in left {
        let distance = i * right.iter().filter(|f| *f == i).count() as isize;
        total += distance;
    }
    Ok(total.into())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_run_algorithm_validdday1_data() -> Result<()> {
        let mut left = vec![3, 7, 10];
        let mut right = vec![1, 4, 8];
        let result = run_algorithm_1(&mut left, &mut right)?;
        assert_eq!(result, 7); // (3-1).abs() + (7-4).abs() + (10-8).abs() = 2 + 3 + 2 = 7
        Ok(())
    }

    #[test]
    fn test_parse_input_p1_1() -> Result<()> {
        let input = "3 1\n7 4\n10 8";
        let (mut left, mut right) = parse_input(input)?;
        let result = run_algorithm_1(&mut left, &mut right)?.to_string();
        assert_eq!(result, "7");
        Ok(())
    }

    #[test]
    fn test_algo_1_fails_empty_input() {
        assert!(run_algorithm_1(&mut Vec::new(), &mut Vec::new()).is_err());
    }

    #[test]
    fn test_algorithm_1_fails_for_different_sizes() {
        assert!(run_algorithm_1(&mut vec![1, 2, 3], &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).is_err())
    }

    #[test]
    fn test_run_algorithm_2_success() -> Result<()> {
        let left = vec![7, 4, 1];
        let right = vec![2, 7, 4, 8, 7, 1, 4, 4, 1];
        let result = run_algorithm_2(&left, &right)?;
        assert_eq!(result, 28);
        Ok(())
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = "3 1\n7 fish\n10 8";
        let result = parse_input(input);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Parse error at line 2, '7 fish', invalid record 2"));
    }

    #[test]
    fn test_parse_input_missing_columns() {
        let input = "3\n7 4\n10 8";
        let result = parse_input(input);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("ParseError on line 1, length not 2"));
    }

    #[test]
    fn test_parse_input_parses_empty_vecs_for_empty_input() -> Result<()> {
        let input = "";
        let (left, right) = parse_input(input)?;
        assert!(left.is_empty());
        assert!(right.is_empty());
        Ok(())
    }
}
