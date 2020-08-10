use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_raw_generic_event_t,
  xcb_void_cookie_t,
};
use super::xcb::{
  xcb_atom_t,
  xcb_cursor_t,
  xcb_keysym_t,
  xcb_str_iterator_t,
  xcb_timestamp_t,
  xcb_window_t,
};
use super::xfixes::xcb_xfixes_barrier_t;

pub const XCB_INPUT_MAJOR_VERSION: u32 = 2;
pub const XCB_INPUT_MINOR_VERSION: u32 = 3;
pub const XCB_INPUT_GET_EXTENSION_VERSION: u32 = 1;
pub const XCB_INPUT_LIST_INPUT_DEVICES: u32 = 2;
pub const XCB_INPUT_OPEN_DEVICE: u32 = 3;
pub const XCB_INPUT_CLOSE_DEVICE: u32 = 4;
pub const XCB_INPUT_SET_DEVICE_MODE: u32 = 5;
pub const XCB_INPUT_SELECT_EXTENSION_EVENT: u32 = 6;
pub const XCB_INPUT_GET_SELECTED_EXTENSION_EVENTS: u32 = 7;
pub const XCB_INPUT_CHANGE_DEVICE_DONT_PROPAGATE_LIST: u32 = 8;
pub const XCB_INPUT_GET_DEVICE_DONT_PROPAGATE_LIST: u32 = 9;
pub const XCB_INPUT_GET_DEVICE_MOTION_EVENTS: u32 = 10;
pub const XCB_INPUT_CHANGE_KEYBOARD_DEVICE: u32 = 11;
pub const XCB_INPUT_CHANGE_POINTER_DEVICE: u32 = 12;
pub const XCB_INPUT_GRAB_DEVICE: u32 = 13;
pub const XCB_INPUT_UNGRAB_DEVICE: u32 = 14;
pub const XCB_INPUT_GRAB_DEVICE_KEY: u32 = 15;
pub const XCB_INPUT_UNGRAB_DEVICE_KEY: u32 = 16;
pub const XCB_INPUT_GRAB_DEVICE_BUTTON: u32 = 17;
pub const XCB_INPUT_UNGRAB_DEVICE_BUTTON: u32 = 18;
pub const XCB_INPUT_ALLOW_DEVICE_EVENTS: u32 = 19;
pub const XCB_INPUT_GET_DEVICE_FOCUS: u32 = 20;
pub const XCB_INPUT_SET_DEVICE_FOCUS: u32 = 21;
pub const XCB_INPUT_GET_FEEDBACK_CONTROL: u32 = 22;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL: u32 = 23;
pub const XCB_INPUT_GET_DEVICE_KEY_MAPPING: u32 = 24;
pub const XCB_INPUT_CHANGE_DEVICE_KEY_MAPPING: u32 = 25;
pub const XCB_INPUT_GET_DEVICE_MODIFIER_MAPPING: u32 = 26;
pub const XCB_INPUT_SET_DEVICE_MODIFIER_MAPPING: u32 = 27;
pub const XCB_INPUT_GET_DEVICE_BUTTON_MAPPING: u32 = 28;
pub const XCB_INPUT_SET_DEVICE_BUTTON_MAPPING: u32 = 29;
pub const XCB_INPUT_QUERY_DEVICE_STATE: u32 = 30;
pub const XCB_INPUT_DEVICE_BELL: u32 = 32;
pub const XCB_INPUT_SET_DEVICE_VALUATORS: u32 = 33;
pub const XCB_INPUT_GET_DEVICE_CONTROL: u32 = 34;
pub const XCB_INPUT_CHANGE_DEVICE_CONTROL: u32 = 35;
pub const XCB_INPUT_LIST_DEVICE_PROPERTIES: u32 = 36;
pub const XCB_INPUT_CHANGE_DEVICE_PROPERTY: u32 = 37;
pub const XCB_INPUT_DELETE_DEVICE_PROPERTY: u32 = 38;
pub const XCB_INPUT_GET_DEVICE_PROPERTY: u32 = 39;
pub const XCB_INPUT_XI_QUERY_POINTER: u32 = 40;
pub const XCB_INPUT_XI_WARP_POINTER: u32 = 41;
pub const XCB_INPUT_XI_CHANGE_CURSOR: u32 = 42;
pub const XCB_INPUT_XI_CHANGE_HIERARCHY: u32 = 43;
pub const XCB_INPUT_XI_SET_CLIENT_POINTER: u32 = 44;
pub const XCB_INPUT_XI_GET_CLIENT_POINTER: u32 = 45;
pub const XCB_INPUT_XI_SELECT_EVENTS: u32 = 46;
pub const XCB_INPUT_XI_QUERY_VERSION: u32 = 47;
pub const XCB_INPUT_XI_QUERY_DEVICE: u32 = 48;
pub const XCB_INPUT_XI_SET_FOCUS: u32 = 49;
pub const XCB_INPUT_XI_GET_FOCUS: u32 = 50;
pub const XCB_INPUT_XI_GRAB_DEVICE: u32 = 51;
pub const XCB_INPUT_XI_UNGRAB_DEVICE: u32 = 52;
pub const XCB_INPUT_XI_ALLOW_EVENTS: u32 = 53;
pub const XCB_INPUT_XI_PASSIVE_GRAB_DEVICE: u32 = 54;
pub const XCB_INPUT_XI_PASSIVE_UNGRAB_DEVICE: u32 = 55;
pub const XCB_INPUT_XI_LIST_PROPERTIES: u32 = 56;
pub const XCB_INPUT_XI_CHANGE_PROPERTY: u32 = 57;
pub const XCB_INPUT_XI_DELETE_PROPERTY: u32 = 58;
pub const XCB_INPUT_XI_GET_PROPERTY: u32 = 59;
pub const XCB_INPUT_XI_GET_SELECTED_EVENTS: u32 = 60;
pub const XCB_INPUT_XI_BARRIER_RELEASE_POINTER: u32 = 61;
pub const XCB_INPUT_DEVICE_VALUATOR: u32 = 0;
pub const XCB_INPUT_DEVICE_KEY_PRESS: u32 = 1;
pub const XCB_INPUT_DEVICE_KEY_RELEASE: u32 = 2;
pub const XCB_INPUT_DEVICE_BUTTON_PRESS: u32 = 3;
pub const XCB_INPUT_DEVICE_BUTTON_RELEASE: u32 = 4;
pub const XCB_INPUT_DEVICE_MOTION_NOTIFY: u32 = 5;
pub const XCB_INPUT_DEVICE_FOCUS_IN: u32 = 6;
pub const XCB_INPUT_DEVICE_FOCUS_OUT: u32 = 7;
pub const XCB_INPUT_PROXIMITY_IN: u32 = 8;
pub const XCB_INPUT_PROXIMITY_OUT: u32 = 9;
pub const XCB_INPUT_DEVICE_STATE_NOTIFY: u32 = 10;
pub const XCB_INPUT_DEVICE_MAPPING_NOTIFY: u32 = 11;
pub const XCB_INPUT_CHANGE_DEVICE_NOTIFY: u32 = 12;
pub const XCB_INPUT_DEVICE_KEY_STATE_NOTIFY: u32 = 13;
pub const XCB_INPUT_DEVICE_BUTTON_STATE_NOTIFY: u32 = 14;
pub const XCB_INPUT_DEVICE_PRESENCE_NOTIFY: u32 = 15;
pub const XCB_INPUT_DEVICE_PROPERTY_NOTIFY: u32 = 16;
pub const XCB_INPUT_DEVICE_CHANGED: u32 = 1;
pub const XCB_INPUT_KEY_PRESS: u32 = 2;
pub const XCB_INPUT_KEY_RELEASE: u32 = 3;
pub const XCB_INPUT_BUTTON_PRESS: u32 = 4;
pub const XCB_INPUT_BUTTON_RELEASE: u32 = 5;
pub const XCB_INPUT_MOTION: u32 = 6;
pub const XCB_INPUT_ENTER: u32 = 7;
pub const XCB_INPUT_LEAVE: u32 = 8;
pub const XCB_INPUT_FOCUS_IN: u32 = 9;
pub const XCB_INPUT_FOCUS_OUT: u32 = 10;
pub const XCB_INPUT_HIERARCHY: u32 = 11;
pub const XCB_INPUT_PROPERTY: u32 = 12;
pub const XCB_INPUT_RAW_KEY_PRESS: u32 = 13;
pub const XCB_INPUT_RAW_KEY_RELEASE: u32 = 14;
pub const XCB_INPUT_RAW_BUTTON_PRESS: u32 = 15;
pub const XCB_INPUT_RAW_BUTTON_RELEASE: u32 = 16;
pub const XCB_INPUT_RAW_MOTION: u32 = 17;
pub const XCB_INPUT_TOUCH_BEGIN: u32 = 18;
pub const XCB_INPUT_TOUCH_UPDATE: u32 = 19;
pub const XCB_INPUT_TOUCH_END: u32 = 20;
pub const XCB_INPUT_TOUCH_OWNERSHIP: u32 = 21;
pub const XCB_INPUT_RAW_TOUCH_BEGIN: u32 = 22;
pub const XCB_INPUT_RAW_TOUCH_UPDATE: u32 = 23;
pub const XCB_INPUT_RAW_TOUCH_END: u32 = 24;
pub const XCB_INPUT_BARRIER_HIT: u32 = 25;
pub const XCB_INPUT_BARRIER_LEAVE: u32 = 26;
pub const XCB_INPUT_SEND_EXTENSION_EVENT: u32 = 31;
pub const XCB_INPUT_DEVICE: u32 = 0;
pub const XCB_INPUT_EVENT: u32 = 1;
pub const XCB_INPUT_MODE: u32 = 2;
pub const XCB_INPUT_DEVICE_BUSY: u32 = 3;
pub const XCB_INPUT_CLASS: u32 = 4;

