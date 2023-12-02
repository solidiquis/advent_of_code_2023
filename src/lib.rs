#![allow(dead_code)]

/// https://adventofcode.com/2023/day/1
mod day_i;

/// https://adventofcode.com/2023/day/2
mod day_ii;

/// Common utilities shared for each problem.
mod common;

#[cfg(test)]
mod test {
    use anyhow::Result;

    #[test]
    fn day_i() -> Result<()> {
        use super::day_i::{solution_part_i, solution_part_ii};

        let part_i = solution_part_i("day_i_part_i.txt")?;
        assert_eq!(part_i, 54667);

        let part_ii = solution_part_ii("day_i_part_ii.txt")?;
        assert_eq!(part_ii, 54203);
        Ok(())
    }

    #[test]
    fn day_ii() -> Result<()> {
        use super::day_ii::{solution_part_i, solution_part_ii, Bag};

        let cube_bag = Bag {
            red_cubes: 12,
            blue_cubes: 14,
            green_cubes: 13,
        };

        let part_i = solution_part_i("day_ii.txt", &cube_bag)?;
        assert_eq!(part_i, 2683);

        let part_ii = solution_part_ii("day_ii.txt")?;
        assert_eq!(part_ii, 49710);

        Ok(())
    }
}
