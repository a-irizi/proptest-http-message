//! HTTP request target in asterisk form strategies.

use proptest::prelude::{Just, Strategy};

/// strategy for generating target asterisk form.
pub fn asterisk() -> impl Strategy<Value = String> {
  Just("*".to_string())
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn asterisk_works(a in asterisk()) {
      assert_eq!("*", a, r#"expected asterisk form to be "*" but got {a:?}"#);
    }
  }
}
