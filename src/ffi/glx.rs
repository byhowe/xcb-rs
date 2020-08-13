use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_font_t, xcb_pixmap_t, xcb_visualid_t, xcb_window_t};

pub const XCB_GLX_MAJOR_VERSION: u32 = 1;
pub const XCB_GLX_MINOR_VERSION: u32 = 4;
pub const XCB_GLX_GENERIC: i32 = -1;
pub const XCB_GLX_BAD_CONTEXT: u32 = 0;
pub const XCB_GLX_BAD_CONTEXT_STATE: u32 = 1;
pub const XCB_GLX_BAD_DRAWABLE: u32 = 2;
pub const XCB_GLX_BAD_PIXMAP: u32 = 3;
pub const XCB_GLX_BAD_CONTEXT_TAG: u32 = 4;
pub const XCB_GLX_BAD_CURRENT_WINDOW: u32 = 5;
pub const XCB_GLX_BAD_RENDER_REQUEST: u32 = 6;
pub const XCB_GLX_BAD_LARGE_REQUEST: u32 = 7;
pub const XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST: u32 = 8;
pub const XCB_GLX_BAD_FB_CONFIG: u32 = 9;
pub const XCB_GLX_BAD_PBUFFER: u32 = 10;
pub const XCB_GLX_BAD_CURRENT_DRAWABLE: u32 = 11;
pub const XCB_GLX_BAD_WINDOW: u32 = 12;
pub const XCB_GLX_GLX_BAD_PROFILE_ARB: u32 = 13;
pub const XCB_GLX_PBUFFER_CLOBBER: u32 = 0;
pub const XCB_GLX_BUFFER_SWAP_COMPLETE: u32 = 1;
pub const XCB_GLX_RENDER: u32 = 1;
pub const XCB_GLX_RENDER_LARGE: u32 = 2;
pub const XCB_GLX_CREATE_CONTEXT: u32 = 3;
pub const XCB_GLX_DESTROY_CONTEXT: u32 = 4;
pub const XCB_GLX_MAKE_CURRENT: u32 = 5;
pub const XCB_GLX_IS_DIRECT: u32 = 6;
pub const XCB_GLX_QUERY_VERSION: u32 = 7;
pub const XCB_GLX_WAIT_GL: u32 = 8;
pub const XCB_GLX_WAIT_X: u32 = 9;
pub const XCB_GLX_COPY_CONTEXT: u32 = 10;
pub const XCB_GLX_SWAP_BUFFERS: u32 = 11;
pub const XCB_GLX_USE_X_FONT: u32 = 12;
pub const XCB_GLX_CREATE_GLX_PIXMAP: u32 = 13;
pub const XCB_GLX_GET_VISUAL_CONFIGS: u32 = 14;
pub const XCB_GLX_DESTROY_GLX_PIXMAP: u32 = 15;
pub const XCB_GLX_VENDOR_PRIVATE: u32 = 16;
pub const XCB_GLX_VENDOR_PRIVATE_WITH_REPLY: u32 = 17;
pub const XCB_GLX_QUERY_EXTENSIONS_STRING: u32 = 18;
pub const XCB_GLX_QUERY_SERVER_STRING: u32 = 19;
pub const XCB_GLX_CLIENT_INFO: u32 = 20;
pub const XCB_GLX_GET_FB_CONFIGS: u32 = 21;
pub const XCB_GLX_CREATE_PIXMAP: u32 = 22;
pub const XCB_GLX_DESTROY_PIXMAP: u32 = 23;
pub const XCB_GLX_CREATE_NEW_CONTEXT: u32 = 24;
pub const XCB_GLX_QUERY_CONTEXT: u32 = 25;
pub const XCB_GLX_MAKE_CONTEXT_CURRENT: u32 = 26;
pub const XCB_GLX_CREATE_PBUFFER: u32 = 27;
pub const XCB_GLX_DESTROY_PBUFFER: u32 = 28;
pub const XCB_GLX_GET_DRAWABLE_ATTRIBUTES: u32 = 29;
pub const XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES: u32 = 30;
pub const XCB_GLX_CREATE_WINDOW: u32 = 31;
pub const XCB_GLX_DELETE_WINDOW: u32 = 32;
pub const XCB_GLX_SET_CLIENT_INFO_ARB: u32 = 33;
pub const XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB: u32 = 34;
pub const XCB_GLX_SET_CLIENT_INFO_2ARB: u32 = 35;
pub const XCB_GLX_NEW_LIST: u32 = 101;
pub const XCB_GLX_END_LIST: u32 = 102;
pub const XCB_GLX_DELETE_LISTS: u32 = 103;
pub const XCB_GLX_GEN_LISTS: u32 = 104;
pub const XCB_GLX_FEEDBACK_BUFFER: u32 = 105;
pub const XCB_GLX_SELECT_BUFFER: u32 = 106;
pub const XCB_GLX_RENDER_MODE: u32 = 107;
pub const XCB_GLX_FINISH: u32 = 108;
pub const XCB_GLX_PIXEL_STOREF: u32 = 109;
pub const XCB_GLX_PIXEL_STOREI: u32 = 110;
pub const XCB_GLX_READ_PIXELS: u32 = 111;
pub const XCB_GLX_GET_BOOLEANV: u32 = 112;
pub const XCB_GLX_GET_CLIP_PLANE: u32 = 113;
pub const XCB_GLX_GET_DOUBLEV: u32 = 114;
pub const XCB_GLX_GET_ERROR: u32 = 115;
pub const XCB_GLX_GET_FLOATV: u32 = 116;
pub const XCB_GLX_GET_INTEGERV: u32 = 117;
pub const XCB_GLX_GET_LIGHTFV: u32 = 118;
pub const XCB_GLX_GET_LIGHTIV: u32 = 119;
pub const XCB_GLX_GET_MAPDV: u32 = 120;
pub const XCB_GLX_GET_MAPFV: u32 = 121;
pub const XCB_GLX_GET_MAPIV: u32 = 122;
pub const XCB_GLX_GET_MATERIALFV: u32 = 123;
pub const XCB_GLX_GET_MATERIALIV: u32 = 124;
pub const XCB_GLX_GET_PIXEL_MAPFV: u32 = 125;
pub const XCB_GLX_GET_PIXEL_MAPUIV: u32 = 126;
pub const XCB_GLX_GET_PIXEL_MAPUSV: u32 = 127;
pub const XCB_GLX_GET_POLYGON_STIPPLE: u32 = 128;
pub const XCB_GLX_GET_STRING: u32 = 129;
pub const XCB_GLX_GET_TEX_ENVFV: u32 = 130;
pub const XCB_GLX_GET_TEX_ENVIV: u32 = 131;
pub const XCB_GLX_GET_TEX_GENDV: u32 = 132;
pub const XCB_GLX_GET_TEX_GENFV: u32 = 133;
pub const XCB_GLX_GET_TEX_GENIV: u32 = 134;
pub const XCB_GLX_GET_TEX_IMAGE: u32 = 135;
pub const XCB_GLX_GET_TEX_PARAMETERFV: u32 = 136;
pub const XCB_GLX_GET_TEX_PARAMETERIV: u32 = 137;
pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERFV: u32 = 138;
pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERIV: u32 = 139;
pub const XCB_GLX_IS_ENABLED: u32 = 140;
pub const XCB_GLX_IS_LIST: u32 = 141;
pub const XCB_GLX_FLUSH: u32 = 142;
pub const XCB_GLX_ARE_TEXTURES_RESIDENT: u32 = 143;
pub const XCB_GLX_DELETE_TEXTURES: u32 = 144;
pub const XCB_GLX_GEN_TEXTURES: u32 = 145;
pub const XCB_GLX_IS_TEXTURE: u32 = 146;
pub const XCB_GLX_GET_COLOR_TABLE: u32 = 147;
pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERFV: u32 = 148;
pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERIV: u32 = 149;
pub const XCB_GLX_GET_CONVOLUTION_FILTER: u32 = 150;
pub const XCB_GLX_GET_CONVOLUTION_PARAMETERFV: u32 = 151;
pub const XCB_GLX_GET_CONVOLUTION_PARAMETERIV: u32 = 152;
pub const XCB_GLX_GET_SEPARABLE_FILTER: u32 = 153;
pub const XCB_GLX_GET_HISTOGRAM: u32 = 154;
pub const XCB_GLX_GET_HISTOGRAM_PARAMETERFV: u32 = 155;
pub const XCB_GLX_GET_HISTOGRAM_PARAMETERIV: u32 = 156;
pub const XCB_GLX_GET_MINMAX: u32 = 157;
pub const XCB_GLX_GET_MINMAX_PARAMETERFV: u32 = 158;
pub const XCB_GLX_GET_MINMAX_PARAMETERIV: u32 = 159;
pub const XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB: u32 = 160;
pub const XCB_GLX_DELETE_QUERIES_ARB: u32 = 161;
pub const XCB_GLX_GEN_QUERIES_ARB: u32 = 162;
pub const XCB_GLX_IS_QUERY_ARB: u32 = 163;
pub const XCB_GLX_GET_QUERYIV_ARB: u32 = 164;
pub const XCB_GLX_GET_QUERY_OBJECTIV_ARB: u32 = 165;
pub const XCB_GLX_GET_QUERY_OBJECTUIV_ARB: u32 = 166;

