# Advent of Code 2023 (Rust)

’Tis the season for pagan worship. This year we will propitiate the god of memory safety, [Ferris](https://ferristhecrab.com/).

# Table of Solutions
* [Day I Part I](#day-i-part-i)
* [Day I Part II](#day-i-part-ii)
* [Day II Part I](#day-ii-part-i)
* [Day II Part II](#day-ii-part-ii)

## [Day I Part I](https://adventofcode.com/2023/day/1)

The efficient solution to this problem requires the "Two Pointer Technique." The algorithm is as follows:
1. Iterate line by line through the test case.
2. For each line, produce an iterator over the characters of that line.
3. Initialize a left cursor and a right cursor which indexes into the first and last items of the character iterator, respectively.
4. Traverse the character iterator rightward and leftward simultaneously until you encounter the first and the last character which can be parsed into a base 10 digit OR until the two cursors meet.
5. If you successfully generated a pair of characters then parse them together into an integer and increment a running total.

```rust
pub fn solution_part_i(test_case: &str) -> Result<u32> {
    let test_case = load_test_case_to_string(test_case)?;

    let mut total = 0;

    for line in test_case.lines() {
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
```

## [Day I Part II](https://adventofcode.com/2023/day/1)

This one was pretty challenging for day one I must admit. It honestly made me go "wtf." The most efficient solution here based from what I can tell would be to build your own finite state machine (or deterministic finite automata) that you'd use to parse each line to find the English spelling of numbers i.e. "one", "two", and so on. The problem, however, is the fact that you'd have overlapping cases such as "oneight" which would require a pretty sophisticated state machine to parse. Now it's possible that I'm overlooking a very simple solution here, but my approach was to use the [Aho-Corasick algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) to find the first and last instances of "one", "two", etc. — even if they overlap — keep track of their starting offsets, and the compare that with the ultimate value of each cursor from part one's approach to determine whether or not the left and/or right item in each pair need to be updated. Here's the solution:

```rust
pub fn solution_part_ii(test_case: &str) -> Result<usize> {
    let test_case = load_test_case_to_string(test_case)?;

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

    for line in test_case.lines() {
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
```

## [Day II Part I](https://adventofcode.com/2023/day/2)

This is one very straightforward.

```rust
pub struct Bag {
    pub red_cubes: usize,
    pub blue_cubes: usize,
    pub green_cubes: usize,
}

pub fn solution_part_i(
    path_to_data: &str,
    Bag {
        red_cubes,
        blue_cubes,
        green_cubes,
    }: &Bag,
) -> Result<usize> {
    let data = load_data_to_string(path_to_data)?;

    let mut total = 0;

    for (game_number, game) in data.lines().enumerate() {
        // Skip "Game "
        let cube_sets = game
            .chars()
            .skip_while(|c| *c != ':')
            .skip(2) // Skip ": "
            .collect::<String>();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for cube_set in cube_sets.split("; ") {
            for cube in cube_set.split(", ") {
                let [num, color]: [&str; 2] = cube
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .try_into()
                    .map_err(|_| Error::MalformedData)?;

                let num_cubes = num.parse::<usize>()?;

                match color {
                    "red" => red = red.max(num_cubes),
                    "blue" => blue = blue.max(num_cubes),
                    "green" => green = green.max(num_cubes),
                    _ => return Err(Error::MalformedData)?,
                }
            }
        }

        if red <= *red_cubes && blue <= *blue_cubes && green <= *green_cubes {
            total += game_number + 1
        }
    }

    Ok(total)
}
```

## [Day II Part II](https://adventofcode.com/2023/day/2)

This one is also very straightforward.

```rust
pub fn solution_part_ii(path_to_data: &str) -> Result<usize> {
    let data = load_data_to_string(path_to_data)?;

    let mut total = 0;

    for game in data.lines() {
        // Skip "Game "
        let cube_sets = game
            .chars()
            .skip_while(|c| *c != ':')
            .skip(2) // Skip ": "
            .collect::<String>();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for cube_set in cube_sets.split("; ") {
            for cube in cube_set.split(", ") {
                let [num, color]: [&str; 2] = cube
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .try_into()
                    .map_err(|_| Error::MalformedData)?;

                let num_cubes = num.parse::<usize>()?;

                match color {
                    "red" => red = red.max(num_cubes),
                    "blue" => blue = blue.max(num_cubes),
                    "green" => green = green.max(num_cubes),
                    _ => return Err(Error::MalformedData)?,
                }
            }
        }

        total += red * green * blue;
    }

    Ok(total)
}
```
