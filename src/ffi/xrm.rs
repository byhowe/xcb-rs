use super::core::xcb_connection_t;
use super::xcb::xcb_screen_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_xrm_database_t
{
  _unused: [u8; 0],
}

#[link(name = "xcb-xrm")]
extern "C" {
  pub fn xcb_xrm_database_from_default(conn: *mut xcb_connection_t) -> *mut xcb_xrm_database_t;

  pub fn xcb_xrm_database_from_resource_manager(
    conn: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
  ) -> *mut xcb_xrm_database_t;

  pub fn xcb_xrm_database_from_string(
    str_: *const ::std::os::raw::c_char
  ) -> *mut xcb_xrm_database_t;

  pub fn xcb_xrm_database_from_file(
    filename: *const ::std::os::raw::c_char
  ) -> *mut xcb_xrm_database_t;

  pub fn xcb_xrm_database_to_string(
    database: *mut xcb_xrm_database_t
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_xrm_database_combine(
    source_db: *mut xcb_xrm_database_t,
    target_db: *mut *mut xcb_xrm_database_t,
    override_: bool,
  );

  pub fn xcb_xrm_database_put_resource(
    database: *mut *mut xcb_xrm_database_t,
    resource: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
  );

  pub fn xcb_xrm_database_put_resource_line(
    database: *mut *mut xcb_xrm_database_t,
    line: *const ::std::os::raw::c_char,
  );

  pub fn xcb_xrm_database_free(database: *mut xcb_xrm_database_t);

  pub fn xcb_xrm_resource_get_string(
    database: *mut xcb_xrm_database_t,
    res_name: *const ::std::os::raw::c_char,
    res_class: *const ::std::os::raw::c_char,
    out: *mut *mut ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xrm_resource_get_long(
    database: *mut xcb_xrm_database_t,
    res_name: *const ::std::os::raw::c_char,
    res_class: *const ::std::os::raw::c_char,
    out: *mut ::std::os::raw::c_long,
  ) -> ::std::os::raw::c_int;

  pub fn xcb_xrm_resource_get_bool(
    database: *mut xcb_xrm_database_t,
    res_name: *const ::std::os::raw::c_char,
    res_class: *const ::std::os::raw::c_char,
    out: *mut bool,
  ) -> ::std::os::raw::c_int;
}
