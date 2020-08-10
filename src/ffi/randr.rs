use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::render::{xcb_render_fixed_t, xcb_render_transform_t};
use super::xcb::{xcb_atom_t, xcb_timestamp_t, xcb_window_t};

pub const XCB_RANDR_MAJOR_VERSION: u32 = 1;
pub const XCB_RANDR_MINOR_VERSION: u32 = 6;
pub const XCB_RANDR_BAD_OUTPUT: u32 = 0;
pub const XCB_RANDR_BAD_CRTC: u32 = 1;
pub const XCB_RANDR_BAD_MODE: u32 = 2;
pub const XCB_RANDR_BAD_PROVIDER: u32 = 3;
pub const XCB_RANDR_QUERY_VERSION: u32 = 0;
pub const XCB_RANDR_SET_SCREEN_CONFIG: u32 = 2;
pub const XCB_RANDR_SELECT_INPUT: u32 = 4;
pub const XCB_RANDR_GET_SCREEN_INFO: u32 = 5;
pub const XCB_RANDR_GET_SCREEN_SIZE_RANGE: u32 = 6;
pub const XCB_RANDR_SET_SCREEN_SIZE: u32 = 7;
pub const XCB_RANDR_GET_SCREEN_RESOURCES: u32 = 8;
pub const XCB_RANDR_GET_OUTPUT_INFO: u32 = 9;
pub const XCB_RANDR_LIST_OUTPUT_PROPERTIES: u32 = 10;
pub const XCB_RANDR_QUERY_OUTPUT_PROPERTY: u32 = 11;
pub const XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY: u32 = 12;
pub const XCB_RANDR_CHANGE_OUTPUT_PROPERTY: u32 = 13;
pub const XCB_RANDR_DELETE_OUTPUT_PROPERTY: u32 = 14;
pub const XCB_RANDR_GET_OUTPUT_PROPERTY: u32 = 15;
pub const XCB_RANDR_CREATE_MODE: u32 = 16;
pub const XCB_RANDR_DESTROY_MODE: u32 = 17;
pub const XCB_RANDR_ADD_OUTPUT_MODE: u32 = 18;
pub const XCB_RANDR_DELETE_OUTPUT_MODE: u32 = 19;
pub const XCB_RANDR_GET_CRTC_INFO: u32 = 20;
pub const XCB_RANDR_SET_CRTC_CONFIG: u32 = 21;
pub const XCB_RANDR_GET_CRTC_GAMMA_SIZE: u32 = 22;
pub const XCB_RANDR_GET_CRTC_GAMMA: u32 = 23;
pub const XCB_RANDR_SET_CRTC_GAMMA: u32 = 24;
pub const XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT: u32 = 25;
pub const XCB_RANDR_SET_CRTC_TRANSFORM: u32 = 26;
pub const XCB_RANDR_GET_CRTC_TRANSFORM: u32 = 27;
pub const XCB_RANDR_GET_PANNING: u32 = 28;
pub const XCB_RANDR_SET_PANNING: u32 = 29;
pub const XCB_RANDR_SET_OUTPUT_PRIMARY: u32 = 30;
pub const XCB_RANDR_GET_OUTPUT_PRIMARY: u32 = 31;
pub const XCB_RANDR_GET_PROVIDERS: u32 = 32;
pub const XCB_RANDR_GET_PROVIDER_INFO: u32 = 33;
pub const XCB_RANDR_SET_PROVIDER_OFFLOAD_SINK: u32 = 34;
pub const XCB_RANDR_SET_PROVIDER_OUTPUT_SOURCE: u32 = 35;
pub const XCB_RANDR_LIST_PROVIDER_PROPERTIES: u32 = 36;
pub const XCB_RANDR_QUERY_PROVIDER_PROPERTY: u32 = 37;
pub const XCB_RANDR_CONFIGURE_PROVIDER_PROPERTY: u32 = 38;
pub const XCB_RANDR_CHANGE_PROVIDER_PROPERTY: u32 = 39;
pub const XCB_RANDR_DELETE_PROVIDER_PROPERTY: u32 = 40;
pub const XCB_RANDR_GET_PROVIDER_PROPERTY: u32 = 41;
pub const XCB_RANDR_SCREEN_CHANGE_NOTIFY: u32 = 0;
pub const XCB_RANDR_GET_MONITORS: u32 = 42;
pub const XCB_RANDR_SET_MONITOR: u32 = 43;
pub const XCB_RANDR_DELETE_MONITOR: u32 = 44;
pub const XCB_RANDR_CREATE_LEASE: u32 = 45;
pub const XCB_RANDR_FREE_LEASE: u32 = 46;
pub const XCB_RANDR_NOTIFY: u32 = 1;

