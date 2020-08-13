use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_window_t};

pub const XCB_XPRINT_MAJOR_VERSION: u32 = 1;
pub const XCB_XPRINT_MINOR_VERSION: u32 = 0;
pub const XCB_X_PRINT_PRINT_QUERY_VERSION: u32 = 0;
pub const XCB_X_PRINT_PRINT_GET_PRINTER_LIST: u32 = 1;
pub const XCB_X_PRINT_PRINT_REHASH_PRINTER_LIST: u32 = 20;
pub const XCB_X_PRINT_CREATE_CONTEXT: u32 = 2;
pub const XCB_X_PRINT_PRINT_SET_CONTEXT: u32 = 3;
pub const XCB_X_PRINT_PRINT_GET_CONTEXT: u32 = 4;
pub const XCB_X_PRINT_PRINT_DESTROY_CONTEXT: u32 = 5;
pub const XCB_X_PRINT_PRINT_GET_SCREEN_OF_CONTEXT: u32 = 6;
pub const XCB_X_PRINT_PRINT_START_JOB: u32 = 7;
pub const XCB_X_PRINT_PRINT_END_JOB: u32 = 8;
pub const XCB_X_PRINT_PRINT_START_DOC: u32 = 9;
pub const XCB_X_PRINT_PRINT_END_DOC: u32 = 10;
pub const XCB_X_PRINT_PRINT_PUT_DOCUMENT_DATA: u32 = 11;
pub const XCB_X_PRINT_PRINT_GET_DOCUMENT_DATA: u32 = 12;
pub const XCB_X_PRINT_PRINT_START_PAGE: u32 = 13;
pub const XCB_X_PRINT_PRINT_END_PAGE: u32 = 14;
pub const XCB_X_PRINT_PRINT_SELECT_INPUT: u32 = 15;
pub const XCB_X_PRINT_PRINT_INPUT_SELECTED: u32 = 16;
pub const XCB_X_PRINT_PRINT_GET_ATTRIBUTES: u32 = 17;
pub const XCB_X_PRINT_PRINT_GET_ONE_ATTRIBUTES: u32 = 19;
pub const XCB_X_PRINT_PRINT_SET_ATTRIBUTES: u32 = 18;
pub const XCB_X_PRINT_PRINT_GET_PAGE_DIMENSIONS: u32 = 21;
pub const XCB_X_PRINT_PRINT_QUERY_SCREENS: u32 = 22;
pub const XCB_X_PRINT_PRINT_SET_IMAGE_RESOLUTION: u32 = 23;
pub const XCB_X_PRINT_PRINT_GET_IMAGE_RESOLUTION: u32 = 24;
pub const XCB_X_PRINT_NOTIFY: u32 = 0;
pub const XCB_X_PRINT_ATTRIBUT_NOTIFY: u32 = 1;
pub const XCB_X_PRINT_BAD_CONTEXT: u32 = 0;
pub const XCB_X_PRINT_BAD_SEQUENCE: u32 = 1;

