use anyhow::Result;

fn main() -> anyhow::Result<()> {
    let input: Vec<i64> = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    let result = pair_sum_is_x(&input, 3488);
    dbg!(result);
    Ok(())
}

fn pair_sum_is_x(s: &[i64], x: i64) -> Option<(i64, i64)> {
    for (a, b) in all_pairs(s) {
        if a + b == x {
            return Some((a, b));
        }
    }
    None
}

fn all_pairs(s: &[i64]) -> Vec<(i64, i64)> {
    let mut pairs: Vec<_> = Default::default();
    for i in 0..s.len() {
        for j in 0..s.len() {
            if i == j {
                continue;
            }
            pairs.push((s[i], s[j]));
        }
    }
    pairs
}
