use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::shm::xcb_shm_seg_t;
use super::xcb::{
  xcb_atom_t,
  xcb_drawable_t,
  xcb_gcontext_t,
  xcb_timestamp_t,
  xcb_visualid_t,
  xcb_window_t,
};

pub const XCB_XV_MAJOR_VERSION: u32 = 2;
pub const XCB_XV_MINOR_VERSION: u32 = 2;
pub const XCB_XV_BAD_PORT: u32 = 0;
pub const XCB_XV_BAD_ENCODING: u32 = 1;
pub const XCB_XV_BAD_CONTROL: u32 = 2;
pub const XCB_XV_VIDEO_NOTIFY: u32 = 0;
pub const XCB_XV_PORT_NOTIFY: u32 = 1;
pub const XCB_XV_QUERY_EXTENSION: u32 = 0;
pub const XCB_XV_QUERY_ADAPTORS: u32 = 1;
pub const XCB_XV_QUERY_ENCODINGS: u32 = 2;
pub const XCB_XV_GRAB_PORT: u32 = 3;
pub const XCB_XV_UNGRAB_PORT: u32 = 4;
pub const XCB_XV_PUT_VIDEO: u32 = 5;
pub const XCB_XV_PUT_STILL: u32 = 6;
pub const XCB_XV_GET_VIDEO: u32 = 7;
pub const XCB_XV_GET_STILL: u32 = 8;
pub const XCB_XV_STOP_VIDEO: u32 = 9;
pub const XCB_XV_SELECT_VIDEO_NOTIFY: u32 = 10;
pub const XCB_XV_SELECT_PORT_NOTIFY: u32 = 11;
pub const XCB_XV_QUERY_BEST_SIZE: u32 = 12;
pub const XCB_XV_SET_PORT_ATTRIBUTE: u32 = 13;
pub const XCB_XV_GET_PORT_ATTRIBUTE: u32 = 14;
pub const XCB_XV_QUERY_PORT_ATTRIBUTES: u32 = 15;
pub const XCB_XV_LIST_IMAGE_FORMATS: u32 = 16;
pub const XCB_XV_QUERY_IMAGE_ATTRIBUTES: u32 = 17;
pub const XCB_XV_PUT_IMAGE: u32 = 18;
pub const XCB_XV_SHM_PUT_IMAGE: u32 = 19;

