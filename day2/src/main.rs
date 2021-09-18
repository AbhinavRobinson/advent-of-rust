use std::ops::RangeInclusive;

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        todo!()
    }
}

fn parse_input(input: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let count  = include_str!("input.txt")
    .lines()
    .map(parse_input)
    .map(Result::unwrap)
    .filter(|(policy, password)| policy.is_valid(password))
    .count();

    println!("{} passwords are valid", count);

    Ok(())
}