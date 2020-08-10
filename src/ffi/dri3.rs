use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_pixmap_t, xcb_window_t};

pub const XCB_DRI3_MAJOR_VERSION: u32 = 1;
pub const XCB_DRI3_MINOR_VERSION: u32 = 2;
pub const XCB_DRI3_QUERY_VERSION: u32 = 0;
pub const XCB_DRI3_OPEN: u32 = 1;
pub const XCB_DRI3_PIXMAP_FROM_BUFFER: u32 = 2;
pub const XCB_DRI3_BUFFER_FROM_PIXMAP: u32 = 3;
pub const XCB_DRI3_FENCE_FROM_FD: u32 = 4;
pub const XCB_DRI3_FD_FROM_FENCE: u32 = 5;
pub const XCB_DRI3_GET_SUPPORTED_MODIFIERS: u32 = 6;
pub const XCB_DRI3_PIXMAP_FROM_BUFFERS: u32 = 7;
pub const XCB_DRI3_BUFFERS_FROM_PIXMAP: u32 = 8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_query_version_reply_t
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
pub struct xcb_dri3_open_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_open_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub provider: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_open_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_pixmap_from_buffer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pixmap: xcb_pixmap_t,
  pub drawable: xcb_drawable_t,
  pub size: u32,
  pub width: u16,
  pub height: u16,
  pub stride: u16,
  pub depth: u8,
  pub bpp: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffer_from_pixmap_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffer_from_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pixmap: xcb_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffer_from_pixmap_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub size: u32,
  pub width: u16,
  pub height: u16,
  pub stride: u16,
  pub depth: u8,
  pub bpp: u8,
  pub pad0: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_fence_from_fd_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub fence: u32,
  pub initially_triggered: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_fd_from_fence_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_fd_from_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub fence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_fd_from_fence_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_get_supported_modifiers_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_get_supported_modifiers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: u32,
  pub depth: u8,
  pub bpp: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_get_supported_modifiers_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_window_modifiers: u32,
  pub num_screen_modifiers: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_pixmap_from_buffers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pixmap: xcb_pixmap_t,
  pub window: xcb_window_t,
  pub num_buffers: u8,
  pub pad0: [u8; 3usize],
  pub width: u16,
  pub height: u16,
  pub stride0: u32,
  pub offset0: u32,
  pub stride1: u32,
  pub offset1: u32,
  pub stride2: u32,
  pub offset2: u32,
  pub stride3: u32,
  pub offset3: u32,
  pub depth: u8,
  pub bpp: u8,
  pub pad1: [u8; 2usize],
  pub modifier: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffers_from_pixmap_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffers_from_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pixmap: xcb_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_dri3_buffers_from_pixmap_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u16,
  pub height: u16,
  pub pad0: [u8; 4usize],
  pub modifier: u64,
  pub depth: u8,
  pub bpp: u8,
  pub pad1: [u8; 6usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_dri3_id: xcb_extension_t;

  pub fn xcb_dri3_query_version(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_dri3_query_version_cookie_t;

  pub fn xcb_dri3_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_dri3_query_version_cookie_t;

  pub fn xcb_dri3_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_query_version_reply_t;

  pub fn xcb_dri3_open(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    provider: u32,
  ) -> xcb_dri3_open_cookie_t;

  pub fn xcb_dri3_open_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    provider: u32,
  ) -> xcb_dri3_open_cookie_t;

  pub fn xcb_dri3_open_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_open_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_open_reply_t;

  pub fn xcb_dri3_open_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_dri3_open_reply_t,
  ) -> *mut ::std::os::raw::c_int;

  pub fn xcb_dri3_pixmap_from_buffer_checked(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    size: u32,
    width: u16,
    height: u16,
    stride: u16,
    depth: u8,
    bpp: u8,
    pixmap_fd: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_pixmap_from_buffer(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    size: u32,
    width: u16,
    height: u16,
    stride: u16,
    depth: u8,
    bpp: u8,
    pixmap_fd: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_buffer_from_pixmap(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_dri3_buffer_from_pixmap_cookie_t;

  pub fn xcb_dri3_buffer_from_pixmap_unchecked(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_dri3_buffer_from_pixmap_cookie_t;

  pub fn xcb_dri3_buffer_from_pixmap_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_buffer_from_pixmap_reply_t;

  pub fn xcb_dri3_buffer_from_pixmap_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_dri3_buffer_from_pixmap_reply_t,
  ) -> *mut ::std::os::raw::c_int;

  pub fn xcb_dri3_fence_from_fd_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: u32,
    initially_triggered: u8,
    fence_fd: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_fence_from_fd(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: u32,
    initially_triggered: u8,
    fence_fd: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_fd_from_fence(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: u32,
  ) -> xcb_dri3_fd_from_fence_cookie_t;

  pub fn xcb_dri3_fd_from_fence_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: u32,
  ) -> xcb_dri3_fd_from_fence_cookie_t;

  pub fn xcb_dri3_fd_from_fence_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_fd_from_fence_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_fd_from_fence_reply_t;

  pub fn xcb_dri3_fd_from_fence_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_dri3_fd_from_fence_reply_t,
  ) -> *mut ::std::os::raw::c_int;

  pub fn xcb_dri3_get_supported_modifiers_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_get_supported_modifiers(
    c: *mut xcb_connection_t,
    window: u32,
    depth: u8,
    bpp: u8,
  ) -> xcb_dri3_get_supported_modifiers_cookie_t;

  pub fn xcb_dri3_get_supported_modifiers_unchecked(
    c: *mut xcb_connection_t,
    window: u32,
    depth: u8,
    bpp: u8,
  ) -> xcb_dri3_get_supported_modifiers_cookie_t;

  pub fn xcb_dri3_get_supported_modifiers_window_modifiers(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> *mut u64;

  pub fn xcb_dri3_get_supported_modifiers_window_modifiers_length(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_get_supported_modifiers_window_modifiers_end(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri3_get_supported_modifiers_screen_modifiers(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> *mut u64;

  pub fn xcb_dri3_get_supported_modifiers_screen_modifiers_length(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_get_supported_modifiers_screen_modifiers_end(
    R: *const xcb_dri3_get_supported_modifiers_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri3_get_supported_modifiers_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_get_supported_modifiers_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_get_supported_modifiers_reply_t;

  pub fn xcb_dri3_pixmap_from_buffers_checked(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
    window: xcb_window_t,
    num_buffers: u8,
    width: u16,
    height: u16,
    stride0: u32,
    offset0: u32,
    stride1: u32,
    offset1: u32,
    stride2: u32,
    offset2: u32,
    stride3: u32,
    offset3: u32,
    depth: u8,
    bpp: u8,
    modifier: u64,
    buffers: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_pixmap_from_buffers(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
    window: xcb_window_t,
    num_buffers: u8,
    width: u16,
    height: u16,
    stride0: u32,
    offset0: u32,
    stride1: u32,
    offset1: u32,
    stride2: u32,
    offset2: u32,
    stride3: u32,
    offset3: u32,
    depth: u8,
    bpp: u8,
    modifier: u64,
    buffers: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_dri3_buffers_from_pixmap_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    buffers: i32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_buffers_from_pixmap(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_dri3_buffers_from_pixmap_cookie_t;

  pub fn xcb_dri3_buffers_from_pixmap_unchecked(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_dri3_buffers_from_pixmap_cookie_t;

  pub fn xcb_dri3_buffers_from_pixmap_strides(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> *mut u32;

  pub fn xcb_dri3_buffers_from_pixmap_strides_length(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_buffers_from_pixmap_strides_end(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri3_buffers_from_pixmap_offsets(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> *mut u32;

  pub fn xcb_dri3_buffers_from_pixmap_offsets_length(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_buffers_from_pixmap_offsets_end(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri3_buffers_from_pixmap_buffers(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> *mut i32;

  pub fn xcb_dri3_buffers_from_pixmap_buffers_length(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_dri3_buffers_from_pixmap_buffers_end(
    R: *const xcb_dri3_buffers_from_pixmap_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_dri3_buffers_from_pixmap_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_dri3_buffers_from_pixmap_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_dri3_buffers_from_pixmap_reply_t;

  pub fn xcb_dri3_buffers_from_pixmap_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_dri3_buffers_from_pixmap_reply_t,
  ) -> *mut ::std::os::raw::c_int;
}
