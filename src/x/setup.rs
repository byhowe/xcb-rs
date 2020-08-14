use super::iterators::ScreenIterator;
use crate::xcb;

pub struct Setup
{
  ptr: *const xcb::setup_t,
}

impl From<*const xcb::setup_t> for Setup
{
  fn from(ptr: *const xcb::setup_t) -> Self
  {
    Self { ptr }
  }
}

impl Setup
{
  pub fn roots(&self) -> ScreenIterator
  {
    ScreenIterator {
      ptr: unsafe { xcb::setup_roots_iterator(self.ptr) },
    }
  }
}
