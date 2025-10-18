//! host domain strategies.

use proptest::{prop_oneof, strategy::Strategy};

fn alphanumeric_and_hyphen() -> impl Strategy<Value = String> {
  "[a-zA-Z0-9\\-]{0,61}"
}

fn alphanumeric_and_hyphen_ends_with_alphanumeric() -> impl Strategy<Value = String> {
  ((alphanumeric_and_hyphen()), "[a-zA-Z0-9]{1}").prop_map(move |(anh, an)| format!("{anh}{an}"))
}

fn optional_alphanumeric() -> impl Strategy<Value = String> {
  "[a-zA-Z0-9]{0,1}"
}

fn domain_label_remainder() -> impl Strategy<Value = String> {
  prop_oneof![optional_alphanumeric(), alphanumeric_and_hyphen_ends_with_alphanumeric()]
}

/// strategy for generating domain labels.
///
/// a domain is composed of labels separated by period.
pub fn domain_label() -> impl Strategy<Value = String> {
  ("[a-zA-Z]", (domain_label_remainder())).prop_map(move |(a, remainder)| format!("{a}{remainder}"))
}

/// strategy for generating domains.
///
/// # Arguments
/// * `max_label_count`: the maximum number of labels composing the domain.
pub fn domain(max_label_count: usize) -> impl Strategy<Value = String> {
  ("\\.{0,1}", proptest::collection::vec(domain_label(), 1..=max_label_count))
    .prop_map(move |(root, labels)| format!("{}{}", root, labels.join(".")))
}

#[cfg(test)]
mod tests {
  use claims::{assert_ge, assert_le};
  use proptest::proptest;

  use super::*;

  fn assert_label_is_correct(label: &str) {
    assert_le!(label.len(), 63, "label must be no more than 63 characters but got {label:?}");
    assert_ge!(label.len(), 1, "label must have at least one letter but got {label:?}");
    assert!(
      label.starts_with(|c: char| c.is_ascii_alphabetic()),
      "domain label must start with letter but got {label:?}"
    );

    assert!(
      label.ends_with(|c: char| c.is_ascii_alphanumeric()),
      "domain label must end with alphanumeric but got {label:?}"
    );
    assert!(
      label.chars().all(|c| matches!(c, 'a'..='z'| 'A'..='Z' | '0'..='9' | '-')),
      "domain label must only contain letters, digits and hyphen"
    );
  }

  proptest! {
    #[test]
    fn domain_label_works(label in domain_label()) {
      assert_label_is_correct(&label);
    }

    #[test]
    fn domain_works(domain in domain(12)) {
      assert!(domain.starts_with(|c: char| c.is_ascii_alphabetic() || c == '.'));
      let domain = domain.strip_prefix('.').unwrap_or(&domain);
      for label in domain.split('.') {
        assert_label_is_correct(label);
      }
    }
  }
}
