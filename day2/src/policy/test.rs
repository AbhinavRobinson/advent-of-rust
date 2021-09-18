#[cfg(test)]

use super::PasswordPolicy;

#[test]
fn test_is_valid() {
  let pp = PasswordPolicy {
    range: 1..=3,
    byte: b'a',
  };
  // Truthy
  assert!(pp.is_valid("bab"), "One a");
  assert!(pp.is_valid("aaab"), "Three a's");
  // Falsy
  assert!(!pp.is_valid("b"), "No a's");
  assert!(!pp.is_valid("baaaa"),  "Too Many a's");
}