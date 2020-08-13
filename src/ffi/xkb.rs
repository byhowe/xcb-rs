use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{
  xcb_atom_t,
  xcb_keycode_t,
  xcb_keysym_t,
  xcb_point_iterator_t,
  xcb_point_t,
  xcb_timestamp_t,
  xcb_window_t,
};

pub const XCB_XKB_MAJOR_VERSION: u32 = 1;
pub const XCB_XKB_MINOR_VERSION: u32 = 0;
pub const XCB_XKB_KEYBOARD: u32 = 0;
pub const XCB_XKB_USE_EXTENSION: u32 = 0;
pub const XCB_XKB_SELECT_EVENTS: u32 = 1;
pub const XCB_XKB_BELL: u32 = 3;
pub const XCB_XKB_GET_STATE: u32 = 4;
pub const XCB_XKB_LATCH_LOCK_STATE: u32 = 5;
pub const XCB_XKB_GET_CONTROLS: u32 = 6;
pub const XCB_XKB_SET_CONTROLS: u32 = 7;
pub const XCB_XKB_GET_MAP: u32 = 8;
pub const XCB_XKB_SET_MAP: u32 = 9;
pub const XCB_XKB_GET_COMPAT_MAP: u32 = 10;
pub const XCB_XKB_SET_COMPAT_MAP: u32 = 11;
pub const XCB_XKB_GET_INDICATOR_STATE: u32 = 12;
pub const XCB_XKB_GET_INDICATOR_MAP: u32 = 13;
pub const XCB_XKB_SET_INDICATOR_MAP: u32 = 14;
pub const XCB_XKB_GET_NAMED_INDICATOR: u32 = 15;
pub const XCB_XKB_SET_NAMED_INDICATOR: u32 = 16;
pub const XCB_XKB_GET_NAMES: u32 = 17;
pub const XCB_XKB_SET_NAMES: u32 = 18;
pub const XCB_XKB_PER_CLIENT_FLAGS: u32 = 21;
pub const XCB_XKB_LIST_COMPONENTS: u32 = 22;
pub const XCB_XKB_GET_KBD_BY_NAME: u32 = 23;
pub const XCB_XKB_GET_DEVICE_INFO: u32 = 24;
pub const XCB_XKB_SET_DEVICE_INFO: u32 = 25;
pub const XCB_XKB_SET_DEBUGGING_FLAGS: u32 = 101;
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: u32 = 0;
pub const XCB_XKB_MAP_NOTIFY: u32 = 1;
pub const XCB_XKB_STATE_NOTIFY: u32 = 2;
pub const XCB_XKB_CONTROLS_NOTIFY: u32 = 3;
pub const XCB_XKB_INDICATOR_STATE_NOTIFY: u32 = 4;
pub const XCB_XKB_INDICATOR_MAP_NOTIFY: u32 = 5;
pub const XCB_XKB_NAMES_NOTIFY: u32 = 6;
pub const XCB_XKB_COMPAT_MAP_NOTIFY: u32 = 7;
pub const XCB_XKB_BELL_NOTIFY: u32 = 8;
pub const XCB_XKB_ACTION_MESSAGE: u32 = 9;
pub const XCB_XKB_ACCESS_X_NOTIFY: u32 = 10;
pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY: u32 = 11;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_const_t
{
  MAX_LEGAL_KEY_CODE = 255,
  PER_KEY_BIT_ARRAY_SIZE = 32,
  KEY_NAME_LENGTH = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_event_type_t
{
  NEW_KEYBOARD_NOTIFY = 1,
  MAP_NOTIFY = 2,
  STATE_NOTIFY = 4,
  CONTROLS_NOTIFY = 8,
  INDICATOR_STATE_NOTIFY = 16,
  INDICATOR_MAP_NOTIFY = 32,
  NAMES_NOTIFY = 64,
  COMPAT_MAP_NOTIFY = 128,
  BELL_NOTIFY = 256,
  ACTION_MESSAGE = 512,
  ACCESS_X_NOTIFY = 1024,
  EXTENSION_DEVICE_NOTIFY = 2048,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_nkn_detail_t
{
  KEYCODES = 1,
  GEOMETRY = 2,
  DEVICE_ID = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_axn_detail_t
{
  SK_PRESS = 1,
  SK_ACCEPT = 2,
  SK_REJECT = 4,
  SK_RELEASE = 8,
  BK_ACCEPT = 16,
  BK_REJECT = 32,
  AXK_WARNING = 64,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_map_part_t
{
  KEY_TYPES = 1,
  KEY_SYMS = 2,
  MODIFIER_MAP = 4,
  EXPLICIT_COMPONENTS = 8,
  KEY_ACTIONS = 16,
  KEY_BEHAVIORS = 32,
  VIRTUAL_MODS = 64,
  VIRTUAL_MOD_MAP = 128,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_set_map_flags_t
{
  RESIZE_TYPES = 1,
  RECOMPUTE_ACTIONS = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_state_part_t
{
  MODIFIER_STATE = 1,
  MODIFIER_BASE = 2,
  MODIFIER_LATCH = 4,
  MODIFIER_LOCK = 8,
  GROUP_STATE = 16,
  GROUP_BASE = 32,
  GROUP_LATCH = 64,
  GROUP_LOCK = 128,
  COMPAT_STATE = 256,
  GRAB_MODS = 512,
  COMPAT_GRAB_MODS = 1024,
  LOOKUP_MODS = 2048,
  COMPAT_LOOKUP_MODS = 4096,
  POINTER_BUTTONS = 8192,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_bool_ctrl_t
{
  REPEAT_KEYS = 1,
  SLOW_KEYS = 2,
  BOUNCE_KEYS = 4,
  STICKY_KEYS = 8,
  MOUSE_KEYS = 16,
  MOUSE_KEYS_ACCEL = 32,
  ACCESS_X_KEYS = 64,
  ACCESS_X_TIMEOUT_MASK = 128,
  ACCESS_X_FEEDBACK_MASK = 256,
  AUDIBLE_BELL_MASK = 512,
  OVERLAY_1_MASK = 1024,
  OVERLAY_2_MASK = 2048,
  IGNORE_GROUP_LOCK_MASK = 4096,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_control_t
{
  GROUPS_WRAP = 134217728,
  INTERNAL_MODS = 268435456,
  IGNORE_LOCK_MODS = 536870912,
  PER_KEY_REPEAT = 1073741824,
  CONTROLS_ENABLED = 2147483648,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_ax_option_t
{
  SK_PRESS_FB = 1,
  SK_ACCEPT_FB = 2,
  FEATURE_FB = 4,
  SLOW_WARN_FB = 8,
  INDICATOR_FB = 16,
  STICKY_KEYS_FB = 32,
  TWO_KEYS = 64,
  LATCH_TO_LOCK = 128,
  SK_RELEASE_FB = 256,
  SK_REJECT_FB = 512,
  BK_REJECT_FB = 1024,
  DUMB_BELL = 2048,
}

pub type xcb_xkb_device_spec_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_device_spec_iterator_t
{
  pub data: *mut xcb_xkb_device_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_led_class_result_t
{
  KBD_FEEDBACK_CLASS = 0,
  LED_FEEDBACK_CLASS = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_led_class_t
{
  KBD_FEEDBACK_CLASS = 0,
  LED_FEEDBACK_CLASS = 4,
  DFLT_XI_CLASS = 768,
  ALL_XI_CLASSES = 1280,
}

pub type xcb_xkb_led_class_spec_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_led_class_spec_iterator_t
{
  pub data: *mut xcb_xkb_led_class_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_bell_class_result_t
{
  KBD_FEEDBACK_CLASS = 0,
  BELL_FEEDBACK_CLASS = 5,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_bell_class_t
{
  KBD_FEEDBACK_CLASS = 0,
  BELL_FEEDBACK_CLASS = 5,
  DFLT_XI_CLASS = 768,
}

pub type xcb_xkb_bell_class_spec_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_bell_class_spec_iterator_t
{
  pub data: *mut xcb_xkb_bell_class_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_id_t
{
  USE_CORE_KBD = 256,
  USE_CORE_PTR = 512,
  DFLT_XI_CLASS = 768,
  DFLT_XI_ID = 1024,
  ALL_XI_CLASS = 1280,
  ALL_XI_ID = 1536,
  XI_NONE = 65280,
}

pub type xcb_xkb_id_spec_t = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_id_spec_iterator_t
{
  pub data: *mut xcb_xkb_id_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_group_t
{
  GROUP_1 = 0,
  GROUP_2 = 1,
  GROUP_3 = 2,
  GROUP_4 = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_groups_t
{
  ANY = 254,
  ALL = 255,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_set_of_group_t
{
  GROUP_1 = 1,
  GROUP_2 = 2,
  GROUP_3 = 4,
  GROUP_4 = 8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_set_of_groups_t
{
  ANY = 128,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_groups_wrap_t
{
  WRAP_INTO_RANGE = 0,
  CLAMP_INTO_RANGE = 64,
  REDIRECT_INTO_RANGE = 128,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_v_mods_high_t
{
  HIGH_15 = 128,
  HIGH_14 = 64,
  HIGH_13 = 32,
  HIGH_12 = 16,
  HIGH_11 = 8,
  HIGH_10 = 4,
  HIGH_9 = 2,
  HIGH_8 = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_v_mods_low_t
{
  LOW_7 = 128,
  LOW_6 = 64,
  LOW_5 = 32,
  LOW_4 = 16,
  LOW_3 = 8,
  LOW_2 = 4,
  LOW_1 = 2,
  LOW_0 = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_v_mod_t
{
  MOD_15 = 32768,
  MOD_14 = 16384,
  MOD_13 = 8192,
  MOD_12 = 4096,
  MOD_11 = 2048,
  MOD_10 = 1024,
  MOD_9 = 512,
  MOD_8 = 256,
  MOD_7 = 128,
  MOD_6 = 64,
  MOD_5 = 32,
  MOD_4 = 16,
  MOD_3 = 8,
  MOD_2 = 4,
  MOD_1 = 2,
  MOD_0 = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_explicit_t
{
  V_MOD_MAP = 128,
  BEHAVIOR = 64,
  AUTO_REPEAT = 32,
  INTERPRET = 16,
  KEY_TYPE_4 = 8,
  KEY_TYPE_3 = 4,
  KEY_TYPE_2 = 2,
  KEY_TYPE_1 = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sym_interpret_match_t
{
  NONE_OF = 0,
  ANY_OF_OR_NONE = 1,
  ANY_OF = 2,
  ALL_OF = 3,
  EXACTLY = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sym_interp_match_t
{
  LEVEL_ONE_ONLY = 128,
  OP_MASK = 127,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_im_flag_t
{
  NO_EXPLICIT = 128,
  NO_AUTOMATIC = 64,
  LED_DRIVES_KB = 32,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_im_mods_which_t
{
  USE_COMPAT = 16,
  USE_EFFECTIVE = 8,
  USE_LOCKED = 4,
  USE_LATCHED = 2,
  USE_BASE = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_im_groups_which_t
{
  USE_COMPAT = 16,
  USE_EFFECTIVE = 8,
  USE_LOCKED = 4,
  USE_LATCHED = 2,
  USE_BASE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_indicator_map_t
{
  pub flags: u8,
  pub whichGroups: u8,
  pub groups: u8,
  pub whichMods: u8,
  pub mods: u8,
  pub realMods: u8,
  pub vmods: u16,
  pub ctrls: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_indicator_map_iterator_t
{
  pub data: *mut xcb_xkb_indicator_map_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_cm_detail_t
{
  SYM_INTERP = 1,
  GROUP_COMPAT = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_name_detail_t
{
  KEYCODES = 1,
  GEOMETRY = 2,
  SYMBOLS = 4,
  PHYS_SYMBOLS = 8,
  TYPES = 16,
  COMPAT = 32,
  KEY_TYPE_NAMES = 64,
  KT_LEVEL_NAMES = 128,
  INDICATOR_NAMES = 256,
  KEY_NAMES = 512,
  KEY_ALIASES = 1024,
  VIRTUAL_MOD_NAMES = 2048,
  GROUP_NAMES = 4096,
  RG_NAMES = 8192,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_gbn_detail_t
{
  TYPES = 1,
  COMPAT_MAP = 2,
  CLIENT_SYMBOLS = 4,
  SERVER_SYMBOLS = 8,
  INDICATOR_MAPS = 16,
  KEY_NAMES = 32,
  GEOMETRY = 64,
  OTHER_NAMES = 128,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_xi_feature_t
{
  KEYBOARDS = 1,
  BUTTON_ACTIONS = 2,
  INDICATOR_NAMES = 4,
  INDICATOR_MAPS = 8,
  INDICATOR_STATE = 16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_per_client_flag_t
{
  DETECTABLE_AUTO_REPEAT = 1,
  GRABS_USE_XKB_STATE = 2,
  AUTO_RESET_CONTROLS = 4,
  LOOKUP_STATE_WHEN_GRABBED = 8,
  SEND_EVENT_USES_XKB_STATE = 16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_mod_def_t
{
  pub mask: u8,
  pub realMods: u8,
  pub vmods: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_mod_def_iterator_t
{
  pub data: *mut xcb_xkb_mod_def_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_name_t
{
  pub name: [::std::os::raw::c_char; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_name_iterator_t
{
  pub data: *mut xcb_xkb_key_name_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_alias_t
{
  pub real: [::std::os::raw::c_char; 4usize],
  pub alias: [::std::os::raw::c_char; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_alias_iterator_t
{
  pub data: *mut xcb_xkb_key_alias_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_counted_string_16_t
{
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_counted_string_16_iterator_t
{
  pub data: *mut xcb_xkb_counted_string_16_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_kt_map_entry_t
{
  pub active: u8,
  pub mods_mask: u8,
  pub level: u8,
  pub mods_mods: u8,
  pub mods_vmods: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_kt_map_entry_iterator_t
{
  pub data: *mut xcb_xkb_kt_map_entry_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_type_t
{
  pub mods_mask: u8,
  pub mods_mods: u8,
  pub mods_vmods: u16,
  pub numLevels: u8,
  pub nMapEntries: u8,
  pub hasPreserve: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_type_iterator_t
{
  pub data: *mut xcb_xkb_key_type_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_sym_map_t
{
  pub kt_index: [u8; 4usize],
  pub groupInfo: u8,
  pub width: u8,
  pub nSyms: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_sym_map_iterator_t
{
  pub data: *mut xcb_xkb_key_sym_map_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_common_behavior_t
{
  pub type_: u8,
  pub data: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_common_behavior_iterator_t
{
  pub data: *mut xcb_xkb_common_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_default_behavior_t
{
  pub type_: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_default_behavior_iterator_t
{
  pub data: *mut xcb_xkb_default_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_lock_behavior_t
{
  pub type_: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_lock_behavior_iterator_t
{
  pub data: *mut xcb_xkb_lock_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_radio_group_behavior_t
{
  pub type_: u8,
  pub group: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_radio_group_behavior_iterator_t
{
  pub data: *mut xcb_xkb_radio_group_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_behavior_t
{
  pub type_: u8,
  pub key: xcb_keycode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_behavior_iterator_t
{
  pub data: *mut xcb_xkb_overlay_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_lock_behavior_t
{
  pub type_: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t
{
  pub data: *mut xcb_xkb_permament_lock_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_radio_group_behavior_t
{
  pub type_: u8,
  pub group: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t
{
  pub data: *mut xcb_xkb_permament_radio_group_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_overlay_behavior_t
{
  pub type_: u8,
  pub key: xcb_keycode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_permament_overlay_behavior_iterator_t
{
  pub data: *mut xcb_xkb_permament_overlay_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_xkb_behavior_t
{
  pub common: xcb_xkb_common_behavior_t,
  pub _default: xcb_xkb_default_behavior_t,
  pub lock: xcb_xkb_lock_behavior_t,
  pub radioGroup: xcb_xkb_radio_group_behavior_t,
  pub overlay1: xcb_xkb_overlay_behavior_t,
  pub overlay2: xcb_xkb_overlay_behavior_t,
  pub permamentLock: xcb_xkb_permament_lock_behavior_t,
  pub permamentRadioGroup: xcb_xkb_permament_radio_group_behavior_t,
  pub permamentOverlay1: xcb_xkb_permament_overlay_behavior_t,
  pub permamentOverlay2: xcb_xkb_permament_overlay_behavior_t,
  pub type_: u8,
  _bindgen_union_align: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_behavior_iterator_t
{
  pub data: *mut xcb_xkb_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_behavior_type_t
{
  DEFAULT = 0,
  LOCK = 1,
  RADIO_GROUP = 2,
  OVERLAY_1 = 3,
  OVERLAY_2 = 4,
  PERMAMENT_LOCK = 129,
  PERMAMENT_RADIO_GROUP = 130,
  PERMAMENT_OVERLAY_1 = 131,
  PERMAMENT_OVERLAY_2 = 132,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_xkb_set_behavior_t
{
  pub keycode: xcb_keycode_t,
  pub behavior: xcb_xkb_behavior_t,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_behavior_iterator_t
{
  pub data: *mut xcb_xkb_set_behavior_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_explicit_t
{
  pub keycode: xcb_keycode_t,
  pub explicit: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_explicit_iterator_t
{
  pub data: *mut xcb_xkb_set_explicit_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_mod_map_t
{
  pub keycode: xcb_keycode_t,
  pub mods: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_mod_map_iterator_t
{
  pub data: *mut xcb_xkb_key_mod_map_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_v_mod_map_t
{
  pub keycode: xcb_keycode_t,
  pub pad0: u8,
  pub vmods: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_v_mod_map_iterator_t
{
  pub data: *mut xcb_xkb_key_v_mod_map_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_kt_set_map_entry_t
{
  pub level: u8,
  pub realMods: u8,
  pub virtualMods: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t
{
  pub data: *mut xcb_xkb_kt_set_map_entry_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_key_type_t
{
  pub mask: u8,
  pub realMods: u8,
  pub virtualMods: u16,
  pub numLevels: u8,
  pub nMapEntries: u8,
  pub preserve: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_key_type_iterator_t
{
  pub data: *mut xcb_xkb_set_key_type_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_xkb_string8_t = ::std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_string8_iterator_t
{
  pub data: *mut xcb_xkb_string8_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_outline_t
{
  pub nPoints: u8,
  pub cornerRadius: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_outline_iterator_t
{
  pub data: *mut xcb_xkb_outline_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_shape_t
{
  pub name: xcb_atom_t,
  pub nOutlines: u8,
  pub primaryNdx: u8,
  pub approxNdx: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_shape_iterator_t
{
  pub data: *mut xcb_xkb_shape_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_t
{
  pub name: [xcb_xkb_string8_t; 4usize],
  pub gap: i16,
  pub shapeNdx: u8,
  pub colorNdx: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_key_iterator_t
{
  pub data: *mut xcb_xkb_key_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_key_t
{
  pub over: [xcb_xkb_string8_t; 4usize],
  pub under: [xcb_xkb_string8_t; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_key_iterator_t
{
  pub data: *mut xcb_xkb_overlay_key_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_row_t
{
  pub rowUnder: u8,
  pub nKeys: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_row_iterator_t
{
  pub data: *mut xcb_xkb_overlay_row_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_t
{
  pub name: xcb_atom_t,
  pub nRows: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_overlay_iterator_t
{
  pub data: *mut xcb_xkb_overlay_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_row_t
{
  pub top: i16,
  pub left: i16,
  pub nKeys: u8,
  pub vertical: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_row_iterator_t
{
  pub data: *mut xcb_xkb_row_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_doodad_type_t
{
  OUTLINE = 1,
  SOLID = 2,
  TEXT = 3,
  INDICATOR = 4,
  LOGO = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_listing_t
{
  pub flags: u16,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_listing_iterator_t
{
  pub data: *mut xcb_xkb_listing_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_device_led_info_t
{
  pub ledClass: xcb_xkb_led_class_spec_t,
  pub ledID: xcb_xkb_id_spec_t,
  pub namesPresent: u32,
  pub mapsPresent: u32,
  pub physIndicators: u32,
  pub state: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_device_led_info_iterator_t
{
  pub data: *mut xcb_xkb_device_led_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_error_t
{
  BAD_DEVICE = 255,
  BAD_CLASS = 254,
  BAD_ID = 253,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_keyboard_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub value: u32,
  pub minorOpcode: u16,
  pub majorOpcode: u8,
  pub pad0: [u8; 21usize],
}

impl xcb_xkb_sa_t
{
  pub const GROUP_ABSOLUTE: xcb_xkb_sa_t = xcb_xkb_sa_t::USE_MOD_MAP_MODS;
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_t
{
  CLEAR_LOCKS = 1,
  LATCH_TO_LOCK = 2,
  USE_MOD_MAP_MODS = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_type_t
{
  NO_ACTION = 0,
  SET_MODS = 1,
  LATCH_MODS = 2,
  LOCK_MODS = 3,
  SET_GROUP = 4,
  LATCH_GROUP = 5,
  LOCK_GROUP = 6,
  MOVE_PTR = 7,
  PTR_BTN = 8,
  LOCK_PTR_BTN = 9,
  SET_PTR_DFLT = 10,
  ISO_LOCK = 11,
  TERMINATE = 12,
  SWITCH_SCREEN = 13,
  SET_CONTROLS = 14,
  LOCK_CONTROLS = 15,
  ACTION_MESSAGE = 16,
  REDIRECT_KEY = 17,
  DEVICE_BTN = 18,
  LOCK_DEVICE_BTN = 19,
  DEVICE_VALUATOR = 20,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_no_action_t
{
  pub type_: u8,
  pub pad0: [u8; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_no_action_iterator_t
{
  pub data: *mut xcb_xkb_sa_no_action_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_mods_t
{
  pub type_: u8,
  pub flags: u8,
  pub mask: u8,
  pub realMods: u8,
  pub vmodsHigh: u8,
  pub vmodsLow: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_mods_iterator_t
{
  pub data: *mut xcb_xkb_sa_set_mods_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_latch_mods_t
{
  pub type_: u8,
  pub flags: u8,
  pub mask: u8,
  pub realMods: u8,
  pub vmodsHigh: u8,
  pub vmodsLow: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_latch_mods_iterator_t
{
  pub data: *mut xcb_xkb_sa_latch_mods_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_mods_t
{
  pub type_: u8,
  pub flags: u8,
  pub mask: u8,
  pub realMods: u8,
  pub vmodsHigh: u8,
  pub vmodsLow: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_mods_iterator_t
{
  pub data: *mut xcb_xkb_sa_lock_mods_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_group_t
{
  pub type_: u8,
  pub flags: u8,
  pub group: i8,
  pub pad0: [u8; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_group_iterator_t
{
  pub data: *mut xcb_xkb_sa_set_group_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_latch_group_t
{
  pub type_: u8,
  pub flags: u8,
  pub group: i8,
  pub pad0: [u8; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_latch_group_iterator_t
{
  pub data: *mut xcb_xkb_sa_latch_group_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_group_t
{
  pub type_: u8,
  pub flags: u8,
  pub group: i8,
  pub pad0: [u8; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_group_iterator_t
{
  pub data: *mut xcb_xkb_sa_lock_group_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_move_ptr_flag_t
{
  NO_ACCELERATION = 1,
  MOVE_ABSOLUTE_X = 2,
  MOVE_ABSOLUTE_Y = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_move_ptr_t
{
  pub type_: u8,
  pub flags: u8,
  pub xHigh: i8,
  pub xLow: u8,
  pub yHigh: i8,
  pub yLow: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_move_ptr_iterator_t
{
  pub data: *mut xcb_xkb_sa_move_ptr_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_ptr_btn_t
{
  pub type_: u8,
  pub flags: u8,
  pub count: u8,
  pub button: u8,
  pub pad0: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t
{
  pub data: *mut xcb_xkb_sa_ptr_btn_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_ptr_btn_t
{
  pub type_: u8,
  pub flags: u8,
  pub pad0: u8,
  pub button: u8,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t
{
  pub data: *mut xcb_xkb_sa_lock_ptr_btn_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_set_ptr_dflt_flag_t
{
  DFLT_BTN_ABSOLUTE = 4,
  AFFECT_DFLT_BUTTON = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_ptr_dflt_t
{
  pub type_: u8,
  pub flags: u8,
  pub affect: u8,
  pub value: i8,
  pub pad0: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t
{
  pub data: *mut xcb_xkb_sa_set_ptr_dflt_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

impl xcb_xkb_sa_iso_lock_flag_t
{
  pub const GROUP_ABSOLUTE: xcb_xkb_sa_iso_lock_flag_t =
    xcb_xkb_sa_iso_lock_flag_t::USE_MOD_MAP_MODS;
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_iso_lock_flag_t
{
  NO_LOCK = 1,
  NO_UNLOCK = 2,
  USE_MOD_MAP_MODS = 4,
  ISO_DFLT_IS_GROUP = 8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_iso_lock_no_affect_t
{
  CTRLS = 8,
  PTR = 16,
  GROUP = 32,
  MODS = 64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_iso_lock_t
{
  pub type_: u8,
  pub flags: u8,
  pub mask: u8,
  pub realMods: u8,
  pub group: i8,
  pub affect: u8,
  pub vmodsHigh: u8,
  pub vmodsLow: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_iso_lock_iterator_t
{
  pub data: *mut xcb_xkb_sa_iso_lock_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_terminate_t
{
  pub type_: u8,
  pub pad0: [u8; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_terminate_iterator_t
{
  pub data: *mut xcb_xkb_sa_terminate_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_switch_screen_flag_t
{
  APPLICATION = 1,
  ABSOLUTE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_switch_screen_t
{
  pub type_: u8,
  pub flags: u8,
  pub newScreen: i8,
  pub pad0: [u8; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_switch_screen_iterator_t
{
  pub data: *mut xcb_xkb_sa_switch_screen_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_bool_ctrls_high_t
{
  ACCESS_X_FEEDBACK = 1,
  AUDIBLE_BELL = 2,
  OVERLAY_1 = 4,
  OVERLAY_2 = 8,
  IGNORE_GROUP_LOCK = 16,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_bool_ctrls_low_t
{
  REPEAT_KEYS = 1,
  SLOW_KEYS = 2,
  BOUNCE_KEYS = 4,
  STICKY_KEYS = 8,
  MOUSE_KEYS = 16,
  MOUSE_KEYS_ACCEL = 32,
  ACCESS_X_KEYS = 64,
  ACCESS_X_TIMEOUT = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_controls_t
{
  pub type_: u8,
  pub pad0: [u8; 3usize],
  pub boolCtrlsHigh: u8,
  pub boolCtrlsLow: u8,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_set_controls_iterator_t
{
  pub data: *mut xcb_xkb_sa_set_controls_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_controls_t
{
  pub type_: u8,
  pub pad0: [u8; 3usize],
  pub boolCtrlsHigh: u8,
  pub boolCtrlsLow: u8,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_controls_iterator_t
{
  pub data: *mut xcb_xkb_sa_lock_controls_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_action_message_flag_t
{
  ON_PRESS = 1,
  ON_RELEASE = 2,
  GEN_KEY_EVENT = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_action_message_t
{
  pub type_: u8,
  pub flags: u8,
  pub message: [u8; 6usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_action_message_iterator_t
{
  pub data: *mut xcb_xkb_sa_action_message_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_redirect_key_t
{
  pub type_: u8,
  pub newkey: xcb_keycode_t,
  pub mask: u8,
  pub realModifiers: u8,
  pub vmodsMaskHigh: u8,
  pub vmodsMaskLow: u8,
  pub vmodsHigh: u8,
  pub vmodsLow: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_redirect_key_iterator_t
{
  pub data: *mut xcb_xkb_sa_redirect_key_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_device_btn_t
{
  pub type_: u8,
  pub flags: u8,
  pub count: u8,
  pub button: u8,
  pub device: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_device_btn_iterator_t
{
  pub data: *mut xcb_xkb_sa_device_btn_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_lock_device_flags_t
{
  NO_LOCK = 1,
  NO_UNLOCK = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_device_btn_t
{
  pub type_: u8,
  pub flags: u8,
  pub pad0: u8,
  pub button: u8,
  pub device: u8,
  pub pad1: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t
{
  pub data: *mut xcb_xkb_sa_lock_device_btn_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_xkb_sa_val_what_t
{
  IGNORE_VAL = 0,
  SET_VAL_MIN = 1,
  SET_VAL_CENTER = 2,
  SET_VAL_MAX = 3,
  SET_VAL_RELATIVE = 4,
  SET_VAL_ABSOLUTE = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_device_valuator_t
{
  pub type_: u8,
  pub device: u8,
  pub val1what: u8,
  pub val1index: u8,
  pub val1value: u8,
  pub val2what: u8,
  pub val2index: u8,
  pub val2value: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sa_device_valuator_iterator_t
{
  pub data: *mut xcb_xkb_sa_device_valuator_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_si_action_t
{
  pub type_: u8,
  pub data: [u8; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_si_action_iterator_t
{
  pub data: *mut xcb_xkb_si_action_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sym_interpret_t
{
  pub sym: xcb_keysym_t,
  pub mods: u8,
  pub match_: u8,
  pub virtualMod: u8,
  pub flags: u8,
  pub action: xcb_xkb_si_action_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_sym_interpret_iterator_t
{
  pub data: *mut xcb_xkb_sym_interpret_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_xkb_action_t
{
  pub noaction: xcb_xkb_sa_no_action_t,
  pub setmods: xcb_xkb_sa_set_mods_t,
  pub latchmods: xcb_xkb_sa_latch_mods_t,
  pub lockmods: xcb_xkb_sa_lock_mods_t,
  pub setgroup: xcb_xkb_sa_set_group_t,
  pub latchgroup: xcb_xkb_sa_latch_group_t,
  pub lockgroup: xcb_xkb_sa_lock_group_t,
  pub moveptr: xcb_xkb_sa_move_ptr_t,
  pub ptrbtn: xcb_xkb_sa_ptr_btn_t,
  pub lockptrbtn: xcb_xkb_sa_lock_ptr_btn_t,
  pub setptrdflt: xcb_xkb_sa_set_ptr_dflt_t,
  pub isolock: xcb_xkb_sa_iso_lock_t,
  pub terminate: xcb_xkb_sa_terminate_t,
  pub switchscreen: xcb_xkb_sa_switch_screen_t,
  pub setcontrols: xcb_xkb_sa_set_controls_t,
  pub lockcontrols: xcb_xkb_sa_lock_controls_t,
  pub message: xcb_xkb_sa_action_message_t,
  pub redirect: xcb_xkb_sa_redirect_key_t,
  pub devbtn: xcb_xkb_sa_device_btn_t,
  pub lockdevbtn: xcb_xkb_sa_lock_device_btn_t,
  pub devval: xcb_xkb_sa_device_valuator_t,
  pub type_: u8,
  _bindgen_union_align: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_action_iterator_t
{
  pub data: *mut xcb_xkb_action_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_use_extension_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_use_extension_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub wantedMajor: u16,
  pub wantedMinor: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_use_extension_reply_t
{
  pub response_type: u8,
  pub supported: u8,
  pub sequence: u16,
  pub length: u32,
  pub serverMajor: u16,
  pub serverMinor: u16,
  pub pad0: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_select_events_details_t
{
  pub affectNewKeyboard: u16,
  pub newKeyboardDetails: u16,
  pub affectState: u16,
  pub stateDetails: u16,
  pub affectCtrls: u32,
  pub ctrlDetails: u32,
  pub affectIndicatorState: u32,
  pub indicatorStateDetails: u32,
  pub affectIndicatorMap: u32,
  pub indicatorMapDetails: u32,
  pub affectNames: u16,
  pub namesDetails: u16,
  pub affectCompat: u8,
  pub compatDetails: u8,
  pub affectBell: u8,
  pub bellDetails: u8,
  pub affectMsgDetails: u8,
  pub msgDetails: u8,
  pub affectAccessX: u16,
  pub accessXDetails: u16,
  pub affectExtDev: u16,
  pub extdevDetails: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_select_events_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub affectWhich: u16,
  pub clear: u16,
  pub selectAll: u16,
  pub affectMap: u16,
  pub map: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_bell_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub bellClass: xcb_xkb_bell_class_spec_t,
  pub bellID: xcb_xkb_id_spec_t,
  pub percent: i8,
  pub forceSound: u8,
  pub eventOnly: u8,
  pub pad0: u8,
  pub pitch: i16,
  pub duration: i16,
  pub pad1: [u8; 2usize],
  pub name: xcb_atom_t,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_state_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_state_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_state_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub mods: u8,
  pub baseMods: u8,
  pub latchedMods: u8,
  pub lockedMods: u8,
  pub group: u8,
  pub lockedGroup: u8,
  pub baseGroup: i16,
  pub latchedGroup: i16,
  pub compatState: u8,
  pub grabMods: u8,
  pub compatGrabMods: u8,
  pub lookupMods: u8,
  pub compatLookupMods: u8,
  pub pad0: u8,
  pub ptrBtnState: u16,
  pub pad1: [u8; 6usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_latch_lock_state_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub affectModLocks: u8,
  pub modLocks: u8,
  pub lockGroup: u8,
  pub groupLock: u8,
  pub affectModLatches: u8,
  pub pad0: u8,
  pub pad1: u8,
  pub latchGroup: u8,
  pub groupLatch: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_controls_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_controls_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_controls_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub mouseKeysDfltBtn: u8,
  pub numGroups: u8,
  pub groupsWrap: u8,
  pub internalModsMask: u8,
  pub ignoreLockModsMask: u8,
  pub internalModsRealMods: u8,
  pub ignoreLockModsRealMods: u8,
  pub pad0: u8,
  pub internalModsVmods: u16,
  pub ignoreLockModsVmods: u16,
  pub repeatDelay: u16,
  pub repeatInterval: u16,
  pub slowKeysDelay: u16,
  pub debounceDelay: u16,
  pub mouseKeysDelay: u16,
  pub mouseKeysInterval: u16,
  pub mouseKeysTimeToMax: u16,
  pub mouseKeysMaxSpeed: u16,
  pub mouseKeysCurve: i16,
  pub accessXOption: u16,
  pub accessXTimeout: u16,
  pub accessXTimeoutOptionsMask: u16,
  pub accessXTimeoutOptionsValues: u16,
  pub pad1: [u8; 2usize],
  pub accessXTimeoutMask: u32,
  pub accessXTimeoutValues: u32,
  pub enabledControls: u32,
  pub perKeyRepeat: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_controls_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub affectInternalRealMods: u8,
  pub internalRealMods: u8,
  pub affectIgnoreLockRealMods: u8,
  pub ignoreLockRealMods: u8,
  pub affectInternalVirtualMods: u16,
  pub internalVirtualMods: u16,
  pub affectIgnoreLockVirtualMods: u16,
  pub ignoreLockVirtualMods: u16,
  pub mouseKeysDfltBtn: u8,
  pub groupsWrap: u8,
  pub accessXOptions: u16,
  pub pad0: [u8; 2usize],
  pub affectEnabledControls: u32,
  pub enabledControls: u32,
  pub changeControls: u32,
  pub repeatDelay: u16,
  pub repeatInterval: u16,
  pub slowKeysDelay: u16,
  pub debounceDelay: u16,
  pub mouseKeysDelay: u16,
  pub mouseKeysInterval: u16,
  pub mouseKeysTimeToMax: u16,
  pub mouseKeysMaxSpeed: u16,
  pub mouseKeysCurve: i16,
  pub accessXTimeout: u16,
  pub accessXTimeoutMask: u32,
  pub accessXTimeoutValues: u32,
  pub accessXTimeoutOptionsMask: u16,
  pub accessXTimeoutOptionsValues: u16,
  pub perKeyRepeat: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_map_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub full: u16,
  pub partial: u16,
  pub firstType: u8,
  pub nTypes: u8,
  pub firstKeySym: xcb_keycode_t,
  pub nKeySyms: u8,
  pub firstKeyAction: xcb_keycode_t,
  pub nKeyActions: u8,
  pub firstKeyBehavior: xcb_keycode_t,
  pub nKeyBehaviors: u8,
  pub virtualMods: u16,
  pub firstKeyExplicit: xcb_keycode_t,
  pub nKeyExplicit: u8,
  pub firstModMapKey: xcb_keycode_t,
  pub nModMapKeys: u8,
  pub firstVModMapKey: xcb_keycode_t,
  pub nVModMapKeys: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_map_map_t
{
  pub types_rtrn: *mut xcb_xkb_key_type_t,
  pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
  pub acts_rtrn_count: *mut u8,
  pub pad2: *mut u8,
  pub acts_rtrn_acts: *mut xcb_xkb_action_t,
  pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
  pub vmods_rtrn: *mut u8,
  pub pad3: *mut u8,
  pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
  pub pad4: *mut u8,
  pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
  pub pad5: *mut u8,
  pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_map_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub pad0: [u8; 2usize],
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub present: u16,
  pub firstType: u8,
  pub nTypes: u8,
  pub totalTypes: u8,
  pub firstKeySym: xcb_keycode_t,
  pub totalSyms: u16,
  pub nKeySyms: u8,
  pub firstKeyAction: xcb_keycode_t,
  pub totalActions: u16,
  pub nKeyActions: u8,
  pub firstKeyBehavior: xcb_keycode_t,
  pub nKeyBehaviors: u8,
  pub totalKeyBehaviors: u8,
  pub firstKeyExplicit: xcb_keycode_t,
  pub nKeyExplicit: u8,
  pub totalKeyExplicit: u8,
  pub firstModMapKey: xcb_keycode_t,
  pub nModMapKeys: u8,
  pub totalModMapKeys: u8,
  pub firstVModMapKey: xcb_keycode_t,
  pub nVModMapKeys: u8,
  pub totalVModMapKeys: u8,
  pub pad1: u8,
  pub virtualMods: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_map_values_t
{
  pub types: *mut xcb_xkb_set_key_type_t,
  pub syms: *mut xcb_xkb_key_sym_map_t,
  pub actionsCount: *mut u8,
  pub actions: *mut xcb_xkb_action_t,
  pub behaviors: *mut xcb_xkb_set_behavior_t,
  pub vmods: *mut u8,
  pub explicit: *mut xcb_xkb_set_explicit_t,
  pub modmap: *mut xcb_xkb_key_mod_map_t,
  pub vmodmap: *mut xcb_xkb_key_v_mod_map_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub present: u16,
  pub flags: u16,
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub firstType: u8,
  pub nTypes: u8,
  pub firstKeySym: xcb_keycode_t,
  pub nKeySyms: u8,
  pub totalSyms: u16,
  pub firstKeyAction: xcb_keycode_t,
  pub nKeyActions: u8,
  pub totalActions: u16,
  pub firstKeyBehavior: xcb_keycode_t,
  pub nKeyBehaviors: u8,
  pub totalKeyBehaviors: u8,
  pub firstKeyExplicit: xcb_keycode_t,
  pub nKeyExplicit: u8,
  pub totalKeyExplicit: u8,
  pub firstModMapKey: xcb_keycode_t,
  pub nModMapKeys: u8,
  pub totalModMapKeys: u8,
  pub firstVModMapKey: xcb_keycode_t,
  pub nVModMapKeys: u8,
  pub totalVModMapKeys: u8,
  pub virtualMods: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_compat_map_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_compat_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub groups: u8,
  pub getAllSI: u8,
  pub firstSI: u16,
  pub nSI: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_compat_map_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub groupsRtrn: u8,
  pub pad0: u8,
  pub firstSIRtrn: u16,
  pub nSIRtrn: u16,
  pub nTotalSI: u16,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_compat_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: u8,
  pub recomputeActions: u8,
  pub truncateSI: u8,
  pub groups: u8,
  pub firstSI: u16,
  pub nSI: u16,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_state_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_state_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_state_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub state: u32,
  pub pad0: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_map_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
  pub which: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_indicator_map_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub which: u32,
  pub realIndicators: u32,
  pub nIndicators: u8,
  pub pad0: [u8; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_indicator_map_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
  pub which: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_named_indicator_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_named_indicator_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub ledClass: xcb_xkb_led_class_spec_t,
  pub ledID: xcb_xkb_id_spec_t,
  pub pad0: [u8; 2usize],
  pub indicator: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_named_indicator_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub indicator: xcb_atom_t,
  pub found: u8,
  pub on: u8,
  pub realIndicator: u8,
  pub ndx: u8,
  pub map_flags: u8,
  pub map_whichGroups: u8,
  pub map_groups: u8,
  pub map_whichMods: u8,
  pub map_mods: u8,
  pub map_realMods: u8,
  pub map_vmod: u16,
  pub map_ctrls: u32,
  pub supported: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_named_indicator_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub ledClass: xcb_xkb_led_class_spec_t,
  pub ledID: xcb_xkb_id_spec_t,
  pub pad0: [u8; 2usize],
  pub indicator: xcb_atom_t,
  pub setState: u8,
  pub on: u8,
  pub setMap: u8,
  pub createMap: u8,
  pub pad1: u8,
  pub map_flags: u8,
  pub map_whichGroups: u8,
  pub map_groups: u8,
  pub map_whichMods: u8,
  pub map_realMods: u8,
  pub map_vmods: u16,
  pub map_ctrls: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_names_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_names_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
  pub which: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_names_value_list_t
{
  pub keycodesName: xcb_atom_t,
  pub geometryName: xcb_atom_t,
  pub symbolsName: xcb_atom_t,
  pub physSymbolsName: xcb_atom_t,
  pub typesName: xcb_atom_t,
  pub compatName: xcb_atom_t,
  pub typeNames: *mut xcb_atom_t,
  pub nLevelsPerType: *mut u8,
  pub pad1: *mut u8,
  pub ktLevelNames: *mut xcb_atom_t,
  pub indicatorNames: *mut xcb_atom_t,
  pub virtualModNames: *mut xcb_atom_t,
  pub groups: *mut xcb_atom_t,
  pub keyNames: *mut xcb_xkb_key_name_t,
  pub keyAliases: *mut xcb_xkb_key_alias_t,
  pub radioGroupNames: *mut xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_names_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub which: u32,
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub nTypes: u8,
  pub groupNames: u8,
  pub virtualMods: u16,
  pub firstKey: xcb_keycode_t,
  pub nKeys: u8,
  pub indicators: u32,
  pub nRadioGroups: u8,
  pub nKeyAliases: u8,
  pub nKTLevels: u16,
  pub pad0: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_names_values_t
{
  pub keycodesName: xcb_atom_t,
  pub geometryName: xcb_atom_t,
  pub symbolsName: xcb_atom_t,
  pub physSymbolsName: xcb_atom_t,
  pub typesName: xcb_atom_t,
  pub compatName: xcb_atom_t,
  pub typeNames: *mut xcb_atom_t,
  pub nLevelsPerType: *mut u8,
  pub ktLevelNames: *mut xcb_atom_t,
  pub indicatorNames: *mut xcb_atom_t,
  pub virtualModNames: *mut xcb_atom_t,
  pub groups: *mut xcb_atom_t,
  pub keyNames: *mut xcb_xkb_key_name_t,
  pub keyAliases: *mut xcb_xkb_key_alias_t,
  pub radioGroupNames: *mut xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_names_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub virtualMods: u16,
  pub which: u32,
  pub firstType: u8,
  pub nTypes: u8,
  pub firstKTLevelt: u8,
  pub nKTLevels: u8,
  pub indicators: u32,
  pub groupNames: u8,
  pub nRadioGroups: u8,
  pub firstKey: xcb_keycode_t,
  pub nKeys: u8,
  pub nKeyAliases: u8,
  pub pad0: u8,
  pub totalKTLevelNames: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_per_client_flags_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_per_client_flags_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub pad0: [u8; 2usize],
  pub change: u32,
  pub value: u32,
  pub ctrlsToChange: u32,
  pub autoCtrls: u32,
  pub autoCtrlsValues: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_per_client_flags_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub supported: u32,
  pub value: u32,
  pub autoCtrls: u32,
  pub autoCtrlsValues: u32,
  pub pad0: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_list_components_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_list_components_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub maxNames: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_list_components_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub nKeymaps: u16,
  pub nKeycodes: u16,
  pub nTypes: u16,
  pub nCompatMaps: u16,
  pub nSymbols: u16,
  pub nGeometries: u16,
  pub extra: u16,
  pub pad0: [u8; 10usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub need: u16,
  pub want: u16,
  pub load: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_types_map_t
{
  pub types_rtrn: *mut xcb_xkb_key_type_t,
  pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
  pub acts_rtrn_count: *mut u8,
  pub acts_rtrn_acts: *mut xcb_xkb_action_t,
  pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
  pub vmods_rtrn: *mut u8,
  pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
  pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
  pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t
{
  pub keycodesName: xcb_atom_t,
  pub geometryName: xcb_atom_t,
  pub symbolsName: xcb_atom_t,
  pub physSymbolsName: xcb_atom_t,
  pub typesName: xcb_atom_t,
  pub compatName: xcb_atom_t,
  pub typeNames: *mut xcb_atom_t,
  pub nLevelsPerType: *mut u8,
  pub ktLevelNames: *mut xcb_atom_t,
  pub indicatorNames: *mut xcb_atom_t,
  pub virtualModNames: *mut xcb_atom_t,
  pub groups: *mut xcb_atom_t,
  pub keyNames: *mut xcb_xkb_key_name_t,
  pub keyAliases: *mut xcb_xkb_key_alias_t,
  pub radioGroupNames: *mut xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t
{
  pub types: xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_1,
  pub compat_map: xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_2,
  pub indicator_maps: xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_3,
  pub key_names: xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_4,
  pub geometry: xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_1
{
  pub getmap_type: u8,
  pub typeDeviceID: u8,
  pub getmap_sequence: u16,
  pub getmap_length: u32,
  pub pad1: [u8; 2usize],
  pub typeMinKeyCode: xcb_keycode_t,
  pub typeMaxKeyCode: xcb_keycode_t,
  pub present: u16,
  pub firstType: u8,
  pub nTypes: u8,
  pub totalTypes: u8,
  pub firstKeySym: xcb_keycode_t,
  pub totalSyms: u16,
  pub nKeySyms: u8,
  pub firstKeyAction: xcb_keycode_t,
  pub totalActions: u16,
  pub nKeyActions: u8,
  pub firstKeyBehavior: xcb_keycode_t,
  pub nKeyBehaviors: u8,
  pub totalKeyBehaviors: u8,
  pub firstKeyExplicit: xcb_keycode_t,
  pub nKeyExplicit: u8,
  pub totalKeyExplicit: u8,
  pub firstModMapKey: xcb_keycode_t,
  pub nModMapKeys: u8,
  pub totalModMapKeys: u8,
  pub firstVModMapKey: xcb_keycode_t,
  pub nVModMapKeys: u8,
  pub totalVModMapKeys: u8,
  pub pad2: u8,
  pub virtualMods: u16,
  pub map: xcb_xkb_get_kbd_by_name_replies_types_map_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_2
{
  pub compatmap_type: u8,
  pub compatDeviceID: u8,
  pub compatmap_sequence: u16,
  pub compatmap_length: u32,
  pub groupsRtrn: u8,
  pub pad7: u8,
  pub firstSIRtrn: u16,
  pub nSIRtrn: u16,
  pub nTotalSI: u16,
  pub pad8: [u8; 16usize],
  pub si_rtrn: *mut xcb_xkb_sym_interpret_t,
  pub group_rtrn: *mut xcb_xkb_mod_def_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_3
{
  pub indicatormap_type: u8,
  pub indicatorDeviceID: u8,
  pub indicatormap_sequence: u16,
  pub indicatormap_length: u32,
  pub which: u32,
  pub realIndicators: u32,
  pub nIndicators: u8,
  pub pad9: [u8; 15usize],
  pub maps: *mut xcb_xkb_indicator_map_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_4
{
  pub keyname_type: u8,
  pub keyDeviceID: u8,
  pub keyname_sequence: u16,
  pub keyname_length: u32,
  pub which: u32,
  pub keyMinKeyCode: xcb_keycode_t,
  pub keyMaxKeyCode: xcb_keycode_t,
  pub nTypes: u8,
  pub groupNames: u8,
  pub virtualMods: u16,
  pub firstKey: xcb_keycode_t,
  pub nKeys: u8,
  pub indicators: u32,
  pub nRadioGroups: u8,
  pub nKeyAliases: u8,
  pub nKTLevels: u16,
  pub pad10: [u8; 4usize],
  pub valueList: xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__bindgen_ty_5
{
  pub geometry_type: u8,
  pub geometryDeviceID: u8,
  pub geometry_sequence: u16,
  pub geometry_length: u32,
  pub name: xcb_atom_t,
  pub geometryFound: u8,
  pub pad12: u8,
  pub widthMM: u16,
  pub heightMM: u16,
  pub nProperties: u16,
  pub nColors: u16,
  pub nShapes: u16,
  pub nSections: u16,
  pub nDoodads: u16,
  pub nKeyAliases: u16,
  pub baseColorNdx: u8,
  pub labelColorNdx: u8,
  pub labelFont: *mut xcb_xkb_counted_string_16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_kbd_by_name_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub loaded: u8,
  pub newKeyboard: u8,
  pub found: u16,
  pub reported: u16,
  pub pad0: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_device_info_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_device_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub wanted: u16,
  pub allButtons: u8,
  pub firstButton: u8,
  pub nButtons: u8,
  pub pad0: u8,
  pub ledClass: xcb_xkb_led_class_spec_t,
  pub ledID: xcb_xkb_id_spec_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_get_device_info_reply_t
{
  pub response_type: u8,
  pub deviceID: u8,
  pub sequence: u16,
  pub length: u32,
  pub present: u16,
  pub supported: u16,
  pub unsupported: u16,
  pub nDeviceLedFBs: u16,
  pub firstBtnWanted: u8,
  pub nBtnsWanted: u8,
  pub firstBtnRtrn: u8,
  pub nBtnsRtrn: u8,
  pub totalBtns: u8,
  pub hasOwnState: u8,
  pub dfltKbdFB: u16,
  pub dfltLedFB: u16,
  pub pad0: [u8; 2usize],
  pub devType: xcb_atom_t,
  pub nameLen: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_device_info_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub deviceSpec: xcb_xkb_device_spec_t,
  pub firstBtn: u8,
  pub nBtns: u8,
  pub change: u16,
  pub nDeviceLedFBs: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_debugging_flags_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_debugging_flags_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub msgLength: u16,
  pub pad0: [u8; 2usize],
  pub affectFlags: u32,
  pub flags: u32,
  pub affectCtrls: u32,
  pub ctrls: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_set_debugging_flags_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub currentFlags: u32,
  pub currentCtrls: u32,
  pub supportedFlags: u32,
  pub supportedCtrls: u32,
  pub pad1: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_new_keyboard_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub oldDeviceID: u8,
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub oldMinKeyCode: xcb_keycode_t,
  pub oldMaxKeyCode: xcb_keycode_t,
  pub requestMajor: u8,
  pub requestMinor: u8,
  pub changed: u16,
  pub pad0: [u8; 14usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_map_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub ptrBtnActions: u8,
  pub changed: u16,
  pub minKeyCode: xcb_keycode_t,
  pub maxKeyCode: xcb_keycode_t,
  pub firstType: u8,
  pub nTypes: u8,
  pub firstKeySym: xcb_keycode_t,
  pub nKeySyms: u8,
  pub firstKeyAct: xcb_keycode_t,
  pub nKeyActs: u8,
  pub firstKeyBehavior: xcb_keycode_t,
  pub nKeyBehavior: u8,
  pub firstKeyExplicit: xcb_keycode_t,
  pub nKeyExplicit: u8,
  pub firstModMapKey: xcb_keycode_t,
  pub nModMapKeys: u8,
  pub firstVModMapKey: xcb_keycode_t,
  pub nVModMapKeys: u8,
  pub virtualMods: u16,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_state_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub mods: u8,
  pub baseMods: u8,
  pub latchedMods: u8,
  pub lockedMods: u8,
  pub group: u8,
  pub baseGroup: i16,
  pub latchedGroup: i16,
  pub lockedGroup: u8,
  pub compatState: u8,
  pub grabMods: u8,
  pub compatGrabMods: u8,
  pub lookupMods: u8,
  pub compatLoockupMods: u8,
  pub ptrBtnState: u16,
  pub changed: u16,
  pub keycode: xcb_keycode_t,
  pub eventType: u8,
  pub requestMajor: u8,
  pub requestMinor: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_controls_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub numGroups: u8,
  pub pad0: [u8; 2usize],
  pub changedControls: u32,
  pub enabledControls: u32,
  pub enabledControlChanges: u32,
  pub keycode: xcb_keycode_t,
  pub eventType: u8,
  pub requestMajor: u8,
  pub requestMinor: u8,
  pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_indicator_state_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub pad0: [u8; 3usize],
  pub state: u32,
  pub stateChanged: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_indicator_map_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub pad0: [u8; 3usize],
  pub state: u32,
  pub mapChanged: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_names_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub pad0: u8,
  pub changed: u16,
  pub firstType: u8,
  pub nTypes: u8,
  pub firstLevelName: u8,
  pub nLevelNames: u8,
  pub pad1: u8,
  pub nRadioGroups: u8,
  pub nKeyAliases: u8,
  pub changedGroupNames: u8,
  pub changedVirtualMods: u16,
  pub firstKey: xcb_keycode_t,
  pub nKeys: u8,
  pub changedIndicators: u32,
  pub pad2: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_compat_map_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub changedGroups: u8,
  pub firstSI: u16,
  pub nSI: u16,
  pub nTotalSI: u16,
  pub pad0: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_bell_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub bellClass: u8,
  pub bellID: u8,
  pub percent: u8,
  pub pitch: u16,
  pub duration: u16,
  pub name: xcb_atom_t,
  pub window: xcb_window_t,
  pub eventOnly: u8,
  pub pad0: [u8; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_action_message_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub keycode: xcb_keycode_t,
  pub press: u8,
  pub keyEventFollows: u8,
  pub mods: u8,
  pub group: u8,
  pub message: [xcb_xkb_string8_t; 8usize],
  pub pad0: [u8; 10usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_access_x_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub keycode: xcb_keycode_t,
  pub detailt: u16,
  pub slowKeysDelay: u16,
  pub debounceDelay: u16,
  pub pad0: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xkb_extension_device_notify_event_t
{
  pub response_type: u8,
  pub xkbType: u8,
  pub sequence: u16,
  pub time: xcb_timestamp_t,
  pub deviceID: u8,
  pub pad0: u8,
  pub reason: u16,
  pub ledClass: u16,
  pub ledID: u16,
  pub ledsDefined: u32,
  pub ledState: u32,
  pub firstButton: u8,
  pub nButtons: u8,
  pub supported: u16,
  pub unsupported: u16,
  pub pad1: [u8; 2usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_xkb_id: xcb_extension_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map(
    R: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_get_kbd_by_name_replies_types_map_t;

  pub fn xcb_xkb_device_spec_next(i: *mut xcb_xkb_device_spec_iterator_t);

  pub fn xcb_xkb_device_spec_end(i: xcb_xkb_device_spec_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_led_class_spec_next(i: *mut xcb_xkb_led_class_spec_iterator_t);

  pub fn xcb_xkb_led_class_spec_end(i: xcb_xkb_led_class_spec_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_xkb_bell_class_spec_next(i: *mut xcb_xkb_bell_class_spec_iterator_t);

  pub fn xcb_xkb_bell_class_spec_end(
    i: xcb_xkb_bell_class_spec_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_id_spec_next(i: *mut xcb_xkb_id_spec_iterator_t);

  pub fn xcb_xkb_id_spec_end(i: xcb_xkb_id_spec_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_indicator_map_next(i: *mut xcb_xkb_indicator_map_iterator_t);

  pub fn xcb_xkb_indicator_map_end(i: xcb_xkb_indicator_map_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_mod_def_next(i: *mut xcb_xkb_mod_def_iterator_t);

  pub fn xcb_xkb_mod_def_end(i: xcb_xkb_mod_def_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_name_next(i: *mut xcb_xkb_key_name_iterator_t);

  pub fn xcb_xkb_key_name_end(i: xcb_xkb_key_name_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_alias_next(i: *mut xcb_xkb_key_alias_iterator_t);

  pub fn xcb_xkb_key_alias_end(i: xcb_xkb_key_alias_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_counted_string_16_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_counted_string_16_string(
    R: *const xcb_xkb_counted_string_16_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xkb_counted_string_16_string_length(
    R: *const xcb_xkb_counted_string_16_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_counted_string_16_string_end(
    R: *const xcb_xkb_counted_string_16_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_counted_string_16_alignment_pad(
    R: *const xcb_xkb_counted_string_16_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_counted_string_16_alignment_pad_length(
    R: *const xcb_xkb_counted_string_16_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_counted_string_16_alignment_pad_end(
    R: *const xcb_xkb_counted_string_16_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_counted_string_16_next(i: *mut xcb_xkb_counted_string_16_iterator_t);

  pub fn xcb_xkb_counted_string_16_end(
    i: xcb_xkb_counted_string_16_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_kt_map_entry_next(i: *mut xcb_xkb_kt_map_entry_iterator_t);

  pub fn xcb_xkb_kt_map_entry_end(i: xcb_xkb_kt_map_entry_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_type_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_key_type_map(R: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_kt_map_entry_t;

  pub fn xcb_xkb_key_type_map_length(R: *const xcb_xkb_key_type_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_key_type_map_iterator(
    R: *const xcb_xkb_key_type_t
  ) -> xcb_xkb_kt_map_entry_iterator_t;

  pub fn xcb_xkb_key_type_preserve(R: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_mod_def_t;

  pub fn xcb_xkb_key_type_preserve_length(R: *const xcb_xkb_key_type_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_key_type_preserve_iterator(
    R: *const xcb_xkb_key_type_t
  ) -> xcb_xkb_mod_def_iterator_t;

  pub fn xcb_xkb_key_type_next(i: *mut xcb_xkb_key_type_iterator_t);

  pub fn xcb_xkb_key_type_end(i: xcb_xkb_key_type_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_sym_map_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_key_sym_map_syms(R: *const xcb_xkb_key_sym_map_t) -> *mut xcb_keysym_t;

  pub fn xcb_xkb_key_sym_map_syms_length(R: *const xcb_xkb_key_sym_map_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_key_sym_map_syms_end(R: *const xcb_xkb_key_sym_map_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_sym_map_next(i: *mut xcb_xkb_key_sym_map_iterator_t);

  pub fn xcb_xkb_key_sym_map_end(i: xcb_xkb_key_sym_map_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_common_behavior_next(i: *mut xcb_xkb_common_behavior_iterator_t);

  pub fn xcb_xkb_common_behavior_end(
    i: xcb_xkb_common_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_default_behavior_next(i: *mut xcb_xkb_default_behavior_iterator_t);

  pub fn xcb_xkb_default_behavior_end(
    i: xcb_xkb_default_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_lock_behavior_next(i: *mut xcb_xkb_lock_behavior_iterator_t);

  pub fn xcb_xkb_lock_behavior_end(i: xcb_xkb_lock_behavior_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_radio_group_behavior_next(i: *mut xcb_xkb_radio_group_behavior_iterator_t);

  pub fn xcb_xkb_radio_group_behavior_end(
    i: xcb_xkb_radio_group_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_overlay_behavior_next(i: *mut xcb_xkb_overlay_behavior_iterator_t);

  pub fn xcb_xkb_overlay_behavior_end(
    i: xcb_xkb_overlay_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_permament_lock_behavior_next(i: *mut xcb_xkb_permament_lock_behavior_iterator_t);

  pub fn xcb_xkb_permament_lock_behavior_end(
    i: xcb_xkb_permament_lock_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_permament_radio_group_behavior_next(
    i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t
  );

  pub fn xcb_xkb_permament_radio_group_behavior_end(
    i: xcb_xkb_permament_radio_group_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_permament_overlay_behavior_next(
    i: *mut xcb_xkb_permament_overlay_behavior_iterator_t
  );

  pub fn xcb_xkb_permament_overlay_behavior_end(
    i: xcb_xkb_permament_overlay_behavior_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_behavior_next(i: *mut xcb_xkb_behavior_iterator_t);

  pub fn xcb_xkb_behavior_end(i: xcb_xkb_behavior_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_behavior_next(i: *mut xcb_xkb_set_behavior_iterator_t);

  pub fn xcb_xkb_set_behavior_end(i: xcb_xkb_set_behavior_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_explicit_next(i: *mut xcb_xkb_set_explicit_iterator_t);

  pub fn xcb_xkb_set_explicit_end(i: xcb_xkb_set_explicit_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_mod_map_next(i: *mut xcb_xkb_key_mod_map_iterator_t);

  pub fn xcb_xkb_key_mod_map_end(i: xcb_xkb_key_mod_map_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_v_mod_map_next(i: *mut xcb_xkb_key_v_mod_map_iterator_t);

  pub fn xcb_xkb_key_v_mod_map_end(i: xcb_xkb_key_v_mod_map_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_kt_set_map_entry_next(i: *mut xcb_xkb_kt_set_map_entry_iterator_t);

  pub fn xcb_xkb_kt_set_map_entry_end(
    i: xcb_xkb_kt_set_map_entry_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_key_type_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_key_type_entries(
    R: *const xcb_xkb_set_key_type_t
  ) -> *mut xcb_xkb_kt_set_map_entry_t;

  pub fn xcb_xkb_set_key_type_entries_length(
    R: *const xcb_xkb_set_key_type_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_key_type_entries_iterator(
    R: *const xcb_xkb_set_key_type_t
  ) -> xcb_xkb_kt_set_map_entry_iterator_t;

  pub fn xcb_xkb_set_key_type_preserve_entries(
    R: *const xcb_xkb_set_key_type_t
  ) -> *mut xcb_xkb_kt_set_map_entry_t;

  pub fn xcb_xkb_set_key_type_preserve_entries_length(
    R: *const xcb_xkb_set_key_type_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_key_type_preserve_entries_iterator(
    R: *const xcb_xkb_set_key_type_t
  ) -> xcb_xkb_kt_set_map_entry_iterator_t;

  pub fn xcb_xkb_set_key_type_next(i: *mut xcb_xkb_set_key_type_iterator_t);

  pub fn xcb_xkb_set_key_type_end(i: xcb_xkb_set_key_type_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_string8_next(i: *mut xcb_xkb_string8_iterator_t);

  pub fn xcb_xkb_string8_end(i: xcb_xkb_string8_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_outline_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_outline_points(R: *const xcb_xkb_outline_t) -> *mut xcb_point_t;

  pub fn xcb_xkb_outline_points_length(R: *const xcb_xkb_outline_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_outline_points_iterator(R: *const xcb_xkb_outline_t) -> xcb_point_iterator_t;

  pub fn xcb_xkb_outline_next(i: *mut xcb_xkb_outline_iterator_t);

  pub fn xcb_xkb_outline_end(i: xcb_xkb_outline_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_shape_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_shape_outlines_length(R: *const xcb_xkb_shape_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_shape_outlines_iterator(R: *const xcb_xkb_shape_t) -> xcb_xkb_outline_iterator_t;

  pub fn xcb_xkb_shape_next(i: *mut xcb_xkb_shape_iterator_t);

  pub fn xcb_xkb_shape_end(i: xcb_xkb_shape_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_key_next(i: *mut xcb_xkb_key_iterator_t);

  pub fn xcb_xkb_key_end(i: xcb_xkb_key_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_overlay_key_next(i: *mut xcb_xkb_overlay_key_iterator_t);

  pub fn xcb_xkb_overlay_key_end(i: xcb_xkb_overlay_key_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_overlay_row_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_overlay_row_keys(R: *const xcb_xkb_overlay_row_t) -> *mut xcb_xkb_overlay_key_t;

  pub fn xcb_xkb_overlay_row_keys_length(R: *const xcb_xkb_overlay_row_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_overlay_row_keys_iterator(
    R: *const xcb_xkb_overlay_row_t
  ) -> xcb_xkb_overlay_key_iterator_t;

  pub fn xcb_xkb_overlay_row_next(i: *mut xcb_xkb_overlay_row_iterator_t);

  pub fn xcb_xkb_overlay_row_end(i: xcb_xkb_overlay_row_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_overlay_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_overlay_rows_length(R: *const xcb_xkb_overlay_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_overlay_rows_iterator(
    R: *const xcb_xkb_overlay_t
  ) -> xcb_xkb_overlay_row_iterator_t;

  pub fn xcb_xkb_overlay_next(i: *mut xcb_xkb_overlay_iterator_t);

  pub fn xcb_xkb_overlay_end(i: xcb_xkb_overlay_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_row_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_row_keys(R: *const xcb_xkb_row_t) -> *mut xcb_xkb_key_t;

  pub fn xcb_xkb_row_keys_length(R: *const xcb_xkb_row_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_row_keys_iterator(R: *const xcb_xkb_row_t) -> xcb_xkb_key_iterator_t;

  pub fn xcb_xkb_row_next(i: *mut xcb_xkb_row_iterator_t);

  pub fn xcb_xkb_row_end(i: xcb_xkb_row_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_listing_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_listing_string(R: *const xcb_xkb_listing_t) -> *mut xcb_xkb_string8_t;

  pub fn xcb_xkb_listing_string_length(R: *const xcb_xkb_listing_t) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_listing_string_end(R: *const xcb_xkb_listing_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_listing_next(i: *mut xcb_xkb_listing_iterator_t);

  pub fn xcb_xkb_listing_end(i: xcb_xkb_listing_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_device_led_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_device_led_info_names(R: *const xcb_xkb_device_led_info_t) -> *mut xcb_atom_t;

  pub fn xcb_xkb_device_led_info_names_length(
    R: *const xcb_xkb_device_led_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_device_led_info_names_end(
    R: *const xcb_xkb_device_led_info_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_device_led_info_maps(
    R: *const xcb_xkb_device_led_info_t
  ) -> *mut xcb_xkb_indicator_map_t;

  pub fn xcb_xkb_device_led_info_maps_length(
    R: *const xcb_xkb_device_led_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_device_led_info_maps_iterator(
    R: *const xcb_xkb_device_led_info_t
  ) -> xcb_xkb_indicator_map_iterator_t;

  pub fn xcb_xkb_device_led_info_next(i: *mut xcb_xkb_device_led_info_iterator_t);

  pub fn xcb_xkb_device_led_info_end(
    i: xcb_xkb_device_led_info_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_no_action_next(i: *mut xcb_xkb_sa_no_action_iterator_t);

  pub fn xcb_xkb_sa_no_action_end(i: xcb_xkb_sa_no_action_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_set_mods_next(i: *mut xcb_xkb_sa_set_mods_iterator_t);

  pub fn xcb_xkb_sa_set_mods_end(i: xcb_xkb_sa_set_mods_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_latch_mods_next(i: *mut xcb_xkb_sa_latch_mods_iterator_t);

  pub fn xcb_xkb_sa_latch_mods_end(i: xcb_xkb_sa_latch_mods_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_lock_mods_next(i: *mut xcb_xkb_sa_lock_mods_iterator_t);

  pub fn xcb_xkb_sa_lock_mods_end(i: xcb_xkb_sa_lock_mods_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_set_group_next(i: *mut xcb_xkb_sa_set_group_iterator_t);

  pub fn xcb_xkb_sa_set_group_end(i: xcb_xkb_sa_set_group_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_latch_group_next(i: *mut xcb_xkb_sa_latch_group_iterator_t);

  pub fn xcb_xkb_sa_latch_group_end(i: xcb_xkb_sa_latch_group_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_lock_group_next(i: *mut xcb_xkb_sa_lock_group_iterator_t);

  pub fn xcb_xkb_sa_lock_group_end(i: xcb_xkb_sa_lock_group_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_move_ptr_next(i: *mut xcb_xkb_sa_move_ptr_iterator_t);

  pub fn xcb_xkb_sa_move_ptr_end(i: xcb_xkb_sa_move_ptr_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_ptr_btn_next(i: *mut xcb_xkb_sa_ptr_btn_iterator_t);

  pub fn xcb_xkb_sa_ptr_btn_end(i: xcb_xkb_sa_ptr_btn_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_lock_ptr_btn_next(i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t);

  pub fn xcb_xkb_sa_lock_ptr_btn_end(
    i: xcb_xkb_sa_lock_ptr_btn_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_set_ptr_dflt_next(i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t);

  pub fn xcb_xkb_sa_set_ptr_dflt_end(
    i: xcb_xkb_sa_set_ptr_dflt_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_iso_lock_next(i: *mut xcb_xkb_sa_iso_lock_iterator_t);

  pub fn xcb_xkb_sa_iso_lock_end(i: xcb_xkb_sa_iso_lock_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_terminate_next(i: *mut xcb_xkb_sa_terminate_iterator_t);

  pub fn xcb_xkb_sa_terminate_end(i: xcb_xkb_sa_terminate_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_switch_screen_next(i: *mut xcb_xkb_sa_switch_screen_iterator_t);

  pub fn xcb_xkb_sa_switch_screen_end(
    i: xcb_xkb_sa_switch_screen_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_set_controls_next(i: *mut xcb_xkb_sa_set_controls_iterator_t);

  pub fn xcb_xkb_sa_set_controls_end(
    i: xcb_xkb_sa_set_controls_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_lock_controls_next(i: *mut xcb_xkb_sa_lock_controls_iterator_t);

  pub fn xcb_xkb_sa_lock_controls_end(
    i: xcb_xkb_sa_lock_controls_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_action_message_next(i: *mut xcb_xkb_sa_action_message_iterator_t);

  pub fn xcb_xkb_sa_action_message_end(
    i: xcb_xkb_sa_action_message_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_redirect_key_next(i: *mut xcb_xkb_sa_redirect_key_iterator_t);

  pub fn xcb_xkb_sa_redirect_key_end(
    i: xcb_xkb_sa_redirect_key_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_device_btn_next(i: *mut xcb_xkb_sa_device_btn_iterator_t);

  pub fn xcb_xkb_sa_device_btn_end(i: xcb_xkb_sa_device_btn_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_lock_device_btn_next(i: *mut xcb_xkb_sa_lock_device_btn_iterator_t);

  pub fn xcb_xkb_sa_lock_device_btn_end(
    i: xcb_xkb_sa_lock_device_btn_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sa_device_valuator_next(i: *mut xcb_xkb_sa_device_valuator_iterator_t);

  pub fn xcb_xkb_sa_device_valuator_end(
    i: xcb_xkb_sa_device_valuator_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_si_action_next(i: *mut xcb_xkb_si_action_iterator_t);

  pub fn xcb_xkb_si_action_end(i: xcb_xkb_si_action_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_sym_interpret_next(i: *mut xcb_xkb_sym_interpret_iterator_t);

  pub fn xcb_xkb_sym_interpret_end(i: xcb_xkb_sym_interpret_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_action_next(i: *mut xcb_xkb_action_iterator_t);

  pub fn xcb_xkb_action_end(i: xcb_xkb_action_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_use_extension(
    c: *mut xcb_connection_t,
    wantedMajor: u16,
    wantedMinor: u16,
  ) -> xcb_xkb_use_extension_cookie_t;

  pub fn xcb_xkb_use_extension_unchecked(
    c: *mut xcb_connection_t,
    wantedMajor: u16,
    wantedMinor: u16,
  ) -> xcb_xkb_use_extension_cookie_t;

  pub fn xcb_xkb_use_extension_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_use_extension_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_use_extension_reply_t;

  pub fn xcb_xkb_select_events_details_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    _aux: *const xcb_xkb_select_events_details_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_select_events_details_unpack(
    _buffer: *const ::std::os::raw::c_void,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    _aux: *mut xcb_xkb_select_events_details_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_select_events_details_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_select_events_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_select_events_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    affectMap: u16,
    map: u16,
    details: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_select_events(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    affectMap: u16,
    map: u16,
    details: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_select_events_aux_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    affectMap: u16,
    map: u16,
    details: *const xcb_xkb_select_events_details_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_select_events_aux(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectWhich: u16,
    clear: u16,
    selectAll: u16,
    affectMap: u16,
    map: u16,
    details: *const xcb_xkb_select_events_details_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_select_events_details(
    R: *const xcb_xkb_select_events_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_bell_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    bellClass: xcb_xkb_bell_class_spec_t,
    bellID: xcb_xkb_id_spec_t,
    percent: i8,
    forceSound: u8,
    eventOnly: u8,
    pitch: i16,
    duration: i16,
    name: xcb_atom_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_bell(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    bellClass: xcb_xkb_bell_class_spec_t,
    bellID: xcb_xkb_id_spec_t,
    percent: i8,
    forceSound: u8,
    eventOnly: u8,
    pitch: i16,
    duration: i16,
    name: xcb_atom_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_get_state(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_state_cookie_t;

  pub fn xcb_xkb_get_state_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_state_cookie_t;

  pub fn xcb_xkb_get_state_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_state_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_state_reply_t;

  pub fn xcb_xkb_latch_lock_state_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectModLocks: u8,
    modLocks: u8,
    lockGroup: u8,
    groupLock: u8,
    affectModLatches: u8,
    latchGroup: u8,
    groupLatch: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_latch_lock_state(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectModLocks: u8,
    modLocks: u8,
    lockGroup: u8,
    groupLock: u8,
    affectModLatches: u8,
    latchGroup: u8,
    groupLatch: u16,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_get_controls(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_controls_cookie_t;

  pub fn xcb_xkb_get_controls_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_controls_cookie_t;

  pub fn xcb_xkb_get_controls_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_controls_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_controls_reply_t;

  pub fn xcb_xkb_set_controls_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectInternalRealMods: u8,
    internalRealMods: u8,
    affectIgnoreLockRealMods: u8,
    ignoreLockRealMods: u8,
    affectInternalVirtualMods: u16,
    internalVirtualMods: u16,
    affectIgnoreLockVirtualMods: u16,
    ignoreLockVirtualMods: u16,
    mouseKeysDfltBtn: u8,
    groupsWrap: u8,
    accessXOptions: u16,
    affectEnabledControls: u32,
    enabledControls: u32,
    changeControls: u32,
    repeatDelay: u16,
    repeatInterval: u16,
    slowKeysDelay: u16,
    debounceDelay: u16,
    mouseKeysDelay: u16,
    mouseKeysInterval: u16,
    mouseKeysTimeToMax: u16,
    mouseKeysMaxSpeed: u16,
    mouseKeysCurve: i16,
    accessXTimeout: u16,
    accessXTimeoutMask: u32,
    accessXTimeoutValues: u32,
    accessXTimeoutOptionsMask: u16,
    accessXTimeoutOptionsValues: u16,
    perKeyRepeat: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_controls(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    affectInternalRealMods: u8,
    internalRealMods: u8,
    affectIgnoreLockRealMods: u8,
    ignoreLockRealMods: u8,
    affectInternalVirtualMods: u16,
    internalVirtualMods: u16,
    affectIgnoreLockVirtualMods: u16,
    ignoreLockVirtualMods: u16,
    mouseKeysDfltBtn: u8,
    groupsWrap: u8,
    accessXOptions: u16,
    affectEnabledControls: u32,
    enabledControls: u32,
    changeControls: u32,
    repeatDelay: u16,
    repeatInterval: u16,
    slowKeysDelay: u16,
    debounceDelay: u16,
    mouseKeysDelay: u16,
    mouseKeysInterval: u16,
    mouseKeysTimeToMax: u16,
    mouseKeysMaxSpeed: u16,
    mouseKeysCurve: i16,
    accessXTimeout: u16,
    accessXTimeoutMask: u32,
    accessXTimeoutValues: u32,
    accessXTimeoutOptionsMask: u16,
    accessXTimeoutOptionsValues: u16,
    perKeyRepeat: *const u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_get_map_map_types_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_types_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_key_type_iterator_t;

  pub fn xcb_xkb_get_map_map_syms_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_syms_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_key_sym_map_iterator_t;

  pub fn xcb_xkb_get_map_map_acts_rtrn_count(S: *const xcb_xkb_get_map_map_t) -> *mut u8;

  pub fn xcb_xkb_get_map_map_acts_rtrn_count_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_acts_rtrn_count_end(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_map_map_acts_rtrn_acts(
    S: *const xcb_xkb_get_map_map_t
  ) -> *mut xcb_xkb_action_t;

  pub fn xcb_xkb_get_map_map_acts_rtrn_acts_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_acts_rtrn_acts_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_action_iterator_t;

  pub fn xcb_xkb_get_map_map_behaviors_rtrn(
    S: *const xcb_xkb_get_map_map_t
  ) -> *mut xcb_xkb_set_behavior_t;

  pub fn xcb_xkb_get_map_map_behaviors_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_behaviors_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_set_behavior_iterator_t;

  pub fn xcb_xkb_get_map_map_vmods_rtrn(S: *const xcb_xkb_get_map_map_t) -> *mut u8;

  pub fn xcb_xkb_get_map_map_vmods_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_vmods_rtrn_end(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_map_map_explicit_rtrn(
    S: *const xcb_xkb_get_map_map_t
  ) -> *mut xcb_xkb_set_explicit_t;

  pub fn xcb_xkb_get_map_map_explicit_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_explicit_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_set_explicit_iterator_t;

  pub fn xcb_xkb_get_map_map_modmap_rtrn(
    S: *const xcb_xkb_get_map_map_t
  ) -> *mut xcb_xkb_key_mod_map_t;

  pub fn xcb_xkb_get_map_map_modmap_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_modmap_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_key_mod_map_iterator_t;

  pub fn xcb_xkb_get_map_map_vmodmap_rtrn(
    S: *const xcb_xkb_get_map_map_t
  ) -> *mut xcb_xkb_key_v_mod_map_t;

  pub fn xcb_xkb_get_map_map_vmodmap_rtrn_length(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_vmodmap_rtrn_iterator(
    R: *const xcb_xkb_get_map_reply_t,
    S: *const xcb_xkb_get_map_map_t,
  ) -> xcb_xkb_key_v_mod_map_iterator_t;

  pub fn xcb_xkb_get_map_map_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *const xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *mut xcb_xkb_get_map_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_map_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    full: u16,
    partial: u16,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    virtualMods: u16,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
  ) -> xcb_xkb_get_map_cookie_t;

  pub fn xcb_xkb_get_map_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    full: u16,
    partial: u16,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    virtualMods: u16,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
  ) -> xcb_xkb_get_map_cookie_t;

  pub fn xcb_xkb_get_map_map(R: *const xcb_xkb_get_map_reply_t) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_get_map_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_map_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_map_reply_t;

  pub fn xcb_xkb_set_map_values_types_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_types_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_set_key_type_iterator_t;

  pub fn xcb_xkb_set_map_values_syms_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_syms_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_key_sym_map_iterator_t;

  pub fn xcb_xkb_set_map_values_actions_count(S: *const xcb_xkb_set_map_values_t) -> *mut u8;

  pub fn xcb_xkb_set_map_values_actions_count_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_actions_count_end(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_map_values_actions(
    S: *const xcb_xkb_set_map_values_t
  ) -> *mut xcb_xkb_action_t;

  pub fn xcb_xkb_set_map_values_actions_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_actions_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_action_iterator_t;

  pub fn xcb_xkb_set_map_values_behaviors(
    S: *const xcb_xkb_set_map_values_t
  ) -> *mut xcb_xkb_set_behavior_t;

  pub fn xcb_xkb_set_map_values_behaviors_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_behaviors_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_set_behavior_iterator_t;

  pub fn xcb_xkb_set_map_values_vmods(S: *const xcb_xkb_set_map_values_t) -> *mut u8;

  pub fn xcb_xkb_set_map_values_vmods_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_vmods_end(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_map_values_explicit(
    S: *const xcb_xkb_set_map_values_t
  ) -> *mut xcb_xkb_set_explicit_t;

  pub fn xcb_xkb_set_map_values_explicit_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_explicit_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_set_explicit_iterator_t;

  pub fn xcb_xkb_set_map_values_modmap(
    S: *const xcb_xkb_set_map_values_t
  ) -> *mut xcb_xkb_key_mod_map_t;

  pub fn xcb_xkb_set_map_values_modmap_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_modmap_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_key_mod_map_iterator_t;

  pub fn xcb_xkb_set_map_values_vmodmap(
    S: *const xcb_xkb_set_map_values_t
  ) -> *mut xcb_xkb_key_v_mod_map_t;

  pub fn xcb_xkb_set_map_values_vmodmap_length(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_vmodmap_iterator(
    R: *const xcb_xkb_set_map_request_t,
    S: *const xcb_xkb_set_map_values_t,
  ) -> xcb_xkb_key_v_mod_map_iterator_t;

  pub fn xcb_xkb_set_map_values_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *const xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *mut xcb_xkb_set_map_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_values_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_map_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    present: u16,
    flags: u16,
    minKeyCode: xcb_keycode_t,
    maxKeyCode: xcb_keycode_t,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    totalSyms: u16,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    totalActions: u16,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    totalKeyBehaviors: u8,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    totalKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    totalModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
    totalVModMapKeys: u8,
    virtualMods: u16,
    values: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    present: u16,
    flags: u16,
    minKeyCode: xcb_keycode_t,
    maxKeyCode: xcb_keycode_t,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    totalSyms: u16,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    totalActions: u16,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    totalKeyBehaviors: u8,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    totalKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    totalModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
    totalVModMapKeys: u8,
    virtualMods: u16,
    values: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_map_aux_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    present: u16,
    flags: u16,
    minKeyCode: xcb_keycode_t,
    maxKeyCode: xcb_keycode_t,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    totalSyms: u16,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    totalActions: u16,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    totalKeyBehaviors: u8,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    totalKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    totalModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
    totalVModMapKeys: u8,
    virtualMods: u16,
    values: *const xcb_xkb_set_map_values_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_map_aux(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    present: u16,
    flags: u16,
    minKeyCode: xcb_keycode_t,
    maxKeyCode: xcb_keycode_t,
    firstType: u8,
    nTypes: u8,
    firstKeySym: xcb_keycode_t,
    nKeySyms: u8,
    totalSyms: u16,
    firstKeyAction: xcb_keycode_t,
    nKeyActions: u8,
    totalActions: u16,
    firstKeyBehavior: xcb_keycode_t,
    nKeyBehaviors: u8,
    totalKeyBehaviors: u8,
    firstKeyExplicit: xcb_keycode_t,
    nKeyExplicit: u8,
    totalKeyExplicit: u8,
    firstModMapKey: xcb_keycode_t,
    nModMapKeys: u8,
    totalModMapKeys: u8,
    firstVModMapKey: xcb_keycode_t,
    nVModMapKeys: u8,
    totalVModMapKeys: u8,
    virtualMods: u16,
    values: *const xcb_xkb_set_map_values_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_map_values(R: *const xcb_xkb_set_map_request_t)
    -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_get_compat_map_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_compat_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    groups: u8,
    getAllSI: u8,
    firstSI: u16,
    nSI: u16,
  ) -> xcb_xkb_get_compat_map_cookie_t;

  pub fn xcb_xkb_get_compat_map_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    groups: u8,
    getAllSI: u8,
    firstSI: u16,
    nSI: u16,
  ) -> xcb_xkb_get_compat_map_cookie_t;

  pub fn xcb_xkb_get_compat_map_si_rtrn(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> *mut xcb_xkb_sym_interpret_t;

  pub fn xcb_xkb_get_compat_map_si_rtrn_length(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_compat_map_si_rtrn_iterator(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> xcb_xkb_sym_interpret_iterator_t;

  pub fn xcb_xkb_get_compat_map_group_rtrn(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> *mut xcb_xkb_mod_def_t;

  pub fn xcb_xkb_get_compat_map_group_rtrn_length(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_compat_map_group_rtrn_iterator(
    R: *const xcb_xkb_get_compat_map_reply_t
  ) -> xcb_xkb_mod_def_iterator_t;

  pub fn xcb_xkb_get_compat_map_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_compat_map_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_compat_map_reply_t;

  pub fn xcb_xkb_set_compat_map_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_compat_map_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    recomputeActions: u8,
    truncateSI: u8,
    groups: u8,
    firstSI: u16,
    nSI: u16,
    si: *const xcb_xkb_sym_interpret_t,
    groupMaps: *const xcb_xkb_mod_def_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_compat_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    recomputeActions: u8,
    truncateSI: u8,
    groups: u8,
    firstSI: u16,
    nSI: u16,
    si: *const xcb_xkb_sym_interpret_t,
    groupMaps: *const xcb_xkb_mod_def_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_compat_map_si(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> *mut xcb_xkb_sym_interpret_t;

  pub fn xcb_xkb_set_compat_map_si_length(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_compat_map_si_iterator(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> xcb_xkb_sym_interpret_iterator_t;

  pub fn xcb_xkb_set_compat_map_group_maps(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> *mut xcb_xkb_mod_def_t;

  pub fn xcb_xkb_set_compat_map_group_maps_length(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_compat_map_group_maps_iterator(
    R: *const xcb_xkb_set_compat_map_request_t
  ) -> xcb_xkb_mod_def_iterator_t;

  pub fn xcb_xkb_get_indicator_state(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_indicator_state_cookie_t;

  pub fn xcb_xkb_get_indicator_state_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
  ) -> xcb_xkb_get_indicator_state_cookie_t;

  pub fn xcb_xkb_get_indicator_state_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_indicator_state_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_indicator_state_reply_t;

  pub fn xcb_xkb_get_indicator_map_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_indicator_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
  ) -> xcb_xkb_get_indicator_map_cookie_t;

  pub fn xcb_xkb_get_indicator_map_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
  ) -> xcb_xkb_get_indicator_map_cookie_t;

  pub fn xcb_xkb_get_indicator_map_maps(
    R: *const xcb_xkb_get_indicator_map_reply_t
  ) -> *mut xcb_xkb_indicator_map_t;

  pub fn xcb_xkb_get_indicator_map_maps_length(
    R: *const xcb_xkb_get_indicator_map_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_indicator_map_maps_iterator(
    R: *const xcb_xkb_get_indicator_map_reply_t
  ) -> xcb_xkb_indicator_map_iterator_t;

  pub fn xcb_xkb_get_indicator_map_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_indicator_map_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_indicator_map_reply_t;

  pub fn xcb_xkb_set_indicator_map_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_indicator_map_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
    maps: *const xcb_xkb_indicator_map_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_indicator_map(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
    maps: *const xcb_xkb_indicator_map_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_indicator_map_maps(
    R: *const xcb_xkb_set_indicator_map_request_t
  ) -> *mut xcb_xkb_indicator_map_t;

  pub fn xcb_xkb_set_indicator_map_maps_length(
    R: *const xcb_xkb_set_indicator_map_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_indicator_map_maps_iterator(
    R: *const xcb_xkb_set_indicator_map_request_t
  ) -> xcb_xkb_indicator_map_iterator_t;

  pub fn xcb_xkb_get_named_indicator(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
    indicator: xcb_atom_t,
  ) -> xcb_xkb_get_named_indicator_cookie_t;

  pub fn xcb_xkb_get_named_indicator_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
    indicator: xcb_atom_t,
  ) -> xcb_xkb_get_named_indicator_cookie_t;

  pub fn xcb_xkb_get_named_indicator_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_named_indicator_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_named_indicator_reply_t;

  pub fn xcb_xkb_set_named_indicator_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
    indicator: xcb_atom_t,
    setState: u8,
    on: u8,
    setMap: u8,
    createMap: u8,
    map_flags: u8,
    map_whichGroups: u8,
    map_groups: u8,
    map_whichMods: u8,
    map_realMods: u8,
    map_vmods: u16,
    map_ctrls: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_named_indicator(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
    indicator: xcb_atom_t,
    setState: u8,
    on: u8,
    setMap: u8,
    createMap: u8,
    map_flags: u8,
    map_whichGroups: u8,
    map_groups: u8,
    map_whichMods: u8,
    map_realMods: u8,
    map_vmods: u16,
    map_ctrls: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_get_names_value_list_type_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_type_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_type_names_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_n_levels_per_type(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut u8;

  pub fn xcb_xkb_get_names_value_list_n_levels_per_type_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_n_levels_per_type_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_kt_level_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_kt_level_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_kt_level_names_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_indicator_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_indicator_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_indicator_names_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_virtual_mod_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_virtual_mod_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_virtual_mod_names_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_groups(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_groups_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_groups_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_key_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_xkb_key_name_t;

  pub fn xcb_xkb_get_names_value_list_key_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_key_names_iterator(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_xkb_key_name_iterator_t;

  pub fn xcb_xkb_get_names_value_list_key_aliases(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_xkb_key_alias_t;

  pub fn xcb_xkb_get_names_value_list_key_aliases_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_key_aliases_iterator(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_xkb_key_alias_iterator_t;

  pub fn xcb_xkb_get_names_value_list_radio_group_names(
    S: *const xcb_xkb_get_names_value_list_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_names_value_list_radio_group_names_length(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_radio_group_names_end(
    R: *const xcb_xkb_get_names_reply_t,
    S: *const xcb_xkb_get_names_value_list_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_names_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *const xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *mut xcb_xkb_get_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_names(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
  ) -> xcb_xkb_get_names_cookie_t;

  pub fn xcb_xkb_get_names_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    which: u32,
  ) -> xcb_xkb_get_names_cookie_t;

  pub fn xcb_xkb_get_names_value_list(
    R: *const xcb_xkb_get_names_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_get_names_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_names_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_names_reply_t;

  pub fn xcb_xkb_set_names_values_type_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_type_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_type_names_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_n_levels_per_type(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut u8;

  pub fn xcb_xkb_set_names_values_n_levels_per_type_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_n_levels_per_type_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_kt_level_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_kt_level_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_kt_level_names_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_indicator_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_indicator_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_indicator_names_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_virtual_mod_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_virtual_mod_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_virtual_mod_names_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_groups(S: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_groups_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_groups_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_key_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_xkb_key_name_t;

  pub fn xcb_xkb_set_names_values_key_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_key_names_iterator(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_xkb_key_name_iterator_t;

  pub fn xcb_xkb_set_names_values_key_aliases(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_xkb_key_alias_t;

  pub fn xcb_xkb_set_names_values_key_aliases_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_key_aliases_iterator(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_xkb_key_alias_iterator_t;

  pub fn xcb_xkb_set_names_values_radio_group_names(
    S: *const xcb_xkb_set_names_values_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_set_names_values_radio_group_names_length(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_radio_group_names_end(
    R: *const xcb_xkb_set_names_request_t,
    S: *const xcb_xkb_set_names_values_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_set_names_values_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *const xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *mut xcb_xkb_set_names_values_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_values_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_names_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    virtualMods: u16,
    which: u32,
    firstType: u8,
    nTypes: u8,
    firstKTLevelt: u8,
    nKTLevels: u8,
    indicators: u32,
    groupNames: u8,
    nRadioGroups: u8,
    firstKey: xcb_keycode_t,
    nKeys: u8,
    nKeyAliases: u8,
    totalKTLevelNames: u16,
    values: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_names(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    virtualMods: u16,
    which: u32,
    firstType: u8,
    nTypes: u8,
    firstKTLevelt: u8,
    nKTLevels: u8,
    indicators: u32,
    groupNames: u8,
    nRadioGroups: u8,
    firstKey: xcb_keycode_t,
    nKeys: u8,
    nKeyAliases: u8,
    totalKTLevelNames: u16,
    values: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_names_aux_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    virtualMods: u16,
    which: u32,
    firstType: u8,
    nTypes: u8,
    firstKTLevelt: u8,
    nKTLevels: u8,
    indicators: u32,
    groupNames: u8,
    nRadioGroups: u8,
    firstKey: xcb_keycode_t,
    nKeys: u8,
    nKeyAliases: u8,
    totalKTLevelNames: u16,
    values: *const xcb_xkb_set_names_values_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_names_aux(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    virtualMods: u16,
    which: u32,
    firstType: u8,
    nTypes: u8,
    firstKTLevelt: u8,
    nKTLevels: u8,
    indicators: u32,
    groupNames: u8,
    nRadioGroups: u8,
    firstKey: xcb_keycode_t,
    nKeys: u8,
    nKeyAliases: u8,
    totalKTLevelNames: u16,
    values: *const xcb_xkb_set_names_values_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_names_values(
    R: *const xcb_xkb_set_names_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_per_client_flags(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    change: u32,
    value: u32,
    ctrlsToChange: u32,
    autoCtrls: u32,
    autoCtrlsValues: u32,
  ) -> xcb_xkb_per_client_flags_cookie_t;

  pub fn xcb_xkb_per_client_flags_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    change: u32,
    value: u32,
    ctrlsToChange: u32,
    autoCtrls: u32,
    autoCtrlsValues: u32,
  ) -> xcb_xkb_per_client_flags_cookie_t;

  pub fn xcb_xkb_per_client_flags_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_per_client_flags_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_per_client_flags_reply_t;

  pub fn xcb_xkb_list_components_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    maxNames: u16,
  ) -> xcb_xkb_list_components_cookie_t;

  pub fn xcb_xkb_list_components_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    maxNames: u16,
  ) -> xcb_xkb_list_components_cookie_t;

  pub fn xcb_xkb_list_components_keymaps_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_keymaps_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_keycodes_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_keycodes_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_types_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_types_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_compat_maps_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_compat_maps_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_symbols_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_symbols_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_geometries_length(
    R: *const xcb_xkb_list_components_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_list_components_geometries_iterator(
    R: *const xcb_xkb_list_components_reply_t
  ) -> xcb_xkb_listing_iterator_t;

  pub fn xcb_xkb_list_components_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_list_components_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_list_components_reply_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_type_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_sym_map_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut u8;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_action_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_action_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_set_behavior_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_set_behavior_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut u8;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_set_explicit_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_set_explicit_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_key_mod_map_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_mod_map_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_key_v_mod_map_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_v_mod_map_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *const xcb_xkb_get_kbd_by_name_replies_types_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
    _aux: *mut xcb_xkb_get_kbd_by_name_replies_types_map_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_types_map_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    nKeySyms: u8,
    nKeyActions: u8,
    totalActions: u16,
    totalKeyBehaviors: u8,
    virtualMods: u16,
    totalKeyExplicit: u8,
    totalModMapKeys: u8,
    totalVModMapKeys: u8,
    present: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut u8;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_key_name_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_name_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_key_alias_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_key_alias_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_atom_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *const xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
    _aux: *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    nTypes: u8,
    indicators: u32,
    virtualMods: u16,
    groupNames: u8,
    nKeys: u8,
    nKeyAliases: u8,
    nRadioGroups: u8,
    which: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_sym_interpret_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_sym_interpret_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_mod_def_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_mod_def_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps(
    S: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_indicator_map_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator(
    R: *const xcb_xkb_get_kbd_by_name_reply_t,
    S: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> xcb_xkb_indicator_map_iterator_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list(
    R: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_geometry_label_font(
    R: *const xcb_xkb_get_kbd_by_name_replies_t
  ) -> *mut xcb_xkb_counted_string_16_t;

  pub fn xcb_xkb_get_kbd_by_name_replies_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    reported: u16,
    _aux: *const xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_unpack(
    _buffer: *const ::std::os::raw::c_void,
    reported: u16,
    _aux: *mut xcb_xkb_get_kbd_by_name_replies_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_replies_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    reported: u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_kbd_by_name(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    need: u16,
    want: u16,
    load: u8,
  ) -> xcb_xkb_get_kbd_by_name_cookie_t;

  pub fn xcb_xkb_get_kbd_by_name_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    need: u16,
    want: u16,
    load: u8,
  ) -> xcb_xkb_get_kbd_by_name_cookie_t;

  pub fn xcb_xkb_get_kbd_by_name_replies(
    R: *const xcb_xkb_get_kbd_by_name_reply_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_xkb_get_kbd_by_name_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_kbd_by_name_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_kbd_by_name_reply_t;

  pub fn xcb_xkb_get_device_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_device_info(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    wanted: u16,
    allButtons: u8,
    firstButton: u8,
    nButtons: u8,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
  ) -> xcb_xkb_get_device_info_cookie_t;

  pub fn xcb_xkb_get_device_info_unchecked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    wanted: u16,
    allButtons: u8,
    firstButton: u8,
    nButtons: u8,
    ledClass: xcb_xkb_led_class_spec_t,
    ledID: xcb_xkb_id_spec_t,
  ) -> xcb_xkb_get_device_info_cookie_t;

  pub fn xcb_xkb_get_device_info_name(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> *mut xcb_xkb_string8_t;

  pub fn xcb_xkb_get_device_info_name_length(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_device_info_name_end(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_xkb_get_device_info_btn_actions(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> *mut xcb_xkb_action_t;

  pub fn xcb_xkb_get_device_info_btn_actions_length(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_device_info_btn_actions_iterator(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> xcb_xkb_action_iterator_t;

  pub fn xcb_xkb_get_device_info_leds_length(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_get_device_info_leds_iterator(
    R: *const xcb_xkb_get_device_info_reply_t
  ) -> xcb_xkb_device_led_info_iterator_t;

  pub fn xcb_xkb_get_device_info_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_get_device_info_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_get_device_info_reply_t;

  pub fn xcb_xkb_set_device_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_device_info_checked(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    firstBtn: u8,
    nBtns: u8,
    change: u16,
    nDeviceLedFBs: u16,
    btnActions: *const xcb_xkb_action_t,
    leds: *const xcb_xkb_device_led_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_device_info(
    c: *mut xcb_connection_t,
    deviceSpec: xcb_xkb_device_spec_t,
    firstBtn: u8,
    nBtns: u8,
    change: u16,
    nDeviceLedFBs: u16,
    btnActions: *const xcb_xkb_action_t,
    leds: *const xcb_xkb_device_led_info_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_xkb_set_device_info_btn_actions(
    R: *const xcb_xkb_set_device_info_request_t
  ) -> *mut xcb_xkb_action_t;

  pub fn xcb_xkb_set_device_info_btn_actions_length(
    R: *const xcb_xkb_set_device_info_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_device_info_btn_actions_iterator(
    R: *const xcb_xkb_set_device_info_request_t
  ) -> xcb_xkb_action_iterator_t;

  pub fn xcb_xkb_set_device_info_leds_length(
    R: *const xcb_xkb_set_device_info_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_device_info_leds_iterator(
    R: *const xcb_xkb_set_device_info_request_t
  ) -> xcb_xkb_device_led_info_iterator_t;

  pub fn xcb_xkb_set_debugging_flags_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xkb_set_debugging_flags(
    c: *mut xcb_connection_t,
    msgLength: u16,
    affectFlags: u32,
    flags: u32,
    affectCtrls: u32,
    ctrls: u32,
    message: *const xcb_xkb_string8_t,
  ) -> xcb_xkb_set_debugging_flags_cookie_t;

  pub fn xcb_xkb_set_debugging_flags_unchecked(
    c: *mut xcb_connection_t,
    msgLength: u16,
    affectFlags: u32,
    flags: u32,
    affectCtrls: u32,
    ctrls: u32,
    message: *const xcb_xkb_string8_t,
  ) -> xcb_xkb_set_debugging_flags_cookie_t;

  pub fn xcb_xkb_set_debugging_flags_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_xkb_set_debugging_flags_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_xkb_set_debugging_flags_reply_t;
}
