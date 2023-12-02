#![allow(dead_code)]
/// https://adventofcode.com/2023/day/1
mod day_i;

/// Common utilities shared for each problem.
mod common;

#[cfg(test)]
mod test {
    use anyhow::Result;

    #[test]
    fn day_i() -> Result<()> {
        let part_i = super::day_i::solution_part_i("day_i_part_i.txt")?;
        assert_eq!(part_i, 54667);

        let part_ii = super::day_i::solution_part_ii("day_i_part_ii.txt")?;
        assert_eq!(part_ii, 54203);
        Ok(())
    }
}
