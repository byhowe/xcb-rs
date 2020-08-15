use super::core::{
  xcb_connection_t,
  xcb_extension_t,
  xcb_generic_error_t,
  xcb_generic_iterator_t,
  xcb_void_cookie_t,
};
use super::xcb::{xcb_drawable_t, xcb_timestamp_t};

pub const XCB_SYNC_MAJOR_VERSION: u32 = 3;
pub const XCB_SYNC_MINOR_VERSION: u32 = 1;
pub const XCB_SYNC_COUNTER: u32 = 0;
pub const XCB_SYNC_ALARM: u32 = 1;
pub const XCB_SYNC_INITIALIZE: u32 = 0;
pub const XCB_SYNC_LIST_SYSTEM_COUNTERS: u32 = 1;
pub const XCB_SYNC_CREATE_COUNTER: u32 = 2;
pub const XCB_SYNC_DESTROY_COUNTER: u32 = 6;
pub const XCB_SYNC_QUERY_COUNTER: u32 = 5;
pub const XCB_SYNC_AWAIT: u32 = 7;
pub const XCB_SYNC_CHANGE_COUNTER: u32 = 4;
pub const XCB_SYNC_SET_COUNTER: u32 = 3;
pub const XCB_SYNC_CREATE_ALARM: u32 = 8;
pub const XCB_SYNC_CHANGE_ALARM: u32 = 9;
pub const XCB_SYNC_DESTROY_ALARM: u32 = 11;
pub const XCB_SYNC_QUERY_ALARM: u32 = 10;
pub const XCB_SYNC_SET_PRIORITY: u32 = 12;
pub const XCB_SYNC_GET_PRIORITY: u32 = 13;
pub const XCB_SYNC_CREATE_FENCE: u32 = 14;
pub const XCB_SYNC_TRIGGER_FENCE: u32 = 15;
pub const XCB_SYNC_RESET_FENCE: u32 = 16;
pub const XCB_SYNC_DESTROY_FENCE: u32 = 17;
pub const XCB_SYNC_QUERY_FENCE: u32 = 18;
pub const XCB_SYNC_AWAIT_FENCE: u32 = 19;
pub const XCB_SYNC_COUNTER_NOTIFY: u32 = 0;
pub const XCB_SYNC_ALARM_NOTIFY: u32 = 1;

