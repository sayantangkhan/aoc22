use anyhow::{Context, Result};
use aoc22::*;

fn main() -> Result<()> {
    let input_args = read_args()?;

    if input_args.problem_num == (6, 1) {
        let result = day_6_1(&input_args.input_file).context("Problem in day 6, part 1")?;
        println!("{}", result);
    } else if input_args.problem_num == (6, 2) {
        let result = day_6_2(&input_args.input_file).context("Problem in day 6, part 2")?;
        println!("{}", result);
    } else if input_args.problem_num == (7, 1) {
        let result = day_7_1(&input_args.input_file).context("Problem in day 7, part 1")?;
        println!("{}", result);
    }

    Ok(())
}
