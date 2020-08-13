use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_void_cookie_t};
use super::xcb::{xcb_pixmap_t, xcb_window_t};
use super::xfixes::xcb_xfixes_region_t;

pub const XCB_COMPOSITE_MAJOR_VERSION: u32 = 0;
pub const XCB_COMPOSITE_MINOR_VERSION: u32 = 4;
pub const XCB_COMPOSITE_QUERY_VERSION: u32 = 0;
pub const XCB_COMPOSITE_REDIRECT_WINDOW: u32 = 1;
pub const XCB_COMPOSITE_REDIRECT_SUBWINDOWS: u32 = 2;
pub const XCB_COMPOSITE_UNREDIRECT_WINDOW: u32 = 3;
pub const XCB_COMPOSITE_UNREDIRECT_SUBWINDOWS: u32 = 4;
pub const XCB_COMPOSITE_CREATE_REGION_FROM_BORDER_CLIP: u32 = 5;
pub const XCB_COMPOSITE_NAME_WINDOW_PIXMAP: u32 = 6;
pub const XCB_COMPOSITE_GET_OVERLAY_WINDOW: u32 = 7;
pub const XCB_COMPOSITE_RELEASE_OVERLAY_WINDOW: u32 = 8;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_composite_redirect_t
{
  AUTOMATIC = 0,
  MANUAL = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u32,
  pub client_minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_query_version_reply_t
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
pub struct xcb_composite_redirect_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub update: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_redirect_subwindows_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub update: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_unredirect_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub update: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_unredirect_subwindows_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub update: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_create_region_from_border_clip_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub region: xcb_xfixes_region_t,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_name_window_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub pixmap: xcb_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_get_overlay_window_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_get_overlay_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_get_overlay_window_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub overlay_win: xcb_window_t,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_composite_release_overlay_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_composite_id: xcb_extension_t;

  pub fn xcb_composite_query_version(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_composite_query_version_cookie_t;

  pub fn xcb_composite_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_composite_query_version_cookie_t;

  pub fn xcb_composite_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_composite_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_composite_query_version_reply_t;

  pub fn xcb_composite_redirect_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_redirect_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_redirect_subwindows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_redirect_subwindows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_unredirect_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_unredirect_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_unredirect_subwindows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_unredirect_subwindows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    update: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_create_region_from_border_clip_checked(
    c: *mut xcb_connection_t,
    region: xcb_xfixes_region_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_create_region_from_border_clip(
    c: *mut xcb_connection_t,
    region: xcb_xfixes_region_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_name_window_pixmap_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_name_window_pixmap(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_get_overlay_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_composite_get_overlay_window_cookie_t;

  pub fn xcb_composite_get_overlay_window_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_composite_get_overlay_window_cookie_t;

  pub fn xcb_composite_get_overlay_window_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_composite_get_overlay_window_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_composite_get_overlay_window_reply_t;

  pub fn xcb_composite_release_overlay_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_composite_release_overlay_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;
}
