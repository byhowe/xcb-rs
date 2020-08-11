use crate::ffi::core;

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
  pub fn has_error(c: *mut core::xcb_connection_t) -> Option<Self>
  {
    match unsafe { core::xcb_connection_has_error(c) } {
      0 => None,
      core::XCB_CONN_ERROR => Some(Self::Error),
      core::XCB_CONN_CLOSED_EXT_NOTSUPPORTED => Some(Self::ClosedExtNotsupported),
      core::XCB_CONN_CLOSED_MEM_INSUFFICIENT => Some(Self::ClosedMemInsufficient),
      core::XCB_CONN_CLOSED_REQ_LEN_EXCEED => Some(Self::ClosedReqLenExceed),
      core::XCB_CONN_CLOSED_PARSE_ERR => Some(Self::ClosedParseErr),
      core::XCB_CONN_CLOSED_INVALID_SCREEN => Some(Self::ClosedInvalidScreen),
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
