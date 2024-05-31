use super::*;

pub(super) struct Message {
  pub(super) success: bool,
  pub(super) block: Option<Vec<u8>>,
}

impl Message {
  pub(super) fn from_payload(tx: &Transaction, payload: &[u8]) -> Self {
    let plen = payload.len();
    if plen < 4 {
      return Self { success: false, block: None };
    }
    let protocol_key = u128::try_from(payload[0]).unwrap();
    if Tag::Protocol != protocol_key {
      return Self { success: false, block: None };
    }
    let protocol_val = u128::try_from(payload[1]).unwrap();
    if Tag::RuneBridge != protocol_val {
      return Self { success: false, block: None };
    }
    let block_key = u128::try_from(payload[2]).unwrap();
    if Tag::Block != block_key {
      return Self { success: false, block: None };
    }
    return Self { success: true, block: Some(payload[3..].to_owned()) };
  }
}
