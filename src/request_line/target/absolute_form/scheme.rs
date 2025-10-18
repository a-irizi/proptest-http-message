use proptest::prelude::Strategy;

/// HTTP scheme.
///
/// # Returns
/// HTTP or HTTPS with random letter casing.
pub fn http_scheme() -> impl Strategy<Value = String> {
  "(H|h)(t|T)(t|T)(p|P)(s|S){0,1}"
}

/// any valid scheme.
///
/// Scheme names consist of a sequence of characters beginning with a
/// letter and followed by any combination of letters, digits, plus
/// ("+"), period ("."), or hyphen ("-").
pub fn any_scheme() -> impl Strategy<Value = String> {
  "[a-zA-Z][a-zA-Z0-9+\\-.]"
}

#[cfg(test)]
mod tests {
  use claims::assert_matches;
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn http_scheme_works(scheme in http_scheme()) {
      assert_matches!(scheme.to_lowercase().as_str(), "http" | "https");
    }

    #[test]
    fn any_scheme_works(scheme in any_scheme()) {
      assert!(
        scheme.starts_with(|c: char| c.is_ascii_alphabetic()),
        "scheme name should start with a ASCII letter but got {scheme:?}"
      );
      assert!(
        scheme.chars().all(|c| matches!(c, 'a'..='z'|'A'..='Z'|'0'..='9'| '+' | '-' | '.')),
        "scheme name should only contain letters, digits, '+', '-' and '.' but got {scheme:?}"
      );
    }
  }
}
