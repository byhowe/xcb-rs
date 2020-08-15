use super::core::xcb_connection_t;
use super::xcb::{
  xcb_key_press_event_t,
  xcb_key_release_event_t,
  xcb_keycode_t,
  xcb_keysym_t,
  xcb_mapping_notify_event_t,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XCBKeySymbols
{
  _unused: [u8; 0],
}

pub type xcb_key_symbols_t = _XCBKeySymbols;

#[link(name = "xcb-keysyms")]
extern "C" {
  pub fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;

  pub fn xcb_key_symbols_free(syms: *mut xcb_key_symbols_t);

  pub fn xcb_key_symbols_get_keysym(
    syms: *mut xcb_key_symbols_t,
    keycode: xcb_keycode_t,
    col: ::std::os::raw::c_int,
  ) -> xcb_keysym_t;

  pub fn xcb_key_symbols_get_keycode(
    syms: *mut xcb_key_symbols_t,
    keysym: xcb_keysym_t,
  ) -> *mut xcb_keycode_t;

  pub fn xcb_key_press_lookup_keysym(
    syms: *mut xcb_key_symbols_t,
    event: *mut xcb_key_press_event_t,
    col: ::std::os::raw::c_int,
  ) -> xcb_keysym_t;

  pub fn xcb_key_release_lookup_keysym(
    syms: *mut xcb_key_symbols_t,
    event: *mut xcb_key_release_event_t,
    col: ::std::os::raw::c_int,
  ) -> xcb_keysym_t;

  pub fn xcb_refresh_keyboard_mapping(
    syms: *mut xcb_key_symbols_t,
    event: *mut xcb_mapping_notify_event_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_is_keypad_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_private_keypad_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_cursor_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_pf_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_function_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_misc_function_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;

  pub fn xcb_is_modifier_key(keysym: xcb_keysym_t) -> ::std::os::raw::c_int;
}
