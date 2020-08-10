use super::core::{
  xcb_connection_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};

pub const XCB_KEY_PRESS: u32 = 2;
pub const XCB_KEY_RELEASE: u32 = 3;
pub const XCB_BUTTON_PRESS: u32 = 4;
pub const XCB_BUTTON_RELEASE: u32 = 5;
pub const XCB_MOTION_NOTIFY: u32 = 6;
pub const XCB_ENTER_NOTIFY: u32 = 7;
pub const XCB_LEAVE_NOTIFY: u32 = 8;
pub const XCB_FOCUS_IN: u32 = 9;
pub const XCB_FOCUS_OUT: u32 = 10;
pub const XCB_KEYMAP_NOTIFY: u32 = 11;
pub const XCB_EXPOSE: u32 = 12;
pub const XCB_GRAPHICS_EXPOSURE: u32 = 13;
pub const XCB_NO_EXPOSURE: u32 = 14;
pub const XCB_VISIBILITY_NOTIFY: u32 = 15;
pub const XCB_CREATE_NOTIFY: u32 = 16;
pub const XCB_DESTROY_NOTIFY: u32 = 17;
pub const XCB_UNMAP_NOTIFY: u32 = 18;
pub const XCB_MAP_NOTIFY: u32 = 19;
pub const XCB_MAP_REQUEST: u32 = 20;
pub const XCB_REPARENT_NOTIFY: u32 = 21;
pub const XCB_CONFIGURE_NOTIFY: u32 = 22;
pub const XCB_CONFIGURE_REQUEST: u32 = 23;
pub const XCB_GRAVITY_NOTIFY: u32 = 24;
pub const XCB_RESIZE_REQUEST: u32 = 25;
pub const XCB_CIRCULATE_NOTIFY: u32 = 26;
pub const XCB_CIRCULATE_REQUEST: u32 = 27;
pub const XCB_PROPERTY_NOTIFY: u32 = 28;
pub const XCB_SELECTION_CLEAR: u32 = 29;
pub const XCB_SELECTION_REQUEST: u32 = 30;
pub const XCB_SELECTION_NOTIFY: u32 = 31;
pub const XCB_COLORMAP_NOTIFY: u32 = 32;
pub const XCB_CLIENT_MESSAGE: u32 = 33;
pub const XCB_MAPPING_NOTIFY: u32 = 34;
pub const XCB_GE_GENERIC: u32 = 35;
pub const XCB_REQUEST: u32 = 1;
pub const XCB_VALUE: u32 = 2;
pub const XCB_WINDOW: u32 = 3;
pub const XCB_PIXMAP: u32 = 4;
pub const XCB_ATOM: u32 = 5;
pub const XCB_CURSOR: u32 = 6;
pub const XCB_FONT: u32 = 7;
pub const XCB_MATCH: u32 = 8;
pub const XCB_DRAWABLE: u32 = 9;
pub const XCB_ACCESS: u32 = 10;
pub const XCB_ALLOC: u32 = 11;
pub const XCB_COLORMAP: u32 = 12;
pub const XCB_G_CONTEXT: u32 = 13;
pub const XCB_ID_CHOICE: u32 = 14;
pub const XCB_NAME: u32 = 15;
pub const XCB_LENGTH: u32 = 16;
pub const XCB_IMPLEMENTATION: u32 = 17;
pub const XCB_CREATE_WINDOW: u32 = 1;
pub const XCB_CHANGE_WINDOW_ATTRIBUTES: u32 = 2;
pub const XCB_GET_WINDOW_ATTRIBUTES: u32 = 3;
pub const XCB_DESTROY_WINDOW: u32 = 4;
pub const XCB_DESTROY_SUBWINDOWS: u32 = 5;
pub const XCB_CHANGE_SAVE_SET: u32 = 6;
pub const XCB_REPARENT_WINDOW: u32 = 7;
pub const XCB_MAP_WINDOW: u32 = 8;
pub const XCB_MAP_SUBWINDOWS: u32 = 9;
pub const XCB_UNMAP_WINDOW: u32 = 10;
pub const XCB_UNMAP_SUBWINDOWS: u32 = 11;
pub const XCB_CONFIGURE_WINDOW: u32 = 12;
pub const XCB_CIRCULATE_WINDOW: u32 = 13;
pub const XCB_GET_GEOMETRY: u32 = 14;
pub const XCB_QUERY_TREE: u32 = 15;
pub const XCB_INTERN_ATOM: u32 = 16;
pub const XCB_GET_ATOM_NAME: u32 = 17;
pub const XCB_CHANGE_PROPERTY: u32 = 18;
pub const XCB_DELETE_PROPERTY: u32 = 19;
pub const XCB_GET_PROPERTY: u32 = 20;
pub const XCB_LIST_PROPERTIES: u32 = 21;
pub const XCB_SET_SELECTION_OWNER: u32 = 22;
pub const XCB_GET_SELECTION_OWNER: u32 = 23;
pub const XCB_CONVERT_SELECTION: u32 = 24;
pub const XCB_SEND_EVENT: u32 = 25;
pub const XCB_GRAB_POINTER: u32 = 26;
pub const XCB_UNGRAB_POINTER: u32 = 27;
pub const XCB_GRAB_BUTTON: u32 = 28;
pub const XCB_UNGRAB_BUTTON: u32 = 29;
pub const XCB_CHANGE_ACTIVE_POINTER_GRAB: u32 = 30;
pub const XCB_GRAB_KEYBOARD: u32 = 31;
pub const XCB_UNGRAB_KEYBOARD: u32 = 32;
pub const XCB_GRAB_KEY: u32 = 33;
pub const XCB_UNGRAB_KEY: u32 = 34;
pub const XCB_ALLOW_EVENTS: u32 = 35;
pub const XCB_GRAB_SERVER: u32 = 36;
pub const XCB_UNGRAB_SERVER: u32 = 37;
pub const XCB_QUERY_POINTER: u32 = 38;
pub const XCB_GET_MOTION_EVENTS: u32 = 39;
pub const XCB_TRANSLATE_COORDINATES: u32 = 40;
pub const XCB_WARP_POINTER: u32 = 41;
pub const XCB_SET_INPUT_FOCUS: u32 = 42;
pub const XCB_GET_INPUT_FOCUS: u32 = 43;
pub const XCB_QUERY_KEYMAP: u32 = 44;
pub const XCB_OPEN_FONT: u32 = 45;
pub const XCB_CLOSE_FONT: u32 = 46;
pub const XCB_QUERY_FONT: u32 = 47;
pub const XCB_QUERY_TEXT_EXTENTS: u32 = 48;
pub const XCB_LIST_FONTS: u32 = 49;
pub const XCB_LIST_FONTS_WITH_INFO: u32 = 50;
pub const XCB_SET_FONT_PATH: u32 = 51;
pub const XCB_GET_FONT_PATH: u32 = 52;
pub const XCB_CREATE_PIXMAP: u32 = 53;
pub const XCB_FREE_PIXMAP: u32 = 54;
pub const XCB_CREATE_GC: u32 = 55;
pub const XCB_CHANGE_GC: u32 = 56;
pub const XCB_COPY_GC: u32 = 57;
pub const XCB_SET_DASHES: u32 = 58;
pub const XCB_SET_CLIP_RECTANGLES: u32 = 59;
pub const XCB_FREE_GC: u32 = 60;
pub const XCB_CLEAR_AREA: u32 = 61;
pub const XCB_COPY_AREA: u32 = 62;
pub const XCB_COPY_PLANE: u32 = 63;
pub const XCB_POLY_POINT: u32 = 64;
pub const XCB_POLY_LINE: u32 = 65;
pub const XCB_POLY_SEGMENT: u32 = 66;
pub const XCB_POLY_RECTANGLE: u32 = 67;
pub const XCB_POLY_ARC: u32 = 68;
pub const XCB_FILL_POLY: u32 = 69;
pub const XCB_POLY_FILL_RECTANGLE: u32 = 70;
pub const XCB_POLY_FILL_ARC: u32 = 71;
pub const XCB_PUT_IMAGE: u32 = 72;
pub const XCB_GET_IMAGE: u32 = 73;
pub const XCB_POLY_TEXT_8: u32 = 74;
pub const XCB_POLY_TEXT_16: u32 = 75;
pub const XCB_IMAGE_TEXT_8: u32 = 76;
pub const XCB_IMAGE_TEXT_16: u32 = 77;
pub const XCB_CREATE_COLORMAP: u32 = 78;
pub const XCB_FREE_COLORMAP: u32 = 79;
pub const XCB_COPY_COLORMAP_AND_FREE: u32 = 80;
pub const XCB_INSTALL_COLORMAP: u32 = 81;
pub const XCB_UNINSTALL_COLORMAP: u32 = 82;
pub const XCB_LIST_INSTALLED_COLORMAPS: u32 = 83;
pub const XCB_ALLOC_COLOR: u32 = 84;
pub const XCB_ALLOC_NAMED_COLOR: u32 = 85;
pub const XCB_ALLOC_COLOR_CELLS: u32 = 86;
pub const XCB_ALLOC_COLOR_PLANES: u32 = 87;
pub const XCB_FREE_COLORS: u32 = 88;
pub const XCB_STORE_COLORS: u32 = 89;
pub const XCB_STORE_NAMED_COLOR: u32 = 90;
pub const XCB_QUERY_COLORS: u32 = 91;
pub const XCB_LOOKUP_COLOR: u32 = 92;
pub const XCB_CREATE_CURSOR: u32 = 93;
pub const XCB_CREATE_GLYPH_CURSOR: u32 = 94;
pub const XCB_FREE_CURSOR: u32 = 95;
pub const XCB_RECOLOR_CURSOR: u32 = 96;
pub const XCB_QUERY_BEST_SIZE: u32 = 97;
pub const XCB_QUERY_EXTENSION: u32 = 98;
pub const XCB_LIST_EXTENSIONS: u32 = 99;
pub const XCB_CHANGE_KEYBOARD_MAPPING: u32 = 100;
pub const XCB_GET_KEYBOARD_MAPPING: u32 = 101;
pub const XCB_CHANGE_KEYBOARD_CONTROL: u32 = 102;
pub const XCB_GET_KEYBOARD_CONTROL: u32 = 103;
pub const XCB_BELL: u32 = 104;
pub const XCB_CHANGE_POINTER_CONTROL: u32 = 105;
pub const XCB_GET_POINTER_CONTROL: u32 = 106;
pub const XCB_SET_SCREEN_SAVER: u32 = 107;
pub const XCB_GET_SCREEN_SAVER: u32 = 108;
pub const XCB_CHANGE_HOSTS: u32 = 109;
pub const XCB_LIST_HOSTS: u32 = 110;
pub const XCB_SET_ACCESS_CONTROL: u32 = 111;
pub const XCB_SET_CLOSE_DOWN_MODE: u32 = 112;
pub const XCB_KILL_CLIENT: u32 = 113;
pub const XCB_ROTATE_PROPERTIES: u32 = 114;
pub const XCB_FORCE_SCREEN_SAVER: u32 = 115;
pub const XCB_SET_POINTER_MAPPING: u32 = 116;
pub const XCB_GET_POINTER_MAPPING: u32 = 117;
pub const XCB_SET_MODIFIER_MAPPING: u32 = 118;
pub const XCB_GET_MODIFIER_MAPPING: u32 = 119;
pub const XCB_NO_OPERATION: u32 = 127;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_char2b_t
{
  pub byte1: u8,
  pub byte2: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_char2b_iterator_t
{
  pub data: *mut xcb_char2b_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_window_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_window_iterator_t
{
  pub data: *mut xcb_window_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_pixmap_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_pixmap_iterator_t
{
  pub data: *mut xcb_pixmap_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_cursor_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_cursor_iterator_t
{
  pub data: *mut xcb_cursor_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_font_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_font_iterator_t
{
  pub data: *mut xcb_font_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_gcontext_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_gcontext_iterator_t
{
  pub data: *mut xcb_gcontext_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_colormap_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_colormap_iterator_t
{
  pub data: *mut xcb_colormap_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_atom_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_atom_iterator_t
{
  pub data: *mut xcb_atom_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_drawable_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_drawable_iterator_t
{
  pub data: *mut xcb_drawable_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_fontable_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontable_iterator_t
{
  pub data: *mut xcb_fontable_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_bool32_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_bool32_iterator_t
{
  pub data: *mut xcb_bool32_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_visualid_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualid_iterator_t
{
  pub data: *mut xcb_visualid_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_timestamp_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timestamp_iterator_t
{
  pub data: *mut xcb_timestamp_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_keysym_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keysym_iterator_t
{
  pub data: *mut xcb_keysym_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_keycode_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keycode_iterator_t
{
  pub data: *mut xcb_keycode_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_keycode32_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keycode32_iterator_t
{
  pub data: *mut xcb_keycode32_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_button_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_iterator_t
{
  pub data: *mut xcb_button_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_point_t
{
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_point_iterator_t
{
  pub data: *mut xcb_point_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rectangle_t
{
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rectangle_iterator_t
{
  pub data: *mut xcb_rectangle_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_arc_t
{
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub angle1: i16,
  pub angle2: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_arc_iterator_t
{
  pub data: *mut xcb_arc_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_format_t
{
  pub depth: u8,
  pub bits_per_pixel: u8,
  pub scanline_pad: u8,
  pub pad0: [u8; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_format_iterator_t
{
  pub data: *mut xcb_format_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_visual_class_t
{
  XCB_VISUAL_CLASS_STATIC_GRAY = 0,
  XCB_VISUAL_CLASS_GRAY_SCALE = 1,
  XCB_VISUAL_CLASS_STATIC_COLOR = 2,
  XCB_VISUAL_CLASS_PSEUDO_COLOR = 3,
  XCB_VISUAL_CLASS_TRUE_COLOR = 4,
  XCB_VISUAL_CLASS_DIRECT_COLOR = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualtype_t
{
  pub visual_id: xcb_visualid_t,
  pub _class: u8,
  pub bits_per_rgb_value: u8,
  pub colormap_entries: u16,
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub pad0: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualtype_iterator_t
{
  pub data: *mut xcb_visualtype_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_depth_t
{
  pub depth: u8,
  pub pad0: u8,
  pub visuals_len: u16,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_depth_iterator_t
{
  pub data: *mut xcb_depth_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_event_mask_t
{
  XCB_EVENT_MASK_NO_EVENT = 0,
  XCB_EVENT_MASK_KEY_PRESS = 1,
  XCB_EVENT_MASK_KEY_RELEASE = 2,
  XCB_EVENT_MASK_BUTTON_PRESS = 4,
  XCB_EVENT_MASK_BUTTON_RELEASE = 8,
  XCB_EVENT_MASK_ENTER_WINDOW = 16,
  XCB_EVENT_MASK_LEAVE_WINDOW = 32,
  XCB_EVENT_MASK_POINTER_MOTION = 64,
  XCB_EVENT_MASK_POINTER_MOTION_HINT = 128,
  XCB_EVENT_MASK_BUTTON_1_MOTION = 256,
  XCB_EVENT_MASK_BUTTON_2_MOTION = 512,
  XCB_EVENT_MASK_BUTTON_3_MOTION = 1024,
  XCB_EVENT_MASK_BUTTON_4_MOTION = 2048,
  XCB_EVENT_MASK_BUTTON_5_MOTION = 4096,
  XCB_EVENT_MASK_BUTTON_MOTION = 8192,
  XCB_EVENT_MASK_KEYMAP_STATE = 16384,
  XCB_EVENT_MASK_EXPOSURE = 32768,
  XCB_EVENT_MASK_VISIBILITY_CHANGE = 65536,
  XCB_EVENT_MASK_STRUCTURE_NOTIFY = 131072,
  XCB_EVENT_MASK_RESIZE_REDIRECT = 262144,
  XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY = 524288,
  XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT = 1048576,
  XCB_EVENT_MASK_FOCUS_CHANGE = 2097152,
  XCB_EVENT_MASK_PROPERTY_CHANGE = 4194304,
  XCB_EVENT_MASK_COLOR_MAP_CHANGE = 8388608,
  XCB_EVENT_MASK_OWNER_GRAB_BUTTON = 16777216,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_backing_store_t
{
  XCB_BACKING_STORE_NOT_USEFUL = 0,
  XCB_BACKING_STORE_WHEN_MAPPED = 1,
  XCB_BACKING_STORE_ALWAYS = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_t
{
  pub root: xcb_window_t,
  pub default_colormap: xcb_colormap_t,
  pub white_pixel: u32,
  pub black_pixel: u32,
  pub current_input_masks: u32,
  pub width_in_pixels: u16,
  pub height_in_pixels: u16,
  pub width_in_millimeters: u16,
  pub height_in_millimeters: u16,
  pub min_installed_maps: u16,
  pub max_installed_maps: u16,
  pub root_visual: xcb_visualid_t,
  pub backing_stores: u8,
  pub save_unders: u8,
  pub root_depth: u8,
  pub allowed_depths_len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_iterator_t
{
  pub data: *mut xcb_screen_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_request_t
{
  pub byte_order: u8,
  pub pad0: u8,
  pub protocol_major_version: u16,
  pub protocol_minor_version: u16,
  pub authorization_protocol_name_len: u16,
  pub authorization_protocol_data_len: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_request_iterator_t
{
  pub data: *mut xcb_setup_request_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_failed_t
{
  pub status: u8,
  pub reason_len: u8,
  pub protocol_major_version: u16,
  pub protocol_minor_version: u16,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_failed_iterator_t
{
  pub data: *mut xcb_setup_failed_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_authenticate_t
{
  pub status: u8,
  pub pad0: [u8; 5usize],
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_authenticate_iterator_t
{
  pub data: *mut xcb_setup_authenticate_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_image_order_t
{
  XCB_IMAGE_ORDER_LSB_FIRST = 0,
  XCB_IMAGE_ORDER_MSB_FIRST = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_t
{
  pub status: u8,
  pub pad0: u8,
  pub protocol_major_version: u16,
  pub protocol_minor_version: u16,
  pub length: u16,
  pub release_number: u32,
  pub resource_id_base: u32,
  pub resource_id_mask: u32,
  pub motion_buffer_size: u32,
  pub vendor_len: u16,
  pub maximum_request_length: u16,
  pub roots_len: u8,
  pub pixmap_formats_len: u8,
  pub image_byte_order: u8,
  pub bitmap_format_bit_order: u8,
  pub bitmap_format_scanline_unit: u8,
  pub bitmap_format_scanline_pad: u8,
  pub min_keycode: xcb_keycode_t,
  pub max_keycode: xcb_keycode_t,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_iterator_t
{
  pub data: *mut xcb_setup_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_mod_mask_t
{
  XCB_MOD_MASK_SHIFT = 1,
  XCB_MOD_MASK_LOCK = 2,
  XCB_MOD_MASK_CONTROL = 4,
  XCB_MOD_MASK_1 = 8,
  XCB_MOD_MASK_2 = 16,
  XCB_MOD_MASK_3 = 32,
  XCB_MOD_MASK_4 = 64,
  XCB_MOD_MASK_5 = 128,
  XCB_MOD_MASK_ANY = 32768,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_key_but_mask_t
{
  XCB_KEY_BUT_MASK_SHIFT = 1,
  XCB_KEY_BUT_MASK_LOCK = 2,
  XCB_KEY_BUT_MASK_CONTROL = 4,
  XCB_KEY_BUT_MASK_MOD_1 = 8,
  XCB_KEY_BUT_MASK_MOD_2 = 16,
  XCB_KEY_BUT_MASK_MOD_3 = 32,
  XCB_KEY_BUT_MASK_MOD_4 = 64,
  XCB_KEY_BUT_MASK_MOD_5 = 128,
  XCB_KEY_BUT_MASK_BUTTON_1 = 256,
  XCB_KEY_BUT_MASK_BUTTON_2 = 512,
  XCB_KEY_BUT_MASK_BUTTON_3 = 1024,
  XCB_KEY_BUT_MASK_BUTTON_4 = 2048,
  XCB_KEY_BUT_MASK_BUTTON_5 = 4096,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_window_enum_t
{
  XCB_WINDOW_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_key_press_event_t
{
  pub response_type: u8,
  pub detail: xcb_keycode_t,
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
  pub pad0: u8,
}

pub type xcb_key_release_event_t = xcb_key_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_button_mask_t
{
  XCB_BUTTON_MASK_1 = 256,
  XCB_BUTTON_MASK_2 = 512,
  XCB_BUTTON_MASK_3 = 1024,
  XCB_BUTTON_MASK_4 = 2048,
  XCB_BUTTON_MASK_5 = 4096,
  XCB_BUTTON_MASK_ANY = 32768,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_press_event_t
{
  pub response_type: u8,
  pub detail: xcb_button_t,
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
  pub pad0: u8,
}

pub type xcb_button_release_event_t = xcb_button_press_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_motion_t
{
  XCB_MOTION_NORMAL = 0,
  XCB_MOTION_HINT = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_motion_notify_event_t
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
  pub pad0: u8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_notify_detail_t
{
  XCB_NOTIFY_DETAIL_ANCESTOR = 0,
  XCB_NOTIFY_DETAIL_VIRTUAL = 1,
  XCB_NOTIFY_DETAIL_INFERIOR = 2,
  XCB_NOTIFY_DETAIL_NONLINEAR = 3,
  XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL = 4,
  XCB_NOTIFY_DETAIL_POINTER = 5,
  XCB_NOTIFY_DETAIL_POINTER_ROOT = 6,
  XCB_NOTIFY_DETAIL_NONE = 7,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_notify_mode_t
{
  XCB_NOTIFY_MODE_NORMAL = 0,
  XCB_NOTIFY_MODE_GRAB = 1,
  XCB_NOTIFY_MODE_UNGRAB = 2,
  XCB_NOTIFY_MODE_WHILE_GRABBED = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_enter_notify_event_t
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
  pub mode: u8,
  pub same_screen_focus: u8,
}

pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_focus_in_event_t
{
  pub response_type: u8,
  pub detail: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub mode: u8,
  pub pad0: [u8; 3usize],
}

pub type xcb_focus_out_event_t = xcb_focus_in_event_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keymap_notify_event_t
{
  pub response_type: u8,
  pub keys: [u8; 31usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_expose_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
  pub count: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_graphics_exposure_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub drawable: xcb_drawable_t,
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
  pub minor_opcode: u16,
  pub count: u16,
  pub major_opcode: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_no_exposure_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub drawable: xcb_drawable_t,
  pub minor_opcode: u16,
  pub major_opcode: u8,
  pub pad1: u8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_visibility_t
{
  XCB_VISIBILITY_UNOBSCURED = 0,
  XCB_VISIBILITY_PARTIALLY_OBSCURED = 1,
  XCB_VISIBILITY_FULLY_OBSCURED = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visibility_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub state: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub parent: xcb_window_t,
  pub window: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub override_redirect: u8,
  pub pad1: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub from_configure: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub override_redirect: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_request_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub parent: xcb_window_t,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_reparent_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub parent: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub override_redirect: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub above_sibling: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub override_redirect: u8,
  pub pad1: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_request_event_t
{
  pub response_type: u8,
  pub stack_mode: u8,
  pub sequence: u16,
  pub parent: xcb_window_t,
  pub window: xcb_window_t,
  pub sibling: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub value_mask: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_gravity_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_resize_request_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub width: u16,
  pub height: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_place_t
{
  XCB_PLACE_ON_TOP = 0,
  XCB_PLACE_ON_BOTTOM = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_circulate_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub event: xcb_window_t,
  pub window: xcb_window_t,
  pub pad1: [u8; 4usize],
  pub place: u8,
  pub pad2: [u8; 3usize],
}

pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_property_t
{
  XCB_PROPERTY_NEW_VALUE = 0,
  XCB_PROPERTY_DELETE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_property_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub atom: xcb_atom_t,
  pub time: xcb_timestamp_t,
  pub state: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_clear_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub owner: xcb_window_t,
  pub selection: xcb_atom_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_time_t
{
  XCB_TIME_CURRENT_TIME = 0,
}

impl xcb_atom_enum_t
{
  pub const XCB_ATOM_ANY: xcb_atom_enum_t = xcb_atom_enum_t::XCB_ATOM_NONE;
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_atom_enum_t
{
  XCB_ATOM_NONE = 0,
  XCB_ATOM_PRIMARY = 1,
  XCB_ATOM_SECONDARY = 2,
  XCB_ATOM_ARC = 3,
  XCB_ATOM_ATOM = 4,
  XCB_ATOM_BITMAP = 5,
  XCB_ATOM_CARDINAL = 6,
  XCB_ATOM_COLORMAP = 7,
  XCB_ATOM_CURSOR = 8,
  XCB_ATOM_CUT_BUFFER0 = 9,
  XCB_ATOM_CUT_BUFFER1 = 10,
  XCB_ATOM_CUT_BUFFER2 = 11,
  XCB_ATOM_CUT_BUFFER3 = 12,
  XCB_ATOM_CUT_BUFFER4 = 13,
  XCB_ATOM_CUT_BUFFER5 = 14,
  XCB_ATOM_CUT_BUFFER6 = 15,
  XCB_ATOM_CUT_BUFFER7 = 16,
  XCB_ATOM_DRAWABLE = 17,
  XCB_ATOM_FONT = 18,
  XCB_ATOM_INTEGER = 19,
  XCB_ATOM_PIXMAP = 20,
  XCB_ATOM_POINT = 21,
  XCB_ATOM_RECTANGLE = 22,
  XCB_ATOM_RESOURCE_MANAGER = 23,
  XCB_ATOM_RGB_COLOR_MAP = 24,
  XCB_ATOM_RGB_BEST_MAP = 25,
  XCB_ATOM_RGB_BLUE_MAP = 26,
  XCB_ATOM_RGB_DEFAULT_MAP = 27,
  XCB_ATOM_RGB_GRAY_MAP = 28,
  XCB_ATOM_RGB_GREEN_MAP = 29,
  XCB_ATOM_RGB_RED_MAP = 30,
  XCB_ATOM_STRING = 31,
  XCB_ATOM_VISUALID = 32,
  XCB_ATOM_WINDOW = 33,
  XCB_ATOM_WM_COMMAND = 34,
  XCB_ATOM_WM_HINTS = 35,
  XCB_ATOM_WM_CLIENT_MACHINE = 36,
  XCB_ATOM_WM_ICON_NAME = 37,
  XCB_ATOM_WM_ICON_SIZE = 38,
  XCB_ATOM_WM_NAME = 39,
  XCB_ATOM_WM_NORMAL_HINTS = 40,
  XCB_ATOM_WM_SIZE_HINTS = 41,
  XCB_ATOM_WM_ZOOM_HINTS = 42,
  XCB_ATOM_MIN_SPACE = 43,
  XCB_ATOM_NORM_SPACE = 44,
  XCB_ATOM_MAX_SPACE = 45,
  XCB_ATOM_END_SPACE = 46,
  XCB_ATOM_SUPERSCRIPT_X = 47,
  XCB_ATOM_SUPERSCRIPT_Y = 48,
  XCB_ATOM_SUBSCRIPT_X = 49,
  XCB_ATOM_SUBSCRIPT_Y = 50,
  XCB_ATOM_UNDERLINE_POSITION = 51,
  XCB_ATOM_UNDERLINE_THICKNESS = 52,
  XCB_ATOM_STRIKEOUT_ASCENT = 53,
  XCB_ATOM_STRIKEOUT_DESCENT = 54,
  XCB_ATOM_ITALIC_ANGLE = 55,
  XCB_ATOM_X_HEIGHT = 56,
  XCB_ATOM_QUAD_WIDTH = 57,
  XCB_ATOM_WEIGHT = 58,
  XCB_ATOM_POINT_SIZE = 59,
  XCB_ATOM_RESOLUTION = 60,
  XCB_ATOM_COPYRIGHT = 61,
  XCB_ATOM_NOTICE = 62,
  XCB_ATOM_FONT_NAME = 63,
  XCB_ATOM_FAMILY_NAME = 64,
  XCB_ATOM_FULL_NAME = 65,
  XCB_ATOM_CAP_HEIGHT = 66,
  XCB_ATOM_WM_CLASS = 67,
  XCB_ATOM_WM_TRANSIENT_FOR = 68,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_request_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub owner: xcb_window_t,
  pub requestor: xcb_window_t,
  pub selection: xcb_atom_t,
  pub target: xcb_atom_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub requestor: xcb_window_t,
  pub selection: xcb_atom_t,
  pub target: xcb_atom_t,
  pub property: xcb_atom_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_colormap_state_t
{
  XCB_COLORMAP_STATE_UNINSTALLED = 0,
  XCB_COLORMAP_STATE_INSTALLED = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_colormap_enum_t
{
  XCB_COLORMAP_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_colormap_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub colormap: xcb_colormap_t,
  pub _new: u8,
  pub state: u8,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_client_message_data_t
{
  pub data8: [u8; 20usize],
  pub data16: [u16; 10usize],
  pub data32: [u32; 5usize],
  _bindgen_union_align: [u32; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_client_message_data_iterator_t
{
  pub data: *mut xcb_client_message_data_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_client_message_event_t
{
  pub response_type: u8,
  pub format: u8,
  pub sequence: u16,
  pub window: xcb_window_t,
  pub type_: xcb_atom_t,
  pub data: xcb_client_message_data_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_mapping_t
{
  XCB_MAPPING_MODIFIER = 0,
  XCB_MAPPING_KEYBOARD = 1,
  XCB_MAPPING_POINTER = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_mapping_notify_event_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub request: u8,
  pub first_keycode: xcb_keycode_t,
  pub count: u8,
  pub pad1: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ge_generic_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub pad0: [u8; 22usize],
  pub full_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_request_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub bad_value: u32,
  pub minor_opcode: u16,
  pub major_opcode: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_value_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub bad_value: u32,
  pub minor_opcode: u16,
  pub major_opcode: u8,
  pub pad0: u8,
}

pub type xcb_window_error_t = xcb_value_error_t;

pub type xcb_pixmap_error_t = xcb_value_error_t;

pub type xcb_atom_error_t = xcb_value_error_t;

pub type xcb_cursor_error_t = xcb_value_error_t;

pub type xcb_font_error_t = xcb_value_error_t;

pub type xcb_match_error_t = xcb_request_error_t;

pub type xcb_drawable_error_t = xcb_value_error_t;

pub type xcb_access_error_t = xcb_request_error_t;

pub type xcb_alloc_error_t = xcb_request_error_t;

pub type xcb_colormap_error_t = xcb_value_error_t;

pub type xcb_g_context_error_t = xcb_value_error_t;

pub type xcb_id_choice_error_t = xcb_value_error_t;

pub type xcb_name_error_t = xcb_request_error_t;

pub type xcb_length_error_t = xcb_request_error_t;

pub type xcb_implementation_error_t = xcb_request_error_t;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_window_class_t
{
  XCB_WINDOW_CLASS_COPY_FROM_PARENT = 0,
  XCB_WINDOW_CLASS_INPUT_OUTPUT = 1,
  XCB_WINDOW_CLASS_INPUT_ONLY = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_cw_t
{
  XCB_CW_BACK_PIXMAP = 1,
  XCB_CW_BACK_PIXEL = 2,
  XCB_CW_BORDER_PIXMAP = 4,
  XCB_CW_BORDER_PIXEL = 8,
  XCB_CW_BIT_GRAVITY = 16,
  XCB_CW_WIN_GRAVITY = 32,
  XCB_CW_BACKING_STORE = 64,
  XCB_CW_BACKING_PLANES = 128,
  XCB_CW_BACKING_PIXEL = 256,
  XCB_CW_OVERRIDE_REDIRECT = 512,
  XCB_CW_SAVE_UNDER = 1024,
  XCB_CW_EVENT_MASK = 2048,
  XCB_CW_DONT_PROPAGATE = 4096,
  XCB_CW_COLORMAP = 8192,
  XCB_CW_CURSOR = 16384,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_back_pixmap_t
{
  XCB_BACK_PIXMAP_NONE = 0,
  XCB_BACK_PIXMAP_PARENT_RELATIVE = 1,
}

impl xcb_gravity_t
{
  pub const XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = xcb_gravity_t::XCB_GRAVITY_BIT_FORGET;
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_gravity_t
{
  XCB_GRAVITY_BIT_FORGET = 0,
  XCB_GRAVITY_NORTH_WEST = 1,
  XCB_GRAVITY_NORTH = 2,
  XCB_GRAVITY_NORTH_EAST = 3,
  XCB_GRAVITY_WEST = 4,
  XCB_GRAVITY_CENTER = 5,
  XCB_GRAVITY_EAST = 6,
  XCB_GRAVITY_SOUTH_WEST = 7,
  XCB_GRAVITY_SOUTH = 8,
  XCB_GRAVITY_SOUTH_EAST = 9,
  XCB_GRAVITY_STATIC = 10,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_window_value_list_t
{
  pub background_pixmap: xcb_pixmap_t,
  pub background_pixel: u32,
  pub border_pixmap: xcb_pixmap_t,
  pub border_pixel: u32,
  pub bit_gravity: u32,
  pub win_gravity: u32,
  pub backing_store: u32,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub override_redirect: xcb_bool32_t,
  pub save_under: xcb_bool32_t,
  pub event_mask: u32,
  pub do_not_propogate_mask: u32,
  pub colormap: xcb_colormap_t,
  pub cursor: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_window_request_t
{
  pub major_opcode: u8,
  pub depth: u8,
  pub length: u16,
  pub wid: xcb_window_t,
  pub parent: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub _class: u16,
  pub visual: xcb_visualid_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_window_attributes_value_list_t
{
  pub background_pixmap: xcb_pixmap_t,
  pub background_pixel: u32,
  pub border_pixmap: xcb_pixmap_t,
  pub border_pixel: u32,
  pub bit_gravity: u32,
  pub win_gravity: u32,
  pub backing_store: u32,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub override_redirect: xcb_bool32_t,
  pub save_under: xcb_bool32_t,
  pub event_mask: u32,
  pub do_not_propogate_mask: u32,
  pub colormap: xcb_colormap_t,
  pub cursor: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_window_attributes_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub value_mask: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_map_state_t
{
  XCB_MAP_STATE_UNMAPPED = 0,
  XCB_MAP_STATE_UNVIEWABLE = 1,
  XCB_MAP_STATE_VIEWABLE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_reply_t
{
  pub response_type: u8,
  pub backing_store: u8,
  pub sequence: u16,
  pub length: u32,
  pub visual: xcb_visualid_t,
  pub _class: u16,
  pub bit_gravity: u8,
  pub win_gravity: u8,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub save_under: u8,
  pub map_is_installed: u8,
  pub map_state: u8,
  pub override_redirect: u8,
  pub colormap: xcb_colormap_t,
  pub all_event_masks: u32,
  pub your_event_mask: u32,
  pub do_not_propagate_mask: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_window_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_subwindows_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_set_mode_t
{
  XCB_SET_MODE_INSERT = 0,
  XCB_SET_MODE_DELETE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_save_set_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_reparent_window_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub parent: xcb_window_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_window_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_subwindows_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_window_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_subwindows_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_config_window_t
{
  XCB_CONFIG_WINDOW_X = 1,
  XCB_CONFIG_WINDOW_Y = 2,
  XCB_CONFIG_WINDOW_WIDTH = 4,
  XCB_CONFIG_WINDOW_HEIGHT = 8,
  XCB_CONFIG_WINDOW_BORDER_WIDTH = 16,
  XCB_CONFIG_WINDOW_SIBLING = 32,
  XCB_CONFIG_WINDOW_STACK_MODE = 64,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_stack_mode_t
{
  XCB_STACK_MODE_ABOVE = 0,
  XCB_STACK_MODE_BELOW = 1,
  XCB_STACK_MODE_TOP_IF = 2,
  XCB_STACK_MODE_BOTTOM_IF = 3,
  XCB_STACK_MODE_OPPOSITE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_window_value_list_t
{
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub border_width: u32,
  pub sibling: xcb_window_t,
  pub stack_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_window_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub value_mask: u16,
  pub pad1: [u8; 2usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_circulate_t
{
  XCB_CIRCULATE_RAISE_LOWEST = 0,
  XCB_CIRCULATE_LOWER_HIGHEST = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_circulate_window_request_t
{
  pub major_opcode: u8,
  pub direction: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_reply_t
{
  pub response_type: u8,
  pub depth: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub border_width: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub parent: xcb_window_t,
  pub children_len: u16,
  pub pad1: [u8; 14usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_request_t
{
  pub major_opcode: u8,
  pub only_if_exists: u8,
  pub length: u16,
  pub name_len: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub atom: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub atom: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub name_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_prop_mode_t
{
  XCB_PROP_MODE_REPLACE = 0,
  XCB_PROP_MODE_PREPEND = 1,
  XCB_PROP_MODE_APPEND = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_property_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub format: u8,
  pub pad0: [u8; 3usize],
  pub data_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_delete_property_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub property: xcb_atom_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_get_property_type_t
{
  XCB_GET_PROPERTY_TYPE_ANY = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_request_t
{
  pub major_opcode: u8,
  pub _delete: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub property: xcb_atom_t,
  pub type_: xcb_atom_t,
  pub long_offset: u32,
  pub long_length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_reply_t
{
  pub response_type: u8,
  pub format: u8,
  pub sequence: u16,
  pub length: u32,
  pub type_: xcb_atom_t,
  pub bytes_after: u32,
  pub value_len: u32,
  pub pad0: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub atoms_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_selection_owner_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub owner: xcb_window_t,
  pub selection: xcb_atom_t,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub selection: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub owner: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_convert_selection_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub requestor: xcb_window_t,
  pub selection: xcb_atom_t,
  pub target: xcb_atom_t,
  pub property: xcb_atom_t,
  pub time: xcb_timestamp_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_send_event_dest_t
{
  XCB_SEND_EVENT_DEST_POINTER_WINDOW = 0,
  XCB_SEND_EVENT_DEST_ITEM_FOCUS = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_send_event_request_t
{
  pub major_opcode: u8,
  pub propagate: u8,
  pub length: u16,
  pub destination: xcb_window_t,
  pub event_mask: u32,
  pub event: [::std::os::raw::c_char; 32usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_grab_mode_t
{
  XCB_GRAB_MODE_SYNC = 0,
  XCB_GRAB_MODE_ASYNC = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_grab_status_t
{
  XCB_GRAB_STATUS_SUCCESS = 0,
  XCB_GRAB_STATUS_ALREADY_GRABBED = 1,
  XCB_GRAB_STATUS_INVALID_TIME = 2,
  XCB_GRAB_STATUS_NOT_VIEWABLE = 3,
  XCB_GRAB_STATUS_FROZEN = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_cursor_enum_t
{
  XCB_CURSOR_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_request_t
{
  pub major_opcode: u8,
  pub owner_events: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub event_mask: u16,
  pub pointer_mode: u8,
  pub keyboard_mode: u8,
  pub confine_to: xcb_window_t,
  pub cursor: xcb_cursor_t,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_pointer_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_button_index_t
{
  XCB_BUTTON_INDEX_ANY = 0,
  XCB_BUTTON_INDEX_1 = 1,
  XCB_BUTTON_INDEX_2 = 2,
  XCB_BUTTON_INDEX_3 = 3,
  XCB_BUTTON_INDEX_4 = 4,
  XCB_BUTTON_INDEX_5 = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_button_request_t
{
  pub major_opcode: u8,
  pub owner_events: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub event_mask: u16,
  pub pointer_mode: u8,
  pub keyboard_mode: u8,
  pub confine_to: xcb_window_t,
  pub cursor: xcb_cursor_t,
  pub button: u8,
  pub pad0: u8,
  pub modifiers: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_button_request_t
{
  pub major_opcode: u8,
  pub button: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub modifiers: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_active_pointer_grab_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cursor: xcb_cursor_t,
  pub time: xcb_timestamp_t,
  pub event_mask: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_request_t
{
  pub major_opcode: u8,
  pub owner_events: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub time: xcb_timestamp_t,
  pub pointer_mode: u8,
  pub keyboard_mode: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_keyboard_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_grab_t
{
  XCB_GRAB_ANY = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_key_request_t
{
  pub major_opcode: u8,
  pub owner_events: u8,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub modifiers: u16,
  pub key: xcb_keycode_t,
  pub pointer_mode: u8,
  pub keyboard_mode: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_key_request_t
{
  pub major_opcode: u8,
  pub key: xcb_keycode_t,
  pub length: u16,
  pub grab_window: xcb_window_t,
  pub modifiers: u16,
  pub pad0: [u8; 2usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_allow_t
{
  XCB_ALLOW_ASYNC_POINTER = 0,
  XCB_ALLOW_SYNC_POINTER = 1,
  XCB_ALLOW_REPLAY_POINTER = 2,
  XCB_ALLOW_ASYNC_KEYBOARD = 3,
  XCB_ALLOW_SYNC_KEYBOARD = 4,
  XCB_ALLOW_REPLAY_KEYBOARD = 5,
  XCB_ALLOW_ASYNC_BOTH = 6,
  XCB_ALLOW_SYNC_BOTH = 7,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_allow_events_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_server_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_server_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_reply_t
{
  pub response_type: u8,
  pub same_screen: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
  pub child: xcb_window_t,
  pub root_x: i16,
  pub root_y: i16,
  pub win_x: i16,
  pub win_y: i16,
  pub mask: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timecoord_t
{
  pub time: xcb_timestamp_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timecoord_iterator_t
{
  pub data: *mut xcb_timecoord_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub start: xcb_timestamp_t,
  pub stop: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub events_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub src_window: xcb_window_t,
  pub dst_window: xcb_window_t,
  pub src_x: i16,
  pub src_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_reply_t
{
  pub response_type: u8,
  pub same_screen: u8,
  pub sequence: u16,
  pub length: u32,
  pub child: xcb_window_t,
  pub dst_x: i16,
  pub dst_y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_warp_pointer_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub src_window: xcb_window_t,
  pub dst_window: xcb_window_t,
  pub src_x: i16,
  pub src_y: i16,
  pub src_width: u16,
  pub src_height: u16,
  pub dst_x: i16,
  pub dst_y: i16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_input_focus_t
{
  XCB_INPUT_FOCUS_NONE = 0,
  XCB_INPUT_FOCUS_POINTER_ROOT = 1,
  XCB_INPUT_FOCUS_PARENT = 2,
  XCB_INPUT_FOCUS_FOLLOW_KEYBOARD = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_input_focus_request_t
{
  pub major_opcode: u8,
  pub revert_to: u8,
  pub length: u16,
  pub focus: xcb_window_t,
  pub time: xcb_timestamp_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_reply_t
{
  pub response_type: u8,
  pub revert_to: u8,
  pub sequence: u16,
  pub length: u32,
  pub focus: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub keys: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_open_font_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub fid: xcb_font_t,
  pub name_len: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_close_font_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub font: xcb_font_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_font_draw_t
{
  XCB_FONT_DRAW_LEFT_TO_RIGHT = 0,
  XCB_FONT_DRAW_RIGHT_TO_LEFT = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontprop_t
{
  pub name: xcb_atom_t,
  pub value: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontprop_iterator_t
{
  pub data: *mut xcb_fontprop_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_charinfo_t
{
  pub left_side_bearing: i16,
  pub right_side_bearing: i16,
  pub character_width: i16,
  pub ascent: i16,
  pub descent: i16,
  pub attributes: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_charinfo_iterator_t
{
  pub data: *mut xcb_charinfo_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub font: xcb_fontable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub min_bounds: xcb_charinfo_t,
  pub pad1: [u8; 4usize],
  pub max_bounds: xcb_charinfo_t,
  pub pad2: [u8; 4usize],
  pub min_char_or_byte2: u16,
  pub max_char_or_byte2: u16,
  pub default_char: u16,
  pub properties_len: u16,
  pub draw_direction: u8,
  pub min_byte1: u8,
  pub max_byte1: u8,
  pub all_chars_exist: u8,
  pub font_ascent: i16,
  pub font_descent: i16,
  pub char_infos_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_request_t
{
  pub major_opcode: u8,
  pub odd_length: u8,
  pub length: u16,
  pub font: xcb_fontable_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_reply_t
{
  pub response_type: u8,
  pub draw_direction: u8,
  pub sequence: u16,
  pub length: u32,
  pub font_ascent: i16,
  pub font_descent: i16,
  pub overall_ascent: i16,
  pub overall_descent: i16,
  pub overall_width: i32,
  pub overall_left: i32,
  pub overall_right: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_str_t
{
  pub name_len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_str_iterator_t
{
  pub data: *mut xcb_str_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub max_names: u16,
  pub pattern_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub names_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub max_names: u16,
  pub pattern_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_reply_t
{
  pub response_type: u8,
  pub name_len: u8,
  pub sequence: u16,
  pub length: u32,
  pub min_bounds: xcb_charinfo_t,
  pub pad0: [u8; 4usize],
  pub max_bounds: xcb_charinfo_t,
  pub pad1: [u8; 4usize],
  pub min_char_or_byte2: u16,
  pub max_char_or_byte2: u16,
  pub default_char: u16,
  pub properties_len: u16,
  pub draw_direction: u8,
  pub min_byte1: u8,
  pub max_byte1: u8,
  pub all_chars_exist: u8,
  pub font_ascent: i16,
  pub font_descent: i16,
  pub replies_hint: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_font_path_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub font_qty: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub path_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_pixmap_request_t
{
  pub major_opcode: u8,
  pub depth: u8,
  pub length: u16,
  pub pid: xcb_pixmap_t,
  pub drawable: xcb_drawable_t,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_pixmap_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub pixmap: xcb_pixmap_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_gc_t
{
  XCB_GC_FUNCTION = 1,
  XCB_GC_PLANE_MASK = 2,
  XCB_GC_FOREGROUND = 4,
  XCB_GC_BACKGROUND = 8,
  XCB_GC_LINE_WIDTH = 16,
  XCB_GC_LINE_STYLE = 32,
  XCB_GC_CAP_STYLE = 64,
  XCB_GC_JOIN_STYLE = 128,
  XCB_GC_FILL_STYLE = 256,
  XCB_GC_FILL_RULE = 512,
  XCB_GC_TILE = 1024,
  XCB_GC_STIPPLE = 2048,
  XCB_GC_TILE_STIPPLE_ORIGIN_X = 4096,
  XCB_GC_TILE_STIPPLE_ORIGIN_Y = 8192,
  XCB_GC_FONT = 16384,
  XCB_GC_SUBWINDOW_MODE = 32768,
  XCB_GC_GRAPHICS_EXPOSURES = 65536,
  XCB_GC_CLIP_ORIGIN_X = 131072,
  XCB_GC_CLIP_ORIGIN_Y = 262144,
  XCB_GC_CLIP_MASK = 524288,
  XCB_GC_DASH_OFFSET = 1048576,
  XCB_GC_DASH_LIST = 2097152,
  XCB_GC_ARC_MODE = 4194304,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_gx_t
{
  XCB_GX_CLEAR = 0,
  XCB_GX_AND = 1,
  XCB_GX_AND_REVERSE = 2,
  XCB_GX_COPY = 3,
  XCB_GX_AND_INVERTED = 4,
  XCB_GX_NOOP = 5,
  XCB_GX_XOR = 6,
  XCB_GX_OR = 7,
  XCB_GX_NOR = 8,
  XCB_GX_EQUIV = 9,
  XCB_GX_INVERT = 10,
  XCB_GX_OR_REVERSE = 11,
  XCB_GX_COPY_INVERTED = 12,
  XCB_GX_OR_INVERTED = 13,
  XCB_GX_NAND = 14,
  XCB_GX_SET = 15,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_line_style_t
{
  XCB_LINE_STYLE_SOLID = 0,
  XCB_LINE_STYLE_ON_OFF_DASH = 1,
  XCB_LINE_STYLE_DOUBLE_DASH = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_cap_style_t
{
  XCB_CAP_STYLE_NOT_LAST = 0,
  XCB_CAP_STYLE_BUTT = 1,
  XCB_CAP_STYLE_ROUND = 2,
  XCB_CAP_STYLE_PROJECTING = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_join_style_t
{
  XCB_JOIN_STYLE_MITER = 0,
  XCB_JOIN_STYLE_ROUND = 1,
  XCB_JOIN_STYLE_BEVEL = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_fill_style_t
{
  XCB_FILL_STYLE_SOLID = 0,
  XCB_FILL_STYLE_TILED = 1,
  XCB_FILL_STYLE_STIPPLED = 2,
  XCB_FILL_STYLE_OPAQUE_STIPPLED = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_fill_rule_t
{
  XCB_FILL_RULE_EVEN_ODD = 0,
  XCB_FILL_RULE_WINDING = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_subwindow_mode_t
{
  XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN = 0,
  XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_arc_mode_t
{
  XCB_ARC_MODE_CHORD = 0,
  XCB_ARC_MODE_PIE_SLICE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_gc_value_list_t
{
  pub function: u32,
  pub plane_mask: u32,
  pub foreground: u32,
  pub background: u32,
  pub line_width: u32,
  pub line_style: u32,
  pub cap_style: u32,
  pub join_style: u32,
  pub fill_style: u32,
  pub fill_rule: u32,
  pub tile: xcb_pixmap_t,
  pub stipple: xcb_pixmap_t,
  pub tile_stipple_x_origin: i32,
  pub tile_stipple_y_origin: i32,
  pub font: xcb_font_t,
  pub subwindow_mode: u32,
  pub graphics_exposures: xcb_bool32_t,
  pub clip_x_origin: i32,
  pub clip_y_origin: i32,
  pub clip_mask: xcb_pixmap_t,
  pub dash_offset: u32,
  pub dashes: u32,
  pub arc_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_gc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cid: xcb_gcontext_t,
  pub drawable: xcb_drawable_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_gc_value_list_t
{
  pub function: u32,
  pub plane_mask: u32,
  pub foreground: u32,
  pub background: u32,
  pub line_width: u32,
  pub line_style: u32,
  pub cap_style: u32,
  pub join_style: u32,
  pub fill_style: u32,
  pub fill_rule: u32,
  pub tile: xcb_pixmap_t,
  pub stipple: xcb_pixmap_t,
  pub tile_stipple_x_origin: i32,
  pub tile_stipple_y_origin: i32,
  pub font: xcb_font_t,
  pub subwindow_mode: u32,
  pub graphics_exposures: xcb_bool32_t,
  pub clip_x_origin: i32,
  pub clip_y_origin: i32,
  pub clip_mask: xcb_pixmap_t,
  pub dash_offset: u32,
  pub dashes: u32,
  pub arc_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_gc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub gc: xcb_gcontext_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_gc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub src_gc: xcb_gcontext_t,
  pub dst_gc: xcb_gcontext_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_dashes_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub gc: xcb_gcontext_t,
  pub dash_offset: u16,
  pub dashes_len: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_clip_ordering_t
{
  XCB_CLIP_ORDERING_UNSORTED = 0,
  XCB_CLIP_ORDERING_Y_SORTED = 1,
  XCB_CLIP_ORDERING_YX_SORTED = 2,
  XCB_CLIP_ORDERING_YX_BANDED = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_clip_rectangles_request_t
{
  pub major_opcode: u8,
  pub ordering: u8,
  pub length: u16,
  pub gc: xcb_gcontext_t,
  pub clip_x_origin: i16,
  pub clip_y_origin: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_gc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_clear_area_request_t
{
  pub major_opcode: u8,
  pub exposures: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_area_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub src_drawable: xcb_drawable_t,
  pub dst_drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub src_x: i16,
  pub src_y: i16,
  pub dst_x: i16,
  pub dst_y: i16,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_plane_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub src_drawable: xcb_drawable_t,
  pub dst_drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub src_x: i16,
  pub src_y: i16,
  pub dst_x: i16,
  pub dst_y: i16,
  pub width: u16,
  pub height: u16,
  pub bit_plane: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_coord_mode_t
{
  XCB_COORD_MODE_ORIGIN = 0,
  XCB_COORD_MODE_PREVIOUS = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_point_request_t
{
  pub major_opcode: u8,
  pub coordinate_mode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_line_request_t
{
  pub major_opcode: u8,
  pub coordinate_mode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_segment_t
{
  pub x1: i16,
  pub y1: i16,
  pub x2: i16,
  pub y2: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_segment_iterator_t
{
  pub data: *mut xcb_segment_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_segment_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_rectangle_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_arc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_poly_shape_t
{
  XCB_POLY_SHAPE_COMPLEX = 0,
  XCB_POLY_SHAPE_NONCONVEX = 1,
  XCB_POLY_SHAPE_CONVEX = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fill_poly_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub shape: u8,
  pub coordinate_mode: u8,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_fill_rectangle_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_fill_arc_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_image_format_t
{
  XCB_IMAGE_FORMAT_XY_BITMAP = 0,
  XCB_IMAGE_FORMAT_XY_PIXMAP = 1,
  XCB_IMAGE_FORMAT_Z_PIXMAP = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_put_image_request_t
{
  pub major_opcode: u8,
  pub format: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub width: u16,
  pub height: u16,
  pub dst_x: i16,
  pub dst_y: i16,
  pub left_pad: u8,
  pub depth: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_request_t
{
  pub major_opcode: u8,
  pub format: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub plane_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_reply_t
{
  pub response_type: u8,
  pub depth: u8,
  pub sequence: u16,
  pub length: u32,
  pub visual: xcb_visualid_t,
  pub pad0: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_text_8_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_text_16_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_image_text_8_request_t
{
  pub major_opcode: u8,
  pub string_len: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub x: i16,
  pub y: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_image_text_16_request_t
{
  pub major_opcode: u8,
  pub string_len: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub gc: xcb_gcontext_t,
  pub x: i16,
  pub y: i16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_colormap_alloc_t
{
  XCB_COLORMAP_ALLOC_NONE = 0,
  XCB_COLORMAP_ALLOC_ALL = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_colormap_request_t
{
  pub major_opcode: u8,
  pub alloc: u8,
  pub length: u16,
  pub mid: xcb_colormap_t,
  pub window: xcb_window_t,
  pub visual: xcb_visualid_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_colormap_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_colormap_and_free_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub mid: xcb_colormap_t,
  pub src_cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_install_colormap_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_uninstall_colormap_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub cmaps_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub pad1: [u8; 2usize],
  pub pixel: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub name_len: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pixel: u32,
  pub exact_red: u16,
  pub exact_green: u16,
  pub exact_blue: u16,
  pub visual_red: u16,
  pub visual_green: u16,
  pub visual_blue: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_request_t
{
  pub major_opcode: u8,
  pub contiguous: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub colors: u16,
  pub planes: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pixels_len: u16,
  pub masks_len: u16,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_request_t
{
  pub major_opcode: u8,
  pub contiguous: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub colors: u16,
  pub reds: u16,
  pub greens: u16,
  pub blues: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub pixels_len: u16,
  pub pad1: [u8; 2usize],
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub pad2: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_colors_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub plane_mask: u32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_color_flag_t
{
  XCB_COLOR_FLAG_RED = 1,
  XCB_COLOR_FLAG_GREEN = 2,
  XCB_COLOR_FLAG_BLUE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_coloritem_t
{
  pub pixel: u32,
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub flags: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_coloritem_iterator_t
{
  pub data: *mut xcb_coloritem_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_store_colors_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_store_named_color_request_t
{
  pub major_opcode: u8,
  pub flags: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub pixel: u32,
  pub name_len: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rgb_t
{
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rgb_iterator_t
{
  pub data: *mut xcb_rgb_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub colors_len: u16,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cmap: xcb_colormap_t,
  pub name_len: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub exact_red: u16,
  pub exact_green: u16,
  pub exact_blue: u16,
  pub visual_red: u16,
  pub visual_green: u16,
  pub visual_blue: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_pixmap_enum_t
{
  XCB_PIXMAP_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_cursor_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cid: xcb_cursor_t,
  pub source: xcb_pixmap_t,
  pub mask: xcb_pixmap_t,
  pub fore_red: u16,
  pub fore_green: u16,
  pub fore_blue: u16,
  pub back_red: u16,
  pub back_green: u16,
  pub back_blue: u16,
  pub x: u16,
  pub y: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_font_enum_t
{
  XCB_FONT_NONE = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_glyph_cursor_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cid: xcb_cursor_t,
  pub source_font: xcb_font_t,
  pub mask_font: xcb_font_t,
  pub source_char: u16,
  pub mask_char: u16,
  pub fore_red: u16,
  pub fore_green: u16,
  pub fore_blue: u16,
  pub back_red: u16,
  pub back_green: u16,
  pub back_blue: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_cursor_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cursor: xcb_cursor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_recolor_cursor_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub cursor: xcb_cursor_t,
  pub fore_red: u16,
  pub fore_green: u16,
  pub fore_blue: u16,
  pub back_red: u16,
  pub back_green: u16,
  pub back_blue: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_query_shape_of_t
{
  XCB_QUERY_SHAPE_OF_LARGEST_CURSOR = 0,
  XCB_QUERY_SHAPE_OF_FASTEST_TILE = 1,
  XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_request_t
{
  pub major_opcode: u8,
  pub _class: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u16,
  pub height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub name_len: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub present: u8,
  pub major_opcode: u8,
  pub first_event: u8,
  pub first_error: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_reply_t
{
  pub response_type: u8,
  pub names_len: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_mapping_request_t
{
  pub major_opcode: u8,
  pub keycode_count: u8,
  pub length: u16,
  pub first_keycode: xcb_keycode_t,
  pub keysyms_per_keycode: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub first_keycode: xcb_keycode_t,
  pub count: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_reply_t
{
  pub response_type: u8,
  pub keysyms_per_keycode: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_kb_t
{
  XCB_KB_KEY_CLICK_PERCENT = 1,
  XCB_KB_BELL_PERCENT = 2,
  XCB_KB_BELL_PITCH = 4,
  XCB_KB_BELL_DURATION = 8,
  XCB_KB_LED = 16,
  XCB_KB_LED_MODE = 32,
  XCB_KB_KEY = 64,
  XCB_KB_AUTO_REPEAT_MODE = 128,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_led_mode_t
{
  XCB_LED_MODE_OFF = 0,
  XCB_LED_MODE_ON = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_auto_repeat_mode_t
{
  XCB_AUTO_REPEAT_MODE_OFF = 0,
  XCB_AUTO_REPEAT_MODE_ON = 1,
  XCB_AUTO_REPEAT_MODE_DEFAULT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_control_value_list_t
{
  pub key_click_percent: i32,
  pub bell_percent: i32,
  pub bell_pitch: i32,
  pub bell_duration: i32,
  pub led: u32,
  pub led_mode: u32,
  pub key: xcb_keycode32_t,
  pub auto_repeat_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_control_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_reply_t
{
  pub response_type: u8,
  pub global_auto_repeat: u8,
  pub sequence: u16,
  pub length: u32,
  pub led_mask: u32,
  pub key_click_percent: u8,
  pub bell_percent: u8,
  pub bell_pitch: u16,
  pub bell_duration: u16,
  pub pad0: [u8; 2usize],
  pub auto_repeats: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_bell_request_t
{
  pub major_opcode: u8,
  pub percent: i8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_pointer_control_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub acceleration_numerator: i16,
  pub acceleration_denominator: i16,
  pub threshold: i16,
  pub do_acceleration: u8,
  pub do_threshold: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub acceleration_numerator: u16,
  pub acceleration_denominator: u16,
  pub threshold: u16,
  pub pad1: [u8; 18usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_blanking_t
{
  XCB_BLANKING_NOT_PREFERRED = 0,
  XCB_BLANKING_PREFERRED = 1,
  XCB_BLANKING_DEFAULT = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_exposures_t
{
  XCB_EXPOSURES_NOT_ALLOWED = 0,
  XCB_EXPOSURES_ALLOWED = 1,
  XCB_EXPOSURES_DEFAULT = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_screen_saver_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub timeout: i16,
  pub interval: i16,
  pub prefer_blanking: u8,
  pub allow_exposures: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub timeout: u16,
  pub interval: u16,
  pub prefer_blanking: u8,
  pub allow_exposures: u8,
  pub pad1: [u8; 18usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_host_mode_t
{
  XCB_HOST_MODE_INSERT = 0,
  XCB_HOST_MODE_DELETE = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_family_t
{
  XCB_FAMILY_INTERNET = 0,
  XCB_FAMILY_DECNET = 1,
  XCB_FAMILY_CHAOS = 2,
  XCB_FAMILY_SERVER_INTERPRETED = 5,
  XCB_FAMILY_INTERNET_6 = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_hosts_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
  pub family: u8,
  pub pad0: u8,
  pub address_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_host_t
{
  pub family: u8,
  pub pad0: u8,
  pub address_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_host_iterator_t
{
  pub data: *mut xcb_host_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_reply_t
{
  pub response_type: u8,
  pub mode: u8,
  pub sequence: u16,
  pub length: u32,
  pub hosts_len: u16,
  pub pad0: [u8; 22usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_access_control_t
{
  XCB_ACCESS_CONTROL_DISABLE = 0,
  XCB_ACCESS_CONTROL_ENABLE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_access_control_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_close_down_t
{
  XCB_CLOSE_DOWN_DESTROY_ALL = 0,
  XCB_CLOSE_DOWN_RETAIN_PERMANENT = 1,
  XCB_CLOSE_DOWN_RETAIN_TEMPORARY = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_close_down_mode_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_kill_t
{
  XCB_KILL_ALL_TEMPORARY = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_kill_client_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub resource: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rotate_properties_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub atoms_len: u16,
  pub delta: i16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_screen_saver_t
{
  XCB_SCREEN_SAVER_RESET = 0,
  XCB_SCREEN_SAVER_ACTIVE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_force_screen_saver_request_t
{
  pub major_opcode: u8,
  pub mode: u8,
  pub length: u16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_mapping_status_t
{
  XCB_MAPPING_STATUS_SUCCESS = 0,
  XCB_MAPPING_STATUS_BUSY = 1,
  XCB_MAPPING_STATUS_FAILURE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_request_t
{
  pub major_opcode: u8,
  pub map_len: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_reply_t
{
  pub response_type: u8,
  pub map_len: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_map_index_t
{
  XCB_MAP_INDEX_SHIFT = 0,
  XCB_MAP_INDEX_LOCK = 1,
  XCB_MAP_INDEX_CONTROL = 2,
  XCB_MAP_INDEX_1 = 3,
  XCB_MAP_INDEX_2 = 4,
  XCB_MAP_INDEX_3 = 5,
  XCB_MAP_INDEX_4 = 6,
  XCB_MAP_INDEX_5 = 7,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_request_t
{
  pub major_opcode: u8,
  pub keycodes_per_modifier: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_reply_t
{
  pub response_type: u8,
  pub keycodes_per_modifier: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 24usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_no_operation_request_t
{
  pub major_opcode: u8,
  pub pad0: u8,
  pub length: u16,
}

#[link(name = "xcb")]
extern "C" {
  pub fn xcb_char2b_next(i: *mut xcb_char2b_iterator_t);

  pub fn xcb_char2b_end(i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_window_next(i: *mut xcb_window_iterator_t);

  pub fn xcb_window_end(i: xcb_window_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_pixmap_next(i: *mut xcb_pixmap_iterator_t);

  pub fn xcb_pixmap_end(i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_cursor_next(i: *mut xcb_cursor_iterator_t);

  pub fn xcb_cursor_end(i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_font_next(i: *mut xcb_font_iterator_t);

  pub fn xcb_font_end(i: xcb_font_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_gcontext_next(i: *mut xcb_gcontext_iterator_t);

  pub fn xcb_gcontext_end(i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_colormap_next(i: *mut xcb_colormap_iterator_t);

  pub fn xcb_colormap_end(i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_atom_next(i: *mut xcb_atom_iterator_t);

  pub fn xcb_atom_end(i: xcb_atom_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_drawable_next(i: *mut xcb_drawable_iterator_t);

  pub fn xcb_drawable_end(i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_fontable_next(i: *mut xcb_fontable_iterator_t);

  pub fn xcb_fontable_end(i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_bool32_next(i: *mut xcb_bool32_iterator_t);

  pub fn xcb_bool32_end(i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_visualid_next(i: *mut xcb_visualid_iterator_t);

  pub fn xcb_visualid_end(i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_timestamp_next(i: *mut xcb_timestamp_iterator_t);

  pub fn xcb_timestamp_end(i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_keysym_next(i: *mut xcb_keysym_iterator_t);

  pub fn xcb_keysym_end(i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_keycode_next(i: *mut xcb_keycode_iterator_t);

  pub fn xcb_keycode_end(i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_keycode32_next(i: *mut xcb_keycode32_iterator_t);

  pub fn xcb_keycode32_end(i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_button_next(i: *mut xcb_button_iterator_t);

  pub fn xcb_button_end(i: xcb_button_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_point_next(i: *mut xcb_point_iterator_t);

  pub fn xcb_point_end(i: xcb_point_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_rectangle_next(i: *mut xcb_rectangle_iterator_t);

  pub fn xcb_rectangle_end(i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_arc_next(i: *mut xcb_arc_iterator_t);

  pub fn xcb_arc_end(i: xcb_arc_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_format_next(i: *mut xcb_format_iterator_t);

  pub fn xcb_format_end(i: xcb_format_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);

  pub fn xcb_visualtype_end(i: xcb_visualtype_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_depth_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_depth_visuals(R: *const xcb_depth_t) -> *mut xcb_visualtype_t;

  pub fn xcb_depth_visuals_length(R: *const xcb_depth_t) -> ::std::os::raw::c_int;

  pub fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;

  pub fn xcb_depth_next(i: *mut xcb_depth_iterator_t);

  pub fn xcb_depth_end(i: xcb_depth_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_screen_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_screen_allowed_depths_length(R: *const xcb_screen_t) -> ::std::os::raw::c_int;

  pub fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;

  pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t);

  pub fn xcb_screen_end(i: xcb_screen_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_setup_request_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_setup_request_authorization_protocol_name(
    R: *const xcb_setup_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_setup_request_authorization_protocol_name_length(
    R: *const xcb_setup_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_setup_request_authorization_protocol_name_end(
    R: *const xcb_setup_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_setup_request_authorization_protocol_data(
    R: *const xcb_setup_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_setup_request_authorization_protocol_data_length(
    R: *const xcb_setup_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_setup_request_authorization_protocol_data_end(
    R: *const xcb_setup_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_setup_request_next(i: *mut xcb_setup_request_iterator_t);

  pub fn xcb_setup_request_end(i: xcb_setup_request_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_setup_failed_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_setup_failed_reason(R: *const xcb_setup_failed_t) -> *mut ::std::os::raw::c_char;

  pub fn xcb_setup_failed_reason_length(R: *const xcb_setup_failed_t) -> ::std::os::raw::c_int;

  pub fn xcb_setup_failed_reason_end(R: *const xcb_setup_failed_t) -> xcb_generic_iterator_t;

  pub fn xcb_setup_failed_next(i: *mut xcb_setup_failed_iterator_t);

  pub fn xcb_setup_failed_end(i: xcb_setup_failed_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_setup_authenticate_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_setup_authenticate_reason(
    R: *const xcb_setup_authenticate_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_setup_authenticate_reason_length(
    R: *const xcb_setup_authenticate_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_setup_authenticate_reason_end(
    R: *const xcb_setup_authenticate_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_setup_authenticate_next(i: *mut xcb_setup_authenticate_iterator_t);

  pub fn xcb_setup_authenticate_end(i: xcb_setup_authenticate_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_setup_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_setup_vendor(R: *const xcb_setup_t) -> *mut ::std::os::raw::c_char;

  pub fn xcb_setup_vendor_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;

  pub fn xcb_setup_vendor_end(R: *const xcb_setup_t) -> xcb_generic_iterator_t;

  pub fn xcb_setup_pixmap_formats(R: *const xcb_setup_t) -> *mut xcb_format_t;

  pub fn xcb_setup_pixmap_formats_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;

  pub fn xcb_setup_pixmap_formats_iterator(R: *const xcb_setup_t) -> xcb_format_iterator_t;

  pub fn xcb_setup_roots_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;

  pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;

  pub fn xcb_setup_next(i: *mut xcb_setup_iterator_t);

  pub fn xcb_setup_end(i: xcb_setup_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_client_message_data_next(i: *mut xcb_client_message_data_iterator_t);

  pub fn xcb_client_message_data_end(
    i: xcb_client_message_data_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_create_window_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_create_window_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_window_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_create_window_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_window_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_window_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_create_window_checked(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u16,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_window(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u16,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_window_aux_checked(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u16,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const xcb_create_window_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_window_aux(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    _class: u16,
    visual: xcb_visualid_t,
    value_mask: u32,
    value_list: *const xcb_create_window_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_window_value_list(
    R: *const xcb_create_window_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_change_window_attributes_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_change_window_attributes_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_window_attributes_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_change_window_attributes_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_window_attributes_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_window_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_window_attributes_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_window_attributes(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_window_attributes_aux_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u32,
    value_list: *const xcb_change_window_attributes_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_window_attributes_aux(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u32,
    value_list: *const xcb_change_window_attributes_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_window_attributes_value_list(
    R: *const xcb_change_window_attributes_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_get_window_attributes(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_window_attributes_cookie_t;

  pub fn xcb_get_window_attributes_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_window_attributes_cookie_t;

  pub fn xcb_get_window_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_window_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_window_attributes_reply_t;

  pub fn xcb_destroy_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_destroy_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_destroy_subwindows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_destroy_subwindows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_save_set_checked(
    c: *mut xcb_connection_t,
    mode: u8,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_save_set(
    c: *mut xcb_connection_t,
    mode: u8,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_reparent_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_reparent_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_map_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_map_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_map_subwindows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_map_subwindows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_unmap_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_unmap_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_unmap_subwindows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_unmap_subwindows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_configure_window_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u16,
    _aux: *const xcb_configure_window_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_configure_window_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u16,
    _aux: *mut xcb_configure_window_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_configure_window_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_configure_window_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_configure_window_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u16,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_configure_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u16,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_configure_window_aux_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u16,
    value_list: *const xcb_configure_window_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_configure_window_aux(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    value_mask: u16,
    value_list: *const xcb_configure_window_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_configure_window_value_list(
    R: *const xcb_configure_window_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_circulate_window_checked(
    c: *mut xcb_connection_t,
    direction: u8,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_circulate_window(
    c: *mut xcb_connection_t,
    direction: u8,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_geometry(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_get_geometry_cookie_t;

  pub fn xcb_get_geometry_unchecked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
  ) -> xcb_get_geometry_cookie_t;

  pub fn xcb_get_geometry_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_geometry_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_geometry_reply_t;

  pub fn xcb_query_tree_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_query_tree(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_query_tree_cookie_t;

  pub fn xcb_query_tree_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_query_tree_cookie_t;

  pub fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;

  pub fn xcb_query_tree_children_length(R: *const xcb_query_tree_reply_t) -> ::std::os::raw::c_int;

  pub fn xcb_query_tree_children_end(R: *const xcb_query_tree_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_query_tree_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_tree_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_tree_reply_t;

  pub fn xcb_intern_atom_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_intern_atom(
    c: *mut xcb_connection_t,
    only_if_exists: u8,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_intern_atom_cookie_t;

  pub fn xcb_intern_atom_unchecked(
    c: *mut xcb_connection_t,
    only_if_exists: u8,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_intern_atom_cookie_t;

  pub fn xcb_intern_atom_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_intern_atom_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_intern_atom_reply_t;

  pub fn xcb_get_atom_name_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_get_atom_name(
    c: *mut xcb_connection_t,
    atom: xcb_atom_t,
  ) -> xcb_get_atom_name_cookie_t;

  pub fn xcb_get_atom_name_unchecked(
    c: *mut xcb_connection_t,
    atom: xcb_atom_t,
  ) -> xcb_get_atom_name_cookie_t;

  pub fn xcb_get_atom_name_name(R: *const xcb_get_atom_name_reply_t)
    -> *mut ::std::os::raw::c_char;

  pub fn xcb_get_atom_name_name_length(
    R: *const xcb_get_atom_name_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_atom_name_name_end(R: *const xcb_get_atom_name_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_get_atom_name_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_atom_name_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_atom_name_reply_t;

  pub fn xcb_change_property_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_property_checked(
    c: *mut xcb_connection_t,
    mode: u8,
    window: xcb_window_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    data_len: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_property(
    c: *mut xcb_connection_t,
    mode: u8,
    window: xcb_window_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    format: u8,
    data_len: u32,
    data: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_property_data(
    R: *const xcb_change_property_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_change_property_data_length(
    R: *const xcb_change_property_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_property_data_end(
    R: *const xcb_change_property_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_delete_property_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_delete_property(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_property_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_get_property(
    c: *mut xcb_connection_t,
    _delete: u8,
    window: xcb_window_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_get_property_unchecked(
    c: *mut xcb_connection_t,
    _delete: u8,
    window: xcb_window_t,
    property: xcb_atom_t,
    type_: xcb_atom_t,
    long_offset: u32,
    long_length: u32,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::std::os::raw::c_void;

  pub fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t)
    -> ::std::os::raw::c_int;

  pub fn xcb_get_property_value_end(R: *const xcb_get_property_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_get_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_property_reply_t;

  pub fn xcb_list_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_properties(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_list_properties_cookie_t;

  pub fn xcb_list_properties_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_list_properties_cookie_t;

  pub fn xcb_list_properties_atoms(R: *const xcb_list_properties_reply_t) -> *mut xcb_atom_t;

  pub fn xcb_list_properties_atoms_length(
    R: *const xcb_list_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_properties_atoms_end(
    R: *const xcb_list_properties_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_list_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_properties_reply_t;

  pub fn xcb_set_selection_owner_checked(
    c: *mut xcb_connection_t,
    owner: xcb_window_t,
    selection: xcb_atom_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_selection_owner(
    c: *mut xcb_connection_t,
    owner: xcb_window_t,
    selection: xcb_atom_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_selection_owner(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_get_selection_owner_cookie_t;

  pub fn xcb_get_selection_owner_unchecked(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_get_selection_owner_cookie_t;

  pub fn xcb_get_selection_owner_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_selection_owner_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_selection_owner_reply_t;

  pub fn xcb_convert_selection_checked(
    c: *mut xcb_connection_t,
    requestor: xcb_window_t,
    selection: xcb_atom_t,
    target: xcb_atom_t,
    property: xcb_atom_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_convert_selection(
    c: *mut xcb_connection_t,
    requestor: xcb_window_t,
    selection: xcb_atom_t,
    target: xcb_atom_t,
    property: xcb_atom_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_send_event_checked(
    c: *mut xcb_connection_t,
    propagate: u8,
    destination: xcb_window_t,
    event_mask: u32,
    event: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_send_event(
    c: *mut xcb_connection_t,
    propagate: u8,
    destination: xcb_window_t,
    event_mask: u32,
    event: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_pointer(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    event_mask: u16,
    pointer_mode: u8,
    keyboard_mode: u8,
    confine_to: xcb_window_t,
    cursor: xcb_cursor_t,
    time: xcb_timestamp_t,
  ) -> xcb_grab_pointer_cookie_t;

  pub fn xcb_grab_pointer_unchecked(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    event_mask: u16,
    pointer_mode: u8,
    keyboard_mode: u8,
    confine_to: xcb_window_t,
    cursor: xcb_cursor_t,
    time: xcb_timestamp_t,
  ) -> xcb_grab_pointer_cookie_t;

  pub fn xcb_grab_pointer_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_grab_pointer_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_grab_pointer_reply_t;

  pub fn xcb_ungrab_pointer_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_pointer(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_button_checked(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    event_mask: u16,
    pointer_mode: u8,
    keyboard_mode: u8,
    confine_to: xcb_window_t,
    cursor: xcb_cursor_t,
    button: u8,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_button(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    event_mask: u16,
    pointer_mode: u8,
    keyboard_mode: u8,
    confine_to: xcb_window_t,
    cursor: xcb_cursor_t,
    button: u8,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_button_checked(
    c: *mut xcb_connection_t,
    button: u8,
    grab_window: xcb_window_t,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_button(
    c: *mut xcb_connection_t,
    button: u8,
    grab_window: xcb_window_t,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_active_pointer_grab_checked(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
    time: xcb_timestamp_t,
    event_mask: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_active_pointer_grab(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
    time: xcb_timestamp_t,
    event_mask: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_keyboard(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    time: xcb_timestamp_t,
    pointer_mode: u8,
    keyboard_mode: u8,
  ) -> xcb_grab_keyboard_cookie_t;

  pub fn xcb_grab_keyboard_unchecked(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    time: xcb_timestamp_t,
    pointer_mode: u8,
    keyboard_mode: u8,
  ) -> xcb_grab_keyboard_cookie_t;

  pub fn xcb_grab_keyboard_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_grab_keyboard_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_grab_keyboard_reply_t;

  pub fn xcb_ungrab_keyboard_checked(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_keyboard(
    c: *mut xcb_connection_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_key_checked(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    modifiers: u16,
    key: xcb_keycode_t,
    pointer_mode: u8,
    keyboard_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_key(
    c: *mut xcb_connection_t,
    owner_events: u8,
    grab_window: xcb_window_t,
    modifiers: u16,
    key: xcb_keycode_t,
    pointer_mode: u8,
    keyboard_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_key_checked(
    c: *mut xcb_connection_t,
    key: xcb_keycode_t,
    grab_window: xcb_window_t,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_key(
    c: *mut xcb_connection_t,
    key: xcb_keycode_t,
    grab_window: xcb_window_t,
    modifiers: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_allow_events_checked(
    c: *mut xcb_connection_t,
    mode: u8,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_allow_events(
    c: *mut xcb_connection_t,
    mode: u8,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_grab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_query_pointer(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_query_pointer_cookie_t;

  pub fn xcb_query_pointer_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_query_pointer_cookie_t;

  pub fn xcb_query_pointer_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_pointer_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_pointer_reply_t;

  pub fn xcb_timecoord_next(i: *mut xcb_timecoord_iterator_t);

  pub fn xcb_timecoord_end(i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_get_motion_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_motion_events(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    start: xcb_timestamp_t,
    stop: xcb_timestamp_t,
  ) -> xcb_get_motion_events_cookie_t;

  pub fn xcb_get_motion_events_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    start: xcb_timestamp_t,
    stop: xcb_timestamp_t,
  ) -> xcb_get_motion_events_cookie_t;

  pub fn xcb_get_motion_events_events(
    R: *const xcb_get_motion_events_reply_t
  ) -> *mut xcb_timecoord_t;

  pub fn xcb_get_motion_events_events_length(
    R: *const xcb_get_motion_events_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_motion_events_events_iterator(
    R: *const xcb_get_motion_events_reply_t
  ) -> xcb_timecoord_iterator_t;

  pub fn xcb_get_motion_events_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_motion_events_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_motion_events_reply_t;

  pub fn xcb_translate_coordinates(
    c: *mut xcb_connection_t,
    src_window: xcb_window_t,
    dst_window: xcb_window_t,
    src_x: i16,
    src_y: i16,
  ) -> xcb_translate_coordinates_cookie_t;

  pub fn xcb_translate_coordinates_unchecked(
    c: *mut xcb_connection_t,
    src_window: xcb_window_t,
    dst_window: xcb_window_t,
    src_x: i16,
    src_y: i16,
  ) -> xcb_translate_coordinates_cookie_t;

  pub fn xcb_translate_coordinates_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_translate_coordinates_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_translate_coordinates_reply_t;

  pub fn xcb_warp_pointer_checked(
    c: *mut xcb_connection_t,
    src_window: xcb_window_t,
    dst_window: xcb_window_t,
    src_x: i16,
    src_y: i16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_warp_pointer(
    c: *mut xcb_connection_t,
    src_window: xcb_window_t,
    dst_window: xcb_window_t,
    src_x: i16,
    src_y: i16,
    src_width: u16,
    src_height: u16,
    dst_x: i16,
    dst_y: i16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_input_focus_checked(
    c: *mut xcb_connection_t,
    revert_to: u8,
    focus: xcb_window_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_input_focus(
    c: *mut xcb_connection_t,
    revert_to: u8,
    focus: xcb_window_t,
    time: xcb_timestamp_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;

  pub fn xcb_get_input_focus_unchecked(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;

  pub fn xcb_get_input_focus_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_input_focus_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_input_focus_reply_t;

  pub fn xcb_query_keymap(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;

  pub fn xcb_query_keymap_unchecked(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;

  pub fn xcb_query_keymap_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_keymap_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_keymap_reply_t;

  pub fn xcb_open_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_open_font_checked(
    c: *mut xcb_connection_t,
    fid: xcb_font_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_open_font(
    c: *mut xcb_connection_t,
    fid: xcb_font_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_open_font_name(R: *const xcb_open_font_request_t) -> *mut ::std::os::raw::c_char;

  pub fn xcb_open_font_name_length(R: *const xcb_open_font_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_open_font_name_end(R: *const xcb_open_font_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_close_font_checked(
    c: *mut xcb_connection_t,
    font: xcb_font_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_close_font(
    c: *mut xcb_connection_t,
    font: xcb_font_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_fontprop_next(i: *mut xcb_fontprop_iterator_t);

  pub fn xcb_fontprop_end(i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_charinfo_next(i: *mut xcb_charinfo_iterator_t);

  pub fn xcb_charinfo_end(i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_query_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_query_font(
    c: *mut xcb_connection_t,
    font: xcb_fontable_t,
  ) -> xcb_query_font_cookie_t;

  pub fn xcb_query_font_unchecked(
    c: *mut xcb_connection_t,
    font: xcb_fontable_t,
  ) -> xcb_query_font_cookie_t;

  pub fn xcb_query_font_properties(R: *const xcb_query_font_reply_t) -> *mut xcb_fontprop_t;

  pub fn xcb_query_font_properties_length(
    R: *const xcb_query_font_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_font_properties_iterator(
    R: *const xcb_query_font_reply_t
  ) -> xcb_fontprop_iterator_t;

  pub fn xcb_query_font_char_infos(R: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t;

  pub fn xcb_query_font_char_infos_length(
    R: *const xcb_query_font_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_font_char_infos_iterator(
    R: *const xcb_query_font_reply_t
  ) -> xcb_charinfo_iterator_t;

  pub fn xcb_query_font_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_font_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_font_reply_t;

  pub fn xcb_query_text_extents_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    string_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_text_extents(
    c: *mut xcb_connection_t,
    font: xcb_fontable_t,
    string_len: u32,
    string: *const xcb_char2b_t,
  ) -> xcb_query_text_extents_cookie_t;

  pub fn xcb_query_text_extents_unchecked(
    c: *mut xcb_connection_t,
    font: xcb_fontable_t,
    string_len: u32,
    string: *const xcb_char2b_t,
  ) -> xcb_query_text_extents_cookie_t;

  pub fn xcb_query_text_extents_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_text_extents_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_text_extents_reply_t;

  pub fn xcb_str_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_str_name(R: *const xcb_str_t) -> *mut ::std::os::raw::c_char;

  pub fn xcb_str_name_length(R: *const xcb_str_t) -> ::std::os::raw::c_int;

  pub fn xcb_str_name_end(R: *const xcb_str_t) -> xcb_generic_iterator_t;

  pub fn xcb_str_next(i: *mut xcb_str_iterator_t);

  pub fn xcb_str_end(i: xcb_str_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_list_fonts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_list_fonts(
    c: *mut xcb_connection_t,
    max_names: u16,
    pattern_len: u16,
    pattern: *const ::std::os::raw::c_char,
  ) -> xcb_list_fonts_cookie_t;

  pub fn xcb_list_fonts_unchecked(
    c: *mut xcb_connection_t,
    max_names: u16,
    pattern_len: u16,
    pattern: *const ::std::os::raw::c_char,
  ) -> xcb_list_fonts_cookie_t;

  pub fn xcb_list_fonts_names_length(R: *const xcb_list_fonts_reply_t) -> ::std::os::raw::c_int;

  pub fn xcb_list_fonts_names_iterator(R: *const xcb_list_fonts_reply_t) -> xcb_str_iterator_t;

  pub fn xcb_list_fonts_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_fonts_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_fonts_reply_t;

  pub fn xcb_list_fonts_with_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_fonts_with_info(
    c: *mut xcb_connection_t,
    max_names: u16,
    pattern_len: u16,
    pattern: *const ::std::os::raw::c_char,
  ) -> xcb_list_fonts_with_info_cookie_t;

  pub fn xcb_list_fonts_with_info_unchecked(
    c: *mut xcb_connection_t,
    max_names: u16,
    pattern_len: u16,
    pattern: *const ::std::os::raw::c_char,
  ) -> xcb_list_fonts_with_info_cookie_t;

  pub fn xcb_list_fonts_with_info_properties(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> *mut xcb_fontprop_t;

  pub fn xcb_list_fonts_with_info_properties_length(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_fonts_with_info_properties_iterator(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> xcb_fontprop_iterator_t;

  pub fn xcb_list_fonts_with_info_name(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_list_fonts_with_info_name_length(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_fonts_with_info_name_end(
    R: *const xcb_list_fonts_with_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_list_fonts_with_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_fonts_with_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_fonts_with_info_reply_t;

  pub fn xcb_set_font_path_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_set_font_path_checked(
    c: *mut xcb_connection_t,
    font_qty: u16,
    font: *const xcb_str_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_font_path(
    c: *mut xcb_connection_t,
    font_qty: u16,
    font: *const xcb_str_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_font_path_font_length(
    R: *const xcb_set_font_path_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_set_font_path_font_iterator(
    R: *const xcb_set_font_path_request_t
  ) -> xcb_str_iterator_t;

  pub fn xcb_get_font_path_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_get_font_path(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;

  pub fn xcb_get_font_path_unchecked(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;

  pub fn xcb_get_font_path_path_length(
    R: *const xcb_get_font_path_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_font_path_path_iterator(R: *const xcb_get_font_path_reply_t)
    -> xcb_str_iterator_t;

  pub fn xcb_get_font_path_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_font_path_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_font_path_reply_t;

  pub fn xcb_create_pixmap_checked(
    c: *mut xcb_connection_t,
    depth: u8,
    pid: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_pixmap(
    c: *mut xcb_connection_t,
    depth: u8,
    pid: xcb_pixmap_t,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_pixmap_checked(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_pixmap(
    c: *mut xcb_connection_t,
    pixmap: xcb_pixmap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_gc_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_create_gc_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_gc_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_create_gc_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_gc_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_create_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_create_gc_checked(
    c: *mut xcb_connection_t,
    cid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_gc(
    c: *mut xcb_connection_t,
    cid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_gc_aux_checked(
    c: *mut xcb_connection_t,
    cid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    value_mask: u32,
    value_list: *const xcb_create_gc_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_gc_aux(
    c: *mut xcb_connection_t,
    cid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    value_mask: u32,
    value_list: *const xcb_create_gc_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_gc_value_list(R: *const xcb_create_gc_request_t)
    -> *mut ::std::os::raw::c_void;

  pub fn xcb_change_gc_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_change_gc_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_gc_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_change_gc_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_gc_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_change_gc_checked(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_gc(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_gc_aux_checked(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    value_mask: u32,
    value_list: *const xcb_change_gc_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_gc_aux(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    value_mask: u32,
    value_list: *const xcb_change_gc_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_gc_value_list(R: *const xcb_change_gc_request_t)
    -> *mut ::std::os::raw::c_void;

  pub fn xcb_copy_gc_checked(
    c: *mut xcb_connection_t,
    src_gc: xcb_gcontext_t,
    dst_gc: xcb_gcontext_t,
    value_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_gc(
    c: *mut xcb_connection_t,
    src_gc: xcb_gcontext_t,
    dst_gc: xcb_gcontext_t,
    value_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_dashes_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_set_dashes_checked(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    dash_offset: u16,
    dashes_len: u16,
    dashes: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_dashes(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    dash_offset: u16,
    dashes_len: u16,
    dashes: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_dashes_dashes(R: *const xcb_set_dashes_request_t) -> *mut u8;

  pub fn xcb_set_dashes_dashes_length(R: *const xcb_set_dashes_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_set_dashes_dashes_end(R: *const xcb_set_dashes_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_set_clip_rectangles_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rectangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_set_clip_rectangles_checked(
    c: *mut xcb_connection_t,
    ordering: u8,
    gc: xcb_gcontext_t,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_clip_rectangles(
    c: *mut xcb_connection_t,
    ordering: u8,
    gc: xcb_gcontext_t,
    clip_x_origin: i16,
    clip_y_origin: i16,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_clip_rectangles_rectangles(
    R: *const xcb_set_clip_rectangles_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_set_clip_rectangles_rectangles_length(
    R: *const xcb_set_clip_rectangles_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_set_clip_rectangles_rectangles_iterator(
    R: *const xcb_set_clip_rectangles_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_free_gc_checked(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_gc(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_clear_area_checked(
    c: *mut xcb_connection_t,
    exposures: u8,
    window: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_clear_area(
    c: *mut xcb_connection_t,
    exposures: u8,
    window: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_area_checked(
    c: *mut xcb_connection_t,
    src_drawable: xcb_drawable_t,
    dst_drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    src_x: i16,
    src_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_area(
    c: *mut xcb_connection_t,
    src_drawable: xcb_drawable_t,
    dst_drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    src_x: i16,
    src_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_plane_checked(
    c: *mut xcb_connection_t,
    src_drawable: xcb_drawable_t,
    dst_drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    src_x: i16,
    src_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
    bit_plane: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_plane(
    c: *mut xcb_connection_t,
    src_drawable: xcb_drawable_t,
    dst_drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    src_x: i16,
    src_y: i16,
    dst_x: i16,
    dst_y: i16,
    width: u16,
    height: u16,
    bit_plane: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_point_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    points_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_point_checked(
    c: *mut xcb_connection_t,
    coordinate_mode: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_point(
    c: *mut xcb_connection_t,
    coordinate_mode: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_point_points(R: *const xcb_poly_point_request_t) -> *mut xcb_point_t;

  pub fn xcb_poly_point_points_length(R: *const xcb_poly_point_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_poly_point_points_iterator(R: *const xcb_poly_point_request_t)
    -> xcb_point_iterator_t;

  pub fn xcb_poly_line_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    points_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_line_checked(
    c: *mut xcb_connection_t,
    coordinate_mode: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_line(
    c: *mut xcb_connection_t,
    coordinate_mode: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_line_points(R: *const xcb_poly_line_request_t) -> *mut xcb_point_t;

  pub fn xcb_poly_line_points_length(R: *const xcb_poly_line_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_poly_line_points_iterator(R: *const xcb_poly_line_request_t) -> xcb_point_iterator_t;

  pub fn xcb_segment_next(i: *mut xcb_segment_iterator_t);

  pub fn xcb_segment_end(i: xcb_segment_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_poly_segment_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    segments_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_segment_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    segments_len: u32,
    segments: *const xcb_segment_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_segment(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    segments_len: u32,
    segments: *const xcb_segment_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_segment_segments(R: *const xcb_poly_segment_request_t) -> *mut xcb_segment_t;

  pub fn xcb_poly_segment_segments_length(
    R: *const xcb_poly_segment_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_segment_segments_iterator(
    R: *const xcb_poly_segment_request_t
  ) -> xcb_segment_iterator_t;

  pub fn xcb_poly_rectangle_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rectangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_rectangle_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_rectangle(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_rectangle_rectangles(
    R: *const xcb_poly_rectangle_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_poly_rectangle_rectangles_length(
    R: *const xcb_poly_rectangle_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_rectangle_rectangles_iterator(
    R: *const xcb_poly_rectangle_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_poly_arc_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    arcs_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_arc_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    arcs_len: u32,
    arcs: *const xcb_arc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_arc(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    arcs_len: u32,
    arcs: *const xcb_arc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_arc_arcs(R: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t;

  pub fn xcb_poly_arc_arcs_length(R: *const xcb_poly_arc_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_poly_arc_arcs_iterator(R: *const xcb_poly_arc_request_t) -> xcb_arc_iterator_t;

  pub fn xcb_fill_poly_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    points_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_fill_poly_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    shape: u8,
    coordinate_mode: u8,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_fill_poly(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    shape: u8,
    coordinate_mode: u8,
    points_len: u32,
    points: *const xcb_point_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_fill_poly_points(R: *const xcb_fill_poly_request_t) -> *mut xcb_point_t;

  pub fn xcb_fill_poly_points_length(R: *const xcb_fill_poly_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_fill_poly_points_iterator(R: *const xcb_fill_poly_request_t) -> xcb_point_iterator_t;

  pub fn xcb_poly_fill_rectangle_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    rectangles_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_fill_rectangle_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_fill_rectangle(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    rectangles_len: u32,
    rectangles: *const xcb_rectangle_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_fill_rectangle_rectangles(
    R: *const xcb_poly_fill_rectangle_request_t
  ) -> *mut xcb_rectangle_t;

  pub fn xcb_poly_fill_rectangle_rectangles_length(
    R: *const xcb_poly_fill_rectangle_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_fill_rectangle_rectangles_iterator(
    R: *const xcb_poly_fill_rectangle_request_t
  ) -> xcb_rectangle_iterator_t;

  pub fn xcb_poly_fill_arc_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    arcs_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_fill_arc_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    arcs_len: u32,
    arcs: *const xcb_arc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_fill_arc(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    arcs_len: u32,
    arcs: *const xcb_arc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_fill_arc_arcs(R: *const xcb_poly_fill_arc_request_t) -> *mut xcb_arc_t;

  pub fn xcb_poly_fill_arc_arcs_length(
    R: *const xcb_poly_fill_arc_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_fill_arc_arcs_iterator(
    R: *const xcb_poly_fill_arc_request_t
  ) -> xcb_arc_iterator_t;

  pub fn xcb_put_image_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    data_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_put_image_checked(
    c: *mut xcb_connection_t,
    format: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    width: u16,
    height: u16,
    dst_x: i16,
    dst_y: i16,
    left_pad: u8,
    depth: u8,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_put_image(
    c: *mut xcb_connection_t,
    format: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    width: u16,
    height: u16,
    dst_x: i16,
    dst_y: i16,
    left_pad: u8,
    depth: u8,
    data_len: u32,
    data: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_put_image_data(R: *const xcb_put_image_request_t) -> *mut u8;

  pub fn xcb_put_image_data_length(R: *const xcb_put_image_request_t) -> ::std::os::raw::c_int;

  pub fn xcb_put_image_data_end(R: *const xcb_put_image_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_get_image_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_get_image(
    c: *mut xcb_connection_t,
    format: u8,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
  ) -> xcb_get_image_cookie_t;

  pub fn xcb_get_image_unchecked(
    c: *mut xcb_connection_t,
    format: u8,
    drawable: xcb_drawable_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    plane_mask: u32,
  ) -> xcb_get_image_cookie_t;

  pub fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut u8;

  pub fn xcb_get_image_data_length(R: *const xcb_get_image_reply_t) -> ::std::os::raw::c_int;

  pub fn xcb_get_image_data_end(R: *const xcb_get_image_reply_t) -> xcb_generic_iterator_t;

  pub fn xcb_get_image_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_image_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_image_reply_t;

  pub fn xcb_poly_text_8_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    items_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_text_8_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    items_len: u32,
    items: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_text_8(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    items_len: u32,
    items: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_text_8_items(R: *const xcb_poly_text_8_request_t) -> *mut u8;

  pub fn xcb_poly_text_8_items_length(R: *const xcb_poly_text_8_request_t)
    -> ::std::os::raw::c_int;

  pub fn xcb_poly_text_8_items_end(R: *const xcb_poly_text_8_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_poly_text_16_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    items_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_text_16_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    items_len: u32,
    items: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_text_16(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    items_len: u32,
    items: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_poly_text_16_items(R: *const xcb_poly_text_16_request_t) -> *mut u8;

  pub fn xcb_poly_text_16_items_length(
    R: *const xcb_poly_text_16_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_poly_text_16_items_end(R: *const xcb_poly_text_16_request_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_image_text_8_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_image_text_8_checked(
    c: *mut xcb_connection_t,
    string_len: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_image_text_8(
    c: *mut xcb_connection_t,
    string_len: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    string: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_image_text_8_string(
    R: *const xcb_image_text_8_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_image_text_8_string_length(
    R: *const xcb_image_text_8_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_image_text_8_string_end(
    R: *const xcb_image_text_8_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_image_text_16_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_image_text_16_checked(
    c: *mut xcb_connection_t,
    string_len: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    string: *const xcb_char2b_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_image_text_16(
    c: *mut xcb_connection_t,
    string_len: u8,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    x: i16,
    y: i16,
    string: *const xcb_char2b_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_image_text_16_string(R: *const xcb_image_text_16_request_t) -> *mut xcb_char2b_t;

  pub fn xcb_image_text_16_string_length(
    R: *const xcb_image_text_16_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_image_text_16_string_iterator(
    R: *const xcb_image_text_16_request_t
  ) -> xcb_char2b_iterator_t;

  pub fn xcb_create_colormap_checked(
    c: *mut xcb_connection_t,
    alloc: u8,
    mid: xcb_colormap_t,
    window: xcb_window_t,
    visual: xcb_visualid_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_colormap(
    c: *mut xcb_connection_t,
    alloc: u8,
    mid: xcb_colormap_t,
    window: xcb_window_t,
    visual: xcb_visualid_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_colormap_checked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_colormap(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_colormap_and_free_checked(
    c: *mut xcb_connection_t,
    mid: xcb_colormap_t,
    src_cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_copy_colormap_and_free(
    c: *mut xcb_connection_t,
    mid: xcb_colormap_t,
    src_cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_install_colormap_checked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_install_colormap(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_uninstall_colormap_checked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_uninstall_colormap(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_list_installed_colormaps_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_installed_colormaps(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_list_installed_colormaps_cookie_t;

  pub fn xcb_list_installed_colormaps_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_list_installed_colormaps_cookie_t;

  pub fn xcb_list_installed_colormaps_cmaps(
    R: *const xcb_list_installed_colormaps_reply_t
  ) -> *mut xcb_colormap_t;

  pub fn xcb_list_installed_colormaps_cmaps_length(
    R: *const xcb_list_installed_colormaps_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_installed_colormaps_cmaps_end(
    R: *const xcb_list_installed_colormaps_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_list_installed_colormaps_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_installed_colormaps_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_installed_colormaps_reply_t;

  pub fn xcb_alloc_color(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    red: u16,
    green: u16,
    blue: u16,
  ) -> xcb_alloc_color_cookie_t;

  pub fn xcb_alloc_color_unchecked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    red: u16,
    green: u16,
    blue: u16,
  ) -> xcb_alloc_color_cookie_t;

  pub fn xcb_alloc_color_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_alloc_color_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_alloc_color_reply_t;

  pub fn xcb_alloc_named_color_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_named_color(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_alloc_named_color_cookie_t;

  pub fn xcb_alloc_named_color_unchecked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_alloc_named_color_cookie_t;

  pub fn xcb_alloc_named_color_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_alloc_named_color_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_alloc_named_color_reply_t;

  pub fn xcb_alloc_color_cells_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_color_cells(
    c: *mut xcb_connection_t,
    contiguous: u8,
    cmap: xcb_colormap_t,
    colors: u16,
    planes: u16,
  ) -> xcb_alloc_color_cells_cookie_t;

  pub fn xcb_alloc_color_cells_unchecked(
    c: *mut xcb_connection_t,
    contiguous: u8,
    cmap: xcb_colormap_t,
    colors: u16,
    planes: u16,
  ) -> xcb_alloc_color_cells_cookie_t;

  pub fn xcb_alloc_color_cells_pixels(R: *const xcb_alloc_color_cells_reply_t) -> *mut u32;

  pub fn xcb_alloc_color_cells_pixels_length(
    R: *const xcb_alloc_color_cells_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_color_cells_pixels_end(
    R: *const xcb_alloc_color_cells_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_alloc_color_cells_masks(R: *const xcb_alloc_color_cells_reply_t) -> *mut u32;

  pub fn xcb_alloc_color_cells_masks_length(
    R: *const xcb_alloc_color_cells_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_color_cells_masks_end(
    R: *const xcb_alloc_color_cells_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_alloc_color_cells_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_alloc_color_cells_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_alloc_color_cells_reply_t;

  pub fn xcb_alloc_color_planes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_color_planes(
    c: *mut xcb_connection_t,
    contiguous: u8,
    cmap: xcb_colormap_t,
    colors: u16,
    reds: u16,
    greens: u16,
    blues: u16,
  ) -> xcb_alloc_color_planes_cookie_t;

  pub fn xcb_alloc_color_planes_unchecked(
    c: *mut xcb_connection_t,
    contiguous: u8,
    cmap: xcb_colormap_t,
    colors: u16,
    reds: u16,
    greens: u16,
    blues: u16,
  ) -> xcb_alloc_color_planes_cookie_t;

  pub fn xcb_alloc_color_planes_pixels(R: *const xcb_alloc_color_planes_reply_t) -> *mut u32;

  pub fn xcb_alloc_color_planes_pixels_length(
    R: *const xcb_alloc_color_planes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_alloc_color_planes_pixels_end(
    R: *const xcb_alloc_color_planes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_alloc_color_planes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_alloc_color_planes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_alloc_color_planes_reply_t;

  pub fn xcb_free_colors_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    pixels_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_free_colors_checked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    plane_mask: u32,
    pixels_len: u32,
    pixels: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_colors(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    plane_mask: u32,
    pixels_len: u32,
    pixels: *const u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_colors_pixels(R: *const xcb_free_colors_request_t) -> *mut u32;

  pub fn xcb_free_colors_pixels_length(
    R: *const xcb_free_colors_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_free_colors_pixels_end(R: *const xcb_free_colors_request_t) -> xcb_generic_iterator_t;

  pub fn xcb_coloritem_next(i: *mut xcb_coloritem_iterator_t);

  pub fn xcb_coloritem_end(i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_store_colors_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    items_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_store_colors_checked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    items_len: u32,
    items: *const xcb_coloritem_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_store_colors(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    items_len: u32,
    items: *const xcb_coloritem_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_store_colors_items(R: *const xcb_store_colors_request_t) -> *mut xcb_coloritem_t;

  pub fn xcb_store_colors_items_length(
    R: *const xcb_store_colors_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_store_colors_items_iterator(
    R: *const xcb_store_colors_request_t
  ) -> xcb_coloritem_iterator_t;

  pub fn xcb_store_named_color_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_store_named_color_checked(
    c: *mut xcb_connection_t,
    flags: u8,
    cmap: xcb_colormap_t,
    pixel: u32,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_store_named_color(
    c: *mut xcb_connection_t,
    flags: u8,
    cmap: xcb_colormap_t,
    pixel: u32,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_store_named_color_name(
    R: *const xcb_store_named_color_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_store_named_color_name_length(
    R: *const xcb_store_named_color_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_store_named_color_name_end(
    R: *const xcb_store_named_color_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_rgb_next(i: *mut xcb_rgb_iterator_t);

  pub fn xcb_rgb_end(i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_query_colors_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    pixels_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_colors(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    pixels_len: u32,
    pixels: *const u32,
  ) -> xcb_query_colors_cookie_t;

  pub fn xcb_query_colors_unchecked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    pixels_len: u32,
    pixels: *const u32,
  ) -> xcb_query_colors_cookie_t;

  pub fn xcb_query_colors_colors(R: *const xcb_query_colors_reply_t) -> *mut xcb_rgb_t;

  pub fn xcb_query_colors_colors_length(
    R: *const xcb_query_colors_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_colors_colors_iterator(R: *const xcb_query_colors_reply_t)
    -> xcb_rgb_iterator_t;

  pub fn xcb_query_colors_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_colors_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_colors_reply_t;

  pub fn xcb_lookup_color_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_lookup_color(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_lookup_color_cookie_t;

  pub fn xcb_lookup_color_unchecked(
    c: *mut xcb_connection_t,
    cmap: xcb_colormap_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_lookup_color_cookie_t;

  pub fn xcb_lookup_color_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_lookup_color_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_lookup_color_reply_t;

  pub fn xcb_create_cursor_checked(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source: xcb_pixmap_t,
    mask: xcb_pixmap_t,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
    x: u16,
    y: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_cursor(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source: xcb_pixmap_t,
    mask: xcb_pixmap_t,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
    x: u16,
    y: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_glyph_cursor_checked(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source_font: xcb_font_t,
    mask_font: xcb_font_t,
    source_char: u16,
    mask_char: u16,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_create_glyph_cursor(
    c: *mut xcb_connection_t,
    cid: xcb_cursor_t,
    source_font: xcb_font_t,
    mask_font: xcb_font_t,
    source_char: u16,
    mask_char: u16,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_cursor_checked(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_free_cursor(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_recolor_cursor_checked(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_recolor_cursor(
    c: *mut xcb_connection_t,
    cursor: xcb_cursor_t,
    fore_red: u16,
    fore_green: u16,
    fore_blue: u16,
    back_red: u16,
    back_green: u16,
    back_blue: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_query_best_size(
    c: *mut xcb_connection_t,
    _class: u8,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
  ) -> xcb_query_best_size_cookie_t;

  pub fn xcb_query_best_size_unchecked(
    c: *mut xcb_connection_t,
    _class: u8,
    drawable: xcb_drawable_t,
    width: u16,
    height: u16,
  ) -> xcb_query_best_size_cookie_t;

  pub fn xcb_query_best_size_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_best_size_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_best_size_reply_t;

  pub fn xcb_query_extension_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_query_extension(
    c: *mut xcb_connection_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_query_extension_cookie_t;

  pub fn xcb_query_extension_unchecked(
    c: *mut xcb_connection_t,
    name_len: u16,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_query_extension_cookie_t;

  pub fn xcb_query_extension_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_query_extension_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_query_extension_reply_t;

  pub fn xcb_list_extensions_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_extensions(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;

  pub fn xcb_list_extensions_unchecked(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;

  pub fn xcb_list_extensions_names_length(
    R: *const xcb_list_extensions_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_list_extensions_names_iterator(
    R: *const xcb_list_extensions_reply_t
  ) -> xcb_str_iterator_t;

  pub fn xcb_list_extensions_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_extensions_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_extensions_reply_t;

  pub fn xcb_change_keyboard_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_mapping_checked(
    c: *mut xcb_connection_t,
    keycode_count: u8,
    first_keycode: xcb_keycode_t,
    keysyms_per_keycode: u8,
    keysyms: *const xcb_keysym_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_mapping(
    c: *mut xcb_connection_t,
    keycode_count: u8,
    first_keycode: xcb_keycode_t,
    keysyms_per_keycode: u8,
    keysyms: *const xcb_keysym_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_mapping_keysyms(
    R: *const xcb_change_keyboard_mapping_request_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_change_keyboard_mapping_keysyms_length(
    R: *const xcb_change_keyboard_mapping_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_mapping_keysyms_end(
    R: *const xcb_change_keyboard_mapping_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_get_keyboard_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_keyboard_mapping(
    c: *mut xcb_connection_t,
    first_keycode: xcb_keycode_t,
    count: u8,
  ) -> xcb_get_keyboard_mapping_cookie_t;

  pub fn xcb_get_keyboard_mapping_unchecked(
    c: *mut xcb_connection_t,
    first_keycode: xcb_keycode_t,
    count: u8,
  ) -> xcb_get_keyboard_mapping_cookie_t;

  pub fn xcb_get_keyboard_mapping_keysyms(
    R: *const xcb_get_keyboard_mapping_reply_t
  ) -> *mut xcb_keysym_t;

  pub fn xcb_get_keyboard_mapping_keysyms_length(
    R: *const xcb_get_keyboard_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_keyboard_mapping_keysyms_end(
    R: *const xcb_get_keyboard_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_get_keyboard_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_keyboard_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_keyboard_mapping_reply_t;

  pub fn xcb_change_keyboard_control_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_change_keyboard_control_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_control_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_change_keyboard_control_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_control_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_control_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_keyboard_control_checked(
    c: *mut xcb_connection_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_control(
    c: *mut xcb_connection_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_control_aux_checked(
    c: *mut xcb_connection_t,
    value_mask: u32,
    value_list: *const xcb_change_keyboard_control_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_control_aux(
    c: *mut xcb_connection_t,
    value_mask: u32,
    value_list: *const xcb_change_keyboard_control_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_keyboard_control_value_list(
    R: *const xcb_change_keyboard_control_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_get_keyboard_control(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t;

  pub fn xcb_get_keyboard_control_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_get_keyboard_control_cookie_t;

  pub fn xcb_get_keyboard_control_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_keyboard_control_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_keyboard_control_reply_t;

  pub fn xcb_bell_checked(
    c: *mut xcb_connection_t,
    percent: i8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_bell(
    c: *mut xcb_connection_t,
    percent: i8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_pointer_control_checked(
    c: *mut xcb_connection_t,
    acceleration_numerator: i16,
    acceleration_denominator: i16,
    threshold: i16,
    do_acceleration: u8,
    do_threshold: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_pointer_control(
    c: *mut xcb_connection_t,
    acceleration_numerator: i16,
    acceleration_denominator: i16,
    threshold: i16,
    do_acceleration: u8,
    do_threshold: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_pointer_control(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t;

  pub fn xcb_get_pointer_control_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_get_pointer_control_cookie_t;

  pub fn xcb_get_pointer_control_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_pointer_control_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_pointer_control_reply_t;

  pub fn xcb_set_screen_saver_checked(
    c: *mut xcb_connection_t,
    timeout: i16,
    interval: i16,
    prefer_blanking: u8,
    allow_exposures: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_screen_saver(
    c: *mut xcb_connection_t,
    timeout: i16,
    interval: i16,
    prefer_blanking: u8,
    allow_exposures: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_get_screen_saver(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t;

  pub fn xcb_get_screen_saver_unchecked(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t;

  pub fn xcb_get_screen_saver_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_screen_saver_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_screen_saver_reply_t;

  pub fn xcb_change_hosts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_change_hosts_checked(
    c: *mut xcb_connection_t,
    mode: u8,
    family: u8,
    address_len: u16,
    address: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_hosts(
    c: *mut xcb_connection_t,
    mode: u8,
    family: u8,
    address_len: u16,
    address: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_change_hosts_address(R: *const xcb_change_hosts_request_t) -> *mut u8;

  pub fn xcb_change_hosts_address_length(
    R: *const xcb_change_hosts_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_change_hosts_address_end(
    R: *const xcb_change_hosts_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_host_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_host_address(R: *const xcb_host_t) -> *mut u8;

  pub fn xcb_host_address_length(R: *const xcb_host_t) -> ::std::os::raw::c_int;

  pub fn xcb_host_address_end(R: *const xcb_host_t) -> xcb_generic_iterator_t;

  pub fn xcb_host_next(i: *mut xcb_host_iterator_t);

  pub fn xcb_host_end(i: xcb_host_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_list_hosts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_list_hosts(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;

  pub fn xcb_list_hosts_unchecked(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;

  pub fn xcb_list_hosts_hosts_length(R: *const xcb_list_hosts_reply_t) -> ::std::os::raw::c_int;

  pub fn xcb_list_hosts_hosts_iterator(R: *const xcb_list_hosts_reply_t) -> xcb_host_iterator_t;

  pub fn xcb_list_hosts_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_list_hosts_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_list_hosts_reply_t;

  pub fn xcb_set_access_control_checked(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_access_control(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_close_down_mode_checked(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_close_down_mode(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_kill_client_checked(
    c: *mut xcb_connection_t,
    resource: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_kill_client(
    c: *mut xcb_connection_t,
    resource: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_rotate_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_rotate_properties_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    atoms_len: u16,
    delta: i16,
    atoms: *const xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_rotate_properties(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    atoms_len: u16,
    delta: i16,
    atoms: *const xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_rotate_properties_atoms(R: *const xcb_rotate_properties_request_t) -> *mut xcb_atom_t;

  pub fn xcb_rotate_properties_atoms_length(
    R: *const xcb_rotate_properties_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_rotate_properties_atoms_end(
    R: *const xcb_rotate_properties_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_force_screen_saver_checked(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_force_screen_saver(
    c: *mut xcb_connection_t,
    mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_set_pointer_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_set_pointer_mapping(
    c: *mut xcb_connection_t,
    map_len: u8,
    map: *const u8,
  ) -> xcb_set_pointer_mapping_cookie_t;

  pub fn xcb_set_pointer_mapping_unchecked(
    c: *mut xcb_connection_t,
    map_len: u8,
    map: *const u8,
  ) -> xcb_set_pointer_mapping_cookie_t;

  pub fn xcb_set_pointer_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_set_pointer_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_set_pointer_mapping_reply_t;

  pub fn xcb_get_pointer_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_pointer_mapping(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t;

  pub fn xcb_get_pointer_mapping_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_get_pointer_mapping_cookie_t;

  pub fn xcb_get_pointer_mapping_map(R: *const xcb_get_pointer_mapping_reply_t) -> *mut u8;

  pub fn xcb_get_pointer_mapping_map_length(
    R: *const xcb_get_pointer_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_pointer_mapping_map_end(
    R: *const xcb_get_pointer_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_get_pointer_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_pointer_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_pointer_mapping_reply_t;

  pub fn xcb_set_modifier_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_set_modifier_mapping(
    c: *mut xcb_connection_t,
    keycodes_per_modifier: u8,
    keycodes: *const xcb_keycode_t,
  ) -> xcb_set_modifier_mapping_cookie_t;

  pub fn xcb_set_modifier_mapping_unchecked(
    c: *mut xcb_connection_t,
    keycodes_per_modifier: u8,
    keycodes: *const xcb_keycode_t,
  ) -> xcb_set_modifier_mapping_cookie_t;

  pub fn xcb_set_modifier_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_set_modifier_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_set_modifier_mapping_reply_t;

  pub fn xcb_get_modifier_mapping_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_modifier_mapping(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;

  pub fn xcb_get_modifier_mapping_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_get_modifier_mapping_cookie_t;

  pub fn xcb_get_modifier_mapping_keycodes(
    R: *const xcb_get_modifier_mapping_reply_t
  ) -> *mut xcb_keycode_t;

  pub fn xcb_get_modifier_mapping_keycodes_length(
    R: *const xcb_get_modifier_mapping_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_get_modifier_mapping_keycodes_end(
    R: *const xcb_get_modifier_mapping_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_get_modifier_mapping_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_modifier_mapping_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_get_modifier_mapping_reply_t;

  pub fn xcb_no_operation_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_no_operation(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
