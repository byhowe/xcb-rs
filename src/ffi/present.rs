use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::randr::xcb_randr_crtc_t;
use super::sync::xcb_sync_fence_t;
use super::xcb::{xcb_pixmap_t, xcb_rectangle_t, xcb_window_t};
use super::xfixes::xcb_xfixes_region_t;

pub const XCB_PRESENT_MAJOR_VERSION: u32 = 1;
pub const XCB_PRESENT_MINOR_VERSION: u32 = 2;
pub const XCB_PRESENT_QUERY_VERSION: u32 = 0;
pub const XCB_PRESENT_PIXMAP: u32 = 1;
pub const XCB_PRESENT_NOTIFY_MSC: u32 = 2;
pub const XCB_PRESENT_SELECT_INPUT: u32 = 3;
pub const XCB_PRESENT_QUERY_CAPABILITIES: u32 = 4;
pub const XCB_PRESENT_GENERIC: u32 = 0;
pub const XCB_PRESENT_CONFIGURE_NOTIFY: u32 = 0;
pub const XCB_PRESENT_COMPLETE_NOTIFY: u32 = 1;
pub const XCB_PRESENT_IDLE_NOTIFY: u32 = 2;
pub const XCB_PRESENT_REDIRECT_NOTIFY: u32 = 3;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_event_enum_t
{
  CONFIGURE_NOTIFY = 0,
  COMPLETE_NOTIFY = 1,
  IDLE_NOTIFY = 2,
  REDIRECT_NOTIFY = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_event_mask_t
{
  NO_EVENT = 0,
  CONFIGURE_NOTIFY = 1,
  COMPLETE_NOTIFY = 2,
  IDLE_NOTIFY = 4,
  REDIRECT_NOTIFY = 8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_option_t
{
  NONE = 0,
  ASYNC = 1,
  COPY = 2,
  UST = 4,
  SUBOPTIMAL = 8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_capability_t
{
  NONE = 0,
  ASYNC = 1,
  FENCE = 2,
  UST = 4,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_complete_kind_t
{
  PIXMAP = 0,
  NOTIFY_MSC = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_present_complete_mode_t
{
  COPY = 0,
  FLIP = 1,
  SKIP = 2,
  SUBOPTIMAL_COPY = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_notify_t
{
  pub window: xcb_window_t,
  pub serial: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_notify_iterator_t
{
  pub data: *mut xcb_present_notify_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u32,
  pub minor_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_pixmap_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub pixmap: xcb_pixmap_t,
  pub serial: u32,
  pub valid: xcb_xfixes_region_t,
  pub update: xcb_xfixes_region_t,
  pub x_off: i16,
  pub y_off: i16,
  pub target_crtc: xcb_randr_crtc_t,
  pub wait_fence: xcb_sync_fence_t,
  pub idle_fence: xcb_sync_fence_t,
  pub options: u32,
  pub pad0: [u8; 4usize],
  pub target_msc: u64,
  pub divisor: u64,
  pub remainder: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_notify_msc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub serial: u32,
  pub pad0: [u8; 4usize],
  pub target_msc: u64,
  pub divisor: u64,
  pub remainder: u64,
}

pub type xcb_present_event_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_event_iterator_t
{
  pub data: *mut xcb_present_event_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub eid: xcb_present_event_t,
  pub window: xcb_window_t,
  pub event_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_capabilities_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_capabilities_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub target: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_query_capabilities_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub capabilities: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_generic_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub evtype: u16,
  pub pad0: [u8; 2usize],
  pub event: xcb_present_event_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_configure_notify_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub pad0: [u8; 2usize],
  pub event: xcb_present_event_t,
  pub window: xcb_window_t,
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub off_x: i16,
  pub off_y: i16,
  pub full_sequence: u32,
  pub pixmap_width: u16,
  pub pixmap_height: u16,
  pub pixmap_flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_complete_notify_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub kind: u8,
  pub mode: u8,
  pub event: xcb_present_event_t,
  pub window: xcb_window_t,
  pub serial: u32,
  pub ust: u64,
  pub full_sequence: u32,
  pub msc: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_idle_notify_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub pad0: [u8; 2usize],
  pub event: xcb_present_event_t,
  pub window: xcb_window_t,
  pub serial: u32,
  pub pixmap: xcb_pixmap_t,
  pub idle_fence: xcb_sync_fence_t,
  pub full_sequence: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_present_redirect_notify_event_t
{
  pub response_type: u8,
  pub extension: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_type: u16,
  pub update_window: u8,
  pub pad0: u8,
  pub event: xcb_present_event_t,
  pub event_window: xcb_window_t,
  pub window: xcb_window_t,
  pub pixmap: xcb_pixmap_t,
  pub serial: u32,
  pub full_sequence: u32,
  pub valid_region: xcb_xfixes_region_t,
  pub update_region: xcb_xfixes_region_t,
  pub valid_rect: xcb_rectangle_t,
  pub update_rect: xcb_rectangle_t,
  pub x_off: i16,
  pub y_off: i16,
  pub target_crtc: xcb_randr_crtc_t,
  pub wait_fence: xcb_sync_fence_t,
  pub idle_fence: xcb_sync_fence_t,
  pub options: u32,
  pub pad1: [u8; 4usize],
  pub target_msc: u64,
  pub divisor: u64,
  pub remainder: u64,
}

#[link(name = "xcb-present")]
extern "C" {
  pub static mut xcb_present_id: xcb_extension_t;

  pub fn xcb_present_notify_next(i: *mut xcb_present_notify_iterator_t);

  pub fn xcb_present_notify_end(i: xcb_present_notify_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_present_query_version(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_present_query_version_cookie_t;

  pub fn xcb_present_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u32,
    minor_version: u32,
  ) -> xcb_present_query_version_cookie_t;

  pub fn xcb_present_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_present_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_present_query_version_reply_t;

  pub fn xcb_present_pixmap_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    notifies_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_present_pixmap_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    pixmap: xcb_pixmap_t,
    serial: u32,
    valid: xcb_xfixes_region_t,
    update: xcb_xfixes_region_t,
    x_off: i16,
    y_off: i16,
    target_crtc: xcb_randr_crtc_t,
    wait_fence: xcb_sync_fence_t,
    idle_fence: xcb_sync_fence_t,
    options: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    notifies_len: u32,
    notifies: *const xcb_present_notify_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_pixmap(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    pixmap: xcb_pixmap_t,
    serial: u32,
    valid: xcb_xfixes_region_t,
    update: xcb_xfixes_region_t,
    x_off: i16,
    y_off: i16,
    target_crtc: xcb_randr_crtc_t,
    wait_fence: xcb_sync_fence_t,
    idle_fence: xcb_sync_fence_t,
    options: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
    notifies_len: u32,
    notifies: *const xcb_present_notify_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_pixmap_notifies(
    R: *const xcb_present_pixmap_request_t
  ) -> *mut xcb_present_notify_t;

  pub fn xcb_present_pixmap_notifies_length(
    R: *const xcb_present_pixmap_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_present_pixmap_notifies_iterator(
    R: *const xcb_present_pixmap_request_t
  ) -> xcb_present_notify_iterator_t;

  pub fn xcb_present_notify_msc_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    serial: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_notify_msc(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    serial: u32,
    target_msc: u64,
    divisor: u64,
    remainder: u64,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_event_next(i: *mut xcb_present_event_iterator_t);

  pub fn xcb_present_event_end(i: xcb_present_event_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_present_select_input_checked(
    c: *mut xcb_connection_t,
    eid: xcb_present_event_t,
    window: xcb_window_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_select_input(
    c: *mut xcb_connection_t,
    eid: xcb_present_event_t,
    window: xcb_window_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_present_query_capabilities(
    c: *mut xcb_connection_t,
    target: u32,
  ) -> xcb_present_query_capabilities_cookie_t;

  pub fn xcb_present_query_capabilities_unchecked(
    c: *mut xcb_connection_t,
    target: u32,
  ) -> xcb_present_query_capabilities_cookie_t;

  pub fn xcb_present_query_capabilities_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_present_query_capabilities_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_present_query_capabilities_reply_t;

  pub fn xcb_present_redirect_notify_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    notifies_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_present_redirect_notify_notifies(
    R: *const xcb_present_redirect_notify_event_t
  ) -> *mut xcb_present_notify_t;

  pub fn xcb_present_redirect_notify_notifies_length(
    R: *const xcb_present_redirect_notify_event_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_present_redirect_notify_notifies_iterator(
    R: *const xcb_present_redirect_notify_event_t
  ) -> xcb_present_notify_iterator_t;
}
