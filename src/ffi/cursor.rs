use super::core::xcb_connection_t;
use super::xcb::{xcb_cursor_t, xcb_screen_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_cursor_context_t
{
  _unused: [u8; 0],
}

#[link(name = "xcb")]
extern "C" {
  pub fn xcb_cursor_context_new(
    conn: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
    ctx: *mut *mut xcb_cursor_context_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_cursor_load_cursor(
    ctx: *mut xcb_cursor_context_t,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_cursor_t;

  pub fn xcb_cursor_context_free(ctx: *mut xcb_cursor_context_t);
}
