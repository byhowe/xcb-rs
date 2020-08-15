use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};

pub const XCB_XF86DRI_MAJOR_VERSION: u32 = 4;
pub const XCB_XF86DRI_MINOR_VERSION: u32 = 1;
pub const XCB_XF86DRI_QUERY_VERSION: u32 = 0;
pub const XCB_XF86DRI_QUERY_DIRECT_RENDERING_CAPABLE: u32 = 1;
pub const XCB_XF86DRI_OPEN_CONNECTION: u32 = 2;
pub const XCB_XF86DRI_CLOSE_CONNECTION: u32 = 3;
pub const XCB_XF86DRI_GET_CLIENT_DRIVER_NAME: u32 = 4;
pub const XCB_XF86DRI_CREATE_CONTEXT: u32 = 5;
pub const XCB_XF86DRI_DESTROY_CONTEXT: u32 = 6;
pub const XCB_XF86DRI_CREATE_DRAWABLE: u32 = 7;
pub const XCB_XF86DRI_DESTROY_DRAWABLE: u32 = 8;
pub const XCB_XF86DRI_GET_DRAWABLE_INFO: u32 = 9;
pub const XCB_XF86DRI_GET_DEVICE_INFO: u32 = 10;
pub const XCB_XF86DRI_AUTH_CONNECTION: u32 = 11;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_drm_clip_rect_t
{
  pub x1: i16,
  pub y1: i16,
  pub x2: i16,
  pub x3: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_drm_clip_rect_iterator_t
{
  pub data: *mut xcb_xf86dri_drm_clip_rect_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub dri_major_version: u16,
  pub dri_minor_version: u16,
  pub dri_minor_patch: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_direct_rendering_capable_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_direct_rendering_capable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_query_direct_rendering_capable_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub is_capable: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_open_connection_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_open_connection_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_open_connection_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub sarea_handle_low: u32,
  pub sarea_handle_high: u32,
  pub bus_id_len: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_close_connection_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_client_driver_name_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_client_driver_name_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_client_driver_name_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub client_driver_major_version: u32,
  pub client_driver_minor_version: u32,
  pub client_driver_patch_version: u32,
  pub client_driver_name_len: u32,
  pub pad1: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub visual: u32,
  pub context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub hw_context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_destroy_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_drawable_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_drawable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub drawable: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_create_drawable_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub hw_drawable_handle: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_destroy_drawable_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub drawable: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_drawable_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_drawable_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub drawable: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_drawable_info_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub drawable_table_index: u32,
  pub drawable_table_stamp: u32,
  pub drawable_origin_X: i16,
  pub drawable_origin_Y: i16,
  pub drawable_size_W: i16,
  pub drawable_size_H: i16,
  pub num_clip_rects: u32,
  pub back_x: i16,
  pub back_y: i16,
  pub num_back_clip_rects: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_device_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_device_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_get_device_info_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub framebuffer_handle_low: u32,
  pub framebuffer_handle_high: u32,
  pub framebuffer_origin_offset: u32,
  pub framebuffer_size: u32,
  pub framebuffer_stride: u32,
  pub device_private_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_auth_connection_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_auth_connection_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub magic: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xf86dri_auth_connection_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub authenticated: u32,
}

#[link(name = "xcb-xf86dri")]
extern "C" {
  pub static mut xcb_xf86dri_id: xcb_extension_t;

  pub fn xcb_xf86dri_drm_clip_rect_next(i: *mut xcb_xf86dri_drm_clip_rect_iterator_t);

  pub fn xcb_xf86dri_drm_clip_rect_end(
    i: xcb_xf86dri_drm_clip_rect_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xf86dri_query_version(c: *mut xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t;

  pub fn xcb_xf86dri_query_version_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_xf86dri_query_version_cookie_t;

  pub fn xcb_xf86dri_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_query_version_reply_t;

  pub fn xcb_xf86dri_query_direct_rendering_capable(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t;

  pub fn xcb_xf86dri_query_direct_rendering_capable_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t;

  pub fn xcb_xf86dri_query_direct_rendering_capable_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t;

  pub fn xcb_xf86dri_open_connection_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_open_connection(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_open_connection_cookie_t;

  pub fn xcb_xf86dri_open_connection_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_open_connection_cookie_t;

  pub fn xcb_xf86dri_open_connection_bus_id(
    R: *const xcb_xf86dri_open_connection_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xf86dri_open_connection_bus_id_length(
    R: *const xcb_xf86dri_open_connection_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_open_connection_bus_id_end(
    R: *const xcb_xf86dri_open_connection_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xf86dri_open_connection_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_open_connection_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_open_connection_reply_t;

  pub fn xcb_xf86dri_close_connection_checked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_close_connection(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_get_client_driver_name_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_client_driver_name(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_get_client_driver_name_cookie_t;

  pub fn xcb_xf86dri_get_client_driver_name_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_get_client_driver_name_cookie_t;

  pub fn xcb_xf86dri_get_client_driver_name_client_driver_name(
    R: *const xcb_xf86dri_get_client_driver_name_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_length(
    R: *const xcb_xf86dri_get_client_driver_name_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_end(
    R: *const xcb_xf86dri_get_client_driver_name_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xf86dri_get_client_driver_name_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_get_client_driver_name_reply_t;

  pub fn xcb_xf86dri_create_context(
    c: *mut xcb_connection_t,
    screen: u32,
    visual: u32,
    context: u32,
  ) -> xcb_xf86dri_create_context_cookie_t;

  pub fn xcb_xf86dri_create_context_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
    visual: u32,
    context: u32,
  ) -> xcb_xf86dri_create_context_cookie_t;

  pub fn xcb_xf86dri_create_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_create_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_create_context_reply_t;

  pub fn xcb_xf86dri_destroy_context_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_destroy_context(
    c: *mut xcb_connection_t,
    screen: u32,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_create_drawable(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_xf86dri_create_drawable_cookie_t;

  pub fn xcb_xf86dri_create_drawable_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_xf86dri_create_drawable_cookie_t;

  pub fn xcb_xf86dri_create_drawable_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_create_drawable_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_create_drawable_reply_t;

  pub fn xcb_xf86dri_destroy_drawable_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_destroy_drawable(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xf86dri_get_drawable_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_drawable_info(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_xf86dri_get_drawable_info_cookie_t;

  pub fn xcb_xf86dri_get_drawable_info_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
    drawable: u32,
  ) -> xcb_xf86dri_get_drawable_info_cookie_t;

  pub fn xcb_xf86dri_get_drawable_info_clip_rects(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> *mut xcb_xf86dri_drm_clip_rect_t;

  pub fn xcb_xf86dri_get_drawable_info_clip_rects_length(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_drawable_info_clip_rects_iterator(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> xcb_xf86dri_drm_clip_rect_iterator_t;

  pub fn xcb_xf86dri_get_drawable_info_back_clip_rects(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> *mut xcb_xf86dri_drm_clip_rect_t;

  pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_length(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator(
    R: *const xcb_xf86dri_get_drawable_info_reply_t
  ) -> xcb_xf86dri_drm_clip_rect_iterator_t;

  pub fn xcb_xf86dri_get_drawable_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_get_drawable_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_get_drawable_info_reply_t;

  pub fn xcb_xf86dri_get_device_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_device_info(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_get_device_info_cookie_t;

  pub fn xcb_xf86dri_get_device_info_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_xf86dri_get_device_info_cookie_t;

  pub fn xcb_xf86dri_get_device_info_device_private(
    R: *const xcb_xf86dri_get_device_info_reply_t
  ) -> *mut u32;

  pub fn xcb_xf86dri_get_device_info_device_private_length(
    R: *const xcb_xf86dri_get_device_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xf86dri_get_device_info_device_private_end(
    R: *const xcb_xf86dri_get_device_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xf86dri_get_device_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_get_device_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_get_device_info_reply_t;

  pub fn xcb_xf86dri_auth_connection(
    c: *mut xcb_connection_t,
    screen: u32,
    magic: u32,
  ) -> xcb_xf86dri_auth_connection_cookie_t;

  pub fn xcb_xf86dri_auth_connection_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
    magic: u32,
  ) -> xcb_xf86dri_auth_connection_cookie_t;

  pub fn xcb_xf86dri_auth_connection_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xf86dri_auth_connection_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xf86dri_auth_connection_reply_t;
}
