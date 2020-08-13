use crate::ffi::core::{
  xcb_connection_has_error,
  xcb_connection_t,
  XCB_CONN_CLOSED_EXT_NOTSUPPORTED,
  XCB_CONN_CLOSED_INVALID_SCREEN,
  XCB_CONN_CLOSED_MEM_INSUFFICIENT,
  XCB_CONN_CLOSED_PARSE_ERR,
  XCB_CONN_CLOSED_REQ_LEN_EXCEED,
  XCB_CONN_ERROR,
};

pub enum ConnectionError
{
  Error,
  ClosedExtNotsupported,
  ClosedMemInsufficient,
  ClosedReqLenExceed,
  ClosedParseErr,
  ClosedInvalidScreen,
}

impl ConnectionError
{
  #[inline(always)]
  pub fn has_error(c: *mut xcb_connection_t) -> Option<Self>
  {
    match unsafe { xcb_connection_has_error(c) } {
      0 => None,
      XCB_CONN_ERROR => Some(Self::Error),
      XCB_CONN_CLOSED_EXT_NOTSUPPORTED => Some(Self::ClosedExtNotsupported),
      XCB_CONN_CLOSED_MEM_INSUFFICIENT => Some(Self::ClosedMemInsufficient),
      XCB_CONN_CLOSED_REQ_LEN_EXCEED => Some(Self::ClosedReqLenExceed),
      XCB_CONN_CLOSED_PARSE_ERR => Some(Self::ClosedParseErr),
      XCB_CONN_CLOSED_INVALID_SCREEN => Some(Self::ClosedInvalidScreen),
      _ => Some(Self::Error),
    }
  }
}

impl std::fmt::Display for ConnectionError
{
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Error => "socket, pipe or other stream errors.",
        Self::ClosedExtNotsupported => "extension not supported.",
        Self::ClosedMemInsufficient => "memory not available.",
        Self::ClosedReqLenExceed => "exceeding request length that server accepts.",
        Self::ClosedParseErr => "error during parsing display string.",
        Self::ClosedInvalidScreen => "the server does not have a screen matching the display.",
      }
    )
  }
}
