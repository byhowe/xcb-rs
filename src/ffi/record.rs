use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};

pub const XCB_RECORD_MAJOR_VERSION: u32 = 1;
pub const XCB_RECORD_MINOR_VERSION: u32 = 13;
pub const XCB_RECORD_BAD_CONTEXT: u32 = 0;
pub const XCB_RECORD_QUERY_VERSION: u32 = 0;
pub const XCB_RECORD_CREATE_CONTEXT: u32 = 1;
pub const XCB_RECORD_REGISTER_CLIENTS: u32 = 2;
pub const XCB_RECORD_UNREGISTER_CLIENTS: u32 = 3;
pub const XCB_RECORD_GET_CONTEXT: u32 = 4;
pub const XCB_RECORD_ENABLE_CONTEXT: u32 = 5;
pub const XCB_RECORD_DISABLE_CONTEXT: u32 = 6;
pub const XCB_RECORD_FREE_CONTEXT: u32 = 7;

pub type xcb_record_context_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_context_iterator_t
{
  pub data: *mut xcb_record_context_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_8_t
{
  pub first: u8,
  pub last: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_8_iterator_t
{
  pub data: *mut xcb_record_range_8_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_16_t
{
  pub first: u16,
  pub last: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_16_iterator_t
{
  pub data: *mut xcb_record_range_16_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_ext_range_t
{
  pub major: xcb_record_range_8_t,
  pub minor: xcb_record_range_16_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_ext_range_iterator_t
{
  pub data: *mut xcb_record_ext_range_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_t
{
  pub core_requests: xcb_record_range_8_t,
  pub core_replies: xcb_record_range_8_t,
  pub ext_requests: xcb_record_ext_range_t,
  pub ext_replies: xcb_record_ext_range_t,
  pub delivered_events: xcb_record_range_8_t,
  pub device_events: xcb_record_range_8_t,
  pub errors: xcb_record_range_8_t,
  pub client_started: u8,
  pub client_died: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_range_iterator_t
{
  pub data: *mut xcb_record_range_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_record_element_header_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_element_header_iterator_t
{
  pub data: *mut xcb_record_element_header_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_record_h_type_t
{
  FROM_SERVER_TIME = 1,
  FROM_CLIENT_TIME = 2,
  FROM_CLIENT_SEQUENCE = 4,
}

pub type xcb_record_client_spec_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_client_spec_iterator_t
{
  pub data: *mut xcb_record_client_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_record_cs_t
{
  CURRENT_CLIENTS = 1,
  FUTURE_CLIENTS = 2,
  ALL_CLIENTS = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_client_info_t
{
  pub client_resource: xcb_record_client_spec_t,
  pub num_ranges: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_client_info_iterator_t
{
  pub data: *mut xcb_record_client_info_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_bad_context_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub invalid_record: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub major_version: u16,
  pub minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_query_version_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u16,
  pub minor_version: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
  pub element_header: xcb_record_element_header_t,
  pub pad0: [u8; 3usize],
  pub num_client_specs: u32,
  pub num_ranges: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_register_clients_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
  pub element_header: xcb_record_element_header_t,
  pub pad0: [u8; 3usize],
  pub num_client_specs: u32,
  pub num_ranges: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_unregister_clients_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
  pub num_client_specs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_get_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_get_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_get_context_reply_t
{
  pub response_type: u8,
  pub enabled: u8,
  pub sequence: u16,
  pub length: u32,
  pub element_header: xcb_record_element_header_t,
  pub pad0: [u8; 3usize],
  pub num_intercepted_clients: u32,
  pub pad1: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_enable_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_enable_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_enable_context_reply_t
{
  pub response_type: u8,
  pub category: u8,
  pub sequence: u16,
  pub length: u32,
  pub element_header: xcb_record_element_header_t,
  pub client_swapped: u8,
  pub pad0: [u8; 2usize],
  pub xid_base: u32,
  pub server_time: u32,
  pub rec_sequence_num: u32,
  pub pad1: [u8; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_disable_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_record_free_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_record_context_t,
}

#[link(name = "xcb-record")]
extern "C" {
  pub static mut xcb_record_id: xcb_extension_t;

  pub fn xcb_record_context_next(i: *mut xcb_record_context_iterator_t);

  pub fn xcb_record_context_end(i: xcb_record_context_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_record_range_8_next(i: *mut xcb_record_range_8_iterator_t);

  pub fn xcb_record_range_8_end(i: xcb_record_range_8_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_record_range_16_next(i: *mut xcb_record_range_16_iterator_t);

  pub fn xcb_record_range_16_end(i: xcb_record_range_16_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_record_ext_range_next(i: *mut xcb_record_ext_range_iterator_t);

  pub fn xcb_record_ext_range_end(i: xcb_record_ext_range_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_record_range_next(i: *mut xcb_record_range_iterator_t);

  pub fn xcb_record_range_end(i: xcb_record_range_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_record_element_header_next(i: *mut xcb_record_element_header_iterator_t);

  pub fn xcb_record_element_header_end(
    i: xcb_record_element_header_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_record_client_spec_next(i: *mut xcb_record_client_spec_iterator_t);

  pub fn xcb_record_client_spec_end(i: xcb_record_client_spec_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_record_client_info_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_client_info_ranges(
    R: *const xcb_record_client_info_t
  ) -> *mut xcb_record_range_t;

  pub fn xcb_record_client_info_ranges_length(
    R: *const xcb_record_client_info_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_client_info_ranges_iterator(
    R: *const xcb_record_client_info_t
  ) -> xcb_record_range_iterator_t;

  pub fn xcb_record_client_info_next(i: *mut xcb_record_client_info_iterator_t);

  pub fn xcb_record_client_info_end(i: xcb_record_client_info_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_record_query_version(
    c: *mut xcb_connection_t,
    major_version: u16,
    minor_version: u16,
  ) -> xcb_record_query_version_cookie_t;

  pub fn xcb_record_query_version_unchecked(
    c: *mut xcb_connection_t,
    major_version: u16,
    minor_version: u16,
  ) -> xcb_record_query_version_cookie_t;

  pub fn xcb_record_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_record_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_record_query_version_reply_t;

  pub fn xcb_record_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_create_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    element_header: xcb_record_element_header_t,
    num_client_specs: u32,
    num_ranges: u32,
    client_specs: *const xcb_record_client_spec_t,
    ranges: *const xcb_record_range_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_create_context(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    element_header: xcb_record_element_header_t,
    num_client_specs: u32,
    num_ranges: u32,
    client_specs: *const xcb_record_client_spec_t,
    ranges: *const xcb_record_range_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_create_context_client_specs(
    R: *const xcb_record_create_context_request_t
  ) -> *mut xcb_record_client_spec_t;

  pub fn xcb_record_create_context_client_specs_length(
    R: *const xcb_record_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_create_context_client_specs_end(
    R: *const xcb_record_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_record_create_context_ranges(
    R: *const xcb_record_create_context_request_t
  ) -> *mut xcb_record_range_t;

  pub fn xcb_record_create_context_ranges_length(
    R: *const xcb_record_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_create_context_ranges_iterator(
    R: *const xcb_record_create_context_request_t
  ) -> xcb_record_range_iterator_t;

  pub fn xcb_record_register_clients_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_register_clients_checked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    element_header: xcb_record_element_header_t,
    num_client_specs: u32,
    num_ranges: u32,
    client_specs: *const xcb_record_client_spec_t,
    ranges: *const xcb_record_range_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_register_clients(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    element_header: xcb_record_element_header_t,
    num_client_specs: u32,
    num_ranges: u32,
    client_specs: *const xcb_record_client_spec_t,
    ranges: *const xcb_record_range_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_register_clients_client_specs(
    R: *const xcb_record_register_clients_request_t
  ) -> *mut xcb_record_client_spec_t;

  pub fn xcb_record_register_clients_client_specs_length(
    R: *const xcb_record_register_clients_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_register_clients_client_specs_end(
    R: *const xcb_record_register_clients_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_record_register_clients_ranges(
    R: *const xcb_record_register_clients_request_t
  ) -> *mut xcb_record_range_t;

  pub fn xcb_record_register_clients_ranges_length(
    R: *const xcb_record_register_clients_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_register_clients_ranges_iterator(
    R: *const xcb_record_register_clients_request_t
  ) -> xcb_record_range_iterator_t;

  pub fn xcb_record_unregister_clients_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_unregister_clients_checked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    num_client_specs: u32,
    client_specs: *const xcb_record_client_spec_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_unregister_clients(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
    num_client_specs: u32,
    client_specs: *const xcb_record_client_spec_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_unregister_clients_client_specs(
    R: *const xcb_record_unregister_clients_request_t
  ) -> *mut xcb_record_client_spec_t;

  pub fn xcb_record_unregister_clients_client_specs_length(
    R: *const xcb_record_unregister_clients_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_unregister_clients_client_specs_end(
    R: *const xcb_record_unregister_clients_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_record_get_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_get_context(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_record_get_context_cookie_t;

  pub fn xcb_record_get_context_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_record_get_context_cookie_t;

  pub fn xcb_record_get_context_intercepted_clients_length(
    R: *const xcb_record_get_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_get_context_intercepted_clients_iterator(
    R: *const xcb_record_get_context_reply_t
  ) -> xcb_record_client_info_iterator_t;

  pub fn xcb_record_get_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_record_get_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_record_get_context_reply_t;

  pub fn xcb_record_enable_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_enable_context(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_record_enable_context_cookie_t;

  pub fn xcb_record_enable_context_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_record_enable_context_cookie_t;

  pub fn xcb_record_enable_context_data(R: *const xcb_record_enable_context_reply_t) -> *mut u8;

  pub fn xcb_record_enable_context_data_length(
    R: *const xcb_record_enable_context_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_record_enable_context_data_end(
    R: *const xcb_record_enable_context_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_record_enable_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_record_enable_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_record_enable_context_reply_t;

  pub fn xcb_record_disable_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_disable_context(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_free_context_checked(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_record_free_context(
    c: *mut xcb_connection_t,
    context: xcb_record_context_t,
  ) -> xcb_void_cookie_t;
}
