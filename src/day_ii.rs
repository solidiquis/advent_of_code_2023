use crate::common::{load_data_to_string, Error};
use anyhow::Result;

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
