//! HTTP request line HTTP version strategies.

use std::fmt;

use proptest::{
  prelude::{Just, Strategy},
  prop_oneof,
};

const HTTP_1_0: &str = "HTTP/1.0";
const HTTP_1_1: &str = "HTTP/1.1";
const HTTP_2: &str = "HTTP/2";
const HTTP_3: &str = "HTTP/3";

/// All valid HTTP versions.
#[derive(Debug, Clone)]
pub enum HttpVersion {
  Http10,
  Http11,
  Http2,
  Http3,
}

impl fmt::Display for HttpVersion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      HttpVersion::Http10 => write!(f, "{HTTP_1_0}"),
      HttpVersion::Http11 => write!(f, "{HTTP_1_1}"),
      HttpVersion::Http2 => write!(f, "{HTTP_2}"),
      HttpVersion::Http3 => write!(f, "{HTTP_3}"),
    }
  }
}

/// strategy for generating HTTP version.
///
/// # Returns
/// [`HttpVersion`] and it representation.
pub fn version() -> impl Strategy<Value = (HttpVersion, String)> {
  prop_oneof![
    Just((HttpVersion::Http10, HttpVersion::Http10.to_string())),
    Just((HttpVersion::Http11, HttpVersion::Http11.to_string())),
    Just((HttpVersion::Http2, HttpVersion::Http2.to_string())),
    Just((HttpVersion::Http3, HttpVersion::Http3.to_string())),
  ]
}

#[cfg(test)]
pub(super) mod tests {
  use proptest::proptest;

  use super::*;

  pub(in super::super) fn version_asserts(version: &HttpVersion, repr: &str) {
    match version {
      HttpVersion::Http10 => assert_eq!(repr, HTTP_1_0, "expected HTTP version 1.0 but got {repr}"),
      HttpVersion::Http11 => assert_eq!(repr, HTTP_1_1, "expected HTTP version 1.1 but got {repr}"),
      HttpVersion::Http2 => assert_eq!(repr, HTTP_2, "expected HTTP version 2 but got {repr}"),
      HttpVersion::Http3 => assert_eq!(repr, HTTP_3, "expected HTTP version 3 but got {repr}"),
    }
  }

  proptest! {
    #[test]
    fn version_works((version, repr) in version()) {
      version_asserts(&version, &repr);
    }
  }
}
