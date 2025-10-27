//! HTTP request target strategies.

use std::{num::NonZero, ops::RangeInclusive};

use proptest::{prelude::Strategy, prop_oneof};

use crate::request_line::target::{
  absolute_form::AbsoluteForm, authority_form::AuthorityForm, origin_form::OriginForm,
};

pub mod absolute_form;
pub mod asterisk_form;
pub mod authority_form;
pub mod components;
pub mod origin_form;

/// All valid HTTP request target forms.
#[derive(Debug)]
pub enum RequestTarget {
  Absolute(AbsoluteForm),
  Origin(OriginForm),
  Authority(AuthorityForm),
  Asterisk,
}

/// strategy for generating HTTP request target.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts in case of authority form,
///   origin form and absolute form.
/// * `max_segments`: maximum number of segments that compose the path in case of absolute form
///   and origin form.
/// * `query_count_range`: range of the number of queries to include in case of  absolute form
///   and origin form.
///
/// # Returns
/// [`RequestTarget`] and it representation.
pub fn target(
  max_label_count: usize,
  max_segments: NonZero<usize>,
  query_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = (RequestTarget, String)> {
  prop_oneof![
    absolute_form::absolute(max_label_count, max_segments, query_count_range.clone())
      .prop_map(|(absolute, repr)| (RequestTarget::Absolute(absolute), repr)),
    origin_form::origin(max_segments, query_count_range)
      .prop_map(|(origin, repr)| (RequestTarget::Origin(origin), repr)),
    authority_form::authority(max_label_count)
      .prop_map(|(authority, repr)| (RequestTarget::Authority(authority), repr)),
    asterisk_form::asterisk().prop_map(|repr| (RequestTarget::Asterisk, repr)),
  ]
}

#[cfg(test)]
pub(super) mod tests {
  use proptest::proptest;

  use super::*;

  pub(in super::super) fn target_asserts(target: &RequestTarget, repr: &str) {
    match target {
      RequestTarget::Absolute(absolute_form) => {
        absolute_form::tests::absolute_asserts(absolute_form, repr);
      }
      RequestTarget::Origin(origin_form) => origin_form::tests::origin_asserts(origin_form, repr),
      RequestTarget::Authority(authority_form) => {
        authority_form::tests::authority_asserts(authority_form, repr);
      }
      RequestTarget::Asterisk => asterisk_form::tests::asterisk_asserts(repr),
    }
  }

  proptest! {
    #[test]
    fn target_works((target, repr) in target(20, 50.try_into().unwrap(), 0..=20)) {
      target_asserts(&target, &repr);
    }
  }
}
