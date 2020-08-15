use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_rectangle_t, xcb_timestamp_t};
use super::xfixes::xcb_xfixes_region_t;

pub const XCB_DAMAGE_MAJOR_VERSION: u32 = 1;
pub const XCB_DAMAGE_MINOR_VERSION: u32 = 1;
pub const XCB_DAMAGE_BAD_DAMAGE: u32 = 0;
pub const XCB_DAMAGE_QUERY_VERSION: u32 = 0;
pub const XCB_DAMAGE_CREATE: u32 = 1;
pub const XCB_DAMAGE_DESTROY: u32 = 2;
pub const XCB_DAMAGE_SUBTRACT: u32 = 3;
pub const XCB_DAMAGE_ADD: u32 = 4;
pub const XCB_DAMAGE_NOTIFY: u32 = 0;

pub type xcb_damage_damage_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_damage_iterator_t
{
  pub data: *mut xcb_damage_damage_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_damage_report_level_t
{
  RAW_RECTANGLES = 0,
  DELTA_RECTANGLES = 1,
  BOUNDING_BOX = 2,
  NON_EMPTY = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_bad_damage_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u32,
  pub client_minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u32,
  pub minor_version: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_create_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub damage: xcb_damage_damage_t,
  pub drawable: xcb_drawable_t,
  pub level: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_destroy_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub damage: xcb_damage_damage_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_subtract_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub damage: xcb_damage_damage_t,
  pub repair: xcb_xfixes_region_t,
  pub parts: xcb_xfixes_region_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_add_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub region: xcb_xfixes_region_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_damage_notify_event_t
{
  pub response_type: u8,
  pub level: u8,
  pub sequence: u16,
  pub drawable: xcb_drawable_t,
  pub damage: xcb_damage_damage_t,
  pub timestamp: xcb_timestamp_t,
  pub area: xcb_rectangle_t,
  pub geometry: xcb_rectangle_t,
}

#[link(name = "xcb-damage")]
extern "C" {
  pub static mut xcb_damage_id: xcb_extension_t;

  pub fn xcb_damage_damage_next(i: *mut xcb_damage_damage_iterator_t);

  pub fn xcb_damage_damage_end(i: xcb_damage_damage_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_damage_query_version(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_damage_query_version_cookie_t;

  pub fn xcb_damage_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_damage_query_version_cookie_t;

  pub fn xcb_damage_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_damage_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_damage_query_version_reply_t;

  pub fn xcb_damage_create_checked(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
    drawable: xcb_drawable_t,
    level: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_create(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
    drawable: xcb_drawable_t,
    level: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_destroy_checked(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_destroy(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_subtract_checked(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
    repair: xcb_xfixes_region_t,
    parts: xcb_xfixes_region_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_subtract(
    c: *mut xcb_connection_t,
    damage: xcb_damage_damage_t,
    repair: xcb_xfixes_region_t,
    parts: xcb_xfixes_region_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_add_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    region: xcb_xfixes_region_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_damage_add(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    region: xcb_xfixes_region_t,
  ) -> xcb_void_cookie_t;
}
