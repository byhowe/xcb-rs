use super::core::{xcb_connection_t, xcb_void_cookie_t};
use super::xcb::{
  xcb_drawable_t,
  xcb_gcontext_t,
  xcb_screen_t,
  xcb_visualid_t,
  xcb_visualtype_t,
  xcb_window_t,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_params_cw_t
{
  pub back_pixmap: u32,
  pub back_pixel: u32,
  pub border_pixmap: u32,
  pub border_pixel: u32,
  pub bit_gravity: u32,
  pub win_gravity: u32,
  pub backing_store: u32,
  pub backing_planes: u32,
  pub backing_pixel: u32,
  pub override_redirect: u32,
  pub save_under: u32,
  pub event_mask: u32,
  pub dont_propagate: u32,
  pub colormap: u32,
  pub cursor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_params_configure_window_t
{
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub border_width: u32,
  pub sibling: u32,
  pub stack_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_params_gc_t
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
  pub tile: u32,
  pub stipple: u32,
  pub tile_stipple_origin_x: u32,
  pub tile_stipple_origin_y: u32,
  pub font: u32,
  pub subwindow_mode: u32,
  pub graphics_exposures: u32,
  pub clip_originX: u32,
  pub clip_originY: u32,
  pub mask: u32,
  pub dash_offset: u32,
  pub dash_list: u32,
  pub arc_mode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_params_keyboard_t
{
  pub key_click_percent: u32,
  pub bell_percent: u32,
  pub bell_pitch: u32,
  pub bell_duration: u32,
  pub led: u32,
  pub led_mode: u32,
  pub key: u32,
  pub auto_repeat_mode: u32,
}

#[link(name = "xcb")]
extern "C" {
  pub fn xcb_aux_get_depth(
    c: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
  ) -> u8;

  pub fn xcb_aux_get_depth_of_visual(
    screen: *mut xcb_screen_t,
    id: xcb_visualid_t,
  ) -> u8;

  pub fn xcb_aux_get_screen(
    c: *mut xcb_connection_t,
    screen: ::std::os::raw::c_int,
  ) -> *mut xcb_screen_t;

  pub fn xcb_aux_get_visualtype(
    c: *mut xcb_connection_t,
    screen: ::std::os::raw::c_int,
    vid: xcb_visualid_t,
  ) -> *mut xcb_visualtype_t;

  pub fn xcb_aux_find_visual_by_id(
    screen: *mut xcb_screen_t,
    id: xcb_visualid_t,
  ) -> *mut xcb_visualtype_t;

  pub fn xcb_aux_find_visual_by_attrs(
    screen: *mut xcb_screen_t,
    class_: i8,
    depth: i8,
  ) -> *mut xcb_visualtype_t;

  pub fn xcb_aux_sync(c: *mut xcb_connection_t);

  pub fn xcb_aux_create_window(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    class_: u16,
    visual: xcb_visualid_t,
    mask: u32,
    params: *const xcb_params_cw_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_create_window_checked(
    c: *mut xcb_connection_t,
    depth: u8,
    wid: xcb_window_t,
    parent: xcb_window_t,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    class_: u16,
    visual: xcb_visualid_t,
    mask: u32,
    params: *const xcb_params_cw_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_change_window_attributes(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    mask: u32,
    params: *const xcb_params_cw_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_change_window_attributes_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    mask: u32,
    params: *const xcb_params_cw_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_configure_window(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    mask: u16,
    params: *const xcb_params_configure_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_create_gc(
    c: *mut xcb_connection_t,
    cid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    mask: u32,
    params: *const xcb_params_gc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_create_gc_checked(
    c: *mut xcb_connection_t,
    gid: xcb_gcontext_t,
    drawable: xcb_drawable_t,
    mask: u32,
    params: *const xcb_params_gc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_change_gc(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    mask: u32,
    params: *const xcb_params_gc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_change_gc_checked(
    c: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    mask: u32,
    params: *const xcb_params_gc_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_change_keyboard_control(
    c: *mut xcb_connection_t,
    mask: u32,
    params: *const xcb_params_keyboard_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_parse_color(
    color_name: *const ::std::os::raw::c_char,
    red: *mut u16,
    green: *mut u16,
    blue: *mut u16,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_aux_set_line_attributes_checked(
    dpy: *mut xcb_connection_t,
    gc: xcb_gcontext_t,
    linewidth: u16,
    linestyle: i32,
    capstyle: i32,
    joinstyle: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_aux_clear_window(
    dpy: *mut xcb_connection_t,
    w: xcb_window_t,
  ) -> xcb_void_cookie_t;
}
