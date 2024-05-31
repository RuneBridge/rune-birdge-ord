use super::*;

pub(super) struct Message {
  pub(super) block: Option<Vec<u8>>,
}

impl Default for Message {
  fn default() -> Self {
    Self {
      block: None,
    }
  }
}

impl Message {
  pub(super) fn from_payload(tx: &Transaction, payload: &[u8]) -> Self {
    let plen = payload.len();
    if plen < 4 {
      return Self::default();
    }
    let protocol_key = u128::try_from(payload[0]).unwrap();
    if Tag::Protocol != protocol_key {
      return Self::default();
    }
    let protocol_val = u128::try_from(payload[1]).unwrap();
    if Tag::RuneBridge != protocol_val {
      return Self::default();
    }
    let block_key = u128::try_from(payload[2]).unwrap();
    if Tag::Block != block_key {
      return Self::default();
    }
    return Self { block: Some(payload[3..].to_owned()) };
  }
}
