use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_iterator_t};
use super::xcb::xcb_window_t;

pub const XCB_XINERAMA_MAJOR_VERSION: u32 = 1;
pub const XCB_XINERAMA_MINOR_VERSION: u32 = 1;
pub const XCB_XINERAMA_QUERY_VERSION: u32 = 0;
pub const XCB_XINERAMA_GET_STATE: u32 = 1;
pub const XCB_XINERAMA_GET_SCREEN_COUNT: u32 = 2;
pub const XCB_XINERAMA_GET_SCREEN_SIZE: u32 = 3;
pub const XCB_XINERAMA_IS_ACTIVE: u32 = 4;
pub const XCB_XINERAMA_QUERY_SCREENS: u32 = 5;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_screen_info_t
{
  pub x_org: i16,
  pub y_org: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_screen_info_iterator_t
{
  pub data: *mut xcb_xinerama_screen_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major: u8,
  pub minor: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major: u16,
  pub minor: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_state_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_state_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_state_reply_t
{
  pub response_type: u8,
  pub state: u8,
  pub sequence: u16,
  pub length: u32,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_count_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_count_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_count_reply_t
{
  pub response_type: u8,
  pub screen_count: u8,
  pub sequence: u16,
  pub length: u32,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_size_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_size_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_get_screen_size_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u32,
  pub height: u32,
  pub window: xcb_window_t,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_is_active_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_is_active_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_is_active_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub state: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_screens_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_screens_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xinerama_query_screens_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub number: u32,
  pub pad1: [u8; 20usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_xinerama_id: xcb_extension_t;

  pub fn xcb_xinerama_screen_info_next(i: *mut xcb_xinerama_screen_info_iterator_t);

  pub fn xcb_xinerama_screen_info_end(
    i: xcb_xinerama_screen_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xinerama_query_version(
    c: *mut xcb_connection_t,
    major: u8,
    minor: u8,
  ) -> xcb_xinerama_query_version_cookie_t;

  pub fn xcb_xinerama_query_version_unchecked(
    c: *mut xcb_connection_t,
    major: u8,
    minor: u8,
  ) -> xcb_xinerama_query_version_cookie_t;

  pub fn xcb_xinerama_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_query_version_reply_t;

  pub fn xcb_xinerama_get_state(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xinerama_get_state_cookie_t;

  pub fn xcb_xinerama_get_state_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xinerama_get_state_cookie_t;

  pub fn xcb_xinerama_get_state_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_get_state_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_get_state_reply_t;

  pub fn xcb_xinerama_get_screen_count(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xinerama_get_screen_count_cookie_t;

  pub fn xcb_xinerama_get_screen_count_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xinerama_get_screen_count_cookie_t;

  pub fn xcb_xinerama_get_screen_count_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_get_screen_count_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_get_screen_count_reply_t;

  pub fn xcb_xinerama_get_screen_size(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    screen: u32,
  ) -> xcb_xinerama_get_screen_size_cookie_t;

  pub fn xcb_xinerama_get_screen_size_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    screen: u32,
  ) -> xcb_xinerama_get_screen_size_cookie_t;

  pub fn xcb_xinerama_get_screen_size_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_get_screen_size_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_get_screen_size_reply_t;

  pub fn xcb_xinerama_is_active(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t;

  pub fn xcb_xinerama_is_active_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_xinerama_is_active_cookie_t;

  pub fn xcb_xinerama_is_active_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_is_active_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_is_active_reply_t;

  pub fn xcb_xinerama_query_screens_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xinerama_query_screens(
    c: *mut xcb_connection_t
  ) -> xcb_xinerama_query_screens_cookie_t;

  pub fn xcb_xinerama_query_screens_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_xinerama_query_screens_cookie_t;

  pub fn xcb_xinerama_query_screens_screen_info(
    R: *const xcb_xinerama_query_screens_reply_t
  ) -> *mut xcb_xinerama_screen_info_t;

  pub fn xcb_xinerama_query_screens_screen_info_length(
    R: *const xcb_xinerama_query_screens_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xinerama_query_screens_screen_info_iterator(
    R: *const xcb_xinerama_query_screens_reply_t
  ) -> xcb_xinerama_screen_info_iterator_t;

  pub fn xcb_xinerama_query_screens_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xinerama_query_screens_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xinerama_query_screens_reply_t;
}
