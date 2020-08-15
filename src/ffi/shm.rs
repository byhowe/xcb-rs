use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_gcontext_t, xcb_pixmap_t, xcb_value_error_t, xcb_visualid_t};

pub const XCB_SHM_MAJOR_VERSION: u32 = 1;
pub const XCB_SHM_MINOR_VERSION: u32 = 2;
pub const XCB_SHM_COMPLETION: u32 = 0;
pub const XCB_SHM_BAD_SEG: u32 = 0;
pub const XCB_SHM_QUERY_VERSION: u32 = 0;
pub const XCB_SHM_ATTACH: u32 = 1;
pub const XCB_SHM_DETACH: u32 = 2;
pub const XCB_SHM_PUT_IMAGE: u32 = 3;
pub const XCB_SHM_GET_IMAGE: u32 = 4;
pub const XCB_SHM_CREATE_PIXMAP: u32 = 5;
pub const XCB_SHM_ATTACH_FD: u32 = 6;
pub const XCB_SHM_CREATE_SEGMENT: u32 = 7;

pub type xcb_shm_seg_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_seg_iterator_t
{
  pub data: *mut xcb_shm_seg_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_completion_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub drawable: xcb_drawable_t,
  pub minor_event: u16,
  pub major_event: u8,
  pub pad1: u8,
  pub shmseg: xcb_shm_seg_t,
  pub offset: u32,
}

pub type xcb_shm_bad_seg_error_t = xcb_value_error_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_query_version_reply_t
{
  pub response_type: u8,
  pub shared_pixmaps: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u16,
  pub minor_version: u16,
  pub uid: u16,
  pub gid: u16,
  pub pixmap_format: u8,
  pub pad0: [u8; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_attach_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub shmseg: xcb_shm_seg_t,
  pub shmid: u32,
  pub read_only: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_detach_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub shmseg: xcb_shm_seg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_put_image_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub total_width: u16,
  pub total_height: u16,
  pub src_x: u16,
  pub src_y: u16,
  pub src_width: u16,
  pub src_height: u16,
  pub dst_x: i16,
  pub dst_y: i16,
  pub depth: u8,
  pub format: u8,
  pub send_event: u8,
  pub pad0: u8,
  pub shmseg: xcb_shm_seg_t,
  pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_get_image_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_get_image_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub plane_mask: u32,
  pub format: u8,
  pub pad0: [u8; 3usize],
  pub shmseg: xcb_shm_seg_t,
  pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_get_image_reply_t
{
  pub response_type: u8,
  pub depth: u8,
  pub sequence: u16,
  pub length: u32,
  pub visual: xcb_visualid_t,
  pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_create_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pid: xcb_pixmap_t,
  pub drawable: xcb_drawable_t,
  pub width: u16,
  pub height: u16,
  pub depth: u8,
  pub pad0: [u8; 3usize],
  pub shmseg: xcb_shm_seg_t,
  pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_attach_fd_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub shmseg: xcb_shm_seg_t,
  pub read_only: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_create_segment_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_create_segment_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub shmseg: xcb_shm_seg_t,
  pub size: u32,
  pub read_only: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_create_segment_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[link(name = "xcb-shm")]
extern "C" {
  pub static mut xcb_shm_id: xcb_extension_t;

  pub fn xcb_shm_seg_next(i: *mut xcb_shm_seg_iterator_t);

  pub fn xcb_shm_seg_end(i: xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_shm_query_version(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t;

  pub fn xcb_shm_query_version_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_shm_query_version_cookie_t;

  pub fn xcb_shm_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shm_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shm_query_version_reply_t;

  pub fn xcb_shm_attach_checked(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    shmid: u32,
    read_only: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_attach(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    shmid: u32,
    read_only: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_detach_checked(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_detach(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_put_image_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    total_width: u16,
    total_height: u16,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
    depth: u8,
    format: u8,
    send_event: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_put_image(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    total_width: u16,
    total_height: u16,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
    depth: u8,
    format: u8,
    send_event: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_get_image(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
    format: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_shm_get_image_cookie_t;

  pub fn xcb_shm_get_image_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
    format: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_shm_get_image_cookie_t;

  pub fn xcb_shm_get_image_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shm_get_image_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shm_get_image_reply_t;

  pub fn xcb_shm_create_pixmap_checked(
    c: *mut xcb_connection_t,
    pid: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
    depth: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_create_pixmap(
    c: *mut xcb_connection_t,
    pid: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
    depth: u8,
    shmseg: xcb_shm_seg_t,
    offset: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_attach_fd_checked(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    shm_fd: i32,
    read_only: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_attach_fd(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    shm_fd: i32,
    read_only: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_shm_create_segment(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    size: u32,
    read_only: u8,
  ) -> xcb_shm_create_segment_cookie_t;

  pub fn xcb_shm_create_segment_unchecked(
    c: *mut xcb_connection_t,
    shmseg: xcb_shm_seg_t,
    size: u32,
    read_only: u8,
  ) -> xcb_shm_create_segment_cookie_t;

  pub fn xcb_shm_create_segment_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_shm_create_segment_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_shm_create_segment_reply_t;

  pub fn xcb_shm_create_segment_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_shm_create_segment_reply_t,
  ) -> *mut ::std::os::raw::c_int;
}
