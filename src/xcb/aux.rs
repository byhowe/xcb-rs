pub use crate::ffi::aux::{
  xcb_aux_change_gc as change_gc,
  xcb_aux_change_gc_checked as change_gc_checked,
  xcb_aux_change_keyboard_control as change_keyboard_control,
  xcb_aux_change_window_attributes as change_window_attributes,
  xcb_aux_change_window_attributes_checked as change_window_attributes_checked,
  xcb_aux_clear_window as clear_window,
  xcb_aux_configure_window as configure_window,
  xcb_aux_create_gc as create_gc,
  xcb_aux_create_gc_checked as create_gc_checked,
  xcb_aux_create_window as create_window,
  xcb_aux_create_window_checked as create_window_checked,
  xcb_aux_find_visual_by_attrs as find_visual_by_attrs,
  xcb_aux_find_visual_by_id as find_visual_by_id,
  xcb_aux_get_depth as get_depth,
  xcb_aux_get_depth_of_visual as get_depth_of_visual,
  xcb_aux_get_screen as get_screen,
  xcb_aux_get_visualtype as get_visualtype,
  xcb_aux_parse_color as parse_color,
  xcb_aux_set_line_attributes_checked as set_line_attributes_checked,
  xcb_params_configure_window_t as params_configure_window_t,
  xcb_params_cw_t as params_cw_t,
  xcb_params_gc_t as params_gc_t,
  xcb_params_keyboard_t as params_keyboard_t,
};
