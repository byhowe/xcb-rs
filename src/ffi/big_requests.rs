use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t};

pub const XCB_BIGREQUESTS_MAJOR_VERSION: u32 = 0;
pub const XCB_BIGREQUESTS_MINOR_VERSION: u32 = 0;
pub const XCB_BIG_REQUESTS_ENABLE: u32 = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_big_requests_enable_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_big_requests_enable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_big_requests_enable_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub maximum_request_length: u32,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_big_requests_id: xcb_extension_t;

  pub fn xcb_big_requests_enable(c: *mut xcb_connection_t) -> xcb_big_requests_enable_cookie_t;

  pub fn xcb_big_requests_enable_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_big_requests_enable_cookie_t;

  pub fn xcb_big_requests_enable_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_big_requests_enable_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_big_requests_enable_reply_t;
}
