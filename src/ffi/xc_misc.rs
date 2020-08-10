use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_iterator_t};

pub const XCB_XCMISC_MAJOR_VERSION: u32 = 1;
pub const XCB_XCMISC_MINOR_VERSION: u32 = 1;
pub const XCB_XC_MISC_GET_VERSION: u32 = 0;
pub const XCB_XC_MISC_GET_XID_RANGE: u32 = 1;
pub const XCB_XC_MISC_GET_XID_LIST: u32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u16,
  pub client_minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub server_major_version: u16,
  pub server_minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_range_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_range_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_range_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub start_id: u32,
  pub count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_list_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xc_misc_get_xid_list_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ids_len: u32,
  pub pad1: [u8; 20usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_xc_misc_id: xcb_extension_t;

  pub fn xcb_xc_misc_get_version(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_xc_misc_get_version_cookie_t;

  pub fn xcb_xc_misc_get_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_xc_misc_get_version_cookie_t;

  pub fn xcb_xc_misc_get_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xc_misc_get_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xc_misc_get_version_reply_t;

  pub fn xcb_xc_misc_get_xid_range(c: *mut xcb_connection_t) -> xcb_xc_misc_get_xid_range_cookie_t;

  pub fn xcb_xc_misc_get_xid_range_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_xc_misc_get_xid_range_cookie_t;

  pub fn xcb_xc_misc_get_xid_range_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xc_misc_get_xid_range_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xc_misc_get_xid_range_reply_t;

  pub fn xcb_xc_misc_get_xid_list_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xc_misc_get_xid_list(
    c: *mut xcb_connection_t,
    count: u32,
  ) -> xcb_xc_misc_get_xid_list_cookie_t;

  pub fn xcb_xc_misc_get_xid_list_unchecked(
    c: *mut xcb_connection_t,
    count: u32,
  ) -> xcb_xc_misc_get_xid_list_cookie_t;

  pub fn xcb_xc_misc_get_xid_list_ids(R: *const xcb_xc_misc_get_xid_list_reply_t) -> *mut u32;

  pub fn xcb_xc_misc_get_xid_list_ids_length(
    R: *const xcb_xc_misc_get_xid_list_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xc_misc_get_xid_list_ids_end(
    R: *const xcb_xc_misc_get_xid_list_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xc_misc_get_xid_list_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xc_misc_get_xid_list_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xc_misc_get_xid_list_reply_t;
}
