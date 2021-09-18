mod test;

use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
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

pub fn parse_input(input: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    peg::parser! {
      grammar parser() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> RangeInclusive<usize>
          = min:number() "-" max:number() { min..=max }

        rule byte() -> u8
          = letter:$(['a'..='z']) { letter.as_bytes()[0] }

        rule password() -> &'input str
          = letters:$([_]*) { letters }

        pub(crate) rule line() -> (PasswordPolicy, &'input str)
          = range:range() " " byte:byte() ": " password:password() {
              (PasswordPolicy { byte, range }, password)
          }
      }
    }

    Ok(parser::line(input)?)
}
