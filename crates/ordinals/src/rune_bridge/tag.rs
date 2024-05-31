

#[derive(Copy, Clone, Debug)]
pub(super) enum Tag {
  Protocol = 0,
  RuneBridge = 2,
  Block = 4,
}

impl From<Tag> for u128 {
  fn from(tag: Tag) -> Self {
    tag as u128
  }
}

impl PartialEq<u128> for Tag {
  fn eq(&self, other: &u128) -> bool {
    u128::from(*self) == *other
  }
}
