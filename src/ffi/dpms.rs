use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_void_cookie_t};

pub const XCB_DPMS_MAJOR_VERSION: u32 = 0;
pub const XCB_DPMS_MINOR_VERSION: u32 = 0;
pub const XCB_DPMS_GET_VERSION: u32 = 0;
pub const XCB_DPMS_CAPABLE: u32 = 1;
pub const XCB_DPMS_GET_TIMEOUTS: u32 = 2;
pub const XCB_DPMS_SET_TIMEOUTS: u32 = 3;
pub const XCB_DPMS_ENABLE: u32 = 4;
pub const XCB_DPMS_DISABLE: u32 = 5;
pub const XCB_DPMS_FORCE_LEVEL: u32 = 6;
pub const XCB_DPMS_INFO: u32 = 7;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u16,
  pub client_minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_version_reply_t
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
pub struct xcb_dpms_capable_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_capable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_capable_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub capable: u8,
  pub pad1: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_timeouts_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_timeouts_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_get_timeouts_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub standby_timeout: u16,
  pub suspend_timeout: u16,
  pub off_timeout: u16,
  pub pad1: [u8; 18usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_set_timeouts_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub standby_timeout: u16,
  pub suspend_timeout: u16,
  pub off_timeout: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_enable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_disable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_dpms_dpms_mode_t
{
  XCB_DPMS_DPMS_MODE_ON = 0,
  XCB_DPMS_DPMS_MODE_STANDBY = 1,
  XCB_DPMS_DPMS_MODE_SUSPEND = 2,
  XCB_DPMS_DPMS_MODE_OFF = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_force_level_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub power_level: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dpms_info_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub power_level: u16,
  pub state: u8,
  pub pad1: [u8; 21usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_dpms_id: xcb_extension_t;

  pub fn xcb_dpms_get_version(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_dpms_get_version_cookie_t;

  pub fn xcb_dpms_get_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_dpms_get_version_cookie_t;

  pub fn xcb_dpms_get_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dpms_get_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dpms_get_version_reply_t;

  pub fn xcb_dpms_capable(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t;

  pub fn xcb_dpms_capable_unchecked(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t;

  pub fn xcb_dpms_capable_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dpms_capable_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dpms_capable_reply_t;

  pub fn xcb_dpms_get_timeouts(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t;

  pub fn xcb_dpms_get_timeouts_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_dpms_get_timeouts_cookie_t;

  pub fn xcb_dpms_get_timeouts_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dpms_get_timeouts_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dpms_get_timeouts_reply_t;

  pub fn xcb_dpms_set_timeouts_checked(
    c: *mut xcb_connection_t,
    standby_timeout: u16,
    suspend_timeout: u16,
    off_timeout: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dpms_set_timeouts(
    c: *mut xcb_connection_t,
    standby_timeout: u16,
    suspend_timeout: u16,
    off_timeout: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dpms_enable_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_dpms_enable(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_dpms_disable_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_dpms_disable(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_dpms_force_level_checked(
    c: *mut xcb_connection_t,
    power_level: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dpms_force_level(
    c: *mut xcb_connection_t,
    power_level: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dpms_info(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t;

  pub fn xcb_dpms_info_unchecked(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t;

  pub fn xcb_dpms_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dpms_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dpms_info_reply_t;
}
