use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{
  xcb_pixmap_t,
  xcb_rectangle_iterator_t,
  xcb_rectangle_t,
  xcb_timestamp_t,
  xcb_window_t,
};

pub const XCB_SHAPE_MAJOR_VERSION: u32 = 1;
pub const XCB_SHAPE_MINOR_VERSION: u32 = 1;
pub const XCB_SHAPE_NOTIFY: u32 = 0;
pub const XCB_SHAPE_QUERY_VERSION: u32 = 0;
pub const XCB_SHAPE_RECTANGLES: u32 = 1;
pub const XCB_SHAPE_MASK: u32 = 2;
pub const XCB_SHAPE_COMBINE: u32 = 3;
pub const XCB_SHAPE_OFFSET: u32 = 4;
pub const XCB_SHAPE_QUERY_EXTENTS: u32 = 5;
pub const XCB_SHAPE_SELECT_INPUT: u32 = 6;
pub const XCB_SHAPE_INPUT_SELECTED: u32 = 7;
pub const XCB_SHAPE_GET_RECTANGLES: u32 = 8;

pub type xcb_shape_op_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_op_iterator_t
{
  pub data: *mut xcb_shape_op_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_shape_kind_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_kind_iterator_t
{
  pub data: *mut xcb_shape_kind_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_shape_so_t
{
  SET = 0,
  UNION = 1,
  INTERSECT = 2,
  SUBTRACT = 3,
  INVERT = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_shape_sk_t
{
  BOUNDING = 0,
  CLIP = 1,
  INPUT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_notify_event_t
{
  pub response_type: u8,
  pub shape_kind: xcb_shape_kind_t,
  pub sequence: u16,
  pub affected_window: xcb_window_t,
  pub extents_x: i16,
  pub extents_y: i16,
  pub extents_width: u16,
  pub extents_height: u16,
  pub server_time: xcb_timestamp_t,
  pub shaped: u8,
  pub pad0: [u8; 11usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u16,
  pub minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_rectangles_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub operation: xcb_shape_op_t,
  pub destination_kind: xcb_shape_kind_t,
  pub ordering: u8,
  pub pad0: u8,
  pub destination_window: xcb_window_t,
  pub x_offset: i16,
  pub y_offset: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_mask_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub operation: xcb_shape_op_t,
  pub destination_kind: xcb_shape_kind_t,
  pub pad0: [u8; 2usize],
  pub destination_window: xcb_window_t,
  pub x_offset: i16,
  pub y_offset: i16,
  pub source_bitmap: xcb_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_combine_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub operation: xcb_shape_op_t,
  pub destination_kind: xcb_shape_kind_t,
  pub source_kind: xcb_shape_kind_t,
  pub pad0: u8,
  pub destination_window: xcb_window_t,
  pub x_offset: i16,
  pub y_offset: i16,
  pub source_window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_offset_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub destination_kind: xcb_shape_kind_t,
  pub pad0: [u8; 3usize],
  pub destination_window: xcb_window_t,
  pub x_offset: i16,
  pub y_offset: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_extents_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_extents_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub destination_window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_query_extents_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub bounding_shaped: u8,
  pub clip_shaped: u8,
  pub pad1: [u8; 2usize],
  pub bounding_shape_extents_x: i16,
  pub bounding_shape_extents_y: i16,
  pub bounding_shape_extents_width: u16,
  pub bounding_shape_extents_height: u16,
  pub clip_shape_extents_x: i16,
  pub clip_shape_extents_y: i16,
  pub clip_shape_extents_width: u16,
  pub clip_shape_extents_height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub destination_window: xcb_window_t,
  pub enable: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_input_selected_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_input_selected_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub destination_window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_input_selected_reply_t
{
  pub response_type: u8,
  pub enabled: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_get_rectangles_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_get_rectangles_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub source_kind: xcb_shape_kind_t,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shape_get_rectangles_reply_t
{
  pub response_type: u8,
  pub ordering: u8,
  pub sequence: u16,
  pub length: u32,
  pub rectangles_len: u32,
  pub pad0: [u8; 20usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_shape_id: xcb_extension_t;

  pub fn xcb_shape_op_next(i: *mut xcb_shape_op_iterator_t);

  pub fn xcb_shape_op_end(i: xcb_shape_op_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_shape_kind_next(i: *mut xcb_shape_kind_iterator_t);

  pub fn xcb_shape_kind_end(i: xcb_shape_kind_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_shape_query_version(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t;

  pub fn xcb_shape_query_version_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_shape_query_version_cookie_t;

  pub fn xcb_shape_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shape_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shape_query_version_reply_t;

  pub fn xcb_shape_rectangles_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rectangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_shape_rectangles_checked(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    ordering: u8,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_rectangles(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    ordering: u8,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_rectangles_rectangles(
    R: *const xcb_shape_rectangles_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_shape_rectangles_rectangles_length(
    R: *const xcb_shape_rectangles_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_shape_rectangles_rectangles_iterator(
    R: *const xcb_shape_rectangles_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_shape_mask_checked(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    source_bitmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_mask(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    source_bitmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_combine_checked(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    source_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    source_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_combine(
    c: *mut xcb_connection_t,
    operation: xcb_shape_op_t,
    destination_kind: xcb_shape_kind_t,
    source_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
    source_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_offset_checked(
    c: *mut xcb_connection_t,
    destination_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_offset(
    c: *mut xcb_connection_t,
    destination_kind: xcb_shape_kind_t,
    destination_window: xcb_window_t,
    x_offset: i16,
    y_offset: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_query_extents(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
  ) -> xcb_shape_query_extents_cookie_t;

  pub fn xcb_shape_query_extents_unchecked(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
  ) -> xcb_shape_query_extents_cookie_t;

  pub fn xcb_shape_query_extents_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shape_query_extents_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shape_query_extents_reply_t;

  pub fn xcb_shape_select_input_checked(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
    enable: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_select_input(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
    enable: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shape_input_selected(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
  ) -> xcb_shape_input_selected_cookie_t;

  pub fn xcb_shape_input_selected_unchecked(
    c: *mut xcb_connection_t,
    destination_window: xcb_window_t,
  ) -> xcb_shape_input_selected_cookie_t;

  pub fn xcb_shape_input_selected_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shape_input_selected_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shape_input_selected_reply_t;

  pub fn xcb_shape_get_rectangles_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_shape_get_rectangles(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    source_kind: xcb_shape_kind_t,
  ) -> xcb_shape_get_rectangles_cookie_t;

  pub fn xcb_shape_get_rectangles_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    source_kind: xcb_shape_kind_t,
  ) -> xcb_shape_get_rectangles_cookie_t;

  pub fn xcb_shape_get_rectangles_rectangles(
    R: *const xcb_shape_get_rectangles_reply_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_shape_get_rectangles_rectangles_length(
    R: *const xcb_shape_get_rectangles_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_shape_get_rectangles_rectangles_iterator(
    R: *const xcb_shape_get_rectangles_reply_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_shape_get_rectangles_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shape_get_rectangles_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shape_get_rectangles_reply_t;
}