pub type xcb_xv_port_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_port_iterator_t
{
  pub data: *mut xcb_xv_port_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_xv_encoding_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_encoding_iterator_t
{
  pub data: *mut xcb_xv_encoding_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_type_t
{
  XCB_XV_TYPE_INPUT_MASK = 1,
  XCB_XV_TYPE_OUTPUT_MASK = 2,
  XCB_XV_TYPE_VIDEO_MASK = 4,
  XCB_XV_TYPE_STILL_MASK = 8,
  XCB_XV_TYPE_IMAGE_MASK = 16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_image_format_info_type_t
{
  XCB_XV_IMAGE_FORMAT_INFO_TYPE_RGB = 0,
  XCB_XV_IMAGE_FORMAT_INFO_TYPE_YUV = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_image_format_info_format_t
{
  XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PACKED = 0,
  XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PLANAR = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_attribute_flag_t
{
  XCB_XV_ATTRIBUTE_FLAG_GETTABLE = 1,
  XCB_XV_ATTRIBUTE_FLAG_SETTABLE = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_video_notify_reason_t
{
  XCB_XV_VIDEO_NOTIFY_REASON_STARTED = 0,
  XCB_XV_VIDEO_NOTIFY_REASON_STOPPED = 1,
  XCB_XV_VIDEO_NOTIFY_REASON_BUSY = 2,
  XCB_XV_VIDEO_NOTIFY_REASON_PREEMPTED = 3,
  XCB_XV_VIDEO_NOTIFY_REASON_HARD_ERROR = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_scanline_order_t
{
  XCB_XV_SCANLINE_ORDER_TOP_TO_BOTTOM = 0,
  XCB_XV_SCANLINE_ORDER_BOTTOM_TO_TOP = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xv_grab_port_status_t
{
  XCB_XV_GRAB_PORT_STATUS_SUCCESS = 0,
  XCB_XV_GRAB_PORT_STATUS_BAD_EXTENSION = 1,
  XCB_XV_GRAB_PORT_STATUS_ALREADY_GRABBED = 2,
  XCB_XV_GRAB_PORT_STATUS_INVALID_TIME = 3,
  XCB_XV_GRAB_PORT_STATUS_BAD_REPLY = 4,
  XCB_XV_GRAB_PORT_STATUS_BAD_ALLOC = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_rational_t
{
  pub numerator: i32,
  pub denominator: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_rational_iterator_t
{
  pub data: *mut xcb_xv_rational_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_format_t
{
  pub visual: xcb_visualid_t,
  pub depth: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_format_iterator_t
{
  pub data: *mut xcb_xv_format_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_adaptor_info_t
{
  pub base_id: xcb_xv_port_t,
  pub name_size: u16,
  pub num_ports: u16,
  pub num_formats: u16,
  pub type_: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_adaptor_info_iterator_t
{
  pub data: *mut xcb_xv_adaptor_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_encoding_info_t
{
  pub encoding: xcb_xv_encoding_t,
  pub name_size: u16,
  pub width: u16,
  pub height: u16,
  pub pad0: [u8; 2usize],
  pub rate: xcb_xv_rational_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_encoding_info_iterator_t
{
  pub data: *mut xcb_xv_encoding_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_image_t
{
  pub id: u32,
  pub width: u16,
  pub height: u16,
  pub data_size: u32,
  pub num_planes: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_image_iterator_t
{
  pub data: *mut xcb_xv_image_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_attribute_info_t
{
  pub flags: u32,
  pub min: i32,
  pub max: i32,
  pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_attribute_info_iterator_t
{
  pub data: *mut xcb_xv_attribute_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_image_format_info_t
{
  pub id: u32,
  pub type_: u8,
  pub byte_order: u8,
  pub pad0: [u8; 2usize],
  pub guid: [u8; 16usize],
  pub bpp: u8,
  pub num_planes: u8,
  pub pad1: [u8; 2usize],
  pub depth: u8,
  pub pad2: [u8; 3usize],
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub format: u8,
  pub pad3: [u8; 3usize],
  pub y_sample_bits: u32,
  pub u_sample_bits: u32,
  pub v_sample_bits: u32,
  pub vhorz_y_period: u32,
  pub vhorz_u_period: u32,
  pub vhorz_v_period: u32,
  pub vvert_y_period: u32,
  pub vvert_u_period: u32,
  pub vvert_v_period: u32,
  pub vcomp_order: [u8; 32usize],
  pub vscanline_order: u8,
  pub pad4: [u8; 11usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_image_format_info_iterator_t
{
  pub data: *mut xcb_xv_image_format_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_bad_port_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_bad_encoding_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_bad_control_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_video_notify_event_t
{
  pub response_type: u8,
  pub reason: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub drawable: xcb_drawable_t,
  pub port: xcb_xv_port_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_port_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub port: xcb_xv_port_t,
  pub attribute: xcb_atom_t,
  pub value: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_extension_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_extension_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_extension_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major: u16,
  pub minor: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_adaptors_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_adaptors_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_adaptors_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_adaptors: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_encodings_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_encodings_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_encodings_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_encodings: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_grab_port_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_grab_port_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_grab_port_reply_t
{
  pub response_type: u8,
  pub result: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_ungrab_port_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_put_video_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub vid_x: i16,
  pub vid_y: i16,
  pub vid_w: u16,
  pub vid_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_put_still_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub vid_x: i16,
  pub vid_y: i16,
  pub vid_w: u16,
  pub vid_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_get_video_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub vid_x: i16,
  pub vid_y: i16,
  pub vid_w: u16,
  pub vid_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_get_still_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub vid_x: i16,
  pub vid_y: i16,
  pub vid_w: u16,
  pub vid_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_stop_video_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_select_video_notify_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub onoff: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_select_port_notify_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub onoff: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_best_size_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_best_size_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub vid_w: u16,
  pub vid_h: u16,
  pub drw_w: u16,
  pub drw_h: u16,
  pub motion: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_best_size_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub actual_width: u16,
  pub actual_height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_set_port_attribute_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub attribute: xcb_atom_t,
  pub value: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_get_port_attribute_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_get_port_attribute_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub attribute: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_get_port_attribute_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub value: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_port_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_port_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_port_attributes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_attributes: u32,
  pub text_size: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_list_image_formats_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_list_image_formats_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_list_image_formats_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_formats: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_image_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_image_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub id: u32,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_query_image_attributes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_planes: u32,
  pub data_size: u32,
  pub width: u16,
  pub height: u16,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_put_image_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub id: u32,
  pub src_x: i16,
  pub src_y: i16,
  pub src_w: u16,
  pub src_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xv_shm_put_image_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub port: xcb_xv_port_t,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub shmseg: xcb_shm_seg_t,
  pub id: u32,
  pub offset: u32,
  pub src_x: i16,
  pub src_y: i16,
  pub src_w: u16,
  pub src_h: u16,
  pub drw_x: i16,
  pub drw_y: i16,
  pub drw_w: u16,
  pub drw_h: u16,
  pub width: u16,
  pub height: u16,
  pub send_event: u8,
  pub pad0: [u8; 3usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_xv_id: xcb_extension_t;

  pub fn xcb_xv_port_next(i: *mut xcb_xv_port_iterator_t);

  pub fn xcb_xv_port_end(i: xcb_xv_port_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_encoding_next(i: *mut xcb_xv_encoding_iterator_t);

  pub fn xcb_xv_encoding_end(i: xcb_xv_encoding_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_rational_next(i: *mut xcb_xv_rational_iterator_t);

  pub fn xcb_xv_rational_end(i: xcb_xv_rational_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_format_next(i: *mut xcb_xv_format_iterator_t);

  pub fn xcb_xv_format_end(i: xcb_xv_format_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_adaptor_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_adaptor_info_name(R: *const xcb_xv_adaptor_info_t) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xv_adaptor_info_name_length(R: *const xcb_xv_adaptor_info_t) -> ::std::os::raw::c_int;

  pub fn xcb_xv_adaptor_info_name_end(R: *const xcb_xv_adaptor_info_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_adaptor_info_formats(R: *const xcb_xv_adaptor_info_t) -> *mut xcb_xv_format_t;

  pub fn xcb_xv_adaptor_info_formats_length(
    R: *const xcb_xv_adaptor_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_adaptor_info_formats_iterator(
    R: *const xcb_xv_adaptor_info_t
  ) -> xcb_xv_format_iterator_t;

  pub fn xcb_xv_adaptor_info_next(i: *mut xcb_xv_adaptor_info_iterator_t);

  pub fn xcb_xv_adaptor_info_end(i: xcb_xv_adaptor_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_encoding_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_encoding_info_name(R: *const xcb_xv_encoding_info_t)
    -> *mut ::std::os::raw::c_char;

  pub fn xcb_xv_encoding_info_name_length(
    R: *const xcb_xv_encoding_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_encoding_info_name_end(R: *const xcb_xv_encoding_info_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_encoding_info_next(i: *mut xcb_xv_encoding_info_iterator_t);

  pub fn xcb_xv_encoding_info_end(i: xcb_xv_encoding_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_image_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xv_image_pitches(R: *const xcb_xv_image_t) -> *mut u32;

  pub fn xcb_xv_image_pitches_length(R: *const xcb_xv_image_t) -> ::std::os::raw::c_int;

  pub fn xcb_xv_image_pitches_end(R: *const xcb_xv_image_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_image_offsets(R: *const xcb_xv_image_t) -> *mut u32;

  pub fn xcb_xv_image_offsets_length(R: *const xcb_xv_image_t) -> ::std::os::raw::c_int;

  pub fn xcb_xv_image_offsets_end(R: *const xcb_xv_image_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_image_data(R: *const xcb_xv_image_t) -> *mut u8;

  pub fn xcb_xv_image_data_length(R: *const xcb_xv_image_t) -> ::std::os::raw::c_int;

  pub fn xcb_xv_image_data_end(R: *const xcb_xv_image_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_image_next(i: *mut xcb_xv_image_iterator_t);

  pub fn xcb_xv_image_end(i: xcb_xv_image_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_attribute_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_attribute_info_name(
    R: *const xcb_xv_attribute_info_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xv_attribute_info_name_length(
    R: *const xcb_xv_attribute_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_attribute_info_name_end(
    R: *const xcb_xv_attribute_info_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xv_attribute_info_next(i: *mut xcb_xv_attribute_info_iterator_t);

  pub fn xcb_xv_attribute_info_end(i: xcb_xv_attribute_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_image_format_info_next(i: *mut xcb_xv_image_format_info_iterator_t);

  pub fn xcb_xv_image_format_info_end(
    i: xcb_xv_image_format_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xv_query_extension(c: *mut xcb_connection_t) -> xcb_xv_query_extension_cookie_t;

  pub fn xcb_xv_query_extension_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_xv_query_extension_cookie_t;

  pub fn xcb_xv_query_extension_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_extension_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_extension_reply_t;

  pub fn xcb_xv_query_adaptors_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_adaptors(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xv_query_adaptors_cookie_t;

  pub fn xcb_xv_query_adaptors_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_xv_query_adaptors_cookie_t;

  pub fn xcb_xv_query_adaptors_info_length(
    R: *const xcb_xv_query_adaptors_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_adaptors_info_iterator(
    R: *const xcb_xv_query_adaptors_reply_t
  ) -> xcb_xv_adaptor_info_iterator_t;

  pub fn xcb_xv_query_adaptors_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_adaptors_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_adaptors_reply_t;

  pub fn xcb_xv_query_encodings_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_encodings(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_query_encodings_cookie_t;

  pub fn xcb_xv_query_encodings_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_query_encodings_cookie_t;

  pub fn xcb_xv_query_encodings_info_length(
    R: *const xcb_xv_query_encodings_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_encodings_info_iterator(
    R: *const xcb_xv_query_encodings_reply_t
  ) -> xcb_xv_encoding_info_iterator_t;

  pub fn xcb_xv_query_encodings_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_encodings_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_encodings_reply_t;

  pub fn xcb_xv_grab_port(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    time: xcb_timestamp_t,
  ) -> xcb_xv_grab_port_cookie_t;

  pub fn xcb_xv_grab_port_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    time: xcb_timestamp_t,
  ) -> xcb_xv_grab_port_cookie_t;

  pub fn xcb_xv_grab_port_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_grab_port_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_grab_port_reply_t;

  pub fn xcb_xv_ungrab_port_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_ungrab_port(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_video_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_video(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_still_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_still(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_get_video_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_get_video(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_get_still_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_get_still(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    vid_x: i16,
    vid_y: i16,
    vid_w: u16,
    vid_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_stop_video_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_stop_video(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_select_video_notify_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    onoff: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_select_video_notify(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    onoff: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_select_port_notify_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    onoff: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_select_port_notify(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    onoff: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_query_best_size(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    vid_w: u16,
    vid_h: u16,
    drw_w: u16,
    drw_h: u16,
    motion: u8,
  ) -> xcb_xv_query_best_size_cookie_t;

  pub fn xcb_xv_query_best_size_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    vid_w: u16,
    vid_h: u16,
    drw_w: u16,
    drw_h: u16,
    motion: u8,
  ) -> xcb_xv_query_best_size_cookie_t;

  pub fn xcb_xv_query_best_size_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_best_size_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_best_size_reply_t;

  pub fn xcb_xv_set_port_attribute_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    attribute: xcb_atom_t,
    value: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_set_port_attribute(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    attribute: xcb_atom_t,
    value: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_get_port_attribute(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    attribute: xcb_atom_t,
  ) -> xcb_xv_get_port_attribute_cookie_t;

  pub fn xcb_xv_get_port_attribute_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    attribute: xcb_atom_t,
  ) -> xcb_xv_get_port_attribute_cookie_t;

  pub fn xcb_xv_get_port_attribute_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_get_port_attribute_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_get_port_attribute_reply_t;

  pub fn xcb_xv_query_port_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_port_attributes(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_query_port_attributes_cookie_t;

  pub fn xcb_xv_query_port_attributes_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_query_port_attributes_cookie_t;

  pub fn xcb_xv_query_port_attributes_attributes_length(
    R: *const xcb_xv_query_port_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_port_attributes_attributes_iterator(
    R: *const xcb_xv_query_port_attributes_reply_t
  ) -> xcb_xv_attribute_info_iterator_t;

  pub fn xcb_xv_query_port_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_port_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_port_attributes_reply_t;

  pub fn xcb_xv_list_image_formats_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_list_image_formats(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_list_image_formats_cookie_t;

  pub fn xcb_xv_list_image_formats_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
  ) -> xcb_xv_list_image_formats_cookie_t;

  pub fn xcb_xv_list_image_formats_format(
    R: *const xcb_xv_list_image_formats_reply_t
  ) -> *mut xcb_xv_image_format_info_t;

  pub fn xcb_xv_list_image_formats_format_length(
    R: *const xcb_xv_list_image_formats_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_list_image_formats_format_iterator(
    R: *const xcb_xv_list_image_formats_reply_t
  ) -> xcb_xv_image_format_info_iterator_t;

  pub fn xcb_xv_list_image_formats_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_list_image_formats_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_list_image_formats_reply_t;

  pub fn xcb_xv_query_image_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_image_attributes(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    id: u32,
    width: u16,
    height: u16,
  ) -> xcb_xv_query_image_attributes_cookie_t;

  pub fn xcb_xv_query_image_attributes_unchecked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    id: u32,
    width: u16,
    height: u16,
  ) -> xcb_xv_query_image_attributes_cookie_t;

  pub fn xcb_xv_query_image_attributes_pitches(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> *mut u32;

  pub fn xcb_xv_query_image_attributes_pitches_length(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_image_attributes_pitches_end(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xv_query_image_attributes_offsets(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> *mut u32;

  pub fn xcb_xv_query_image_attributes_offsets_length(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_query_image_attributes_offsets_end(
    R: *const xcb_xv_query_image_attributes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xv_query_image_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xv_query_image_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xv_query_image_attributes_reply_t;

  pub fn xcb_xv_put_image_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_put_image_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    id: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_image(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    id: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_put_image_data(R: *const xcb_xv_put_image_request_t) -> *mut u8;

  pub fn xcb_xv_put_image_data_length(
    R: *const xcb_xv_put_image_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xv_put_image_data_end(R: *const xcb_xv_put_image_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_xv_shm_put_image_checked(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    shmseg: xcb_shm_seg_t,
    id: u32,
    offset: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    send_event: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xv_shm_put_image(
    c: *mut xcb_connection_t,
    port: xcb_xv_port_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    shmseg: xcb_shm_seg_t,
    id: u32,
    offset: u32,
    src_x: i16,
    src_y: i16,
    src_w: u16,
    src_h: u16,
    drw_x: i16,
    drw_y: i16,
    drw_w: u16,
    drw_h: u16,
    width: u16,
    height: u16,
    send_event: u8,
  ) -> xcb_void_cookie_t;
}
