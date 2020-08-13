use super::core::{xcb_connection_t, xcb_void_cookie_t};
use super::render::{
  xcb_render_glyphset_t,
  xcb_render_pictformat_t,
  xcb_render_pictforminfo_t,
  xcb_render_picture_t,
  xcb_render_pictvisual_t,
  xcb_render_query_pict_formats_reply_t,
  xcb_render_query_version_reply_t,
};
use super::xcb::xcb_visualid_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_pict_format_t
{
  XCB_PICT_FORMAT_ID = 1,
  XCB_PICT_FORMAT_TYPE = 2,
  XCB_PICT_FORMAT_DEPTH = 4,
  XCB_PICT_FORMAT_RED = 8,
  XCB_PICT_FORMAT_RED_MASK = 16,
  XCB_PICT_FORMAT_GREEN = 32,
  XCB_PICT_FORMAT_GREEN_MASK = 64,
  XCB_PICT_FORMAT_BLUE = 128,
  XCB_PICT_FORMAT_BLUE_MASK = 256,
  XCB_PICT_FORMAT_ALPHA = 512,
  XCB_PICT_FORMAT_ALPHA_MASK = 1024,
  XCB_PICT_FORMAT_COLORMAP = 2048,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_pict_standard_t
{
  XCB_PICT_STANDARD_ARGB_32 = 0,
  XCB_PICT_STANDARD_RGB_24 = 1,
  XCB_PICT_STANDARD_A_8 = 2,
  XCB_PICT_STANDARD_A_4 = 3,
  XCB_PICT_STANDARD_A_1 = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_util_composite_text_stream_t
{
  _unused: [u8; 0],
}

#[link(name = "xcb")]
extern "C" {
  pub fn xcb_render_util_find_visual_format(
    formats: *const xcb_render_query_pict_formats_reply_t,
    visual: xcb_visualid_t,
  ) -> *mut xcb_render_pictvisual_t;

  pub fn xcb_render_util_find_format(
    formats: *const xcb_render_query_pict_formats_reply_t,
    mask: ::std::os::raw::c_ulong,
    ptemplate: *const xcb_render_pictforminfo_t,
    count: ::std::os::raw::c_int,
  ) -> *mut xcb_render_pictforminfo_t;

  pub fn xcb_render_util_find_standard_format(
    formats: *const xcb_render_query_pict_formats_reply_t,
    format: xcb_pict_standard_t,
  ) -> *mut xcb_render_pictforminfo_t;

  pub fn xcb_render_util_query_version(
    c: *mut xcb_connection_t
  ) -> *const xcb_render_query_version_reply_t;

  pub fn xcb_render_util_query_formats(
    c: *mut xcb_connection_t
  ) -> *const xcb_render_query_pict_formats_reply_t;

  pub fn xcb_render_util_disconnect(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;

  pub fn xcb_render_util_composite_text_stream(
    initial_glyphset: xcb_render_glyphset_t,
    total_glyphs: u32,
    total_glyphset_changes: u32,
  ) -> *mut xcb_render_util_composite_text_stream_t;

  pub fn xcb_render_util_glyphs_8(
    stream: *mut xcb_render_util_composite_text_stream_t,
    dx: i16,
    dy: i16,
    count: u32,
    glyphs: *const u8,
  );

  pub fn xcb_render_util_glyphs_16(
    stream: *mut xcb_render_util_composite_text_stream_t,
    dx: i16,
    dy: i16,
    count: u32,
    glyphs: *const u16,
  );

  pub fn xcb_render_util_glyphs_32(
    stream: *mut xcb_render_util_composite_text_stream_t,
    dx: i16,
    dy: i16,
    count: u32,
    glyphs: *const u32,
  );

  pub fn xcb_render_util_change_glyphset(
    stream: *mut xcb_render_util_composite_text_stream_t,
    glyphset: xcb_render_glyphset_t,
  );

  pub fn xcb_render_util_composite_text(
    xc: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    stream: *mut xcb_render_util_composite_text_stream_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_util_composite_text_checked(
    xc: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    stream: *mut xcb_render_util_composite_text_stream_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_util_composite_text_free(stream: *mut xcb_render_util_composite_text_stream_t);
}
