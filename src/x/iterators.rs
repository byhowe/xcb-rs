use crate::xcb;

pub struct ArcIterator
{
  iter: xcb::arc_iterator_t,
}

impl From<xcb::arc_iterator_t> for ArcIterator
{
  fn from(iter: xcb::arc_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ArcIterator
{
  type Item = xcb::arc_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::arc_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AtomIterator
{
  iter: xcb::atom_iterator_t,
}

impl From<xcb::atom_iterator_t> for AtomIterator
{
  fn from(iter: xcb::atom_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AtomIterator
{
  type Item = xcb::atom_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::atom_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Bool32Iterator
{
  iter: xcb::bool32_iterator_t,
}

impl From<xcb::bool32_iterator_t> for Bool32Iterator
{
  fn from(iter: xcb::bool32_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Bool32Iterator
{
  type Item = xcb::bool32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::bool32_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonIterator
{
  iter: xcb::button_iterator_t,
}

impl From<xcb::button_iterator_t> for ButtonIterator
{
  fn from(iter: xcb::button_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ButtonIterator
{
  type Item = xcb::button_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::button_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Char2bIterator
{
  iter: xcb::char2b_iterator_t,
}

impl From<xcb::char2b_iterator_t> for Char2bIterator
{
  fn from(iter: xcb::char2b_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Char2bIterator
{
  type Item = xcb::char2b_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::char2b_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CharinfoIterator
{
  iter: xcb::charinfo_iterator_t,
}

impl From<xcb::charinfo_iterator_t> for CharinfoIterator
{
  fn from(iter: xcb::charinfo_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CharinfoIterator
{
  type Item = xcb::charinfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::charinfo_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientMessageDataIterator
{
  iter: xcb::client_message_data_iterator_t,
}

impl From<xcb::client_message_data_iterator_t> for ClientMessageDataIterator
{
  fn from(iter: xcb::client_message_data_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientMessageDataIterator
{
  type Item = xcb::client_message_data_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::client_message_data_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColoritemIterator
{
  iter: xcb::coloritem_iterator_t,
}

impl From<xcb::coloritem_iterator_t> for ColoritemIterator
{
  fn from(iter: xcb::coloritem_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ColoritemIterator
{
  type Item = xcb::coloritem_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::coloritem_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColormapIterator
{
  iter: xcb::colormap_iterator_t,
}

impl From<xcb::colormap_iterator_t> for ColormapIterator
{
  fn from(iter: xcb::colormap_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ColormapIterator
{
  type Item = xcb::colormap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::colormap_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CursorIterator
{
  iter: xcb::cursor_iterator_t,
}

impl From<xcb::cursor_iterator_t> for CursorIterator
{
  fn from(iter: xcb::cursor_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CursorIterator
{
  type Item = xcb::cursor_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::cursor_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DamageIterator
{
  iter: xcb::damage::damage_iterator_t,
}

impl From<xcb::damage::damage_iterator_t> for DamageIterator
{
  fn from(iter: xcb::damage::damage_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DamageIterator
{
  type Item = xcb::damage::damage_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::damage::damage_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DepthIterator
{
  iter: xcb::depth_iterator_t,
}

impl From<xcb::depth_iterator_t> for DepthIterator
{
  fn from(iter: xcb::depth_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DepthIterator
{
  type Item = xcb::depth_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::depth_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DrawableIterator
{
  iter: xcb::drawable_iterator_t,
}

impl From<xcb::drawable_iterator_t> for DrawableIterator
{
  fn from(iter: xcb::drawable_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DrawableIterator
{
  type Item = xcb::drawable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::drawable_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttachFormatIterator
{
  iter: xcb::dri2::attach_format_iterator_t,
}

impl From<xcb::dri2::attach_format_iterator_t> for AttachFormatIterator
{
  fn from(iter: xcb::dri2::attach_format_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AttachFormatIterator
{
  type Item = xcb::dri2::attach_format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::dri2::attach_format_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Dri2BufferIterator
{
  iter: xcb::dri2::dri2_buffer_iterator_t,
}

impl From<xcb::dri2::dri2_buffer_iterator_t> for Dri2BufferIterator
{
  fn from(iter: xcb::dri2::dri2_buffer_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Dri2BufferIterator
{
  type Item = xcb::dri2::dri2_buffer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::dri2::dri2_buffer_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct WmIconIterator
{
  iter: xcb::ewmh::wm_icon_iterator_t,
}

impl From<xcb::ewmh::wm_icon_iterator_t> for WmIconIterator
{
  fn from(iter: xcb::ewmh::wm_icon_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for WmIconIterator
{
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::ewmh::get_wm_icon_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontIterator
{
  iter: xcb::font_iterator_t,
}

impl From<xcb::font_iterator_t> for FontIterator
{
  fn from(iter: xcb::font_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FontIterator
{
  type Item = xcb::font_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::font_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontableIterator
{
  iter: xcb::fontable_iterator_t,
}

impl From<xcb::fontable_iterator_t> for FontableIterator
{
  fn from(iter: xcb::fontable_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FontableIterator
{
  type Item = xcb::fontable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::fontable_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontpropIterator
{
  iter: xcb::fontprop_iterator_t,
}

impl From<xcb::fontprop_iterator_t> for FontpropIterator
{
  fn from(iter: xcb::fontprop_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FontpropIterator
{
  type Item = xcb::fontprop_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::fontprop_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FormatIterator
{
  iter: xcb::format_iterator_t,
}

impl From<xcb::format_iterator_t> for FormatIterator
{
  fn from(iter: xcb::format_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FormatIterator
{
  type Item = xcb::format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::format_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GcontextIterator
{
  iter: xcb::gcontext_iterator_t,
}

impl From<xcb::gcontext_iterator_t> for GcontextIterator
{
  fn from(iter: xcb::gcontext_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GcontextIterator
{
  type Item = xcb::gcontext_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::gcontext_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxBool32Iterator
{
  iter: xcb::glx::bool32_iterator_t,
}

impl From<xcb::glx::bool32_iterator_t> for GlxBool32Iterator
{
  fn from(iter: xcb::glx::bool32_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlxBool32Iterator
{
  type Item = xcb::glx::bool32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::bool32_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxContextIterator
{
  iter: xcb::glx::context_iterator_t,
}

impl From<xcb::glx::context_iterator_t> for GlxContextIterator
{
  fn from(iter: xcb::glx::context_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlxContextIterator
{
  type Item = xcb::glx::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::context_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ContextTagIterator
{
  iter: xcb::glx::context_tag_iterator_t,
}

impl From<xcb::glx::context_tag_iterator_t> for ContextTagIterator
{
  fn from(iter: xcb::glx::context_tag_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ContextTagIterator
{
  type Item = xcb::glx::context_tag_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::context_tag_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxDrawableIterator
{
  iter: xcb::glx::drawable_iterator_t,
}

impl From<xcb::glx::drawable_iterator_t> for GlxDrawableIterator
{
  fn from(iter: xcb::glx::drawable_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlxDrawableIterator
{
  type Item = xcb::glx::drawable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::drawable_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FbconfigIterator
{
  iter: xcb::glx::fbconfig_iterator_t,
}

impl From<xcb::glx::fbconfig_iterator_t> for FbconfigIterator
{
  fn from(iter: xcb::glx::fbconfig_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FbconfigIterator
{
  type Item = xcb::glx::fbconfig_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::fbconfig_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Float32Iterator
{
  iter: xcb::glx::float32_iterator_t,
}

impl From<xcb::glx::float32_iterator_t> for Float32Iterator
{
  fn from(iter: xcb::glx::float32_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Float32Iterator
{
  type Item = xcb::glx::float32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::float32_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Float64Iterator
{
  iter: xcb::glx::float64_iterator_t,
}

impl From<xcb::glx::float64_iterator_t> for Float64Iterator
{
  fn from(iter: xcb::glx::float64_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Float64Iterator
{
  type Item = xcb::glx::float64_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::float64_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PbufferIterator
{
  iter: xcb::glx::pbuffer_iterator_t,
}

impl From<xcb::glx::pbuffer_iterator_t> for PbufferIterator
{
  fn from(iter: xcb::glx::pbuffer_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PbufferIterator
{
  type Item = xcb::glx::pbuffer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::pbuffer_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxPixmapIterator
{
  iter: xcb::glx::pixmap_iterator_t,
}

impl From<xcb::glx::pixmap_iterator_t> for GlxPixmapIterator
{
  fn from(iter: xcb::glx::pixmap_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlxPixmapIterator
{
  type Item = xcb::glx::pixmap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::pixmap_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxWindowIterator
{
  iter: xcb::glx::window_iterator_t,
}

impl From<xcb::glx::window_iterator_t> for GlxWindowIterator
{
  fn from(iter: xcb::glx::window_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlxWindowIterator
{
  type Item = xcb::glx::window_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::glx::window_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct HostIterator
{
  iter: xcb::host_iterator_t,
}

impl From<xcb::host_iterator_t> for HostIterator
{
  fn from(iter: xcb::host_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for HostIterator
{
  type Item = xcb::host_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::host_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AddMasterIterator
{
  iter: xcb::input::add_master_iterator_t,
}

impl From<xcb::input::add_master_iterator_t> for AddMasterIterator
{
  fn from(iter: xcb::input::add_master_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AddMasterIterator
{
  type Item = xcb::input::add_master_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::add_master_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttachSlaveIterator
{
  iter: xcb::input::attach_slave_iterator_t,
}

impl From<xcb::input::attach_slave_iterator_t> for AttachSlaveIterator
{
  fn from(iter: xcb::input::attach_slave_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AttachSlaveIterator
{
  type Item = xcb::input::attach_slave_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::attach_slave_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AxisInfoIterator
{
  iter: xcb::input::axis_info_iterator_t,
}

impl From<xcb::input::axis_info_iterator_t> for AxisInfoIterator
{
  fn from(iter: xcb::input::axis_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AxisInfoIterator
{
  type Item = xcb::input::axis_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::axis_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BarrierReleasePointerInfoIterator
{
  iter: xcb::input::barrier_release_pointer_info_iterator_t,
}

impl From<xcb::input::barrier_release_pointer_info_iterator_t> for BarrierReleasePointerInfoIterator
{
  fn from(iter: xcb::input::barrier_release_pointer_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BarrierReleasePointerInfoIterator
{
  type Item = xcb::input::barrier_release_pointer_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::barrier_release_pointer_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellFeedbackCtlIterator
{
  iter: xcb::input::bell_feedback_ctl_iterator_t,
}

impl From<xcb::input::bell_feedback_ctl_iterator_t> for BellFeedbackCtlIterator
{
  fn from(iter: xcb::input::bell_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BellFeedbackCtlIterator
{
  type Item = xcb::input::bell_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::bell_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellFeedbackStateIterator
{
  iter: xcb::input::bell_feedback_state_iterator_t,
}

impl From<xcb::input::bell_feedback_state_iterator_t> for BellFeedbackStateIterator
{
  fn from(iter: xcb::input::bell_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BellFeedbackStateIterator
{
  type Item = xcb::input::bell_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::bell_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonClassIterator
{
  iter: xcb::input::button_class_iterator_t,
}

impl From<xcb::input::button_class_iterator_t> for ButtonClassIterator
{
  fn from(iter: xcb::input::button_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ButtonClassIterator
{
  type Item = xcb::input::button_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::button_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonInfoIterator
{
  iter: xcb::input::button_info_iterator_t,
}

impl From<xcb::input::button_info_iterator_t> for ButtonInfoIterator
{
  fn from(iter: xcb::input::button_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ButtonInfoIterator
{
  type Item = xcb::input::button_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::button_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonStateIterator
{
  iter: xcb::input::button_state_iterator_t,
}

impl From<xcb::input::button_state_iterator_t> for ButtonStateIterator
{
  fn from(iter: xcb::input::button_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ButtonStateIterator
{
  type Item = xcb::input::button_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::button_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DetachSlaveIterator
{
  iter: xcb::input::detach_slave_iterator_t,
}

impl From<xcb::input::detach_slave_iterator_t> for DetachSlaveIterator
{
  fn from(iter: xcb::input::detach_slave_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DetachSlaveIterator
{
  type Item = xcb::input::detach_slave_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::detach_slave_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsAreaCtrlIterator
{
  iter: xcb::input::device_abs_area_ctrl_iterator_t,
}

impl From<xcb::input::device_abs_area_ctrl_iterator_t> for DeviceAbsAreaCtrlIterator
{
  fn from(iter: xcb::input::device_abs_area_ctrl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceAbsAreaCtrlIterator
{
  type Item = xcb::input::device_abs_area_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_abs_area_ctrl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsAreaStateIterator
{
  iter: xcb::input::device_abs_area_state_iterator_t,
}

impl From<xcb::input::device_abs_area_state_iterator_t> for DeviceAbsAreaStateIterator
{
  fn from(iter: xcb::input::device_abs_area_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceAbsAreaStateIterator
{
  type Item = xcb::input::device_abs_area_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_abs_area_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsCalibCtlIterator
{
  iter: xcb::input::device_abs_calib_ctl_iterator_t,
}

impl From<xcb::input::device_abs_calib_ctl_iterator_t> for DeviceAbsCalibCtlIterator
{
  fn from(iter: xcb::input::device_abs_calib_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceAbsCalibCtlIterator
{
  type Item = xcb::input::device_abs_calib_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_abs_calib_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsCalibStateIterator
{
  iter: xcb::input::device_abs_calib_state_iterator_t,
}

impl From<xcb::input::device_abs_calib_state_iterator_t> for DeviceAbsCalibStateIterator
{
  fn from(iter: xcb::input::device_abs_calib_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceAbsCalibStateIterator
{
  type Item = xcb::input::device_abs_calib_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_abs_calib_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceClassIterator
{
  iter: xcb::input::device_class_iterator_t,
}

impl From<xcb::input::device_class_iterator_t> for DeviceClassIterator
{
  fn from(iter: xcb::input::device_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceClassIterator
{
  type Item = xcb::input::device_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCoreCtrlIterator
{
  iter: xcb::input::device_core_ctrl_iterator_t,
}

impl From<xcb::input::device_core_ctrl_iterator_t> for DeviceCoreCtrlIterator
{
  fn from(iter: xcb::input::device_core_ctrl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceCoreCtrlIterator
{
  type Item = xcb::input::device_core_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_core_ctrl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCoreStateIterator
{
  iter: xcb::input::device_core_state_iterator_t,
}

impl From<xcb::input::device_core_state_iterator_t> for DeviceCoreStateIterator
{
  fn from(iter: xcb::input::device_core_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceCoreStateIterator
{
  type Item = xcb::input::device_core_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_core_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCtlIterator
{
  iter: xcb::input::device_ctl_iterator_t,
}

impl From<xcb::input::device_ctl_iterator_t> for DeviceCtlIterator
{
  fn from(iter: xcb::input::device_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceCtlIterator
{
  type Item = xcb::input::device_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceEnableCtrlIterator
{
  iter: xcb::input::device_enable_ctrl_iterator_t,
}

impl From<xcb::input::device_enable_ctrl_iterator_t> for DeviceEnableCtrlIterator
{
  fn from(iter: xcb::input::device_enable_ctrl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceEnableCtrlIterator
{
  type Item = xcb::input::device_enable_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_enable_ctrl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceEnableStateIterator
{
  iter: xcb::input::device_enable_state_iterator_t,
}

impl From<xcb::input::device_enable_state_iterator_t> for DeviceEnableStateIterator
{
  fn from(iter: xcb::input::device_enable_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceEnableStateIterator
{
  type Item = xcb::input::device_enable_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_enable_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceIdIterator
{
  iter: xcb::input::device_id_iterator_t,
}

impl From<xcb::input::device_id_iterator_t> for DeviceIdIterator
{
  fn from(iter: xcb::input::device_id_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceIdIterator
{
  type Item = xcb::input::device_id_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_id_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceInfoIterator
{
  iter: xcb::input::device_info_iterator_t,
}

impl From<xcb::input::device_info_iterator_t> for DeviceInfoIterator
{
  fn from(iter: xcb::input::device_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceInfoIterator
{
  type Item = xcb::input::device_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceNameIterator
{
  iter: xcb::input::device_name_iterator_t,
}

impl From<xcb::input::device_name_iterator_t> for DeviceNameIterator
{
  fn from(iter: xcb::input::device_name_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceNameIterator
{
  type Item = xcb::input::device_name_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_name_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceResolutionCtlIterator
{
  iter: xcb::input::device_resolution_ctl_iterator_t,
}

impl From<xcb::input::device_resolution_ctl_iterator_t> for DeviceResolutionCtlIterator
{
  fn from(iter: xcb::input::device_resolution_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceResolutionCtlIterator
{
  type Item = xcb::input::device_resolution_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_resolution_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceResolutionStateIterator
{
  iter: xcb::input::device_resolution_state_iterator_t,
}

impl From<xcb::input::device_resolution_state_iterator_t> for DeviceResolutionStateIterator
{
  fn from(iter: xcb::input::device_resolution_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceResolutionStateIterator
{
  type Item = xcb::input::device_resolution_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_resolution_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceStateIterator
{
  iter: xcb::input::device_state_iterator_t,
}

impl From<xcb::input::device_state_iterator_t> for DeviceStateIterator
{
  fn from(iter: xcb::input::device_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceStateIterator
{
  type Item = xcb::input::device_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceTimeCoordIterator
{
  iter: xcb::input::device_time_coord_iterator_t,
}

impl From<xcb::input::device_time_coord_iterator_t> for DeviceTimeCoordIterator
{
  fn from(iter: xcb::input::device_time_coord_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceTimeCoordIterator
{
  type Item = xcb::input::device_time_coord_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::device_time_coord_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventClassIterator
{
  iter: xcb::input::event_class_iterator_t,
}

impl From<xcb::input::event_class_iterator_t> for EventClassIterator
{
  fn from(iter: xcb::input::event_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EventClassIterator
{
  type Item = xcb::input::event_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::event_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventForSendIterator
{
  iter: xcb::input::event_for_send_iterator_t,
}

impl From<xcb::input::event_for_send_iterator_t> for EventForSendIterator
{
  fn from(iter: xcb::input::event_for_send_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EventForSendIterator
{
  type Item = xcb::input::event_for_send_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::event_for_send_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventMaskIterator
{
  iter: xcb::input::event_mask_iterator_t,
}

impl From<xcb::input::event_mask_iterator_t> for EventMaskIterator
{
  fn from(iter: xcb::input::event_mask_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EventMaskIterator
{
  type Item = xcb::input::event_mask_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::event_mask_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventTypeBaseIterator
{
  iter: xcb::input::event_type_base_iterator_t,
}

impl From<xcb::input::event_type_base_iterator_t> for EventTypeBaseIterator
{
  fn from(iter: xcb::input::event_type_base_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EventTypeBaseIterator
{
  type Item = xcb::input::event_type_base_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::event_type_base_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FeedbackCtlIterator
{
  iter: xcb::input::feedback_ctl_iterator_t,
}

impl From<xcb::input::feedback_ctl_iterator_t> for FeedbackCtlIterator
{
  fn from(iter: xcb::input::feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FeedbackCtlIterator
{
  type Item = xcb::input::feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FeedbackStateIterator
{
  iter: xcb::input::feedback_state_iterator_t,
}

impl From<xcb::input::feedback_state_iterator_t> for FeedbackStateIterator
{
  fn from(iter: xcb::input::feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FeedbackStateIterator
{
  type Item = xcb::input::feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Fp1616Iterator
{
  iter: xcb::input::fp1616_iterator_t,
}

impl From<xcb::input::fp1616_iterator_t> for Fp1616Iterator
{
  fn from(iter: xcb::input::fp1616_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Fp1616Iterator
{
  type Item = xcb::input::fp1616_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::fp1616_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Fp3232Iterator
{
  iter: xcb::input::fp3232_iterator_t,
}

impl From<xcb::input::fp3232_iterator_t> for Fp3232Iterator
{
  fn from(iter: xcb::input::fp3232_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Fp3232Iterator
{
  type Item = xcb::input::fp3232_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::fp3232_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GrabModifierInfoIterator
{
  iter: xcb::input::grab_modifier_info_iterator_t,
}

impl From<xcb::input::grab_modifier_info_iterator_t> for GrabModifierInfoIterator
{
  fn from(iter: xcb::input::grab_modifier_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GrabModifierInfoIterator
{
  type Item = xcb::input::grab_modifier_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::grab_modifier_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GroupInfoIterator
{
  iter: xcb::input::group_info_iterator_t,
}

impl From<xcb::input::group_info_iterator_t> for GroupInfoIterator
{
  fn from(iter: xcb::input::group_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GroupInfoIterator
{
  type Item = xcb::input::group_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::group_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct HierarchyChangeIterator
{
  iter: xcb::input::hierarchy_change_iterator_t,
}

impl From<xcb::input::hierarchy_change_iterator_t> for HierarchyChangeIterator
{
  fn from(iter: xcb::input::hierarchy_change_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for HierarchyChangeIterator
{
  type Item = xcb::input::hierarchy_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::hierarchy_change_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct HierarchyInfoIterator
{
  iter: xcb::input::hierarchy_info_iterator_t,
}

impl From<xcb::input::hierarchy_info_iterator_t> for HierarchyInfoIterator
{
  fn from(iter: xcb::input::hierarchy_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for HierarchyInfoIterator
{
  type Item = xcb::input::hierarchy_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::hierarchy_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputClassInfoIterator
{
  iter: xcb::input::input_class_info_iterator_t,
}

impl From<xcb::input::input_class_info_iterator_t> for InputClassInfoIterator
{
  fn from(iter: xcb::input::input_class_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for InputClassInfoIterator
{
  type Item = xcb::input::input_class_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::input_class_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputInfoIterator
{
  iter: xcb::input::input_info_iterator_t,
}

impl From<xcb::input::input_info_iterator_t> for InputInfoIterator
{
  fn from(iter: xcb::input::input_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for InputInfoIterator
{
  type Item = xcb::input::input_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::input_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputStateIterator
{
  iter: xcb::input::input_state_iterator_t,
}

impl From<xcb::input::input_state_iterator_t> for InputStateIterator
{
  fn from(iter: xcb::input::input_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for InputStateIterator
{
  type Item = xcb::input::input_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::input_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct IntegerFeedbackCtlIterator
{
  iter: xcb::input::integer_feedback_ctl_iterator_t,
}

impl From<xcb::input::integer_feedback_ctl_iterator_t> for IntegerFeedbackCtlIterator
{
  fn from(iter: xcb::input::integer_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for IntegerFeedbackCtlIterator
{
  type Item = xcb::input::integer_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::integer_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct IntegerFeedbackStateIterator
{
  iter: xcb::input::integer_feedback_state_iterator_t,
}

impl From<xcb::input::integer_feedback_state_iterator_t> for IntegerFeedbackStateIterator
{
  fn from(iter: xcb::input::integer_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for IntegerFeedbackStateIterator
{
  type Item = xcb::input::integer_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::integer_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KbdFeedbackCtlIterator
{
  iter: xcb::input::kbd_feedback_ctl_iterator_t,
}

impl From<xcb::input::kbd_feedback_ctl_iterator_t> for KbdFeedbackCtlIterator
{
  fn from(iter: xcb::input::kbd_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KbdFeedbackCtlIterator
{
  type Item = xcb::input::kbd_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::kbd_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KbdFeedbackStateIterator
{
  iter: xcb::input::kbd_feedback_state_iterator_t,
}

impl From<xcb::input::kbd_feedback_state_iterator_t> for KbdFeedbackStateIterator
{
  fn from(iter: xcb::input::kbd_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KbdFeedbackStateIterator
{
  type Item = xcb::input::kbd_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::kbd_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyClassIterator
{
  iter: xcb::input::key_class_iterator_t,
}

impl From<xcb::input::key_class_iterator_t> for KeyClassIterator
{
  fn from(iter: xcb::input::key_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyClassIterator
{
  type Item = xcb::input::key_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::key_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyCodeIterator
{
  iter: xcb::input::key_code_iterator_t,
}

impl From<xcb::input::key_code_iterator_t> for KeyCodeIterator
{
  fn from(iter: xcb::input::key_code_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyCodeIterator
{
  type Item = xcb::input::key_code_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::key_code_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyInfoIterator
{
  iter: xcb::input::key_info_iterator_t,
}

impl From<xcb::input::key_info_iterator_t> for KeyInfoIterator
{
  fn from(iter: xcb::input::key_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyInfoIterator
{
  type Item = xcb::input::key_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::key_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyStateIterator
{
  iter: xcb::input::key_state_iterator_t,
}

impl From<xcb::input::key_state_iterator_t> for KeyStateIterator
{
  fn from(iter: xcb::input::key_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyStateIterator
{
  type Item = xcb::input::key_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::key_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedFeedbackCtlIterator
{
  iter: xcb::input::led_feedback_ctl_iterator_t,
}

impl From<xcb::input::led_feedback_ctl_iterator_t> for LedFeedbackCtlIterator
{
  fn from(iter: xcb::input::led_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LedFeedbackCtlIterator
{
  type Item = xcb::input::led_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::led_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedFeedbackStateIterator
{
  iter: xcb::input::led_feedback_state_iterator_t,
}

impl From<xcb::input::led_feedback_state_iterator_t> for LedFeedbackStateIterator
{
  fn from(iter: xcb::input::led_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LedFeedbackStateIterator
{
  type Item = xcb::input::led_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::led_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModifierInfoIterator
{
  iter: xcb::input::modifier_info_iterator_t,
}

impl From<xcb::input::modifier_info_iterator_t> for ModifierInfoIterator
{
  fn from(iter: xcb::input::modifier_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ModifierInfoIterator
{
  type Item = xcb::input::modifier_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::modifier_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PtrFeedbackCtlIterator
{
  iter: xcb::input::ptr_feedback_ctl_iterator_t,
}

impl From<xcb::input::ptr_feedback_ctl_iterator_t> for PtrFeedbackCtlIterator
{
  fn from(iter: xcb::input::ptr_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PtrFeedbackCtlIterator
{
  type Item = xcb::input::ptr_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::ptr_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PtrFeedbackStateIterator
{
  iter: xcb::input::ptr_feedback_state_iterator_t,
}

impl From<xcb::input::ptr_feedback_state_iterator_t> for PtrFeedbackStateIterator
{
  fn from(iter: xcb::input::ptr_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PtrFeedbackStateIterator
{
  type Item = xcb::input::ptr_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::ptr_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RemoveMasterIterator
{
  iter: xcb::input::remove_master_iterator_t,
}

impl From<xcb::input::remove_master_iterator_t> for RemoveMasterIterator
{
  fn from(iter: xcb::input::remove_master_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RemoveMasterIterator
{
  type Item = xcb::input::remove_master_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::remove_master_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScrollClassIterator
{
  iter: xcb::input::scroll_class_iterator_t,
}

impl From<xcb::input::scroll_class_iterator_t> for ScrollClassIterator
{
  fn from(iter: xcb::input::scroll_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ScrollClassIterator
{
  type Item = xcb::input::scroll_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::scroll_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct StringFeedbackCtlIterator
{
  iter: xcb::input::string_feedback_ctl_iterator_t,
}

impl From<xcb::input::string_feedback_ctl_iterator_t> for StringFeedbackCtlIterator
{
  fn from(iter: xcb::input::string_feedback_ctl_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for StringFeedbackCtlIterator
{
  type Item = xcb::input::string_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::string_feedback_ctl_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct StringFeedbackStateIterator
{
  iter: xcb::input::string_feedback_state_iterator_t,
}

impl From<xcb::input::string_feedback_state_iterator_t> for StringFeedbackStateIterator
{
  fn from(iter: xcb::input::string_feedback_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for StringFeedbackStateIterator
{
  type Item = xcb::input::string_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::string_feedback_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TouchClassIterator
{
  iter: xcb::input::touch_class_iterator_t,
}

impl From<xcb::input::touch_class_iterator_t> for TouchClassIterator
{
  fn from(iter: xcb::input::touch_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TouchClassIterator
{
  type Item = xcb::input::touch_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::touch_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorClassIterator
{
  iter: xcb::input::valuator_class_iterator_t,
}

impl From<xcb::input::valuator_class_iterator_t> for ValuatorClassIterator
{
  fn from(iter: xcb::input::valuator_class_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ValuatorClassIterator
{
  type Item = xcb::input::valuator_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::valuator_class_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorInfoIterator
{
  iter: xcb::input::valuator_info_iterator_t,
}

impl From<xcb::input::valuator_info_iterator_t> for ValuatorInfoIterator
{
  fn from(iter: xcb::input::valuator_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ValuatorInfoIterator
{
  type Item = xcb::input::valuator_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::valuator_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorStateIterator
{
  iter: xcb::input::valuator_state_iterator_t,
}

impl From<xcb::input::valuator_state_iterator_t> for ValuatorStateIterator
{
  fn from(iter: xcb::input::valuator_state_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ValuatorStateIterator
{
  type Item = xcb::input::valuator_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::valuator_state_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XiDeviceInfoIterator
{
  iter: xcb::input::xi_device_info_iterator_t,
}

impl From<xcb::input::xi_device_info_iterator_t> for XiDeviceInfoIterator
{
  fn from(iter: xcb::input::xi_device_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XiDeviceInfoIterator
{
  type Item = xcb::input::xi_device_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::input::xi_device_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Keycode32Iterator
{
  iter: xcb::keycode32_iterator_t,
}

impl From<xcb::keycode32_iterator_t> for Keycode32Iterator
{
  fn from(iter: xcb::keycode32_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Keycode32Iterator
{
  type Item = xcb::keycode32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::keycode32_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeycodeIterator
{
  iter: xcb::keycode_iterator_t,
}

impl From<xcb::keycode_iterator_t> for KeycodeIterator
{
  fn from(iter: xcb::keycode_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeycodeIterator
{
  type Item = xcb::keycode_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::keycode_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeysymIterator
{
  iter: xcb::keysym_iterator_t,
}

impl From<xcb::keysym_iterator_t> for KeysymIterator
{
  fn from(iter: xcb::keysym_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeysymIterator
{
  type Item = xcb::keysym_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::keysym_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PixmapIterator
{
  iter: xcb::pixmap_iterator_t,
}

impl From<xcb::pixmap_iterator_t> for PixmapIterator
{
  fn from(iter: xcb::pixmap_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PixmapIterator
{
  type Item = xcb::pixmap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::pixmap_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PointIterator
{
  iter: xcb::point_iterator_t,
}

impl From<xcb::point_iterator_t> for PointIterator
{
  fn from(iter: xcb::point_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PointIterator
{
  type Item = xcb::point_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::point_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PresentEventIterator
{
  iter: xcb::present::event_iterator_t,
}

impl From<xcb::present::event_iterator_t> for PresentEventIterator
{
  fn from(iter: xcb::present::event_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PresentEventIterator
{
  type Item = xcb::present::event_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::present::event_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct NotifyIterator
{
  iter: xcb::present::notify_iterator_t,
}

impl From<xcb::present::notify_iterator_t> for NotifyIterator
{
  fn from(iter: xcb::present::notify_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for NotifyIterator
{
  type Item = xcb::present::notify_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::present::notify_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CrtcChangeIterator
{
  iter: xcb::randr::crtc_change_iterator_t,
}

impl From<xcb::randr::crtc_change_iterator_t> for CrtcChangeIterator
{
  fn from(iter: xcb::randr::crtc_change_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CrtcChangeIterator
{
  type Item = xcb::randr::crtc_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::crtc_change_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CrtcIterator
{
  iter: xcb::randr::crtc_iterator_t,
}

impl From<xcb::randr::crtc_iterator_t> for CrtcIterator
{
  fn from(iter: xcb::randr::crtc_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CrtcIterator
{
  type Item = xcb::randr::crtc_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::crtc_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LeaseIterator
{
  iter: xcb::randr::lease_iterator_t,
}

impl From<xcb::randr::lease_iterator_t> for LeaseIterator
{
  fn from(iter: xcb::randr::lease_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LeaseIterator
{
  type Item = xcb::randr::lease_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::lease_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LeaseNotifyIterator
{
  iter: xcb::randr::lease_notify_iterator_t,
}

impl From<xcb::randr::lease_notify_iterator_t> for LeaseNotifyIterator
{
  fn from(iter: xcb::randr::lease_notify_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LeaseNotifyIterator
{
  type Item = xcb::randr::lease_notify_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::lease_notify_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModeInfoIterator
{
  iter: xcb::randr::mode_info_iterator_t,
}

impl From<xcb::randr::mode_info_iterator_t> for ModeInfoIterator
{
  fn from(iter: xcb::randr::mode_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ModeInfoIterator
{
  type Item = xcb::randr::mode_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::mode_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModeIterator
{
  iter: xcb::randr::mode_iterator_t,
}

impl From<xcb::randr::mode_iterator_t> for ModeIterator
{
  fn from(iter: xcb::randr::mode_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ModeIterator
{
  type Item = xcb::randr::mode_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::mode_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct MonitorInfoIterator
{
  iter: xcb::randr::monitor_info_iterator_t,
}

impl From<xcb::randr::monitor_info_iterator_t> for MonitorInfoIterator
{
  fn from(iter: xcb::randr::monitor_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for MonitorInfoIterator
{
  type Item = xcb::randr::monitor_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::monitor_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct NotifyDataIterator
{
  iter: xcb::randr::notify_data_iterator_t,
}

impl From<xcb::randr::notify_data_iterator_t> for NotifyDataIterator
{
  fn from(iter: xcb::randr::notify_data_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for NotifyDataIterator
{
  type Item = xcb::randr::notify_data_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::notify_data_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputChangeIterator
{
  iter: xcb::randr::output_change_iterator_t,
}

impl From<xcb::randr::output_change_iterator_t> for OutputChangeIterator
{
  fn from(iter: xcb::randr::output_change_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OutputChangeIterator
{
  type Item = xcb::randr::output_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::output_change_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputIterator
{
  iter: xcb::randr::output_iterator_t,
}

impl From<xcb::randr::output_iterator_t> for OutputIterator
{
  fn from(iter: xcb::randr::output_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OutputIterator
{
  type Item = xcb::randr::output_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::output_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputPropertyIterator
{
  iter: xcb::randr::output_property_iterator_t,
}

impl From<xcb::randr::output_property_iterator_t> for OutputPropertyIterator
{
  fn from(iter: xcb::randr::output_property_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OutputPropertyIterator
{
  type Item = xcb::randr::output_property_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::output_property_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderChangeIterator
{
  iter: xcb::randr::provider_change_iterator_t,
}

impl From<xcb::randr::provider_change_iterator_t> for ProviderChangeIterator
{
  fn from(iter: xcb::randr::provider_change_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ProviderChangeIterator
{
  type Item = xcb::randr::provider_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::provider_change_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderIterator
{
  iter: xcb::randr::provider_iterator_t,
}

impl From<xcb::randr::provider_iterator_t> for ProviderIterator
{
  fn from(iter: xcb::randr::provider_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ProviderIterator
{
  type Item = xcb::randr::provider_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::provider_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderPropertyIterator
{
  iter: xcb::randr::provider_property_iterator_t,
}

impl From<xcb::randr::provider_property_iterator_t> for ProviderPropertyIterator
{
  fn from(iter: xcb::randr::provider_property_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ProviderPropertyIterator
{
  type Item = xcb::randr::provider_property_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::provider_property_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RefreshRatesIterator
{
  iter: xcb::randr::refresh_rates_iterator_t,
}

impl From<xcb::randr::refresh_rates_iterator_t> for RefreshRatesIterator
{
  fn from(iter: xcb::randr::refresh_rates_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RefreshRatesIterator
{
  type Item = xcb::randr::refresh_rates_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::refresh_rates_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceChangeIterator
{
  iter: xcb::randr::resource_change_iterator_t,
}

impl From<xcb::randr::resource_change_iterator_t> for ResourceChangeIterator
{
  fn from(iter: xcb::randr::resource_change_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ResourceChangeIterator
{
  type Item = xcb::randr::resource_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::resource_change_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenSizeIterator
{
  iter: xcb::randr::screen_size_iterator_t,
}

impl From<xcb::randr::screen_size_iterator_t> for ScreenSizeIterator
{
  fn from(iter: xcb::randr::screen_size_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ScreenSizeIterator
{
  type Item = xcb::randr::screen_size_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::randr::screen_size_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientInfoIterator
{
  iter: xcb::record::client_info_iterator_t,
}

impl From<xcb::record::client_info_iterator_t> for ClientInfoIterator
{
  fn from(iter: xcb::record::client_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientInfoIterator
{
  type Item = xcb::record::client_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::client_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientSpecIterator
{
  iter: xcb::record::client_spec_iterator_t,
}

impl From<xcb::record::client_spec_iterator_t> for ClientSpecIterator
{
  fn from(iter: xcb::record::client_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientSpecIterator
{
  type Item = xcb::record::client_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::client_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RecordContextIterator
{
  iter: xcb::record::context_iterator_t,
}

impl From<xcb::record::context_iterator_t> for RecordContextIterator
{
  fn from(iter: xcb::record::context_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RecordContextIterator
{
  type Item = xcb::record::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::context_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ElementHeaderIterator
{
  iter: xcb::record::element_header_iterator_t,
}

impl From<xcb::record::element_header_iterator_t> for ElementHeaderIterator
{
  fn from(iter: xcb::record::element_header_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ElementHeaderIterator
{
  type Item = xcb::record::element_header_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::element_header_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ExtRangeIterator
{
  iter: xcb::record::ext_range_iterator_t,
}

impl From<xcb::record::ext_range_iterator_t> for ExtRangeIterator
{
  fn from(iter: xcb::record::ext_range_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ExtRangeIterator
{
  type Item = xcb::record::ext_range_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::ext_range_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Range16Iterator
{
  iter: xcb::record::range_16_iterator_t,
}

impl From<xcb::record::range_16_iterator_t> for Range16Iterator
{
  fn from(iter: xcb::record::range_16_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Range16Iterator
{
  type Item = xcb::record::range_16_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::range_16_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Range8Iterator
{
  iter: xcb::record::range_8_iterator_t,
}

impl From<xcb::record::range_8_iterator_t> for Range8Iterator
{
  fn from(iter: xcb::record::range_8_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Range8Iterator
{
  type Item = xcb::record::range_8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::range_8_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RangeIterator
{
  iter: xcb::record::range_iterator_t,
}

impl From<xcb::record::range_iterator_t> for RangeIterator
{
  fn from(iter: xcb::record::range_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RangeIterator
{
  type Item = xcb::record::range_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::record::range_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RectangleIterator
{
  iter: xcb::rectangle_iterator_t,
}

impl From<xcb::rectangle_iterator_t> for RectangleIterator
{
  fn from(iter: xcb::rectangle_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RectangleIterator
{
  type Item = xcb::rectangle_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::rectangle_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AnimcursoreltIterator
{
  iter: xcb::render::animcursorelt_iterator_t,
}

impl From<xcb::render::animcursorelt_iterator_t> for AnimcursoreltIterator
{
  fn from(iter: xcb::render::animcursorelt_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AnimcursoreltIterator
{
  type Item = xcb::render::animcursorelt_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::animcursorelt_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColorIterator
{
  iter: xcb::render::color_iterator_t,
}

impl From<xcb::render::color_iterator_t> for ColorIterator
{
  fn from(iter: xcb::render::color_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ColorIterator
{
  type Item = xcb::render::color_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::color_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DirectformatIterator
{
  iter: xcb::render::directformat_iterator_t,
}

impl From<xcb::render::directformat_iterator_t> for DirectformatIterator
{
  fn from(iter: xcb::render::directformat_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DirectformatIterator
{
  type Item = xcb::render::directformat_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::directformat_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FixedIterator
{
  iter: xcb::render::fixed_iterator_t,
}

impl From<xcb::render::fixed_iterator_t> for FixedIterator
{
  fn from(iter: xcb::render::fixed_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FixedIterator
{
  type Item = xcb::render::fixed_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::fixed_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphIterator
{
  iter: xcb::render::glyph_iterator_t,
}

impl From<xcb::render::glyph_iterator_t> for GlyphIterator
{
  fn from(iter: xcb::render::glyph_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlyphIterator
{
  type Item = xcb::render::glyph_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::glyph_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphinfoIterator
{
  iter: xcb::render::glyphinfo_iterator_t,
}

impl From<xcb::render::glyphinfo_iterator_t> for GlyphinfoIterator
{
  fn from(iter: xcb::render::glyphinfo_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlyphinfoIterator
{
  type Item = xcb::render::glyphinfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::glyphinfo_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphsetIterator
{
  iter: xcb::render::glyphset_iterator_t,
}

impl From<xcb::render::glyphset_iterator_t> for GlyphsetIterator
{
  fn from(iter: xcb::render::glyphset_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for GlyphsetIterator
{
  type Item = xcb::render::glyphset_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::glyphset_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct IndexvalueIterator
{
  iter: xcb::render::indexvalue_iterator_t,
}

impl From<xcb::render::indexvalue_iterator_t> for IndexvalueIterator
{
  fn from(iter: xcb::render::indexvalue_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for IndexvalueIterator
{
  type Item = xcb::render::indexvalue_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::indexvalue_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LinefixIterator
{
  iter: xcb::render::linefix_iterator_t,
}

impl From<xcb::render::linefix_iterator_t> for LinefixIterator
{
  fn from(iter: xcb::render::linefix_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LinefixIterator
{
  type Item = xcb::render::linefix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::linefix_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictdepthIterator
{
  iter: xcb::render::pictdepth_iterator_t,
}

impl From<xcb::render::pictdepth_iterator_t> for PictdepthIterator
{
  fn from(iter: xcb::render::pictdepth_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictdepthIterator
{
  type Item = xcb::render::pictdepth_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pictdepth_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictformatIterator
{
  iter: xcb::render::pictformat_iterator_t,
}

impl From<xcb::render::pictformat_iterator_t> for PictformatIterator
{
  fn from(iter: xcb::render::pictformat_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictformatIterator
{
  type Item = xcb::render::pictformat_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pictformat_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictforminfoIterator
{
  iter: xcb::render::pictforminfo_iterator_t,
}

impl From<xcb::render::pictforminfo_iterator_t> for PictforminfoIterator
{
  fn from(iter: xcb::render::pictforminfo_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictforminfoIterator
{
  type Item = xcb::render::pictforminfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pictforminfo_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictscreenIterator
{
  iter: xcb::render::pictscreen_iterator_t,
}

impl From<xcb::render::pictscreen_iterator_t> for PictscreenIterator
{
  fn from(iter: xcb::render::pictscreen_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictscreenIterator
{
  type Item = xcb::render::pictscreen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pictscreen_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictureIterator
{
  iter: xcb::render::picture_iterator_t,
}

impl From<xcb::render::picture_iterator_t> for PictureIterator
{
  fn from(iter: xcb::render::picture_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictureIterator
{
  type Item = xcb::render::picture_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::picture_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictvisualIterator
{
  iter: xcb::render::pictvisual_iterator_t,
}

impl From<xcb::render::pictvisual_iterator_t> for PictvisualIterator
{
  fn from(iter: xcb::render::pictvisual_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PictvisualIterator
{
  type Item = xcb::render::pictvisual_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pictvisual_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PointfixIterator
{
  iter: xcb::render::pointfix_iterator_t,
}

impl From<xcb::render::pointfix_iterator_t> for PointfixIterator
{
  fn from(iter: xcb::render::pointfix_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PointfixIterator
{
  type Item = xcb::render::pointfix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::pointfix_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SpanfixIterator
{
  iter: xcb::render::spanfix_iterator_t,
}

impl From<xcb::render::spanfix_iterator_t> for SpanfixIterator
{
  fn from(iter: xcb::render::spanfix_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SpanfixIterator
{
  type Item = xcb::render::spanfix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::spanfix_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TransformIterator
{
  iter: xcb::render::transform_iterator_t,
}

impl From<xcb::render::transform_iterator_t> for TransformIterator
{
  fn from(iter: xcb::render::transform_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TransformIterator
{
  type Item = xcb::render::transform_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::transform_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TrapIterator
{
  iter: xcb::render::trap_iterator_t,
}

impl From<xcb::render::trap_iterator_t> for TrapIterator
{
  fn from(iter: xcb::render::trap_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TrapIterator
{
  type Item = xcb::render::trap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::trap_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TrapezoidIterator
{
  iter: xcb::render::trapezoid_iterator_t,
}

impl From<xcb::render::trapezoid_iterator_t> for TrapezoidIterator
{
  fn from(iter: xcb::render::trapezoid_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TrapezoidIterator
{
  type Item = xcb::render::trapezoid_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::trapezoid_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TriangleIterator
{
  iter: xcb::render::triangle_iterator_t,
}

impl From<xcb::render::triangle_iterator_t> for TriangleIterator
{
  fn from(iter: xcb::render::triangle_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TriangleIterator
{
  type Item = xcb::render::triangle_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::render::triangle_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIdSpecIterator
{
  iter: xcb::res::client_id_spec_iterator_t,
}

impl From<xcb::res::client_id_spec_iterator_t> for ClientIdSpecIterator
{
  fn from(iter: xcb::res::client_id_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientIdSpecIterator
{
  type Item = xcb::res::client_id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::client_id_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIdValueIterator
{
  iter: xcb::res::client_id_value_iterator_t,
}

impl From<xcb::res::client_id_value_iterator_t> for ClientIdValueIterator
{
  fn from(iter: xcb::res::client_id_value_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientIdValueIterator
{
  type Item = xcb::res::client_id_value_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::client_id_value_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIterator
{
  iter: xcb::res::client_iterator_t,
}

impl From<xcb::res::client_iterator_t> for ClientIterator
{
  fn from(iter: xcb::res::client_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ClientIterator
{
  type Item = xcb::res::client_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::client_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceIdSpecIterator
{
  iter: xcb::res::resource_id_spec_iterator_t,
}

impl From<xcb::res::resource_id_spec_iterator_t> for ResourceIdSpecIterator
{
  fn from(iter: xcb::res::resource_id_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ResourceIdSpecIterator
{
  type Item = xcb::res::resource_id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::resource_id_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceSizeSpecIterator
{
  iter: xcb::res::resource_size_spec_iterator_t,
}

impl From<xcb::res::resource_size_spec_iterator_t> for ResourceSizeSpecIterator
{
  fn from(iter: xcb::res::resource_size_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ResourceSizeSpecIterator
{
  type Item = xcb::res::resource_size_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::resource_size_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceSizeValueIterator
{
  iter: xcb::res::resource_size_value_iterator_t,
}

impl From<xcb::res::resource_size_value_iterator_t> for ResourceSizeValueIterator
{
  fn from(iter: xcb::res::resource_size_value_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ResourceSizeValueIterator
{
  type Item = xcb::res::resource_size_value_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::resource_size_value_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TypeIterator
{
  iter: xcb::res::type_iterator_t,
}

impl From<xcb::res::type_iterator_t> for TypeIterator
{
  fn from(iter: xcb::res::type_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TypeIterator
{
  type Item = xcb::res::type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::res::type_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RgbIterator
{
  iter: xcb::rgb_iterator_t,
}

impl From<xcb::rgb_iterator_t> for RgbIterator
{
  fn from(iter: xcb::rgb_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RgbIterator
{
  type Item = xcb::rgb_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::rgb_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenIterator
{
  iter: xcb::screen_iterator_t,
}

impl From<xcb::screen_iterator_t> for ScreenIterator
{
  fn from(iter: xcb::screen_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ScreenIterator
{
  type Item = xcb::screen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::screen_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SegmentIterator
{
  iter: xcb::segment_iterator_t,
}

impl From<xcb::segment_iterator_t> for SegmentIterator
{
  fn from(iter: xcb::segment_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SegmentIterator
{
  type Item = xcb::segment_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::segment_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ListItemIterator
{
  iter: xcb::selinux::list_item_iterator_t,
}

impl From<xcb::selinux::list_item_iterator_t> for ListItemIterator
{
  fn from(iter: xcb::selinux::list_item_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ListItemIterator
{
  type Item = xcb::selinux::list_item_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::selinux::list_item_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupAuthenticateIterator
{
  iter: xcb::setup_authenticate_iterator_t,
}

impl From<xcb::setup_authenticate_iterator_t> for SetupAuthenticateIterator
{
  fn from(iter: xcb::setup_authenticate_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetupAuthenticateIterator
{
  type Item = xcb::setup_authenticate_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::setup_authenticate_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupFailedIterator
{
  iter: xcb::setup_failed_iterator_t,
}

impl From<xcb::setup_failed_iterator_t> for SetupFailedIterator
{
  fn from(iter: xcb::setup_failed_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetupFailedIterator
{
  type Item = xcb::setup_failed_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::setup_failed_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupIterator
{
  iter: xcb::setup_iterator_t,
}

impl From<xcb::setup_iterator_t> for SetupIterator
{
  fn from(iter: xcb::setup_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetupIterator
{
  type Item = xcb::setup_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::setup_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupRequestIterator
{
  iter: xcb::setup_request_iterator_t,
}

impl From<xcb::setup_request_iterator_t> for SetupRequestIterator
{
  fn from(iter: xcb::setup_request_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetupRequestIterator
{
  type Item = xcb::setup_request_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::setup_request_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KindIterator
{
  iter: xcb::shape::kind_iterator_t,
}

impl From<xcb::shape::kind_iterator_t> for KindIterator
{
  fn from(iter: xcb::shape::kind_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KindIterator
{
  type Item = xcb::shape::kind_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::shape::kind_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OpIterator
{
  iter: xcb::shape::op_iterator_t,
}

impl From<xcb::shape::op_iterator_t> for OpIterator
{
  fn from(iter: xcb::shape::op_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OpIterator
{
  type Item = xcb::shape::op_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::shape::op_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SegIterator
{
  iter: xcb::shm::seg_iterator_t,
}

impl From<xcb::shm::seg_iterator_t> for SegIterator
{
  fn from(iter: xcb::shm::seg_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SegIterator
{
  type Item = xcb::shm::seg_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::shm::seg_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct StrIterator
{
  iter: xcb::str_iterator_t,
}

impl From<xcb::str_iterator_t> for StrIterator
{
  fn from(iter: xcb::str_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for StrIterator
{
  type Item = xcb::str_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::str_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AlarmIterator
{
  iter: xcb::sync::alarm_iterator_t,
}

impl From<xcb::sync::alarm_iterator_t> for AlarmIterator
{
  fn from(iter: xcb::sync::alarm_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AlarmIterator
{
  type Item = xcb::sync::alarm_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::alarm_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CounterIterator
{
  iter: xcb::sync::counter_iterator_t,
}

impl From<xcb::sync::counter_iterator_t> for CounterIterator
{
  fn from(iter: xcb::sync::counter_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CounterIterator
{
  type Item = xcb::sync::counter_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::counter_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct FenceIterator
{
  iter: xcb::sync::fence_iterator_t,
}

impl From<xcb::sync::fence_iterator_t> for FenceIterator
{
  fn from(iter: xcb::sync::fence_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for FenceIterator
{
  type Item = xcb::sync::fence_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::fence_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct Int64Iterator
{
  iter: xcb::sync::int64_iterator_t,
}

impl From<xcb::sync::int64_iterator_t> for Int64Iterator
{
  fn from(iter: xcb::sync::int64_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for Int64Iterator
{
  type Item = xcb::sync::int64_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::int64_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SystemcounterIterator
{
  iter: xcb::sync::systemcounter_iterator_t,
}

impl From<xcb::sync::systemcounter_iterator_t> for SystemcounterIterator
{
  fn from(iter: xcb::sync::systemcounter_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SystemcounterIterator
{
  type Item = xcb::sync::systemcounter_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::systemcounter_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TriggerIterator
{
  iter: xcb::sync::trigger_iterator_t,
}

impl From<xcb::sync::trigger_iterator_t> for TriggerIterator
{
  fn from(iter: xcb::sync::trigger_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TriggerIterator
{
  type Item = xcb::sync::trigger_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::trigger_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct WaitconditionIterator
{
  iter: xcb::sync::waitcondition_iterator_t,
}

impl From<xcb::sync::waitcondition_iterator_t> for WaitconditionIterator
{
  fn from(iter: xcb::sync::waitcondition_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for WaitconditionIterator
{
  type Item = xcb::sync::waitcondition_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::sync::waitcondition_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TimecoordIterator
{
  iter: xcb::timecoord_iterator_t,
}

impl From<xcb::timecoord_iterator_t> for TimecoordIterator
{
  fn from(iter: xcb::timecoord_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TimecoordIterator
{
  type Item = xcb::timecoord_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::timecoord_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct TimestampIterator
{
  iter: xcb::timestamp_iterator_t,
}

impl From<xcb::timestamp_iterator_t> for TimestampIterator
{
  fn from(iter: xcb::timestamp_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for TimestampIterator
{
  type Item = xcb::timestamp_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::timestamp_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct VisualidIterator
{
  iter: xcb::visualid_iterator_t,
}

impl From<xcb::visualid_iterator_t> for VisualidIterator
{
  fn from(iter: xcb::visualid_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for VisualidIterator
{
  type Item = xcb::visualid_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::visualid_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct VisualtypeIterator
{
  iter: xcb::visualtype_iterator_t,
}

impl From<xcb::visualtype_iterator_t> for VisualtypeIterator
{
  fn from(iter: xcb::visualtype_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for VisualtypeIterator
{
  type Item = xcb::visualtype_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::visualtype_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct WindowIterator
{
  iter: xcb::window_iterator_t,
}

impl From<xcb::window_iterator_t> for WindowIterator
{
  fn from(iter: xcb::window_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for WindowIterator
{
  type Item = xcb::window_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::window_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PcontextIterator
{
  iter: xcb::xprint::pcontext_iterator_t,
}

impl From<xcb::xprint::pcontext_iterator_t> for PcontextIterator
{
  fn from(iter: xcb::xprint::pcontext_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PcontextIterator
{
  type Item = xcb::xprint::pcontext_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xprint::pcontext_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PrinterIterator
{
  iter: xcb::xprint::printer_iterator_t,
}

impl From<xcb::xprint::printer_iterator_t> for PrinterIterator
{
  fn from(iter: xcb::xprint::printer_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PrinterIterator
{
  type Item = xcb::xprint::printer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xprint::printer_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XprintString8Iterator
{
  iter: xcb::xprint::string8_iterator_t,
}

impl From<xcb::xprint::string8_iterator_t> for XprintString8Iterator
{
  fn from(iter: xcb::xprint::string8_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XprintString8Iterator
{
  type Item = xcb::xprint::string8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xprint::string8_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XevieEventIterator
{
  iter: xcb::xevie::event_iterator_t,
}

impl From<xcb::xevie::event_iterator_t> for XevieEventIterator
{
  fn from(iter: xcb::xevie::event_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XevieEventIterator
{
  type Item = xcb::xevie::event_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xevie::event_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DrmClipRectIterator
{
  iter: xcb::xf86dri::drm_clip_rect_iterator_t,
}

impl From<xcb::xf86dri::drm_clip_rect_iterator_t> for DrmClipRectIterator
{
  fn from(iter: xcb::xf86dri::drm_clip_rect_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DrmClipRectIterator
{
  type Item = xcb::xf86dri::drm_clip_rect_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xf86dri::drm_clip_rect_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BarrierIterator
{
  iter: xcb::xfixes::barrier_iterator_t,
}

impl From<xcb::xfixes::barrier_iterator_t> for BarrierIterator
{
  fn from(iter: xcb::xfixes::barrier_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BarrierIterator
{
  type Item = xcb::xfixes::barrier_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xfixes::barrier_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RegionIterator
{
  iter: xcb::xfixes::region_iterator_t,
}

impl From<xcb::xfixes::region_iterator_t> for RegionIterator
{
  fn from(iter: xcb::xfixes::region_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RegionIterator
{
  type Item = xcb::xfixes::region_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xfixes::region_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenInfoIterator
{
  iter: xcb::xinerama::screen_info_iterator_t,
}

impl From<xcb::xinerama::screen_info_iterator_t> for ScreenInfoIterator
{
  fn from(iter: xcb::xinerama::screen_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ScreenInfoIterator
{
  type Item = xcb::xinerama::screen_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xinerama::screen_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ActionIterator
{
  iter: xcb::xkb::action_iterator_t,
}

impl From<xcb::xkb::action_iterator_t> for ActionIterator
{
  fn from(iter: xcb::xkb::action_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ActionIterator
{
  type Item = xcb::xkb::action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::action_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BehaviorIterator
{
  iter: xcb::xkb::behavior_iterator_t,
}

impl From<xcb::xkb::behavior_iterator_t> for BehaviorIterator
{
  fn from(iter: xcb::xkb::behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BehaviorIterator
{
  type Item = xcb::xkb::behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellClassSpecIterator
{
  iter: xcb::xkb::bell_class_spec_iterator_t,
}

impl From<xcb::xkb::bell_class_spec_iterator_t> for BellClassSpecIterator
{
  fn from(iter: xcb::xkb::bell_class_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for BellClassSpecIterator
{
  type Item = xcb::xkb::bell_class_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::bell_class_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CommonBehaviorIterator
{
  iter: xcb::xkb::common_behavior_iterator_t,
}

impl From<xcb::xkb::common_behavior_iterator_t> for CommonBehaviorIterator
{
  fn from(iter: xcb::xkb::common_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CommonBehaviorIterator
{
  type Item = xcb::xkb::common_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::common_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct CountedString16Iterator
{
  iter: xcb::xkb::counted_string_16_iterator_t,
}

impl From<xcb::xkb::counted_string_16_iterator_t> for CountedString16Iterator
{
  fn from(iter: xcb::xkb::counted_string_16_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for CountedString16Iterator
{
  type Item = xcb::xkb::counted_string_16_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::counted_string_16_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DefaultBehaviorIterator
{
  iter: xcb::xkb::default_behavior_iterator_t,
}

impl From<xcb::xkb::default_behavior_iterator_t> for DefaultBehaviorIterator
{
  fn from(iter: xcb::xkb::default_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DefaultBehaviorIterator
{
  type Item = xcb::xkb::default_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::default_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceLedInfoIterator
{
  iter: xcb::xkb::device_led_info_iterator_t,
}

impl From<xcb::xkb::device_led_info_iterator_t> for DeviceLedInfoIterator
{
  fn from(iter: xcb::xkb::device_led_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceLedInfoIterator
{
  type Item = xcb::xkb::device_led_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::device_led_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceSpecIterator
{
  iter: xcb::xkb::device_spec_iterator_t,
}

impl From<xcb::xkb::device_spec_iterator_t> for DeviceSpecIterator
{
  fn from(iter: xcb::xkb::device_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for DeviceSpecIterator
{
  type Item = xcb::xkb::device_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::device_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct IdSpecIterator
{
  iter: xcb::xkb::id_spec_iterator_t,
}

impl From<xcb::xkb::id_spec_iterator_t> for IdSpecIterator
{
  fn from(iter: xcb::xkb::id_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for IdSpecIterator
{
  type Item = xcb::xkb::id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::id_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct IndicatorMapIterator
{
  iter: xcb::xkb::indicator_map_iterator_t,
}

impl From<xcb::xkb::indicator_map_iterator_t> for IndicatorMapIterator
{
  fn from(iter: xcb::xkb::indicator_map_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for IndicatorMapIterator
{
  type Item = xcb::xkb::indicator_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::indicator_map_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyAliasIterator
{
  iter: xcb::xkb::key_alias_iterator_t,
}

impl From<xcb::xkb::key_alias_iterator_t> for KeyAliasIterator
{
  fn from(iter: xcb::xkb::key_alias_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyAliasIterator
{
  type Item = xcb::xkb::key_alias_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_alias_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyIterator
{
  iter: xcb::xkb::key_iterator_t,
}

impl From<xcb::xkb::key_iterator_t> for KeyIterator
{
  fn from(iter: xcb::xkb::key_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyIterator
{
  type Item = xcb::xkb::key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyModMapIterator
{
  iter: xcb::xkb::key_mod_map_iterator_t,
}

impl From<xcb::xkb::key_mod_map_iterator_t> for KeyModMapIterator
{
  fn from(iter: xcb::xkb::key_mod_map_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyModMapIterator
{
  type Item = xcb::xkb::key_mod_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_mod_map_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyNameIterator
{
  iter: xcb::xkb::key_name_iterator_t,
}

impl From<xcb::xkb::key_name_iterator_t> for KeyNameIterator
{
  fn from(iter: xcb::xkb::key_name_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyNameIterator
{
  type Item = xcb::xkb::key_name_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_name_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeySymMapIterator
{
  iter: xcb::xkb::key_sym_map_iterator_t,
}

impl From<xcb::xkb::key_sym_map_iterator_t> for KeySymMapIterator
{
  fn from(iter: xcb::xkb::key_sym_map_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeySymMapIterator
{
  type Item = xcb::xkb::key_sym_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_sym_map_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyTypeIterator
{
  iter: xcb::xkb::key_type_iterator_t,
}

impl From<xcb::xkb::key_type_iterator_t> for KeyTypeIterator
{
  fn from(iter: xcb::xkb::key_type_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyTypeIterator
{
  type Item = xcb::xkb::key_type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_type_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyVModMapIterator
{
  iter: xcb::xkb::key_v_mod_map_iterator_t,
}

impl From<xcb::xkb::key_v_mod_map_iterator_t> for KeyVModMapIterator
{
  fn from(iter: xcb::xkb::key_v_mod_map_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KeyVModMapIterator
{
  type Item = xcb::xkb::key_v_mod_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::key_v_mod_map_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KtMapEntryIterator
{
  iter: xcb::xkb::kt_map_entry_iterator_t,
}

impl From<xcb::xkb::kt_map_entry_iterator_t> for KtMapEntryIterator
{
  fn from(iter: xcb::xkb::kt_map_entry_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KtMapEntryIterator
{
  type Item = xcb::xkb::kt_map_entry_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::kt_map_entry_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct KtSetMapEntryIterator
{
  iter: xcb::xkb::kt_set_map_entry_iterator_t,
}

impl From<xcb::xkb::kt_set_map_entry_iterator_t> for KtSetMapEntryIterator
{
  fn from(iter: xcb::xkb::kt_set_map_entry_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for KtSetMapEntryIterator
{
  type Item = xcb::xkb::kt_set_map_entry_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::kt_set_map_entry_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedClassSpecIterator
{
  iter: xcb::xkb::led_class_spec_iterator_t,
}

impl From<xcb::xkb::led_class_spec_iterator_t> for LedClassSpecIterator
{
  fn from(iter: xcb::xkb::led_class_spec_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LedClassSpecIterator
{
  type Item = xcb::xkb::led_class_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::led_class_spec_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ListingIterator
{
  iter: xcb::xkb::listing_iterator_t,
}

impl From<xcb::xkb::listing_iterator_t> for ListingIterator
{
  fn from(iter: xcb::xkb::listing_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ListingIterator
{
  type Item = xcb::xkb::listing_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::listing_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct LockBehaviorIterator
{
  iter: xcb::xkb::lock_behavior_iterator_t,
}

impl From<xcb::xkb::lock_behavior_iterator_t> for LockBehaviorIterator
{
  fn from(iter: xcb::xkb::lock_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for LockBehaviorIterator
{
  type Item = xcb::xkb::lock_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::lock_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModDefIterator
{
  iter: xcb::xkb::mod_def_iterator_t,
}

impl From<xcb::xkb::mod_def_iterator_t> for ModDefIterator
{
  fn from(iter: xcb::xkb::mod_def_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ModDefIterator
{
  type Item = xcb::xkb::mod_def_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::mod_def_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutlineIterator
{
  iter: xcb::xkb::outline_iterator_t,
}

impl From<xcb::xkb::outline_iterator_t> for OutlineIterator
{
  fn from(iter: xcb::xkb::outline_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OutlineIterator
{
  type Item = xcb::xkb::outline_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::outline_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayBehaviorIterator
{
  iter: xcb::xkb::overlay_behavior_iterator_t,
}

impl From<xcb::xkb::overlay_behavior_iterator_t> for OverlayBehaviorIterator
{
  fn from(iter: xcb::xkb::overlay_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OverlayBehaviorIterator
{
  type Item = xcb::xkb::overlay_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::overlay_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayIterator
{
  iter: xcb::xkb::overlay_iterator_t,
}

impl From<xcb::xkb::overlay_iterator_t> for OverlayIterator
{
  fn from(iter: xcb::xkb::overlay_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OverlayIterator
{
  type Item = xcb::xkb::overlay_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::overlay_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayKeyIterator
{
  iter: xcb::xkb::overlay_key_iterator_t,
}

impl From<xcb::xkb::overlay_key_iterator_t> for OverlayKeyIterator
{
  fn from(iter: xcb::xkb::overlay_key_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OverlayKeyIterator
{
  type Item = xcb::xkb::overlay_key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::overlay_key_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayRowIterator
{
  iter: xcb::xkb::overlay_row_iterator_t,
}

impl From<xcb::xkb::overlay_row_iterator_t> for OverlayRowIterator
{
  fn from(iter: xcb::xkb::overlay_row_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for OverlayRowIterator
{
  type Item = xcb::xkb::overlay_row_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::overlay_row_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentLockBehaviorIterator
{
  iter: xcb::xkb::permament_lock_behavior_iterator_t,
}

impl From<xcb::xkb::permament_lock_behavior_iterator_t> for PermamentLockBehaviorIterator
{
  fn from(iter: xcb::xkb::permament_lock_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PermamentLockBehaviorIterator
{
  type Item = xcb::xkb::permament_lock_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::permament_lock_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentOverlayBehaviorIterator
{
  iter: xcb::xkb::permament_overlay_behavior_iterator_t,
}

impl From<xcb::xkb::permament_overlay_behavior_iterator_t> for PermamentOverlayBehaviorIterator
{
  fn from(iter: xcb::xkb::permament_overlay_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PermamentOverlayBehaviorIterator
{
  type Item = xcb::xkb::permament_overlay_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::permament_overlay_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentRadioGroupBehaviorIterator
{
  iter: xcb::xkb::permament_radio_group_behavior_iterator_t,
}

impl From<xcb::xkb::permament_radio_group_behavior_iterator_t>
  for PermamentRadioGroupBehaviorIterator
{
  fn from(iter: xcb::xkb::permament_radio_group_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PermamentRadioGroupBehaviorIterator
{
  type Item = xcb::xkb::permament_radio_group_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::permament_radio_group_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RadioGroupBehaviorIterator
{
  iter: xcb::xkb::radio_group_behavior_iterator_t,
}

impl From<xcb::xkb::radio_group_behavior_iterator_t> for RadioGroupBehaviorIterator
{
  fn from(iter: xcb::xkb::radio_group_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RadioGroupBehaviorIterator
{
  type Item = xcb::xkb::radio_group_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::radio_group_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RowIterator
{
  iter: xcb::xkb::row_iterator_t,
}

impl From<xcb::xkb::row_iterator_t> for RowIterator
{
  fn from(iter: xcb::xkb::row_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RowIterator
{
  type Item = xcb::xkb::row_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::row_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaActionMessageIterator
{
  iter: xcb::xkb::sa_action_message_iterator_t,
}

impl From<xcb::xkb::sa_action_message_iterator_t> for SaActionMessageIterator
{
  fn from(iter: xcb::xkb::sa_action_message_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaActionMessageIterator
{
  type Item = xcb::xkb::sa_action_message_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_action_message_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaDeviceBtnIterator
{
  iter: xcb::xkb::sa_device_btn_iterator_t,
}

impl From<xcb::xkb::sa_device_btn_iterator_t> for SaDeviceBtnIterator
{
  fn from(iter: xcb::xkb::sa_device_btn_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaDeviceBtnIterator
{
  type Item = xcb::xkb::sa_device_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_device_btn_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaDeviceValuatorIterator
{
  iter: xcb::xkb::sa_device_valuator_iterator_t,
}

impl From<xcb::xkb::sa_device_valuator_iterator_t> for SaDeviceValuatorIterator
{
  fn from(iter: xcb::xkb::sa_device_valuator_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaDeviceValuatorIterator
{
  type Item = xcb::xkb::sa_device_valuator_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_device_valuator_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaIsoLockIterator
{
  iter: xcb::xkb::sa_iso_lock_iterator_t,
}

impl From<xcb::xkb::sa_iso_lock_iterator_t> for SaIsoLockIterator
{
  fn from(iter: xcb::xkb::sa_iso_lock_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaIsoLockIterator
{
  type Item = xcb::xkb::sa_iso_lock_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_iso_lock_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLatchGroupIterator
{
  iter: xcb::xkb::sa_latch_group_iterator_t,
}

impl From<xcb::xkb::sa_latch_group_iterator_t> for SaLatchGroupIterator
{
  fn from(iter: xcb::xkb::sa_latch_group_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLatchGroupIterator
{
  type Item = xcb::xkb::sa_latch_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_latch_group_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLatchModsIterator
{
  iter: xcb::xkb::sa_latch_mods_iterator_t,
}

impl From<xcb::xkb::sa_latch_mods_iterator_t> for SaLatchModsIterator
{
  fn from(iter: xcb::xkb::sa_latch_mods_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLatchModsIterator
{
  type Item = xcb::xkb::sa_latch_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_latch_mods_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockControlsIterator
{
  iter: xcb::xkb::sa_lock_controls_iterator_t,
}

impl From<xcb::xkb::sa_lock_controls_iterator_t> for SaLockControlsIterator
{
  fn from(iter: xcb::xkb::sa_lock_controls_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLockControlsIterator
{
  type Item = xcb::xkb::sa_lock_controls_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_lock_controls_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockDeviceBtnIterator
{
  iter: xcb::xkb::sa_lock_device_btn_iterator_t,
}

impl From<xcb::xkb::sa_lock_device_btn_iterator_t> for SaLockDeviceBtnIterator
{
  fn from(iter: xcb::xkb::sa_lock_device_btn_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLockDeviceBtnIterator
{
  type Item = xcb::xkb::sa_lock_device_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_lock_device_btn_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockGroupIterator
{
  iter: xcb::xkb::sa_lock_group_iterator_t,
}

impl From<xcb::xkb::sa_lock_group_iterator_t> for SaLockGroupIterator
{
  fn from(iter: xcb::xkb::sa_lock_group_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLockGroupIterator
{
  type Item = xcb::xkb::sa_lock_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_lock_group_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockModsIterator
{
  iter: xcb::xkb::sa_lock_mods_iterator_t,
}

impl From<xcb::xkb::sa_lock_mods_iterator_t> for SaLockModsIterator
{
  fn from(iter: xcb::xkb::sa_lock_mods_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLockModsIterator
{
  type Item = xcb::xkb::sa_lock_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_lock_mods_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockPtrBtnIterator
{
  iter: xcb::xkb::sa_lock_ptr_btn_iterator_t,
}

impl From<xcb::xkb::sa_lock_ptr_btn_iterator_t> for SaLockPtrBtnIterator
{
  fn from(iter: xcb::xkb::sa_lock_ptr_btn_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaLockPtrBtnIterator
{
  type Item = xcb::xkb::sa_lock_ptr_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_lock_ptr_btn_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaMovePtrIterator
{
  iter: xcb::xkb::sa_move_ptr_iterator_t,
}

impl From<xcb::xkb::sa_move_ptr_iterator_t> for SaMovePtrIterator
{
  fn from(iter: xcb::xkb::sa_move_ptr_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaMovePtrIterator
{
  type Item = xcb::xkb::sa_move_ptr_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_move_ptr_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaNoActionIterator
{
  iter: xcb::xkb::sa_no_action_iterator_t,
}

impl From<xcb::xkb::sa_no_action_iterator_t> for SaNoActionIterator
{
  fn from(iter: xcb::xkb::sa_no_action_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaNoActionIterator
{
  type Item = xcb::xkb::sa_no_action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_no_action_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaPtrBtnIterator
{
  iter: xcb::xkb::sa_ptr_btn_iterator_t,
}

impl From<xcb::xkb::sa_ptr_btn_iterator_t> for SaPtrBtnIterator
{
  fn from(iter: xcb::xkb::sa_ptr_btn_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaPtrBtnIterator
{
  type Item = xcb::xkb::sa_ptr_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_ptr_btn_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaRedirectKeyIterator
{
  iter: xcb::xkb::sa_redirect_key_iterator_t,
}

impl From<xcb::xkb::sa_redirect_key_iterator_t> for SaRedirectKeyIterator
{
  fn from(iter: xcb::xkb::sa_redirect_key_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaRedirectKeyIterator
{
  type Item = xcb::xkb::sa_redirect_key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_redirect_key_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetControlsIterator
{
  iter: xcb::xkb::sa_set_controls_iterator_t,
}

impl From<xcb::xkb::sa_set_controls_iterator_t> for SaSetControlsIterator
{
  fn from(iter: xcb::xkb::sa_set_controls_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaSetControlsIterator
{
  type Item = xcb::xkb::sa_set_controls_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_set_controls_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetGroupIterator
{
  iter: xcb::xkb::sa_set_group_iterator_t,
}

impl From<xcb::xkb::sa_set_group_iterator_t> for SaSetGroupIterator
{
  fn from(iter: xcb::xkb::sa_set_group_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaSetGroupIterator
{
  type Item = xcb::xkb::sa_set_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_set_group_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetModsIterator
{
  iter: xcb::xkb::sa_set_mods_iterator_t,
}

impl From<xcb::xkb::sa_set_mods_iterator_t> for SaSetModsIterator
{
  fn from(iter: xcb::xkb::sa_set_mods_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaSetModsIterator
{
  type Item = xcb::xkb::sa_set_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_set_mods_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetPtrDfltIterator
{
  iter: xcb::xkb::sa_set_ptr_dflt_iterator_t,
}

impl From<xcb::xkb::sa_set_ptr_dflt_iterator_t> for SaSetPtrDfltIterator
{
  fn from(iter: xcb::xkb::sa_set_ptr_dflt_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaSetPtrDfltIterator
{
  type Item = xcb::xkb::sa_set_ptr_dflt_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_set_ptr_dflt_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSwitchScreenIterator
{
  iter: xcb::xkb::sa_switch_screen_iterator_t,
}

impl From<xcb::xkb::sa_switch_screen_iterator_t> for SaSwitchScreenIterator
{
  fn from(iter: xcb::xkb::sa_switch_screen_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaSwitchScreenIterator
{
  type Item = xcb::xkb::sa_switch_screen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_switch_screen_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaTerminateIterator
{
  iter: xcb::xkb::sa_terminate_iterator_t,
}

impl From<xcb::xkb::sa_terminate_iterator_t> for SaTerminateIterator
{
  fn from(iter: xcb::xkb::sa_terminate_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SaTerminateIterator
{
  type Item = xcb::xkb::sa_terminate_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sa_terminate_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetBehaviorIterator
{
  iter: xcb::xkb::set_behavior_iterator_t,
}

impl From<xcb::xkb::set_behavior_iterator_t> for SetBehaviorIterator
{
  fn from(iter: xcb::xkb::set_behavior_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetBehaviorIterator
{
  type Item = xcb::xkb::set_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::set_behavior_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetExplicitIterator
{
  iter: xcb::xkb::set_explicit_iterator_t,
}

impl From<xcb::xkb::set_explicit_iterator_t> for SetExplicitIterator
{
  fn from(iter: xcb::xkb::set_explicit_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetExplicitIterator
{
  type Item = xcb::xkb::set_explicit_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::set_explicit_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetKeyTypeIterator
{
  iter: xcb::xkb::set_key_type_iterator_t,
}

impl From<xcb::xkb::set_key_type_iterator_t> for SetKeyTypeIterator
{
  fn from(iter: xcb::xkb::set_key_type_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SetKeyTypeIterator
{
  type Item = xcb::xkb::set_key_type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::set_key_type_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ShapeIterator
{
  iter: xcb::xkb::shape_iterator_t,
}

impl From<xcb::xkb::shape_iterator_t> for ShapeIterator
{
  fn from(iter: xcb::xkb::shape_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ShapeIterator
{
  type Item = xcb::xkb::shape_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::shape_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SiActionIterator
{
  iter: xcb::xkb::si_action_iterator_t,
}

impl From<xcb::xkb::si_action_iterator_t> for SiActionIterator
{
  fn from(iter: xcb::xkb::si_action_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SiActionIterator
{
  type Item = xcb::xkb::si_action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::si_action_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XkbString8Iterator
{
  iter: xcb::xkb::string8_iterator_t,
}

impl From<xcb::xkb::string8_iterator_t> for XkbString8Iterator
{
  fn from(iter: xcb::xkb::string8_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XkbString8Iterator
{
  type Item = xcb::xkb::string8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::string8_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SymInterpretIterator
{
  iter: xcb::xkb::sym_interpret_iterator_t,
}

impl From<xcb::xkb::sym_interpret_iterator_t> for SymInterpretIterator
{
  fn from(iter: xcb::xkb::sym_interpret_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SymInterpretIterator
{
  type Item = xcb::xkb::sym_interpret_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xkb::sym_interpret_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AdaptorInfoIterator
{
  iter: xcb::xv::adaptor_info_iterator_t,
}

impl From<xcb::xv::adaptor_info_iterator_t> for AdaptorInfoIterator
{
  fn from(iter: xcb::xv::adaptor_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AdaptorInfoIterator
{
  type Item = xcb::xv::adaptor_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::adaptor_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttributeInfoIterator
{
  iter: xcb::xv::attribute_info_iterator_t,
}

impl From<xcb::xv::attribute_info_iterator_t> for AttributeInfoIterator
{
  fn from(iter: xcb::xv::attribute_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for AttributeInfoIterator
{
  type Item = xcb::xv::attribute_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::attribute_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EncodingInfoIterator
{
  iter: xcb::xv::encoding_info_iterator_t,
}

impl From<xcb::xv::encoding_info_iterator_t> for EncodingInfoIterator
{
  fn from(iter: xcb::xv::encoding_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EncodingInfoIterator
{
  type Item = xcb::xv::encoding_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::encoding_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct EncodingIterator
{
  iter: xcb::xv::encoding_iterator_t,
}

impl From<xcb::xv::encoding_iterator_t> for EncodingIterator
{
  fn from(iter: xcb::xv::encoding_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for EncodingIterator
{
  type Item = xcb::xv::encoding_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::encoding_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XvFormatIterator
{
  iter: xcb::xv::format_iterator_t,
}

impl From<xcb::xv::format_iterator_t> for XvFormatIterator
{
  fn from(iter: xcb::xv::format_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XvFormatIterator
{
  type Item = xcb::xv::format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::format_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ImageFormatInfoIterator
{
  iter: xcb::xv::image_format_info_iterator_t,
}

impl From<xcb::xv::image_format_info_iterator_t> for ImageFormatInfoIterator
{
  fn from(iter: xcb::xv::image_format_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ImageFormatInfoIterator
{
  type Item = xcb::xv::image_format_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::image_format_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct ImageIterator
{
  iter: xcb::xv::image_iterator_t,
}

impl From<xcb::xv::image_iterator_t> for ImageIterator
{
  fn from(iter: xcb::xv::image_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for ImageIterator
{
  type Item = xcb::xv::image_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::image_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct PortIterator
{
  iter: xcb::xv::port_iterator_t,
}

impl From<xcb::xv::port_iterator_t> for PortIterator
{
  fn from(iter: xcb::xv::port_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for PortIterator
{
  type Item = xcb::xv::port_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::port_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct RationalIterator
{
  iter: xcb::xv::rational_iterator_t,
}

impl From<xcb::xv::rational_iterator_t> for RationalIterator
{
  fn from(iter: xcb::xv::rational_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for RationalIterator
{
  type Item = xcb::xv::rational_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xv::rational_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct XvmcContextIterator
{
  iter: xcb::xvmc::context_iterator_t,
}

impl From<xcb::xvmc::context_iterator_t> for XvmcContextIterator
{
  fn from(iter: xcb::xvmc::context_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for XvmcContextIterator
{
  type Item = xcb::xvmc::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xvmc::context_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SubpictureIterator
{
  iter: xcb::xvmc::subpicture_iterator_t,
}

impl From<xcb::xvmc::subpicture_iterator_t> for SubpictureIterator
{
  fn from(iter: xcb::xvmc::subpicture_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SubpictureIterator
{
  type Item = xcb::xvmc::subpicture_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xvmc::subpicture_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SurfaceInfoIterator
{
  iter: xcb::xvmc::surface_info_iterator_t,
}

impl From<xcb::xvmc::surface_info_iterator_t> for SurfaceInfoIterator
{
  fn from(iter: xcb::xvmc::surface_info_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SurfaceInfoIterator
{
  type Item = xcb::xvmc::surface_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xvmc::surface_info_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}

pub struct SurfaceIterator
{
  iter: xcb::xvmc::surface_iterator_t,
}

impl From<xcb::xvmc::surface_iterator_t> for SurfaceIterator
{
  fn from(iter: xcb::xvmc::surface_iterator_t) -> Self
  {
    Self { iter }
  }
}

impl Iterator for SurfaceIterator
{
  type Item = xcb::xvmc::surface_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.iter.rem == 0 {
      None
    } else {
      let data = self.iter.data;
      unsafe { xcb::xvmc::surface_next(&mut self.iter) };
      Some(unsafe { *data })
    }
  }
}
