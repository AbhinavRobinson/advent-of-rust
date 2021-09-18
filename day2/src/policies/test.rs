#[cfg(test)]
use super::{parse_input, PasswordPolicy};

#[test]
fn test_is_valid() {
	let pp = PasswordPolicy {
		positions: [1, 3],
		byte: b'a',
	};
	assert!(pp.is_valid("abcde"), "'a' in position 1");
	assert!(pp.is_valid("bcade"), "'a' in position 3");
	assert!(!pp.is_valid("food"), "no 'a' whatsoever");
	assert!(!pp.is_valid("abacus"), "'a' in both positions");
}

#[test]
fn test_parse() {
	assert_eq!(
		parse_input("1-3 a: banana").unwrap(),
		(
			PasswordPolicy {
				positions: [1, 3],
				byte: b'a',
			},
			"banana"
		)
	);
}
