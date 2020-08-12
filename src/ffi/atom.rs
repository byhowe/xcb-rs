#[link(name = "xcb")]
extern "C" {
  pub fn xcb_atom_name_by_screen(
    base: *const ::std::os::raw::c_char,
    screen: u8,
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_atom_name_by_resource(
    base: *const ::std::os::raw::c_char,
    resource: u32,
  ) -> *mut ::std::os::raw::c_char;

  pub fn xcb_atom_name_unique(
    base: *const ::std::os::raw::c_char,
    id: u32,
  ) -> *mut ::std::os::raw::c_char;
}
