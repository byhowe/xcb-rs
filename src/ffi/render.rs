use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{
  xcb_atom_t,
  xcb_colormap_t,
  xcb_cursor_t,
  xcb_drawable_t,
  xcb_pixmap_t,
  xcb_rectangle_iterator_t,
  xcb_rectangle_t,
  xcb_str_iterator_t,
  xcb_visualid_t,
};

pub const XCB_RENDER_MAJOR_VERSION: u32 = 0;
pub const XCB_RENDER_MINOR_VERSION: u32 = 11;
pub const XCB_RENDER_PICT_FORMAT: u32 = 0;
pub const XCB_RENDER_PICTURE: u32 = 1;
pub const XCB_RENDER_PICT_OP: u32 = 2;
pub const XCB_RENDER_GLYPH_SET: u32 = 3;
pub const XCB_RENDER_GLYPH: u32 = 4;
pub const XCB_RENDER_QUERY_VERSION: u32 = 0;
pub const XCB_RENDER_QUERY_PICT_FORMATS: u32 = 1;
pub const XCB_RENDER_QUERY_PICT_INDEX_VALUES: u32 = 2;
pub const XCB_RENDER_CREATE_PICTURE: u32 = 4;
pub const XCB_RENDER_CHANGE_PICTURE: u32 = 5;
pub const XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES: u32 = 6;
pub const XCB_RENDER_FREE_PICTURE: u32 = 7;
pub const XCB_RENDER_COMPOSITE: u32 = 8;
pub const XCB_RENDER_TRAPEZOIDS: u32 = 10;
pub const XCB_RENDER_TRIANGLES: u32 = 11;
pub const XCB_RENDER_TRI_STRIP: u32 = 12;
pub const XCB_RENDER_TRI_FAN: u32 = 13;
pub const XCB_RENDER_CREATE_GLYPH_SET: u32 = 17;
pub const XCB_RENDER_REFERENCE_GLYPH_SET: u32 = 18;
pub const XCB_RENDER_FREE_GLYPH_SET: u32 = 19;
pub const XCB_RENDER_ADD_GLYPHS: u32 = 20;
pub const XCB_RENDER_FREE_GLYPHS: u32 = 22;
pub const XCB_RENDER_COMPOSITE_GLYPHS_8: u32 = 23;
pub const XCB_RENDER_COMPOSITE_GLYPHS_16: u32 = 24;
pub const XCB_RENDER_COMPOSITE_GLYPHS_32: u32 = 25;
pub const XCB_RENDER_FILL_RECTANGLES: u32 = 26;
pub const XCB_RENDER_CREATE_CURSOR: u32 = 27;
pub const XCB_RENDER_SET_PICTURE_TRANSFORM: u32 = 28;
pub const XCB_RENDER_QUERY_FILTERS: u32 = 29;
pub const XCB_RENDER_SET_PICTURE_FILTER: u32 = 30;
pub const XCB_RENDER_CREATE_ANIM_CURSOR: u32 = 31;
pub const XCB_RENDER_ADD_TRAPS: u32 = 32;
pub const XCB_RENDER_CREATE_SOLID_FILL: u32 = 33;
pub const XCB_RENDER_CREATE_LINEAR_GRADIENT: u32 = 34;
pub const XCB_RENDER_CREATE_RADIAL_GRADIENT: u32 = 35;
pub const XCB_RENDER_CREATE_CONICAL_GRADIENT: u32 = 36;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_pict_type_t
{
  XCB_RENDER_PICT_TYPE_INDEXED = 0,
  XCB_RENDER_PICT_TYPE_DIRECT = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_picture_enum_t
{
  XCB_RENDER_PICTURE_NONE = 0,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_pict_op_t
{
  XCB_RENDER_PICT_OP_CLEAR = 0,
  XCB_RENDER_PICT_OP_SRC = 1,
  XCB_RENDER_PICT_OP_DST = 2,
  XCB_RENDER_PICT_OP_OVER = 3,
  XCB_RENDER_PICT_OP_OVER_REVERSE = 4,
  XCB_RENDER_PICT_OP_IN = 5,
  XCB_RENDER_PICT_OP_IN_REVERSE = 6,
  XCB_RENDER_PICT_OP_OUT = 7,
  XCB_RENDER_PICT_OP_OUT_REVERSE = 8,
  XCB_RENDER_PICT_OP_ATOP = 9,
  XCB_RENDER_PICT_OP_ATOP_REVERSE = 10,
  XCB_RENDER_PICT_OP_XOR = 11,
  XCB_RENDER_PICT_OP_ADD = 12,
  XCB_RENDER_PICT_OP_SATURATE = 13,
  XCB_RENDER_PICT_OP_DISJOINT_CLEAR = 16,
  XCB_RENDER_PICT_OP_DISJOINT_SRC = 17,
  XCB_RENDER_PICT_OP_DISJOINT_DST = 18,
  XCB_RENDER_PICT_OP_DISJOINT_OVER = 19,
  XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE = 20,
  XCB_RENDER_PICT_OP_DISJOINT_IN = 21,
  XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE = 22,
  XCB_RENDER_PICT_OP_DISJOINT_OUT = 23,
  XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE = 24,
  XCB_RENDER_PICT_OP_DISJOINT_ATOP = 25,
  XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE = 26,
  XCB_RENDER_PICT_OP_DISJOINT_XOR = 27,
  XCB_RENDER_PICT_OP_CONJOINT_CLEAR = 32,
  XCB_RENDER_PICT_OP_CONJOINT_SRC = 33,
  XCB_RENDER_PICT_OP_CONJOINT_DST = 34,
  XCB_RENDER_PICT_OP_CONJOINT_OVER = 35,
  XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE = 36,
  XCB_RENDER_PICT_OP_CONJOINT_IN = 37,
  XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE = 38,
  XCB_RENDER_PICT_OP_CONJOINT_OUT = 39,
  XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE = 40,
  XCB_RENDER_PICT_OP_CONJOINT_ATOP = 41,
  XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE = 42,
  XCB_RENDER_PICT_OP_CONJOINT_XOR = 43,
  XCB_RENDER_PICT_OP_MULTIPLY = 48,
  XCB_RENDER_PICT_OP_SCREEN = 49,
  XCB_RENDER_PICT_OP_OVERLAY = 50,
  XCB_RENDER_PICT_OP_DARKEN = 51,
  XCB_RENDER_PICT_OP_LIGHTEN = 52,
  XCB_RENDER_PICT_OP_COLOR_DODGE = 53,
  XCB_RENDER_PICT_OP_COLOR_BURN = 54,
  XCB_RENDER_PICT_OP_HARD_LIGHT = 55,
  XCB_RENDER_PICT_OP_SOFT_LIGHT = 56,
  XCB_RENDER_PICT_OP_DIFFERENCE = 57,
  XCB_RENDER_PICT_OP_EXCLUSION = 58,
  XCB_RENDER_PICT_OP_HSL_HUE = 59,
  XCB_RENDER_PICT_OP_HSL_SATURATION = 60,
  XCB_RENDER_PICT_OP_HSL_COLOR = 61,
  XCB_RENDER_PICT_OP_HSL_LUMINOSITY = 62,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_poly_edge_t
{
  XCB_RENDER_POLY_EDGE_SHARP = 0,
  XCB_RENDER_POLY_EDGE_SMOOTH = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_poly_mode_t
{
  XCB_RENDER_POLY_MODE_PRECISE = 0,
  XCB_RENDER_POLY_MODE_IMPRECISE = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_cp_t
{
  XCB_RENDER_CP_REPEAT = 1,
  XCB_RENDER_CP_ALPHA_MAP = 2,
  XCB_RENDER_CP_ALPHA_X_ORIGIN = 4,
  XCB_RENDER_CP_ALPHA_Y_ORIGIN = 8,
  XCB_RENDER_CP_CLIP_X_ORIGIN = 16,
  XCB_RENDER_CP_CLIP_Y_ORIGIN = 32,
  XCB_RENDER_CP_CLIP_MASK = 64,
  XCB_RENDER_CP_GRAPHICS_EXPOSURE = 128,
  XCB_RENDER_CP_SUBWINDOW_MODE = 256,
  XCB_RENDER_CP_POLY_EDGE = 512,
  XCB_RENDER_CP_POLY_MODE = 1024,
  XCB_RENDER_CP_DITHER = 2048,
  XCB_RENDER_CP_COMPONENT_ALPHA = 4096,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_sub_pixel_t
{
  XCB_RENDER_SUB_PIXEL_UNKNOWN = 0,
  XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB = 1,
  XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR = 2,
  XCB_RENDER_SUB_PIXEL_VERTICAL_RGB = 3,
  XCB_RENDER_SUB_PIXEL_VERTICAL_BGR = 4,
  XCB_RENDER_SUB_PIXEL_NONE = 5,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_render_repeat_t
{
  XCB_RENDER_REPEAT_NONE = 0,
  XCB_RENDER_REPEAT_NORMAL = 1,
  XCB_RENDER_REPEAT_PAD = 2,
  XCB_RENDER_REPEAT_REFLECT = 3,
}

pub type xcb_render_glyph_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyph_iterator_t
{
  pub data: *mut xcb_render_glyph_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_render_glyphset_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyphset_iterator_t
{
  pub data: *mut xcb_render_glyphset_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_render_picture_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_picture_iterator_t
{
  pub data: *mut xcb_render_picture_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_render_pictformat_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictformat_iterator_t
{
  pub data: *mut xcb_render_pictformat_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_render_fixed_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_fixed_iterator_t
{
  pub data: *mut xcb_render_fixed_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pict_format_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_picture_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pict_op_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyph_set_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyph_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_directformat_t
{
  pub red_shift: u16,
  pub red_mask: u16,
  pub green_shift: u16,
  pub green_mask: u16,
  pub blue_shift: u16,
  pub blue_mask: u16,
  pub alpha_shift: u16,
  pub alpha_mask: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_directformat_iterator_t
{
  pub data: *mut xcb_render_directformat_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictforminfo_t
{
  pub id: xcb_render_pictformat_t,
  pub type_: u8,
  pub depth: u8,
  pub pad0: [u8; 2usize],
  pub direct: xcb_render_directformat_t,
  pub colormap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictforminfo_iterator_t
{
  pub data: *mut xcb_render_pictforminfo_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictvisual_t
{
  pub visual: xcb_visualid_t,
  pub format: xcb_render_pictformat_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictvisual_iterator_t
{
  pub data: *mut xcb_render_pictvisual_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictdepth_t
{
  pub depth: u8,
  pub pad0: u8,
  pub num_visuals: u16,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictdepth_iterator_t
{
  pub data: *mut xcb_render_pictdepth_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictscreen_t
{
  pub num_depths: u32,
  pub fallback: xcb_render_pictformat_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pictscreen_iterator_t
{
  pub data: *mut xcb_render_pictscreen_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_indexvalue_t
{
  pub pixel: u32,
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub alpha: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_indexvalue_iterator_t
{
  pub data: *mut xcb_render_indexvalue_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_color_t
{
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub alpha: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_color_iterator_t
{
  pub data: *mut xcb_render_color_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pointfix_t
{
  pub x: xcb_render_fixed_t,
  pub y: xcb_render_fixed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_pointfix_iterator_t
{
  pub data: *mut xcb_render_pointfix_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_linefix_t
{
  pub p1: xcb_render_pointfix_t,
  pub p2: xcb_render_pointfix_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_linefix_iterator_t
{
  pub data: *mut xcb_render_linefix_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_triangle_t
{
  pub p1: xcb_render_pointfix_t,
  pub p2: xcb_render_pointfix_t,
  pub p3: xcb_render_pointfix_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_triangle_iterator_t
{
  pub data: *mut xcb_render_triangle_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_trapezoid_t
{
  pub top: xcb_render_fixed_t,
  pub bottom: xcb_render_fixed_t,
  pub left: xcb_render_linefix_t,
  pub right: xcb_render_linefix_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_trapezoid_iterator_t
{
  pub data: *mut xcb_render_trapezoid_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyphinfo_t
{
  pub width: u16,
  pub height: u16,
  pub x: i16,
  pub y: i16,
  pub x_off: i16,
  pub y_off: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_glyphinfo_iterator_t
{
  pub data: *mut xcb_render_glyphinfo_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major_version: u32,
  pub client_minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_version_reply_t
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
pub struct xcb_render_query_pict_formats_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_pict_formats_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_pict_formats_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_formats: u32,
  pub num_screens: u32,
  pub num_depths: u32,
  pub num_visuals: u32,
  pub num_subpixel: u32,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_pict_index_values_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_pict_index_values_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub format: xcb_render_pictformat_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_pict_index_values_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_values: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_picture_value_list_t
{
  pub repeat: u32,
  pub alphamap: xcb_render_picture_t,
  pub alphaxorigin: i32,
  pub alphayorigin: i32,
  pub clipxorigin: i32,
  pub clipyorigin: i32,
  pub clipmask: xcb_pixmap_t,
  pub graphicsexposure: u32,
  pub subwindowmode: u32,
  pub polyedge: u32,
  pub polymode: u32,
  pub dither: xcb_atom_t,
  pub componentalpha: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_picture_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pid: xcb_render_picture_t,
  pub drawable: xcb_drawable_t,
  pub format: xcb_render_pictformat_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_change_picture_value_list_t
{
  pub repeat: u32,
  pub alphamap: xcb_render_picture_t,
  pub alphaxorigin: i32,
  pub alphayorigin: i32,
  pub clipxorigin: i32,
  pub clipyorigin: i32,
  pub clipmask: xcb_pixmap_t,
  pub graphicsexposure: u32,
  pub subwindowmode: u32,
  pub polyedge: u32,
  pub polymode: u32,
  pub dither: xcb_atom_t,
  pub componentalpha: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_change_picture_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_set_picture_clip_rectangles_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub clip_x_origin: i16,
  pub clip_y_origin: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_free_picture_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_composite_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub mask: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub src_x: i16,
  pub src_y: i16,
  pub mask_x: i16,
  pub mask_y: i16,
  pub dst_x: i16,
  pub dst_y: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_trapezoids_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_triangles_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_tri_strip_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_tri_fan_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_glyph_set_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub gsid: xcb_render_glyphset_t,
  pub format: xcb_render_pictformat_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_reference_glyph_set_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub gsid: xcb_render_glyphset_t,
  pub existing: xcb_render_glyphset_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_free_glyph_set_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glyphset: xcb_render_glyphset_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_add_glyphs_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glyphset: xcb_render_glyphset_t,
  pub glyphs_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_free_glyphs_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glyphset: xcb_render_glyphset_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_composite_glyphs_8_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub glyphset: xcb_render_glyphset_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_composite_glyphs_16_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub glyphset: xcb_render_glyphset_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_composite_glyphs_32_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub src: xcb_render_picture_t,
  pub dst: xcb_render_picture_t,
  pub mask_format: xcb_render_pictformat_t,
  pub glyphset: xcb_render_glyphset_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_fill_rectangles_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub op: u8,
  pub pad0: [u8; 3usize],
  pub dst: xcb_render_picture_t,
  pub color: xcb_render_color_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_cursor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cid: xcb_cursor_t,
  pub source: xcb_render_picture_t,
  pub x: u16,
  pub y: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_transform_t
{
  pub matrix11: xcb_render_fixed_t,
  pub matrix12: xcb_render_fixed_t,
  pub matrix13: xcb_render_fixed_t,
  pub matrix21: xcb_render_fixed_t,
  pub matrix22: xcb_render_fixed_t,
  pub matrix23: xcb_render_fixed_t,
  pub matrix31: xcb_render_fixed_t,
  pub matrix32: xcb_render_fixed_t,
  pub matrix33: xcb_render_fixed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_transform_iterator_t
{
  pub data: *mut xcb_render_transform_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_set_picture_transform_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub transform: xcb_render_transform_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_filters_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_filters_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_query_filters_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_aliases: u32,
  pub num_filters: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_set_picture_filter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub filter_len: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_animcursorelt_t
{
  pub cursor: xcb_cursor_t,
  pub delay: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_animcursorelt_iterator_t
{
  pub data: *mut xcb_render_animcursorelt_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_anim_cursor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cid: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_spanfix_t
{
  pub l: xcb_render_fixed_t,
  pub r: xcb_render_fixed_t,
  pub y: xcb_render_fixed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_spanfix_iterator_t
{
  pub data: *mut xcb_render_spanfix_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_trap_t
{
  pub top: xcb_render_spanfix_t,
  pub bot: xcb_render_spanfix_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_trap_iterator_t
{
  pub data: *mut xcb_render_trap_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_add_traps_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub x_off: i16,
  pub y_off: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_solid_fill_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub color: xcb_render_color_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_linear_gradient_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub p1: xcb_render_pointfix_t,
  pub p2: xcb_render_pointfix_t,
  pub num_stops: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_radial_gradient_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub inner: xcb_render_pointfix_t,
  pub outer: xcb_render_pointfix_t,
  pub inner_radius: xcb_render_fixed_t,
  pub outer_radius: xcb_render_fixed_t,
  pub num_stops: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_render_create_conical_gradient_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub picture: xcb_render_picture_t,
  pub center: xcb_render_pointfix_t,
  pub angle: xcb_render_fixed_t,
  pub num_stops: u32,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_render_id: xcb_extension_t;

  pub fn xcb_render_glyph_next(i: *mut xcb_render_glyph_iterator_t);

  pub fn xcb_render_glyph_end(i: xcb_render_glyph_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_glyphset_next(i: *mut xcb_render_glyphset_iterator_t);

  pub fn xcb_render_glyphset_end(i: xcb_render_glyphset_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_picture_next(i: *mut xcb_render_picture_iterator_t);

  pub fn xcb_render_picture_end(i: xcb_render_picture_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_pictformat_next(i: *mut xcb_render_pictformat_iterator_t);

  pub fn xcb_render_pictformat_end(i: xcb_render_pictformat_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_fixed_next(i: *mut xcb_render_fixed_iterator_t);

  pub fn xcb_render_fixed_end(i: xcb_render_fixed_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_directformat_next(i: *mut xcb_render_directformat_iterator_t);

  pub fn xcb_render_directformat_end(
    i: xcb_render_directformat_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_pictforminfo_next(i: *mut xcb_render_pictforminfo_iterator_t);

  pub fn xcb_render_pictforminfo_end(
    i: xcb_render_pictforminfo_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_pictvisual_next(i: *mut xcb_render_pictvisual_iterator_t);

  pub fn xcb_render_pictvisual_end(i: xcb_render_pictvisual_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_pictdepth_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_pictdepth_visuals(
    R: *const xcb_render_pictdepth_t
  ) -> *mut xcb_render_pictvisual_t;

  pub fn xcb_render_pictdepth_visuals_length(
    R: *const xcb_render_pictdepth_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_pictdepth_visuals_iterator(
    R: *const xcb_render_pictdepth_t
  ) -> xcb_render_pictvisual_iterator_t;

  pub fn xcb_render_pictdepth_next(i: *mut xcb_render_pictdepth_iterator_t);

  pub fn xcb_render_pictdepth_end(i: xcb_render_pictdepth_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_pictscreen_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_pictscreen_depths_length(
    R: *const xcb_render_pictscreen_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_pictscreen_depths_iterator(
    R: *const xcb_render_pictscreen_t
  ) -> xcb_render_pictdepth_iterator_t;

  pub fn xcb_render_pictscreen_next(i: *mut xcb_render_pictscreen_iterator_t);

  pub fn xcb_render_pictscreen_end(i: xcb_render_pictscreen_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_indexvalue_next(i: *mut xcb_render_indexvalue_iterator_t);

  pub fn xcb_render_indexvalue_end(i: xcb_render_indexvalue_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_color_next(i: *mut xcb_render_color_iterator_t);

  pub fn xcb_render_color_end(i: xcb_render_color_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_pointfix_next(i: *mut xcb_render_pointfix_iterator_t);

  pub fn xcb_render_pointfix_end(i: xcb_render_pointfix_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_linefix_next(i: *mut xcb_render_linefix_iterator_t);

  pub fn xcb_render_linefix_end(i: xcb_render_linefix_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_triangle_next(i: *mut xcb_render_triangle_iterator_t);

  pub fn xcb_render_triangle_end(i: xcb_render_triangle_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_trapezoid_next(i: *mut xcb_render_trapezoid_iterator_t);

  pub fn xcb_render_trapezoid_end(i: xcb_render_trapezoid_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_glyphinfo_next(i: *mut xcb_render_glyphinfo_iterator_t);

  pub fn xcb_render_glyphinfo_end(i: xcb_render_glyphinfo_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_query_version(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_render_query_version_cookie_t;

  pub fn xcb_render_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major_version: u32,
    client_minor_version: u32,
  ) -> xcb_render_query_version_cookie_t;

  pub fn xcb_render_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_render_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_render_query_version_reply_t;

  pub fn xcb_render_query_pict_formats_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_formats(
    c: *mut xcb_connection_t
  ) -> xcb_render_query_pict_formats_cookie_t;

  pub fn xcb_render_query_pict_formats_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_render_query_pict_formats_cookie_t;

  pub fn xcb_render_query_pict_formats_formats(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> *mut xcb_render_pictforminfo_t;

  pub fn xcb_render_query_pict_formats_formats_length(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_formats_formats_iterator(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> xcb_render_pictforminfo_iterator_t;

  pub fn xcb_render_query_pict_formats_screens_length(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_formats_screens_iterator(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> xcb_render_pictscreen_iterator_t;

  pub fn xcb_render_query_pict_formats_subpixels(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> *mut u32;

  pub fn xcb_render_query_pict_formats_subpixels_length(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_formats_subpixels_end(
    R: *const xcb_render_query_pict_formats_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_query_pict_formats_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_render_query_pict_formats_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_render_query_pict_formats_reply_t;

  pub fn xcb_render_query_pict_index_values_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_index_values(
    c: *mut xcb_connection_t,
    format: xcb_render_pictformat_t,
  ) -> xcb_render_query_pict_index_values_cookie_t;

  pub fn xcb_render_query_pict_index_values_unchecked(
    c: *mut xcb_connection_t,
    format: xcb_render_pictformat_t,
  ) -> xcb_render_query_pict_index_values_cookie_t;

  pub fn xcb_render_query_pict_index_values_values(
    R: *const xcb_render_query_pict_index_values_reply_t
  ) -> *mut xcb_render_indexvalue_t;

  pub fn xcb_render_query_pict_index_values_values_length(
    R: *const xcb_render_query_pict_index_values_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_pict_index_values_values_iterator(
    R: *const xcb_render_query_pict_index_values_reply_t
  ) -> xcb_render_indexvalue_iterator_t;

  pub fn xcb_render_query_pict_index_values_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_render_query_pict_index_values_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_render_query_pict_index_values_reply_t;

  pub fn xcb_render_create_picture_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_render_create_picture_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_picture_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_render_create_picture_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_picture_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_picture_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_picture_checked(
    c: *mut xcb_connection_t,
    pid: xcb_render_picture_t,
    drawable: xcb_drawable_t,
    format: xcb_render_pictformat_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_picture(
    c: *mut xcb_connection_t,
    pid: xcb_render_picture_t,
    drawable: xcb_drawable_t,
    format: xcb_render_pictformat_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_picture_aux_checked(
    c: *mut xcb_connection_t,
    pid: xcb_render_picture_t,
    drawable: xcb_drawable_t,
    format: xcb_render_pictformat_t,
    value_mask: u32,
    value_list: *const xcb_render_create_picture_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_picture_aux(
    c: *mut xcb_connection_t,
    pid: xcb_render_picture_t,
    drawable: xcb_drawable_t,
    format: xcb_render_pictformat_t,
    value_mask: u32,
    value_list: *const xcb_render_create_picture_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_picture_value_list(
    R: *const xcb_render_create_picture_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_render_change_picture_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_render_change_picture_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_change_picture_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_render_change_picture_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_change_picture_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_change_picture_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_change_picture_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_change_picture(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_change_picture_aux_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    value_mask: u32,
    value_list: *const xcb_render_change_picture_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_change_picture_aux(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    value_mask: u32,
    value_list: *const xcb_render_change_picture_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_change_picture_value_list(
    R: *const xcb_render_change_picture_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_render_set_picture_clip_rectangles_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rectangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_set_picture_clip_rectangles_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_set_picture_clip_rectangles(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_set_picture_clip_rectangles_rectangles(
    R: *const xcb_render_set_picture_clip_rectangles_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_render_set_picture_clip_rectangles_rectangles_length(
    R: *const xcb_render_set_picture_clip_rectangles_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_set_picture_clip_rectangles_rectangles_iterator(
    R: *const xcb_render_set_picture_clip_rectangles_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_render_free_picture_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_free_picture(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    mask: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    src_x: i16,
    src_y: i16,
    mask_x: i16,
    mask_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    mask: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    src_x: i16,
    src_y: i16,
    mask_x: i16,
    mask_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_trapezoids_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    traps_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_trapezoids_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    traps_len: u32,
    traps: *const xcb_render_trapezoid_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_trapezoids(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    traps_len: u32,
    traps: *const xcb_render_trapezoid_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_trapezoids_traps(
    R: *const xcb_render_trapezoids_request_t
  ) -> *mut xcb_render_trapezoid_t;

  pub fn xcb_render_trapezoids_traps_length(
    R: *const xcb_render_trapezoids_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_trapezoids_traps_iterator(
    R: *const xcb_render_trapezoids_request_t
  ) -> xcb_render_trapezoid_iterator_t;

  pub fn xcb_render_triangles_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    triangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_triangles_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    triangles_len: u32,
    triangles: *const xcb_render_triangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_triangles(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    triangles_len: u32,
    triangles: *const xcb_render_triangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_triangles_triangles(
    R: *const xcb_render_triangles_request_t
  ) -> *mut xcb_render_triangle_t;

  pub fn xcb_render_triangles_triangles_length(
    R: *const xcb_render_triangles_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_triangles_triangles_iterator(
    R: *const xcb_render_triangles_request_t
  ) -> xcb_render_triangle_iterator_t;

  pub fn xcb_render_tri_strip_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    points_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_tri_strip_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    points_len: u32,
    points: *const xcb_render_pointfix_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_tri_strip(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    points_len: u32,
    points: *const xcb_render_pointfix_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_tri_strip_points(
    R: *const xcb_render_tri_strip_request_t
  ) -> *mut xcb_render_pointfix_t;

  pub fn xcb_render_tri_strip_points_length(
    R: *const xcb_render_tri_strip_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_tri_strip_points_iterator(
    R: *const xcb_render_tri_strip_request_t
  ) -> xcb_render_pointfix_iterator_t;

  pub fn xcb_render_tri_fan_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    points_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_tri_fan_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    points_len: u32,
    points: *const xcb_render_pointfix_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_tri_fan(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    src_x: i16,
    src_y: i16,
    points_len: u32,
    points: *const xcb_render_pointfix_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_tri_fan_points(
    R: *const xcb_render_tri_fan_request_t
  ) -> *mut xcb_render_pointfix_t;

  pub fn xcb_render_tri_fan_points_length(
    R: *const xcb_render_tri_fan_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_tri_fan_points_iterator(
    R: *const xcb_render_tri_fan_request_t
  ) -> xcb_render_pointfix_iterator_t;

  pub fn xcb_render_create_glyph_set_checked(
    c: *mut xcb_connection_t,
    gsid: xcb_render_glyphset_t,
    format: xcb_render_pictformat_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_glyph_set(
    c: *mut xcb_connection_t,
    gsid: xcb_render_glyphset_t,
    format: xcb_render_pictformat_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_reference_glyph_set_checked(
    c: *mut xcb_connection_t,
    gsid: xcb_render_glyphset_t,
    existing: xcb_render_glyphset_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_reference_glyph_set(
    c: *mut xcb_connection_t,
    gsid: xcb_render_glyphset_t,
    existing: xcb_render_glyphset_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_free_glyph_set_checked(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_free_glyph_set(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_add_glyphs_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_glyphs_checked(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
    glyphs_len: u32,
    glyphids: *const u32,
    glyphs: *const xcb_render_glyphinfo_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_add_glyphs(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
    glyphs_len: u32,
    glyphids: *const u32,
    glyphs: *const xcb_render_glyphinfo_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_add_glyphs_glyphids(R: *const xcb_render_add_glyphs_request_t) -> *mut u32;

  pub fn xcb_render_add_glyphs_glyphids_length(
    R: *const xcb_render_add_glyphs_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_glyphs_glyphids_end(
    R: *const xcb_render_add_glyphs_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_add_glyphs_glyphs(
    R: *const xcb_render_add_glyphs_request_t
  ) -> *mut xcb_render_glyphinfo_t;

  pub fn xcb_render_add_glyphs_glyphs_length(
    R: *const xcb_render_add_glyphs_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_glyphs_glyphs_iterator(
    R: *const xcb_render_add_glyphs_request_t
  ) -> xcb_render_glyphinfo_iterator_t;

  pub fn xcb_render_add_glyphs_data(R: *const xcb_render_add_glyphs_request_t) -> *mut u8;

  pub fn xcb_render_add_glyphs_data_length(
    R: *const xcb_render_add_glyphs_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_glyphs_data_end(
    R: *const xcb_render_add_glyphs_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_free_glyphs_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    glyphs_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_free_glyphs_checked(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
    glyphs_len: u32,
    glyphs: *const xcb_render_glyph_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_free_glyphs(
    c: *mut xcb_connection_t,
    glyphset: xcb_render_glyphset_t,
    glyphs_len: u32,
    glyphs: *const xcb_render_glyph_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_free_glyphs_glyphs(
    R: *const xcb_render_free_glyphs_request_t
  ) -> *mut xcb_render_glyph_t;

  pub fn xcb_render_free_glyphs_glyphs_length(
    R: *const xcb_render_free_glyphs_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_free_glyphs_glyphs_end(
    R: *const xcb_render_free_glyphs_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_composite_glyphs_8_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    glyphcmds_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_8_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_8(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_8_glyphcmds(
    R: *const xcb_render_composite_glyphs_8_request_t
  ) -> *mut u8;

  pub fn xcb_render_composite_glyphs_8_glyphcmds_length(
    R: *const xcb_render_composite_glyphs_8_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_8_glyphcmds_end(
    R: *const xcb_render_composite_glyphs_8_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_composite_glyphs_16_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    glyphcmds_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_16_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_16(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_16_glyphcmds(
    R: *const xcb_render_composite_glyphs_16_request_t
  ) -> *mut u8;

  pub fn xcb_render_composite_glyphs_16_glyphcmds_length(
    R: *const xcb_render_composite_glyphs_16_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_16_glyphcmds_end(
    R: *const xcb_render_composite_glyphs_16_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_composite_glyphs_32_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    glyphcmds_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_32_checked(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_32(
    c: *mut xcb_connection_t,
    op: u8,
    src: xcb_render_picture_t,
    dst: xcb_render_picture_t,
    mask_format: xcb_render_pictformat_t,
    glyphset: xcb_render_glyphset_t,
    src_x: i16,
    src_y: i16,
    glyphcmds_len: u32,
    glyphcmds: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_composite_glyphs_32_glyphcmds(
    R: *const xcb_render_composite_glyphs_32_request_t
  ) -> *mut u8;

  pub fn xcb_render_composite_glyphs_32_glyphcmds_length(
    R: *const xcb_render_composite_glyphs_32_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_composite_glyphs_32_glyphcmds_end(
    R: *const xcb_render_composite_glyphs_32_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_fill_rectangles_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rects_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_fill_rectangles_checked(
    c: *mut xcb_connection_t,
    op: u8,
    dst: xcb_render_picture_t,
    color: xcb_render_color_t,
    rects_len: u32,
    rects: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_fill_rectangles(
    c: *mut xcb_connection_t,
    op: u8,
    dst: xcb_render_picture_t,
    color: xcb_render_color_t,
    rects_len: u32,
    rects: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_fill_rectangles_rects(
    R: *const xcb_render_fill_rectangles_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_render_fill_rectangles_rects_length(
    R: *const xcb_render_fill_rectangles_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_fill_rectangles_rects_iterator(
    R: *const xcb_render_fill_rectangles_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_render_create_cursor_checked(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source: xcb_render_picture_t,
    x: u16,
    y: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_cursor(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source: xcb_render_picture_t,
    x: u16,
    y: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_transform_next(i: *mut xcb_render_transform_iterator_t);

  pub fn xcb_render_transform_end(i: xcb_render_transform_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_set_picture_transform_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    transform: xcb_render_transform_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_set_picture_transform(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    transform: xcb_render_transform_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_query_filters_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_filters(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_render_query_filters_cookie_t;

  pub fn xcb_render_query_filters_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_render_query_filters_cookie_t;

  pub fn xcb_render_query_filters_aliases(R: *const xcb_render_query_filters_reply_t) -> *mut u16;

  pub fn xcb_render_query_filters_aliases_length(
    R: *const xcb_render_query_filters_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_filters_aliases_end(
    R: *const xcb_render_query_filters_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_query_filters_filters_length(
    R: *const xcb_render_query_filters_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_query_filters_filters_iterator(
    R: *const xcb_render_query_filters_reply_t
  ) -> xcb_str_iterator_t;

  pub fn xcb_render_query_filters_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_render_query_filters_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_render_query_filters_reply_t;

  pub fn xcb_render_set_picture_filter_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    values_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_set_picture_filter_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    filter_len: u16,
    filter: *const ::std::os::raw::c_char,
    values_len: u32,
    values: *const xcb_render_fixed_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_set_picture_filter(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    filter_len: u16,
    filter: *const ::std::os::raw::c_char,
    values_len: u32,
    values: *const xcb_render_fixed_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_set_picture_filter_filter(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_render_set_picture_filter_filter_length(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_set_picture_filter_filter_end(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_set_picture_filter_values(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_render_set_picture_filter_values_length(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_set_picture_filter_values_end(
    R: *const xcb_render_set_picture_filter_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_animcursorelt_next(i: *mut xcb_render_animcursorelt_iterator_t);

  pub fn xcb_render_animcursorelt_end(
    i: xcb_render_animcursorelt_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_create_anim_cursor_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    cursors_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_anim_cursor_checked(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    cursors_len: u32,
    cursors: *const xcb_render_animcursorelt_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_anim_cursor(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    cursors_len: u32,
    cursors: *const xcb_render_animcursorelt_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_anim_cursor_cursors(
    R: *const xcb_render_create_anim_cursor_request_t
  ) -> *mut xcb_render_animcursorelt_t;

  pub fn xcb_render_create_anim_cursor_cursors_length(
    R: *const xcb_render_create_anim_cursor_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_anim_cursor_cursors_iterator(
    R: *const xcb_render_create_anim_cursor_request_t
  ) -> xcb_render_animcursorelt_iterator_t;

  pub fn xcb_render_spanfix_next(i: *mut xcb_render_spanfix_iterator_t);

  pub fn xcb_render_spanfix_end(i: xcb_render_spanfix_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_trap_next(i: *mut xcb_render_trap_iterator_t);

  pub fn xcb_render_trap_end(i: xcb_render_trap_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_render_add_traps_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    traps_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_traps_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    x_off: i16,
    y_off: i16,
    traps_len: u32,
    traps: *const xcb_render_trap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_add_traps(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    x_off: i16,
    y_off: i16,
    traps_len: u32,
    traps: *const xcb_render_trap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_add_traps_traps(
    R: *const xcb_render_add_traps_request_t
  ) -> *mut xcb_render_trap_t;

  pub fn xcb_render_add_traps_traps_length(
    R: *const xcb_render_add_traps_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_add_traps_traps_iterator(
    R: *const xcb_render_add_traps_request_t
  ) -> xcb_render_trap_iterator_t;

  pub fn xcb_render_create_solid_fill_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    color: xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_solid_fill(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    color: xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_linear_gradient_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_linear_gradient_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    p1: xcb_render_pointfix_t,
    p2: xcb_render_pointfix_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_linear_gradient(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    p1: xcb_render_pointfix_t,
    p2: xcb_render_pointfix_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_linear_gradient_stops(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_render_create_linear_gradient_stops_length(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_linear_gradient_stops_end(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_create_linear_gradient_colors(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> *mut xcb_render_color_t;

  pub fn xcb_render_create_linear_gradient_colors_length(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_linear_gradient_colors_iterator(
    R: *const xcb_render_create_linear_gradient_request_t
  ) -> xcb_render_color_iterator_t;

  pub fn xcb_render_create_radial_gradient_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_radial_gradient_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    inner: xcb_render_pointfix_t,
    outer: xcb_render_pointfix_t,
    inner_radius: xcb_render_fixed_t,
    outer_radius: xcb_render_fixed_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_radial_gradient(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    inner: xcb_render_pointfix_t,
    outer: xcb_render_pointfix_t,
    inner_radius: xcb_render_fixed_t,
    outer_radius: xcb_render_fixed_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_radial_gradient_stops(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_render_create_radial_gradient_stops_length(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_radial_gradient_stops_end(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_create_radial_gradient_colors(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> *mut xcb_render_color_t;

  pub fn xcb_render_create_radial_gradient_colors_length(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_radial_gradient_colors_iterator(
    R: *const xcb_render_create_radial_gradient_request_t
  ) -> xcb_render_color_iterator_t;

  pub fn xcb_render_create_conical_gradient_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_conical_gradient_checked(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    center: xcb_render_pointfix_t,
    angle: xcb_render_fixed_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_conical_gradient(
    c: *mut xcb_connection_t,
    picture: xcb_render_picture_t,
    center: xcb_render_pointfix_t,
    angle: xcb_render_fixed_t,
    num_stops: u32,
    stops: *const xcb_render_fixed_t,
    colors: *const xcb_render_color_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_render_create_conical_gradient_stops(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_render_create_conical_gradient_stops_length(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_conical_gradient_stops_end(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_render_create_conical_gradient_colors(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> *mut xcb_render_color_t;

  pub fn xcb_render_create_conical_gradient_colors_length(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_render_create_conical_gradient_colors_iterator(
    R: *const xcb_render_create_conical_gradient_request_t
  ) -> xcb_render_color_iterator_t;
}