pub type xcb_sync_alarm_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_alarm_iterator_t
{
  pub data: *mut xcb_sync_alarm_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_sync_alarmstate_t
{
  ACTIVE = 0,
  INACTIVE = 1,
  DESTROYED = 2,
}

pub type xcb_sync_counter_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_counter_iterator_t
{
  pub data: *mut xcb_sync_counter_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

pub type xcb_sync_fence_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_fence_iterator_t
{
  pub data: *mut xcb_sync_fence_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_sync_testtype_t
{
  POSITIVE_TRANSITION = 0,
  NEGATIVE_TRANSITION = 1,
  POSITIVE_COMPARISON = 2,
  NEGATIVE_COMPARISON = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_sync_valuetype_t
{
  ABSOLUTE = 0,
  RELATIVE = 1,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xcb_sync_ca_t
{
  COUNTER = 1,
  VALUE_TYPE = 2,
  VALUE = 4,
  TEST_TYPE = 8,
  DELTA = 16,
  EVENTS = 32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_int64_t
{
  pub hi: i32,
  pub lo: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_int64_iterator_t
{
  pub data: *mut xcb_sync_int64_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_systemcounter_t
{
  pub counter: xcb_sync_counter_t,
  pub resolution: xcb_sync_int64_t,
  pub name_len: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_systemcounter_iterator_t
{
  pub data: *mut xcb_sync_systemcounter_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_trigger_t
{
  pub counter: xcb_sync_counter_t,
  pub wait_type: u32,
  pub wait_value: xcb_sync_int64_t,
  pub test_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_trigger_iterator_t
{
  pub data: *mut xcb_sync_trigger_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_waitcondition_t
{
  pub trigger: xcb_sync_trigger_t,
  pub event_threshold: xcb_sync_int64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_waitcondition_iterator_t
{
  pub data: *mut xcb_sync_waitcondition_t,
  pub rem: ::std::os::raw::c_int,
  pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_counter_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub bad_counter: u32,
  pub minor_opcode: u16,
  pub major_opcode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_alarm_error_t
{
  pub response_type: u8,
  pub error_code: u8,
  pub sequence: u16,
  pub bad_alarm: u32,
  pub minor_opcode: u16,
  pub major_opcode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_initialize_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_initialize_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub desired_major_version: u8,
  pub desired_minor_version: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_initialize_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub major_version: u8,
  pub minor_version: u8,
  pub pad1: [u8; 22usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_list_system_counters_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_list_system_counters_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_list_system_counters_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub counters_len: u32,
  pub pad1: [u8; 20usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_create_counter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub id: xcb_sync_counter_t,
  pub initial_value: xcb_sync_int64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_destroy_counter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub counter: xcb_sync_counter_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_counter_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_counter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub counter: xcb_sync_counter_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_counter_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub counter_value: xcb_sync_int64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_await_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_change_counter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub counter: xcb_sync_counter_t,
  pub amount: xcb_sync_int64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_set_counter_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub counter: xcb_sync_counter_t,
  pub value: xcb_sync_int64_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_create_alarm_value_list_t
{
  pub counter: xcb_sync_counter_t,
  pub valueType: u32,
  pub value: xcb_sync_int64_t,
  pub testType: u32,
  pub delta: xcb_sync_int64_t,
  pub events: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_create_alarm_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub id: xcb_sync_alarm_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_change_alarm_value_list_t
{
  pub counter: xcb_sync_counter_t,
  pub valueType: u32,
  pub value: xcb_sync_int64_t,
  pub testType: u32,
  pub delta: xcb_sync_int64_t,
  pub events: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_change_alarm_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub id: xcb_sync_alarm_t,
  pub value_mask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_destroy_alarm_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub alarm: xcb_sync_alarm_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_alarm_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_alarm_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub alarm: xcb_sync_alarm_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_alarm_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub trigger: xcb_sync_trigger_t,
  pub delta: xcb_sync_int64_t,
  pub events: u8,
  pub state: u8,
  pub pad1: [u8; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_set_priority_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub id: u32,
  pub priority: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_get_priority_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_get_priority_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_get_priority_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub priority: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_create_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub drawable: xcb_drawable_t,
  pub fence: xcb_sync_fence_t,
  pub initially_triggered: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_trigger_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub fence: xcb_sync_fence_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_reset_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub fence: xcb_sync_fence_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_destroy_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub fence: xcb_sync_fence_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_fence_cookie_t
{
  pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
  pub fence: xcb_sync_fence_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_query_fence_reply_t
{
  pub response_type: u8,
  pub pad0: u8,
  pub sequence: u16,
  pub length: u32,
  pub triggered: u8,
  pub pad1: [u8; 23usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_await_fence_request_t
{
  pub major_opcode: u8,
  pub minor_opcode: u8,
  pub length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_counter_notify_event_t
{
  pub response_type: u8,
  pub kind: u8,
  pub sequence: u16,
  pub counter: xcb_sync_counter_t,
  pub wait_value: xcb_sync_int64_t,
  pub counter_value: xcb_sync_int64_t,
  pub timestamp: xcb_timestamp_t,
  pub count: u16,
  pub destroyed: u8,
  pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_sync_alarm_notify_event_t
{
  pub response_type: u8,
  pub kind: u8,
  pub sequence: u16,
  pub alarm: xcb_sync_alarm_t,
  pub counter_value: xcb_sync_int64_t,
  pub alarm_value: xcb_sync_int64_t,
  pub timestamp: xcb_timestamp_t,
  pub state: u8,
  pub pad0: [u8; 3usize],
}

#[link(name = "xcb-sync")]
extern "C" {
  pub static mut xcb_sync_id: xcb_extension_t;

  pub fn xcb_sync_alarm_next(i: *mut xcb_sync_alarm_iterator_t);

  pub fn xcb_sync_alarm_end(i: xcb_sync_alarm_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_sync_counter_next(i: *mut xcb_sync_counter_iterator_t);

  pub fn xcb_sync_counter_end(i: xcb_sync_counter_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_sync_fence_next(i: *mut xcb_sync_fence_iterator_t);

  pub fn xcb_sync_fence_end(i: xcb_sync_fence_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_sync_int64_next(i: *mut xcb_sync_int64_iterator_t);

  pub fn xcb_sync_int64_end(i: xcb_sync_int64_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_sync_systemcounter_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_systemcounter_name(
    R: *const xcb_sync_systemcounter_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_sync_systemcounter_name_length(
    R: *const xcb_sync_systemcounter_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_systemcounter_name_end(
    R: *const xcb_sync_systemcounter_t
  ) -> xcb_generic_iterator_t;

  pub fn xcb_sync_systemcounter_next(i: *mut xcb_sync_systemcounter_iterator_t);

  pub fn xcb_sync_systemcounter_end(i: xcb_sync_systemcounter_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_sync_trigger_next(i: *mut xcb_sync_trigger_iterator_t);

  pub fn xcb_sync_trigger_end(i: xcb_sync_trigger_iterator_t) -> xcb_generic_iterator_t;

  pub fn xcb_sync_waitcondition_next(i: *mut xcb_sync_waitcondition_iterator_t);

  pub fn xcb_sync_waitcondition_end(i: xcb_sync_waitcondition_iterator_t)
    -> xcb_generic_iterator_t;

  pub fn xcb_sync_initialize(
    c: *mut xcb_connection_t,
    desired_major_version: u8,
    desired_minor_version: u8,
  ) -> xcb_sync_initialize_cookie_t;

  pub fn xcb_sync_initialize_unchecked(
    c: *mut xcb_connection_t,
    desired_major_version: u8,
    desired_minor_version: u8,
  ) -> xcb_sync_initialize_cookie_t;

  pub fn xcb_sync_initialize_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_initialize_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_initialize_reply_t;

  pub fn xcb_sync_list_system_counters_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_list_system_counters(
    c: *mut xcb_connection_t
  ) -> xcb_sync_list_system_counters_cookie_t;

  pub fn xcb_sync_list_system_counters_unchecked(
    c: *mut xcb_connection_t
  ) -> xcb_sync_list_system_counters_cookie_t;

  pub fn xcb_sync_list_system_counters_counters_length(
    R: *const xcb_sync_list_system_counters_reply_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_list_system_counters_counters_iterator(
    R: *const xcb_sync_list_system_counters_reply_t
  ) -> xcb_sync_systemcounter_iterator_t;

  pub fn xcb_sync_list_system_counters_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_list_system_counters_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_list_system_counters_reply_t;

  pub fn xcb_sync_create_counter_checked(
    c: *mut xcb_connection_t,
    id: xcb_sync_counter_t,
    initial_value: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_counter(
    c: *mut xcb_connection_t,
    id: xcb_sync_counter_t,
    initial_value: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_destroy_counter_checked(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_destroy_counter(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_query_counter(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
  ) -> xcb_sync_query_counter_cookie_t;

  pub fn xcb_sync_query_counter_unchecked(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
  ) -> xcb_sync_query_counter_cookie_t;

  pub fn xcb_sync_query_counter_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_query_counter_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_query_counter_reply_t;

  pub fn xcb_sync_await_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    wait_list_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_await_checked(
    c: *mut xcb_connection_t,
    wait_list_len: u32,
    wait_list: *const xcb_sync_waitcondition_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_await(
    c: *mut xcb_connection_t,
    wait_list_len: u32,
    wait_list: *const xcb_sync_waitcondition_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_await_wait_list(
    R: *const xcb_sync_await_request_t
  ) -> *mut xcb_sync_waitcondition_t;

  pub fn xcb_sync_await_wait_list_length(
    R: *const xcb_sync_await_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_await_wait_list_iterator(
    R: *const xcb_sync_await_request_t
  ) -> xcb_sync_waitcondition_iterator_t;

  pub fn xcb_sync_change_counter_checked(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
    amount: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_change_counter(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
    amount: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_set_counter_checked(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
    value: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_set_counter(
    c: *mut xcb_connection_t,
    counter: xcb_sync_counter_t,
    value: xcb_sync_int64_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_alarm_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_sync_create_alarm_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_create_alarm_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_sync_create_alarm_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_create_alarm_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_create_alarm_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_create_alarm_checked(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_alarm(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_alarm_aux_checked(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const xcb_sync_create_alarm_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_alarm_aux(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const xcb_sync_create_alarm_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_alarm_value_list(
    R: *const xcb_sync_create_alarm_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_sync_change_alarm_value_list_serialize(
    _buffer: *mut *mut ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *const xcb_sync_change_alarm_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_change_alarm_value_list_unpack(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
    _aux: *mut xcb_sync_change_alarm_value_list_t,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_change_alarm_value_list_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    value_mask: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_change_alarm_sizeof(
    _buffer: *const ::std::os::raw::c_void
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_change_alarm_checked(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_change_alarm(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const ::std::os::raw::c_void,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_change_alarm_aux_checked(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const xcb_sync_change_alarm_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_change_alarm_aux(
    c: *mut xcb_connection_t,
    id: xcb_sync_alarm_t,
    value_mask: u32,
    value_list: *const xcb_sync_change_alarm_value_list_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_change_alarm_value_list(
    R: *const xcb_sync_change_alarm_request_t
  ) -> *mut ::std::os::raw::c_void;

  pub fn xcb_sync_destroy_alarm_checked(
    c: *mut xcb_connection_t,
    alarm: xcb_sync_alarm_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_destroy_alarm(
    c: *mut xcb_connection_t,
    alarm: xcb_sync_alarm_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_query_alarm(
    c: *mut xcb_connection_t,
    alarm: xcb_sync_alarm_t,
  ) -> xcb_sync_query_alarm_cookie_t;

  pub fn xcb_sync_query_alarm_unchecked(
    c: *mut xcb_connection_t,
    alarm: xcb_sync_alarm_t,
  ) -> xcb_sync_query_alarm_cookie_t;

  pub fn xcb_sync_query_alarm_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_query_alarm_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_query_alarm_reply_t;

  pub fn xcb_sync_set_priority_checked(
    c: *mut xcb_connection_t,
    id: u32,
    priority: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_set_priority(
    c: *mut xcb_connection_t,
    id: u32,
    priority: i32,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_get_priority(
    c: *mut xcb_connection_t,
    id: u32,
  ) -> xcb_sync_get_priority_cookie_t;

  pub fn xcb_sync_get_priority_unchecked(
    c: *mut xcb_connection_t,
    id: u32,
  ) -> xcb_sync_get_priority_cookie_t;

  pub fn xcb_sync_get_priority_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_get_priority_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_get_priority_reply_t;

  pub fn xcb_sync_create_fence_checked(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: xcb_sync_fence_t,
    initially_triggered: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_create_fence(
    c: *mut xcb_connection_t,
    drawable: xcb_drawable_t,
    fence: xcb_sync_fence_t,
    initially_triggered: u8,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_trigger_fence_checked(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_trigger_fence(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_reset_fence_checked(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_reset_fence(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_destroy_fence_checked(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_destroy_fence(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_query_fence(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_sync_query_fence_cookie_t;

  pub fn xcb_sync_query_fence_unchecked(
    c: *mut xcb_connection_t,
    fence: xcb_sync_fence_t,
  ) -> xcb_sync_query_fence_cookie_t;

  pub fn xcb_sync_query_fence_reply(
    c: *mut xcb_connection_t,
    cookie: xcb_sync_query_fence_cookie_t,
    e: *mut *mut xcb_generic_error_t,
  ) -> *mut xcb_sync_query_fence_reply_t;

  pub fn xcb_sync_await_fence_sizeof(
    _buffer: *const ::std::os::raw::c_void,
    fence_list_len: u32,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_await_fence_checked(
    c: *mut xcb_connection_t,
    fence_list_len: u32,
    fence_list: *const xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_await_fence(
    c: *mut xcb_connection_t,
    fence_list_len: u32,
    fence_list: *const xcb_sync_fence_t,
  ) -> xcb_void_cookie_t;

  pub fn xcb_sync_await_fence_fence_list(
    R: *const xcb_sync_await_fence_request_t
  ) -> *mut xcb_sync_fence_t;

  pub fn xcb_sync_await_fence_fence_list_length(
    R: *const xcb_sync_await_fence_request_t
  ) -> ::std::os::raw::c_int;

  pub fn xcb_sync_await_fence_fence_list_end(
    R: *const xcb_sync_await_fence_request_t
  ) -> xcb_generic_iterator_t;
}
