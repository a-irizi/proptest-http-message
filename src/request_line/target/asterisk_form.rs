//! HTTP request target in asterisk form strategies.

use proptest::prelude::{Just, Strategy};

/// strategy for generating target asterisk form.
pub fn asterisk() -> impl Strategy<Value = String> {
  Just("*".to_string())
}

#[cfg(test)]
pub(super) mod tests {
  use proptest::proptest;

  use super::*;

  pub(in super::super) fn asterisk_asserts(repr: &str) {
    assert_eq!("*", repr, r#"expected asterisk form to be "*" but got {repr:?}"#);
  }

  proptest! {
    #[test]
    fn asterisk_works(a in asterisk()) {
      asterisk_asserts(&a);
    }
  }
}
