use super::core::{xcb_connection_t, xcb_generic_error_t, xcb_void_cookie_t};
use super::xcb::{
  xcb_atom_t,
  xcb_get_property_cookie_t,
  xcb_get_property_reply_t,
  xcb_gravity_t,
  xcb_pixmap_t,
  xcb_window_t,
};

pub const XCB_ICCCM_NUM_WM_SIZE_HINTS_ELEMENTS: u32 = 18;
pub const XCB_ICCCM_NUM_WM_HINTS_ELEMENTS: u32 = 9;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_icccm_get_text_property_reply_t
{
  pub _reply: *mut xcb_get_property_reply_t,
  pub encoding: xcb_atom_t,
  pub name_len: u32,
  pub name: *mut ::std::os::raw::c_char,
  pub format: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_icccm_get_wm_colormap_windows_reply_t
{
  pub windows_len: u32,
  pub windows: *mut xcb_window_t,
  pub _reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_icccm_get_wm_class_reply_t
{
  pub instance_name: *mut ::std::os::raw::c_char,
  pub class_name: *mut ::std::os::raw::c_char,
  pub _reply: *mut xcb_get_property_reply_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_icccm_size_hints_flags_t
{
  US_POSITION = 1,
  US_SIZE = 2,
  P_POSITION = 4,
  P_SIZE = 8,
  P_MIN_SIZE = 16,
  P_MAX_SIZE = 32,
  P_RESIZE_INC = 64,
  P_ASPECT = 128,
  BASE_SIZE = 256,
  P_WIN_GRAVITY = 512,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_size_hints_t
{
  pub flags: u32,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub min_width: i32,
  pub min_height: i32,
  pub max_width: i32,
  pub max_height: i32,
  pub width_inc: i32,
  pub height_inc: i32,
  pub min_aspect_num: i32,
  pub min_aspect_den: i32,
  pub max_aspect_num: i32,
  pub max_aspect_den: i32,
  pub base_width: i32,
  pub base_height: i32,
  pub win_gravity: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_icccm_wm_hints_t
{
  pub flags: i32,
  pub input: u32,
  pub initial_state: i32,
  pub icon_pixmap: xcb_pixmap_t,
  pub icon_window: xcb_window_t,
  pub icon_x: i32,
  pub icon_y: i32,
  pub icon_mask: xcb_pixmap_t,
  pub window_group: xcb_window_t,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_icccm_wm_state_t
{
  WITHDRAWN = 0,
  NORMAL = 1,
  ICONIC = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_icccm_wm_t
{
  HINT_INPUT = 1,
  HINT_STATE = 2,
  HINT_ICON_PIXMAP = 4,
  HINT_ICON_WINDOW = 8,
  HINT_ICON_POSITION = 16,
  HINT_ICON_MASK = 32,
  HINT_WINDOW_GROUP = 64,
  HINT_X_URGENCY = 256,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_icccm_get_wm_protocols_reply_t
{
  pub atoms_len: u32,
  pub atoms: *mut xcb_atom_t,
  pub _reply: *mut xcb_get_property_reply_t,
}

#[link(name = "xcb-icccm")]
extern "C" {
  pub fn xcb_icccm_get_text_property(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_text_property_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_text_property_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_icccm_get_text_property_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_get_text_property_reply_wipe(prop: *mut xcb_icccm_get_text_property_reply_t);

  pub fn xcb_icccm_set_wm_name_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_name(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_name(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_name_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_name_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_icccm_get_text_property_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_set_wm_icon_name_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_icon_name(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_icon_name(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_icon_name_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_icon_name_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_icccm_get_text_property_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_set_wm_colormap_windows_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_colormap_windows_atom: xcb_atom_t,
    list_len: u32,
    list: *const xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_colormap_windows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_colormap_windows_atom: xcb_atom_t,
    list_len: u32,
    list: *const xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_colormap_windows(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_colormap_windows_atom: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_colormap_windows_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_colormap_windows_atom: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_colormap_windows_from_reply(
    reply: *mut xcb_get_property_reply_t,
    colormap_windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_colormap_windows_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_colormap_windows_reply_wipe(
    windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t
  );

  pub fn xcb_icccm_set_wm_client_machine_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_client_machine(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    encoding: xcb_atom_t,
    format: u8,
    name_len: u32,
    name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_client_machine(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_client_machine_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_client_machine_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_icccm_get_text_property_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_set_wm_class_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    class_len: u32,
    class_name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_class(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    class_len: u32,
    class_name: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_class(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_class_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_class_from_reply(
    prop: *mut xcb_icccm_get_wm_class_reply_t,
    reply: *mut xcb_get_property_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_class_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_icccm_get_wm_class_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_class_reply_wipe(prop: *mut xcb_icccm_get_wm_class_reply_t);

  pub fn xcb_icccm_set_wm_transient_for_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    transient_for_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_transient_for(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    transient_for_window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_transient_for(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_transient_for_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_transient_for_from_reply(
    prop: *mut xcb_window_t,
    reply: *mut xcb_get_property_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_transient_for_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    prop: *mut xcb_window_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_size_hints_set_position(
    hints: *mut xcb_size_hints_t,
    user_specified: ::std::os::raw::c_int,
    x: i32,
    y: i32,
  );

  pub fn xcb_icccm_size_hints_set_size(
    hints: *mut xcb_size_hints_t,
    user_specified: ::std::os::raw::c_int,
    width: i32,
    height: i32,
  );

  pub fn xcb_icccm_size_hints_set_min_size(
    hints: *mut xcb_size_hints_t,
    min_width: i32,
    min_height: i32,
  );

  pub fn xcb_icccm_size_hints_set_max_size(
    hints: *mut xcb_size_hints_t,
    max_width: i32,
    max_height: i32,
  );

  pub fn xcb_icccm_size_hints_set_resize_inc(
    hints: *mut xcb_size_hints_t,
    width_inc: i32,
    height_inc: i32,
  );

  pub fn xcb_icccm_size_hints_set_aspect(
    hints: *mut xcb_size_hints_t,
    min_aspect_num: i32,
    min_aspect_den: i32,
    max_aspect_num: i32,
    max_aspect_den: i32,
  );

  pub fn xcb_icccm_size_hints_set_base_size(
    hints: *mut xcb_size_hints_t,
    base_width: i32,
    base_height: i32,
  );

  pub fn xcb_icccm_size_hints_set_win_gravity(
    hints: *mut xcb_size_hints_t,
    win_gravity: xcb_gravity_t,
  );

  pub fn xcb_icccm_set_wm_size_hints_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
    hints: *mut xcb_size_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_size_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
    hints: *mut xcb_size_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_size_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_size_hints_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_size_hints_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    hints: *mut xcb_size_hints_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_set_wm_normal_hints_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    hints: *mut xcb_size_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_normal_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    hints: *mut xcb_size_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_normal_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_normal_hints_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_size_hints_from_reply(
    hints: *mut xcb_size_hints_t,
    reply: *mut xcb_get_property_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_normal_hints_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    hints: *mut xcb_size_hints_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_wm_hints_get_urgency(hints: *mut xcb_icccm_wm_hints_t) -> u32;

  pub fn xcb_icccm_wm_hints_set_input(
    hints: *mut xcb_icccm_wm_hints_t,
    input: u8,
  );

  pub fn xcb_icccm_wm_hints_set_iconic(hints: *mut xcb_icccm_wm_hints_t);

  pub fn xcb_icccm_wm_hints_set_normal(hints: *mut xcb_icccm_wm_hints_t);

  pub fn xcb_icccm_wm_hints_set_withdrawn(hints: *mut xcb_icccm_wm_hints_t);

  pub fn xcb_icccm_wm_hints_set_none(hints: *mut xcb_icccm_wm_hints_t);

  pub fn xcb_icccm_wm_hints_set_icon_pixmap(
    hints: *mut xcb_icccm_wm_hints_t,
    icon_pixmap: xcb_pixmap_t,
  );

  pub fn xcb_icccm_wm_hints_set_icon_mask(
    hints: *mut xcb_icccm_wm_hints_t,
    icon_mask: xcb_pixmap_t,
  );

  pub fn xcb_icccm_wm_hints_set_icon_window(
    hints: *mut xcb_icccm_wm_hints_t,
    icon_window: xcb_window_t,
  );

  pub fn xcb_icccm_wm_hints_set_window_group(
    hints: *mut xcb_icccm_wm_hints_t,
    window_group: xcb_window_t,
  );

  pub fn xcb_icccm_wm_hints_set_urgency(hints: *mut xcb_icccm_wm_hints_t);

  pub fn xcb_icccm_set_wm_hints_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    hints: *mut xcb_icccm_wm_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    hints: *mut xcb_icccm_wm_hints_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_hints(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_hints_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_hints_from_reply(
    hints: *mut xcb_icccm_wm_hints_t,
    reply: *mut xcb_get_property_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_hints_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    hints: *mut xcb_icccm_wm_hints_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_set_wm_protocols_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_protocols: xcb_atom_t,
    list_len: u32,
    list: *mut xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_set_wm_protocols(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_protocols: xcb_atom_t,
    list_len: u32,
    list: *mut xcb_atom_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_icccm_get_wm_protocols(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_protocol_atom: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_protocols_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    wm_protocol_atom: xcb_atom_t,
  ) -> xcb_get_property_cookie_t;

  pub fn xcb_icccm_get_wm_protocols_from_reply(
    reply: *mut xcb_get_property_reply_t,
    protocols: *mut xcb_icccm_get_wm_protocols_reply_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_protocols_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_get_property_cookie_t,
    protocols: *mut xcb_icccm_get_wm_protocols_reply_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> u8;

  pub fn xcb_icccm_get_wm_protocols_reply_wipe(protocols: *mut xcb_icccm_get_wm_protocols_reply_t);
}
