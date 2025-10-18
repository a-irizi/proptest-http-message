//! strategies for generating IP v4 addresses.

use std::net::Ipv4Addr;

use proptest::prelude::{Strategy, any};

/// strategy for generating IP v4 address.
///
/// # Returns
/// `Ipv4Addr` and its representation.
pub fn ip_v4() -> impl Strategy<Value = (Ipv4Addr, String)> {
  ((any::<u8>()), (any::<u8>()), (any::<u8>()), (any::<u8>()))
    .prop_map(move |(a, b, c, d)| (Ipv4Addr::new(a, b, c, d), format!("{a}.{b}.{c}.{d}")))
}

#[cfg(test)]
mod tests {
  use claims::assert_ok;
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn ip_v4_works((ip, repr) in ip_v4()) {
      let parsed_ip: Ipv4Addr = assert_ok!(repr.parse());
      assert_eq!(ip, parsed_ip);
    }
  }
}
