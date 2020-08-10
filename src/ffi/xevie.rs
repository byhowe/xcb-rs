use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_iterator_t};

pub const XCB_XEVIE_MAJOR_VERSION: u32 = 1;
pub const XCB_XEVIE_MINOR_VERSION: u32 = 0;
pub const XCB_XEVIE_QUERY_VERSION: u32 = 0;
pub const XCB_XEVIE_START: u32 = 1;
pub const XCB_XEVIE_END: u32 = 2;
pub const XCB_XEVIE_SEND: u32 = 3;
pub const XCB_XEVIE_SELECT_INPUT: u32 = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u16,
  pub client_minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub server_major_version: u16,
  pub server_minor_version: u16,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_start_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_start_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_start_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_end_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_end_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cmap: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_end_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xevie_datatype_t
{
  XCB_XEVIE_DATATYPE_UNMODIFIED = 0,
  XCB_XEVIE_DATATYPE_MODIFIED = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_event_t
{
  pub pad0: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_event_iterator_t
{
  pub data: *mut xcb_xevie_event_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_send_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_xevie_send_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub event: xcb_xevie_event_t,
  pub data_type: u32,
  pub pad0: [u8; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_send_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_select_input_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub event_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xevie_select_input_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_xevie_id: xcb_extension_t;

  pub fn xcb_xevie_query_version(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_xevie_query_version_cookie_t;

  pub fn xcb_xevie_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u16,
    client_minor_version: u16,
  ) -> xcb_xevie_query_version_cookie_t;

  pub fn xcb_xevie_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xevie_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xevie_query_version_reply_t;

  pub fn xcb_xevie_start(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xevie_start_cookie_t;

  pub fn xcb_xevie_start_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xevie_start_cookie_t;

  pub fn xcb_xevie_start_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xevie_start_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xevie_start_reply_t;

  pub fn xcb_xevie_end(
    c: *mut xcb_connection_t,
    cmap: u32,
  ) -> xcb_xevie_end_cookie_t;

  pub fn xcb_xevie_end_unchecked(
    c: *mut xcb_connection_t,
    cmap: u32,
  ) -> xcb_xevie_end_cookie_t;

  pub fn xcb_xevie_end_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xevie_end_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xevie_end_reply_t;

  pub fn xcb_xevie_event_next(i: *mut xcb_xevie_event_iterator_t);

  pub fn xcb_xevie_event_end(i: xcb_xevie_event_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xevie_send(
    c: *mut xcb_connection_t,
    event: xcb_xevie_event_t,
    data_type: u32,
  ) -> xcb_xevie_send_cookie_t;

  pub fn xcb_xevie_send_unchecked(
    c: *mut xcb_connection_t,
    event: xcb_xevie_event_t,
    data_type: u32,
  ) -> xcb_xevie_send_cookie_t;

  pub fn xcb_xevie_send_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xevie_send_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xevie_send_reply_t;

  pub fn xcb_xevie_select_input(
    c: *mut xcb_connection_t,
    event_mask: u32,
  ) -> xcb_xevie_select_input_cookie_t;

  pub fn xcb_xevie_select_input_unchecked(
    c: *mut xcb_connection_t,
    event_mask: u32,
  ) -> xcb_xevie_select_input_cookie_t;

  pub fn xcb_xevie_select_input_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xevie_select_input_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xevie_select_input_reply_t;
}