pub type xcb_randr_mode_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_mode_iterator_t
{
  pub data: *mut xcb_randr_mode_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_randr_crtc_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_crtc_iterator_t
{
  pub data: *mut xcb_randr_crtc_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_randr_output_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_output_iterator_t
{
  pub data: *mut xcb_randr_output_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_randr_provider_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_provider_iterator_t
{
  pub data: *mut xcb_randr_provider_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_randr_lease_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_lease_iterator_t
{
  pub data: *mut xcb_randr_lease_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_bad_output_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_bad_crtc_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_bad_mode_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_bad_provider_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_rotation_t
{
  XCB_RANDR_ROTATION_ROTATE_0 = 1,
  XCB_RANDR_ROTATION_ROTATE_90 = 2,
  XCB_RANDR_ROTATION_ROTATE_180 = 4,
  XCB_RANDR_ROTATION_ROTATE_270 = 8,
  XCB_RANDR_ROTATION_REFLECT_X = 16,
  XCB_RANDR_ROTATION_REFLECT_Y = 32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_screen_size_t
{
  pub width: u16,
  pub height: u16,
  pub mwidth: u16,
  pub mheight: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_screen_size_iterator_t
{
  pub data: *mut xcb_randr_screen_size_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_refresh_rates_t
{
  pub nRates: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_refresh_rates_iterator_t
{
  pub data: *mut xcb_randr_refresh_rates_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u32,
  pub minor_version: u32,
  pub pad1: [u8; 16usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_set_config_t
{
  XCB_RANDR_SET_CONFIG_SUCCESS = 0,
  XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME = 1,
  XCB_RANDR_SET_CONFIG_INVALID_TIME = 2,
  XCB_RANDR_SET_CONFIG_FAILED = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_screen_config_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_screen_config_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub sizeID: u16,
  pub rotation: u16,
  pub rate: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_screen_config_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub new_timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub root: xcb_window_t,
  pub subpixel_order: u16,
  pub pad0: [u8; 10usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_notify_mask_t
{
  XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE = 1,
  XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE = 2,
  XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE = 4,
  XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY = 8,
  XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE = 16,
  XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY = 32,
  XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE = 64,
  XCB_RANDR_NOTIFY_MASK_LEASE = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub enable: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_info_reply_t
{
  pub response_type: u8,
  pub rotations: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub nSizes: u16,
  pub sizeID: u16,
  pub rotation: u16,
  pub rate: u16,
  pub nInfo: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_size_range_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_size_range_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_size_range_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub min_width: u16,
  pub min_height: u16,
  pub max_width: u16,
  pub max_height: u16,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_screen_size_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub width: u16,
  pub height: u16,
  pub mm_width: u32,
  pub mm_height: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_mode_flag_t
{
  XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE = 1,
  XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE = 2,
  XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE = 4,
  XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE = 8,
  XCB_RANDR_MODE_FLAG_INTERLACE = 16,
  XCB_RANDR_MODE_FLAG_DOUBLE_SCAN = 32,
  XCB_RANDR_MODE_FLAG_CSYNC = 64,
  XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE = 128,
  XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE = 256,
  XCB_RANDR_MODE_FLAG_HSKEW_PRESENT = 512,
  XCB_RANDR_MODE_FLAG_BCAST = 1024,
  XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX = 2048,
  XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK = 4096,
  XCB_RANDR_MODE_FLAG_HALVE_CLOCK = 8192,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_mode_info_t
{
  pub id: u32,
  pub width: u16,
  pub height: u16,
  pub dot_clock: u32,
  pub hsync_start: u16,
  pub hsync_end: u16,
  pub htotal: u16,
  pub hskew: u16,
  pub vsync_start: u16,
  pub vsync_end: u16,
  pub vtotal: u16,
  pub name_len: u16,
  pub mode_flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_mode_info_iterator_t
{
  pub data: *mut xcb_randr_mode_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub num_crtcs: u16,
  pub num_outputs: u16,
  pub num_modes: u16,
  pub names_len: u16,
  pub pad1: [u8; 8usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_connection_t
{
  XCB_RANDR_CONNECTION_CONNECTED = 0,
  XCB_RANDR_CONNECTION_DISCONNECTED = 1,
  XCB_RANDR_CONNECTION_UNKNOWN = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub config_timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_info_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub crtc: xcb_randr_crtc_t,
  pub mm_width: u32,
  pub mm_height: u32,
  pub connection: u8,
  pub subpixel_order: u8,
  pub num_crtcs: u16,
  pub num_modes: u16,
  pub num_preferred: u16,
  pub num_clones: u16,
  pub name_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_output_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_output_properties_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_output_properties_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_atoms: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_output_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_output_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_output_property_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pending: u8,
  pub range: u8,
  pub immutable: u8,
  pub pad1: [u8; 21usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_configure_output_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub property: xcb_atom_t,
  pub pending: u8,
  pub range: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_change_output_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub format: u8,
  pub mode: u8,
  pub pad0: [u8; 2usize],
  pub num_units: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_delete_output_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub long_offset: u32,
  pub long_length: u32,
  pub _delete: u8,
  pub pending: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_property_reply_t
{
  pub response_type: u8,
  pub format: u8,
  pub sequence: u16,
  pub length: u32,
  pub type_: xcb_atom_t,
  pub bytes_after: u32,
  pub num_items: u32,
  pub pad0: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_mode_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub mode_info: xcb_randr_mode_info_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_mode_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub mode: xcb_randr_mode_t,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_destroy_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub mode: xcb_randr_mode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_add_output_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub mode: xcb_randr_mode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_delete_output_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output: xcb_randr_output_t,
  pub mode: xcb_randr_mode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
  pub config_timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_info_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub mode: xcb_randr_mode_t,
  pub rotation: u16,
  pub rotations: u16,
  pub num_outputs: u16,
  pub num_possible_outputs: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_crtc_config_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_crtc_config_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub x: i16,
  pub y: i16,
  pub mode: xcb_randr_mode_t,
  pub rotation: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_crtc_config_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub pad0: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_size_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_size_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_size_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub size: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_gamma_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub size: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_crtc_gamma_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
  pub size: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_current_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_current_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_screen_resources_current_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub num_crtcs: u16,
  pub num_outputs: u16,
  pub num_modes: u16,
  pub names_len: u16,
  pub pad1: [u8; 8usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_transform_t
{
  XCB_RANDR_TRANSFORM_UNIT = 1,
  XCB_RANDR_TRANSFORM_SCALE_UP = 2,
  XCB_RANDR_TRANSFORM_SCALE_DOWN = 4,
  XCB_RANDR_TRANSFORM_PROJECTIVE = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_crtc_transform_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
  pub transform: xcb_render_transform_t,
  pub filter_len: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_transform_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_transform_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_crtc_transform_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pending_transform: xcb_render_transform_t,
  pub has_transforms: u8,
  pub pad1: [u8; 3usize],
  pub current_transform: xcb_render_transform_t,
  pub pad2: [u8; 4usize],
  pub pending_len: u16,
  pub pending_nparams: u16,
  pub current_len: u16,
  pub current_nparams: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_panning_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_panning_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_panning_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub left: u16,
  pub top: u16,
  pub width: u16,
  pub height: u16,
  pub track_left: u16,
  pub track_top: u16,
  pub track_width: u16,
  pub track_height: u16,
  pub border_left: i16,
  pub border_top: i16,
  pub border_right: i16,
  pub border_bottom: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_panning_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_panning_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub crtc: xcb_randr_crtc_t,
  pub timestamp: xcb_timestamp_t,
  pub left: u16,
  pub top: u16,
  pub width: u16,
  pub height: u16,
  pub track_left: u16,
  pub track_top: u16,
  pub track_width: u16,
  pub track_height: u16,
  pub border_left: i16,
  pub border_top: i16,
  pub border_right: i16,
  pub border_bottom: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_panning_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_output_primary_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub output: xcb_randr_output_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_primary_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_primary_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_output_primary_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub output: xcb_randr_output_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_providers_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_providers_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_providers_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub num_providers: u16,
  pub pad1: [u8; 18usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_provider_capability_t
{
  XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OUTPUT = 1,
  XCB_RANDR_PROVIDER_CAPABILITY_SINK_OUTPUT = 2,
  XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OFFLOAD = 4,
  XCB_RANDR_PROVIDER_CAPABILITY_SINK_OFFLOAD = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub config_timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_info_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub capabilities: u32,
  pub num_crtcs: u16,
  pub num_outputs: u16,
  pub num_associated_providers: u16,
  pub name_len: u16,
  pub pad0: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_provider_offload_sink_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub sink_provider: xcb_randr_provider_t,
  pub config_timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_provider_output_source_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub source_provider: xcb_randr_provider_t,
  pub config_timestamp: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_provider_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_provider_properties_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_list_provider_properties_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_atoms: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_provider_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_provider_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_query_provider_property_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pending: u8,
  pub range: u8,
  pub immutable: u8,
  pub pad1: [u8; 21usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_configure_provider_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub property: xcb_atom_t,
  pub pending: u8,
  pub range: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_change_provider_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub format: u8,
  pub mode: u8,
  pub pad0: [u8; 2usize],
  pub num_items: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_delete_provider_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub provider: xcb_randr_provider_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub long_offset: u32,
  pub long_length: u32,
  pub _delete: u8,
  pub pending: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_provider_property_reply_t
{
  pub response_type: u8,
  pub format: u8,
  pub sequence: u16,
  pub length: u32,
  pub type_: xcb_atom_t,
  pub bytes_after: u32,
  pub num_items: u32,
  pub pad0: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_screen_change_notify_event_t
{
  pub response_type: u8,
  pub rotation: u8,
  pub sequence: u16,
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub root: xcb_window_t,
  pub request_window: xcb_window_t,
  pub sizeID: u16,
  pub subpixel_order: u16,
  pub width: u16,
  pub height: u16,
  pub mwidth: u16,
  pub mheight: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_randr_notify_t
{
  XCB_RANDR_NOTIFY_CRTC_CHANGE = 0,
  XCB_RANDR_NOTIFY_OUTPUT_CHANGE = 1,
  XCB_RANDR_NOTIFY_OUTPUT_PROPERTY = 2,
  XCB_RANDR_NOTIFY_PROVIDER_CHANGE = 3,
  XCB_RANDR_NOTIFY_PROVIDER_PROPERTY = 4,
  XCB_RANDR_NOTIFY_RESOURCE_CHANGE = 5,
  XCB_RANDR_NOTIFY_LEASE = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_crtc_change_t
{
  pub timestamp: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub crtc: xcb_randr_crtc_t,
  pub mode: xcb_randr_mode_t,
  pub rotation: u16,
  pub pad0: [u8; 2usize],
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_crtc_change_iterator_t
{
  pub data: *mut xcb_randr_crtc_change_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_output_change_t
{
  pub timestamp: xcb_timestamp_t,
  pub config_timestamp: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub output: xcb_randr_output_t,
  pub crtc: xcb_randr_crtc_t,
  pub mode: xcb_randr_mode_t,
  pub rotation: u16,
  pub connection: u8,
  pub subpixel_order: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_output_change_iterator_t
{
  pub data: *mut xcb_randr_output_change_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_output_property_t
{
  pub window: xcb_window_t,
  pub output: xcb_randr_output_t,
  pub atom: xcb_atom_t,
  pub timestamp: xcb_timestamp_t,
  pub status: u8,
  pub pad0: [u8; 11usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_output_property_iterator_t
{
  pub data: *mut xcb_randr_output_property_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_provider_change_t
{
  pub timestamp: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub provider: xcb_randr_provider_t,
  pub pad0: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_provider_change_iterator_t
{
  pub data: *mut xcb_randr_provider_change_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_provider_property_t
{
  pub window: xcb_window_t,
  pub provider: xcb_randr_provider_t,
  pub atom: xcb_atom_t,
  pub timestamp: xcb_timestamp_t,
  pub state: u8,
  pub pad0: [u8; 11usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_provider_property_iterator_t
{
  pub data: *mut xcb_randr_provider_property_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_resource_change_t
{
  pub timestamp: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub pad0: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_resource_change_iterator_t
{
  pub data: *mut xcb_randr_resource_change_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_monitor_info_t
{
  pub name: xcb_atom_t,
  pub primary: u8,
  pub automatic: u8,
  pub nOutput: u16,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub width_in_millimeters: u32,
  pub height_in_millimeters: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_monitor_info_iterator_t
{
  pub data: *mut xcb_randr_monitor_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_monitors_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_monitors_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub get_active: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_get_monitors_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub timestamp: xcb_timestamp_t,
  pub nMonitors: u32,
  pub nOutputs: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_set_monitor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_delete_monitor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub name: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_lease_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_lease_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub lid: xcb_randr_lease_t,
  pub num_crtcs: u16,
  pub num_outputs: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_create_lease_reply_t
{
  pub response_type: u8,
  pub nfd: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_free_lease_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub lid: xcb_randr_lease_t,
  pub terminate: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_lease_notify_t
{
  pub timestamp: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub lease: xcb_randr_lease_t,
  pub created: u8,
  pub pad0: [u8; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_lease_notify_iterator_t
{
  pub data: *mut xcb_randr_lease_notify_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_randr_notify_data_t
{
  pub cc: xcb_randr_crtc_change_t,
  pub oc: xcb_randr_output_change_t,
  pub op: xcb_randr_output_property_t,
  pub pc: xcb_randr_provider_change_t,
  pub pp: xcb_randr_provider_property_t,
  pub rc: xcb_randr_resource_change_t,
  pub lc: xcb_randr_lease_notify_t,
  _bindgen_union_align: [u32; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_randr_notify_data_iterator_t
{
  pub data: *mut xcb_randr_notify_data_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_randr_notify_event_t
{
  pub response_type: u8,
  pub subCode: u8,
  pub sequence: u16,
  pub u: xcb_randr_notify_data_t,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_randr_id: xcb_extension_t;

  pub fn xcb_randr_mode_next(i: *mut xcb_randr_mode_iterator_t);

  pub fn xcb_randr_mode_end(i: xcb_randr_mode_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_crtc_next(i: *mut xcb_randr_crtc_iterator_t);

  pub fn xcb_randr_crtc_end(i: xcb_randr_crtc_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_output_next(i: *mut xcb_randr_output_iterator_t);

  pub fn xcb_randr_output_end(i: xcb_randr_output_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_provider_next(i: *mut xcb_randr_provider_iterator_t);

  pub fn xcb_randr_provider_end(i: xcb_randr_provider_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_lease_next(i: *mut xcb_randr_lease_iterator_t);

  pub fn xcb_randr_lease_end(i: xcb_randr_lease_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_screen_size_next(i: *mut xcb_randr_screen_size_iterator_t);

  pub fn xcb_randr_screen_size_end(i: xcb_randr_screen_size_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_refresh_rates_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_refresh_rates_rates(R: *const xcb_randr_refresh_rates_t) -> *mut u16;

  pub fn xcb_randr_refresh_rates_rates_length(
    R: *const xcb_randr_refresh_rates_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_refresh_rates_rates_end(
    R: *const xcb_randr_refresh_rates_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_refresh_rates_next(i: *mut xcb_randr_refresh_rates_iterator_t);

  pub fn xcb_randr_refresh_rates_end(
    i: xcb_randr_refresh_rates_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_query_version(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_randr_query_version_cookie_t;

  pub fn xcb_randr_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_randr_query_version_cookie_t;

  pub fn xcb_randr_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_query_version_reply_t;

  pub fn xcb_randr_set_screen_config(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    timestamp: xcb_timestamp_t,
    config_timestamp: xcb_timestamp_t,
    sizeID: u16,
    rotation: u16,
    rate: u16,
  ) -> xcb_randr_set_screen_config_cookie_t;

  pub fn xcb_randr_set_screen_config_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    timestamp: xcb_timestamp_t,
    config_timestamp: xcb_timestamp_t,
    sizeID: u16,
    rotation: u16,
    rate: u16,
  ) -> xcb_randr_set_screen_config_cookie_t;

  pub fn xcb_randr_set_screen_config_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_set_screen_config_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_set_screen_config_reply_t;

  pub fn xcb_randr_select_input_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    enable: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_select_input(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    enable: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_get_screen_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_info(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_info_cookie_t;

  pub fn xcb_randr_get_screen_info_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_info_cookie_t;

  pub fn xcb_randr_get_screen_info_sizes(
    R: *const xcb_randr_get_screen_info_reply_t
  ) -> *mut xcb_randr_screen_size_t;

  pub fn xcb_randr_get_screen_info_sizes_length(
    R: *const xcb_randr_get_screen_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_info_sizes_iterator(
    R: *const xcb_randr_get_screen_info_reply_t
  ) -> xcb_randr_screen_size_iterator_t;

  pub fn xcb_randr_get_screen_info_rates_length(
    R: *const xcb_randr_get_screen_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_info_rates_iterator(
    R: *const xcb_randr_get_screen_info_reply_t
  ) -> xcb_randr_refresh_rates_iterator_t;

  pub fn xcb_randr_get_screen_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_screen_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_screen_info_reply_t;

  pub fn xcb_randr_get_screen_size_range(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_size_range_cookie_t;

  pub fn xcb_randr_get_screen_size_range_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_size_range_cookie_t;

  pub fn xcb_randr_get_screen_size_range_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_screen_size_range_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_screen_size_range_reply_t;

  pub fn xcb_randr_set_screen_size_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    width: u16,
    height: u16,
    mm_width: u32,
    mm_height: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_screen_size(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    width: u16,
    height: u16,
    mm_width: u32,
    mm_height: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_mode_info_next(i: *mut xcb_randr_mode_info_iterator_t);

  pub fn xcb_randr_mode_info_end(i: xcb_randr_mode_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_resources_cookie_t;

  pub fn xcb_randr_get_screen_resources_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_resources_cookie_t;

  pub fn xcb_randr_get_screen_resources_crtcs(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> *mut xcb_randr_crtc_t;

  pub fn xcb_randr_get_screen_resources_crtcs_length(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_crtcs_end(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_outputs(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_screen_resources_outputs_length(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_outputs_end(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_modes(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> *mut xcb_randr_mode_info_t;

  pub fn xcb_randr_get_screen_resources_modes_length(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_modes_iterator(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> xcb_randr_mode_info_iterator_t;

  pub fn xcb_randr_get_screen_resources_names(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> *mut u8;

  pub fn xcb_randr_get_screen_resources_names_length(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_names_end(
    R: *const xcb_randr_get_screen_resources_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_screen_resources_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_screen_resources_reply_t;

  pub fn xcb_randr_get_output_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_info(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_output_info_cookie_t;

  pub fn xcb_randr_get_output_info_unchecked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_output_info_cookie_t;

  pub fn xcb_randr_get_output_info_crtcs(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> *mut xcb_randr_crtc_t;

  pub fn xcb_randr_get_output_info_crtcs_length(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_info_crtcs_end(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_output_info_modes(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> *mut xcb_randr_mode_t;

  pub fn xcb_randr_get_output_info_modes_length(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_info_modes_end(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_output_info_clones(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_output_info_clones_length(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_info_clones_end(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_output_info_name(R: *const xcb_randr_get_output_info_reply_t) -> *mut u8;

  pub fn xcb_randr_get_output_info_name_length(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_info_name_end(
    R: *const xcb_randr_get_output_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_output_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_output_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_output_info_reply_t;

  pub fn xcb_randr_list_output_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_list_output_properties(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
  ) -> xcb_randr_list_output_properties_cookie_t;

  pub fn xcb_randr_list_output_properties_unchecked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
  ) -> xcb_randr_list_output_properties_cookie_t;

  pub fn xcb_randr_list_output_properties_atoms(
    R: *const xcb_randr_list_output_properties_reply_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_randr_list_output_properties_atoms_length(
    R: *const xcb_randr_list_output_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_list_output_properties_atoms_end(
    R: *const xcb_randr_list_output_properties_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_list_output_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_list_output_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_list_output_properties_reply_t;

  pub fn xcb_randr_query_output_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_query_output_property(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
  ) -> xcb_randr_query_output_property_cookie_t;

  pub fn xcb_randr_query_output_property_unchecked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
  ) -> xcb_randr_query_output_property_cookie_t;

  pub fn xcb_randr_query_output_property_valid_values(
    R: *const xcb_randr_query_output_property_reply_t
  ) -> *mut i32;

  pub fn xcb_randr_query_output_property_valid_values_length(
    R: *const xcb_randr_query_output_property_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_query_output_property_valid_values_end(
    R: *const xcb_randr_query_output_property_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_query_output_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_query_output_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_query_output_property_reply_t;

  pub fn xcb_randr_configure_output_property_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    values_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_configure_output_property_checked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    pending: u8,
    range: u8,
    values_len: u32,
    values: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_configure_output_property(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    pending: u8,
    range: u8,
    values_len: u32,
    values: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_configure_output_property_values(
    R: *const xcb_randr_configure_output_property_request_t
  ) -> *mut i32;

  pub fn xcb_randr_configure_output_property_values_length(
    R: *const xcb_randr_configure_output_property_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_configure_output_property_values_end(
    R: *const xcb_randr_configure_output_property_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_change_output_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_change_output_property_checked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    mode: u8,
    num_units: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_change_output_property(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    mode: u8,
    num_units: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_change_output_property_data(
    R: *const xcb_randr_change_output_property_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_randr_change_output_property_data_length(
    R: *const xcb_randr_change_output_property_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_change_output_property_data_end(
    R: *const xcb_randr_change_output_property_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_delete_output_property_checked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_delete_output_property(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_get_output_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_property(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
    _delete: u8,
    pending: u8,
  ) -> xcb_randr_get_output_property_cookie_t;

  pub fn xcb_randr_get_output_property_unchecked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
    _delete: u8,
    pending: u8,
  ) -> xcb_randr_get_output_property_cookie_t;

  pub fn xcb_randr_get_output_property_data(
    R: *const xcb_randr_get_output_property_reply_t
  ) -> *mut u8;

  pub fn xcb_randr_get_output_property_data_length(
    R: *const xcb_randr_get_output_property_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_output_property_data_end(
    R: *const xcb_randr_get_output_property_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_output_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_output_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_output_property_reply_t;

  pub fn xcb_randr_create_mode_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    name_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_create_mode(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    mode_info: xcb_randr_mode_info_t,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_randr_create_mode_cookie_t;

  pub fn xcb_randr_create_mode_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    mode_info: xcb_randr_mode_info_t,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_randr_create_mode_cookie_t;

  pub fn xcb_randr_create_mode_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_create_mode_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_create_mode_reply_t;

  pub fn xcb_randr_destroy_mode_checked(
    c: *mut xcb_connection_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_destroy_mode(
    c: *mut xcb_connection_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_add_output_mode_checked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_add_output_mode(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_delete_output_mode_checked(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_delete_output_mode(
    c: *mut xcb_connection_t,
    output: xcb_randr_output_t,
    mode: xcb_randr_mode_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_get_crtc_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_info(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_crtc_info_cookie_t;

  pub fn xcb_randr_get_crtc_info_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_crtc_info_cookie_t;

  pub fn xcb_randr_get_crtc_info_outputs(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_crtc_info_outputs_length(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_info_outputs_end(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_info_possible(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_crtc_info_possible_length(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_info_possible_end(
    R: *const xcb_randr_get_crtc_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_crtc_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_crtc_info_reply_t;

  pub fn xcb_randr_set_crtc_config_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    outputs_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_config(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    timestamp: xcb_timestamp_t,
    config_timestamp: xcb_timestamp_t,
    x: i16,
    y: i16,
    mode: xcb_randr_mode_t,
    rotation: u16,
    outputs_len: u32,
    outputs: *const xcb_randr_output_t,
  ) -> xcb_randr_set_crtc_config_cookie_t;

  pub fn xcb_randr_set_crtc_config_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    timestamp: xcb_timestamp_t,
    config_timestamp: xcb_timestamp_t,
    x: i16,
    y: i16,
    mode: xcb_randr_mode_t,
    rotation: u16,
    outputs_len: u32,
    outputs: *const xcb_randr_output_t,
  ) -> xcb_randr_set_crtc_config_cookie_t;

  pub fn xcb_randr_set_crtc_config_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_set_crtc_config_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_set_crtc_config_reply_t;

  pub fn xcb_randr_get_crtc_gamma_size(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_gamma_size_cookie_t;

  pub fn xcb_randr_get_crtc_gamma_size_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_gamma_size_cookie_t;

  pub fn xcb_randr_get_crtc_gamma_size_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_crtc_gamma_size_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_crtc_gamma_size_reply_t;

  pub fn xcb_randr_get_crtc_gamma_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_gamma(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_gamma_cookie_t;

  pub fn xcb_randr_get_crtc_gamma_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_gamma_cookie_t;

  pub fn xcb_randr_get_crtc_gamma_red(R: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;

  pub fn xcb_randr_get_crtc_gamma_red_length(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_gamma_red_end(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_gamma_green(R: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;

  pub fn xcb_randr_get_crtc_gamma_green_length(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_gamma_green_end(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_gamma_blue(R: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;

  pub fn xcb_randr_get_crtc_gamma_blue_length(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_gamma_blue_end(
    R: *const xcb_randr_get_crtc_gamma_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_gamma_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_crtc_gamma_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_crtc_gamma_reply_t;

  pub fn xcb_randr_set_crtc_gamma_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_gamma_checked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    size: u16,
    red: *const u16,
    green: *const u16,
    blue: *const u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_crtc_gamma(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    size: u16,
    red: *const u16,
    green: *const u16,
    blue: *const u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_crtc_gamma_red(R: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16;

  pub fn xcb_randr_set_crtc_gamma_red_length(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_gamma_red_end(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_set_crtc_gamma_green(R: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16;

  pub fn xcb_randr_set_crtc_gamma_green_length(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_gamma_green_end(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_set_crtc_gamma_blue(R: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16;

  pub fn xcb_randr_set_crtc_gamma_blue_length(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_gamma_blue_end(
    R: *const xcb_randr_set_crtc_gamma_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_current_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_current(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_resources_current_cookie_t;

  pub fn xcb_randr_get_screen_resources_current_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_screen_resources_current_cookie_t;

  pub fn xcb_randr_get_screen_resources_current_crtcs(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> *mut xcb_randr_crtc_t;

  pub fn xcb_randr_get_screen_resources_current_crtcs_length(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_current_crtcs_end(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_current_outputs(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_screen_resources_current_outputs_length(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_current_outputs_end(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_current_modes(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> *mut xcb_randr_mode_info_t;

  pub fn xcb_randr_get_screen_resources_current_modes_length(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_current_modes_iterator(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> xcb_randr_mode_info_iterator_t;

  pub fn xcb_randr_get_screen_resources_current_names(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> *mut u8;

  pub fn xcb_randr_get_screen_resources_current_names_length(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_screen_resources_current_names_end(
    R: *const xcb_randr_get_screen_resources_current_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_screen_resources_current_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_screen_resources_current_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_screen_resources_current_reply_t;

  pub fn xcb_randr_set_crtc_transform_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    filter_params_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_transform_checked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    transform: xcb_render_transform_t,
    filter_len: u16,
    filter_name: *const ::std::os::raw::c_char,
    filter_params_len: u32,
    filter_params: *const xcb_render_fixed_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_crtc_transform(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    transform: xcb_render_transform_t,
    filter_len: u16,
    filter_name: *const ::std::os::raw::c_char,
    filter_params_len: u32,
    filter_params: *const xcb_render_fixed_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_crtc_transform_filter_name(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_randr_set_crtc_transform_filter_name_length(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_transform_filter_name_end(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_set_crtc_transform_filter_params(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_randr_set_crtc_transform_filter_params_length(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_crtc_transform_filter_params_end(
    R: *const xcb_randr_set_crtc_transform_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_transform_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_transform(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_transform_cookie_t;

  pub fn xcb_randr_get_crtc_transform_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_crtc_transform_cookie_t;

  pub fn xcb_randr_get_crtc_transform_pending_filter_name(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_randr_get_crtc_transform_pending_filter_name_length(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_transform_pending_filter_name_end(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_transform_pending_params(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_randr_get_crtc_transform_pending_params_length(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_transform_pending_params_end(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_transform_current_filter_name(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_randr_get_crtc_transform_current_filter_name_length(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_transform_current_filter_name_end(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_transform_current_params(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> *mut xcb_render_fixed_t;

  pub fn xcb_randr_get_crtc_transform_current_params_length(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_crtc_transform_current_params_end(
    R: *const xcb_randr_get_crtc_transform_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_crtc_transform_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_crtc_transform_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_crtc_transform_reply_t;

  pub fn xcb_randr_get_panning(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_panning_cookie_t;

  pub fn xcb_randr_get_panning_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
  ) -> xcb_randr_get_panning_cookie_t;

  pub fn xcb_randr_get_panning_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_panning_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_panning_reply_t;

  pub fn xcb_randr_set_panning(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    timestamp: xcb_timestamp_t,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    track_left: u16,
    track_top: u16,
    track_width: u16,
    track_height: u16,
    border_left: i16,
    border_top: i16,
    border_right: i16,
    border_bottom: i16,
  ) -> xcb_randr_set_panning_cookie_t;

  pub fn xcb_randr_set_panning_unchecked(
    c: *mut xcb_connection_t,
    crtc: xcb_randr_crtc_t,
    timestamp: xcb_timestamp_t,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    track_left: u16,
    track_top: u16,
    track_width: u16,
    track_height: u16,
    border_left: i16,
    border_top: i16,
    border_right: i16,
    border_bottom: i16,
  ) -> xcb_randr_set_panning_cookie_t;

  pub fn xcb_randr_set_panning_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_set_panning_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_set_panning_reply_t;

  pub fn xcb_randr_set_output_primary_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    output: xcb_randr_output_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_output_primary(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    output: xcb_randr_output_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_get_output_primary(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_output_primary_cookie_t;

  pub fn xcb_randr_get_output_primary_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_output_primary_cookie_t;

  pub fn xcb_randr_get_output_primary_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_output_primary_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_output_primary_reply_t;

  pub fn xcb_randr_get_providers_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_providers(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_providers_cookie_t;

  pub fn xcb_randr_get_providers_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_randr_get_providers_cookie_t;

  pub fn xcb_randr_get_providers_providers(
    R: *const xcb_randr_get_providers_reply_t
  ) -> *mut xcb_randr_provider_t;

  pub fn xcb_randr_get_providers_providers_length(
    R: *const xcb_randr_get_providers_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_providers_providers_end(
    R: *const xcb_randr_get_providers_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_providers_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_providers_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_providers_reply_t;

  pub fn xcb_randr_get_provider_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_provider_info_cookie_t;

  pub fn xcb_randr_get_provider_info_unchecked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_randr_get_provider_info_cookie_t;

  pub fn xcb_randr_get_provider_info_crtcs(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> *mut xcb_randr_crtc_t;

  pub fn xcb_randr_get_provider_info_crtcs_length(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info_crtcs_end(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_info_outputs(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_get_provider_info_outputs_length(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info_outputs_end(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_info_associated_providers(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> *mut xcb_randr_provider_t;

  pub fn xcb_randr_get_provider_info_associated_providers_length(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info_associated_providers_end(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_info_associated_capability(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> *mut u32;

  pub fn xcb_randr_get_provider_info_associated_capability_length(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info_associated_capability_end(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_info_name(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_randr_get_provider_info_name_length(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_info_name_end(
    R: *const xcb_randr_get_provider_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_provider_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_provider_info_reply_t;

  pub fn xcb_randr_set_provider_offload_sink_checked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    sink_provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_provider_offload_sink(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    sink_provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_provider_output_source_checked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    source_provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_provider_output_source(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    source_provider: xcb_randr_provider_t,
    config_timestamp: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_list_provider_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_list_provider_properties(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
  ) -> xcb_randr_list_provider_properties_cookie_t;

  pub fn xcb_randr_list_provider_properties_unchecked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
  ) -> xcb_randr_list_provider_properties_cookie_t;

  pub fn xcb_randr_list_provider_properties_atoms(
    R: *const xcb_randr_list_provider_properties_reply_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_randr_list_provider_properties_atoms_length(
    R: *const xcb_randr_list_provider_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_list_provider_properties_atoms_end(
    R: *const xcb_randr_list_provider_properties_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_list_provider_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_list_provider_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_list_provider_properties_reply_t;

  pub fn xcb_randr_query_provider_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_query_provider_property(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
  ) -> xcb_randr_query_provider_property_cookie_t;

  pub fn xcb_randr_query_provider_property_unchecked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
  ) -> xcb_randr_query_provider_property_cookie_t;

  pub fn xcb_randr_query_provider_property_valid_values(
    R: *const xcb_randr_query_provider_property_reply_t
  ) -> *mut i32;

  pub fn xcb_randr_query_provider_property_valid_values_length(
    R: *const xcb_randr_query_provider_property_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_query_provider_property_valid_values_end(
    R: *const xcb_randr_query_provider_property_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_query_provider_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_query_provider_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_query_provider_property_reply_t;

  pub fn xcb_randr_configure_provider_property_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    values_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_configure_provider_property_checked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    pending: u8,
    range: u8,
    values_len: u32,
    values: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_configure_provider_property(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    pending: u8,
    range: u8,
    values_len: u32,
    values: *const i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_configure_provider_property_values(
    R: *const xcb_randr_configure_provider_property_request_t
  ) -> *mut i32;

  pub fn xcb_randr_configure_provider_property_values_length(
    R: *const xcb_randr_configure_provider_property_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_configure_provider_property_values_end(
    R: *const xcb_randr_configure_provider_property_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_change_provider_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_change_provider_property_checked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    mode: u8,
    num_items: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_change_provider_property(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    mode: u8,
    num_items: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_change_provider_property_data(
    R: *const xcb_randr_change_provider_property_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_randr_change_provider_property_data_length(
    R: *const xcb_randr_change_provider_property_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_change_provider_property_data_end(
    R: *const xcb_randr_change_provider_property_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_delete_provider_property_checked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_delete_provider_property(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_get_provider_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_property(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
    _delete: u8,
    pending: u8,
  ) -> xcb_randr_get_provider_property_cookie_t;

  pub fn xcb_randr_get_provider_property_unchecked(
    c: *mut xcb_connection_t,
    provider: xcb_randr_provider_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
    _delete: u8,
    pending: u8,
  ) -> xcb_randr_get_provider_property_cookie_t;

  pub fn xcb_randr_get_provider_property_data(
    R: *const xcb_randr_get_provider_property_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_randr_get_provider_property_data_length(
    R: *const xcb_randr_get_provider_property_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_provider_property_data_end(
    R: *const xcb_randr_get_provider_property_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_provider_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_provider_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_provider_property_reply_t;

  pub fn xcb_randr_crtc_change_next(i: *mut xcb_randr_crtc_change_iterator_t);

  pub fn xcb_randr_crtc_change_end(i: xcb_randr_crtc_change_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_randr_output_change_next(i: *mut xcb_randr_output_change_iterator_t);

  pub fn xcb_randr_output_change_end(
    i: xcb_randr_output_change_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_output_property_next(i: *mut xcb_randr_output_property_iterator_t);

  pub fn xcb_randr_output_property_end(
    i: xcb_randr_output_property_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_provider_change_next(i: *mut xcb_randr_provider_change_iterator_t);

  pub fn xcb_randr_provider_change_end(
    i: xcb_randr_provider_change_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_provider_property_next(i: *mut xcb_randr_provider_property_iterator_t);

  pub fn xcb_randr_provider_property_end(
    i: xcb_randr_provider_property_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_resource_change_next(i: *mut xcb_randr_resource_change_iterator_t);

  pub fn xcb_randr_resource_change_end(
    i: xcb_randr_resource_change_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_monitor_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_monitor_info_outputs(
    R: *const xcb_randr_monitor_info_t
  ) -> *mut xcb_randr_output_t;

  pub fn xcb_randr_monitor_info_outputs_length(
    R: *const xcb_randr_monitor_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_monitor_info_outputs_end(
    R: *const xcb_randr_monitor_info_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_randr_monitor_info_next(i: *mut xcb_randr_monitor_info_iterator_t);

  pub fn xcb_randr_monitor_info_end(i: xcb_randr_monitor_info_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_randr_get_monitors_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_monitors(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    get_active: u8,
  ) -> xcb_randr_get_monitors_cookie_t;

  pub fn xcb_randr_get_monitors_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    get_active: u8,
  ) -> xcb_randr_get_monitors_cookie_t;

  pub fn xcb_randr_get_monitors_monitors_length(
    R: *const xcb_randr_get_monitors_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_get_monitors_monitors_iterator(
    R: *const xcb_randr_get_monitors_reply_t
  ) -> xcb_randr_monitor_info_iterator_t;

  pub fn xcb_randr_get_monitors_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_get_monitors_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_get_monitors_reply_t;

  pub fn xcb_randr_set_monitor_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_set_monitor_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    monitorinfo: *mut xcb_randr_monitor_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_monitor(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    monitorinfo: *mut xcb_randr_monitor_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_set_monitor_monitorinfo(
    R: *const xcb_randr_set_monitor_request_t
  ) -> *mut xcb_randr_monitor_info_t;

  pub fn xcb_randr_delete_monitor_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    name: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_delete_monitor(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    name: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_create_lease_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_randr_create_lease(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    lid: xcb_randr_lease_t,
    num_crtcs: u16,
    num_outputs: u16,
    crtcs: *const xcb_randr_crtc_t,
    outputs: *const xcb_randr_output_t,
  ) -> xcb_randr_create_lease_cookie_t;

  pub fn xcb_randr_create_lease_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    lid: xcb_randr_lease_t,
    num_crtcs: u16,
    num_outputs: u16,
    crtcs: *const xcb_randr_crtc_t,
    outputs: *const xcb_randr_output_t,
  ) -> xcb_randr_create_lease_cookie_t;

  pub fn xcb_randr_create_lease_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_randr_create_lease_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_randr_create_lease_reply_t;

  pub fn xcb_randr_create_lease_reply_fds(
    c: *mut xcb_connection_t,
    reply: *mut xcb_randr_create_lease_reply_t,
  ) -> *mut ::std::os::raw::c_int;

  pub fn xcb_randr_free_lease_checked(
    c: *mut xcb_connection_t,
    lid: xcb_randr_lease_t,
    terminate: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_free_lease(
    c: *mut xcb_connection_t,
    lid: xcb_randr_lease_t,
    terminate: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_randr_lease_notify_next(i: *mut xcb_randr_lease_notify_iterator_t);

  pub fn xcb_randr_lease_notify_end(i: xcb_randr_lease_notify_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_randr_notify_data_next(i: *mut xcb_randr_notify_data_iterator_t);

  pub fn xcb_randr_notify_data_end(i: xcb_randr_notify_data_iterator_t) -> xcb_generic_iterator_t;
}
