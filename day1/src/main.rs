fn main() -> anyhow::Result<()> {
    let input: Vec<i64> = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let result = pair_sum_is_x(&input, 3488);
    dbg!(result);
    Ok(())
}

fn pair_sum_is_x(s: &[i64], x: i64) -> Option<(i64, i64)> {
    for i in 0..s.len() {
        for j in 0..s.len() {
            if i == j {
                continue;
            }
            if s[i] + s[j] == x {
                return Some((s[i], s[j]));
            }
        }
    }
    None
}
