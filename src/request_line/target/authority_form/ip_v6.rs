//! strategies for generating IP v6 addresses.

use std::net::Ipv6Addr;

use proptest::{prop_oneof, strategy::Strategy};

use crate::request_line::target::authority_form::ip_v4::ip_v4;

#[must_use = "strategies do nothing unless used"]
fn ip_v6_segment_strategy() -> impl Strategy<Value = (u16, String)> {
  ((0u16..), (1..=4)).prop_map(move |(segment, pad)| match pad {
    1 => (segment, format!("{segment:01x}")),
    2 => (segment, format!("{segment:02x}")),
    3 => (segment, format!("{segment:03x}")),
    4 => (segment, format!("{segment:04x}")),
    _ => unreachable!("pad in in 1..4"),
  })
}

/// strategy for generating uncompressed IP v6 address.
///
/// # Returns
/// `Ipv6Addr` and its uncompressed representation.
pub fn ip_v6_uncompressed() -> impl Strategy<Value = (Ipv6Addr, String)> {
  (
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
    (ip_v6_segment_strategy()),
  )
    .prop_map(move |(a, b, c, d, e, f, g, h)| {
      (
        Ipv6Addr::new(a.0, b.0, c.0, d.0, e.0, f.0, g.0, h.0),
        format!("{}:{}:{}:{}:{}:{}:{}:{}", a.1, b.1, c.1, d.1, e.1, f.1, g.1, h.1),
      )
    })
}

/// strategy for generating compressed at the start IP v6 address.
///
/// the number of compressed segments is between 1 and 7 segments.
///
/// # Returns
/// `Ipv6Addr` and its compressed at the start representation.
pub fn ip_v6_compressed_start() -> impl Strategy<Value = (Ipv6Addr, String)> {
  proptest::collection::vec(ip_v6_segment_strategy(), 1usize..=7).prop_map(move |segments| {
    let (segments, segments_str): (Vec<_>, Vec<_>) = segments.into_iter().unzip();
    let mut full_segments = [0u16; 8];
    let start_index = 8 - segments.len();
    full_segments[start_index..].copy_from_slice(&segments);
    (
      Ipv6Addr::new(
        full_segments[0],
        full_segments[1],
        full_segments[2],
        full_segments[3],
        full_segments[4],
        full_segments[5],
        full_segments[6],
        full_segments[7],
      ),
      format!("::{}", segments_str.join(":")),
    )
  })
}

/// strategy for generating compressed at the end IP v6 address.
///
/// the number of compressed segments is between 1 and 7 segments.
///
/// # Returns
/// `Ipv6Addr` and its compressed at the end representation.
pub fn ip_v6_compressed_end() -> impl Strategy<Value = (Ipv6Addr, String)> {
  proptest::collection::vec(ip_v6_segment_strategy(), 1usize..=7).prop_map(move |segments| {
    let (segments, segments_str): (Vec<_>, Vec<_>) = segments.into_iter().unzip();
    let mut full_segments = [0u16; 8];
    full_segments[..segments.len()].copy_from_slice(&segments);
    (
      Ipv6Addr::new(
        full_segments[0],
        full_segments[1],
        full_segments[2],
        full_segments[3],
        full_segments[4],
        full_segments[5],
        full_segments[6],
        full_segments[7],
      ),
      format!("{}::", segments_str.join(":")),
    )
  })
}

/// strategy for generating compressed in the middle IP v6 address.
///
/// the number of compressed segments is between 1 and 6 segments.
///
/// # Returns
/// `Ipv6Addr` and its compressed in the middle representation.
pub fn ip_v6_compressed_middle() -> impl Strategy<Value = (Ipv6Addr, String)> {
  (1..=6usize)
    .prop_flat_map(move |start_segment_count| {
      (
        proptest::collection::vec(ip_v6_segment_strategy(), start_segment_count),
        proptest::collection::vec(ip_v6_segment_strategy(), 1..=7 - start_segment_count),
      )
    })
    .prop_map(move |(start_segments, end_segments)| {
      let (start_segments, start_segments_str): (Vec<_>, Vec<_>) =
        start_segments.into_iter().unzip();
      let (end_segments, end_segments_str): (Vec<_>, Vec<_>) = end_segments.into_iter().unzip();

      let mut full_segments = [0u16; 8];
      full_segments[..start_segments.len()].copy_from_slice(&start_segments);
      let end_segments_index = 8 - end_segments.len();
      full_segments[end_segments_index..].copy_from_slice(&end_segments);

      (
        Ipv6Addr::new(
          full_segments[0],
          full_segments[1],
          full_segments[2],
          full_segments[3],
          full_segments[4],
          full_segments[5],
          full_segments[6],
          full_segments[7],
        ),
        format!("{}::{}", start_segments_str.join(":"), end_segments_str.join(":")),
      )
    })
}

/// strategy for generating IP v6 mapped IP v4.
///
/// # Returns
/// `Ipv6Add` mapped IP v6 and its representation.
pub fn ip_v6_mapped_ip_v4() -> impl Strategy<Value = (Ipv6Addr, String)> {
  ip_v4()
    .prop_map(move |(ip_v4, ip_v4_repr)| (ip_v4.to_ipv6_mapped(), format!("::ffff:{ip_v4_repr}")))
}

pub fn ip_v6() -> impl Strategy<Value = (Ipv6Addr, String)> {
  prop_oneof![
    ip_v6_uncompressed(),
    ip_v6_compressed_start(),
    ip_v6_compressed_middle(),
    ip_v6_compressed_end(),
    ip_v6_mapped_ip_v4()
  ]
}

#[cfg(test)]
mod tests {
  use claims::assert_ok;
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn ip_v6_uncompressed_works((ip, repr) in ip_v6_uncompressed()) {
      assert!(!repr.contains("::"));
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }

    #[test]
    fn ip_v6_compressed_start_works((ip, repr) in ip_v6_compressed_start()) {
      assert!(repr.starts_with("::"));
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }

    #[test]
    fn ip_v6_compressed_end_works((ip, repr) in ip_v6_compressed_end()) {
      assert!(repr.ends_with("::"));
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }

    #[test]
    fn ip_v6_compressed_middle_works((ip, repr) in ip_v6_compressed_middle()) {
      assert!(!repr.starts_with("::"));
      assert!(!repr.ends_with("::"));
      assert!(repr.contains("::"));
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }

    #[test]
    fn ip_v6_mapped_ip_v4_works((ip, repr) in ip_v6_mapped_ip_v4()) {
      assert!(repr.to_lowercase().starts_with("::ffff:"));
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }

    #[test]
    fn ip_v6_works((ip, repr) in ip_v6()) {
      let parsed_ip: Ipv6Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }
  }
}
