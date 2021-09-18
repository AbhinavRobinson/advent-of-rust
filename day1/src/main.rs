use anyhow::Result;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input: Vec<i64> = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    let result = pair_sum_is_x(&input, 3488);
    dbg!(result);
    Ok(())
}

fn pair_sum_is_x(s: &[i64], x: i64) -> Option<(&i64, &i64)> {
    s.iter().tuple_combinations().find(|(a, b)| *a + *b == x)
}
