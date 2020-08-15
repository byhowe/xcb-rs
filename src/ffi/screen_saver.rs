use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_void_cookie_t};
use super::xcb::{
  xcb_bool32_t,
  xcb_colormap_t,
  xcb_cursor_t,
  xcb_drawable_t,
  xcb_pixmap_t,
  xcb_timestamp_t,
  xcb_visualid_t,
  xcb_window_t,
};

pub const XCB_SCREENSAVER_MAJOR_VERSION: u32 = 1;
pub const XCB_SCREENSAVER_MINOR_VERSION: u32 = 1;
pub const XCB_SCREENSAVER_QUERY_VERSION: u32 = 0;
pub const XCB_SCREENSAVER_QUERY_INFO: u32 = 1;
pub const XCB_SCREENSAVER_SELECT_INPUT: u32 = 2;
pub const XCB_SCREENSAVER_SET_ATTRIBUTES: u32 = 3;
pub const XCB_SCREENSAVER_UNSET_ATTRIBUTES: u32 = 4;
pub const XCB_SCREENSAVER_SUSPEND: u32 = 5;
pub const XCB_SCREENSAVER_NOTIFY: u32 = 0;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_screensaver_kind_t
{
  BLANKED = 0,
  INTERNAL = 1,
  EXTERNAL = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_screensaver_event_t
{
  NOTIFY_MASK = 1,
  CYCLE_MASK = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_screensaver_state_t
{
  OFF = 0,
  ON = 1,
  CYCLE = 2,
  DISABLED = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u8,
  pub client_minor_version: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_query_version_reply_t
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
pub struct xcb_screensaver_query_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_query_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_query_info_reply_t
{
  pub response_type: u8,
  pub state: u8,
  pub sequence: u16,
  pub length: u32,
  pub saver_window: xcb_window_t,
  pub ms_until_server: u32,
  pub ms_since_user_input: u32,
  pub event_mask: u32,
  pub kind: u8,
  pub pad0: [u8; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub event_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_set_attributes_value_list_t
{
  pub background_pixmap: xcb_pixmap_t,
  pub background_pixel: u32,
  pub border_pixmap: xcb_pixmap_t,
  pub border_pixel: u32,
  pub bit_gravity: u32,
  pub win_gravity: u32,
  pub backing_store: u32,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub override_redirect: xcb_bool32_t,
  pub save_under: xcb_bool32_t,
  pub event_mask: u32,
  pub do_not_propogate_mask: u32,
  pub colormap: xcb_colormap_t,
  pub cursor: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_set_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub _class: u8,
  pub depth: u8,
  pub visual: xcb_visualid_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_unset_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_suspend_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub suspend: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screensaver_notify_event_t
{
  pub response_type: u8,
  pub state: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub root: xcb_window_t,
  pub window: xcb_window_t,
  pub kind: u8,
  pub forced: u8,
  pub pad0: [u8; 14usize],
}

#[link(name = "xcb-screensaver")]
extern "C" {
  pub static mut xcb_screensaver_id: xcb_extension_t;

  pub fn xcb_screensaver_query_version(
    c: *mut xcb_connection_t,
    client_major_version: u8,
    client_minor_version: u8,
  ) -> xcb_screensaver_query_version_cookie_t;

  pub fn xcb_screensaver_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u8,
    client_minor_version: u8,
  ) -> xcb_screensaver_query_version_cookie_t;

  pub fn xcb_screensaver_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_screensaver_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_screensaver_query_version_reply_t;

  pub fn xcb_screensaver_query_info(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_screensaver_query_info_cookie_t;

  pub fn xcb_screensaver_query_info_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_screensaver_query_info_cookie_t;

  pub fn xcb_screensaver_query_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_screensaver_query_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_screensaver_query_info_reply_t;

  pub fn xcb_screensaver_select_input_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_select_input(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_set_attributes_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_screensaver_set_attributes_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_screensaver_set_attributes_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_screensaver_set_attributes_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_screensaver_set_attributes_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_screensaver_set_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_screensaver_set_attributes_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u8,
    depth: u8,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_set_attributes(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u8,
    depth: u8,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_set_attributes_aux_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u8,
    depth: u8,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const xcb_screensaver_set_attributes_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_set_attributes_aux(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u8,
    depth: u8,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const xcb_screensaver_set_attributes_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_set_attributes_value_list(
    R: *const xcb_screensaver_set_attributes_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_screensaver_unset_attributes_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_unset_attributes(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_suspend_checked(
    c: *mut xcb_connection_t,
    suspend: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_screensaver_suspend(
    c: *mut xcb_connection_t,
    suspend: u32,
  ) -> xcb_void_cookie_t;
}
