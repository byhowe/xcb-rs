use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_void_cookie_t};
use super::xcb::{xcb_cursor_t, xcb_window_t};

pub const XCB_TEST_MAJOR_VERSION: u32 = 2;
pub const XCB_TEST_MINOR_VERSION: u32 = 2;
pub const XCB_TEST_GET_VERSION: u32 = 0;
pub const XCB_TEST_COMPARE_CURSOR: u32 = 1;
pub const XCB_TEST_FAKE_INPUT: u32 = 2;
pub const XCB_TEST_GRAB_CONTROL: u32 = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_get_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_get_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u8,
  pub pad0: u8,
  pub minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_get_version_reply_t
{
  pub response_type: u8,
  pub major_version: u8,
  pub sequence: u16,
  pub length: u32,
  pub minor_version: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_test_cursor_t
{
  XCB_TEST_CURSOR_NONE = 0,
  XCB_TEST_CURSOR_CURRENT = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_compare_cursor_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_compare_cursor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub cursor: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_compare_cursor_reply_t
{
  pub response_type: u8,
  pub same: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_fake_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub type_: u8,
  pub detail: u8,
  pub pad0: [u8; 2usize],
  pub time: u32,
  pub root: xcb_window_t,
  pub pad1: [u8; 8usize],
  pub rootX: i16,
  pub rootY: i16,
  pub pad2: [u8; 7usize],
  pub deviceid: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_test_grab_control_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub impervious: u8,
  pub pad0: [u8; 3usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_test_id: xcb_extension_t;

  pub fn xcb_test_get_version(
    c: *mut xcb_connection_t,
    major_version: u8,
    minor_version: u16,
  ) -> xcb_test_get_version_cookie_t;

  pub fn xcb_test_get_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u8,
    minor_version: u16,
  ) -> xcb_test_get_version_cookie_t;

  pub fn xcb_test_get_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_test_get_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_test_get_version_reply_t;

  pub fn xcb_test_compare_cursor(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    cursor: xcb_cursor_t,
  ) -> xcb_test_compare_cursor_cookie_t;

  pub fn xcb_test_compare_cursor_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    cursor: xcb_cursor_t,
  ) -> xcb_test_compare_cursor_cookie_t;

  pub fn xcb_test_compare_cursor_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_test_compare_cursor_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_test_compare_cursor_reply_t;

  pub fn xcb_test_fake_input_checked(
    c: *mut xcb_connection_t,
    type_: u8,
    detail: u8,
    time: u32,
    root: xcb_window_t,
    rootX: i16,
    rootY: i16,
    deviceid: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_test_fake_input(
    c: *mut xcb_connection_t,
    type_: u8,
    detail: u8,
    time: u32,
    root: xcb_window_t,
    rootX: i16,
    rootY: i16,
    deviceid: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_test_grab_control_checked(
    c: *mut xcb_connection_t,
    impervious: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_test_grab_control(
    c: *mut xcb_connection_t,
    impervious: u8,
  ) -> xcb_void_cookie_t;
}
