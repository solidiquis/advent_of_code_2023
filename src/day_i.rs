use crate::common::load_data_to_string;
use aho_corasick::AhoCorasick;
use anyhow::Result;

pub fn solution_part_i(path_to_data: &str) -> Result<u32> {
    let data = load_data_to_string(path_to_data)?;

    let mut total = 0;

    for line in data.lines() {
        let chars = line.chars().collect::<Vec<_>>();

        let mut left_cursor = 0;
        let mut right_cursor = match chars.len().checked_sub(1) {
            Some(num) => num,
            None => continue,
        };

        let mut left = None;
        let mut right = None;

        while left_cursor <= right_cursor && (left.is_none() || right.is_none()) {
            if left.is_none() {
                if chars[left_cursor].is_ascii_digit() {
                    left = Some(chars[left_cursor]);
                } else {
                    left_cursor += 1;
                }
            }

            if right.is_none() {
                if chars[right_cursor].is_ascii_digit() {
                    right = Some(chars[right_cursor])
                } else {
                    right_cursor -= 1;
                }
            }
        }

        let num = format!(
            "{}{}",
            left.map_or_else(String::new, |c| c.to_string()),
            right.map_or_else(String::new, |c| c.to_string())
        );

        total += num.parse::<u32>().unwrap_or(0);
    }

    Ok(total)
}

pub fn solution_part_ii(path_to_data: &str) -> Result<usize> {
    let data = load_data_to_string(path_to_data)?;

    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns)?;

    let alpha_to_char = |numstr| match numstr {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => unreachable!(),
    };

    let mut total = 0;

    for line in data.lines() {
        let first_alpha_match = ac.find_overlapping_iter(line).next();
        let last_alpha_match = ac.find_overlapping_iter(line).last();

        let chars = line.chars().collect::<Vec<_>>();

        let mut left_cursor = 0;
        let mut right_cursor = match chars.len().checked_sub(1) {
            Some(num) => num,
            None => continue,
        };

        let mut left = None;
        let mut right = None;

        while left_cursor <= right_cursor && (left.is_none() || right.is_none()) {
            if left.is_none() {
                if chars[left_cursor].is_ascii_digit() {
                    left = Some(chars[left_cursor]);
                } else {
                    left_cursor += 1;
                }
            }

            if right.is_none() {
                if chars[right_cursor].is_ascii_digit() {
                    right = Some(chars[right_cursor])
                } else {
                    right_cursor -= 1;
                }
            }
        }

        if let Some(mat) = first_alpha_match {
            if left_cursor > mat.start() {
                left = Some(alpha_to_char(&line[mat.start()..mat.end()]));
            }
        }

        if let Some(mat) = last_alpha_match {
            if right_cursor < mat.start() {
                right = Some(alpha_to_char(&line[mat.start()..mat.end()]));
            }
        }

        let num = format!(
            "{}{}",
            left.map_or_else(String::new, |c| c.to_string()),
            right.map_or_else(String::new, |c| c.to_string())
        );

        total += num.parse::<usize>().unwrap_or(0);
    }

    Ok(total)
}
