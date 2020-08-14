use crate::xcb;

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
  pub fn has_error(c: *mut xcb::connection_t) -> Option<Self>
  {
    match unsafe { xcb::connection_has_error(c) } {
      0 => None,
      xcb::CONN_ERROR => Some(Self::Error),
      xcb::CONN_CLOSED_EXT_NOTSUPPORTED => Some(Self::ClosedExtNotsupported),
      xcb::CONN_CLOSED_MEM_INSUFFICIENT => Some(Self::ClosedMemInsufficient),
      xcb::CONN_CLOSED_REQ_LEN_EXCEED => Some(Self::ClosedReqLenExceed),
      xcb::CONN_CLOSED_PARSE_ERR => Some(Self::ClosedParseErr),
      xcb::CONN_CLOSED_INVALID_SCREEN => Some(Self::ClosedInvalidScreen),
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