pub type xcb_glx_pixmap_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_pixmap_iterator_t
{
  pub data: *mut xcb_glx_pixmap_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_context_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_context_iterator_t
{
  pub data: *mut xcb_glx_context_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_pbuffer_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_pbuffer_iterator_t
{
  pub data: *mut xcb_glx_pbuffer_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_window_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_window_iterator_t
{
  pub data: *mut xcb_glx_window_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_fbconfig_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_fbconfig_iterator_t
{
  pub data: *mut xcb_glx_fbconfig_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_drawable_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_drawable_iterator_t
{
  pub data: *mut xcb_glx_drawable_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_float32_t = f32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_float32_iterator_t
{
  pub data: *mut xcb_glx_float32_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_float64_t = f64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_float64_iterator_t
{
  pub data: *mut xcb_glx_float64_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_bool32_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_bool32_iterator_t
{
  pub data: *mut xcb_glx_bool32_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_glx_context_tag_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_context_tag_iterator_t
{
  pub data: *mut xcb_glx_context_tag_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_generic_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub bad_value: u32,
  pub minor_opcode: u16,
  pub major_opcode: u8,
  pub pad0: [u8; 21usize],
}

pub type xcb_glx_bad_context_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_context_state_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_drawable_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_pixmap_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_context_tag_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_current_window_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_render_request_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_large_request_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_unsupported_private_request_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_fb_config_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_pbuffer_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_current_drawable_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_bad_window_error_t = xcb_glx_generic_error_t;

pub type xcb_glx_glx_bad_profile_arb_error_t = xcb_glx_generic_error_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_pbuffer_clobber_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event_type: u16,
  pub draw_type: u16,
  pub drawable: xcb_glx_drawable_t,
  pub b_mask: u32,
  pub aux_buffer: u16,
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
  pub count: u16,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_buffer_swap_complete_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event_type: u16,
  pub pad1: [u8; 2usize],
  pub drawable: xcb_glx_drawable_t,
  pub ust_hi: u32,
  pub ust_lo: u32,
  pub msc_hi: u32,
  pub msc_lo: u32,
  pub sbc: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_glx_pbcet_t
{
  DAMAGED = 32791,
  SAVED = 32792,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_glx_pbcdt_t
{
  WINDOW = 32793,
  PBUFFER = 32794,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_render_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_render_large_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub request_num: u16,
  pub request_total: u16,
  pub data_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
  pub visual: xcb_visualid_t,
  pub screen: u32,
  pub share_list: xcb_glx_context_t,
  pub is_direct: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_destroy_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_current_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_current_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_glx_drawable_t,
  pub context: xcb_glx_context_t,
  pub old_context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_current_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_tag: xcb_glx_context_tag_t,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_direct_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_direct_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_direct_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub is_direct: u8,
  pub pad1: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_version_reply_t
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
pub struct xcb_glx_wait_gl_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_wait_x_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_copy_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub src: xcb_glx_context_t,
  pub dest: xcb_glx_context_t,
  pub mask: u32,
  pub src_context_tag: xcb_glx_context_tag_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_glx_gc_t
{
  GL_CURRENT_BIT = 1,
  GL_POINT_BIT = 2,
  GL_LINE_BIT = 4,
  GL_POLYGON_BIT = 8,
  GL_POLYGON_STIPPLE_BIT = 16,
  GL_PIXEL_MODE_BIT = 32,
  GL_LIGHTING_BIT = 64,
  GL_FOG_BIT = 128,
  GL_DEPTH_BUFFER_BIT = 256,
  GL_ACCUM_BUFFER_BIT = 512,
  GL_STENCIL_BUFFER_BIT = 1024,
  GL_VIEWPORT_BIT = 2048,
  GL_TRANSFORM_BIT = 4096,
  GL_ENABLE_BIT = 8192,
  GL_COLOR_BUFFER_BIT = 16384,
  GL_HINT_BIT = 32768,
  GL_EVAL_BIT = 65536,
  GL_LIST_BIT = 131072,
  GL_TEXTURE_BIT = 262144,
  GL_SCISSOR_BIT = 524288,
  GL_ALL_ATTRIB_BITS = 16777215,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_swap_buffers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub drawable: xcb_glx_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_use_x_font_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub font: xcb_font_t,
  pub first: u32,
  pub count: u32,
  pub list_base: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_glx_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub visual: xcb_visualid_t,
  pub pixmap: xcb_pixmap_t,
  pub glx_pixmap: xcb_glx_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_visual_configs_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_visual_configs_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_visual_configs_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_visuals: u32,
  pub num_properties: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_destroy_glx_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glx_pixmap: xcb_glx_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_vendor_private_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub vendor_code: u32,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_vendor_private_with_reply_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_vendor_private_with_reply_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub vendor_code: u32,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_vendor_private_with_reply_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub retval: u32,
  pub data1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_extensions_string_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_extensions_string_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_extensions_string_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub pad2: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_server_string_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_server_string_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub name: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_server_string_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub str_len: u32,
  pub pad2: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_client_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
  pub str_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_fb_configs_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_fb_configs_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_fb_configs_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_FB_configs: u32,
  pub num_properties: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub fbconfig: xcb_glx_fbconfig_t,
  pub pixmap: xcb_pixmap_t,
  pub glx_pixmap: xcb_glx_pixmap_t,
  pub num_attribs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_destroy_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glx_pixmap: xcb_glx_pixmap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_new_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
  pub fbconfig: xcb_glx_fbconfig_t,
  pub screen: u32,
  pub render_type: u32,
  pub share_list: xcb_glx_context_t,
  pub is_direct: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_query_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_attribs: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_context_current_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_context_current_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub old_context_tag: xcb_glx_context_tag_t,
  pub drawable: xcb_glx_drawable_t,
  pub read_drawable: xcb_glx_drawable_t,
  pub context: xcb_glx_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_make_context_current_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_tag: xcb_glx_context_tag_t,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_pbuffer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub fbconfig: xcb_glx_fbconfig_t,
  pub pbuffer: xcb_glx_pbuffer_t,
  pub num_attribs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_destroy_pbuffer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub pbuffer: xcb_glx_pbuffer_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_drawable_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_drawable_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_glx_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_drawable_attributes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_attribs: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_change_drawable_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_glx_drawable_t,
  pub num_attribs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub screen: u32,
  pub fbconfig: xcb_glx_fbconfig_t,
  pub window: xcb_window_t,
  pub glx_window: xcb_glx_window_t,
  pub num_attribs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_delete_window_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub glxwindow: xcb_glx_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_set_client_info_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
  pub num_versions: u32,
  pub gl_str_len: u32,
  pub glx_str_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_create_context_attribs_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_glx_context_t,
  pub fbconfig: xcb_glx_fbconfig_t,
  pub screen: u32,
  pub share_list: xcb_glx_context_t,
  pub is_direct: u8,
  pub pad0: [u8; 3usize],
  pub num_attribs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_set_client_info_2arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
  pub num_versions: u32,
  pub gl_str_len: u32,
  pub glx_str_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_new_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub list: u32,
  pub mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_end_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_delete_lists_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub list: u32,
  pub range: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_lists_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_lists_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub range: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_lists_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_feedback_buffer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub size: i32,
  pub type_: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_select_buffer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub size: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_render_mode_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_render_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_render_mode_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: u32,
  pub n: u32,
  pub new_mode: u32,
  pub pad1: [u8; 12usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_glx_rm_t
{
  GL_RENDER = 7168,
  GL_FEEDBACK = 7169,
  GL_SELECT = 7170,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_finish_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_finish_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_finish_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_pixel_storef_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: u32,
  pub datum: xcb_glx_float32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_pixel_storei_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: u32,
  pub datum: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_read_pixels_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_read_pixels_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
  pub lsb_first: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_read_pixels_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_booleanv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_booleanv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_booleanv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: u8,
  pub pad2: [u8; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_clip_plane_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_clip_plane_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub plane: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_clip_plane_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_doublev_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_doublev_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_doublev_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float64_t,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_error_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_error_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_error_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub error: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_floatv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_floatv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_floatv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_integerv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_integerv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_integerv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub light: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightiv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightiv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub light: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_lightiv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapdv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapdv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub query: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapdv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float64_t,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub query: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapiv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapiv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub query: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_mapiv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub face: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialiv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialiv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub face: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_materialiv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub map: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapuiv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapuiv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub map: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapuiv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: u32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapusv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapusv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub map: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_pixel_mapusv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: u16,
  pub pad2: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_polygon_stipple_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_polygon_stipple_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub lsb_first: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_polygon_stipple_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_string_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_string_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub name: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_string_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub pad2: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_envfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_envfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_envfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_enviv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_enviv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_enviv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_gendv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_gendv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub coord: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_gendv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float64_t,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_genfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_genfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub coord: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_genfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_geniv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_geniv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub coord: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_geniv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_image_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_image_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub level: i32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_image_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub width: i32,
  pub height: i32,
  pub depth: i32,
  pub pad2: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub level: i32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub level: i32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_tex_level_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_enabled_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_enabled_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub capability: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_enabled_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: xcb_glx_bool32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_list_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub list: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_list_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: xcb_glx_bool32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_flush_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_are_textures_resident_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_are_textures_resident_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub n: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_are_textures_resident_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: xcb_glx_bool32_t,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_delete_textures_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub n: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_textures_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_textures_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub n: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_textures_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_texture_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_texture_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub texture: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_texture_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: xcb_glx_bool32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub width: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_color_table_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_filter_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_filter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_filter_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub width: i32,
  pub height: i32,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_convolution_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_separable_filter_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_separable_filter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_separable_filter_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub row_w: i32,
  pub col_h: i32,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
  pub reset: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub width: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_histogram_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub format: u32,
  pub type_: u32,
  pub swap_bytes: u8,
  pub reset: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameterfv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameterfv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameterfv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: xcb_glx_float32_t,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameteriv_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameteriv_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_minmax_parameteriv_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_compressed_tex_image_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_compressed_tex_image_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub level: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_compressed_tex_image_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 8usize],
  pub size: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_delete_queries_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub n: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_queries_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_queries_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub n: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_gen_queries_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_query_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_query_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_is_query_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub ret_val: xcb_glx_bool32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_queryiv_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_queryiv_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub target: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_queryiv_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectiv_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectiv_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub id: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectiv_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: i32,
  pub pad2: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectuiv_arb_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectuiv_arb_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_tag: xcb_glx_context_tag_t,
  pub id: u32,
  pub pname: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_glx_get_query_objectuiv_arb_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad1: [u8; 4usize],
  pub n: u32,
  pub datum: u32,
  pub pad2: [u8; 12usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_glx_id: xcb_extension_t;

  pub fn xcb_glx_pixmap_next(i: *mut xcb_glx_pixmap_iterator_t);

  pub fn xcb_glx_pixmap_end(i: xcb_glx_pixmap_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_context_next(i: *mut xcb_glx_context_iterator_t);

  pub fn xcb_glx_context_end(i: xcb_glx_context_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_pbuffer_next(i: *mut xcb_glx_pbuffer_iterator_t);

  pub fn xcb_glx_pbuffer_end(i: xcb_glx_pbuffer_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_window_next(i: *mut xcb_glx_window_iterator_t);

  pub fn xcb_glx_window_end(i: xcb_glx_window_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_fbconfig_next(i: *mut xcb_glx_fbconfig_iterator_t);

  pub fn xcb_glx_fbconfig_end(i: xcb_glx_fbconfig_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_drawable_next(i: *mut xcb_glx_drawable_iterator_t);

  pub fn xcb_glx_drawable_end(i: xcb_glx_drawable_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_float32_next(i: *mut xcb_glx_float32_iterator_t);

  pub fn xcb_glx_float32_end(i: xcb_glx_float32_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_float64_next(i: *mut xcb_glx_float64_iterator_t);

  pub fn xcb_glx_float64_end(i: xcb_glx_float64_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_bool32_next(i: *mut xcb_glx_bool32_iterator_t);

  pub fn xcb_glx_bool32_end(i: xcb_glx_bool32_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_context_tag_next(i: *mut xcb_glx_context_tag_iterator_t);

  pub fn xcb_glx_context_tag_end(i: xcb_glx_context_tag_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_render_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_render(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_render_data(R: *const xcb_glx_render_request_t) -> *mut u8;

  pub fn xcb_glx_render_data_length(R: *const xcb_glx_render_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_data_end(R: *const xcb_glx_render_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_render_large_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_large_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    request_num: u16,
    request_total: u16,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_render_large(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    request_num: u16,
    request_total: u16,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_render_large_data(R: *const xcb_glx_render_large_request_t) -> *mut u8;

  pub fn xcb_glx_render_large_data_length(
    R: *const xcb_glx_render_large_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_large_data_end(
    R: *const xcb_glx_render_large_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_create_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    visual: xcb_visualid_t,
    screen: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_context(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    visual: xcb_visualid_t,
    screen: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_destroy_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_destroy_context(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_make_current(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
    context: xcb_glx_context_t,
    old_context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_make_current_cookie_t;

  pub fn xcb_glx_make_current_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
    context: xcb_glx_context_t,
    old_context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_make_current_cookie_t;

  pub fn xcb_glx_make_current_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_make_current_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_make_current_reply_t;

  pub fn xcb_glx_is_direct(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_is_direct_cookie_t;

  pub fn xcb_glx_is_direct_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_is_direct_cookie_t;

  pub fn xcb_glx_is_direct_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_is_direct_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_is_direct_reply_t;

  pub fn xcb_glx_query_version(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_glx_query_version_cookie_t;

  pub fn xcb_glx_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_glx_query_version_cookie_t;

  pub fn xcb_glx_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_query_version_reply_t;

  pub fn xcb_glx_wait_gl_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_wait_gl(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_wait_x_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_wait_x(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_copy_context_checked(
    c: *mut xcb_connection_t,
    src: xcb_glx_context_t,
    dest: xcb_glx_context_t,
    mask: u32,
    src_context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_copy_context(
    c: *mut xcb_connection_t,
    src: xcb_glx_context_t,
    dest: xcb_glx_context_t,
    mask: u32,
    src_context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_swap_buffers_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    drawable: xcb_glx_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_swap_buffers(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    drawable: xcb_glx_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_use_x_font_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    font: xcb_font_t,
    first: u32,
    count: u32,
    list_base: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_use_x_font(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    font: xcb_font_t,
    first: u32,
    count: u32,
    list_base: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_glx_pixmap_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    visual: xcb_visualid_t,
    pixmap: xcb_pixmap_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_glx_pixmap(
    c: *mut xcb_connection_t,
    screen: u32,
    visual: xcb_visualid_t,
    pixmap: xcb_pixmap_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_get_visual_configs_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_visual_configs(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_get_visual_configs_cookie_t;

  pub fn xcb_glx_get_visual_configs_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_get_visual_configs_cookie_t;

  pub fn xcb_glx_get_visual_configs_property_list(
    R: *const xcb_glx_get_visual_configs_reply_t
  ) -> *mut u32;

  pub fn xcb_glx_get_visual_configs_property_list_length(
    R: *const xcb_glx_get_visual_configs_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_visual_configs_property_list_end(
    R: *const xcb_glx_get_visual_configs_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_visual_configs_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_visual_configs_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_visual_configs_reply_t;

  pub fn xcb_glx_destroy_glx_pixmap_checked(
    c: *mut xcb_connection_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_destroy_glx_pixmap(
    c: *mut xcb_connection_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_vendor_private_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_vendor_private_checked(
    c: *mut xcb_connection_t,
    vendor_code: u32,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_vendor_private(
    c: *mut xcb_connection_t,
    vendor_code: u32,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_vendor_private_data(R: *const xcb_glx_vendor_private_request_t) -> *mut u8;

  pub fn xcb_glx_vendor_private_data_length(
    R: *const xcb_glx_vendor_private_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_vendor_private_data_end(
    R: *const xcb_glx_vendor_private_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_vendor_private_with_reply_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_vendor_private_with_reply(
    c: *mut xcb_connection_t,
    vendor_code: u32,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_glx_vendor_private_with_reply_cookie_t;

  pub fn xcb_glx_vendor_private_with_reply_unchecked(
    c: *mut xcb_connection_t,
    vendor_code: u32,
    context_tag: xcb_glx_context_tag_t,
    data_len: u32,
    data: *const u8,
  ) -> xcb_glx_vendor_private_with_reply_cookie_t;

  pub fn xcb_glx_vendor_private_with_reply_data_2(
    R: *const xcb_glx_vendor_private_with_reply_reply_t
  ) -> *mut u8;

  pub fn xcb_glx_vendor_private_with_reply_data_2_length(
    R: *const xcb_glx_vendor_private_with_reply_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_vendor_private_with_reply_data_2_end(
    R: *const xcb_glx_vendor_private_with_reply_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_vendor_private_with_reply_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_vendor_private_with_reply_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_vendor_private_with_reply_reply_t;

  pub fn xcb_glx_query_extensions_string(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_query_extensions_string_cookie_t;

  pub fn xcb_glx_query_extensions_string_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_query_extensions_string_cookie_t;

  pub fn xcb_glx_query_extensions_string_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_query_extensions_string_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_query_extensions_string_reply_t;

  pub fn xcb_glx_query_server_string_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_query_server_string(
    c: *mut xcb_connection_t,
    screen: u32,
    name: u32,
  ) -> xcb_glx_query_server_string_cookie_t;

  pub fn xcb_glx_query_server_string_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
    name: u32,
  ) -> xcb_glx_query_server_string_cookie_t;

  pub fn xcb_glx_query_server_string_string(
    R: *const xcb_glx_query_server_string_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_query_server_string_string_length(
    R: *const xcb_glx_query_server_string_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_query_server_string_string_end(
    R: *const xcb_glx_query_server_string_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_query_server_string_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_query_server_string_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_query_server_string_reply_t;

  pub fn xcb_glx_client_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_client_info_checked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    str_len: u32,
    string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_client_info(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    str_len: u32,
    string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_client_info_string(
    R: *const xcb_glx_client_info_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_client_info_string_length(
    R: *const xcb_glx_client_info_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_client_info_string_end(
    R: *const xcb_glx_client_info_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_fb_configs_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_fb_configs(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_get_fb_configs_cookie_t;

  pub fn xcb_glx_get_fb_configs_unchecked(
    c: *mut xcb_connection_t,
    screen: u32,
  ) -> xcb_glx_get_fb_configs_cookie_t;

  pub fn xcb_glx_get_fb_configs_property_list(R: *const xcb_glx_get_fb_configs_reply_t)
    -> *mut u32;

  pub fn xcb_glx_get_fb_configs_property_list_length(
    R: *const xcb_glx_get_fb_configs_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_fb_configs_property_list_end(
    R: *const xcb_glx_get_fb_configs_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_fb_configs_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_fb_configs_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_fb_configs_reply_t;

  pub fn xcb_glx_create_pixmap_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_pixmap_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    pixmap: xcb_pixmap_t,
    glx_pixmap: xcb_glx_pixmap_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_pixmap(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    pixmap: xcb_pixmap_t,
    glx_pixmap: xcb_glx_pixmap_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_pixmap_attribs(R: *const xcb_glx_create_pixmap_request_t) -> *mut u32;

  pub fn xcb_glx_create_pixmap_attribs_length(
    R: *const xcb_glx_create_pixmap_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_pixmap_attribs_end(
    R: *const xcb_glx_create_pixmap_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_destroy_pixmap_checked(
    c: *mut xcb_connection_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_destroy_pixmap(
    c: *mut xcb_connection_t,
    glx_pixmap: xcb_glx_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_new_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    fbconfig: xcb_glx_fbconfig_t,
    screen: u32,
    render_type: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_new_context(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    fbconfig: xcb_glx_fbconfig_t,
    screen: u32,
    render_type: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_query_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_query_context(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_query_context_cookie_t;

  pub fn xcb_glx_query_context_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_query_context_cookie_t;

  pub fn xcb_glx_query_context_attribs(R: *const xcb_glx_query_context_reply_t) -> *mut u32;

  pub fn xcb_glx_query_context_attribs_length(
    R: *const xcb_glx_query_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_query_context_attribs_end(
    R: *const xcb_glx_query_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_query_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_query_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_query_context_reply_t;

  pub fn xcb_glx_make_context_current(
    c: *mut xcb_connection_t,
    old_context_tag: xcb_glx_context_tag_t,
    drawable: xcb_glx_drawable_t,
    read_drawable: xcb_glx_drawable_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_make_context_current_cookie_t;

  pub fn xcb_glx_make_context_current_unchecked(
    c: *mut xcb_connection_t,
    old_context_tag: xcb_glx_context_tag_t,
    drawable: xcb_glx_drawable_t,
    read_drawable: xcb_glx_drawable_t,
    context: xcb_glx_context_t,
  ) -> xcb_glx_make_context_current_cookie_t;

  pub fn xcb_glx_make_context_current_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_make_context_current_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_make_context_current_reply_t;

  pub fn xcb_glx_create_pbuffer_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_pbuffer_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    pbuffer: xcb_glx_pbuffer_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_pbuffer(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    pbuffer: xcb_glx_pbuffer_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_pbuffer_attribs(R: *const xcb_glx_create_pbuffer_request_t) -> *mut u32;

  pub fn xcb_glx_create_pbuffer_attribs_length(
    R: *const xcb_glx_create_pbuffer_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_pbuffer_attribs_end(
    R: *const xcb_glx_create_pbuffer_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_destroy_pbuffer_checked(
    c: *mut xcb_connection_t,
    pbuffer: xcb_glx_pbuffer_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_destroy_pbuffer(
    c: *mut xcb_connection_t,
    pbuffer: xcb_glx_pbuffer_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_get_drawable_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_drawable_attributes(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
  ) -> xcb_glx_get_drawable_attributes_cookie_t;

  pub fn xcb_glx_get_drawable_attributes_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
  ) -> xcb_glx_get_drawable_attributes_cookie_t;

  pub fn xcb_glx_get_drawable_attributes_attribs(
    R: *const xcb_glx_get_drawable_attributes_reply_t
  ) -> *mut u32;

  pub fn xcb_glx_get_drawable_attributes_attribs_length(
    R: *const xcb_glx_get_drawable_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_drawable_attributes_attribs_end(
    R: *const xcb_glx_get_drawable_attributes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_drawable_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_drawable_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_drawable_attributes_reply_t;

  pub fn xcb_glx_change_drawable_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_change_drawable_attributes_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_change_drawable_attributes(
    c: *mut xcb_connection_t,
    drawable: xcb_glx_drawable_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_change_drawable_attributes_attribs(
    R: *const xcb_glx_change_drawable_attributes_request_t
  ) -> *mut u32;

  pub fn xcb_glx_change_drawable_attributes_attribs_length(
    R: *const xcb_glx_change_drawable_attributes_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_change_drawable_attributes_attribs_end(
    R: *const xcb_glx_change_drawable_attributes_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_create_window_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_window_checked(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    window: xcb_window_t,
    glx_window: xcb_glx_window_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_window(
    c: *mut xcb_connection_t,
    screen: u32,
    fbconfig: xcb_glx_fbconfig_t,
    window: xcb_window_t,
    glx_window: xcb_glx_window_t,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_window_attribs(R: *const xcb_glx_create_window_request_t) -> *mut u32;

  pub fn xcb_glx_create_window_attribs_length(
    R: *const xcb_glx_create_window_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_window_attribs_end(
    R: *const xcb_glx_create_window_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_delete_window_checked(
    c: *mut xcb_connection_t,
    glxwindow: xcb_glx_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_window(
    c: *mut xcb_connection_t,
    glxwindow: xcb_glx_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_set_client_info_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_arb_checked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: *const u32,
    gl_extension_string: *const ::std::os::raw::c_char,
    glx_extension_string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_set_client_info_arb(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: *const u32,
    gl_extension_string: *const ::std::os::raw::c_char,
    glx_extension_string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_set_client_info_arb_gl_versions(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> *mut u32;

  pub fn xcb_glx_set_client_info_arb_gl_versions_length(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_arb_gl_versions_end(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_set_client_info_arb_gl_extension_string(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_set_client_info_arb_gl_extension_string_length(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_arb_gl_extension_string_end(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_set_client_info_arb_glx_extension_string(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_set_client_info_arb_glx_extension_string_length(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_arb_glx_extension_string_end(
    R: *const xcb_glx_set_client_info_arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_create_context_attribs_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_context_attribs_arb_checked(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    fbconfig: xcb_glx_fbconfig_t,
    screen: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_context_attribs_arb(
    c: *mut xcb_connection_t,
    context: xcb_glx_context_t,
    fbconfig: xcb_glx_fbconfig_t,
    screen: u32,
    share_list: xcb_glx_context_t,
    is_direct: u8,
    num_attribs: u32,
    attribs: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_create_context_attribs_arb_attribs(
    R: *const xcb_glx_create_context_attribs_arb_request_t
  ) -> *mut u32;

  pub fn xcb_glx_create_context_attribs_arb_attribs_length(
    R: *const xcb_glx_create_context_attribs_arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_create_context_attribs_arb_attribs_end(
    R: *const xcb_glx_create_context_attribs_arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_set_client_info_2arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_2arb_checked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: *const u32,
    gl_extension_string: *const ::std::os::raw::c_char,
    glx_extension_string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_set_client_info_2arb(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
    num_versions: u32,
    gl_str_len: u32,
    glx_str_len: u32,
    gl_versions: *const u32,
    gl_extension_string: *const ::std::os::raw::c_char,
    glx_extension_string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_set_client_info_2arb_gl_versions(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> *mut u32;

  pub fn xcb_glx_set_client_info_2arb_gl_versions_length(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_2arb_gl_versions_end(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_set_client_info_2arb_gl_extension_string(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_set_client_info_2arb_gl_extension_string_length(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_2arb_gl_extension_string_end(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_set_client_info_2arb_glx_extension_string(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_set_client_info_2arb_glx_extension_string_length(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_set_client_info_2arb_glx_extension_string_end(
    R: *const xcb_glx_set_client_info_2arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_new_list_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
    mode: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_new_list(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
    mode: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_end_list_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_end_list(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_lists_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
    range: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_lists(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
    range: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_gen_lists(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    range: i32,
  ) -> xcb_glx_gen_lists_cookie_t;

  pub fn xcb_glx_gen_lists_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    range: i32,
  ) -> xcb_glx_gen_lists_cookie_t;

  pub fn xcb_glx_gen_lists_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_gen_lists_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_gen_lists_reply_t;

  pub fn xcb_glx_feedback_buffer_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    size: i32,
    type_: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_feedback_buffer(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    size: i32,
    type_: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_select_buffer_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    size: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_select_buffer(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    size: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_render_mode_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_mode(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    mode: u32,
  ) -> xcb_glx_render_mode_cookie_t;

  pub fn xcb_glx_render_mode_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    mode: u32,
  ) -> xcb_glx_render_mode_cookie_t;

  pub fn xcb_glx_render_mode_data(R: *const xcb_glx_render_mode_reply_t) -> *mut u32;

  pub fn xcb_glx_render_mode_data_length(
    R: *const xcb_glx_render_mode_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_render_mode_data_end(
    R: *const xcb_glx_render_mode_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_render_mode_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_render_mode_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_render_mode_reply_t;

  pub fn xcb_glx_finish(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_finish_cookie_t;

  pub fn xcb_glx_finish_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_finish_cookie_t;

  pub fn xcb_glx_finish_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_finish_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_finish_reply_t;

  pub fn xcb_glx_pixel_storef_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
    datum: xcb_glx_float32_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_pixel_storef(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
    datum: xcb_glx_float32_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_pixel_storei_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
    datum: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_pixel_storei(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
    datum: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_read_pixels_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_read_pixels(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    lsb_first: u8,
  ) -> xcb_glx_read_pixels_cookie_t;

  pub fn xcb_glx_read_pixels_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    lsb_first: u8,
  ) -> xcb_glx_read_pixels_cookie_t;

  pub fn xcb_glx_read_pixels_data(R: *const xcb_glx_read_pixels_reply_t) -> *mut u8;

  pub fn xcb_glx_read_pixels_data_length(
    R: *const xcb_glx_read_pixels_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_read_pixels_data_end(
    R: *const xcb_glx_read_pixels_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_read_pixels_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_read_pixels_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_read_pixels_reply_t;

  pub fn xcb_glx_get_booleanv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_booleanv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: i32,
  ) -> xcb_glx_get_booleanv_cookie_t;

  pub fn xcb_glx_get_booleanv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: i32,
  ) -> xcb_glx_get_booleanv_cookie_t;

  pub fn xcb_glx_get_booleanv_data(R: *const xcb_glx_get_booleanv_reply_t) -> *mut u8;

  pub fn xcb_glx_get_booleanv_data_length(
    R: *const xcb_glx_get_booleanv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_booleanv_data_end(
    R: *const xcb_glx_get_booleanv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_booleanv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_booleanv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_booleanv_reply_t;

  pub fn xcb_glx_get_clip_plane_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_clip_plane(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    plane: i32,
  ) -> xcb_glx_get_clip_plane_cookie_t;

  pub fn xcb_glx_get_clip_plane_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    plane: i32,
  ) -> xcb_glx_get_clip_plane_cookie_t;

  pub fn xcb_glx_get_clip_plane_data(
    R: *const xcb_glx_get_clip_plane_reply_t
  ) -> *mut xcb_glx_float64_t;

  pub fn xcb_glx_get_clip_plane_data_length(
    R: *const xcb_glx_get_clip_plane_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_clip_plane_data_end(
    R: *const xcb_glx_get_clip_plane_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_clip_plane_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_clip_plane_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_clip_plane_reply_t;

  pub fn xcb_glx_get_doublev_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_doublev(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_doublev_cookie_t;

  pub fn xcb_glx_get_doublev_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_doublev_cookie_t;

  pub fn xcb_glx_get_doublev_data(R: *const xcb_glx_get_doublev_reply_t) -> *mut xcb_glx_float64_t;

  pub fn xcb_glx_get_doublev_data_length(
    R: *const xcb_glx_get_doublev_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_doublev_data_end(
    R: *const xcb_glx_get_doublev_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_doublev_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_doublev_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_doublev_reply_t;

  pub fn xcb_glx_get_error(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_get_error_cookie_t;

  pub fn xcb_glx_get_error_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_glx_get_error_cookie_t;

  pub fn xcb_glx_get_error_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_error_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_error_reply_t;

  pub fn xcb_glx_get_floatv_sizeof(_buffer: *const ::std::os::raw::c_void)
    -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_floatv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_floatv_cookie_t;

  pub fn xcb_glx_get_floatv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_floatv_cookie_t;

  pub fn xcb_glx_get_floatv_data(R: *const xcb_glx_get_floatv_reply_t) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_floatv_data_length(
    R: *const xcb_glx_get_floatv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_floatv_data_end(
    R: *const xcb_glx_get_floatv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_floatv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_floatv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_floatv_reply_t;

  pub fn xcb_glx_get_integerv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_integerv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_integerv_cookie_t;

  pub fn xcb_glx_get_integerv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    pname: u32,
  ) -> xcb_glx_get_integerv_cookie_t;

  pub fn xcb_glx_get_integerv_data(R: *const xcb_glx_get_integerv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_integerv_data_length(
    R: *const xcb_glx_get_integerv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_integerv_data_end(
    R: *const xcb_glx_get_integerv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_integerv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_integerv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_integerv_reply_t;

  pub fn xcb_glx_get_lightfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_lightfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    light: u32,
    pname: u32,
  ) -> xcb_glx_get_lightfv_cookie_t;

  pub fn xcb_glx_get_lightfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    light: u32,
    pname: u32,
  ) -> xcb_glx_get_lightfv_cookie_t;

  pub fn xcb_glx_get_lightfv_data(R: *const xcb_glx_get_lightfv_reply_t) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_lightfv_data_length(
    R: *const xcb_glx_get_lightfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_lightfv_data_end(
    R: *const xcb_glx_get_lightfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_lightfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_lightfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_lightfv_reply_t;

  pub fn xcb_glx_get_lightiv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_lightiv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    light: u32,
    pname: u32,
  ) -> xcb_glx_get_lightiv_cookie_t;

  pub fn xcb_glx_get_lightiv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    light: u32,
    pname: u32,
  ) -> xcb_glx_get_lightiv_cookie_t;

  pub fn xcb_glx_get_lightiv_data(R: *const xcb_glx_get_lightiv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_lightiv_data_length(
    R: *const xcb_glx_get_lightiv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_lightiv_data_end(
    R: *const xcb_glx_get_lightiv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_lightiv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_lightiv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_lightiv_reply_t;

  pub fn xcb_glx_get_mapdv_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapdv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapdv_cookie_t;

  pub fn xcb_glx_get_mapdv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapdv_cookie_t;

  pub fn xcb_glx_get_mapdv_data(R: *const xcb_glx_get_mapdv_reply_t) -> *mut xcb_glx_float64_t;

  pub fn xcb_glx_get_mapdv_data_length(
    R: *const xcb_glx_get_mapdv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapdv_data_end(R: *const xcb_glx_get_mapdv_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_mapdv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_mapdv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_mapdv_reply_t;

  pub fn xcb_glx_get_mapfv_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapfv_cookie_t;

  pub fn xcb_glx_get_mapfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapfv_cookie_t;

  pub fn xcb_glx_get_mapfv_data(R: *const xcb_glx_get_mapfv_reply_t) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_mapfv_data_length(
    R: *const xcb_glx_get_mapfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapfv_data_end(R: *const xcb_glx_get_mapfv_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_mapfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_mapfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_mapfv_reply_t;

  pub fn xcb_glx_get_mapiv_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapiv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapiv_cookie_t;

  pub fn xcb_glx_get_mapiv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    query: u32,
  ) -> xcb_glx_get_mapiv_cookie_t;

  pub fn xcb_glx_get_mapiv_data(R: *const xcb_glx_get_mapiv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_mapiv_data_length(
    R: *const xcb_glx_get_mapiv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_mapiv_data_end(R: *const xcb_glx_get_mapiv_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_mapiv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_mapiv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_mapiv_reply_t;

  pub fn xcb_glx_get_materialfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_materialfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    face: u32,
    pname: u32,
  ) -> xcb_glx_get_materialfv_cookie_t;

  pub fn xcb_glx_get_materialfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    face: u32,
    pname: u32,
  ) -> xcb_glx_get_materialfv_cookie_t;

  pub fn xcb_glx_get_materialfv_data(
    R: *const xcb_glx_get_materialfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_materialfv_data_length(
    R: *const xcb_glx_get_materialfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_materialfv_data_end(
    R: *const xcb_glx_get_materialfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_materialfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_materialfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_materialfv_reply_t;

  pub fn xcb_glx_get_materialiv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_materialiv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    face: u32,
    pname: u32,
  ) -> xcb_glx_get_materialiv_cookie_t;

  pub fn xcb_glx_get_materialiv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    face: u32,
    pname: u32,
  ) -> xcb_glx_get_materialiv_cookie_t;

  pub fn xcb_glx_get_materialiv_data(R: *const xcb_glx_get_materialiv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_materialiv_data_length(
    R: *const xcb_glx_get_materialiv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_materialiv_data_end(
    R: *const xcb_glx_get_materialiv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_materialiv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_materialiv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_materialiv_reply_t;

  pub fn xcb_glx_get_pixel_mapfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapfv_cookie_t;

  pub fn xcb_glx_get_pixel_mapfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapfv_cookie_t;

  pub fn xcb_glx_get_pixel_mapfv_data(
    R: *const xcb_glx_get_pixel_mapfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_pixel_mapfv_data_length(
    R: *const xcb_glx_get_pixel_mapfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapfv_data_end(
    R: *const xcb_glx_get_pixel_mapfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_pixel_mapfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_pixel_mapfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_pixel_mapfv_reply_t;

  pub fn xcb_glx_get_pixel_mapuiv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapuiv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapuiv_cookie_t;

  pub fn xcb_glx_get_pixel_mapuiv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapuiv_cookie_t;

  pub fn xcb_glx_get_pixel_mapuiv_data(R: *const xcb_glx_get_pixel_mapuiv_reply_t) -> *mut u32;

  pub fn xcb_glx_get_pixel_mapuiv_data_length(
    R: *const xcb_glx_get_pixel_mapuiv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapuiv_data_end(
    R: *const xcb_glx_get_pixel_mapuiv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_pixel_mapuiv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_pixel_mapuiv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_pixel_mapuiv_reply_t;

  pub fn xcb_glx_get_pixel_mapusv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapusv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapusv_cookie_t;

  pub fn xcb_glx_get_pixel_mapusv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    map: u32,
  ) -> xcb_glx_get_pixel_mapusv_cookie_t;

  pub fn xcb_glx_get_pixel_mapusv_data(R: *const xcb_glx_get_pixel_mapusv_reply_t) -> *mut u16;

  pub fn xcb_glx_get_pixel_mapusv_data_length(
    R: *const xcb_glx_get_pixel_mapusv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_pixel_mapusv_data_end(
    R: *const xcb_glx_get_pixel_mapusv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_pixel_mapusv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_pixel_mapusv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_pixel_mapusv_reply_t;

  pub fn xcb_glx_get_polygon_stipple_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_polygon_stipple(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    lsb_first: u8,
  ) -> xcb_glx_get_polygon_stipple_cookie_t;

  pub fn xcb_glx_get_polygon_stipple_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    lsb_first: u8,
  ) -> xcb_glx_get_polygon_stipple_cookie_t;

  pub fn xcb_glx_get_polygon_stipple_data(R: *const xcb_glx_get_polygon_stipple_reply_t)
    -> *mut u8;

  pub fn xcb_glx_get_polygon_stipple_data_length(
    R: *const xcb_glx_get_polygon_stipple_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_polygon_stipple_data_end(
    R: *const xcb_glx_get_polygon_stipple_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_polygon_stipple_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_polygon_stipple_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_polygon_stipple_reply_t;

  pub fn xcb_glx_get_string_sizeof(_buffer: *const ::std::os::raw::c_void)
    -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_string(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    name: u32,
  ) -> xcb_glx_get_string_cookie_t;

  pub fn xcb_glx_get_string_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    name: u32,
  ) -> xcb_glx_get_string_cookie_t;

  pub fn xcb_glx_get_string_string(
    R: *const xcb_glx_get_string_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_glx_get_string_string_length(
    R: *const xcb_glx_get_string_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_string_string_end(
    R: *const xcb_glx_get_string_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_string_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_string_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_string_reply_t;

  pub fn xcb_glx_get_tex_envfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_envfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_envfv_cookie_t;

  pub fn xcb_glx_get_tex_envfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_envfv_cookie_t;

  pub fn xcb_glx_get_tex_envfv_data(
    R: *const xcb_glx_get_tex_envfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_tex_envfv_data_length(
    R: *const xcb_glx_get_tex_envfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_envfv_data_end(
    R: *const xcb_glx_get_tex_envfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_envfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_envfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_envfv_reply_t;

  pub fn xcb_glx_get_tex_enviv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_enviv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_enviv_cookie_t;

  pub fn xcb_glx_get_tex_enviv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_enviv_cookie_t;

  pub fn xcb_glx_get_tex_enviv_data(R: *const xcb_glx_get_tex_enviv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_tex_enviv_data_length(
    R: *const xcb_glx_get_tex_enviv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_enviv_data_end(
    R: *const xcb_glx_get_tex_enviv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_enviv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_enviv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_enviv_reply_t;

  pub fn xcb_glx_get_tex_gendv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_gendv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_gendv_cookie_t;

  pub fn xcb_glx_get_tex_gendv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_gendv_cookie_t;

  pub fn xcb_glx_get_tex_gendv_data(
    R: *const xcb_glx_get_tex_gendv_reply_t
  ) -> *mut xcb_glx_float64_t;

  pub fn xcb_glx_get_tex_gendv_data_length(
    R: *const xcb_glx_get_tex_gendv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_gendv_data_end(
    R: *const xcb_glx_get_tex_gendv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_gendv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_gendv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_gendv_reply_t;

  pub fn xcb_glx_get_tex_genfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_genfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_genfv_cookie_t;

  pub fn xcb_glx_get_tex_genfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_genfv_cookie_t;

  pub fn xcb_glx_get_tex_genfv_data(
    R: *const xcb_glx_get_tex_genfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_tex_genfv_data_length(
    R: *const xcb_glx_get_tex_genfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_genfv_data_end(
    R: *const xcb_glx_get_tex_genfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_genfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_genfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_genfv_reply_t;

  pub fn xcb_glx_get_tex_geniv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_geniv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_geniv_cookie_t;

  pub fn xcb_glx_get_tex_geniv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    coord: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_geniv_cookie_t;

  pub fn xcb_glx_get_tex_geniv_data(R: *const xcb_glx_get_tex_geniv_reply_t) -> *mut i32;

  pub fn xcb_glx_get_tex_geniv_data_length(
    R: *const xcb_glx_get_tex_geniv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_geniv_data_end(
    R: *const xcb_glx_get_tex_geniv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_geniv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_geniv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_geniv_reply_t;

  pub fn xcb_glx_get_tex_image_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_image(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_tex_image_cookie_t;

  pub fn xcb_glx_get_tex_image_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_tex_image_cookie_t;

  pub fn xcb_glx_get_tex_image_data(R: *const xcb_glx_get_tex_image_reply_t) -> *mut u8;

  pub fn xcb_glx_get_tex_image_data_length(
    R: *const xcb_glx_get_tex_image_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_image_data_end(
    R: *const xcb_glx_get_tex_image_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_image_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_image_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_image_reply_t;

  pub fn xcb_glx_get_tex_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_parameterfv_cookie_t;

  pub fn xcb_glx_get_tex_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_parameterfv_cookie_t;

  pub fn xcb_glx_get_tex_parameterfv_data(
    R: *const xcb_glx_get_tex_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_tex_parameterfv_data_length(
    R: *const xcb_glx_get_tex_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_parameterfv_data_end(
    R: *const xcb_glx_get_tex_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_parameterfv_reply_t;

  pub fn xcb_glx_get_tex_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_parameteriv_cookie_t;

  pub fn xcb_glx_get_tex_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_tex_parameteriv_cookie_t;

  pub fn xcb_glx_get_tex_parameteriv_data(
    R: *const xcb_glx_get_tex_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_tex_parameteriv_data_length(
    R: *const xcb_glx_get_tex_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_parameteriv_data_end(
    R: *const xcb_glx_get_tex_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_parameteriv_reply_t;

  pub fn xcb_glx_get_tex_level_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_level_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    pname: u32,
  ) -> xcb_glx_get_tex_level_parameterfv_cookie_t;

  pub fn xcb_glx_get_tex_level_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    pname: u32,
  ) -> xcb_glx_get_tex_level_parameterfv_cookie_t;

  pub fn xcb_glx_get_tex_level_parameterfv_data(
    R: *const xcb_glx_get_tex_level_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_tex_level_parameterfv_data_length(
    R: *const xcb_glx_get_tex_level_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_level_parameterfv_data_end(
    R: *const xcb_glx_get_tex_level_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_level_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_level_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_level_parameterfv_reply_t;

  pub fn xcb_glx_get_tex_level_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_level_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    pname: u32,
  ) -> xcb_glx_get_tex_level_parameteriv_cookie_t;

  pub fn xcb_glx_get_tex_level_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
    pname: u32,
  ) -> xcb_glx_get_tex_level_parameteriv_cookie_t;

  pub fn xcb_glx_get_tex_level_parameteriv_data(
    R: *const xcb_glx_get_tex_level_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_tex_level_parameteriv_data_length(
    R: *const xcb_glx_get_tex_level_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_tex_level_parameteriv_data_end(
    R: *const xcb_glx_get_tex_level_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_tex_level_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_tex_level_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_tex_level_parameteriv_reply_t;

  pub fn xcb_glx_is_enabled(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    capability: u32,
  ) -> xcb_glx_is_enabled_cookie_t;

  pub fn xcb_glx_is_enabled_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    capability: u32,
  ) -> xcb_glx_is_enabled_cookie_t;

  pub fn xcb_glx_is_enabled_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_is_enabled_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_is_enabled_reply_t;

  pub fn xcb_glx_is_list(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
  ) -> xcb_glx_is_list_cookie_t;

  pub fn xcb_glx_is_list_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    list: u32,
  ) -> xcb_glx_is_list_cookie_t;

  pub fn xcb_glx_is_list_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_is_list_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_is_list_reply_t;

  pub fn xcb_glx_flush_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_flush(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_are_textures_resident_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_are_textures_resident(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    textures: *const u32,
  ) -> xcb_glx_are_textures_resident_cookie_t;

  pub fn xcb_glx_are_textures_resident_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    textures: *const u32,
  ) -> xcb_glx_are_textures_resident_cookie_t;

  pub fn xcb_glx_are_textures_resident_data(
    R: *const xcb_glx_are_textures_resident_reply_t
  ) -> *mut u8;

  pub fn xcb_glx_are_textures_resident_data_length(
    R: *const xcb_glx_are_textures_resident_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_are_textures_resident_data_end(
    R: *const xcb_glx_are_textures_resident_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_are_textures_resident_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_are_textures_resident_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_are_textures_resident_reply_t;

  pub fn xcb_glx_delete_textures_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_delete_textures_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    textures: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_textures(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    textures: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_textures_textures(R: *const xcb_glx_delete_textures_request_t) -> *mut u32;

  pub fn xcb_glx_delete_textures_textures_length(
    R: *const xcb_glx_delete_textures_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_delete_textures_textures_end(
    R: *const xcb_glx_delete_textures_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_gen_textures_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_gen_textures(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
  ) -> xcb_glx_gen_textures_cookie_t;

  pub fn xcb_glx_gen_textures_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
  ) -> xcb_glx_gen_textures_cookie_t;

  pub fn xcb_glx_gen_textures_data(R: *const xcb_glx_gen_textures_reply_t) -> *mut u32;

  pub fn xcb_glx_gen_textures_data_length(
    R: *const xcb_glx_gen_textures_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_gen_textures_data_end(
    R: *const xcb_glx_gen_textures_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_gen_textures_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_gen_textures_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_gen_textures_reply_t;

  pub fn xcb_glx_is_texture(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    texture: u32,
  ) -> xcb_glx_is_texture_cookie_t;

  pub fn xcb_glx_is_texture_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    texture: u32,
  ) -> xcb_glx_is_texture_cookie_t;

  pub fn xcb_glx_is_texture_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_is_texture_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_is_texture_reply_t;

  pub fn xcb_glx_get_color_table_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_color_table_cookie_t;

  pub fn xcb_glx_get_color_table_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_color_table_cookie_t;

  pub fn xcb_glx_get_color_table_data(R: *const xcb_glx_get_color_table_reply_t) -> *mut u8;

  pub fn xcb_glx_get_color_table_data_length(
    R: *const xcb_glx_get_color_table_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table_data_end(
    R: *const xcb_glx_get_color_table_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_color_table_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_color_table_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_color_table_reply_t;

  pub fn xcb_glx_get_color_table_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_color_table_parameterfv_cookie_t;

  pub fn xcb_glx_get_color_table_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_color_table_parameterfv_cookie_t;

  pub fn xcb_glx_get_color_table_parameterfv_data(
    R: *const xcb_glx_get_color_table_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_color_table_parameterfv_data_length(
    R: *const xcb_glx_get_color_table_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table_parameterfv_data_end(
    R: *const xcb_glx_get_color_table_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_color_table_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_color_table_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_color_table_parameterfv_reply_t;

  pub fn xcb_glx_get_color_table_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_color_table_parameteriv_cookie_t;

  pub fn xcb_glx_get_color_table_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_color_table_parameteriv_cookie_t;

  pub fn xcb_glx_get_color_table_parameteriv_data(
    R: *const xcb_glx_get_color_table_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_color_table_parameteriv_data_length(
    R: *const xcb_glx_get_color_table_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_color_table_parameteriv_data_end(
    R: *const xcb_glx_get_color_table_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_color_table_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_color_table_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_color_table_parameteriv_reply_t;

  pub fn xcb_glx_get_convolution_filter_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_filter(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_convolution_filter_cookie_t;

  pub fn xcb_glx_get_convolution_filter_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_convolution_filter_cookie_t;

  pub fn xcb_glx_get_convolution_filter_data(
    R: *const xcb_glx_get_convolution_filter_reply_t
  ) -> *mut u8;

  pub fn xcb_glx_get_convolution_filter_data_length(
    R: *const xcb_glx_get_convolution_filter_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_filter_data_end(
    R: *const xcb_glx_get_convolution_filter_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_convolution_filter_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_convolution_filter_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_convolution_filter_reply_t;

  pub fn xcb_glx_get_convolution_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_convolution_parameterfv_cookie_t;

  pub fn xcb_glx_get_convolution_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_convolution_parameterfv_cookie_t;

  pub fn xcb_glx_get_convolution_parameterfv_data(
    R: *const xcb_glx_get_convolution_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_convolution_parameterfv_data_length(
    R: *const xcb_glx_get_convolution_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_parameterfv_data_end(
    R: *const xcb_glx_get_convolution_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_convolution_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_convolution_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_convolution_parameterfv_reply_t;

  pub fn xcb_glx_get_convolution_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_convolution_parameteriv_cookie_t;

  pub fn xcb_glx_get_convolution_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_convolution_parameteriv_cookie_t;

  pub fn xcb_glx_get_convolution_parameteriv_data(
    R: *const xcb_glx_get_convolution_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_convolution_parameteriv_data_length(
    R: *const xcb_glx_get_convolution_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_convolution_parameteriv_data_end(
    R: *const xcb_glx_get_convolution_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_convolution_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_convolution_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_convolution_parameteriv_reply_t;

  pub fn xcb_glx_get_separable_filter_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_separable_filter(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_separable_filter_cookie_t;

  pub fn xcb_glx_get_separable_filter_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
  ) -> xcb_glx_get_separable_filter_cookie_t;

  pub fn xcb_glx_get_separable_filter_rows_and_cols(
    R: *const xcb_glx_get_separable_filter_reply_t
  ) -> *mut u8;

  pub fn xcb_glx_get_separable_filter_rows_and_cols_length(
    R: *const xcb_glx_get_separable_filter_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_separable_filter_rows_and_cols_end(
    R: *const xcb_glx_get_separable_filter_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_separable_filter_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_separable_filter_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_separable_filter_reply_t;

  pub fn xcb_glx_get_histogram_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    reset: u8,
  ) -> xcb_glx_get_histogram_cookie_t;

  pub fn xcb_glx_get_histogram_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    reset: u8,
  ) -> xcb_glx_get_histogram_cookie_t;

  pub fn xcb_glx_get_histogram_data(R: *const xcb_glx_get_histogram_reply_t) -> *mut u8;

  pub fn xcb_glx_get_histogram_data_length(
    R: *const xcb_glx_get_histogram_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram_data_end(
    R: *const xcb_glx_get_histogram_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_histogram_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_histogram_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_histogram_reply_t;

  pub fn xcb_glx_get_histogram_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_histogram_parameterfv_cookie_t;

  pub fn xcb_glx_get_histogram_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_histogram_parameterfv_cookie_t;

  pub fn xcb_glx_get_histogram_parameterfv_data(
    R: *const xcb_glx_get_histogram_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_histogram_parameterfv_data_length(
    R: *const xcb_glx_get_histogram_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram_parameterfv_data_end(
    R: *const xcb_glx_get_histogram_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_histogram_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_histogram_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_histogram_parameterfv_reply_t;

  pub fn xcb_glx_get_histogram_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_histogram_parameteriv_cookie_t;

  pub fn xcb_glx_get_histogram_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_histogram_parameteriv_cookie_t;

  pub fn xcb_glx_get_histogram_parameteriv_data(
    R: *const xcb_glx_get_histogram_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_histogram_parameteriv_data_length(
    R: *const xcb_glx_get_histogram_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_histogram_parameteriv_data_end(
    R: *const xcb_glx_get_histogram_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_histogram_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_histogram_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_histogram_parameteriv_reply_t;

  pub fn xcb_glx_get_minmax_sizeof(_buffer: *const ::std::os::raw::c_void)
    -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    reset: u8,
  ) -> xcb_glx_get_minmax_cookie_t;

  pub fn xcb_glx_get_minmax_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    format: u32,
    type_: u32,
    swap_bytes: u8,
    reset: u8,
  ) -> xcb_glx_get_minmax_cookie_t;

  pub fn xcb_glx_get_minmax_data(R: *const xcb_glx_get_minmax_reply_t) -> *mut u8;

  pub fn xcb_glx_get_minmax_data_length(
    R: *const xcb_glx_get_minmax_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax_data_end(
    R: *const xcb_glx_get_minmax_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_minmax_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_minmax_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_minmax_reply_t;

  pub fn xcb_glx_get_minmax_parameterfv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax_parameterfv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_minmax_parameterfv_cookie_t;

  pub fn xcb_glx_get_minmax_parameterfv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_minmax_parameterfv_cookie_t;

  pub fn xcb_glx_get_minmax_parameterfv_data(
    R: *const xcb_glx_get_minmax_parameterfv_reply_t
  ) -> *mut xcb_glx_float32_t;

  pub fn xcb_glx_get_minmax_parameterfv_data_length(
    R: *const xcb_glx_get_minmax_parameterfv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax_parameterfv_data_end(
    R: *const xcb_glx_get_minmax_parameterfv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_minmax_parameterfv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_minmax_parameterfv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_minmax_parameterfv_reply_t;

  pub fn xcb_glx_get_minmax_parameteriv_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax_parameteriv(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_minmax_parameteriv_cookie_t;

  pub fn xcb_glx_get_minmax_parameteriv_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_minmax_parameteriv_cookie_t;

  pub fn xcb_glx_get_minmax_parameteriv_data(
    R: *const xcb_glx_get_minmax_parameteriv_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_minmax_parameteriv_data_length(
    R: *const xcb_glx_get_minmax_parameteriv_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_minmax_parameteriv_data_end(
    R: *const xcb_glx_get_minmax_parameteriv_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_minmax_parameteriv_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_minmax_parameteriv_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_minmax_parameteriv_reply_t;

  pub fn xcb_glx_get_compressed_tex_image_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_compressed_tex_image_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
  ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t;

  pub fn xcb_glx_get_compressed_tex_image_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    level: i32,
  ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t;

  pub fn xcb_glx_get_compressed_tex_image_arb_data(
    R: *const xcb_glx_get_compressed_tex_image_arb_reply_t
  ) -> *mut u8;

  pub fn xcb_glx_get_compressed_tex_image_arb_data_length(
    R: *const xcb_glx_get_compressed_tex_image_arb_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_compressed_tex_image_arb_data_end(
    R: *const xcb_glx_get_compressed_tex_image_arb_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_compressed_tex_image_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_compressed_tex_image_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_compressed_tex_image_arb_reply_t;

  pub fn xcb_glx_delete_queries_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_delete_queries_arb_checked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    ids: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_queries_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
    ids: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_glx_delete_queries_arb_ids(R: *const xcb_glx_delete_queries_arb_request_t)
    -> *mut u32;

  pub fn xcb_glx_delete_queries_arb_ids_length(
    R: *const xcb_glx_delete_queries_arb_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_delete_queries_arb_ids_end(
    R: *const xcb_glx_delete_queries_arb_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_gen_queries_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_gen_queries_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
  ) -> xcb_glx_gen_queries_arb_cookie_t;

  pub fn xcb_glx_gen_queries_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    n: i32,
  ) -> xcb_glx_gen_queries_arb_cookie_t;

  pub fn xcb_glx_gen_queries_arb_data(R: *const xcb_glx_gen_queries_arb_reply_t) -> *mut u32;

  pub fn xcb_glx_gen_queries_arb_data_length(
    R: *const xcb_glx_gen_queries_arb_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_gen_queries_arb_data_end(
    R: *const xcb_glx_gen_queries_arb_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_gen_queries_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_gen_queries_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_gen_queries_arb_reply_t;

  pub fn xcb_glx_is_query_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
  ) -> xcb_glx_is_query_arb_cookie_t;

  pub fn xcb_glx_is_query_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
  ) -> xcb_glx_is_query_arb_cookie_t;

  pub fn xcb_glx_is_query_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_is_query_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_is_query_arb_reply_t;

  pub fn xcb_glx_get_queryiv_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_queryiv_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_queryiv_arb_cookie_t;

  pub fn xcb_glx_get_queryiv_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    target: u32,
    pname: u32,
  ) -> xcb_glx_get_queryiv_arb_cookie_t;

  pub fn xcb_glx_get_queryiv_arb_data(R: *const xcb_glx_get_queryiv_arb_reply_t) -> *mut i32;

  pub fn xcb_glx_get_queryiv_arb_data_length(
    R: *const xcb_glx_get_queryiv_arb_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_queryiv_arb_data_end(
    R: *const xcb_glx_get_queryiv_arb_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_queryiv_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_queryiv_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_queryiv_arb_reply_t;

  pub fn xcb_glx_get_query_objectiv_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_query_objectiv_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
    pname: u32,
  ) -> xcb_glx_get_query_objectiv_arb_cookie_t;

  pub fn xcb_glx_get_query_objectiv_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
    pname: u32,
  ) -> xcb_glx_get_query_objectiv_arb_cookie_t;

  pub fn xcb_glx_get_query_objectiv_arb_data(
    R: *const xcb_glx_get_query_objectiv_arb_reply_t
  ) -> *mut i32;

  pub fn xcb_glx_get_query_objectiv_arb_data_length(
    R: *const xcb_glx_get_query_objectiv_arb_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_query_objectiv_arb_data_end(
    R: *const xcb_glx_get_query_objectiv_arb_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_query_objectiv_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_query_objectiv_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_query_objectiv_arb_reply_t;

  pub fn xcb_glx_get_query_objectuiv_arb_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_query_objectuiv_arb(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
    pname: u32,
  ) -> xcb_glx_get_query_objectuiv_arb_cookie_t;

  pub fn xcb_glx_get_query_objectuiv_arb_unchecked(
    c: *mut xcb_connection_t,
    context_tag: xcb_glx_context_tag_t,
    id: u32,
    pname: u32,
  ) -> xcb_glx_get_query_objectuiv_arb_cookie_t;

  pub fn xcb_glx_get_query_objectuiv_arb_data(
    R: *const xcb_glx_get_query_objectuiv_arb_reply_t
  ) -> *mut u32;

  pub fn xcb_glx_get_query_objectuiv_arb_data_length(
    R: *const xcb_glx_get_query_objectuiv_arb_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_glx_get_query_objectuiv_arb_data_end(
    R: *const xcb_glx_get_query_objectuiv_arb_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_glx_get_query_objectuiv_arb_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_glx_get_query_objectuiv_arb_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_glx_get_query_objectuiv_arb_reply_t;
}
