use std::ops::RangeInclusive;

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn isValid(&self, password: &str) -> bool {
        todo!()
    }
}

fn parse_input(input: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let input  = include_str!("input.txt");

    Ok(())
}