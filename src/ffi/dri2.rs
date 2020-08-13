use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_window_t};

pub const XCB_DRI2_MAJOR_VERSION: u32 = 1;
pub const XCB_DRI2_MINOR_VERSION: u32 = 4;
pub const XCB_DRI2_QUERY_VERSION: u32 = 0;
pub const XCB_DRI2_CONNECT: u32 = 1;
pub const XCB_DRI2_AUTHENTICATE: u32 = 2;
pub const XCB_DRI2_CREATE_DRAWABLE: u32 = 3;
pub const XCB_DRI2_DESTROY_DRAWABLE: u32 = 4;
pub const XCB_DRI2_GET_BUFFERS: u32 = 5;
pub const XCB_DRI2_COPY_REGION: u32 = 6;
pub const XCB_DRI2_GET_BUFFERS_WITH_FORMAT: u32 = 7;
pub const XCB_DRI2_SWAP_BUFFERS: u32 = 8;
pub const XCB_DRI2_GET_MSC: u32 = 9;
pub const XCB_DRI2_WAIT_MSC: u32 = 10;
pub const XCB_DRI2_WAIT_SBC: u32 = 11;
pub const XCB_DRI2_SWAP_INTERVAL: u32 = 12;
pub const XCB_DRI2_GET_PARAM: u32 = 13;
pub const XCB_DRI2_BUFFER_SWAP_COMPLETE: u32 = 0;
pub const XCB_DRI2_INVALIDATE_BUFFERS: u32 = 1;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_dri2_attachment_t
{
  BUFFER_FRONT_LEFT = 0,
  BUFFER_BACK_LEFT = 1,
  BUFFER_FRONT_RIGHT = 2,
  BUFFER_BACK_RIGHT = 3,
  BUFFER_DEPTH = 4,
  BUFFER_STENCIL = 5,
  BUFFER_ACCUM = 6,
  BUFFER_FAKE_FRONT_LEFT = 7,
  BUFFER_FAKE_FRONT_RIGHT = 8,
  BUFFER_DEPTH_STENCIL = 9,
  BUFFER_HIZ = 10,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_dri2_driver_type_t
{
  DRI = 0,
  VDPAU = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_dri2_event_type_t
{
  EXCHANGE_COMPLETE = 1,
  BLIT_COMPLETE = 2,
  FLIP_COMPLETE = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_dri2_buffer_t
{
  pub attachment: u32,
  pub name: u32,
  pub pitch: u32,
  pub cpp: u32,
  pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_dri2_buffer_iterator_t
{
  pub data: *mut xcb_dri2_dri2_buffer_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_attach_format_t
{
  pub attachment: u32,
  pub format: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_attach_format_iterator_t
{
  pub data: *mut xcb_dri2_attach_format_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_connect_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_connect_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub driver_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_connect_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub driver_name_length: u32,
  pub device_name_length: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_authenticate_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_authenticate_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub magic: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_authenticate_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub authenticated: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_create_drawable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_destroy_drawable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u32,
  pub height: u32,
  pub count: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_copy_region_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_copy_region_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub region: u32,
  pub dest: u32,
  pub src: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_copy_region_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_with_format_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_with_format_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_buffers_with_format_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u32,
  pub height: u32,
  pub count: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_swap_buffers_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_swap_buffers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub target_msc_hi: u32,
  pub target_msc_lo: u32,
  pub divisor_hi: u32,
  pub divisor_lo: u32,
  pub remainder_hi: u32,
  pub remainder_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_swap_buffers_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub swap_hi: u32,
  pub swap_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_msc_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_msc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_msc_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ust_hi: u32,
  pub ust_lo: u32,
  pub msc_hi: u32,
  pub msc_lo: u32,
  pub sbc_hi: u32,
  pub sbc_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_msc_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_msc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub target_msc_hi: u32,
  pub target_msc_lo: u32,
  pub divisor_hi: u32,
  pub divisor_lo: u32,
  pub remainder_hi: u32,
  pub remainder_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_msc_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ust_hi: u32,
  pub ust_lo: u32,
  pub msc_hi: u32,
  pub msc_lo: u32,
  pub sbc_hi: u32,
  pub sbc_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_sbc_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_sbc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub target_sbc_hi: u32,
  pub target_sbc_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_wait_sbc_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ust_hi: u32,
  pub ust_lo: u32,
  pub msc_hi: u32,
  pub msc_lo: u32,
  pub sbc_hi: u32,
  pub sbc_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_swap_interval_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub interval: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_param_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_param_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub param: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_get_param_reply_t
{
  pub response_type: u8,
  pub is_param_recognized: u8,
  pub sequence: u16,
  pub length: u32,
  pub value_hi: u32,
  pub value_lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_buffer_swap_complete_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event_type: u16,
  pub pad1: [u8; 2usize],
  pub drawable: xcb_drawable_t,
  pub ust_hi: u32,
  pub ust_lo: u32,
  pub msc_hi: u32,
  pub msc_lo: u32,
  pub sbc: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri2_invalidate_buffers_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub drawable: xcb_drawable_t,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_dri2_id: xcb_extension_t;

  pub fn xcb_dri2_dri2_buffer_next(i: *mut xcb_dri2_dri2_buffer_iterator_t);

  pub fn xcb_dri2_dri2_buffer_end(i: xcb_dri2_dri2_buffer_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_dri2_attach_format_next(i: *mut xcb_dri2_attach_format_iterator_t);

  pub fn xcb_dri2_attach_format_end(i: xcb_dri2_attach_format_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_dri2_query_version(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_dri2_query_version_cookie_t;

  pub fn xcb_dri2_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_dri2_query_version_cookie_t;

  pub fn xcb_dri2_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_query_version_reply_t;

  pub fn xcb_dri2_connect_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_connect(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    driver_type: u32,
  ) -> xcb_dri2_connect_cookie_t;

  pub fn xcb_dri2_connect_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    driver_type: u32,
  ) -> xcb_dri2_connect_cookie_t;

  pub fn xcb_dri2_connect_driver_name(
    R: *const xcb_dri2_connect_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_dri2_connect_driver_name_length(
    R: *const xcb_dri2_connect_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_connect_driver_name_end(
    R: *const xcb_dri2_connect_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri2_connect_alignment_pad(
    R: *const xcb_dri2_connect_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_dri2_connect_alignment_pad_length(
    R: *const xcb_dri2_connect_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_connect_alignment_pad_end(
    R: *const xcb_dri2_connect_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri2_connect_device_name(
    R: *const xcb_dri2_connect_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_dri2_connect_device_name_length(
    R: *const xcb_dri2_connect_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_connect_device_name_end(
    R: *const xcb_dri2_connect_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri2_connect_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_connect_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_connect_reply_t;

  pub fn xcb_dri2_authenticate(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    magic: u32,
  ) -> xcb_dri2_authenticate_cookie_t;

  pub fn xcb_dri2_authenticate_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    magic: u32,
  ) -> xcb_dri2_authenticate_cookie_t;

  pub fn xcb_dri2_authenticate_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_authenticate_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_authenticate_reply_t;

  pub fn xcb_dri2_create_drawable_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_create_drawable(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_destroy_drawable_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_destroy_drawable(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_get_buffers_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    attachments_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_get_buffers(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    count: u32,
    attachments_len: u32,
    attachments: *const u32,
  ) -> xcb_dri2_get_buffers_cookie_t;

  pub fn xcb_dri2_get_buffers_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    count: u32,
    attachments_len: u32,
    attachments: *const u32,
  ) -> xcb_dri2_get_buffers_cookie_t;

  pub fn xcb_dri2_get_buffers_buffers(
    R: *const xcb_dri2_get_buffers_reply_t
  ) -> *mut xcb_dri2_dri2_buffer_t;

  pub fn xcb_dri2_get_buffers_buffers_length(
    R: *const xcb_dri2_get_buffers_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_get_buffers_buffers_iterator(
    R: *const xcb_dri2_get_buffers_reply_t
  ) -> xcb_dri2_dri2_buffer_iterator_t;

  pub fn xcb_dri2_get_buffers_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_get_buffers_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_get_buffers_reply_t;

  pub fn xcb_dri2_copy_region(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    region: u32,
    dest: u32,
    src: u32,
  ) -> xcb_dri2_copy_region_cookie_t;

  pub fn xcb_dri2_copy_region_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    region: u32,
    dest: u32,
    src: u32,
  ) -> xcb_dri2_copy_region_cookie_t;

  pub fn xcb_dri2_copy_region_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_copy_region_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_copy_region_reply_t;

  pub fn xcb_dri2_get_buffers_with_format_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    attachments_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_get_buffers_with_format(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    count: u32,
    attachments_len: u32,
    attachments: *const xcb_dri2_attach_format_t,
  ) -> xcb_dri2_get_buffers_with_format_cookie_t;

  pub fn xcb_dri2_get_buffers_with_format_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    count: u32,
    attachments_len: u32,
    attachments: *const xcb_dri2_attach_format_t,
  ) -> xcb_dri2_get_buffers_with_format_cookie_t;

  pub fn xcb_dri2_get_buffers_with_format_buffers(
    R: *const xcb_dri2_get_buffers_with_format_reply_t
  ) -> *mut xcb_dri2_dri2_buffer_t;

  pub fn xcb_dri2_get_buffers_with_format_buffers_length(
    R: *const xcb_dri2_get_buffers_with_format_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri2_get_buffers_with_format_buffers_iterator(
    R: *const xcb_dri2_get_buffers_with_format_reply_t
  ) -> xcb_dri2_dri2_buffer_iterator_t;

  pub fn xcb_dri2_get_buffers_with_format_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_get_buffers_with_format_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_get_buffers_with_format_reply_t;

  pub fn xcb_dri2_swap_buffers(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
  ) -> xcb_dri2_swap_buffers_cookie_t;

  pub fn xcb_dri2_swap_buffers_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
  ) -> xcb_dri2_swap_buffers_cookie_t;

  pub fn xcb_dri2_swap_buffers_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_swap_buffers_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_swap_buffers_reply_t;

  pub fn xcb_dri2_get_msc(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_dri2_get_msc_cookie_t;

  pub fn xcb_dri2_get_msc_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_dri2_get_msc_cookie_t;

  pub fn xcb_dri2_get_msc_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_get_msc_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_get_msc_reply_t;

  pub fn xcb_dri2_wait_msc(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
  ) -> xcb_dri2_wait_msc_cookie_t;

  pub fn xcb_dri2_wait_msc_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_msc_hi: u32,
    target_msc_lo: u32,
    divisor_hi: u32,
    divisor_lo: u32,
    remainder_hi: u32,
    remainder_lo: u32,
  ) -> xcb_dri2_wait_msc_cookie_t;

  pub fn xcb_dri2_wait_msc_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_wait_msc_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_wait_msc_reply_t;

  pub fn xcb_dri2_wait_sbc(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_sbc_hi: u32,
    target_sbc_lo: u32,
  ) -> xcb_dri2_wait_sbc_cookie_t;

  pub fn xcb_dri2_wait_sbc_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    target_sbc_hi: u32,
    target_sbc_lo: u32,
  ) -> xcb_dri2_wait_sbc_cookie_t;

  pub fn xcb_dri2_wait_sbc_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_wait_sbc_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_wait_sbc_reply_t;

  pub fn xcb_dri2_swap_interval_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    interval: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_swap_interval(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    interval: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri2_get_param(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    param: u32,
  ) -> xcb_dri2_get_param_cookie_t;

  pub fn xcb_dri2_get_param_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    param: u32,
  ) -> xcb_dri2_get_param_cookie_t;

  pub fn xcb_dri2_get_param_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri2_get_param_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri2_get_param_reply_t;
}
