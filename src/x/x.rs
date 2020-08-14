use super::{ConnectionError, Setup};
use crate::xcb;
use std::ffi::CString;

pub struct X
{
  c: *mut xcb::connection_t,
  default_screen: i32,
}

impl X
{
  #[inline(always)]
  pub fn connect(display_name: &str) -> Result<Self, ConnectionError>
  {
    let display_name: *const i8 = if display_name.is_empty() {
      std::ptr::null()
    } else {
      CString::new(display_name).unwrap().as_ptr()
    };

    let mut default_screen: i32 = 0;
    let c: *mut xcb::connection_t = unsafe { xcb::connect(display_name, &mut default_screen) };

    match ConnectionError::has_error(c) {
      Some(err) => Err(err),
      None => Ok(Self { c, default_screen }),
    }
  }

  #[inline(always)]
  pub fn has_error(&self) -> Option<ConnectionError>
  {
    ConnectionError::has_error(self.c)
  }

  #[inline(always)]
  pub fn flush(&self) -> bool
  {
    unsafe { xcb::flush(self.c) > 0 }
  }

  #[inline(always)]
  pub fn generate_id(&self) -> u32
  {
    unsafe { xcb::generate_id(self.c) }
  }

  #[inline(always)]
  pub fn get_setup(&self) -> Setup
  {
    Setup::from(unsafe { xcb::get_setup(self.c) })
  }
}

impl Drop for X
{
  fn drop(&mut self)
  {
    unsafe { xcb::disconnect(self.c) }
  }
}
