use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_atom_t, xcb_window_t};

pub const XCB_SELINUX_MAJOR_VERSION: u32 = 1;
pub const XCB_SELINUX_MINOR_VERSION: u32 = 0;
pub const XCB_SELINUX_QUERY_VERSION: u32 = 0;
pub const XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT: u32 = 1;
pub const XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT: u32 = 2;
pub const XCB_SELINUX_SET_DEVICE_CONTEXT: u32 = 3;
pub const XCB_SELINUX_GET_DEVICE_CONTEXT: u32 = 4;
pub const XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT: u32 = 5;
pub const XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT: u32 = 6;
pub const XCB_SELINUX_GET_WINDOW_CONTEXT: u32 = 7;
pub const XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT: u32 = 8;
pub const XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT: u32 = 9;
pub const XCB_SELINUX_SET_PROPERTY_USE_CONTEXT: u32 = 10;
pub const XCB_SELINUX_GET_PROPERTY_USE_CONTEXT: u32 = 11;
pub const XCB_SELINUX_GET_PROPERTY_CONTEXT: u32 = 12;
pub const XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT: u32 = 13;
pub const XCB_SELINUX_LIST_PROPERTIES: u32 = 14;
pub const XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT: u32 = 15;
pub const XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT: u32 = 16;
pub const XCB_SELINUX_SET_SELECTION_USE_CONTEXT: u32 = 17;
pub const XCB_SELINUX_GET_SELECTION_USE_CONTEXT: u32 = 18;
pub const XCB_SELINUX_GET_SELECTION_CONTEXT: u32 = 19;
pub const XCB_SELINUX_GET_SELECTION_DATA_CONTEXT: u32 = 20;
pub const XCB_SELINUX_LIST_SELECTIONS: u32 = 21;
pub const XCB_SELINUX_GET_CLIENT_CONTEXT: u32 = 22;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major: u8,
  pub client_minor: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub server_major: u16,
  pub server_minor: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_device_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_create_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_create_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_device_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device: u32,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub device: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_device_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_window_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_create_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_create_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_window_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_item_t
{
  pub name: xcb_atom_t,
  pub object_context_len: u32,
  pub data_context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_item_iterator_t
{
  pub data: *mut xcb_selinux_list_item_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_property_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_create_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_create_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_property_use_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_use_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_use_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_use_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_data_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_data_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
  pub property: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_property_data_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_properties_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_properties_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_properties_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub properties_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_selection_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_create_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_create_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_set_selection_use_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_use_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_use_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_use_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub selection: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_data_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_data_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub selection: xcb_atom_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_selection_data_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_selections_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_selections_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_list_selections_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub selections_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_client_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_client_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub resource: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selinux_get_client_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context_len: u32,
  pub pad1: [u8; 20usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_selinux_id: xcb_extension_t;

  pub fn xcb_selinux_query_version(
    c: *mut xcb_connection_t,
    client_major: u8,
    client_minor: u8,
  ) -> xcb_selinux_query_version_cookie_t;

  pub fn xcb_selinux_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major: u8,
    client_minor: u8,
  ) -> xcb_selinux_query_version_cookie_t;

  pub fn xcb_selinux_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_query_version_reply_t;

  pub fn xcb_selinux_set_device_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_device_create_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_device_create_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_device_create_context_context(
    R: *const xcb_selinux_set_device_create_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_device_create_context_context_length(
    R: *const xcb_selinux_set_device_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_device_create_context_context_end(
    R: *const xcb_selinux_set_device_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_device_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_device_create_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_device_create_context_cookie_t;

  pub fn xcb_selinux_get_device_create_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_device_create_context_cookie_t;

  pub fn xcb_selinux_get_device_create_context_context(
    R: *const xcb_selinux_get_device_create_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_device_create_context_context_length(
    R: *const xcb_selinux_get_device_create_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_device_create_context_context_end(
    R: *const xcb_selinux_get_device_create_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_device_create_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_device_create_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_device_create_context_reply_t;

  pub fn xcb_selinux_set_device_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_device_context_checked(
    c: *mut xcb_connection_t,
    device: u32,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_device_context(
    c: *mut xcb_connection_t,
    device: u32,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_device_context_context(
    R: *const xcb_selinux_set_device_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_device_context_context_length(
    R: *const xcb_selinux_set_device_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_device_context_context_end(
    R: *const xcb_selinux_set_device_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_device_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_device_context(
    c: *mut xcb_connection_t,
    device: u32,
  ) -> xcb_selinux_get_device_context_cookie_t;

  pub fn xcb_selinux_get_device_context_unchecked(
    c: *mut xcb_connection_t,
    device: u32,
  ) -> xcb_selinux_get_device_context_cookie_t;

  pub fn xcb_selinux_get_device_context_context(
    R: *const xcb_selinux_get_device_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_device_context_context_length(
    R: *const xcb_selinux_get_device_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_device_context_context_end(
    R: *const xcb_selinux_get_device_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_device_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_device_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_device_context_reply_t;

  pub fn xcb_selinux_set_window_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_window_create_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_window_create_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_window_create_context_context(
    R: *const xcb_selinux_set_window_create_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_window_create_context_context_length(
    R: *const xcb_selinux_set_window_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_window_create_context_context_end(
    R: *const xcb_selinux_set_window_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_window_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_window_create_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_window_create_context_cookie_t;

  pub fn xcb_selinux_get_window_create_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_window_create_context_cookie_t;

  pub fn xcb_selinux_get_window_create_context_context(
    R: *const xcb_selinux_get_window_create_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_window_create_context_context_length(
    R: *const xcb_selinux_get_window_create_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_window_create_context_context_end(
    R: *const xcb_selinux_get_window_create_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_window_create_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_window_create_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_window_create_context_reply_t;

  pub fn xcb_selinux_get_window_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_window_context(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_selinux_get_window_context_cookie_t;

  pub fn xcb_selinux_get_window_context_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_selinux_get_window_context_cookie_t;

  pub fn xcb_selinux_get_window_context_context(
    R: *const xcb_selinux_get_window_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_window_context_context_length(
    R: *const xcb_selinux_get_window_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_window_context_context_end(
    R: *const xcb_selinux_get_window_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_window_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_window_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_window_context_reply_t;

  pub fn xcb_selinux_list_item_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_item_object_context(
    R: *const xcb_selinux_list_item_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_list_item_object_context_length(
    R: *const xcb_selinux_list_item_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_item_object_context_end(
    R: *const xcb_selinux_list_item_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_list_item_data_context(
    R: *const xcb_selinux_list_item_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_list_item_data_context_length(
    R: *const xcb_selinux_list_item_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_item_data_context_end(
    R: *const xcb_selinux_list_item_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_list_item_next(i: *mut xcb_selinux_list_item_iterator_t);

  pub fn xcb_selinux_list_item_end(i: xcb_selinux_list_item_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_set_property_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_property_create_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_property_create_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_property_create_context_context(
    R: *const xcb_selinux_set_property_create_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_property_create_context_context_length(
    R: *const xcb_selinux_set_property_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_property_create_context_context_end(
    R: *const xcb_selinux_set_property_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_create_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_property_create_context_cookie_t;

  pub fn xcb_selinux_get_property_create_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_property_create_context_cookie_t;

  pub fn xcb_selinux_get_property_create_context_context(
    R: *const xcb_selinux_get_property_create_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_property_create_context_context_length(
    R: *const xcb_selinux_get_property_create_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_create_context_context_end(
    R: *const xcb_selinux_get_property_create_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_create_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_property_create_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_property_create_context_reply_t;

  pub fn xcb_selinux_set_property_use_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_property_use_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_property_use_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_property_use_context_context(
    R: *const xcb_selinux_set_property_use_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_property_use_context_context_length(
    R: *const xcb_selinux_set_property_use_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_property_use_context_context_end(
    R: *const xcb_selinux_set_property_use_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_use_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_use_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_property_use_context_cookie_t;

  pub fn xcb_selinux_get_property_use_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_property_use_context_cookie_t;

  pub fn xcb_selinux_get_property_use_context_context(
    R: *const xcb_selinux_get_property_use_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_property_use_context_context_length(
    R: *const xcb_selinux_get_property_use_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_use_context_context_end(
    R: *const xcb_selinux_get_property_use_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_use_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_property_use_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_property_use_context_reply_t;

  pub fn xcb_selinux_get_property_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_context(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_selinux_get_property_context_cookie_t;

  pub fn xcb_selinux_get_property_context_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_selinux_get_property_context_cookie_t;

  pub fn xcb_selinux_get_property_context_context(
    R: *const xcb_selinux_get_property_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_property_context_context_length(
    R: *const xcb_selinux_get_property_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_context_context_end(
    R: *const xcb_selinux_get_property_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_property_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_property_context_reply_t;

  pub fn xcb_selinux_get_property_data_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_data_context(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_selinux_get_property_data_context_cookie_t;

  pub fn xcb_selinux_get_property_data_context_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
  ) -> xcb_selinux_get_property_data_context_cookie_t;

  pub fn xcb_selinux_get_property_data_context_context(
    R: *const xcb_selinux_get_property_data_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_property_data_context_context_length(
    R: *const xcb_selinux_get_property_data_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_property_data_context_context_end(
    R: *const xcb_selinux_get_property_data_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_property_data_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_property_data_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_property_data_context_reply_t;

  pub fn xcb_selinux_list_properties_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_properties(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_selinux_list_properties_cookie_t;

  pub fn xcb_selinux_list_properties_unchecked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_selinux_list_properties_cookie_t;

  pub fn xcb_selinux_list_properties_properties_length(
    R: *const xcb_selinux_list_properties_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_properties_properties_iterator(
    R: *const xcb_selinux_list_properties_reply_t
  ) -> xcb_selinux_list_item_iterator_t;

  pub fn xcb_selinux_list_properties_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_list_properties_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_list_properties_reply_t;

  pub fn xcb_selinux_set_selection_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_selection_create_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_selection_create_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_selection_create_context_context(
    R: *const xcb_selinux_set_selection_create_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_selection_create_context_context_length(
    R: *const xcb_selinux_set_selection_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_selection_create_context_context_end(
    R: *const xcb_selinux_set_selection_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_create_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_selection_create_context_cookie_t;

  pub fn xcb_selinux_get_selection_create_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_selection_create_context_cookie_t;

  pub fn xcb_selinux_get_selection_create_context_context(
    R: *const xcb_selinux_get_selection_create_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_selection_create_context_context_length(
    R: *const xcb_selinux_get_selection_create_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_create_context_context_end(
    R: *const xcb_selinux_get_selection_create_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_create_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_selection_create_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_selection_create_context_reply_t;

  pub fn xcb_selinux_set_selection_use_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_selection_use_context_checked(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_selection_use_context(
    c: *mut xcb_connection_t,
    context_len: u32,
    context: *const ::std::os::raw::c_char,
  ) -> xcb_void_cookie_t;

  pub fn xcb_selinux_set_selection_use_context_context(
    R: *const xcb_selinux_set_selection_use_context_request_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_set_selection_use_context_context_length(
    R: *const xcb_selinux_set_selection_use_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_set_selection_use_context_context_end(
    R: *const xcb_selinux_set_selection_use_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_use_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_use_context(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_selection_use_context_cookie_t;

  pub fn xcb_selinux_get_selection_use_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_get_selection_use_context_cookie_t;

  pub fn xcb_selinux_get_selection_use_context_context(
    R: *const xcb_selinux_get_selection_use_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_selection_use_context_context_length(
    R: *const xcb_selinux_get_selection_use_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_use_context_context_end(
    R: *const xcb_selinux_get_selection_use_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_use_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_selection_use_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_selection_use_context_reply_t;

  pub fn xcb_selinux_get_selection_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_context(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_selinux_get_selection_context_cookie_t;

  pub fn xcb_selinux_get_selection_context_unchecked(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_selinux_get_selection_context_cookie_t;

  pub fn xcb_selinux_get_selection_context_context(
    R: *const xcb_selinux_get_selection_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_selection_context_context_length(
    R: *const xcb_selinux_get_selection_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_context_context_end(
    R: *const xcb_selinux_get_selection_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_selection_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_selection_context_reply_t;

  pub fn xcb_selinux_get_selection_data_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_data_context(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_selinux_get_selection_data_context_cookie_t;

  pub fn xcb_selinux_get_selection_data_context_unchecked(
    c: *mut xcb_connection_t,
    selection: xcb_atom_t,
  ) -> xcb_selinux_get_selection_data_context_cookie_t;

  pub fn xcb_selinux_get_selection_data_context_context(
    R: *const xcb_selinux_get_selection_data_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_selection_data_context_context_length(
    R: *const xcb_selinux_get_selection_data_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_selection_data_context_context_end(
    R: *const xcb_selinux_get_selection_data_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_selection_data_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_selection_data_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_selection_data_context_reply_t;

  pub fn xcb_selinux_list_selections_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_selections(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_list_selections_cookie_t;

  pub fn xcb_selinux_list_selections_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_selinux_list_selections_cookie_t;

  pub fn xcb_selinux_list_selections_selections_length(
    R: *const xcb_selinux_list_selections_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_list_selections_selections_iterator(
    R: *const xcb_selinux_list_selections_reply_t
  ) -> xcb_selinux_list_item_iterator_t;

  pub fn xcb_selinux_list_selections_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_list_selections_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_list_selections_reply_t;

  pub fn xcb_selinux_get_client_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_client_context(
    c: *mut xcb_connection_t,
    resource: u32,
  ) -> xcb_selinux_get_client_context_cookie_t;

  pub fn xcb_selinux_get_client_context_unchecked(
    c: *mut xcb_connection_t,
    resource: u32,
  ) -> xcb_selinux_get_client_context_cookie_t;

  pub fn xcb_selinux_get_client_context_context(
    R: *const xcb_selinux_get_client_context_reply_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_selinux_get_client_context_context_length(
    R: *const xcb_selinux_get_client_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_selinux_get_client_context_context_end(
    R: *const xcb_selinux_get_client_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_selinux_get_client_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_selinux_get_client_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_selinux_get_client_context_reply_t;
}
