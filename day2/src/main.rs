mod policies;
use policies::parse_input;

fn main() -> anyhow::Result<()> {
		let count = include_str!("input.txt")
			.lines()
			.map(parse_input)
			.map(Result::unwrap)
			.filter(|(policy, password)| policy.is_valid(password))
			.count();

		println!("{} passwords are valid", count);

		Ok(())
}
