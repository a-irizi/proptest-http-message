use proptest::prelude::Strategy;

pub fn unreserved() -> impl Strategy<Value = String> {
  "[a-zA-Z0-9\\-._~]"
}
