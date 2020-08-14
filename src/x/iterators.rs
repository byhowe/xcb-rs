use crate::xcb;

pub struct ArcIterator
{
  ptr: *mut xcb::arc_iterator_t,
}

impl Iterator for ArcIterator
{
  type Item = xcb::arc_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::arc_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AtomIterator
{
  ptr: *mut xcb::atom_iterator_t,
}

impl Iterator for AtomIterator
{
  type Item = xcb::atom_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::atom_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Bool32Iterator
{
  ptr: *mut xcb::bool32_iterator_t,
}

impl Iterator for Bool32Iterator
{
  type Item = xcb::bool32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::bool32_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonIterator
{
  ptr: *mut xcb::button_iterator_t,
}

impl Iterator for ButtonIterator
{
  type Item = xcb::button_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::button_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Char2bIterator
{
  ptr: *mut xcb::char2b_iterator_t,
}

impl Iterator for Char2bIterator
{
  type Item = xcb::char2b_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::char2b_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CharinfoIterator
{
  ptr: *mut xcb::charinfo_iterator_t,
}

impl Iterator for CharinfoIterator
{
  type Item = xcb::charinfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::charinfo_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientMessageDataIterator
{
  ptr: *mut xcb::client_message_data_iterator_t,
}

impl Iterator for ClientMessageDataIterator
{
  type Item = xcb::client_message_data_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::client_message_data_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColoritemIterator
{
  ptr: *mut xcb::coloritem_iterator_t,
}

impl Iterator for ColoritemIterator
{
  type Item = xcb::coloritem_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::coloritem_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColormapIterator
{
  ptr: *mut xcb::colormap_iterator_t,
}

impl Iterator for ColormapIterator
{
  type Item = xcb::colormap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::colormap_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CursorIterator
{
  ptr: *mut xcb::cursor_iterator_t,
}

impl Iterator for CursorIterator
{
  type Item = xcb::cursor_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::cursor_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DamageIterator
{
  ptr: *mut xcb::damage::damage_iterator_t,
}

impl Iterator for DamageIterator
{
  type Item = xcb::damage::damage_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::damage::damage_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DepthIterator
{
  ptr: *mut xcb::depth_iterator_t,
}

impl Iterator for DepthIterator
{
  type Item = xcb::depth_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::depth_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DrawableIterator
{
  ptr: *mut xcb::drawable_iterator_t,
}

impl Iterator for DrawableIterator
{
  type Item = xcb::drawable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::drawable_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttachFormatIterator
{
  ptr: *mut xcb::dri2::attach_format_iterator_t,
}

impl Iterator for AttachFormatIterator
{
  type Item = xcb::dri2::attach_format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::dri2::attach_format_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Dri2BufferIterator
{
  ptr: *mut xcb::dri2::dri2_buffer_iterator_t,
}

impl Iterator for Dri2BufferIterator
{
  type Item = xcb::dri2::dri2_buffer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::dri2::dri2_buffer_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct WmIconIterator
{
  ptr: *mut xcb::ewmh::wm_icon_iterator_t,
}

impl Iterator for WmIconIterator
{
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::ewmh::get_wm_icon_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontIterator
{
  ptr: *mut xcb::font_iterator_t,
}

impl Iterator for FontIterator
{
  type Item = xcb::font_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::font_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontableIterator
{
  ptr: *mut xcb::fontable_iterator_t,
}

impl Iterator for FontableIterator
{
  type Item = xcb::fontable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::fontable_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FontpropIterator
{
  ptr: *mut xcb::fontprop_iterator_t,
}

impl Iterator for FontpropIterator
{
  type Item = xcb::fontprop_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::fontprop_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FormatIterator
{
  ptr: *mut xcb::format_iterator_t,
}

impl Iterator for FormatIterator
{
  type Item = xcb::format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::format_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GcontextIterator
{
  ptr: *mut xcb::gcontext_iterator_t,
}

impl Iterator for GcontextIterator
{
  type Item = xcb::gcontext_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::gcontext_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxBool32Iterator
{
  ptr: *mut xcb::glx::bool32_iterator_t,
}

impl Iterator for GlxBool32Iterator
{
  type Item = xcb::glx::bool32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::bool32_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxContextIterator
{
  ptr: *mut xcb::glx::context_iterator_t,
}

impl Iterator for GlxContextIterator
{
  type Item = xcb::glx::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::context_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ContextTagIterator
{
  ptr: *mut xcb::glx::context_tag_iterator_t,
}

impl Iterator for ContextTagIterator
{
  type Item = xcb::glx::context_tag_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::context_tag_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxDrawableIterator
{
  ptr: *mut xcb::glx::drawable_iterator_t,
}

impl Iterator for GlxDrawableIterator
{
  type Item = xcb::glx::drawable_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::drawable_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FbconfigIterator
{
  ptr: *mut xcb::glx::fbconfig_iterator_t,
}

impl Iterator for FbconfigIterator
{
  type Item = xcb::glx::fbconfig_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::fbconfig_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Float32Iterator
{
  ptr: *mut xcb::glx::float32_iterator_t,
}

impl Iterator for Float32Iterator
{
  type Item = xcb::glx::float32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::float32_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Float64Iterator
{
  ptr: *mut xcb::glx::float64_iterator_t,
}

impl Iterator for Float64Iterator
{
  type Item = xcb::glx::float64_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::float64_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PbufferIterator
{
  ptr: *mut xcb::glx::pbuffer_iterator_t,
}

impl Iterator for PbufferIterator
{
  type Item = xcb::glx::pbuffer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::pbuffer_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxPixmapIterator
{
  ptr: *mut xcb::glx::pixmap_iterator_t,
}

impl Iterator for GlxPixmapIterator
{
  type Item = xcb::glx::pixmap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::pixmap_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlxWindowIterator
{
  ptr: *mut xcb::glx::window_iterator_t,
}

impl Iterator for GlxWindowIterator
{
  type Item = xcb::glx::window_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::glx::window_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct HostIterator
{
  ptr: *mut xcb::host_iterator_t,
}

impl Iterator for HostIterator
{
  type Item = xcb::host_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::host_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AddMasterIterator
{
  ptr: *mut xcb::input::add_master_iterator_t,
}

impl Iterator for AddMasterIterator
{
  type Item = xcb::input::add_master_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::add_master_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttachSlaveIterator
{
  ptr: *mut xcb::input::attach_slave_iterator_t,
}

impl Iterator for AttachSlaveIterator
{
  type Item = xcb::input::attach_slave_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::attach_slave_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AxisInfoIterator
{
  ptr: *mut xcb::input::axis_info_iterator_t,
}

impl Iterator for AxisInfoIterator
{
  type Item = xcb::input::axis_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::axis_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BarrierReleasePointerInfoIterator
{
  ptr: *mut xcb::input::barrier_release_pointer_info_iterator_t,
}

impl Iterator for BarrierReleasePointerInfoIterator
{
  type Item = xcb::input::barrier_release_pointer_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::barrier_release_pointer_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellFeedbackCtlIterator
{
  ptr: *mut xcb::input::bell_feedback_ctl_iterator_t,
}

impl Iterator for BellFeedbackCtlIterator
{
  type Item = xcb::input::bell_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::bell_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellFeedbackStateIterator
{
  ptr: *mut xcb::input::bell_feedback_state_iterator_t,
}

impl Iterator for BellFeedbackStateIterator
{
  type Item = xcb::input::bell_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::bell_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonClassIterator
{
  ptr: *mut xcb::input::button_class_iterator_t,
}

impl Iterator for ButtonClassIterator
{
  type Item = xcb::input::button_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::button_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonInfoIterator
{
  ptr: *mut xcb::input::button_info_iterator_t,
}

impl Iterator for ButtonInfoIterator
{
  type Item = xcb::input::button_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::button_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ButtonStateIterator
{
  ptr: *mut xcb::input::button_state_iterator_t,
}

impl Iterator for ButtonStateIterator
{
  type Item = xcb::input::button_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::button_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DetachSlaveIterator
{
  ptr: *mut xcb::input::detach_slave_iterator_t,
}

impl Iterator for DetachSlaveIterator
{
  type Item = xcb::input::detach_slave_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::detach_slave_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsAreaCtrlIterator
{
  ptr: *mut xcb::input::device_abs_area_ctrl_iterator_t,
}

impl Iterator for DeviceAbsAreaCtrlIterator
{
  type Item = xcb::input::device_abs_area_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_abs_area_ctrl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsAreaStateIterator
{
  ptr: *mut xcb::input::device_abs_area_state_iterator_t,
}

impl Iterator for DeviceAbsAreaStateIterator
{
  type Item = xcb::input::device_abs_area_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_abs_area_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsCalibCtlIterator
{
  ptr: *mut xcb::input::device_abs_calib_ctl_iterator_t,
}

impl Iterator for DeviceAbsCalibCtlIterator
{
  type Item = xcb::input::device_abs_calib_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_abs_calib_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceAbsCalibStateIterator
{
  ptr: *mut xcb::input::device_abs_calib_state_iterator_t,
}

impl Iterator for DeviceAbsCalibStateIterator
{
  type Item = xcb::input::device_abs_calib_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_abs_calib_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceClassIterator
{
  ptr: *mut xcb::input::device_class_iterator_t,
}

impl Iterator for DeviceClassIterator
{
  type Item = xcb::input::device_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCoreCtrlIterator
{
  ptr: *mut xcb::input::device_core_ctrl_iterator_t,
}

impl Iterator for DeviceCoreCtrlIterator
{
  type Item = xcb::input::device_core_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_core_ctrl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCoreStateIterator
{
  ptr: *mut xcb::input::device_core_state_iterator_t,
}

impl Iterator for DeviceCoreStateIterator
{
  type Item = xcb::input::device_core_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_core_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceCtlIterator
{
  ptr: *mut xcb::input::device_ctl_iterator_t,
}

impl Iterator for DeviceCtlIterator
{
  type Item = xcb::input::device_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceEnableCtrlIterator
{
  ptr: *mut xcb::input::device_enable_ctrl_iterator_t,
}

impl Iterator for DeviceEnableCtrlIterator
{
  type Item = xcb::input::device_enable_ctrl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_enable_ctrl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceEnableStateIterator
{
  ptr: *mut xcb::input::device_enable_state_iterator_t,
}

impl Iterator for DeviceEnableStateIterator
{
  type Item = xcb::input::device_enable_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_enable_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceIdIterator
{
  ptr: *mut xcb::input::device_id_iterator_t,
}

impl Iterator for DeviceIdIterator
{
  type Item = xcb::input::device_id_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_id_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceInfoIterator
{
  ptr: *mut xcb::input::device_info_iterator_t,
}

impl Iterator for DeviceInfoIterator
{
  type Item = xcb::input::device_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceNameIterator
{
  ptr: *mut xcb::input::device_name_iterator_t,
}

impl Iterator for DeviceNameIterator
{
  type Item = xcb::input::device_name_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_name_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceResolutionCtlIterator
{
  ptr: *mut xcb::input::device_resolution_ctl_iterator_t,
}

impl Iterator for DeviceResolutionCtlIterator
{
  type Item = xcb::input::device_resolution_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_resolution_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceResolutionStateIterator
{
  ptr: *mut xcb::input::device_resolution_state_iterator_t,
}

impl Iterator for DeviceResolutionStateIterator
{
  type Item = xcb::input::device_resolution_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_resolution_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceStateIterator
{
  ptr: *mut xcb::input::device_state_iterator_t,
}

impl Iterator for DeviceStateIterator
{
  type Item = xcb::input::device_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceTimeCoordIterator
{
  ptr: *mut xcb::input::device_time_coord_iterator_t,
}

impl Iterator for DeviceTimeCoordIterator
{
  type Item = xcb::input::device_time_coord_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::device_time_coord_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventClassIterator
{
  ptr: *mut xcb::input::event_class_iterator_t,
}

impl Iterator for EventClassIterator
{
  type Item = xcb::input::event_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::event_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventForSendIterator
{
  ptr: *mut xcb::input::event_for_send_iterator_t,
}

impl Iterator for EventForSendIterator
{
  type Item = xcb::input::event_for_send_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::event_for_send_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventMaskIterator
{
  ptr: *mut xcb::input::event_mask_iterator_t,
}

impl Iterator for EventMaskIterator
{
  type Item = xcb::input::event_mask_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::event_mask_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EventTypeBaseIterator
{
  ptr: *mut xcb::input::event_type_base_iterator_t,
}

impl Iterator for EventTypeBaseIterator
{
  type Item = xcb::input::event_type_base_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::event_type_base_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FeedbackCtlIterator
{
  ptr: *mut xcb::input::feedback_ctl_iterator_t,
}

impl Iterator for FeedbackCtlIterator
{
  type Item = xcb::input::feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FeedbackStateIterator
{
  ptr: *mut xcb::input::feedback_state_iterator_t,
}

impl Iterator for FeedbackStateIterator
{
  type Item = xcb::input::feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Fp1616Iterator
{
  ptr: *mut xcb::input::fp1616_iterator_t,
}

impl Iterator for Fp1616Iterator
{
  type Item = xcb::input::fp1616_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::fp1616_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Fp3232Iterator
{
  ptr: *mut xcb::input::fp3232_iterator_t,
}

impl Iterator for Fp3232Iterator
{
  type Item = xcb::input::fp3232_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::fp3232_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GrabModifierInfoIterator
{
  ptr: *mut xcb::input::grab_modifier_info_iterator_t,
}

impl Iterator for GrabModifierInfoIterator
{
  type Item = xcb::input::grab_modifier_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::grab_modifier_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GroupInfoIterator
{
  ptr: *mut xcb::input::group_info_iterator_t,
}

impl Iterator for GroupInfoIterator
{
  type Item = xcb::input::group_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::group_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct HierarchyChangeIterator
{
  ptr: *mut xcb::input::hierarchy_change_iterator_t,
}

impl Iterator for HierarchyChangeIterator
{
  type Item = xcb::input::hierarchy_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::hierarchy_change_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct HierarchyInfoIterator
{
  ptr: *mut xcb::input::hierarchy_info_iterator_t,
}

impl Iterator for HierarchyInfoIterator
{
  type Item = xcb::input::hierarchy_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::hierarchy_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputClassInfoIterator
{
  ptr: *mut xcb::input::input_class_info_iterator_t,
}

impl Iterator for InputClassInfoIterator
{
  type Item = xcb::input::input_class_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::input_class_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputInfoIterator
{
  ptr: *mut xcb::input::input_info_iterator_t,
}

impl Iterator for InputInfoIterator
{
  type Item = xcb::input::input_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::input_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct InputStateIterator
{
  ptr: *mut xcb::input::input_state_iterator_t,
}

impl Iterator for InputStateIterator
{
  type Item = xcb::input::input_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::input_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct IntegerFeedbackCtlIterator
{
  ptr: *mut xcb::input::integer_feedback_ctl_iterator_t,
}

impl Iterator for IntegerFeedbackCtlIterator
{
  type Item = xcb::input::integer_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::integer_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct IntegerFeedbackStateIterator
{
  ptr: *mut xcb::input::integer_feedback_state_iterator_t,
}

impl Iterator for IntegerFeedbackStateIterator
{
  type Item = xcb::input::integer_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::integer_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KbdFeedbackCtlIterator
{
  ptr: *mut xcb::input::kbd_feedback_ctl_iterator_t,
}

impl Iterator for KbdFeedbackCtlIterator
{
  type Item = xcb::input::kbd_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::kbd_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KbdFeedbackStateIterator
{
  ptr: *mut xcb::input::kbd_feedback_state_iterator_t,
}

impl Iterator for KbdFeedbackStateIterator
{
  type Item = xcb::input::kbd_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::kbd_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyClassIterator
{
  ptr: *mut xcb::input::key_class_iterator_t,
}

impl Iterator for KeyClassIterator
{
  type Item = xcb::input::key_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::key_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyCodeIterator
{
  ptr: *mut xcb::input::key_code_iterator_t,
}

impl Iterator for KeyCodeIterator
{
  type Item = xcb::input::key_code_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::key_code_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyInfoIterator
{
  ptr: *mut xcb::input::key_info_iterator_t,
}

impl Iterator for KeyInfoIterator
{
  type Item = xcb::input::key_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::key_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyStateIterator
{
  ptr: *mut xcb::input::key_state_iterator_t,
}

impl Iterator for KeyStateIterator
{
  type Item = xcb::input::key_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::key_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedFeedbackCtlIterator
{
  ptr: *mut xcb::input::led_feedback_ctl_iterator_t,
}

impl Iterator for LedFeedbackCtlIterator
{
  type Item = xcb::input::led_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::led_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedFeedbackStateIterator
{
  ptr: *mut xcb::input::led_feedback_state_iterator_t,
}

impl Iterator for LedFeedbackStateIterator
{
  type Item = xcb::input::led_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::led_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModifierInfoIterator
{
  ptr: *mut xcb::input::modifier_info_iterator_t,
}

impl Iterator for ModifierInfoIterator
{
  type Item = xcb::input::modifier_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::modifier_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PtrFeedbackCtlIterator
{
  ptr: *mut xcb::input::ptr_feedback_ctl_iterator_t,
}

impl Iterator for PtrFeedbackCtlIterator
{
  type Item = xcb::input::ptr_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::ptr_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PtrFeedbackStateIterator
{
  ptr: *mut xcb::input::ptr_feedback_state_iterator_t,
}

impl Iterator for PtrFeedbackStateIterator
{
  type Item = xcb::input::ptr_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::ptr_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RemoveMasterIterator
{
  ptr: *mut xcb::input::remove_master_iterator_t,
}

impl Iterator for RemoveMasterIterator
{
  type Item = xcb::input::remove_master_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::remove_master_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScrollClassIterator
{
  ptr: *mut xcb::input::scroll_class_iterator_t,
}

impl Iterator for ScrollClassIterator
{
  type Item = xcb::input::scroll_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::scroll_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct StringFeedbackCtlIterator
{
  ptr: *mut xcb::input::string_feedback_ctl_iterator_t,
}

impl Iterator for StringFeedbackCtlIterator
{
  type Item = xcb::input::string_feedback_ctl_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::string_feedback_ctl_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct StringFeedbackStateIterator
{
  ptr: *mut xcb::input::string_feedback_state_iterator_t,
}

impl Iterator for StringFeedbackStateIterator
{
  type Item = xcb::input::string_feedback_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::string_feedback_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TouchClassIterator
{
  ptr: *mut xcb::input::touch_class_iterator_t,
}

impl Iterator for TouchClassIterator
{
  type Item = xcb::input::touch_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::touch_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorClassIterator
{
  ptr: *mut xcb::input::valuator_class_iterator_t,
}

impl Iterator for ValuatorClassIterator
{
  type Item = xcb::input::valuator_class_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::valuator_class_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorInfoIterator
{
  ptr: *mut xcb::input::valuator_info_iterator_t,
}

impl Iterator for ValuatorInfoIterator
{
  type Item = xcb::input::valuator_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::valuator_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ValuatorStateIterator
{
  ptr: *mut xcb::input::valuator_state_iterator_t,
}

impl Iterator for ValuatorStateIterator
{
  type Item = xcb::input::valuator_state_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::valuator_state_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XiDeviceInfoIterator
{
  ptr: *mut xcb::input::xi_device_info_iterator_t,
}

impl Iterator for XiDeviceInfoIterator
{
  type Item = xcb::input::xi_device_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::input::xi_device_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Keycode32Iterator
{
  ptr: *mut xcb::keycode32_iterator_t,
}

impl Iterator for Keycode32Iterator
{
  type Item = xcb::keycode32_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::keycode32_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeycodeIterator
{
  ptr: *mut xcb::keycode_iterator_t,
}

impl Iterator for KeycodeIterator
{
  type Item = xcb::keycode_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::keycode_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeysymIterator
{
  ptr: *mut xcb::keysym_iterator_t,
}

impl Iterator for KeysymIterator
{
  type Item = xcb::keysym_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::keysym_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PixmapIterator
{
  ptr: *mut xcb::pixmap_iterator_t,
}

impl Iterator for PixmapIterator
{
  type Item = xcb::pixmap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::pixmap_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PointIterator
{
  ptr: *mut xcb::point_iterator_t,
}

impl Iterator for PointIterator
{
  type Item = xcb::point_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::point_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PresentEventIterator
{
  ptr: *mut xcb::present::event_iterator_t,
}

impl Iterator for PresentEventIterator
{
  type Item = xcb::present::event_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::present::event_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct NotifyIterator
{
  ptr: *mut xcb::present::notify_iterator_t,
}

impl Iterator for NotifyIterator
{
  type Item = xcb::present::notify_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::present::notify_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CrtcChangeIterator
{
  ptr: *mut xcb::randr::crtc_change_iterator_t,
}

impl Iterator for CrtcChangeIterator
{
  type Item = xcb::randr::crtc_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::crtc_change_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CrtcIterator
{
  ptr: *mut xcb::randr::crtc_iterator_t,
}

impl Iterator for CrtcIterator
{
  type Item = xcb::randr::crtc_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::crtc_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LeaseIterator
{
  ptr: *mut xcb::randr::lease_iterator_t,
}

impl Iterator for LeaseIterator
{
  type Item = xcb::randr::lease_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::lease_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LeaseNotifyIterator
{
  ptr: *mut xcb::randr::lease_notify_iterator_t,
}

impl Iterator for LeaseNotifyIterator
{
  type Item = xcb::randr::lease_notify_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::lease_notify_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModeInfoIterator
{
  ptr: *mut xcb::randr::mode_info_iterator_t,
}

impl Iterator for ModeInfoIterator
{
  type Item = xcb::randr::mode_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::mode_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModeIterator
{
  ptr: *mut xcb::randr::mode_iterator_t,
}

impl Iterator for ModeIterator
{
  type Item = xcb::randr::mode_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::mode_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct MonitorInfoIterator
{
  ptr: *mut xcb::randr::monitor_info_iterator_t,
}

impl Iterator for MonitorInfoIterator
{
  type Item = xcb::randr::monitor_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::monitor_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct NotifyDataIterator
{
  ptr: *mut xcb::randr::notify_data_iterator_t,
}

impl Iterator for NotifyDataIterator
{
  type Item = xcb::randr::notify_data_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::notify_data_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputChangeIterator
{
  ptr: *mut xcb::randr::output_change_iterator_t,
}

impl Iterator for OutputChangeIterator
{
  type Item = xcb::randr::output_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::output_change_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputIterator
{
  ptr: *mut xcb::randr::output_iterator_t,
}

impl Iterator for OutputIterator
{
  type Item = xcb::randr::output_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::output_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutputPropertyIterator
{
  ptr: *mut xcb::randr::output_property_iterator_t,
}

impl Iterator for OutputPropertyIterator
{
  type Item = xcb::randr::output_property_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::output_property_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderChangeIterator
{
  ptr: *mut xcb::randr::provider_change_iterator_t,
}

impl Iterator for ProviderChangeIterator
{
  type Item = xcb::randr::provider_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::provider_change_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderIterator
{
  ptr: *mut xcb::randr::provider_iterator_t,
}

impl Iterator for ProviderIterator
{
  type Item = xcb::randr::provider_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::provider_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ProviderPropertyIterator
{
  ptr: *mut xcb::randr::provider_property_iterator_t,
}

impl Iterator for ProviderPropertyIterator
{
  type Item = xcb::randr::provider_property_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::provider_property_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RefreshRatesIterator
{
  ptr: *mut xcb::randr::refresh_rates_iterator_t,
}

impl Iterator for RefreshRatesIterator
{
  type Item = xcb::randr::refresh_rates_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::refresh_rates_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceChangeIterator
{
  ptr: *mut xcb::randr::resource_change_iterator_t,
}

impl Iterator for ResourceChangeIterator
{
  type Item = xcb::randr::resource_change_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::resource_change_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenSizeIterator
{
  ptr: *mut xcb::randr::screen_size_iterator_t,
}

impl Iterator for ScreenSizeIterator
{
  type Item = xcb::randr::screen_size_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::randr::screen_size_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientInfoIterator
{
  ptr: *mut xcb::record::client_info_iterator_t,
}

impl Iterator for ClientInfoIterator
{
  type Item = xcb::record::client_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::client_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientSpecIterator
{
  ptr: *mut xcb::record::client_spec_iterator_t,
}

impl Iterator for ClientSpecIterator
{
  type Item = xcb::record::client_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::client_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RecordContextIterator
{
  ptr: *mut xcb::record::context_iterator_t,
}

impl Iterator for RecordContextIterator
{
  type Item = xcb::record::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::context_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ElementHeaderIterator
{
  ptr: *mut xcb::record::element_header_iterator_t,
}

impl Iterator for ElementHeaderIterator
{
  type Item = xcb::record::element_header_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::element_header_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ExtRangeIterator
{
  ptr: *mut xcb::record::ext_range_iterator_t,
}

impl Iterator for ExtRangeIterator
{
  type Item = xcb::record::ext_range_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::ext_range_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Range16Iterator
{
  ptr: *mut xcb::record::range_16_iterator_t,
}

impl Iterator for Range16Iterator
{
  type Item = xcb::record::range_16_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::range_16_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Range8Iterator
{
  ptr: *mut xcb::record::range_8_iterator_t,
}

impl Iterator for Range8Iterator
{
  type Item = xcb::record::range_8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::range_8_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RangeIterator
{
  ptr: *mut xcb::record::range_iterator_t,
}

impl Iterator for RangeIterator
{
  type Item = xcb::record::range_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::record::range_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RectangleIterator
{
  ptr: *mut xcb::rectangle_iterator_t,
}

impl Iterator for RectangleIterator
{
  type Item = xcb::rectangle_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::rectangle_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AnimcursoreltIterator
{
  ptr: *mut xcb::render::animcursorelt_iterator_t,
}

impl Iterator for AnimcursoreltIterator
{
  type Item = xcb::render::animcursorelt_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::animcursorelt_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ColorIterator
{
  ptr: *mut xcb::render::color_iterator_t,
}

impl Iterator for ColorIterator
{
  type Item = xcb::render::color_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::color_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DirectformatIterator
{
  ptr: *mut xcb::render::directformat_iterator_t,
}

impl Iterator for DirectformatIterator
{
  type Item = xcb::render::directformat_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::directformat_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FixedIterator
{
  ptr: *mut xcb::render::fixed_iterator_t,
}

impl Iterator for FixedIterator
{
  type Item = xcb::render::fixed_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::fixed_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphIterator
{
  ptr: *mut xcb::render::glyph_iterator_t,
}

impl Iterator for GlyphIterator
{
  type Item = xcb::render::glyph_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::glyph_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphinfoIterator
{
  ptr: *mut xcb::render::glyphinfo_iterator_t,
}

impl Iterator for GlyphinfoIterator
{
  type Item = xcb::render::glyphinfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::glyphinfo_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct GlyphsetIterator
{
  ptr: *mut xcb::render::glyphset_iterator_t,
}

impl Iterator for GlyphsetIterator
{
  type Item = xcb::render::glyphset_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::glyphset_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct IndexvalueIterator
{
  ptr: *mut xcb::render::indexvalue_iterator_t,
}

impl Iterator for IndexvalueIterator
{
  type Item = xcb::render::indexvalue_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::indexvalue_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LinefixIterator
{
  ptr: *mut xcb::render::linefix_iterator_t,
}

impl Iterator for LinefixIterator
{
  type Item = xcb::render::linefix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::linefix_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictdepthIterator
{
  ptr: *mut xcb::render::pictdepth_iterator_t,
}

impl Iterator for PictdepthIterator
{
  type Item = xcb::render::pictdepth_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pictdepth_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictformatIterator
{
  ptr: *mut xcb::render::pictformat_iterator_t,
}

impl Iterator for PictformatIterator
{
  type Item = xcb::render::pictformat_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pictformat_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictforminfoIterator
{
  ptr: *mut xcb::render::pictforminfo_iterator_t,
}

impl Iterator for PictforminfoIterator
{
  type Item = xcb::render::pictforminfo_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pictforminfo_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictscreenIterator
{
  ptr: *mut xcb::render::pictscreen_iterator_t,
}

impl Iterator for PictscreenIterator
{
  type Item = xcb::render::pictscreen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pictscreen_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictureIterator
{
  ptr: *mut xcb::render::picture_iterator_t,
}

impl Iterator for PictureIterator
{
  type Item = xcb::render::picture_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::picture_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PictvisualIterator
{
  ptr: *mut xcb::render::pictvisual_iterator_t,
}

impl Iterator for PictvisualIterator
{
  type Item = xcb::render::pictvisual_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pictvisual_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PointfixIterator
{
  ptr: *mut xcb::render::pointfix_iterator_t,
}

impl Iterator for PointfixIterator
{
  type Item = xcb::render::pointfix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::pointfix_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SpanfixIterator
{
  ptr: *mut xcb::render::spanfix_iterator_t,
}

impl Iterator for SpanfixIterator
{
  type Item = xcb::render::spanfix_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::spanfix_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TransformIterator
{
  ptr: *mut xcb::render::transform_iterator_t,
}

impl Iterator for TransformIterator
{
  type Item = xcb::render::transform_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::transform_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TrapIterator
{
  ptr: *mut xcb::render::trap_iterator_t,
}

impl Iterator for TrapIterator
{
  type Item = xcb::render::trap_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::trap_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TrapezoidIterator
{
  ptr: *mut xcb::render::trapezoid_iterator_t,
}

impl Iterator for TrapezoidIterator
{
  type Item = xcb::render::trapezoid_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::trapezoid_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TriangleIterator
{
  ptr: *mut xcb::render::triangle_iterator_t,
}

impl Iterator for TriangleIterator
{
  type Item = xcb::render::triangle_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::render::triangle_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIdSpecIterator
{
  ptr: *mut xcb::res::client_id_spec_iterator_t,
}

impl Iterator for ClientIdSpecIterator
{
  type Item = xcb::res::client_id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::client_id_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIdValueIterator
{
  ptr: *mut xcb::res::client_id_value_iterator_t,
}

impl Iterator for ClientIdValueIterator
{
  type Item = xcb::res::client_id_value_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::client_id_value_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ClientIterator
{
  ptr: *mut xcb::res::client_iterator_t,
}

impl Iterator for ClientIterator
{
  type Item = xcb::res::client_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::client_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceIdSpecIterator
{
  ptr: *mut xcb::res::resource_id_spec_iterator_t,
}

impl Iterator for ResourceIdSpecIterator
{
  type Item = xcb::res::resource_id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::resource_id_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceSizeSpecIterator
{
  ptr: *mut xcb::res::resource_size_spec_iterator_t,
}

impl Iterator for ResourceSizeSpecIterator
{
  type Item = xcb::res::resource_size_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::resource_size_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ResourceSizeValueIterator
{
  ptr: *mut xcb::res::resource_size_value_iterator_t,
}

impl Iterator for ResourceSizeValueIterator
{
  type Item = xcb::res::resource_size_value_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::resource_size_value_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TypeIterator
{
  ptr: *mut xcb::res::type_iterator_t,
}

impl Iterator for TypeIterator
{
  type Item = xcb::res::type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::res::type_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RgbIterator
{
  ptr: *mut xcb::rgb_iterator_t,
}

impl Iterator for RgbIterator
{
  type Item = xcb::rgb_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::rgb_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenIterator
{
  ptr: *mut xcb::screen_iterator_t,
}

impl Iterator for ScreenIterator
{
  type Item = xcb::screen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::screen_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SegmentIterator
{
  ptr: *mut xcb::segment_iterator_t,
}

impl Iterator for SegmentIterator
{
  type Item = xcb::segment_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::segment_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ListItemIterator
{
  ptr: *mut xcb::selinux::list_item_iterator_t,
}

impl Iterator for ListItemIterator
{
  type Item = xcb::selinux::list_item_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::selinux::list_item_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupAuthenticateIterator
{
  ptr: *mut xcb::setup_authenticate_iterator_t,
}

impl Iterator for SetupAuthenticateIterator
{
  type Item = xcb::setup_authenticate_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::setup_authenticate_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupFailedIterator
{
  ptr: *mut xcb::setup_failed_iterator_t,
}

impl Iterator for SetupFailedIterator
{
  type Item = xcb::setup_failed_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::setup_failed_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupIterator
{
  ptr: *mut xcb::setup_iterator_t,
}

impl Iterator for SetupIterator
{
  type Item = xcb::setup_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::setup_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetupRequestIterator
{
  ptr: *mut xcb::setup_request_iterator_t,
}

impl Iterator for SetupRequestIterator
{
  type Item = xcb::setup_request_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::setup_request_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KindIterator
{
  ptr: *mut xcb::shape::kind_iterator_t,
}

impl Iterator for KindIterator
{
  type Item = xcb::shape::kind_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::shape::kind_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OpIterator
{
  ptr: *mut xcb::shape::op_iterator_t,
}

impl Iterator for OpIterator
{
  type Item = xcb::shape::op_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::shape::op_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SegIterator
{
  ptr: *mut xcb::shm::seg_iterator_t,
}

impl Iterator for SegIterator
{
  type Item = xcb::shm::seg_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::shm::seg_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct StrIterator
{
  ptr: *mut xcb::str_iterator_t,
}

impl Iterator for StrIterator
{
  type Item = xcb::str_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::str_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AlarmIterator
{
  ptr: *mut xcb::sync::alarm_iterator_t,
}

impl Iterator for AlarmIterator
{
  type Item = xcb::sync::alarm_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::alarm_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CounterIterator
{
  ptr: *mut xcb::sync::counter_iterator_t,
}

impl Iterator for CounterIterator
{
  type Item = xcb::sync::counter_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::counter_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct FenceIterator
{
  ptr: *mut xcb::sync::fence_iterator_t,
}

impl Iterator for FenceIterator
{
  type Item = xcb::sync::fence_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::fence_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct Int64Iterator
{
  ptr: *mut xcb::sync::int64_iterator_t,
}

impl Iterator for Int64Iterator
{
  type Item = xcb::sync::int64_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::int64_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SystemcounterIterator
{
  ptr: *mut xcb::sync::systemcounter_iterator_t,
}

impl Iterator for SystemcounterIterator
{
  type Item = xcb::sync::systemcounter_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::systemcounter_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TriggerIterator
{
  ptr: *mut xcb::sync::trigger_iterator_t,
}

impl Iterator for TriggerIterator
{
  type Item = xcb::sync::trigger_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::trigger_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct WaitconditionIterator
{
  ptr: *mut xcb::sync::waitcondition_iterator_t,
}

impl Iterator for WaitconditionIterator
{
  type Item = xcb::sync::waitcondition_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::sync::waitcondition_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TimecoordIterator
{
  ptr: *mut xcb::timecoord_iterator_t,
}

impl Iterator for TimecoordIterator
{
  type Item = xcb::timecoord_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::timecoord_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct TimestampIterator
{
  ptr: *mut xcb::timestamp_iterator_t,
}

impl Iterator for TimestampIterator
{
  type Item = xcb::timestamp_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::timestamp_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct VisualidIterator
{
  ptr: *mut xcb::visualid_iterator_t,
}

impl Iterator for VisualidIterator
{
  type Item = xcb::visualid_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::visualid_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct VisualtypeIterator
{
  ptr: *mut xcb::visualtype_iterator_t,
}

impl Iterator for VisualtypeIterator
{
  type Item = xcb::visualtype_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::visualtype_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct WindowIterator
{
  ptr: *mut xcb::window_iterator_t,
}

impl Iterator for WindowIterator
{
  type Item = xcb::window_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::window_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PcontextIterator
{
  ptr: *mut xcb::xprint::pcontext_iterator_t,
}

impl Iterator for PcontextIterator
{
  type Item = xcb::xprint::pcontext_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xprint::pcontext_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PrinterIterator
{
  ptr: *mut xcb::xprint::printer_iterator_t,
}

impl Iterator for PrinterIterator
{
  type Item = xcb::xprint::printer_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xprint::printer_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XprintString8Iterator
{
  ptr: *mut xcb::xprint::string8_iterator_t,
}

impl Iterator for XprintString8Iterator
{
  type Item = xcb::xprint::string8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xprint::string8_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XevieEventIterator
{
  ptr: *mut xcb::xevie::event_iterator_t,
}

impl Iterator for XevieEventIterator
{
  type Item = xcb::xevie::event_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xevie::event_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DrmClipRectIterator
{
  ptr: *mut xcb::xf86dri::drm_clip_rect_iterator_t,
}

impl Iterator for DrmClipRectIterator
{
  type Item = xcb::xf86dri::drm_clip_rect_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xf86dri::drm_clip_rect_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BarrierIterator
{
  ptr: *mut xcb::xfixes::barrier_iterator_t,
}

impl Iterator for BarrierIterator
{
  type Item = xcb::xfixes::barrier_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xfixes::barrier_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RegionIterator
{
  ptr: *mut xcb::xfixes::region_iterator_t,
}

impl Iterator for RegionIterator
{
  type Item = xcb::xfixes::region_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xfixes::region_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ScreenInfoIterator
{
  ptr: *mut xcb::xinerama::screen_info_iterator_t,
}

impl Iterator for ScreenInfoIterator
{
  type Item = xcb::xinerama::screen_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xinerama::screen_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ActionIterator
{
  ptr: *mut xcb::xkb::action_iterator_t,
}

impl Iterator for ActionIterator
{
  type Item = xcb::xkb::action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::action_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BehaviorIterator
{
  ptr: *mut xcb::xkb::behavior_iterator_t,
}

impl Iterator for BehaviorIterator
{
  type Item = xcb::xkb::behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct BellClassSpecIterator
{
  ptr: *mut xcb::xkb::bell_class_spec_iterator_t,
}

impl Iterator for BellClassSpecIterator
{
  type Item = xcb::xkb::bell_class_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::bell_class_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CommonBehaviorIterator
{
  ptr: *mut xcb::xkb::common_behavior_iterator_t,
}

impl Iterator for CommonBehaviorIterator
{
  type Item = xcb::xkb::common_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::common_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct CountedString16Iterator
{
  ptr: *mut xcb::xkb::counted_string_16_iterator_t,
}

impl Iterator for CountedString16Iterator
{
  type Item = xcb::xkb::counted_string_16_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::counted_string_16_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DefaultBehaviorIterator
{
  ptr: *mut xcb::xkb::default_behavior_iterator_t,
}

impl Iterator for DefaultBehaviorIterator
{
  type Item = xcb::xkb::default_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::default_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceLedInfoIterator
{
  ptr: *mut xcb::xkb::device_led_info_iterator_t,
}

impl Iterator for DeviceLedInfoIterator
{
  type Item = xcb::xkb::device_led_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::device_led_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct DeviceSpecIterator
{
  ptr: *mut xcb::xkb::device_spec_iterator_t,
}

impl Iterator for DeviceSpecIterator
{
  type Item = xcb::xkb::device_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::device_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct IdSpecIterator
{
  ptr: *mut xcb::xkb::id_spec_iterator_t,
}

impl Iterator for IdSpecIterator
{
  type Item = xcb::xkb::id_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::id_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct IndicatorMapIterator
{
  ptr: *mut xcb::xkb::indicator_map_iterator_t,
}

impl Iterator for IndicatorMapIterator
{
  type Item = xcb::xkb::indicator_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::indicator_map_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyAliasIterator
{
  ptr: *mut xcb::xkb::key_alias_iterator_t,
}

impl Iterator for KeyAliasIterator
{
  type Item = xcb::xkb::key_alias_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_alias_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyIterator
{
  ptr: *mut xcb::xkb::key_iterator_t,
}

impl Iterator for KeyIterator
{
  type Item = xcb::xkb::key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyModMapIterator
{
  ptr: *mut xcb::xkb::key_mod_map_iterator_t,
}

impl Iterator for KeyModMapIterator
{
  type Item = xcb::xkb::key_mod_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_mod_map_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyNameIterator
{
  ptr: *mut xcb::xkb::key_name_iterator_t,
}

impl Iterator for KeyNameIterator
{
  type Item = xcb::xkb::key_name_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_name_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeySymMapIterator
{
  ptr: *mut xcb::xkb::key_sym_map_iterator_t,
}

impl Iterator for KeySymMapIterator
{
  type Item = xcb::xkb::key_sym_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_sym_map_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyTypeIterator
{
  ptr: *mut xcb::xkb::key_type_iterator_t,
}

impl Iterator for KeyTypeIterator
{
  type Item = xcb::xkb::key_type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_type_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KeyVModMapIterator
{
  ptr: *mut xcb::xkb::key_v_mod_map_iterator_t,
}

impl Iterator for KeyVModMapIterator
{
  type Item = xcb::xkb::key_v_mod_map_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::key_v_mod_map_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KtMapEntryIterator
{
  ptr: *mut xcb::xkb::kt_map_entry_iterator_t,
}

impl Iterator for KtMapEntryIterator
{
  type Item = xcb::xkb::kt_map_entry_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::kt_map_entry_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct KtSetMapEntryIterator
{
  ptr: *mut xcb::xkb::kt_set_map_entry_iterator_t,
}

impl Iterator for KtSetMapEntryIterator
{
  type Item = xcb::xkb::kt_set_map_entry_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::kt_set_map_entry_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LedClassSpecIterator
{
  ptr: *mut xcb::xkb::led_class_spec_iterator_t,
}

impl Iterator for LedClassSpecIterator
{
  type Item = xcb::xkb::led_class_spec_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::led_class_spec_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ListingIterator
{
  ptr: *mut xcb::xkb::listing_iterator_t,
}

impl Iterator for ListingIterator
{
  type Item = xcb::xkb::listing_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::listing_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct LockBehaviorIterator
{
  ptr: *mut xcb::xkb::lock_behavior_iterator_t,
}

impl Iterator for LockBehaviorIterator
{
  type Item = xcb::xkb::lock_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::lock_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ModDefIterator
{
  ptr: *mut xcb::xkb::mod_def_iterator_t,
}

impl Iterator for ModDefIterator
{
  type Item = xcb::xkb::mod_def_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::mod_def_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OutlineIterator
{
  ptr: *mut xcb::xkb::outline_iterator_t,
}

impl Iterator for OutlineIterator
{
  type Item = xcb::xkb::outline_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::outline_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayBehaviorIterator
{
  ptr: *mut xcb::xkb::overlay_behavior_iterator_t,
}

impl Iterator for OverlayBehaviorIterator
{
  type Item = xcb::xkb::overlay_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::overlay_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayIterator
{
  ptr: *mut xcb::xkb::overlay_iterator_t,
}

impl Iterator for OverlayIterator
{
  type Item = xcb::xkb::overlay_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::overlay_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayKeyIterator
{
  ptr: *mut xcb::xkb::overlay_key_iterator_t,
}

impl Iterator for OverlayKeyIterator
{
  type Item = xcb::xkb::overlay_key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::overlay_key_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct OverlayRowIterator
{
  ptr: *mut xcb::xkb::overlay_row_iterator_t,
}

impl Iterator for OverlayRowIterator
{
  type Item = xcb::xkb::overlay_row_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::overlay_row_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentLockBehaviorIterator
{
  ptr: *mut xcb::xkb::permament_lock_behavior_iterator_t,
}

impl Iterator for PermamentLockBehaviorIterator
{
  type Item = xcb::xkb::permament_lock_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::permament_lock_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentOverlayBehaviorIterator
{
  ptr: *mut xcb::xkb::permament_overlay_behavior_iterator_t,
}

impl Iterator for PermamentOverlayBehaviorIterator
{
  type Item = xcb::xkb::permament_overlay_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::permament_overlay_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PermamentRadioGroupBehaviorIterator
{
  ptr: *mut xcb::xkb::permament_radio_group_behavior_iterator_t,
}

impl Iterator for PermamentRadioGroupBehaviorIterator
{
  type Item = xcb::xkb::permament_radio_group_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::permament_radio_group_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RadioGroupBehaviorIterator
{
  ptr: *mut xcb::xkb::radio_group_behavior_iterator_t,
}

impl Iterator for RadioGroupBehaviorIterator
{
  type Item = xcb::xkb::radio_group_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::radio_group_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RowIterator
{
  ptr: *mut xcb::xkb::row_iterator_t,
}

impl Iterator for RowIterator
{
  type Item = xcb::xkb::row_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::row_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaActionMessageIterator
{
  ptr: *mut xcb::xkb::sa_action_message_iterator_t,
}

impl Iterator for SaActionMessageIterator
{
  type Item = xcb::xkb::sa_action_message_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_action_message_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaDeviceBtnIterator
{
  ptr: *mut xcb::xkb::sa_device_btn_iterator_t,
}

impl Iterator for SaDeviceBtnIterator
{
  type Item = xcb::xkb::sa_device_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_device_btn_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaDeviceValuatorIterator
{
  ptr: *mut xcb::xkb::sa_device_valuator_iterator_t,
}

impl Iterator for SaDeviceValuatorIterator
{
  type Item = xcb::xkb::sa_device_valuator_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_device_valuator_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaIsoLockIterator
{
  ptr: *mut xcb::xkb::sa_iso_lock_iterator_t,
}

impl Iterator for SaIsoLockIterator
{
  type Item = xcb::xkb::sa_iso_lock_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_iso_lock_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLatchGroupIterator
{
  ptr: *mut xcb::xkb::sa_latch_group_iterator_t,
}

impl Iterator for SaLatchGroupIterator
{
  type Item = xcb::xkb::sa_latch_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_latch_group_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLatchModsIterator
{
  ptr: *mut xcb::xkb::sa_latch_mods_iterator_t,
}

impl Iterator for SaLatchModsIterator
{
  type Item = xcb::xkb::sa_latch_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_latch_mods_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockControlsIterator
{
  ptr: *mut xcb::xkb::sa_lock_controls_iterator_t,
}

impl Iterator for SaLockControlsIterator
{
  type Item = xcb::xkb::sa_lock_controls_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_lock_controls_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockDeviceBtnIterator
{
  ptr: *mut xcb::xkb::sa_lock_device_btn_iterator_t,
}

impl Iterator for SaLockDeviceBtnIterator
{
  type Item = xcb::xkb::sa_lock_device_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_lock_device_btn_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockGroupIterator
{
  ptr: *mut xcb::xkb::sa_lock_group_iterator_t,
}

impl Iterator for SaLockGroupIterator
{
  type Item = xcb::xkb::sa_lock_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_lock_group_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockModsIterator
{
  ptr: *mut xcb::xkb::sa_lock_mods_iterator_t,
}

impl Iterator for SaLockModsIterator
{
  type Item = xcb::xkb::sa_lock_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_lock_mods_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaLockPtrBtnIterator
{
  ptr: *mut xcb::xkb::sa_lock_ptr_btn_iterator_t,
}

impl Iterator for SaLockPtrBtnIterator
{
  type Item = xcb::xkb::sa_lock_ptr_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_lock_ptr_btn_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaMovePtrIterator
{
  ptr: *mut xcb::xkb::sa_move_ptr_iterator_t,
}

impl Iterator for SaMovePtrIterator
{
  type Item = xcb::xkb::sa_move_ptr_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_move_ptr_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaNoActionIterator
{
  ptr: *mut xcb::xkb::sa_no_action_iterator_t,
}

impl Iterator for SaNoActionIterator
{
  type Item = xcb::xkb::sa_no_action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_no_action_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaPtrBtnIterator
{
  ptr: *mut xcb::xkb::sa_ptr_btn_iterator_t,
}

impl Iterator for SaPtrBtnIterator
{
  type Item = xcb::xkb::sa_ptr_btn_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_ptr_btn_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaRedirectKeyIterator
{
  ptr: *mut xcb::xkb::sa_redirect_key_iterator_t,
}

impl Iterator for SaRedirectKeyIterator
{
  type Item = xcb::xkb::sa_redirect_key_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_redirect_key_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetControlsIterator
{
  ptr: *mut xcb::xkb::sa_set_controls_iterator_t,
}

impl Iterator for SaSetControlsIterator
{
  type Item = xcb::xkb::sa_set_controls_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_set_controls_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetGroupIterator
{
  ptr: *mut xcb::xkb::sa_set_group_iterator_t,
}

impl Iterator for SaSetGroupIterator
{
  type Item = xcb::xkb::sa_set_group_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_set_group_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetModsIterator
{
  ptr: *mut xcb::xkb::sa_set_mods_iterator_t,
}

impl Iterator for SaSetModsIterator
{
  type Item = xcb::xkb::sa_set_mods_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_set_mods_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSetPtrDfltIterator
{
  ptr: *mut xcb::xkb::sa_set_ptr_dflt_iterator_t,
}

impl Iterator for SaSetPtrDfltIterator
{
  type Item = xcb::xkb::sa_set_ptr_dflt_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_set_ptr_dflt_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaSwitchScreenIterator
{
  ptr: *mut xcb::xkb::sa_switch_screen_iterator_t,
}

impl Iterator for SaSwitchScreenIterator
{
  type Item = xcb::xkb::sa_switch_screen_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_switch_screen_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SaTerminateIterator
{
  ptr: *mut xcb::xkb::sa_terminate_iterator_t,
}

impl Iterator for SaTerminateIterator
{
  type Item = xcb::xkb::sa_terminate_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sa_terminate_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetBehaviorIterator
{
  ptr: *mut xcb::xkb::set_behavior_iterator_t,
}

impl Iterator for SetBehaviorIterator
{
  type Item = xcb::xkb::set_behavior_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::set_behavior_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetExplicitIterator
{
  ptr: *mut xcb::xkb::set_explicit_iterator_t,
}

impl Iterator for SetExplicitIterator
{
  type Item = xcb::xkb::set_explicit_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::set_explicit_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SetKeyTypeIterator
{
  ptr: *mut xcb::xkb::set_key_type_iterator_t,
}

impl Iterator for SetKeyTypeIterator
{
  type Item = xcb::xkb::set_key_type_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::set_key_type_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ShapeIterator
{
  ptr: *mut xcb::xkb::shape_iterator_t,
}

impl Iterator for ShapeIterator
{
  type Item = xcb::xkb::shape_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::shape_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SiActionIterator
{
  ptr: *mut xcb::xkb::si_action_iterator_t,
}

impl Iterator for SiActionIterator
{
  type Item = xcb::xkb::si_action_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::si_action_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XkbString8Iterator
{
  ptr: *mut xcb::xkb::string8_iterator_t,
}

impl Iterator for XkbString8Iterator
{
  type Item = xcb::xkb::string8_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::string8_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SymInterpretIterator
{
  ptr: *mut xcb::xkb::sym_interpret_iterator_t,
}

impl Iterator for SymInterpretIterator
{
  type Item = xcb::xkb::sym_interpret_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xkb::sym_interpret_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AdaptorInfoIterator
{
  ptr: *mut xcb::xv::adaptor_info_iterator_t,
}

impl Iterator for AdaptorInfoIterator
{
  type Item = xcb::xv::adaptor_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::adaptor_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct AttributeInfoIterator
{
  ptr: *mut xcb::xv::attribute_info_iterator_t,
}

impl Iterator for AttributeInfoIterator
{
  type Item = xcb::xv::attribute_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::attribute_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EncodingInfoIterator
{
  ptr: *mut xcb::xv::encoding_info_iterator_t,
}

impl Iterator for EncodingInfoIterator
{
  type Item = xcb::xv::encoding_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::encoding_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct EncodingIterator
{
  ptr: *mut xcb::xv::encoding_iterator_t,
}

impl Iterator for EncodingIterator
{
  type Item = xcb::xv::encoding_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::encoding_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XvFormatIterator
{
  ptr: *mut xcb::xv::format_iterator_t,
}

impl Iterator for XvFormatIterator
{
  type Item = xcb::xv::format_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::format_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ImageFormatInfoIterator
{
  ptr: *mut xcb::xv::image_format_info_iterator_t,
}

impl Iterator for ImageFormatInfoIterator
{
  type Item = xcb::xv::image_format_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::image_format_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct ImageIterator
{
  ptr: *mut xcb::xv::image_iterator_t,
}

impl Iterator for ImageIterator
{
  type Item = xcb::xv::image_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::image_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct PortIterator
{
  ptr: *mut xcb::xv::port_iterator_t,
}

impl Iterator for PortIterator
{
  type Item = xcb::xv::port_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::port_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct RationalIterator
{
  ptr: *mut xcb::xv::rational_iterator_t,
}

impl Iterator for RationalIterator
{
  type Item = xcb::xv::rational_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xv::rational_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct XvmcContextIterator
{
  ptr: *mut xcb::xvmc::context_iterator_t,
}

impl Iterator for XvmcContextIterator
{
  type Item = xcb::xvmc::context_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xvmc::context_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SubpictureIterator
{
  ptr: *mut xcb::xvmc::subpicture_iterator_t,
}

impl Iterator for SubpictureIterator
{
  type Item = xcb::xvmc::subpicture_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xvmc::subpicture_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SurfaceInfoIterator
{
  ptr: *mut xcb::xvmc::surface_info_iterator_t,
}

impl Iterator for SurfaceInfoIterator
{
  type Item = xcb::xvmc::surface_info_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xvmc::surface_info_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}

pub struct SurfaceIterator
{
  ptr: *mut xcb::xvmc::surface_iterator_t,
}

impl Iterator for SurfaceIterator
{
  type Item = xcb::xvmc::surface_t;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xcb::xvmc::surface_next(self.ptr) };
      Some(unsafe { *data })
    }
  }
}
