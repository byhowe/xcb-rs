use super::core::{xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_iterator_t};
use super::xcb::xcb_atom_t;

pub const XCB_RES_MAJOR_VERSION: u32 = 1;
pub const XCB_RES_MINOR_VERSION: u32 = 2;
pub const XCB_RES_QUERY_VERSION: u32 = 0;
pub const XCB_RES_QUERY_CLIENTS: u32 = 1;
pub const XCB_RES_QUERY_CLIENT_RESOURCES: u32 = 2;
pub const XCB_RES_QUERY_CLIENT_PIXMAP_BYTES: u32 = 3;
pub const XCB_RES_QUERY_CLIENT_IDS: u32 = 4;
pub const XCB_RES_QUERY_RESOURCE_BYTES: u32 = 5;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_t
{
  pub resource_base: u32,
  pub resource_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_iterator_t
{
  pub data: *mut xcb_res_client_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_type_t
{
  pub resource_type: xcb_atom_t,
  pub count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_type_iterator_t
{
  pub data: *mut xcb_res_type_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_res_client_id_mask_t
{
  CLIENT_XID = 1,
  LOCAL_CLIENT_PID = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_id_spec_t
{
  pub client: u32,
  pub mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_id_spec_iterator_t
{
  pub data: *mut xcb_res_client_id_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_id_value_t
{
  pub spec: xcb_res_client_id_spec_t,
  pub length: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_client_id_value_iterator_t
{
  pub data: *mut xcb_res_client_id_value_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_id_spec_t
{
  pub resource: u32,
  pub type_: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_id_spec_iterator_t
{
  pub data: *mut xcb_res_resource_id_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_size_spec_t
{
  pub spec: xcb_res_resource_id_spec_t,
  pub bytes: u32,
  pub ref_count: u32,
  pub use_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_size_spec_iterator_t
{
  pub data: *mut xcb_res_resource_size_spec_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_size_value_t
{
  pub size: xcb_res_resource_size_spec_t,
  pub num_cross_references: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_resource_size_value_iterator_t
{
  pub data: *mut xcb_res_resource_size_value_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client_major: u8,
  pub client_minor: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_version_reply_t
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
pub struct xcb_res_query_clients_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_clients_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_clients_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_clients: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_resources_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_resources_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub xid: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_resources_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_types: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_pixmap_bytes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_pixmap_bytes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub xid: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_pixmap_bytes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub bytes: u32,
  pub bytes_overflow: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_ids_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_ids_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub num_specs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_client_ids_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_ids: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_resource_bytes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_resource_bytes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub client: u32,
  pub num_specs: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_res_query_resource_bytes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub num_sizes: u32,
  pub pad1: [u8; 20usize],
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_res_id: xcb_extension_t;

  pub fn xcb_res_client_next(i: *mut xcb_res_client_iterator_t);

  pub fn xcb_res_client_end(i: xcb_res_client_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_res_type_next(i: *mut xcb_res_type_iterator_t);

  pub fn xcb_res_type_end(i: xcb_res_type_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_res_client_id_spec_next(i: *mut xcb_res_client_id_spec_iterator_t);

  pub fn xcb_res_client_id_spec_end(i: xcb_res_client_id_spec_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_res_client_id_value_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_client_id_value_value(R: *const xcb_res_client_id_value_t) -> *mut u32;

  pub fn xcb_res_client_id_value_value_length(
    R: *const xcb_res_client_id_value_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_client_id_value_value_end(
    R: *const xcb_res_client_id_value_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_res_client_id_value_next(i: *mut xcb_res_client_id_value_iterator_t);

  pub fn xcb_res_client_id_value_end(
    i: xcb_res_client_id_value_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_res_resource_id_spec_next(i: *mut xcb_res_resource_id_spec_iterator_t);

  pub fn xcb_res_resource_id_spec_end(
    i: xcb_res_resource_id_spec_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_res_resource_size_spec_next(i: *mut xcb_res_resource_size_spec_iterator_t);

  pub fn xcb_res_resource_size_spec_end(
    i: xcb_res_resource_size_spec_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_res_resource_size_value_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_resource_size_value_cross_references(
    R: *const xcb_res_resource_size_value_t
  ) -> *mut xcb_res_resource_size_spec_t;

  pub fn xcb_res_resource_size_value_cross_references_length(
    R: *const xcb_res_resource_size_value_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_resource_size_value_cross_references_iterator(
    R: *const xcb_res_resource_size_value_t
  ) -> xcb_res_resource_size_spec_iterator_t;

  pub fn xcb_res_resource_size_value_next(i: *mut xcb_res_resource_size_value_iterator_t);

  pub fn xcb_res_resource_size_value_end(
    i: xcb_res_resource_size_value_iterator_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_res_query_version(
    c: *mut xcb_connection_t,
    client_major: u8,
    client_minor: u8,
  ) -> xcb_res_query_version_cookie_t;

  pub fn xcb_res_query_version_unchecked(
    c: *mut xcb_connection_t,
    client_major: u8,
    client_minor: u8,
  ) -> xcb_res_query_version_cookie_t;

  pub fn xcb_res_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_version_reply_t;

  pub fn xcb_res_query_clients_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_clients(c: *mut xcb_connection_t) -> xcb_res_query_clients_cookie_t;

  pub fn xcb_res_query_clients_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_res_query_clients_cookie_t;

  pub fn xcb_res_query_clients_clients(
    R: *const xcb_res_query_clients_reply_t
  ) -> *mut xcb_res_client_t;

  pub fn xcb_res_query_clients_clients_length(
    R: *const xcb_res_query_clients_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_clients_clients_iterator(
    R: *const xcb_res_query_clients_reply_t
  ) -> xcb_res_client_iterator_t;

  pub fn xcb_res_query_clients_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_clients_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_clients_reply_t;

  pub fn xcb_res_query_client_resources_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_client_resources(
    c: *mut xcb_connection_t,
    xid: u32,
  ) -> xcb_res_query_client_resources_cookie_t;

  pub fn xcb_res_query_client_resources_unchecked(
    c: *mut xcb_connection_t,
    xid: u32,
  ) -> xcb_res_query_client_resources_cookie_t;

  pub fn xcb_res_query_client_resources_types(
    R: *const xcb_res_query_client_resources_reply_t
  ) -> *mut xcb_res_type_t;

  pub fn xcb_res_query_client_resources_types_length(
    R: *const xcb_res_query_client_resources_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_client_resources_types_iterator(
    R: *const xcb_res_query_client_resources_reply_t
  ) -> xcb_res_type_iterator_t;

  pub fn xcb_res_query_client_resources_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_client_resources_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_client_resources_reply_t;

  pub fn xcb_res_query_client_pixmap_bytes(
    c: *mut xcb_connection_t,
    xid: u32,
  ) -> xcb_res_query_client_pixmap_bytes_cookie_t;

  pub fn xcb_res_query_client_pixmap_bytes_unchecked(
    c: *mut xcb_connection_t,
    xid: u32,
  ) -> xcb_res_query_client_pixmap_bytes_cookie_t;

  pub fn xcb_res_query_client_pixmap_bytes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_client_pixmap_bytes_reply_t;

  pub fn xcb_res_query_client_ids_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_client_ids(
    c: *mut xcb_connection_t,
    num_specs: u32,
    specs: *const xcb_res_client_id_spec_t,
  ) -> xcb_res_query_client_ids_cookie_t;

  pub fn xcb_res_query_client_ids_unchecked(
    c: *mut xcb_connection_t,
    num_specs: u32,
    specs: *const xcb_res_client_id_spec_t,
  ) -> xcb_res_query_client_ids_cookie_t;

  pub fn xcb_res_query_client_ids_ids_length(
    R: *const xcb_res_query_client_ids_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_client_ids_ids_iterator(
    R: *const xcb_res_query_client_ids_reply_t
  ) -> xcb_res_client_id_value_iterator_t;

  pub fn xcb_res_query_client_ids_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_client_ids_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_client_ids_reply_t;

  pub fn xcb_res_query_resource_bytes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_resource_bytes(
    c: *mut xcb_connection_t,
    client: u32,
    num_specs: u32,
    specs: *const xcb_res_resource_id_spec_t,
  ) -> xcb_res_query_resource_bytes_cookie_t;

  pub fn xcb_res_query_resource_bytes_unchecked(
    c: *mut xcb_connection_t,
    client: u32,
    num_specs: u32,
    specs: *const xcb_res_resource_id_spec_t,
  ) -> xcb_res_query_resource_bytes_cookie_t;

  pub fn xcb_res_query_resource_bytes_sizes_length(
    R: *const xcb_res_query_resource_bytes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_res_query_resource_bytes_sizes_iterator(
    R: *const xcb_res_query_resource_bytes_reply_t
  ) -> xcb_res_resource_size_value_iterator_t;

  pub fn xcb_res_query_resource_bytes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_res_query_resource_bytes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_res_query_resource_bytes_reply_t;
}
