mod test;

#[derive(PartialEq, Debug)]
pub struct PasswordPolicy {
	byte: u8,
	positions: [usize; 2],
}

impl PasswordPolicy {
	pub fn is_valid(&self, password: &str) -> bool {
		self.positions
			.iter()
			.copied()
			.filter(|&index| password.as_bytes()[index] == self.byte)
			.count()
			== 1
	}
}

pub fn parse_input(input: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
	peg::parser! {
		grammar parser() for str {
			rule number() -> usize
				= n:$(['0'..='9']+) { n.parse().unwrap() }

			rule position() -> usize
				= n:number() { n - 1 }

			rule positions() -> [usize; 2]
				= first:position() "-" second:position() { [first, second] }

			rule byte() -> u8
				= letter:$(['a'..='z']) { letter.as_bytes()[0] }

			rule password() -> &'input str
				= letters:$([_]*) { letters }

			pub(crate) rule line() -> (PasswordPolicy, &'input str)
				= positions:positions() " " byte:byte() ": " password:password() {
					(PasswordPolicy { positions, byte }, password)
				}
		}
	}

	Ok(parser::line(input)?)
}
