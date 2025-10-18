//! URL host strategies.

use std::net::{Ipv4Addr, Ipv6Addr};

use proptest::{prelude::Strategy, prop_oneof};

mod domain;
mod ip_v4;
mod ip_v6;

/// strategy for generating IP v6 hosts.
///
/// IP v6 host is just an IP v6 wrapped in square brackets.
///
/// # Returns
/// `Ipv6Addr` and its host representation.
pub fn ip_v6_host() -> impl Strategy<Value = (Ipv6Addr, String)> {
  ip_v6::ip_v6().prop_map(move |(ip, repr)| (ip, format!("[{repr}]")))
}

/// URI host variants.
#[derive(Debug)]
pub enum Host {
  Domain(String),
  Ipv6(Ipv6Addr, String),
  Ipv4(Ipv4Addr, String),
}

/// strategy for generating URI hosts.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts.
pub fn host(max_label_count: usize) -> impl Strategy<Value = Host> {
  prop_oneof![
    domain::domain(max_label_count).prop_map(Host::Domain),
    ip_v4::ip_v4().prop_map(|(ip, repr)| Host::Ipv4(ip, repr)),
    ip_v6_host().prop_map(|(ip, repr)| Host::Ipv6(ip, repr))
  ]
}