pub type xcb_input_event_class_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_class_iterator_t
{
  pub data: *mut xcb_input_event_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_input_key_code_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_code_iterator_t
{
  pub data: *mut xcb_input_key_code_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_input_device_id_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_id_iterator_t
{
  pub data: *mut xcb_input_device_id_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_input_fp1616_t = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_fp1616_iterator_t
{
  pub data: *mut xcb_input_fp1616_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_fp3232_t
{
  pub integral: i32,
  pub frac: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_fp3232_iterator_t
{
  pub data: *mut xcb_input_fp3232_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_extension_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_extension_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub name_len: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_extension_version_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub server_major: u16,
  pub server_minor: u16,
  pub present: u8,
  pub pad0: [u8; 19usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_use_t
{
  XCB_INPUT_DEVICE_USE_IS_X_POINTER = 0,
  XCB_INPUT_DEVICE_USE_IS_X_KEYBOARD = 1,
  XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_DEVICE = 2,
  XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_KEYBOARD = 3,
  XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_POINTER = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_input_class_t
{
  XCB_INPUT_INPUT_CLASS_KEY = 0,
  XCB_INPUT_INPUT_CLASS_BUTTON = 1,
  XCB_INPUT_INPUT_CLASS_VALUATOR = 2,
  XCB_INPUT_INPUT_CLASS_FEEDBACK = 3,
  XCB_INPUT_INPUT_CLASS_PROXIMITY = 4,
  XCB_INPUT_INPUT_CLASS_FOCUS = 5,
  XCB_INPUT_INPUT_CLASS_OTHER = 6,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_valuator_mode_t
{
  XCB_INPUT_VALUATOR_MODE_RELATIVE = 0,
  XCB_INPUT_VALUATOR_MODE_ABSOLUTE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_info_t
{
  pub device_type: xcb_atom_t,
  pub device_id: u8,
  pub num_class_info: u8,
  pub device_use: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_info_iterator_t
{
  pub data: *mut xcb_input_device_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_info_t
{
  pub class_id: u8,
  pub len: u8,
  pub min_keycode: xcb_input_key_code_t,
  pub max_keycode: xcb_input_key_code_t,
  pub num_keys: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_info_iterator_t
{
  pub data: *mut xcb_input_key_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_info_t
{
  pub class_id: u8,
  pub len: u8,
  pub num_buttons: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_info_iterator_t
{
  pub data: *mut xcb_input_button_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_axis_info_t
{
  pub resolution: u32,
  pub minimum: i32,
  pub maximum: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_axis_info_iterator_t
{
  pub data: *mut xcb_input_axis_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_info_t
{
  pub class_id: u8,
  pub len: u8,
  pub axes_len: u8,
  pub mode: u8,
  pub motion_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_info_iterator_t
{
  pub data: *mut xcb_input_valuator_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_info_t
{
  pub key: xcb_input_input_info_info_t__bindgen_ty_1,
  pub button: xcb_input_input_info_info_t__bindgen_ty_2,
  pub valuator: xcb_input_input_info_info_t__bindgen_ty_3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_info_t__bindgen_ty_1
{
  pub min_keycode: xcb_input_key_code_t,
  pub max_keycode: xcb_input_key_code_t,
  pub num_keys: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_info_t__bindgen_ty_2
{
  pub num_buttons: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_info_t__bindgen_ty_3
{
  pub axes_len: u8,
  pub mode: u8,
  pub motion_size: u32,
  pub axes: *mut xcb_input_axis_info_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_t
{
  pub class_id: u8,
  pub len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_info_iterator_t
{
  pub data: *mut xcb_input_input_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_name_t
{
  pub len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_name_iterator_t
{
  pub data: *mut xcb_input_device_name_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_input_devices_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_input_devices_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_input_devices_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub devices_len: u8,
  pub pad0: [u8; 23usize],
}

pub type xcb_input_event_type_base_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_type_base_iterator_t
{
  pub data: *mut xcb_input_event_type_base_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_class_info_t
{
  pub class_id: u8,
  pub event_type_base: xcb_input_event_type_base_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_class_info_iterator_t
{
  pub data: *mut xcb_input_input_class_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_open_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_open_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_open_device_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_classes: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_close_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_mode_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_mode_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub mode: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_mode_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_select_extension_event_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub num_classes: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_selected_extension_events_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_selected_extension_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_selected_extension_events_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_this_classes: u16,
  pub num_all_classes: u16,
  pub pad0: [u8; 20usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_propagate_mode_t
{
  XCB_INPUT_PROPAGATE_MODE_ADD_TO_LIST = 0,
  XCB_INPUT_PROPAGATE_MODE_DELETE_FROM_LIST = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_dont_propagate_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub num_classes: u16,
  pub mode: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_dont_propagate_list_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_dont_propagate_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_dont_propagate_list_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_classes: u16,
  pub pad0: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_time_coord_t
{
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_time_coord_iterator_t
{
  pub data: *mut xcb_input_device_time_coord_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
  pub num_axes: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_motion_events_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_motion_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub start: xcb_timestamp_t,
  pub stop: xcb_timestamp_t,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_motion_events_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_events: u32,
  pub num_axes: u8,
  pub device_mode: u8,
  pub pad0: [u8; 18usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_keyboard_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_keyboard_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_keyboard_device_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_pointer_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_pointer_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub x_axis: u8,
  pub y_axis: u8,
  pub device_id: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_pointer_device_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub num_classes: u16,
  pub this_device_mode: u8,
  pub other_device_mode: u8,
  pub owner_events: u8,
  pub device_id: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_device_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ungrab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_modifier_device_t
{
  XCB_INPUT_MODIFIER_DEVICE_USE_X_KEYBOARD = 255,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_device_key_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub num_classes: u16,
  pub modifiers: u16,
  pub modifier_device: u8,
  pub grabbed_device: u8,
  pub key: u8,
  pub this_device_mode: u8,
  pub other_device_mode: u8,
  pub owner_events: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ungrab_device_key_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grabWindow: xcb_window_t,
  pub modifiers: u16,
  pub modifier_device: u8,
  pub key: u8,
  pub grabbed_device: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_device_button_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub grabbed_device: u8,
  pub modifier_device: u8,
  pub num_classes: u16,
  pub modifiers: u16,
  pub this_device_mode: u8,
  pub other_device_mode: u8,
  pub button: u8,
  pub owner_events: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ungrab_device_button_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub modifiers: u16,
  pub modifier_device: u8,
  pub button: u8,
  pub grabbed_device: u8,
  pub pad0: [u8; 3usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_input_mode_t
{
  XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_THIS_DEVICE = 0,
  XCB_INPUT_DEVICE_INPUT_MODE_SYNC_THIS_DEVICE = 1,
  XCB_INPUT_DEVICE_INPUT_MODE_REPLAY_THIS_DEVICE = 2,
  XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_OTHER_DEVICES = 3,
  XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_ALL = 4,
  XCB_INPUT_DEVICE_INPUT_MODE_SYNC_ALL = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_allow_device_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
  pub mode: u8,
  pub device_id: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_focus_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_focus_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_focus_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub focus: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub revert_to: u8,
  pub pad0: [u8; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_focus_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub focus: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub revert_to: u8,
  pub device_id: u8,
  pub pad0: [u8; 2usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_feedback_class_t
{
  XCB_INPUT_FEEDBACK_CLASS_KEYBOARD = 0,
  XCB_INPUT_FEEDBACK_CLASS_POINTER = 1,
  XCB_INPUT_FEEDBACK_CLASS_STRING = 2,
  XCB_INPUT_FEEDBACK_CLASS_INTEGER = 3,
  XCB_INPUT_FEEDBACK_CLASS_LED = 4,
  XCB_INPUT_FEEDBACK_CLASS_BELL = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_kbd_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub pitch: u16,
  pub duration: u16,
  pub led_mask: u32,
  pub led_values: u32,
  pub global_auto_repeat: u8,
  pub click: u8,
  pub percent: u8,
  pub pad0: u8,
  pub auto_repeats: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_kbd_feedback_state_iterator_t
{
  pub data: *mut xcb_input_kbd_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ptr_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub pad0: [u8; 2usize],
  pub accel_num: u16,
  pub accel_denom: u16,
  pub threshold: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ptr_feedback_state_iterator_t
{
  pub data: *mut xcb_input_ptr_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_integer_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub resolution: u32,
  pub min_value: i32,
  pub max_value: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_integer_feedback_state_iterator_t
{
  pub data: *mut xcb_input_integer_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_string_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub max_symbols: u16,
  pub num_keysyms: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_string_feedback_state_iterator_t
{
  pub data: *mut xcb_input_string_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_bell_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub percent: u8,
  pub pad0: [u8; 3usize],
  pub pitch: u16,
  pub duration: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_bell_feedback_state_iterator_t
{
  pub data: *mut xcb_input_bell_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_led_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_led_feedback_state_iterator_t
{
  pub data: *mut xcb_input_led_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t
{
  pub keyboard: xcb_input_feedback_state_data_t__bindgen_ty_1,
  pub pointer: xcb_input_feedback_state_data_t__bindgen_ty_2,
  pub string: xcb_input_feedback_state_data_t__bindgen_ty_3,
  pub integer: xcb_input_feedback_state_data_t__bindgen_ty_4,
  pub led: xcb_input_feedback_state_data_t__bindgen_ty_5,
  pub bell: xcb_input_feedback_state_data_t__bindgen_ty_6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_1
{
  pub pitch: u16,
  pub duration: u16,
  pub led_mask: u32,
  pub led_values: u32,
  pub global_auto_repeat: u8,
  pub click: u8,
  pub percent: u8,
  pub pad0: u8,
  pub auto_repeats: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_2
{
  pub pad1: [u8; 2usize],
  pub accel_num: u16,
  pub accel_denom: u16,
  pub threshold: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_3
{
  pub max_symbols: u16,
  pub num_keysyms: u16,
  pub keysyms: *mut xcb_keysym_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_4
{
  pub resolution: u32,
  pub min_value: i32,
  pub max_value: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_5
{
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_data_t__bindgen_ty_6
{
  pub percent: u8,
  pub pad2: [u8; 3usize],
  pub pitch: u16,
  pub duration: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_state_iterator_t
{
  pub data: *mut xcb_input_feedback_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_feedback_control_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_feedback_control_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_feedback_control_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_feedbacks: u16,
  pub pad0: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_kbd_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub key: xcb_input_key_code_t,
  pub auto_repeat_mode: u8,
  pub key_click_percent: i8,
  pub bell_percent: i8,
  pub bell_pitch: i16,
  pub bell_duration: i16,
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_kbd_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_kbd_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ptr_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub pad0: [u8; 2usize],
  pub num: i16,
  pub denom: i16,
  pub threshold: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_ptr_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_ptr_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_integer_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub int_to_display: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_integer_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_integer_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_string_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub pad0: [u8; 2usize],
  pub num_keysyms: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_string_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_string_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_bell_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub percent: i8,
  pub pad0: [u8; 3usize],
  pub pitch: i16,
  pub duration: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_bell_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_bell_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_led_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_led_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_led_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t
{
  pub keyboard: xcb_input_feedback_ctl_data_t__bindgen_ty_1,
  pub pointer: xcb_input_feedback_ctl_data_t__bindgen_ty_2,
  pub string: xcb_input_feedback_ctl_data_t__bindgen_ty_3,
  pub integer: xcb_input_feedback_ctl_data_t__bindgen_ty_4,
  pub led: xcb_input_feedback_ctl_data_t__bindgen_ty_5,
  pub bell: xcb_input_feedback_ctl_data_t__bindgen_ty_6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_1
{
  pub key: xcb_input_key_code_t,
  pub auto_repeat_mode: u8,
  pub key_click_percent: i8,
  pub bell_percent: i8,
  pub bell_pitch: i16,
  pub bell_duration: i16,
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_2
{
  pub pad0: [u8; 2usize],
  pub num: i16,
  pub denom: i16,
  pub threshold: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_3
{
  pub pad1: [u8; 2usize],
  pub num_keysyms: u16,
  pub keysyms: *mut xcb_keysym_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_4
{
  pub int_to_display: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_5
{
  pub led_mask: u32,
  pub led_values: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_data_t__bindgen_ty_6
{
  pub percent: i8,
  pub pad2: [u8; 3usize],
  pub pitch: i16,
  pub duration: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_t
{
  pub class_id: u8,
  pub feedback_id: u8,
  pub len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_feedback_ctl_iterator_t
{
  pub data: *mut xcb_input_feedback_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

impl xcb_input_change_feedback_control_mask_t
{
  pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_STRING : xcb_input_change_feedback_control_mask_t = xcb_input_change_feedback_control_mask_t :: XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT ;
}

impl xcb_input_change_feedback_control_mask_t
{
  pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_INTEGER : xcb_input_change_feedback_control_mask_t = xcb_input_change_feedback_control_mask_t :: XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT ;
}

impl xcb_input_change_feedback_control_mask_t
{
  pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_NUM : xcb_input_change_feedback_control_mask_t = xcb_input_change_feedback_control_mask_t :: XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT ;
}

impl xcb_input_change_feedback_control_mask_t
{
  pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_DENOM:
    xcb_input_change_feedback_control_mask_t =
    xcb_input_change_feedback_control_mask_t::XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PERCENT;
}

impl xcb_input_change_feedback_control_mask_t
{
  pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_THRESHOLD:
    xcb_input_change_feedback_control_mask_t =
    xcb_input_change_feedback_control_mask_t::XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PITCH;
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_change_feedback_control_mask_t
{
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT = 1,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PERCENT = 2,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PITCH = 4,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_DURATION = 8,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED = 16,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED_MODE = 32,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY = 64,
  XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_AUTO_REPEAT_MODE = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_feedback_control_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub mask: u32,
  pub device_id: u8,
  pub feedback_id: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_key_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_key_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub first_keycode: xcb_input_key_code_t,
  pub count: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_key_mapping_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub keysyms_per_keycode: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_key_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub first_keycode: xcb_input_key_code_t,
  pub keysyms_per_keycode: u8,
  pub keycode_count: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_modifier_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_modifier_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_modifier_mapping_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub keycodes_per_modifier: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_modifier_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_modifier_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub keycodes_per_modifier: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_modifier_mapping_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_button_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_button_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_button_mapping_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub map_size: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_button_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_button_mapping_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub map_size: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_button_mapping_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_state_t
{
  pub class_id: u8,
  pub len: u8,
  pub num_keys: u8,
  pub pad0: u8,
  pub keys: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_state_iterator_t
{
  pub data: *mut xcb_input_key_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_state_t
{
  pub class_id: u8,
  pub len: u8,
  pub num_buttons: u8,
  pub pad0: u8,
  pub buttons: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_state_iterator_t
{
  pub data: *mut xcb_input_button_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_valuator_state_mode_mask_t
{
  XCB_INPUT_VALUATOR_STATE_MODE_MASK_DEVICE_MODE_ABSOLUTE = 1,
  XCB_INPUT_VALUATOR_STATE_MODE_MASK_OUT_OF_PROXIMITY = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_state_t
{
  pub class_id: u8,
  pub len: u8,
  pub num_valuators: u8,
  pub mode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_state_iterator_t
{
  pub data: *mut xcb_input_valuator_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_data_t
{
  pub key: xcb_input_input_state_data_t__bindgen_ty_1,
  pub button: xcb_input_input_state_data_t__bindgen_ty_2,
  pub valuator: xcb_input_input_state_data_t__bindgen_ty_3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_data_t__bindgen_ty_1
{
  pub num_keys: u8,
  pub pad0: u8,
  pub keys: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_data_t__bindgen_ty_2
{
  pub num_buttons: u8,
  pub pad1: u8,
  pub buttons: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_data_t__bindgen_ty_3
{
  pub num_valuators: u8,
  pub mode: u8,
  pub valuators: *mut i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_t
{
  pub class_id: u8,
  pub len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_input_state_iterator_t
{
  pub data: *mut xcb_input_input_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_query_device_state_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_query_device_state_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_query_device_state_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_classes: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_bell_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub feedback_id: u8,
  pub feedback_class: u8,
  pub percent: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_valuators_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_valuators_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub first_valuator: u8,
  pub num_valuators: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_set_device_valuators_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_control_t
{
  XCB_INPUT_DEVICE_CONTROL_RESOLUTION = 1,
  XCB_INPUT_DEVICE_CONTROL_ABS_CALIB = 2,
  XCB_INPUT_DEVICE_CONTROL_CORE = 3,
  XCB_INPUT_DEVICE_CONTROL_ENABLE = 4,
  XCB_INPUT_DEVICE_CONTROL_ABS_AREA = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_resolution_state_t
{
  pub control_id: u16,
  pub len: u16,
  pub num_valuators: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_resolution_state_iterator_t
{
  pub data: *mut xcb_input_device_resolution_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_calib_state_t
{
  pub control_id: u16,
  pub len: u16,
  pub min_x: i32,
  pub max_x: i32,
  pub min_y: i32,
  pub max_y: i32,
  pub flip_x: u32,
  pub flip_y: u32,
  pub rotation: u32,
  pub button_threshold: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_calib_state_iterator_t
{
  pub data: *mut xcb_input_device_abs_calib_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_area_state_t
{
  pub control_id: u16,
  pub len: u16,
  pub offset_x: u32,
  pub offset_y: u32,
  pub width: u32,
  pub height: u32,
  pub screen: u32,
  pub following: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_area_state_iterator_t
{
  pub data: *mut xcb_input_device_abs_area_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_core_state_t
{
  pub control_id: u16,
  pub len: u16,
  pub status: u8,
  pub iscore: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_core_state_iterator_t
{
  pub data: *mut xcb_input_device_core_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_enable_state_t
{
  pub control_id: u16,
  pub len: u16,
  pub enable: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_enable_state_iterator_t
{
  pub data: *mut xcb_input_device_enable_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t
{
  pub resolution: xcb_input_device_state_data_t__bindgen_ty_1,
  pub abs_calib: xcb_input_device_state_data_t__bindgen_ty_2,
  pub core: xcb_input_device_state_data_t__bindgen_ty_3,
  pub enable: xcb_input_device_state_data_t__bindgen_ty_4,
  pub abs_area: xcb_input_device_state_data_t__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t__bindgen_ty_1
{
  pub num_valuators: u32,
  pub resolution_values: *mut u32,
  pub resolution_min: *mut u32,
  pub resolution_max: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t__bindgen_ty_2
{
  pub min_x: i32,
  pub max_x: i32,
  pub min_y: i32,
  pub max_y: i32,
  pub flip_x: u32,
  pub flip_y: u32,
  pub rotation: u32,
  pub button_threshold: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t__bindgen_ty_3
{
  pub status: u8,
  pub iscore: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t__bindgen_ty_4
{
  pub enable: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_data_t__bindgen_ty_5
{
  pub offset_x: u32,
  pub offset_y: u32,
  pub width: u32,
  pub height: u32,
  pub screen: u32,
  pub following: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_t
{
  pub control_id: u16,
  pub len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_iterator_t
{
  pub data: *mut xcb_input_device_state_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_control_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_control_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub control_id: u16,
  pub device_id: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_control_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_resolution_ctl_t
{
  pub control_id: u16,
  pub len: u16,
  pub first_valuator: u8,
  pub num_valuators: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_resolution_ctl_iterator_t
{
  pub data: *mut xcb_input_device_resolution_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_calib_ctl_t
{
  pub control_id: u16,
  pub len: u16,
  pub min_x: i32,
  pub max_x: i32,
  pub min_y: i32,
  pub max_y: i32,
  pub flip_x: u32,
  pub flip_y: u32,
  pub rotation: u32,
  pub button_threshold: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_calib_ctl_iterator_t
{
  pub data: *mut xcb_input_device_abs_calib_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_area_ctrl_t
{
  pub control_id: u16,
  pub len: u16,
  pub offset_x: u32,
  pub offset_y: u32,
  pub width: i32,
  pub height: i32,
  pub screen: i32,
  pub following: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_abs_area_ctrl_iterator_t
{
  pub data: *mut xcb_input_device_abs_area_ctrl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_core_ctrl_t
{
  pub control_id: u16,
  pub len: u16,
  pub status: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_core_ctrl_iterator_t
{
  pub data: *mut xcb_input_device_core_ctrl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_enable_ctrl_t
{
  pub control_id: u16,
  pub len: u16,
  pub enable: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_enable_ctrl_iterator_t
{
  pub data: *mut xcb_input_device_enable_ctrl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t
{
  pub resolution: xcb_input_device_ctl_data_t__bindgen_ty_1,
  pub abs_calib: xcb_input_device_ctl_data_t__bindgen_ty_2,
  pub core: xcb_input_device_ctl_data_t__bindgen_ty_3,
  pub enable: xcb_input_device_ctl_data_t__bindgen_ty_4,
  pub abs_area: xcb_input_device_ctl_data_t__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t__bindgen_ty_1
{
  pub first_valuator: u8,
  pub num_valuators: u8,
  pub pad0: [u8; 2usize],
  pub resolution_values: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t__bindgen_ty_2
{
  pub min_x: i32,
  pub max_x: i32,
  pub min_y: i32,
  pub max_y: i32,
  pub flip_x: u32,
  pub flip_y: u32,
  pub rotation: u32,
  pub button_threshold: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t__bindgen_ty_3
{
  pub status: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t__bindgen_ty_4
{
  pub enable: u8,
  pub pad2: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_data_t__bindgen_ty_5
{
  pub offset_x: u32,
  pub offset_y: u32,
  pub width: i32,
  pub height: i32,
  pub screen: i32,
  pub following: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_t
{
  pub control_id: u16,
  pub len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_ctl_iterator_t
{
  pub data: *mut xcb_input_device_ctl_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_control_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_control_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub control_id: u16,
  pub device_id: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_control_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_device_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_device_properties_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_list_device_properties_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_atoms: u16,
  pub pad0: [u8; 22usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_property_format_t
{
  XCB_INPUT_PROPERTY_FORMAT_8_BITS = 8,
  XCB_INPUT_PROPERTY_FORMAT_16_BITS = 16,
  XCB_INPUT_PROPERTY_FORMAT_32_BITS = 32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_property_items_t
{
  pub data8: *mut u8,
  pub data16: *mut u16,
  pub data32: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub device_id: u8,
  pub format: u8,
  pub mode: u8,
  pub pad0: u8,
  pub num_items: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_delete_device_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub property: xcb_atom_t,
  pub device_id: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub offset: u32,
  pub len: u32,
  pub device_id: u8,
  pub _delete: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_property_items_t
{
  pub data8: *mut u8,
  pub data16: *mut u16,
  pub data32: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_get_device_property_reply_t
{
  pub response_type: u8,
  pub xi_reply_type: u8,
  pub sequence: u16,
  pub length: u32,
  pub type_: xcb_atom_t,
  pub bytes_after: u32,
  pub num_items: u32,
  pub format: u8,
  pub device_id: u8,
  pub pad0: [u8; 10usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_t
{
  XCB_INPUT_DEVICE_ALL = 0,
  XCB_INPUT_DEVICE_ALL_MASTER = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_group_info_t
{
  pub base: u8,
  pub latched: u8,
  pub locked: u8,
  pub effective: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_group_info_iterator_t
{
  pub data: *mut xcb_input_group_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_modifier_info_t
{
  pub base: u32,
  pub latched: u32,
  pub locked: u32,
  pub effective: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_modifier_info_iterator_t
{
  pub data: *mut xcb_input_modifier_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_pointer_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_pointer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_pointer_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub child: xcb_window_t,
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub win_x: xcb_input_fp1616_t,
  pub win_y: xcb_input_fp1616_t,
  pub same_screen: u8,
  pub pad1: u8,
  pub buttons_len: u16,
  pub mods: xcb_input_modifier_info_t,
  pub group: xcb_input_group_info_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_warp_pointer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub src_win: xcb_window_t,
  pub dst_win: xcb_window_t,
  pub src_x: xcb_input_fp1616_t,
  pub src_y: xcb_input_fp1616_t,
  pub src_width: u16,
  pub src_height: u16,
  pub dst_x: xcb_input_fp1616_t,
  pub dst_y: xcb_input_fp1616_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_change_cursor_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub cursor: xcb_cursor_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_hierarchy_change_type_t
{
  XCB_INPUT_HIERARCHY_CHANGE_TYPE_ADD_MASTER = 1,
  XCB_INPUT_HIERARCHY_CHANGE_TYPE_REMOVE_MASTER = 2,
  XCB_INPUT_HIERARCHY_CHANGE_TYPE_ATTACH_SLAVE = 3,
  XCB_INPUT_HIERARCHY_CHANGE_TYPE_DETACH_SLAVE = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_change_mode_t
{
  XCB_INPUT_CHANGE_MODE_ATTACH = 1,
  XCB_INPUT_CHANGE_MODE_FLOAT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_add_master_t
{
  pub type_: u16,
  pub len: u16,
  pub name_len: u16,
  pub send_core: u8,
  pub enable: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_add_master_iterator_t
{
  pub data: *mut xcb_input_add_master_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_remove_master_t
{
  pub type_: u16,
  pub len: u16,
  pub deviceid: xcb_input_device_id_t,
  pub return_mode: u8,
  pub pad0: u8,
  pub return_pointer: xcb_input_device_id_t,
  pub return_keyboard: xcb_input_device_id_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_remove_master_iterator_t
{
  pub data: *mut xcb_input_remove_master_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_attach_slave_t
{
  pub type_: u16,
  pub len: u16,
  pub deviceid: xcb_input_device_id_t,
  pub master: xcb_input_device_id_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_attach_slave_iterator_t
{
  pub data: *mut xcb_input_attach_slave_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_detach_slave_t
{
  pub type_: u16,
  pub len: u16,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_detach_slave_iterator_t
{
  pub data: *mut xcb_input_detach_slave_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_data_t
{
  pub add_master: xcb_input_hierarchy_change_data_t__bindgen_ty_1,
  pub remove_master: xcb_input_hierarchy_change_data_t__bindgen_ty_2,
  pub attach_slave: xcb_input_hierarchy_change_data_t__bindgen_ty_3,
  pub detach_slave: xcb_input_hierarchy_change_data_t__bindgen_ty_4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_data_t__bindgen_ty_1
{
  pub name_len: u16,
  pub send_core: u8,
  pub enable: u8,
  pub name: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_data_t__bindgen_ty_2
{
  pub deviceid: xcb_input_device_id_t,
  pub return_mode: u8,
  pub pad1: u8,
  pub return_pointer: xcb_input_device_id_t,
  pub return_keyboard: xcb_input_device_id_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_data_t__bindgen_ty_3
{
  pub deviceid: xcb_input_device_id_t,
  pub master: xcb_input_device_id_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_data_t__bindgen_ty_4
{
  pub deviceid: xcb_input_device_id_t,
  pub pad2: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_t
{
  pub type_: u16,
  pub len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_change_iterator_t
{
  pub data: *mut xcb_input_hierarchy_change_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_change_hierarchy_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub num_changes: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_set_client_pointer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_client_pointer_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_client_pointer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_client_pointer_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub set: u8,
  pub pad1: u8,
  pub deviceid: xcb_input_device_id_t,
  pub pad2: [u8; 20usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_xi_event_mask_t
{
  XCB_INPUT_XI_EVENT_MASK_DEVICE_CHANGED = 2,
  XCB_INPUT_XI_EVENT_MASK_KEY_PRESS = 4,
  XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE = 8,
  XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS = 16,
  XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE = 32,
  XCB_INPUT_XI_EVENT_MASK_MOTION = 64,
  XCB_INPUT_XI_EVENT_MASK_ENTER = 128,
  XCB_INPUT_XI_EVENT_MASK_LEAVE = 256,
  XCB_INPUT_XI_EVENT_MASK_FOCUS_IN = 512,
  XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT = 1024,
  XCB_INPUT_XI_EVENT_MASK_HIERARCHY = 2048,
  XCB_INPUT_XI_EVENT_MASK_PROPERTY = 4096,
  XCB_INPUT_XI_EVENT_MASK_RAW_KEY_PRESS = 8192,
  XCB_INPUT_XI_EVENT_MASK_RAW_KEY_RELEASE = 16384,
  XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_PRESS = 32768,
  XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_RELEASE = 65536,
  XCB_INPUT_XI_EVENT_MASK_RAW_MOTION = 131072,
  XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN = 262144,
  XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE = 524288,
  XCB_INPUT_XI_EVENT_MASK_TOUCH_END = 1048576,
  XCB_INPUT_XI_EVENT_MASK_TOUCH_OWNERSHIP = 2097152,
  XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_BEGIN = 4194304,
  XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_UPDATE = 8388608,
  XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_END = 16777216,
  XCB_INPUT_XI_EVENT_MASK_BARRIER_HIT = 33554432,
  XCB_INPUT_XI_EVENT_MASK_BARRIER_LEAVE = 67108864,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_mask_t
{
  pub deviceid: xcb_input_device_id_t,
  pub mask_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_mask_iterator_t
{
  pub data: *mut xcb_input_event_mask_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_select_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub num_mask: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u16,
  pub minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u16,
  pub minor_version: u16,
  pub pad1: [u8; 20usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_class_type_t
{
  XCB_INPUT_DEVICE_CLASS_TYPE_KEY = 0,
  XCB_INPUT_DEVICE_CLASS_TYPE_BUTTON = 1,
  XCB_INPUT_DEVICE_CLASS_TYPE_VALUATOR = 2,
  XCB_INPUT_DEVICE_CLASS_TYPE_SCROLL = 3,
  XCB_INPUT_DEVICE_CLASS_TYPE_TOUCH = 8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_type_t
{
  XCB_INPUT_DEVICE_TYPE_MASTER_POINTER = 1,
  XCB_INPUT_DEVICE_TYPE_MASTER_KEYBOARD = 2,
  XCB_INPUT_DEVICE_TYPE_SLAVE_POINTER = 3,
  XCB_INPUT_DEVICE_TYPE_SLAVE_KEYBOARD = 4,
  XCB_INPUT_DEVICE_TYPE_FLOATING_SLAVE = 5,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_scroll_flags_t
{
  XCB_INPUT_SCROLL_FLAGS_NO_EMULATION = 1,
  XCB_INPUT_SCROLL_FLAGS_PREFERRED = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_scroll_type_t
{
  XCB_INPUT_SCROLL_TYPE_VERTICAL = 1,
  XCB_INPUT_SCROLL_TYPE_HORIZONTAL = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_touch_mode_t
{
  XCB_INPUT_TOUCH_MODE_DIRECT = 1,
  XCB_INPUT_TOUCH_MODE_DEPENDENT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub num_buttons: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_class_iterator_t
{
  pub data: *mut xcb_input_button_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub num_keys: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_class_iterator_t
{
  pub data: *mut xcb_input_key_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_scroll_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub number: u16,
  pub scroll_type: u16,
  pub pad0: [u8; 2usize],
  pub flags: u32,
  pub increment: xcb_input_fp3232_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_scroll_class_iterator_t
{
  pub data: *mut xcb_input_scroll_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_touch_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub mode: u8,
  pub num_touches: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_touch_class_iterator_t
{
  pub data: *mut xcb_input_touch_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub number: u16,
  pub label: xcb_atom_t,
  pub min: xcb_input_fp3232_t,
  pub max: xcb_input_fp3232_t,
  pub value: xcb_input_fp3232_t,
  pub resolution: u32,
  pub mode: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_valuator_class_iterator_t
{
  pub data: *mut xcb_input_valuator_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t
{
  pub key: xcb_input_device_class_data_t__bindgen_ty_1,
  pub button: xcb_input_device_class_data_t__bindgen_ty_2,
  pub valuator: xcb_input_device_class_data_t__bindgen_ty_3,
  pub scroll: xcb_input_device_class_data_t__bindgen_ty_4,
  pub touch: xcb_input_device_class_data_t__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t__bindgen_ty_1
{
  pub num_keys: u16,
  pub keys: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t__bindgen_ty_2
{
  pub num_buttons: u16,
  pub state: *mut u32,
  pub labels: *mut xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t__bindgen_ty_3
{
  pub number: u16,
  pub label: xcb_atom_t,
  pub min: xcb_input_fp3232_t,
  pub max: xcb_input_fp3232_t,
  pub value: xcb_input_fp3232_t,
  pub resolution: u32,
  pub mode: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t__bindgen_ty_4
{
  pub number: u16,
  pub scroll_type: u16,
  pub pad1: [u8; 2usize],
  pub flags: u32,
  pub increment: xcb_input_fp3232_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_data_t__bindgen_ty_5
{
  pub mode: u8,
  pub num_touches: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_t
{
  pub type_: u16,
  pub len: u16,
  pub sourceid: xcb_input_device_id_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_class_iterator_t
{
  pub data: *mut xcb_input_device_class_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_device_info_t
{
  pub deviceid: xcb_input_device_id_t,
  pub type_: u16,
  pub attachment: xcb_input_device_id_t,
  pub num_classes: u16,
  pub name_len: u16,
  pub enabled: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_device_info_iterator_t
{
  pub data: *mut xcb_input_xi_device_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_query_device_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_infos: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_set_focus_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_focus_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_focus_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_focus_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub focus: xcb_window_t,
  pub pad1: [u8; 20usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_grab_owner_t
{
  XCB_INPUT_GRAB_OWNER_NO_OWNER = 0,
  XCB_INPUT_GRAB_OWNER_OWNER = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_grab_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_grab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub cursor: xcb_cursor_t,
  pub deviceid: xcb_input_device_id_t,
  pub mode: u8,
  pub paired_device_mode: u8,
  pub owner_events: u8,
  pub pad0: u8,
  pub mask_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_grab_device_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub status: u8,
  pub pad1: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_ungrab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_event_mode_t
{
  XCB_INPUT_EVENT_MODE_ASYNC_DEVICE = 0,
  XCB_INPUT_EVENT_MODE_SYNC_DEVICE = 1,
  XCB_INPUT_EVENT_MODE_REPLAY_DEVICE = 2,
  XCB_INPUT_EVENT_MODE_ASYNC_PAIRED_DEVICE = 3,
  XCB_INPUT_EVENT_MODE_ASYNC_PAIR = 4,
  XCB_INPUT_EVENT_MODE_SYNC_PAIR = 5,
  XCB_INPUT_EVENT_MODE_ACCEPT_TOUCH = 6,
  XCB_INPUT_EVENT_MODE_REJECT_TOUCH = 7,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_allow_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
  pub deviceid: xcb_input_device_id_t,
  pub event_mode: u8,
  pub pad0: u8,
  pub touchid: u32,
  pub grab_window: xcb_window_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_grab_mode_22_t
{
  XCB_INPUT_GRAB_MODE_22_SYNC = 0,
  XCB_INPUT_GRAB_MODE_22_ASYNC = 1,
  XCB_INPUT_GRAB_MODE_22_TOUCH = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_grab_type_t
{
  XCB_INPUT_GRAB_TYPE_BUTTON = 0,
  XCB_INPUT_GRAB_TYPE_KEYCODE = 1,
  XCB_INPUT_GRAB_TYPE_ENTER = 2,
  XCB_INPUT_GRAB_TYPE_FOCUS_IN = 3,
  XCB_INPUT_GRAB_TYPE_TOUCH_BEGIN = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_modifier_mask_t
{
  XCB_INPUT_MODIFIER_MASK_ANY = 2147483648,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_modifier_info_t
{
  pub modifiers: u32,
  pub status: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_grab_modifier_info_iterator_t
{
  pub data: *mut xcb_input_grab_modifier_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_passive_grab_device_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_passive_grab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
  pub grab_window: xcb_window_t,
  pub cursor: xcb_cursor_t,
  pub detail: u32,
  pub deviceid: xcb_input_device_id_t,
  pub num_modifiers: u16,
  pub mask_len: u16,
  pub grab_type: u8,
  pub grab_mode: u8,
  pub paired_device_mode: u8,
  pub owner_events: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_passive_grab_device_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_modifiers: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_passive_ungrab_device_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub detail: u32,
  pub deviceid: xcb_input_device_id_t,
  pub num_modifiers: u16,
  pub grab_type: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_list_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_list_properties_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_list_properties_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_properties: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_change_property_items_t
{
  pub data8: *mut u8,
  pub data16: *mut u16,
  pub data32: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_change_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub mode: u8,
  pub format: u8,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub num_items: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_delete_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_property_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceid: xcb_input_device_id_t,
  pub _delete: u8,
  pub pad0: u8,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub offset: u32,
  pub len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_property_items_t
{
  pub data8: *mut u8,
  pub data16: *mut u16,
  pub data32: *mut u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_property_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub type_: xcb_atom_t,
  pub bytes_after: u32,
  pub num_items: u32,
  pub format: u8,
  pub pad1: [u8; 11usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_selected_events_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_selected_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_get_selected_events_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_masks: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_barrier_release_pointer_info_t
{
  pub deviceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub barrier: xcb_xfixes_barrier_t,
  pub eventid: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_barrier_release_pointer_info_iterator_t
{
  pub data: *mut xcb_input_barrier_release_pointer_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_xi_barrier_release_pointer_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub num_barriers: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_valuator_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub device_state: u16,
  pub num_valuators: u8,
  pub first_valuator: u8,
  pub valuators: [i32; 6usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_more_events_mask_t
{
  XCB_INPUT_MORE_EVENTS_MASK_MORE_EVENTS = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_key_press_event_t
{
  pub response_type: u8,
  pub detail: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub root_x: i16,
  pub root_y: i16,
  pub event_x: i16,
  pub event_y: i16,
  pub state: u16,
  pub same_screen: u8,
  pub device_id: u8,
}

pub type xcb_input_device_key_release_event_t = xcb_input_device_key_press_event_t;

pub type xcb_input_device_button_press_event_t = xcb_input_device_key_press_event_t;

pub type xcb_input_device_button_release_event_t = xcb_input_device_key_press_event_t;

pub type xcb_input_device_motion_notify_event_t = xcb_input_device_key_press_event_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_focus_in_event_t
{
  pub response_type: u8,
  pub detail: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub window: xcb_window_t,
  pub mode: u8,
  pub device_id: u8,
  pub pad0: [u8; 18usize],
}

pub type xcb_input_device_focus_out_event_t = xcb_input_device_focus_in_event_t;

pub type xcb_input_proximity_in_event_t = xcb_input_device_key_press_event_t;

pub type xcb_input_proximity_out_event_t = xcb_input_device_key_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_classes_reported_mask_t
{
  XCB_INPUT_CLASSES_REPORTED_MASK_OUT_OF_PROXIMITY = 128,
  XCB_INPUT_CLASSES_REPORTED_MASK_DEVICE_MODE_ABSOLUTE = 64,
  XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_VALUATORS = 4,
  XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_BUTTONS = 2,
  XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_KEYS = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_state_notify_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub num_keys: u8,
  pub num_buttons: u8,
  pub num_valuators: u8,
  pub classes_reported: u8,
  pub buttons: [u8; 4usize],
  pub keys: [u8; 4usize],
  pub valuators: [u32; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_mapping_notify_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub request: u8,
  pub first_keycode: xcb_input_key_code_t,
  pub count: u8,
  pub pad0: u8,
  pub time: xcb_timestamp_t,
  pub pad1: [u8; 20usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_change_device_t
{
  XCB_INPUT_CHANGE_DEVICE_NEW_POINTER = 0,
  XCB_INPUT_CHANGE_DEVICE_NEW_KEYBOARD = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_change_device_notify_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub request: u8,
  pub pad0: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_key_state_notify_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub keys: [u8; 28usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_button_state_notify_event_t
{
  pub response_type: u8,
  pub device_id: u8,
  pub sequence: u16,
  pub buttons: [u8; 28usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_device_change_t
{
  XCB_INPUT_DEVICE_CHANGE_ADDED = 0,
  XCB_INPUT_DEVICE_CHANGE_REMOVED = 1,
  XCB_INPUT_DEVICE_CHANGE_ENABLED = 2,
  XCB_INPUT_DEVICE_CHANGE_DISABLED = 3,
  XCB_INPUT_DEVICE_CHANGE_UNRECOVERABLE = 4,
  XCB_INPUT_DEVICE_CHANGE_CONTROL_CHANGED = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_presence_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub devchange: u8,
  pub device_id: u8,
  pub control: u16,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_property_notify_event_t
{
  pub response_type: u8,
  pub state: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub property: xcb_atom_t,
  pub pad0: [u8; 19usize],
  pub device_id: u8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_change_reason_t
{
  XCB_INPUT_CHANGE_REASON_SLAVE_SWITCH = 1,
  XCB_INPUT_CHANGE_REASON_DEVICE_CHANGE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_changed_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub num_classes: u16,
  pub sourceid: xcb_input_device_id_t,
  pub reason: u8,
  pub pad0: [u8; 11usize],
  pub full_sequence: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_key_event_flags_t
{
  XCB_INPUT_KEY_EVENT_FLAGS_KEY_REPEAT = 65536,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_key_press_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub full_sequence: u32,
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub event_x: xcb_input_fp1616_t,
  pub event_y: xcb_input_fp1616_t,
  pub buttons_len: u16,
  pub valuators_len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub flags: u32,
  pub mods: xcb_input_modifier_info_t,
  pub group: xcb_input_group_info_t,
}

pub type xcb_input_key_release_event_t = xcb_input_key_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_pointer_event_flags_t
{
  XCB_INPUT_POINTER_EVENT_FLAGS_POINTER_EMULATED = 65536,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_button_press_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub full_sequence: u32,
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub event_x: xcb_input_fp1616_t,
  pub event_y: xcb_input_fp1616_t,
  pub buttons_len: u16,
  pub valuators_len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub flags: u32,
  pub mods: xcb_input_modifier_info_t,
  pub group: xcb_input_group_info_t,
}

pub type xcb_input_button_release_event_t = xcb_input_button_press_event_t;

pub type xcb_input_motion_event_t = xcb_input_button_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_notify_mode_t
{
  XCB_INPUT_NOTIFY_MODE_NORMAL = 0,
  XCB_INPUT_NOTIFY_MODE_GRAB = 1,
  XCB_INPUT_NOTIFY_MODE_UNGRAB = 2,
  XCB_INPUT_NOTIFY_MODE_WHILE_GRABBED = 3,
  XCB_INPUT_NOTIFY_MODE_PASSIVE_GRAB = 4,
  XCB_INPUT_NOTIFY_MODE_PASSIVE_UNGRAB = 5,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_notify_detail_t
{
  XCB_INPUT_NOTIFY_DETAIL_ANCESTOR = 0,
  XCB_INPUT_NOTIFY_DETAIL_VIRTUAL = 1,
  XCB_INPUT_NOTIFY_DETAIL_INFERIOR = 2,
  XCB_INPUT_NOTIFY_DETAIL_NONLINEAR = 3,
  XCB_INPUT_NOTIFY_DETAIL_NONLINEAR_VIRTUAL = 4,
  XCB_INPUT_NOTIFY_DETAIL_POINTER = 5,
  XCB_INPUT_NOTIFY_DETAIL_POINTER_ROOT = 6,
  XCB_INPUT_NOTIFY_DETAIL_NONE = 7,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_enter_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub sourceid: xcb_input_device_id_t,
  pub mode: u8,
  pub detail: u8,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub full_sequence: u32,
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub event_x: xcb_input_fp1616_t,
  pub event_y: xcb_input_fp1616_t,
  pub same_screen: u8,
  pub focus: u8,
  pub buttons_len: u16,
  pub mods: xcb_input_modifier_info_t,
  pub group: xcb_input_group_info_t,
}

pub type xcb_input_leave_event_t = xcb_input_enter_event_t;

pub type xcb_input_focus_in_event_t = xcb_input_enter_event_t;

pub type xcb_input_focus_out_event_t = xcb_input_enter_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_hierarchy_mask_t
{
  XCB_INPUT_HIERARCHY_MASK_MASTER_ADDED = 1,
  XCB_INPUT_HIERARCHY_MASK_MASTER_REMOVED = 2,
  XCB_INPUT_HIERARCHY_MASK_SLAVE_ADDED = 4,
  XCB_INPUT_HIERARCHY_MASK_SLAVE_REMOVED = 8,
  XCB_INPUT_HIERARCHY_MASK_SLAVE_ATTACHED = 16,
  XCB_INPUT_HIERARCHY_MASK_SLAVE_DETACHED = 32,
  XCB_INPUT_HIERARCHY_MASK_DEVICE_ENABLED = 64,
  XCB_INPUT_HIERARCHY_MASK_DEVICE_DISABLED = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_info_t
{
  pub deviceid: xcb_input_device_id_t,
  pub attachment: xcb_input_device_id_t,
  pub type_: u8,
  pub enabled: u8,
  pub pad0: [u8; 2usize],
  pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_info_iterator_t
{
  pub data: *mut xcb_input_hierarchy_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_hierarchy_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub flags: u32,
  pub num_infos: u16,
  pub pad0: [u8; 10usize],
  pub full_sequence: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_property_flag_t
{
  XCB_INPUT_PROPERTY_FLAG_DELETED = 0,
  XCB_INPUT_PROPERTY_FLAG_CREATED = 1,
  XCB_INPUT_PROPERTY_FLAG_MODIFIED = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_property_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub property: xcb_atom_t,
  pub what: u8,
  pub pad0: [u8; 11usize],
  pub full_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_raw_key_press_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub sourceid: xcb_input_device_id_t,
  pub valuators_len: u16,
  pub flags: u32,
  pub pad0: [u8; 4usize],
  pub full_sequence: u32,
}

pub type xcb_input_raw_key_release_event_t = xcb_input_raw_key_press_event_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_raw_button_press_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub sourceid: xcb_input_device_id_t,
  pub valuators_len: u16,
  pub flags: u32,
  pub pad0: [u8; 4usize],
  pub full_sequence: u32,
}

pub type xcb_input_raw_button_release_event_t = xcb_input_raw_button_press_event_t;

pub type xcb_input_raw_motion_event_t = xcb_input_raw_button_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_touch_event_flags_t
{
  XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_PENDING_END = 65536,
  XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_EMULATING_POINTER = 131072,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_touch_begin_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub full_sequence: u32,
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub event_x: xcb_input_fp1616_t,
  pub event_y: xcb_input_fp1616_t,
  pub buttons_len: u16,
  pub valuators_len: u16,
  pub sourceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub flags: u32,
  pub mods: xcb_input_modifier_info_t,
  pub group: xcb_input_group_info_t,
}

pub type xcb_input_touch_update_event_t = xcb_input_touch_begin_event_t;

pub type xcb_input_touch_end_event_t = xcb_input_touch_begin_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_touch_ownership_flags_t
{
  XCB_INPUT_TOUCH_OWNERSHIP_FLAGS_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_touch_ownership_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub touchid: u32,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub child: xcb_window_t,
  pub full_sequence: u32,
  pub sourceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub flags: u32,
  pub pad1: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_raw_touch_begin_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub detail: u32,
  pub sourceid: xcb_input_device_id_t,
  pub valuators_len: u16,
  pub flags: u32,
  pub pad0: [u8; 4usize],
  pub full_sequence: u32,
}

pub type xcb_input_raw_touch_update_event_t = xcb_input_raw_touch_begin_event_t;

pub type xcb_input_raw_touch_end_event_t = xcb_input_raw_touch_begin_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_barrier_flags_t
{
  XCB_INPUT_BARRIER_FLAGS_POINTER_RELEASED = 1,
  XCB_INPUT_BARRIER_FLAGS_DEVICE_IS_GRABBED = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_barrier_hit_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub deviceid: xcb_input_device_id_t,
  pub time: xcb_timestamp_t,
  pub eventid: u32,
  pub root: xcb_window_t,
  pub event: xcb_window_t,
  pub barrier: xcb_xfixes_barrier_t,
  pub full_sequence: u32,
  pub dtime: u32,
  pub flags: u32,
  pub sourceid: xcb_input_device_id_t,
  pub pad0: [u8; 2usize],
  pub root_x: xcb_input_fp1616_t,
  pub root_y: xcb_input_fp1616_t,
  pub dx: xcb_input_fp3232_t,
  pub dy: xcb_input_fp3232_t,
}

pub type xcb_input_barrier_leave_event_t = xcb_input_barrier_hit_event_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_input_event_for_send_t
{
  pub device_valuator: xcb_input_device_valuator_event_t,
  pub device_key_press: xcb_input_device_key_press_event_t,
  pub device_key_release: xcb_input_device_key_release_event_t,
  pub device_button_press: xcb_input_device_button_press_event_t,
  pub device_button_release: xcb_input_device_button_release_event_t,
  pub device_motion_notify: xcb_input_device_motion_notify_event_t,
  pub device_focus_in: xcb_input_device_focus_in_event_t,
  pub device_focus_out: xcb_input_device_focus_out_event_t,
  pub proximity_in: xcb_input_proximity_in_event_t,
  pub proximity_out: xcb_input_proximity_out_event_t,
  pub device_state_notify: xcb_input_device_state_notify_event_t,
  pub device_mapping_notify: xcb_input_device_mapping_notify_event_t,
  pub change_device_notify: xcb_input_change_device_notify_event_t,
  pub device_key_state_notify: xcb_input_device_key_state_notify_event_t,
  pub device_button_state_notify: xcb_input_device_button_state_notify_event_t,
  pub device_presence_notify: xcb_input_device_presence_notify_event_t,
  pub event_header: xcb_raw_generic_event_t,
  _bindgen_union_align: [u32; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_for_send_iterator_t
{
  pub data: *mut xcb_input_event_for_send_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_send_extension_event_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub destination: xcb_window_t,
  pub device_id: u8,
  pub propagate: u8,
  pub num_classes: u16,
  pub num_events: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_event_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_mode_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_device_busy_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_input_class_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_input_id: xcb_extension_t;

  pub fn xcb_input_input_info_info(R: *const xcb_input_input_info_t)
    -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_feedback_state_data(
    R: *const xcb_input_feedback_state_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_feedback_ctl_data(
    R: *const xcb_input_feedback_ctl_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_input_state_data(
    R: *const xcb_input_input_state_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_device_state_data(
    R: *const xcb_input_device_state_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_device_ctl_data(R: *const xcb_input_device_ctl_t)
    -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_hierarchy_change_data(
    R: *const xcb_input_hierarchy_change_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_device_class_data(
    R: *const xcb_input_device_class_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_event_class_next(i: *mut xcb_input_event_class_iterator_t);

  pub fn xcb_input_event_class_end(i: xcb_input_event_class_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_key_code_next(i: *mut xcb_input_key_code_iterator_t);

  pub fn xcb_input_key_code_end(i: xcb_input_key_code_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_id_next(i: *mut xcb_input_device_id_iterator_t);

  pub fn xcb_input_device_id_end(i: xcb_input_device_id_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_fp1616_next(i: *mut xcb_input_fp1616_iterator_t);

  pub fn xcb_input_fp1616_end(i: xcb_input_fp1616_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_fp3232_next(i: *mut xcb_input_fp3232_iterator_t);

  pub fn xcb_input_fp3232_end(i: xcb_input_fp3232_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_extension_version_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_extension_version(
    c: *mut xcb_connection_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_input_get_extension_version_cookie_t;

  pub fn xcb_input_get_extension_version_unchecked(
    c: *mut xcb_connection_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_input_get_extension_version_cookie_t;

  pub fn xcb_input_get_extension_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_extension_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_extension_version_reply_t;

  pub fn xcb_input_device_info_next(i: *mut xcb_input_device_info_iterator_t);

  pub fn xcb_input_device_info_end(i: xcb_input_device_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_key_info_next(i: *mut xcb_input_key_info_iterator_t);

  pub fn xcb_input_key_info_end(i: xcb_input_key_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_info_next(i: *mut xcb_input_button_info_iterator_t);

  pub fn xcb_input_button_info_end(i: xcb_input_button_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_axis_info_next(i: *mut xcb_input_axis_info_iterator_t);

  pub fn xcb_input_axis_info_end(i: xcb_input_axis_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_valuator_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_valuator_info_axes(
    R: *const xcb_input_valuator_info_t
  ) -> *mut xcb_input_axis_info_t;

  pub fn xcb_input_valuator_info_axes_length(
    R: *const xcb_input_valuator_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_valuator_info_axes_iterator(
    R: *const xcb_input_valuator_info_t
  ) -> xcb_input_axis_info_iterator_t;

  pub fn xcb_input_valuator_info_next(i: *mut xcb_input_valuator_info_iterator_t);

  pub fn xcb_input_valuator_info_end(
    i: xcb_input_valuator_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_input_info_info_valuator_axes(
    S: *const xcb_input_input_info_info_t
  ) -> *mut xcb_input_axis_info_t;

  pub fn xcb_input_input_info_info_valuator_axes_length(
    R: *const xcb_input_input_info_t,
    S: *const xcb_input_input_info_info_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_info_info_valuator_axes_iterator(
    R: *const xcb_input_input_info_t,
    S: *const xcb_input_input_info_info_t,
  ) -> xcb_input_axis_info_iterator_t;

  pub fn xcb_input_input_info_info_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    class_id: u8,
    _aux: *const xcb_input_input_info_info_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_info_info_unpack(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
    _aux: *mut xcb_input_input_info_info_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_info_info_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_info_next(i: *mut xcb_input_input_info_iterator_t);

  pub fn xcb_input_input_info_end(i: xcb_input_input_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_name_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_name_string(
    R: *const xcb_input_device_name_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_input_device_name_string_length(
    R: *const xcb_input_device_name_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_name_string_end(
    R: *const xcb_input_device_name_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_name_next(i: *mut xcb_input_device_name_iterator_t);

  pub fn xcb_input_device_name_end(i: xcb_input_device_name_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_list_input_devices_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_input_devices(
    c: *mut xcb_connection_t
  ) -> xcb_input_list_input_devices_cookie_t;

  pub fn xcb_input_list_input_devices_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_input_list_input_devices_cookie_t;

  pub fn xcb_input_list_input_devices_devices(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> *mut xcb_input_device_info_t;

  pub fn xcb_input_list_input_devices_devices_length(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_input_devices_devices_iterator(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> xcb_input_device_info_iterator_t;

  pub fn xcb_input_list_input_devices_infos_length(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_input_devices_infos_iterator(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> xcb_input_input_info_iterator_t;

  pub fn xcb_input_list_input_devices_names_length(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_input_devices_names_iterator(
    R: *const xcb_input_list_input_devices_reply_t
  ) -> xcb_str_iterator_t;

  pub fn xcb_input_list_input_devices_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_list_input_devices_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_list_input_devices_reply_t;

  pub fn xcb_input_event_type_base_next(i: *mut xcb_input_event_type_base_iterator_t);

  pub fn xcb_input_event_type_base_end(
    i: xcb_input_event_type_base_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_input_class_info_next(i: *mut xcb_input_input_class_info_iterator_t);

  pub fn xcb_input_input_class_info_end(
    i: xcb_input_input_class_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_open_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_open_device(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_open_device_cookie_t;

  pub fn xcb_input_open_device_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_open_device_cookie_t;

  pub fn xcb_input_open_device_class_info(
    R: *const xcb_input_open_device_reply_t
  ) -> *mut xcb_input_input_class_info_t;

  pub fn xcb_input_open_device_class_info_length(
    R: *const xcb_input_open_device_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_open_device_class_info_iterator(
    R: *const xcb_input_open_device_reply_t
  ) -> xcb_input_input_class_info_iterator_t;

  pub fn xcb_input_open_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_open_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_open_device_reply_t;

  pub fn xcb_input_close_device_checked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_close_device(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_set_device_mode(
    c: *mut xcb_connection_t,
    device_id: u8,
    mode: u8,
  ) -> xcb_input_set_device_mode_cookie_t;

  pub fn xcb_input_set_device_mode_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
    mode: u8,
  ) -> xcb_input_set_device_mode_cookie_t;

  pub fn xcb_input_set_device_mode_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_set_device_mode_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_set_device_mode_reply_t;

  pub fn xcb_input_select_extension_event_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_select_extension_event_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_classes: u16,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_select_extension_event(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_classes: u16,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_select_extension_event_classes(
    R: *const xcb_input_select_extension_event_request_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_select_extension_event_classes_length(
    R: *const xcb_input_select_extension_event_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_select_extension_event_classes_end(
    R: *const xcb_input_select_extension_event_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_selected_extension_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_selected_extension_events(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_get_selected_extension_events_cookie_t;

  pub fn xcb_input_get_selected_extension_events_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_get_selected_extension_events_cookie_t;

  pub fn xcb_input_get_selected_extension_events_this_classes(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_get_selected_extension_events_this_classes_length(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_selected_extension_events_this_classes_end(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_selected_extension_events_all_classes(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_get_selected_extension_events_all_classes_length(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_selected_extension_events_all_classes_end(
    R: *const xcb_input_get_selected_extension_events_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_selected_extension_events_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_selected_extension_events_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_selected_extension_events_reply_t;

  pub fn xcb_input_change_device_dont_propagate_list_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_dont_propagate_list_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_classes: u16,
    mode: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_dont_propagate_list(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_classes: u16,
    mode: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_dont_propagate_list_classes(
    R: *const xcb_input_change_device_dont_propagate_list_request_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_change_device_dont_propagate_list_classes_length(
    R: *const xcb_input_change_device_dont_propagate_list_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_dont_propagate_list_classes_end(
    R: *const xcb_input_change_device_dont_propagate_list_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_dont_propagate_list_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_dont_propagate_list(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_get_device_dont_propagate_list_cookie_t;

  pub fn xcb_input_get_device_dont_propagate_list_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_get_device_dont_propagate_list_cookie_t;

  pub fn xcb_input_get_device_dont_propagate_list_classes(
    R: *const xcb_input_get_device_dont_propagate_list_reply_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_get_device_dont_propagate_list_classes_length(
    R: *const xcb_input_get_device_dont_propagate_list_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_dont_propagate_list_classes_end(
    R: *const xcb_input_get_device_dont_propagate_list_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_dont_propagate_list_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_dont_propagate_list_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_dont_propagate_list_reply_t;

  pub fn xcb_input_device_time_coord_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    num_axes: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_time_coord_axisvalues(
    R: *const xcb_input_device_time_coord_t
  ) -> *mut i32;

  pub fn xcb_input_device_time_coord_axisvalues_length(
    R: *const xcb_input_device_time_coord_t,
    num_axes: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_time_coord_axisvalues_end(
    R: *const xcb_input_device_time_coord_t,
    num_axes: u8,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_time_coord_next(i: *mut xcb_input_device_time_coord_iterator_t);

  pub fn xcb_input_device_time_coord_end(
    i: xcb_input_device_time_coord_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_motion_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_motion_events(
    c: *mut xcb_connection_t,
    start: xcb_timestamp_t,
    stop: xcb_timestamp_t,
    device_id: u8,
  ) -> xcb_input_get_device_motion_events_cookie_t;

  pub fn xcb_input_get_device_motion_events_unchecked(
    c: *mut xcb_connection_t,
    start: xcb_timestamp_t,
    stop: xcb_timestamp_t,
    device_id: u8,
  ) -> xcb_input_get_device_motion_events_cookie_t;

  pub fn xcb_input_get_device_motion_events_events_length(
    R: *const xcb_input_get_device_motion_events_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_motion_events_events_iterator(
    R: *const xcb_input_get_device_motion_events_reply_t
  ) -> xcb_input_device_time_coord_iterator_t;

  pub fn xcb_input_get_device_motion_events_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_motion_events_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_motion_events_reply_t;

  pub fn xcb_input_change_keyboard_device(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_change_keyboard_device_cookie_t;

  pub fn xcb_input_change_keyboard_device_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_change_keyboard_device_cookie_t;

  pub fn xcb_input_change_keyboard_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_change_keyboard_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_change_keyboard_device_reply_t;

  pub fn xcb_input_change_pointer_device(
    c: *mut xcb_connection_t,
    x_axis: u8,
    y_axis: u8,
    device_id: u8,
  ) -> xcb_input_change_pointer_device_cookie_t;

  pub fn xcb_input_change_pointer_device_unchecked(
    c: *mut xcb_connection_t,
    x_axis: u8,
    y_axis: u8,
    device_id: u8,
  ) -> xcb_input_change_pointer_device_cookie_t;

  pub fn xcb_input_change_pointer_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_change_pointer_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_change_pointer_device_reply_t;

  pub fn xcb_input_grab_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_grab_device(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    time: xcb_timestamp_t,
    num_classes: u16,
    this_device_mode: u8,
    other_device_mode: u8,
    owner_events: u8,
    device_id: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_input_grab_device_cookie_t;

  pub fn xcb_input_grab_device_unchecked(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    time: xcb_timestamp_t,
    num_classes: u16,
    this_device_mode: u8,
    other_device_mode: u8,
    owner_events: u8,
    device_id: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_input_grab_device_cookie_t;

  pub fn xcb_input_grab_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_grab_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_grab_device_reply_t;

  pub fn xcb_input_ungrab_device_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_ungrab_device(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_key_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_grab_device_key_checked(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    num_classes: u16,
    modifiers: u16,
    modifier_device: u8,
    grabbed_device: u8,
    key: u8,
    this_device_mode: u8,
    other_device_mode: u8,
    owner_events: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_key(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    num_classes: u16,
    modifiers: u16,
    modifier_device: u8,
    grabbed_device: u8,
    key: u8,
    this_device_mode: u8,
    other_device_mode: u8,
    owner_events: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_key_classes(
    R: *const xcb_input_grab_device_key_request_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_grab_device_key_classes_length(
    R: *const xcb_input_grab_device_key_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_grab_device_key_classes_end(
    R: *const xcb_input_grab_device_key_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_ungrab_device_key_checked(
    c: *mut xcb_connection_t,
    grabWindow: xcb_window_t,
    modifiers: u16,
    modifier_device: u8,
    key: u8,
    grabbed_device: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_ungrab_device_key(
    c: *mut xcb_connection_t,
    grabWindow: xcb_window_t,
    modifiers: u16,
    modifier_device: u8,
    key: u8,
    grabbed_device: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_button_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_grab_device_button_checked(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    grabbed_device: u8,
    modifier_device: u8,
    num_classes: u16,
    modifiers: u16,
    this_device_mode: u8,
    other_device_mode: u8,
    button: u8,
    owner_events: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_button(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    grabbed_device: u8,
    modifier_device: u8,
    num_classes: u16,
    modifiers: u16,
    this_device_mode: u8,
    other_device_mode: u8,
    button: u8,
    owner_events: u8,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_device_button_classes(
    R: *const xcb_input_grab_device_button_request_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_grab_device_button_classes_length(
    R: *const xcb_input_grab_device_button_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_grab_device_button_classes_end(
    R: *const xcb_input_grab_device_button_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_ungrab_device_button_checked(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    modifiers: u16,
    modifier_device: u8,
    button: u8,
    grabbed_device: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_ungrab_device_button(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    modifiers: u16,
    modifier_device: u8,
    button: u8,
    grabbed_device: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_allow_device_events_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    mode: u8,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_allow_device_events(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    mode: u8,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_get_device_focus(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_focus_cookie_t;

  pub fn xcb_input_get_device_focus_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_focus_cookie_t;

  pub fn xcb_input_get_device_focus_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_focus_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_focus_reply_t;

  pub fn xcb_input_set_device_focus_checked(
    c: *mut xcb_connection_t,
    focus: xcb_window_t,
    time: xcb_timestamp_t,
    revert_to: u8,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_set_device_focus(
    c: *mut xcb_connection_t,
    focus: xcb_window_t,
    time: xcb_timestamp_t,
    revert_to: u8,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_kbd_feedback_state_next(i: *mut xcb_input_kbd_feedback_state_iterator_t);

  pub fn xcb_input_kbd_feedback_state_end(
    i: xcb_input_kbd_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_ptr_feedback_state_next(i: *mut xcb_input_ptr_feedback_state_iterator_t);

  pub fn xcb_input_ptr_feedback_state_end(
    i: xcb_input_ptr_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_integer_feedback_state_next(i: *mut xcb_input_integer_feedback_state_iterator_t);

  pub fn xcb_input_integer_feedback_state_end(
    i: xcb_input_integer_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_string_feedback_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_string_feedback_state_keysyms(
    R: *const xcb_input_string_feedback_state_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_string_feedback_state_keysyms_length(
    R: *const xcb_input_string_feedback_state_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_string_feedback_state_keysyms_end(
    R: *const xcb_input_string_feedback_state_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_string_feedback_state_next(i: *mut xcb_input_string_feedback_state_iterator_t);

  pub fn xcb_input_string_feedback_state_end(
    i: xcb_input_string_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_bell_feedback_state_next(i: *mut xcb_input_bell_feedback_state_iterator_t);

  pub fn xcb_input_bell_feedback_state_end(
    i: xcb_input_bell_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_led_feedback_state_next(i: *mut xcb_input_led_feedback_state_iterator_t);

  pub fn xcb_input_led_feedback_state_end(
    i: xcb_input_led_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_feedback_state_data_string_keysyms(
    S: *const xcb_input_feedback_state_data_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_feedback_state_data_string_keysyms_length(
    R: *const xcb_input_feedback_state_t,
    S: *const xcb_input_feedback_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_state_data_string_keysyms_end(
    R: *const xcb_input_feedback_state_t,
    S: *const xcb_input_feedback_state_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_feedback_state_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    class_id: u8,
    _aux: *const xcb_input_feedback_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_state_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
    _aux: *mut xcb_input_feedback_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_state_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_state_next(i: *mut xcb_input_feedback_state_iterator_t);

  pub fn xcb_input_feedback_state_end(
    i: xcb_input_feedback_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_feedback_control_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_feedback_control(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_feedback_control_cookie_t;

  pub fn xcb_input_get_feedback_control_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_feedback_control_cookie_t;

  pub fn xcb_input_get_feedback_control_feedbacks_length(
    R: *const xcb_input_get_feedback_control_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_feedback_control_feedbacks_iterator(
    R: *const xcb_input_get_feedback_control_reply_t
  ) -> xcb_input_feedback_state_iterator_t;

  pub fn xcb_input_get_feedback_control_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_feedback_control_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_feedback_control_reply_t;

  pub fn xcb_input_kbd_feedback_ctl_next(i: *mut xcb_input_kbd_feedback_ctl_iterator_t);

  pub fn xcb_input_kbd_feedback_ctl_end(
    i: xcb_input_kbd_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_ptr_feedback_ctl_next(i: *mut xcb_input_ptr_feedback_ctl_iterator_t);

  pub fn xcb_input_ptr_feedback_ctl_end(
    i: xcb_input_ptr_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_integer_feedback_ctl_next(i: *mut xcb_input_integer_feedback_ctl_iterator_t);

  pub fn xcb_input_integer_feedback_ctl_end(
    i: xcb_input_integer_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_string_feedback_ctl_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_string_feedback_ctl_keysyms(
    R: *const xcb_input_string_feedback_ctl_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_string_feedback_ctl_keysyms_length(
    R: *const xcb_input_string_feedback_ctl_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_string_feedback_ctl_keysyms_end(
    R: *const xcb_input_string_feedback_ctl_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_string_feedback_ctl_next(i: *mut xcb_input_string_feedback_ctl_iterator_t);

  pub fn xcb_input_string_feedback_ctl_end(
    i: xcb_input_string_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_bell_feedback_ctl_next(i: *mut xcb_input_bell_feedback_ctl_iterator_t);

  pub fn xcb_input_bell_feedback_ctl_end(
    i: xcb_input_bell_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_led_feedback_ctl_next(i: *mut xcb_input_led_feedback_ctl_iterator_t);

  pub fn xcb_input_led_feedback_ctl_end(
    i: xcb_input_led_feedback_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_feedback_ctl_data_string_keysyms(
    S: *const xcb_input_feedback_ctl_data_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_feedback_ctl_data_string_keysyms_length(
    R: *const xcb_input_feedback_ctl_t,
    S: *const xcb_input_feedback_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_ctl_data_string_keysyms_end(
    R: *const xcb_input_feedback_ctl_t,
    S: *const xcb_input_feedback_ctl_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_feedback_ctl_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    class_id: u8,
    _aux: *const xcb_input_feedback_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_ctl_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
    _aux: *mut xcb_input_feedback_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_ctl_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_ctl_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_feedback_ctl_next(i: *mut xcb_input_feedback_ctl_iterator_t);

  pub fn xcb_input_feedback_ctl_end(i: xcb_input_feedback_ctl_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_change_feedback_control_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_feedback_control_checked(
    c: *mut xcb_connection_t,
    mask: u32,
    device_id: u8,
    feedback_id: u8,
    feedback: *mut xcb_input_feedback_ctl_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_feedback_control(
    c: *mut xcb_connection_t,
    mask: u32,
    device_id: u8,
    feedback_id: u8,
    feedback: *mut xcb_input_feedback_ctl_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_feedback_control_feedback(
    R: *const xcb_input_change_feedback_control_request_t
  ) -> *mut xcb_input_feedback_ctl_t;

  pub fn xcb_input_get_device_key_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_key_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_keycode: xcb_input_key_code_t,
    count: u8,
  ) -> xcb_input_get_device_key_mapping_cookie_t;

  pub fn xcb_input_get_device_key_mapping_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_keycode: xcb_input_key_code_t,
    count: u8,
  ) -> xcb_input_get_device_key_mapping_cookie_t;

  pub fn xcb_input_get_device_key_mapping_keysyms(
    R: *const xcb_input_get_device_key_mapping_reply_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_get_device_key_mapping_keysyms_length(
    R: *const xcb_input_get_device_key_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_key_mapping_keysyms_end(
    R: *const xcb_input_get_device_key_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_key_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_key_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_key_mapping_reply_t;

  pub fn xcb_input_change_device_key_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_key_mapping_checked(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_keycode: xcb_input_key_code_t,
    keysyms_per_keycode: u8,
    keycode_count: u8,
    keysyms: *const xcb_keysym_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_key_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_keycode: xcb_input_key_code_t,
    keysyms_per_keycode: u8,
    keycode_count: u8,
    keysyms: *const xcb_keysym_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_key_mapping_keysyms(
    R: *const xcb_input_change_device_key_mapping_request_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_input_change_device_key_mapping_keysyms_length(
    R: *const xcb_input_change_device_key_mapping_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_key_mapping_keysyms_end(
    R: *const xcb_input_change_device_key_mapping_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_modifier_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_modifier_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_modifier_mapping_cookie_t;

  pub fn xcb_input_get_device_modifier_mapping_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_modifier_mapping_cookie_t;

  pub fn xcb_input_get_device_modifier_mapping_keymaps(
    R: *const xcb_input_get_device_modifier_mapping_reply_t
  ) -> *mut u8;

  pub fn xcb_input_get_device_modifier_mapping_keymaps_length(
    R: *const xcb_input_get_device_modifier_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_modifier_mapping_keymaps_end(
    R: *const xcb_input_get_device_modifier_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_modifier_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_modifier_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_modifier_mapping_reply_t;

  pub fn xcb_input_set_device_modifier_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_set_device_modifier_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
    keycodes_per_modifier: u8,
    keymaps: *const u8,
  ) -> xcb_input_set_device_modifier_mapping_cookie_t;

  pub fn xcb_input_set_device_modifier_mapping_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
    keycodes_per_modifier: u8,
    keymaps: *const u8,
  ) -> xcb_input_set_device_modifier_mapping_cookie_t;

  pub fn xcb_input_set_device_modifier_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_set_device_modifier_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_set_device_modifier_mapping_reply_t;

  pub fn xcb_input_get_device_button_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_button_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_button_mapping_cookie_t;

  pub fn xcb_input_get_device_button_mapping_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_get_device_button_mapping_cookie_t;

  pub fn xcb_input_get_device_button_mapping_map(
    R: *const xcb_input_get_device_button_mapping_reply_t
  ) -> *mut u8;

  pub fn xcb_input_get_device_button_mapping_map_length(
    R: *const xcb_input_get_device_button_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_button_mapping_map_end(
    R: *const xcb_input_get_device_button_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_button_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_button_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_button_mapping_reply_t;

  pub fn xcb_input_set_device_button_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_set_device_button_mapping(
    c: *mut xcb_connection_t,
    device_id: u8,
    map_size: u8,
    map: *const u8,
  ) -> xcb_input_set_device_button_mapping_cookie_t;

  pub fn xcb_input_set_device_button_mapping_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
    map_size: u8,
    map: *const u8,
  ) -> xcb_input_set_device_button_mapping_cookie_t;

  pub fn xcb_input_set_device_button_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_set_device_button_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_set_device_button_mapping_reply_t;

  pub fn xcb_input_key_state_next(i: *mut xcb_input_key_state_iterator_t);

  pub fn xcb_input_key_state_end(i: xcb_input_key_state_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_state_next(i: *mut xcb_input_button_state_iterator_t);

  pub fn xcb_input_button_state_end(i: xcb_input_button_state_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_valuator_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_valuator_state_valuators(R: *const xcb_input_valuator_state_t) -> *mut i32;

  pub fn xcb_input_valuator_state_valuators_length(
    R: *const xcb_input_valuator_state_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_valuator_state_valuators_end(
    R: *const xcb_input_valuator_state_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_valuator_state_next(i: *mut xcb_input_valuator_state_iterator_t);

  pub fn xcb_input_valuator_state_end(
    i: xcb_input_valuator_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_input_state_data_valuator_valuators(
    S: *const xcb_input_input_state_data_t
  ) -> *mut i32;

  pub fn xcb_input_input_state_data_valuator_valuators_length(
    R: *const xcb_input_input_state_t,
    S: *const xcb_input_input_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_state_data_valuator_valuators_end(
    R: *const xcb_input_input_state_t,
    S: *const xcb_input_input_state_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_input_state_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    class_id: u8,
    _aux: *const xcb_input_input_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_state_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
    _aux: *mut xcb_input_input_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_state_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    class_id: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_input_state_next(i: *mut xcb_input_input_state_iterator_t);

  pub fn xcb_input_input_state_end(i: xcb_input_input_state_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_query_device_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_query_device_state(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_query_device_state_cookie_t;

  pub fn xcb_input_query_device_state_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_query_device_state_cookie_t;

  pub fn xcb_input_query_device_state_classes_length(
    R: *const xcb_input_query_device_state_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_query_device_state_classes_iterator(
    R: *const xcb_input_query_device_state_reply_t
  ) -> xcb_input_input_state_iterator_t;

  pub fn xcb_input_query_device_state_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_query_device_state_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_query_device_state_reply_t;

  pub fn xcb_input_device_bell_checked(
    c: *mut xcb_connection_t,
    device_id: u8,
    feedback_id: u8,
    feedback_class: u8,
    percent: i8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_device_bell(
    c: *mut xcb_connection_t,
    device_id: u8,
    feedback_id: u8,
    feedback_class: u8,
    percent: i8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_set_device_valuators_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_set_device_valuators(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_valuator: u8,
    num_valuators: u8,
    valuators: *const i32,
  ) -> xcb_input_set_device_valuators_cookie_t;

  pub fn xcb_input_set_device_valuators_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
    first_valuator: u8,
    num_valuators: u8,
    valuators: *const i32,
  ) -> xcb_input_set_device_valuators_cookie_t;

  pub fn xcb_input_set_device_valuators_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_set_device_valuators_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_set_device_valuators_reply_t;

  pub fn xcb_input_device_resolution_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_state_resolution_values(
    R: *const xcb_input_device_resolution_state_t
  ) -> *mut u32;

  pub fn xcb_input_device_resolution_state_resolution_values_length(
    R: *const xcb_input_device_resolution_state_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_state_resolution_values_end(
    R: *const xcb_input_device_resolution_state_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_resolution_state_resolution_min(
    R: *const xcb_input_device_resolution_state_t
  ) -> *mut u32;

  pub fn xcb_input_device_resolution_state_resolution_min_length(
    R: *const xcb_input_device_resolution_state_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_state_resolution_min_end(
    R: *const xcb_input_device_resolution_state_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_resolution_state_resolution_max(
    R: *const xcb_input_device_resolution_state_t
  ) -> *mut u32;

  pub fn xcb_input_device_resolution_state_resolution_max_length(
    R: *const xcb_input_device_resolution_state_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_state_resolution_max_end(
    R: *const xcb_input_device_resolution_state_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_resolution_state_next(
    i: *mut xcb_input_device_resolution_state_iterator_t
  );

  pub fn xcb_input_device_resolution_state_end(
    i: xcb_input_device_resolution_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_abs_calib_state_next(i: *mut xcb_input_device_abs_calib_state_iterator_t);

  pub fn xcb_input_device_abs_calib_state_end(
    i: xcb_input_device_abs_calib_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_abs_area_state_next(i: *mut xcb_input_device_abs_area_state_iterator_t);

  pub fn xcb_input_device_abs_area_state_end(
    i: xcb_input_device_abs_area_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_core_state_next(i: *mut xcb_input_device_core_state_iterator_t);

  pub fn xcb_input_device_core_state_end(
    i: xcb_input_device_core_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_enable_state_next(i: *mut xcb_input_device_enable_state_iterator_t);

  pub fn xcb_input_device_enable_state_end(
    i: xcb_input_device_enable_state_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_state_data_resolution_resolution_values(
    S: *const xcb_input_device_state_data_t
  ) -> *mut u32;

  pub fn xcb_input_device_state_data_resolution_resolution_values_length(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_data_resolution_resolution_values_end(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_state_data_resolution_resolution_min(
    S: *const xcb_input_device_state_data_t
  ) -> *mut u32;

  pub fn xcb_input_device_state_data_resolution_resolution_min_length(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_data_resolution_resolution_min_end(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_state_data_resolution_resolution_max(
    S: *const xcb_input_device_state_data_t
  ) -> *mut u32;

  pub fn xcb_input_device_state_data_resolution_resolution_max_length(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_data_resolution_resolution_max_end(
    R: *const xcb_input_device_state_t,
    S: *const xcb_input_device_state_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_state_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    control_id: u16,
    _aux: *const xcb_input_device_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    control_id: u16,
    _aux: *mut xcb_input_device_state_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    control_id: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_state_next(i: *mut xcb_input_device_state_iterator_t);

  pub fn xcb_input_device_state_end(i: xcb_input_device_state_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_control_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_control(
    c: *mut xcb_connection_t,
    control_id: u16,
    device_id: u8,
  ) -> xcb_input_get_device_control_cookie_t;

  pub fn xcb_input_get_device_control_unchecked(
    c: *mut xcb_connection_t,
    control_id: u16,
    device_id: u8,
  ) -> xcb_input_get_device_control_cookie_t;

  pub fn xcb_input_get_device_control_control(
    R: *const xcb_input_get_device_control_reply_t
  ) -> *mut xcb_input_device_state_t;

  pub fn xcb_input_get_device_control_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_control_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_control_reply_t;

  pub fn xcb_input_device_resolution_ctl_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_ctl_resolution_values(
    R: *const xcb_input_device_resolution_ctl_t
  ) -> *mut u32;

  pub fn xcb_input_device_resolution_ctl_resolution_values_length(
    R: *const xcb_input_device_resolution_ctl_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_resolution_ctl_resolution_values_end(
    R: *const xcb_input_device_resolution_ctl_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_resolution_ctl_next(i: *mut xcb_input_device_resolution_ctl_iterator_t);

  pub fn xcb_input_device_resolution_ctl_end(
    i: xcb_input_device_resolution_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_abs_calib_ctl_next(i: *mut xcb_input_device_abs_calib_ctl_iterator_t);

  pub fn xcb_input_device_abs_calib_ctl_end(
    i: xcb_input_device_abs_calib_ctl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_abs_area_ctrl_next(i: *mut xcb_input_device_abs_area_ctrl_iterator_t);

  pub fn xcb_input_device_abs_area_ctrl_end(
    i: xcb_input_device_abs_area_ctrl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_core_ctrl_next(i: *mut xcb_input_device_core_ctrl_iterator_t);

  pub fn xcb_input_device_core_ctrl_end(
    i: xcb_input_device_core_ctrl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_enable_ctrl_next(i: *mut xcb_input_device_enable_ctrl_iterator_t);

  pub fn xcb_input_device_enable_ctrl_end(
    i: xcb_input_device_enable_ctrl_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_ctl_data_resolution_resolution_values(
    S: *const xcb_input_device_ctl_data_t
  ) -> *mut u32;

  pub fn xcb_input_device_ctl_data_resolution_resolution_values_length(
    R: *const xcb_input_device_ctl_t,
    S: *const xcb_input_device_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_ctl_data_resolution_resolution_values_end(
    R: *const xcb_input_device_ctl_t,
    S: *const xcb_input_device_ctl_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_ctl_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    control_id: u16,
    _aux: *const xcb_input_device_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_ctl_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    control_id: u16,
    _aux: *mut xcb_input_device_ctl_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_ctl_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    control_id: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_ctl_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_ctl_next(i: *mut xcb_input_device_ctl_iterator_t);

  pub fn xcb_input_device_ctl_end(i: xcb_input_device_ctl_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_change_device_control_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_control(
    c: *mut xcb_connection_t,
    control_id: u16,
    device_id: u8,
    control: *mut xcb_input_device_ctl_t,
  ) -> xcb_input_change_device_control_cookie_t;

  pub fn xcb_input_change_device_control_unchecked(
    c: *mut xcb_connection_t,
    control_id: u16,
    device_id: u8,
    control: *mut xcb_input_device_ctl_t,
  ) -> xcb_input_change_device_control_cookie_t;

  pub fn xcb_input_change_device_control_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_change_device_control_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_change_device_control_reply_t;

  pub fn xcb_input_list_device_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_device_properties(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_list_device_properties_cookie_t;

  pub fn xcb_input_list_device_properties_unchecked(
    c: *mut xcb_connection_t,
    device_id: u8,
  ) -> xcb_input_list_device_properties_cookie_t;

  pub fn xcb_input_list_device_properties_atoms(
    R: *const xcb_input_list_device_properties_reply_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_input_list_device_properties_atoms_length(
    R: *const xcb_input_list_device_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_list_device_properties_atoms_end(
    R: *const xcb_input_list_device_properties_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_list_device_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_list_device_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_list_device_properties_reply_t;

  pub fn xcb_input_change_device_property_items_data_8(
    S: *const xcb_input_change_device_property_items_t
  ) -> *mut u8;

  pub fn xcb_input_change_device_property_items_data_8_length(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_items_data_8_end(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_change_device_property_items_data_16(
    S: *const xcb_input_change_device_property_items_t
  ) -> *mut u16;

  pub fn xcb_input_change_device_property_items_data_16_length(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_items_data_16_end(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_change_device_property_items_data_32(
    S: *const xcb_input_change_device_property_items_t
  ) -> *mut u32;

  pub fn xcb_input_change_device_property_items_data_32_length(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_items_data_32_end(
    R: *const xcb_input_change_device_property_request_t,
    S: *const xcb_input_change_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_change_device_property_items_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *const xcb_input_change_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_items_unpack(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *mut xcb_input_change_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_items_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_change_device_property_checked(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    device_id: u8,
    format: u8,
    mode: u8,
    num_items: u32,
    items: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_property(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    device_id: u8,
    format: u8,
    mode: u8,
    num_items: u32,
    items: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_property_aux_checked(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    device_id: u8,
    format: u8,
    mode: u8,
    num_items: u32,
    items: *const xcb_input_change_device_property_items_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_property_aux(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    device_id: u8,
    format: u8,
    mode: u8,
    num_items: u32,
    items: *const xcb_input_change_device_property_items_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_change_device_property_items(
    R: *const xcb_input_change_device_property_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_delete_device_property_checked(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_delete_device_property(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    device_id: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_get_device_property_items_data_8(
    S: *const xcb_input_get_device_property_items_t
  ) -> *mut u8;

  pub fn xcb_input_get_device_property_items_data_8_length(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_items_data_8_end(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_property_items_data_16(
    S: *const xcb_input_get_device_property_items_t
  ) -> *mut u16;

  pub fn xcb_input_get_device_property_items_data_16_length(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_items_data_16_end(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_property_items_data_32(
    S: *const xcb_input_get_device_property_items_t
  ) -> *mut u32;

  pub fn xcb_input_get_device_property_items_data_32_length(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_items_data_32_end(
    R: *const xcb_input_get_device_property_reply_t,
    S: *const xcb_input_get_device_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_get_device_property_items_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *const xcb_input_get_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_items_unpack(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *mut xcb_input_get_device_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_items_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_get_device_property(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    offset: u32,
    len: u32,
    device_id: u8,
    _delete: u8,
  ) -> xcb_input_get_device_property_cookie_t;

  pub fn xcb_input_get_device_property_unchecked(
    c: *mut xcb_connection_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    offset: u32,
    len: u32,
    device_id: u8,
    _delete: u8,
  ) -> xcb_input_get_device_property_cookie_t;

  pub fn xcb_input_get_device_property_items(
    R: *const xcb_input_get_device_property_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_get_device_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_get_device_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_get_device_property_reply_t;

  pub fn xcb_input_group_info_next(i: *mut xcb_input_group_info_iterator_t);

  pub fn xcb_input_group_info_end(i: xcb_input_group_info_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_modifier_info_next(i: *mut xcb_input_modifier_info_iterator_t);

  pub fn xcb_input_modifier_info_end(
    i: xcb_input_modifier_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_query_pointer_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_query_pointer(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_query_pointer_cookie_t;

  pub fn xcb_input_xi_query_pointer_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_query_pointer_cookie_t;

  pub fn xcb_input_xi_query_pointer_buttons(
    R: *const xcb_input_xi_query_pointer_reply_t
  ) -> *mut u32;

  pub fn xcb_input_xi_query_pointer_buttons_length(
    R: *const xcb_input_xi_query_pointer_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_query_pointer_buttons_end(
    R: *const xcb_input_xi_query_pointer_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_query_pointer_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_query_pointer_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_query_pointer_reply_t;

  pub fn xcb_input_xi_warp_pointer_checked(
    c: *mut xcb_connection_t,
    src_win: xcb_window_t,
    dst_win: xcb_window_t,
    src_x: xcb_input_fp1616_t,
    src_y: xcb_input_fp1616_t,
    src_width: u16,
    src_height: u16,
    dst_x: xcb_input_fp1616_t,
    dst_y: xcb_input_fp1616_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_warp_pointer(
    c: *mut xcb_connection_t,
    src_win: xcb_window_t,
    dst_win: xcb_window_t,
    src_x: xcb_input_fp1616_t,
    src_y: xcb_input_fp1616_t,
    src_width: u16,
    src_height: u16,
    dst_x: xcb_input_fp1616_t,
    dst_y: xcb_input_fp1616_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_cursor_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    cursor: xcb_cursor_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_cursor(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    cursor: xcb_cursor_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_add_master_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_add_master_name(R: *const xcb_input_add_master_t)
    -> *mut ::std::os::raw::c_char;

  pub fn xcb_input_add_master_name_length(
    R: *const xcb_input_add_master_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_add_master_name_end(R: *const xcb_input_add_master_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_add_master_next(i: *mut xcb_input_add_master_iterator_t);

  pub fn xcb_input_add_master_end(i: xcb_input_add_master_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_remove_master_next(i: *mut xcb_input_remove_master_iterator_t);

  pub fn xcb_input_remove_master_end(
    i: xcb_input_remove_master_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_attach_slave_next(i: *mut xcb_input_attach_slave_iterator_t);

  pub fn xcb_input_attach_slave_end(i: xcb_input_attach_slave_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_detach_slave_next(i: *mut xcb_input_detach_slave_iterator_t);

  pub fn xcb_input_detach_slave_end(i: xcb_input_detach_slave_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_hierarchy_change_data_add_master_name(
    S: *const xcb_input_hierarchy_change_data_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_input_hierarchy_change_data_add_master_name_length(
    R: *const xcb_input_hierarchy_change_t,
    S: *const xcb_input_hierarchy_change_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_change_data_add_master_name_end(
    R: *const xcb_input_hierarchy_change_t,
    S: *const xcb_input_hierarchy_change_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_hierarchy_change_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    type_: u16,
    _aux: *const xcb_input_hierarchy_change_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_change_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    type_: u16,
    _aux: *mut xcb_input_hierarchy_change_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_change_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    type_: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_change_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_change_next(i: *mut xcb_input_hierarchy_change_iterator_t);

  pub fn xcb_input_hierarchy_change_end(
    i: xcb_input_hierarchy_change_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_change_hierarchy_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_hierarchy_checked(
    c: *mut xcb_connection_t,
    num_changes: u8,
    changes: *const xcb_input_hierarchy_change_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_hierarchy(
    c: *mut xcb_connection_t,
    num_changes: u8,
    changes: *const xcb_input_hierarchy_change_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_hierarchy_changes_length(
    R: *const xcb_input_xi_change_hierarchy_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_hierarchy_changes_iterator(
    R: *const xcb_input_xi_change_hierarchy_request_t
  ) -> xcb_input_hierarchy_change_iterator_t;

  pub fn xcb_input_xi_set_client_pointer_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_set_client_pointer(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_get_client_pointer(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_xi_get_client_pointer_cookie_t;

  pub fn xcb_input_xi_get_client_pointer_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_xi_get_client_pointer_cookie_t;

  pub fn xcb_input_xi_get_client_pointer_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_get_client_pointer_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_get_client_pointer_reply_t;

  pub fn xcb_input_event_mask_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_event_mask_mask(R: *const xcb_input_event_mask_t) -> *mut u32;

  pub fn xcb_input_event_mask_mask_length(
    R: *const xcb_input_event_mask_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_event_mask_mask_end(R: *const xcb_input_event_mask_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_event_mask_next(i: *mut xcb_input_event_mask_iterator_t);

  pub fn xcb_input_event_mask_end(i: xcb_input_event_mask_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_select_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_select_events_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_mask: u16,
    masks: *const xcb_input_event_mask_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_select_events(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    num_mask: u16,
    masks: *const xcb_input_event_mask_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_select_events_masks_length(
    R: *const xcb_input_xi_select_events_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_select_events_masks_iterator(
    R: *const xcb_input_xi_select_events_request_t
  ) -> xcb_input_event_mask_iterator_t;

  pub fn xcb_input_xi_query_version(
    c: *mut xcb_connection_t,
    major_version: u16,
    minor_version: u16,
  ) -> xcb_input_xi_query_version_cookie_t;

  pub fn xcb_input_xi_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u16,
    minor_version: u16,
  ) -> xcb_input_xi_query_version_cookie_t;

  pub fn xcb_input_xi_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_query_version_reply_t;

  pub fn xcb_input_button_class_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_class_state(R: *const xcb_input_button_class_t) -> *mut u32;

  pub fn xcb_input_button_class_state_length(
    R: *const xcb_input_button_class_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_class_state_end(
    R: *const xcb_input_button_class_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_class_labels(R: *const xcb_input_button_class_t) -> *mut xcb_atom_t;

  pub fn xcb_input_button_class_labels_length(
    R: *const xcb_input_button_class_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_class_labels_end(
    R: *const xcb_input_button_class_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_class_next(i: *mut xcb_input_button_class_iterator_t);

  pub fn xcb_input_button_class_end(i: xcb_input_button_class_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_key_class_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_class_keys(R: *const xcb_input_key_class_t) -> *mut u32;

  pub fn xcb_input_key_class_keys_length(R: *const xcb_input_key_class_t) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_class_keys_end(R: *const xcb_input_key_class_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_key_class_next(i: *mut xcb_input_key_class_iterator_t);

  pub fn xcb_input_key_class_end(i: xcb_input_key_class_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_scroll_class_next(i: *mut xcb_input_scroll_class_iterator_t);

  pub fn xcb_input_scroll_class_end(i: xcb_input_scroll_class_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_touch_class_next(i: *mut xcb_input_touch_class_iterator_t);

  pub fn xcb_input_touch_class_end(i: xcb_input_touch_class_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_valuator_class_next(i: *mut xcb_input_valuator_class_iterator_t);

  pub fn xcb_input_valuator_class_end(
    i: xcb_input_valuator_class_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_class_data_key_keys(S: *const xcb_input_device_class_data_t) -> *mut u32;

  pub fn xcb_input_device_class_data_key_keys_length(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_data_key_keys_end(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_class_data_button_state(
    S: *const xcb_input_device_class_data_t
  ) -> *mut u32;

  pub fn xcb_input_device_class_data_button_state_length(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_data_button_state_end(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_class_data_button_labels(
    S: *const xcb_input_device_class_data_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_input_device_class_data_button_labels_length(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_data_button_labels_end(
    R: *const xcb_input_device_class_t,
    S: *const xcb_input_device_class_data_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_device_class_data_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    type_: u16,
    _aux: *const xcb_input_device_class_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_data_unpack(
    _buffer: *const ::std::os::raw::c_void,
    type_: u16,
    _aux: *mut xcb_input_device_class_data_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_data_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    type_: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_class_next(i: *mut xcb_input_device_class_iterator_t);

  pub fn xcb_input_device_class_end(i: xcb_input_device_class_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_device_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_device_info_name(
    R: *const xcb_input_xi_device_info_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_input_xi_device_info_name_length(
    R: *const xcb_input_xi_device_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_device_info_name_end(
    R: *const xcb_input_xi_device_info_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_device_info_classes_length(
    R: *const xcb_input_xi_device_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_device_info_classes_iterator(
    R: *const xcb_input_xi_device_info_t
  ) -> xcb_input_device_class_iterator_t;

  pub fn xcb_input_xi_device_info_next(i: *mut xcb_input_xi_device_info_iterator_t);

  pub fn xcb_input_xi_device_info_end(
    i: xcb_input_xi_device_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_query_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_query_device(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_query_device_cookie_t;

  pub fn xcb_input_xi_query_device_unchecked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_query_device_cookie_t;

  pub fn xcb_input_xi_query_device_infos_length(
    R: *const xcb_input_xi_query_device_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_query_device_infos_iterator(
    R: *const xcb_input_xi_query_device_reply_t
  ) -> xcb_input_xi_device_info_iterator_t;

  pub fn xcb_input_xi_query_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_query_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_query_device_reply_t;

  pub fn xcb_input_xi_set_focus_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_set_focus(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_get_focus(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_get_focus_cookie_t;

  pub fn xcb_input_xi_get_focus_unchecked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_get_focus_cookie_t;

  pub fn xcb_input_xi_get_focus_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_get_focus_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_get_focus_reply_t;

  pub fn xcb_input_xi_grab_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_grab_device(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    time: xcb_timestamp_t,
    cursor: xcb_cursor_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    paired_device_mode: u8,
    owner_events: u8,
    mask_len: u16,
    mask: *const u32,
  ) -> xcb_input_xi_grab_device_cookie_t;

  pub fn xcb_input_xi_grab_device_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    time: xcb_timestamp_t,
    cursor: xcb_cursor_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    paired_device_mode: u8,
    owner_events: u8,
    mask_len: u16,
    mask: *const u32,
  ) -> xcb_input_xi_grab_device_cookie_t;

  pub fn xcb_input_xi_grab_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_grab_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_grab_device_reply_t;

  pub fn xcb_input_xi_ungrab_device_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_ungrab_device(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_allow_events_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
    event_mode: u8,
    touchid: u32,
    grab_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_allow_events(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    deviceid: xcb_input_device_id_t,
    event_mode: u8,
    touchid: u32,
    grab_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_grab_modifier_info_next(i: *mut xcb_input_grab_modifier_info_iterator_t);

  pub fn xcb_input_grab_modifier_info_end(
    i: xcb_input_grab_modifier_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_passive_grab_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_passive_grab_device(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    grab_window: xcb_window_t,
    cursor: xcb_cursor_t,
    detail: u32,
    deviceid: xcb_input_device_id_t,
    num_modifiers: u16,
    mask_len: u16,
    grab_type: u8,
    grab_mode: u8,
    paired_device_mode: u8,
    owner_events: u8,
    mask: *const u32,
    modifiers: *const u32,
  ) -> xcb_input_xi_passive_grab_device_cookie_t;

  pub fn xcb_input_xi_passive_grab_device_unchecked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
    grab_window: xcb_window_t,
    cursor: xcb_cursor_t,
    detail: u32,
    deviceid: xcb_input_device_id_t,
    num_modifiers: u16,
    mask_len: u16,
    grab_type: u8,
    grab_mode: u8,
    paired_device_mode: u8,
    owner_events: u8,
    mask: *const u32,
    modifiers: *const u32,
  ) -> xcb_input_xi_passive_grab_device_cookie_t;

  pub fn xcb_input_xi_passive_grab_device_modifiers(
    R: *const xcb_input_xi_passive_grab_device_reply_t
  ) -> *mut xcb_input_grab_modifier_info_t;

  pub fn xcb_input_xi_passive_grab_device_modifiers_length(
    R: *const xcb_input_xi_passive_grab_device_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_passive_grab_device_modifiers_iterator(
    R: *const xcb_input_xi_passive_grab_device_reply_t
  ) -> xcb_input_grab_modifier_info_iterator_t;

  pub fn xcb_input_xi_passive_grab_device_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_passive_grab_device_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_passive_grab_device_reply_t;

  pub fn xcb_input_xi_passive_ungrab_device_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_passive_ungrab_device_checked(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    detail: u32,
    deviceid: xcb_input_device_id_t,
    num_modifiers: u16,
    grab_type: u8,
    modifiers: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_passive_ungrab_device(
    c: *mut xcb_connection_t,
    grab_window: xcb_window_t,
    detail: u32,
    deviceid: xcb_input_device_id_t,
    num_modifiers: u16,
    grab_type: u8,
    modifiers: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_passive_ungrab_device_modifiers(
    R: *const xcb_input_xi_passive_ungrab_device_request_t
  ) -> *mut u32;

  pub fn xcb_input_xi_passive_ungrab_device_modifiers_length(
    R: *const xcb_input_xi_passive_ungrab_device_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_passive_ungrab_device_modifiers_end(
    R: *const xcb_input_xi_passive_ungrab_device_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_list_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_list_properties(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_list_properties_cookie_t;

  pub fn xcb_input_xi_list_properties_unchecked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
  ) -> xcb_input_xi_list_properties_cookie_t;

  pub fn xcb_input_xi_list_properties_properties(
    R: *const xcb_input_xi_list_properties_reply_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_input_xi_list_properties_properties_length(
    R: *const xcb_input_xi_list_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_list_properties_properties_end(
    R: *const xcb_input_xi_list_properties_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_list_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_list_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_list_properties_reply_t;

  pub fn xcb_input_xi_change_property_items_data_8(
    S: *const xcb_input_xi_change_property_items_t
  ) -> *mut u8;

  pub fn xcb_input_xi_change_property_items_data_8_length(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_items_data_8_end(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_change_property_items_data_16(
    S: *const xcb_input_xi_change_property_items_t
  ) -> *mut u16;

  pub fn xcb_input_xi_change_property_items_data_16_length(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_items_data_16_end(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_change_property_items_data_32(
    S: *const xcb_input_xi_change_property_items_t
  ) -> *mut u32;

  pub fn xcb_input_xi_change_property_items_data_32_length(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_items_data_32_end(
    R: *const xcb_input_xi_change_property_request_t,
    S: *const xcb_input_xi_change_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_change_property_items_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *const xcb_input_xi_change_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_items_unpack(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *mut xcb_input_xi_change_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_items_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_change_property_checked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    format: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    num_items: u32,
    items: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_property(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    format: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    num_items: u32,
    items: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_property_aux_checked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    format: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    num_items: u32,
    items: *const xcb_input_xi_change_property_items_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_property_aux(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    mode: u8,
    format: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    num_items: u32,
    items: *const xcb_input_xi_change_property_items_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_change_property_items(
    R: *const xcb_input_xi_change_property_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_xi_delete_property_checked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_delete_property(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_get_property_items_data_8(
    S: *const xcb_input_xi_get_property_items_t
  ) -> *mut u8;

  pub fn xcb_input_xi_get_property_items_data_8_length(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_items_data_8_end(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_get_property_items_data_16(
    S: *const xcb_input_xi_get_property_items_t
  ) -> *mut u16;

  pub fn xcb_input_xi_get_property_items_data_16_length(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_items_data_16_end(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_get_property_items_data_32(
    S: *const xcb_input_xi_get_property_items_t
  ) -> *mut u32;

  pub fn xcb_input_xi_get_property_items_data_32_length(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_items_data_32_end(
    R: *const xcb_input_xi_get_property_reply_t,
    S: *const xcb_input_xi_get_property_items_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_get_property_items_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *const xcb_input_xi_get_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_items_unpack(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
    _aux: *mut xcb_input_xi_get_property_items_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_items_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    num_items: u32,
    format: u8,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_property(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    _delete: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    offset: u32,
    len: u32,
  ) -> xcb_input_xi_get_property_cookie_t;

  pub fn xcb_input_xi_get_property_unchecked(
    c: *mut xcb_connection_t,
    deviceid: xcb_input_device_id_t,
    _delete: u8,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    offset: u32,
    len: u32,
  ) -> xcb_input_xi_get_property_cookie_t;

  pub fn xcb_input_xi_get_property_items(
    R: *const xcb_input_xi_get_property_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_input_xi_get_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_get_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_get_property_reply_t;

  pub fn xcb_input_xi_get_selected_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_selected_events(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_xi_get_selected_events_cookie_t;

  pub fn xcb_input_xi_get_selected_events_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_input_xi_get_selected_events_cookie_t;

  pub fn xcb_input_xi_get_selected_events_masks_length(
    R: *const xcb_input_xi_get_selected_events_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_get_selected_events_masks_iterator(
    R: *const xcb_input_xi_get_selected_events_reply_t
  ) -> xcb_input_event_mask_iterator_t;

  pub fn xcb_input_xi_get_selected_events_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_input_xi_get_selected_events_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_input_xi_get_selected_events_reply_t;

  pub fn xcb_input_barrier_release_pointer_info_next(
    i: *mut xcb_input_barrier_release_pointer_info_iterator_t
  );

  pub fn xcb_input_barrier_release_pointer_info_end(
    i: xcb_input_barrier_release_pointer_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_xi_barrier_release_pointer_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_barrier_release_pointer_checked(
    c: *mut xcb_connection_t,
    num_barriers: u32,
    barriers: *const xcb_input_barrier_release_pointer_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_barrier_release_pointer(
    c: *mut xcb_connection_t,
    num_barriers: u32,
    barriers: *const xcb_input_barrier_release_pointer_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_xi_barrier_release_pointer_barriers(
    R: *const xcb_input_xi_barrier_release_pointer_request_t
  ) -> *mut xcb_input_barrier_release_pointer_info_t;

  pub fn xcb_input_xi_barrier_release_pointer_barriers_length(
    R: *const xcb_input_xi_barrier_release_pointer_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_xi_barrier_release_pointer_barriers_iterator(
    R: *const xcb_input_xi_barrier_release_pointer_request_t
  ) -> xcb_input_barrier_release_pointer_info_iterator_t;

  pub fn xcb_input_device_changed_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_changed_classes_length(
    R: *const xcb_input_device_changed_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_device_changed_classes_iterator(
    R: *const xcb_input_device_changed_event_t
  ) -> xcb_input_device_class_iterator_t;

  pub fn xcb_input_key_press_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_press_button_mask(R: *const xcb_input_key_press_event_t) -> *mut u32;

  pub fn xcb_input_key_press_button_mask_length(
    R: *const xcb_input_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_press_button_mask_end(
    R: *const xcb_input_key_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_key_press_valuator_mask(R: *const xcb_input_key_press_event_t) -> *mut u32;

  pub fn xcb_input_key_press_valuator_mask_length(
    R: *const xcb_input_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_press_valuator_mask_end(
    R: *const xcb_input_key_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_key_press_axisvalues(
    R: *const xcb_input_key_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_key_press_axisvalues_length(
    R: *const xcb_input_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_key_press_axisvalues_iterator(
    R: *const xcb_input_key_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_key_release_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_press_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_press_button_mask(R: *const xcb_input_button_press_event_t) -> *mut u32;

  pub fn xcb_input_button_press_button_mask_length(
    R: *const xcb_input_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_press_button_mask_end(
    R: *const xcb_input_button_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_press_valuator_mask(R: *const xcb_input_button_press_event_t)
    -> *mut u32;

  pub fn xcb_input_button_press_valuator_mask_length(
    R: *const xcb_input_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_press_valuator_mask_end(
    R: *const xcb_input_button_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_button_press_axisvalues(
    R: *const xcb_input_button_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_button_press_axisvalues_length(
    R: *const xcb_input_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_button_press_axisvalues_iterator(
    R: *const xcb_input_button_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_button_release_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_motion_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_input_enter_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_input_enter_buttons(R: *const xcb_input_enter_event_t) -> *mut u32;

  pub fn xcb_input_enter_buttons_length(R: *const xcb_input_enter_event_t)
    -> ::std::os::raw::c_int;

  pub fn xcb_input_enter_buttons_end(R: *const xcb_input_enter_event_t) -> xcb_generic_iterator_t;

  pub fn xcb_input_leave_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_input_focus_in_sizeof(_buffer: *const ::std::os::raw::c_void)
    -> ::std::os::raw::c_int;

  pub fn xcb_input_focus_out_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_info_next(i: *mut xcb_input_hierarchy_info_iterator_t);

  pub fn xcb_input_hierarchy_info_end(
    i: xcb_input_hierarchy_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_hierarchy_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_infos(
    R: *const xcb_input_hierarchy_event_t
  ) -> *mut xcb_input_hierarchy_info_t;

  pub fn xcb_input_hierarchy_infos_length(
    R: *const xcb_input_hierarchy_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_hierarchy_infos_iterator(
    R: *const xcb_input_hierarchy_event_t
  ) -> xcb_input_hierarchy_info_iterator_t;

  pub fn xcb_input_raw_key_press_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_key_press_valuator_mask(
    R: *const xcb_input_raw_key_press_event_t
  ) -> *mut u32;

  pub fn xcb_input_raw_key_press_valuator_mask_length(
    R: *const xcb_input_raw_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_key_press_valuator_mask_end(
    R: *const xcb_input_raw_key_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_raw_key_press_axisvalues(
    R: *const xcb_input_raw_key_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_key_press_axisvalues_length(
    R: *const xcb_input_raw_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_key_press_axisvalues_iterator(
    R: *const xcb_input_raw_key_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_key_press_axisvalues_raw(
    R: *const xcb_input_raw_key_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_key_press_axisvalues_raw_length(
    R: *const xcb_input_raw_key_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_key_press_axisvalues_raw_iterator(
    R: *const xcb_input_raw_key_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_key_release_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_button_press_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_button_press_valuator_mask(
    R: *const xcb_input_raw_button_press_event_t
  ) -> *mut u32;

  pub fn xcb_input_raw_button_press_valuator_mask_length(
    R: *const xcb_input_raw_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_button_press_valuator_mask_end(
    R: *const xcb_input_raw_button_press_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_raw_button_press_axisvalues(
    R: *const xcb_input_raw_button_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_button_press_axisvalues_length(
    R: *const xcb_input_raw_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_button_press_axisvalues_iterator(
    R: *const xcb_input_raw_button_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_button_press_axisvalues_raw(
    R: *const xcb_input_raw_button_press_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_button_press_axisvalues_raw_length(
    R: *const xcb_input_raw_button_press_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_button_press_axisvalues_raw_iterator(
    R: *const xcb_input_raw_button_press_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_button_release_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_motion_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_begin_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_begin_button_mask(R: *const xcb_input_touch_begin_event_t) -> *mut u32;

  pub fn xcb_input_touch_begin_button_mask_length(
    R: *const xcb_input_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_begin_button_mask_end(
    R: *const xcb_input_touch_begin_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_touch_begin_valuator_mask(R: *const xcb_input_touch_begin_event_t) -> *mut u32;

  pub fn xcb_input_touch_begin_valuator_mask_length(
    R: *const xcb_input_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_begin_valuator_mask_end(
    R: *const xcb_input_touch_begin_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_touch_begin_axisvalues(
    R: *const xcb_input_touch_begin_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_touch_begin_axisvalues_length(
    R: *const xcb_input_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_begin_axisvalues_iterator(
    R: *const xcb_input_touch_begin_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_touch_update_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_touch_end_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_begin_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_begin_valuator_mask(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> *mut u32;

  pub fn xcb_input_raw_touch_begin_valuator_mask_length(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_begin_valuator_mask_end(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_raw_touch_begin_axisvalues(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_touch_begin_axisvalues_length(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_begin_axisvalues_iterator(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_touch_begin_axisvalues_raw(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> *mut xcb_input_fp3232_t;

  pub fn xcb_input_raw_touch_begin_axisvalues_raw_length(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_begin_axisvalues_raw_iterator(
    R: *const xcb_input_raw_touch_begin_event_t
  ) -> xcb_input_fp3232_iterator_t;

  pub fn xcb_input_raw_touch_update_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_raw_touch_end_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_event_for_send_next(i: *mut xcb_input_event_for_send_iterator_t);

  pub fn xcb_input_event_for_send_end(
    i: xcb_input_event_for_send_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_input_send_extension_event_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_send_extension_event_checked(
    c: *mut xcb_connection_t,
    destination: xcb_window_t,
    device_id: u8,
    propagate: u8,
    num_classes: u16,
    num_events: u8,
    events: *const xcb_input_event_for_send_t,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_send_extension_event(
    c: *mut xcb_connection_t,
    destination: xcb_window_t,
    device_id: u8,
    propagate: u8,
    num_classes: u16,
    num_events: u8,
    events: *const xcb_input_event_for_send_t,
    classes: *const xcb_input_event_class_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_input_send_extension_event_events(
    R: *const xcb_input_send_extension_event_request_t
  ) -> *mut xcb_input_event_for_send_t;

  pub fn xcb_input_send_extension_event_events_length(
    R: *const xcb_input_send_extension_event_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_send_extension_event_events_iterator(
    R: *const xcb_input_send_extension_event_request_t
  ) -> xcb_input_event_for_send_iterator_t;

  pub fn xcb_input_send_extension_event_classes(
    R: *const xcb_input_send_extension_event_request_t
  ) -> *mut xcb_input_event_class_t;

  pub fn xcb_input_send_extension_event_classes_length(
    R: *const xcb_input_send_extension_event_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_input_send_extension_event_classes_end(
    R: *const xcb_input_send_extension_event_request_t
  ) -> xcb_generic_iterator_t;
}
