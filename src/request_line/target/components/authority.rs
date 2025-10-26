//! URL authority strategies.

use std::fmt::Write;

use proptest::{
  option::of,
  prelude::{Strategy, any},
};

use super::host::{Host, host};
use super::user_info::{UserInfo, user_info};

/// URL authority.
#[derive(Debug)]
pub struct Authority {
  pub user_info: Option<UserInfo>,
  pub host: Host,
  pub port: Option<u16>,
}

/// strategy for generating URL authority.
///
/// URL authority has following format: `[<user-info>@]<host>[:<port]`
///
/// # Returns
/// [Authority] with it representation.
pub fn authority(max_label_count: usize) -> impl Strategy<Value = (Authority, String)> {
  (of(user_info()), host(max_label_count), of(any::<u16>())).prop_map(|(user_info, host, port)| {
    let mut repr = String::new();
    if let Some((_, user_info_repr)) = user_info.as_ref() {
      let _ = write!(repr, "{user_info_repr}@");
    }
    let host_repr = match &host {
      Host::Domain(host) | Host::Ipv6(_, host) | Host::Ipv4(_, host) => host,
    };
    let _ = write!(repr, "{host_repr}");
    if let Some(port_repr) = port.as_ref() {
      let _ = write!(repr, ":{port_repr}");
    }

    (Authority { user_info: user_info.map(|user_info| user_info.0), host, port }, repr)
  })
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn authority_works((authority, repr) in authority(25)) {
      if authority.user_info.as_ref().is_some() {
        assert!(repr.contains('@'), r#"authority without user info should contain "@" but got {repr:?}"#);
      } else {
        assert!(!repr.contains('@'), r#"authority without user info should not contain "@" but got {repr:?}"#);
      }

      if let Some(port) = authority.port {
        assert!(repr.ends_with(&format!(":{port}")));
      }
    }
  }
}
