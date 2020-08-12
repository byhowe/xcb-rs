pub const XCB_EVENT_RESPONSE_TYPE_MASK: u32 = 127;

#[link(name = "xcb")]
extern "C" {
  pub fn xcb_event_get_label(type_: u8) -> *const ::std::os::raw::c_char;

  pub fn xcb_event_get_error_label(type_: u8) -> *const ::std::os::raw::c_char;

  pub fn xcb_event_get_request_label(type_: u8) -> *const ::std::os::raw::c_char;
}