pub type xcb_x_print_string8_t = ::std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_string8_iterator_t
{
  pub data: *mut xcb_x_print_string8_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_printer_t
{
  pub nameLen: u32,
  pub descLen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_printer_iterator_t
{
  pub data: *mut xcb_x_print_printer_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_x_print_pcontext_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_pcontext_iterator_t
{
  pub data: *mut xcb_x_print_pcontext_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_x_print_get_doc_t
{
  FINISHED = 0,
  SECOND_CONSUMER = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_x_print_ev_mask_t
{
  NO_EVENT_MASK = 0,
  PRINT_MASK = 1,
  ATTRIBUTE_MASK = 2,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_x_print_detail_t
{
  START_JOB_NOTIFY = 1,
  END_JOB_NOTIFY = 2,
  START_DOC_NOTIFY = 3,
  END_DOC_NOTIFY = 4,
  START_PAGE_NOTIFY = 5,
  END_PAGE_NOTIFY = 6,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_x_print_attr_t
{
  JOB_ATTR = 1,
  DOC_ATTR = 2,
  PAGE_ATTR = 3,
  PRINTER_ATTR = 4,
  SERVER_ATTR = 5,
  MEDIUM_ATTR = 6,
  SPOOLER_ATTR = 7,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_version_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_version_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_version_reply_t
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
pub struct xcb_x_print_print_get_printer_list_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_printer_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub printerNameLen: u32,
  pub localeLen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_printer_list_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub listCount: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_rehash_printer_list_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_create_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context_id: u32,
  pub printerNameLen: u32,
  pub localeLen: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_set_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_destroy_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_screen_of_context_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_screen_of_context_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_screen_of_context_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub root: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_start_job_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub output_mode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_end_job_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cancel: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_start_doc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub driver_mode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_end_doc_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cancel: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_put_document_data_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub len_data: u32,
  pub len_fmt: u16,
  pub len_options: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_document_data_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_document_data_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub max_bytes: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_document_data_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub status_code: u32,
  pub finished_flag: u32,
  pub dataLen: u32,
  pub pad1: [u8; 12usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_start_page_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub window: xcb_window_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_end_page_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub cancel: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_select_input_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub event_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_input_selected_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_input_selected_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_input_selected_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub event_mask: u32,
  pub all_events_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub pool: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_attributes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub stringLen: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_one_attributes_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_one_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub nameLen: u32,
  pub pool: u8,
  pub pad0: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_one_attributes_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub valueLen: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_set_attributes_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub stringLen: u32,
  pub pool: u8,
  pub rule: u8,
  pub pad0: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_page_dimensions_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_page_dimensions_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_page_dimensions_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub width: u16,
  pub height: u16,
  pub offset_x: u16,
  pub offset_y: u16,
  pub reproducible_width: u16,
  pub reproducible_height: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_screens_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_screens_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_query_screens_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub listCount: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_set_image_resolution_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_set_image_resolution_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
  pub image_resolution: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_set_image_resolution_reply_t
{
  pub response_type: u8,
  pub status: u8,
  pub sequence: u16,
  pub length: u32,
  pub previous_resolutions: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_image_resolution_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_image_resolution_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub context: xcb_x_print_pcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_print_get_image_resolution_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub image_resolution: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_notify_event_t
{
  pub response_type: u8,
  pub detail: u8,
  pub sequence: u16,
  pub context: xcb_x_print_pcontext_t,
  pub cancel: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_attribut_notify_event_t
{
  pub response_type: u8,
  pub detail: u8,
  pub sequence: u16,
  pub context: xcb_x_print_pcontext_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_bad_context_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_x_print_bad_sequence_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
}

#[link(name = "xcb")]
extern "C" {
  pub static mut xcb_x_print_id: xcb_extension_t;

  pub fn xcb_x_print_string8_next(i: *mut xcb_x_print_string8_iterator_t);

  pub fn xcb_x_print_string8_end(i: xcb_x_print_string8_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_printer_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    _aux: *const xcb_x_print_printer_t,
    name: *const xcb_x_print_string8_t,
    description: *const xcb_x_print_string8_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_printer_unserialize(
    _buffer: *const ::std::os::raw::c_void,
    _aux: *mut *mut xcb_x_print_printer_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_printer_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_printer_name(R: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_printer_name_length(R: *const xcb_x_print_printer_t) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_printer_name_end(R: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_printer_description(
    R: *const xcb_x_print_printer_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_printer_description_length(
    R: *const xcb_x_print_printer_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_printer_description_end(
    R: *const xcb_x_print_printer_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_printer_next(i: *mut xcb_x_print_printer_iterator_t);

  pub fn xcb_x_print_printer_end(i: xcb_x_print_printer_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_pcontext_next(i: *mut xcb_x_print_pcontext_iterator_t);

  pub fn xcb_x_print_pcontext_end(i: xcb_x_print_pcontext_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_query_version(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_query_version_cookie_t;

  pub fn xcb_x_print_print_query_version_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_query_version_cookie_t;

  pub fn xcb_x_print_print_query_version_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_query_version_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_query_version_reply_t;

  pub fn xcb_x_print_print_get_printer_list_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_printer_list(
    c: *mut xcb_connection_t,
    printerNameLen: u32,
    localeLen: u32,
    printer_name: *const xcb_x_print_string8_t,
    locale: *const xcb_x_print_string8_t,
  ) -> xcb_x_print_print_get_printer_list_cookie_t;

  pub fn xcb_x_print_print_get_printer_list_unchecked(
    c: *mut xcb_connection_t,
    printerNameLen: u32,
    localeLen: u32,
    printer_name: *const xcb_x_print_string8_t,
    locale: *const xcb_x_print_string8_t,
  ) -> xcb_x_print_print_get_printer_list_cookie_t;

  pub fn xcb_x_print_print_get_printer_list_printers_length(
    R: *const xcb_x_print_print_get_printer_list_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_printer_list_printers_iterator(
    R: *const xcb_x_print_print_get_printer_list_reply_t
  ) -> xcb_x_print_printer_iterator_t;

  pub fn xcb_x_print_print_get_printer_list_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_printer_list_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_printer_list_reply_t;

  pub fn xcb_x_print_print_rehash_printer_list_checked(
    c: *mut xcb_connection_t
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_rehash_printer_list(c: *mut xcb_connection_t) -> xcb_void_cookie_t;

  pub fn xcb_x_print_create_context_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_create_context_checked(
    c: *mut xcb_connection_t,
    context_id: u32,
    printerNameLen: u32,
    localeLen: u32,
    printerName: *const xcb_x_print_string8_t,
    locale: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_create_context(
    c: *mut xcb_connection_t,
    context_id: u32,
    printerNameLen: u32,
    localeLen: u32,
    printerName: *const xcb_x_print_string8_t,
    locale: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_create_context_printer_name(
    R: *const xcb_x_print_create_context_request_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_create_context_printer_name_length(
    R: *const xcb_x_print_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_create_context_printer_name_end(
    R: *const xcb_x_print_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_create_context_locale(
    R: *const xcb_x_print_create_context_request_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_create_context_locale_length(
    R: *const xcb_x_print_create_context_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_create_context_locale_end(
    R: *const xcb_x_print_create_context_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_set_context_checked(
    c: *mut xcb_connection_t,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_set_context(
    c: *mut xcb_connection_t,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_get_context(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_get_context_cookie_t;

  pub fn xcb_x_print_print_get_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_get_context_cookie_t;

  pub fn xcb_x_print_print_get_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_context_reply_t;

  pub fn xcb_x_print_print_destroy_context_checked(
    c: *mut xcb_connection_t,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_destroy_context(
    c: *mut xcb_connection_t,
    context: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_get_screen_of_context(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_get_screen_of_context_cookie_t;

  pub fn xcb_x_print_print_get_screen_of_context_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_get_screen_of_context_cookie_t;

  pub fn xcb_x_print_print_get_screen_of_context_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_screen_of_context_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_screen_of_context_reply_t;

  pub fn xcb_x_print_print_start_job_checked(
    c: *mut xcb_connection_t,
    output_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_start_job(
    c: *mut xcb_connection_t,
    output_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_job_checked(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_job(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_start_doc_checked(
    c: *mut xcb_connection_t,
    driver_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_start_doc(
    c: *mut xcb_connection_t,
    driver_mode: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_doc_checked(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_doc(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_put_document_data_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_put_document_data_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    len_data: u32,
    len_fmt: u16,
    len_options: u16,
    data: *const u8,
    doc_format: *const xcb_x_print_string8_t,
    options: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_put_document_data(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    len_data: u32,
    len_fmt: u16,
    len_options: u16,
    data: *const u8,
    doc_format: *const xcb_x_print_string8_t,
    options: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_put_document_data_data(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> *mut u8;

  pub fn xcb_x_print_print_put_document_data_data_length(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_put_document_data_data_end(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_put_document_data_doc_format(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_print_put_document_data_doc_format_length(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_put_document_data_doc_format_end(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_put_document_data_options(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_print_put_document_data_options_length(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_put_document_data_options_end(
    R: *const xcb_x_print_print_put_document_data_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_get_document_data_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_document_data(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    max_bytes: u32,
  ) -> xcb_x_print_print_get_document_data_cookie_t;

  pub fn xcb_x_print_print_get_document_data_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    max_bytes: u32,
  ) -> xcb_x_print_print_get_document_data_cookie_t;

  pub fn xcb_x_print_print_get_document_data_data(
    R: *const xcb_x_print_print_get_document_data_reply_t
  ) -> *mut u8;

  pub fn xcb_x_print_print_get_document_data_data_length(
    R: *const xcb_x_print_print_get_document_data_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_document_data_data_end(
    R: *const xcb_x_print_print_get_document_data_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_get_document_data_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_document_data_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_document_data_reply_t;

  pub fn xcb_x_print_print_start_page_checked(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_start_page(
    c: *mut xcb_connection_t,
    window: xcb_window_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_page_checked(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_end_page(
    c: *mut xcb_connection_t,
    cancel: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_select_input_checked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_select_input(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    event_mask: u32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_input_selected(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_input_selected_cookie_t;

  pub fn xcb_x_print_print_input_selected_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_input_selected_cookie_t;

  pub fn xcb_x_print_print_input_selected_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_input_selected_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_input_selected_reply_t;

  pub fn xcb_x_print_print_get_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_attributes(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    pool: u8,
  ) -> xcb_x_print_print_get_attributes_cookie_t;

  pub fn xcb_x_print_print_get_attributes_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    pool: u8,
  ) -> xcb_x_print_print_get_attributes_cookie_t;

  pub fn xcb_x_print_print_get_attributes_attributes(
    R: *const xcb_x_print_print_get_attributes_reply_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_print_get_attributes_attributes_length(
    R: *const xcb_x_print_print_get_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_attributes_attributes_end(
    R: *const xcb_x_print_print_get_attributes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_get_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_attributes_reply_t;

  pub fn xcb_x_print_print_get_one_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_one_attributes(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    nameLen: u32,
    pool: u8,
    name: *const xcb_x_print_string8_t,
  ) -> xcb_x_print_print_get_one_attributes_cookie_t;

  pub fn xcb_x_print_print_get_one_attributes_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    nameLen: u32,
    pool: u8,
    name: *const xcb_x_print_string8_t,
  ) -> xcb_x_print_print_get_one_attributes_cookie_t;

  pub fn xcb_x_print_print_get_one_attributes_value(
    R: *const xcb_x_print_print_get_one_attributes_reply_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_print_get_one_attributes_value_length(
    R: *const xcb_x_print_print_get_one_attributes_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_get_one_attributes_value_end(
    R: *const xcb_x_print_print_get_one_attributes_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_get_one_attributes_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_one_attributes_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_one_attributes_reply_t;

  pub fn xcb_x_print_print_set_attributes_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    attributes_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_set_attributes_checked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    stringLen: u32,
    pool: u8,
    rule: u8,
    attributes_len: u32,
    attributes: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_set_attributes(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    stringLen: u32,
    pool: u8,
    rule: u8,
    attributes_len: u32,
    attributes: *const xcb_x_print_string8_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_x_print_print_set_attributes_attributes(
    R: *const xcb_x_print_print_set_attributes_request_t
  ) -> *mut xcb_x_print_string8_t;

  pub fn xcb_x_print_print_set_attributes_attributes_length(
    R: *const xcb_x_print_print_set_attributes_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_set_attributes_attributes_end(
    R: *const xcb_x_print_print_set_attributes_request_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_get_page_dimensions(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_get_page_dimensions_cookie_t;

  pub fn xcb_x_print_print_get_page_dimensions_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_get_page_dimensions_cookie_t;

  pub fn xcb_x_print_print_get_page_dimensions_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_page_dimensions_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_page_dimensions_reply_t;

  pub fn xcb_x_print_print_query_screens_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_query_screens(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_query_screens_cookie_t;

  pub fn xcb_x_print_print_query_screens_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_x_print_print_query_screens_cookie_t;

  pub fn xcb_x_print_print_query_screens_roots(
    R: *const xcb_x_print_print_query_screens_reply_t
  ) -> *mut xcb_window_t;

  pub fn xcb_x_print_print_query_screens_roots_length(
    R: *const xcb_x_print_print_query_screens_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_x_print_print_query_screens_roots_end(
    R: *const xcb_x_print_print_query_screens_reply_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_x_print_print_query_screens_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_query_screens_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_query_screens_reply_t;

  pub fn xcb_x_print_print_set_image_resolution(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    image_resolution: u16,
  ) -> xcb_x_print_print_set_image_resolution_cookie_t;

  pub fn xcb_x_print_print_set_image_resolution_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
    image_resolution: u16,
  ) -> xcb_x_print_print_set_image_resolution_cookie_t;

  pub fn xcb_x_print_print_set_image_resolution_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_set_image_resolution_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_set_image_resolution_reply_t;

  pub fn xcb_x_print_print_get_image_resolution(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_get_image_resolution_cookie_t;

  pub fn xcb_x_print_print_get_image_resolution_unchecked(
    c: *mut xcb_connection_t,
    context: xcb_x_print_pcontext_t,
  ) -> xcb_x_print_print_get_image_resolution_cookie_t;

  pub fn xcb_x_print_print_get_image_resolution_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_x_print_print_get_image_resolution_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_x_print_print_get_image_resolution_reply_t;
}
