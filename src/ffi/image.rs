use super::core::{xcb_connection_t, xcb_void_cookie_t};
use super::shm::xcb_shm_seg_t;
use super::xcb::{
  xcb_drawable_t,
  xcb_gcontext_t,
  xcb_image_format_t,
  xcb_image_order_t,
  xcb_pixmap_t,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_image_t
{
  pub width: u16,
  pub height: u16,
  pub format: xcb_image_format_t,
  pub scanline_pad: u8,
  pub depth: u8,
  pub bpp: u8,
  pub unit: u8,
  pub plane_mask: u32,
  pub byte_order: xcb_image_order_t,
  pub bit_order: xcb_image_order_t,
  pub stride: u32,
  pub size: u32,
  pub base: *mut ::std::os::raw::c_void,
  pub data: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_shm_segment_info_t
{
  pub shmseg: xcb_shm_seg_t,
  pub shmid: u32,
  pub shmaddr: *mut u8,
}

#[link(name = "xcb-image")]
extern "C" {
  pub fn xcb_image_annotate(image: *mut xcb_image_t);

  pub fn xcb_image_create(
    width: u16,
    height: u16,
    format: xcb_image_format_t,
    xpad: u8,
    depth: u8,
    bpp: u8,
    unit: u8,
    byte_order: xcb_image_order_t,
    bit_order: xcb_image_order_t,
    base: *mut ::std::os::raw::c_void,
    bytes: u32,
    data: *mut u8,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_create_native(
    c: *mut xcb_connection_t,
    width: u16,
    height: u16,
    format: xcb_image_format_t,
    depth: u8,
    base: *mut ::std::os::raw::c_void,
    bytes: u32,
    data: *mut u8,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_destroy(image: *mut xcb_image_t);

  pub fn xcb_image_get(
    conn: *mut xcb_connection_t,
    draw: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
    format: xcb_image_format_t,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_put(
    conn: *mut xcb_connection_t,
    draw: xcb_drawable_t,
    gc: xcb_gcontext_t,
    image: *mut xcb_image_t,
    x: i16,
    y: i16,
    left_pad: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_image_native(
    c: *mut xcb_connection_t,
    image: *mut xcb_image_t,
    convert: ::std::os::raw::c_int,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_put_pixel(
    image: *mut xcb_image_t,
    x: u32,
    y: u32,
    pixel: u32,
  );

  pub fn xcb_image_get_pixel(
    image: *mut xcb_image_t,
    x: u32,
    y: u32,
  ) -> u32;

  pub fn xcb_image_convert(
    src: *mut xcb_image_t,
    dst: *mut xcb_image_t,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_subimage(
    image: *mut xcb_image_t,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    base: *mut ::std::os::raw::c_void,
    bytes: u32,
    data: *mut u8,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_shm_put(
    conn: *mut xcb_connection_t,
    draw: xcb_drawable_t,
    gc: xcb_gcontext_t,
    image: *mut xcb_image_t,
    shminfo: xcb_shm_segment_info_t,
    src_x: i16,
    src_y: i16,
    dest_x: i16,
    dest_y: i16,
    src_width: u16,
    src_height: u16,
    send_event: u8,
  ) -> *mut xcb_image_t;

  pub fn xcb_image_shm_get(
    conn: *mut xcb_connection_t,
    draw: xcb_drawable_t,
    image: *mut xcb_image_t,
    shminfo: xcb_shm_segment_info_t,
    x: i16,
    y: i16,
    plane_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_image_create_from_bitmap_data(
    data: *mut u8,
    width: u32,
    height: u32,
  ) -> *mut xcb_image_t;

  pub fn xcb_create_pixmap_from_bitmap_data(
    display: *mut xcb_connection_t,
    d: xcb_drawable_t,
    data: *mut u8,
    width: u32,
    height: u32,
    depth: u32,
    fg: u32,
    bg: u32,
    gcp: *mut xcb_gcontext_t,
  ) -> xcb_pixmap_t;
}
