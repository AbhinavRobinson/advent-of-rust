mod test;

use std::ops::RangeInclusive;

pub struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    pub fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .filter(|&b| *b == self.byte)
                .count(),
        )
    }
}
