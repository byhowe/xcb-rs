pub use crate::ffi::randr::{
  xcb_randr_add_output_mode as add_output_mode,
  xcb_randr_add_output_mode_checked as add_output_mode_checked,
  xcb_randr_add_output_mode_request_t as add_output_mode_request_t,
  xcb_randr_bad_crtc_error_t as bad_crtc_error_t,
  xcb_randr_bad_mode_error_t as bad_mode_error_t,
  xcb_randr_bad_output_error_t as bad_output_error_t,
  xcb_randr_bad_provider_error_t as bad_provider_error_t,
  xcb_randr_change_output_property as change_output_property,
  xcb_randr_change_output_property_checked as change_output_property_checked,
  xcb_randr_change_output_property_data as change_output_property_data,
  xcb_randr_change_output_property_data_end as change_output_property_data_end,
  xcb_randr_change_output_property_data_length as change_output_property_data_length,
  xcb_randr_change_output_property_request_t as change_output_property_request_t,
  xcb_randr_change_output_property_sizeof as change_output_property_sizeof,
  xcb_randr_change_provider_property as change_provider_property,
  xcb_randr_change_provider_property_checked as change_provider_property_checked,
  xcb_randr_change_provider_property_data as change_provider_property_data,
  xcb_randr_change_provider_property_data_end as change_provider_property_data_end,
  xcb_randr_change_provider_property_data_length as change_provider_property_data_length,
  xcb_randr_change_provider_property_request_t as change_provider_property_request_t,
  xcb_randr_change_provider_property_sizeof as change_provider_property_sizeof,
  xcb_randr_configure_output_property as configure_output_property,
  xcb_randr_configure_output_property_checked as configure_output_property_checked,
  xcb_randr_configure_output_property_request_t as configure_output_property_request_t,
  xcb_randr_configure_output_property_sizeof as configure_output_property_sizeof,
  xcb_randr_configure_output_property_values as configure_output_property_values,
  xcb_randr_configure_output_property_values_end as configure_output_property_values_end,
  xcb_randr_configure_output_property_values_length as configure_output_property_values_length,
  xcb_randr_configure_provider_property as configure_provider_property,
  xcb_randr_configure_provider_property_checked as configure_provider_property_checked,
  xcb_randr_configure_provider_property_request_t as configure_provider_property_request_t,
  xcb_randr_configure_provider_property_sizeof as configure_provider_property_sizeof,
  xcb_randr_configure_provider_property_values as configure_provider_property_values,
  xcb_randr_configure_provider_property_values_end as configure_provider_property_values_end,
  xcb_randr_configure_provider_property_values_length as configure_provider_property_values_length,
  xcb_randr_connection_t as connection_t,
  xcb_randr_create_lease as create_lease,
  xcb_randr_create_lease_cookie_t as create_lease_cookie_t,
  xcb_randr_create_lease_reply as create_lease_reply,
  xcb_randr_create_lease_reply_fds as create_lease_reply_fds,
  xcb_randr_create_lease_reply_t as create_lease_reply_t,
  xcb_randr_create_lease_request_t as create_lease_request_t,
  xcb_randr_create_lease_sizeof as create_lease_sizeof,
  xcb_randr_create_lease_unchecked as create_lease_unchecked,
  xcb_randr_create_mode as create_mode,
  xcb_randr_create_mode_cookie_t as create_mode_cookie_t,
  xcb_randr_create_mode_reply as create_mode_reply,
  xcb_randr_create_mode_reply_t as create_mode_reply_t,
  xcb_randr_create_mode_request_t as create_mode_request_t,
  xcb_randr_create_mode_sizeof as create_mode_sizeof,
  xcb_randr_create_mode_unchecked as create_mode_unchecked,
  xcb_randr_crtc_change_end as crtc_change_end,
  xcb_randr_crtc_change_iterator_t as crtc_change_iterator_t,
  xcb_randr_crtc_change_next as crtc_change_next,
  xcb_randr_crtc_change_t as crtc_change_t,
  xcb_randr_crtc_end as crtc_end,
  xcb_randr_crtc_iterator_t as crtc_iterator_t,
  xcb_randr_crtc_next as crtc_next,
  xcb_randr_crtc_t as crtc_t,
  xcb_randr_delete_monitor as delete_monitor,
  xcb_randr_delete_monitor_checked as delete_monitor_checked,
  xcb_randr_delete_monitor_request_t as delete_monitor_request_t,
  xcb_randr_delete_output_mode as delete_output_mode,
  xcb_randr_delete_output_mode_checked as delete_output_mode_checked,
  xcb_randr_delete_output_mode_request_t as delete_output_mode_request_t,
  xcb_randr_delete_output_property as delete_output_property,
  xcb_randr_delete_output_property_checked as delete_output_property_checked,
  xcb_randr_delete_output_property_request_t as delete_output_property_request_t,
  xcb_randr_delete_provider_property as delete_provider_property,
  xcb_randr_delete_provider_property_checked as delete_provider_property_checked,
  xcb_randr_delete_provider_property_request_t as delete_provider_property_request_t,
  xcb_randr_destroy_mode as destroy_mode,
  xcb_randr_destroy_mode_checked as destroy_mode_checked,
  xcb_randr_destroy_mode_request_t as destroy_mode_request_t,
  xcb_randr_free_lease as free_lease,
  xcb_randr_free_lease_checked as free_lease_checked,
  xcb_randr_free_lease_request_t as free_lease_request_t,
  xcb_randr_get_crtc_gamma as get_crtc_gamma,
  xcb_randr_get_crtc_gamma_blue as get_crtc_gamma_blue,
  xcb_randr_get_crtc_gamma_blue_end as get_crtc_gamma_blue_end,
  xcb_randr_get_crtc_gamma_blue_length as get_crtc_gamma_blue_length,
  xcb_randr_get_crtc_gamma_cookie_t as get_crtc_gamma_cookie_t,
  xcb_randr_get_crtc_gamma_green as get_crtc_gamma_green,
  xcb_randr_get_crtc_gamma_green_end as get_crtc_gamma_green_end,
  xcb_randr_get_crtc_gamma_green_length as get_crtc_gamma_green_length,
  xcb_randr_get_crtc_gamma_red as get_crtc_gamma_red,
  xcb_randr_get_crtc_gamma_red_end as get_crtc_gamma_red_end,
  xcb_randr_get_crtc_gamma_red_length as get_crtc_gamma_red_length,
  xcb_randr_get_crtc_gamma_reply as get_crtc_gamma_reply,
  xcb_randr_get_crtc_gamma_reply_t as get_crtc_gamma_reply_t,
  xcb_randr_get_crtc_gamma_request_t as get_crtc_gamma_request_t,
  xcb_randr_get_crtc_gamma_size as get_crtc_gamma_size,
  xcb_randr_get_crtc_gamma_size_cookie_t as get_crtc_gamma_size_cookie_t,
  xcb_randr_get_crtc_gamma_size_reply as get_crtc_gamma_size_reply,
  xcb_randr_get_crtc_gamma_size_reply_t as get_crtc_gamma_size_reply_t,
  xcb_randr_get_crtc_gamma_size_request_t as get_crtc_gamma_size_request_t,
  xcb_randr_get_crtc_gamma_size_unchecked as get_crtc_gamma_size_unchecked,
  xcb_randr_get_crtc_gamma_sizeof as get_crtc_gamma_sizeof,
  xcb_randr_get_crtc_gamma_unchecked as get_crtc_gamma_unchecked,
  xcb_randr_get_crtc_info as get_crtc_info,
  xcb_randr_get_crtc_info_cookie_t as get_crtc_info_cookie_t,
  xcb_randr_get_crtc_info_outputs as get_crtc_info_outputs,
  xcb_randr_get_crtc_info_outputs_end as get_crtc_info_outputs_end,
  xcb_randr_get_crtc_info_outputs_length as get_crtc_info_outputs_length,
  xcb_randr_get_crtc_info_possible as get_crtc_info_possible,
  xcb_randr_get_crtc_info_possible_end as get_crtc_info_possible_end,
  xcb_randr_get_crtc_info_possible_length as get_crtc_info_possible_length,
  xcb_randr_get_crtc_info_reply as get_crtc_info_reply,
  xcb_randr_get_crtc_info_reply_t as get_crtc_info_reply_t,
  xcb_randr_get_crtc_info_request_t as get_crtc_info_request_t,
  xcb_randr_get_crtc_info_sizeof as get_crtc_info_sizeof,
  xcb_randr_get_crtc_info_unchecked as get_crtc_info_unchecked,
  xcb_randr_get_crtc_transform as get_crtc_transform,
  xcb_randr_get_crtc_transform_cookie_t as get_crtc_transform_cookie_t,
  xcb_randr_get_crtc_transform_current_filter_name as get_crtc_transform_current_filter_name,
  xcb_randr_get_crtc_transform_current_filter_name_end as get_crtc_transform_current_filter_name_end,
  xcb_randr_get_crtc_transform_current_filter_name_length as get_crtc_transform_current_filter_name_length,
  xcb_randr_get_crtc_transform_current_params as get_crtc_transform_current_params,
  xcb_randr_get_crtc_transform_current_params_end as get_crtc_transform_current_params_end,
  xcb_randr_get_crtc_transform_current_params_length as get_crtc_transform_current_params_length,
  xcb_randr_get_crtc_transform_pending_filter_name as get_crtc_transform_pending_filter_name,
  xcb_randr_get_crtc_transform_pending_filter_name_end as get_crtc_transform_pending_filter_name_end,
  xcb_randr_get_crtc_transform_pending_filter_name_length as get_crtc_transform_pending_filter_name_length,
  xcb_randr_get_crtc_transform_pending_params as get_crtc_transform_pending_params,
  xcb_randr_get_crtc_transform_pending_params_end as get_crtc_transform_pending_params_end,
  xcb_randr_get_crtc_transform_pending_params_length as get_crtc_transform_pending_params_length,
  xcb_randr_get_crtc_transform_reply as get_crtc_transform_reply,
  xcb_randr_get_crtc_transform_reply_t as get_crtc_transform_reply_t,
  xcb_randr_get_crtc_transform_request_t as get_crtc_transform_request_t,
  xcb_randr_get_crtc_transform_sizeof as get_crtc_transform_sizeof,
  xcb_randr_get_crtc_transform_unchecked as get_crtc_transform_unchecked,
  xcb_randr_get_monitors as get_monitors,
  xcb_randr_get_monitors_cookie_t as get_monitors_cookie_t,
  xcb_randr_get_monitors_monitors_iterator as get_monitors_monitors_iterator,
  xcb_randr_get_monitors_monitors_length as get_monitors_monitors_length,
  xcb_randr_get_monitors_reply as get_monitors_reply,
  xcb_randr_get_monitors_reply_t as get_monitors_reply_t,
  xcb_randr_get_monitors_request_t as get_monitors_request_t,
  xcb_randr_get_monitors_sizeof as get_monitors_sizeof,
  xcb_randr_get_monitors_unchecked as get_monitors_unchecked,
  xcb_randr_get_output_info as get_output_info,
  xcb_randr_get_output_info_clones as get_output_info_clones,
  xcb_randr_get_output_info_clones_end as get_output_info_clones_end,
  xcb_randr_get_output_info_clones_length as get_output_info_clones_length,
  xcb_randr_get_output_info_cookie_t as get_output_info_cookie_t,
  xcb_randr_get_output_info_crtcs as get_output_info_crtcs,
  xcb_randr_get_output_info_crtcs_end as get_output_info_crtcs_end,
  xcb_randr_get_output_info_crtcs_length as get_output_info_crtcs_length,
  xcb_randr_get_output_info_modes as get_output_info_modes,
  xcb_randr_get_output_info_modes_end as get_output_info_modes_end,
  xcb_randr_get_output_info_modes_length as get_output_info_modes_length,
  xcb_randr_get_output_info_name as get_output_info_name,
  xcb_randr_get_output_info_name_end as get_output_info_name_end,
  xcb_randr_get_output_info_name_length as get_output_info_name_length,
  xcb_randr_get_output_info_reply as get_output_info_reply,
  xcb_randr_get_output_info_reply_t as get_output_info_reply_t,
  xcb_randr_get_output_info_request_t as get_output_info_request_t,
  xcb_randr_get_output_info_sizeof as get_output_info_sizeof,
  xcb_randr_get_output_info_unchecked as get_output_info_unchecked,
  xcb_randr_get_output_primary as get_output_primary,
  xcb_randr_get_output_primary_cookie_t as get_output_primary_cookie_t,
  xcb_randr_get_output_primary_reply as get_output_primary_reply,
  xcb_randr_get_output_primary_reply_t as get_output_primary_reply_t,
  xcb_randr_get_output_primary_request_t as get_output_primary_request_t,
  xcb_randr_get_output_primary_unchecked as get_output_primary_unchecked,
  xcb_randr_get_output_property as get_output_property,
  xcb_randr_get_output_property_cookie_t as get_output_property_cookie_t,
  xcb_randr_get_output_property_data as get_output_property_data,
  xcb_randr_get_output_property_data_end as get_output_property_data_end,
  xcb_randr_get_output_property_data_length as get_output_property_data_length,
  xcb_randr_get_output_property_reply as get_output_property_reply,
  xcb_randr_get_output_property_reply_t as get_output_property_reply_t,
  xcb_randr_get_output_property_request_t as get_output_property_request_t,
  xcb_randr_get_output_property_sizeof as get_output_property_sizeof,
  xcb_randr_get_output_property_unchecked as get_output_property_unchecked,
  xcb_randr_get_panning as get_panning,
  xcb_randr_get_panning_cookie_t as get_panning_cookie_t,
  xcb_randr_get_panning_reply as get_panning_reply,
  xcb_randr_get_panning_reply_t as get_panning_reply_t,
  xcb_randr_get_panning_request_t as get_panning_request_t,
  xcb_randr_get_panning_unchecked as get_panning_unchecked,
  xcb_randr_get_provider_info as get_provider_info,
  xcb_randr_get_provider_info_associated_capability as get_provider_info_associated_capability,
  xcb_randr_get_provider_info_associated_capability_end as get_provider_info_associated_capability_end,
  xcb_randr_get_provider_info_associated_capability_length as get_provider_info_associated_capability_length,
  xcb_randr_get_provider_info_associated_providers as get_provider_info_associated_providers,
  xcb_randr_get_provider_info_associated_providers_end as get_provider_info_associated_providers_end,
  xcb_randr_get_provider_info_associated_providers_length as get_provider_info_associated_providers_length,
  xcb_randr_get_provider_info_cookie_t as get_provider_info_cookie_t,
  xcb_randr_get_provider_info_crtcs as get_provider_info_crtcs,
  xcb_randr_get_provider_info_crtcs_end as get_provider_info_crtcs_end,
  xcb_randr_get_provider_info_crtcs_length as get_provider_info_crtcs_length,
  xcb_randr_get_provider_info_name as get_provider_info_name,
  xcb_randr_get_provider_info_name_end as get_provider_info_name_end,
  xcb_randr_get_provider_info_name_length as get_provider_info_name_length,
  xcb_randr_get_provider_info_outputs as get_provider_info_outputs,
  xcb_randr_get_provider_info_outputs_end as get_provider_info_outputs_end,
  xcb_randr_get_provider_info_outputs_length as get_provider_info_outputs_length,
  xcb_randr_get_provider_info_reply as get_provider_info_reply,
  xcb_randr_get_provider_info_reply_t as get_provider_info_reply_t,
  xcb_randr_get_provider_info_request_t as get_provider_info_request_t,
  xcb_randr_get_provider_info_sizeof as get_provider_info_sizeof,
  xcb_randr_get_provider_info_unchecked as get_provider_info_unchecked,
  xcb_randr_get_provider_property as get_provider_property,
  xcb_randr_get_provider_property_cookie_t as get_provider_property_cookie_t,
  xcb_randr_get_provider_property_data as get_provider_property_data,
  xcb_randr_get_provider_property_data_end as get_provider_property_data_end,
  xcb_randr_get_provider_property_data_length as get_provider_property_data_length,
  xcb_randr_get_provider_property_reply as get_provider_property_reply,
  xcb_randr_get_provider_property_reply_t as get_provider_property_reply_t,
  xcb_randr_get_provider_property_request_t as get_provider_property_request_t,
  xcb_randr_get_provider_property_sizeof as get_provider_property_sizeof,
  xcb_randr_get_provider_property_unchecked as get_provider_property_unchecked,
  xcb_randr_get_providers as get_providers,
  xcb_randr_get_providers_cookie_t as get_providers_cookie_t,
  xcb_randr_get_providers_providers as get_providers_providers,
  xcb_randr_get_providers_providers_end as get_providers_providers_end,
  xcb_randr_get_providers_providers_length as get_providers_providers_length,
  xcb_randr_get_providers_reply as get_providers_reply,
  xcb_randr_get_providers_reply_t as get_providers_reply_t,
  xcb_randr_get_providers_request_t as get_providers_request_t,
  xcb_randr_get_providers_sizeof as get_providers_sizeof,
  xcb_randr_get_providers_unchecked as get_providers_unchecked,
  xcb_randr_get_screen_info as get_screen_info,
  xcb_randr_get_screen_info_cookie_t as get_screen_info_cookie_t,
  xcb_randr_get_screen_info_rates_iterator as get_screen_info_rates_iterator,
  xcb_randr_get_screen_info_rates_length as get_screen_info_rates_length,
  xcb_randr_get_screen_info_reply as get_screen_info_reply,
  xcb_randr_get_screen_info_reply_t as get_screen_info_reply_t,
  xcb_randr_get_screen_info_request_t as get_screen_info_request_t,
  xcb_randr_get_screen_info_sizeof as get_screen_info_sizeof,
  xcb_randr_get_screen_info_sizes as get_screen_info_sizes,
  xcb_randr_get_screen_info_sizes_iterator as get_screen_info_sizes_iterator,
  xcb_randr_get_screen_info_sizes_length as get_screen_info_sizes_length,
  xcb_randr_get_screen_info_unchecked as get_screen_info_unchecked,
  xcb_randr_get_screen_resources as get_screen_resources,
  xcb_randr_get_screen_resources_cookie_t as get_screen_resources_cookie_t,
  xcb_randr_get_screen_resources_crtcs as get_screen_resources_crtcs,
  xcb_randr_get_screen_resources_crtcs_end as get_screen_resources_crtcs_end,
  xcb_randr_get_screen_resources_crtcs_length as get_screen_resources_crtcs_length,
  xcb_randr_get_screen_resources_current as get_screen_resources_current,
  xcb_randr_get_screen_resources_current_cookie_t as get_screen_resources_current_cookie_t,
  xcb_randr_get_screen_resources_current_crtcs as get_screen_resources_current_crtcs,
  xcb_randr_get_screen_resources_current_crtcs_end as get_screen_resources_current_crtcs_end,
  xcb_randr_get_screen_resources_current_crtcs_length as get_screen_resources_current_crtcs_length,
  xcb_randr_get_screen_resources_current_modes as get_screen_resources_current_modes,
  xcb_randr_get_screen_resources_current_modes_iterator as get_screen_resources_current_modes_iterator,
  xcb_randr_get_screen_resources_current_modes_length as get_screen_resources_current_modes_length,
  xcb_randr_get_screen_resources_current_names as get_screen_resources_current_names,
  xcb_randr_get_screen_resources_current_names_end as get_screen_resources_current_names_end,
  xcb_randr_get_screen_resources_current_names_length as get_screen_resources_current_names_length,
  xcb_randr_get_screen_resources_current_outputs as get_screen_resources_current_outputs,
  xcb_randr_get_screen_resources_current_outputs_end as get_screen_resources_current_outputs_end,
  xcb_randr_get_screen_resources_current_outputs_length as get_screen_resources_current_outputs_length,
  xcb_randr_get_screen_resources_current_reply as get_screen_resources_current_reply,
  xcb_randr_get_screen_resources_current_reply_t as get_screen_resources_current_reply_t,
  xcb_randr_get_screen_resources_current_request_t as get_screen_resources_current_request_t,
  xcb_randr_get_screen_resources_current_sizeof as get_screen_resources_current_sizeof,
  xcb_randr_get_screen_resources_current_unchecked as get_screen_resources_current_unchecked,
  xcb_randr_get_screen_resources_modes as get_screen_resources_modes,
  xcb_randr_get_screen_resources_modes_iterator as get_screen_resources_modes_iterator,
  xcb_randr_get_screen_resources_modes_length as get_screen_resources_modes_length,
  xcb_randr_get_screen_resources_names as get_screen_resources_names,
  xcb_randr_get_screen_resources_names_end as get_screen_resources_names_end,
  xcb_randr_get_screen_resources_names_length as get_screen_resources_names_length,
  xcb_randr_get_screen_resources_outputs as get_screen_resources_outputs,
  xcb_randr_get_screen_resources_outputs_end as get_screen_resources_outputs_end,
  xcb_randr_get_screen_resources_outputs_length as get_screen_resources_outputs_length,
  xcb_randr_get_screen_resources_reply as get_screen_resources_reply,
  xcb_randr_get_screen_resources_reply_t as get_screen_resources_reply_t,
  xcb_randr_get_screen_resources_request_t as get_screen_resources_request_t,
  xcb_randr_get_screen_resources_sizeof as get_screen_resources_sizeof,
  xcb_randr_get_screen_resources_unchecked as get_screen_resources_unchecked,
  xcb_randr_get_screen_size_range as get_screen_size_range,
  xcb_randr_get_screen_size_range_cookie_t as get_screen_size_range_cookie_t,
  xcb_randr_get_screen_size_range_reply as get_screen_size_range_reply,
  xcb_randr_get_screen_size_range_reply_t as get_screen_size_range_reply_t,
  xcb_randr_get_screen_size_range_request_t as get_screen_size_range_request_t,
  xcb_randr_get_screen_size_range_unchecked as get_screen_size_range_unchecked,
  xcb_randr_id as id,
  xcb_randr_lease_end as lease_end,
  xcb_randr_lease_iterator_t as lease_iterator_t,
  xcb_randr_lease_next as lease_next,
  xcb_randr_lease_notify_end as lease_notify_end,
  xcb_randr_lease_notify_iterator_t as lease_notify_iterator_t,
  xcb_randr_lease_notify_next as lease_notify_next,
  xcb_randr_lease_notify_t as lease_notify_t,
  xcb_randr_lease_t as lease_t,
  xcb_randr_list_output_properties as list_output_properties,
  xcb_randr_list_output_properties_atoms as list_output_properties_atoms,
  xcb_randr_list_output_properties_atoms_end as list_output_properties_atoms_end,
  xcb_randr_list_output_properties_atoms_length as list_output_properties_atoms_length,
  xcb_randr_list_output_properties_cookie_t as list_output_properties_cookie_t,
  xcb_randr_list_output_properties_reply as list_output_properties_reply,
  xcb_randr_list_output_properties_reply_t as list_output_properties_reply_t,
  xcb_randr_list_output_properties_request_t as list_output_properties_request_t,
  xcb_randr_list_output_properties_sizeof as list_output_properties_sizeof,
  xcb_randr_list_output_properties_unchecked as list_output_properties_unchecked,
  xcb_randr_list_provider_properties as list_provider_properties,
  xcb_randr_list_provider_properties_atoms as list_provider_properties_atoms,
  xcb_randr_list_provider_properties_atoms_end as list_provider_properties_atoms_end,
  xcb_randr_list_provider_properties_atoms_length as list_provider_properties_atoms_length,
  xcb_randr_list_provider_properties_cookie_t as list_provider_properties_cookie_t,
  xcb_randr_list_provider_properties_reply as list_provider_properties_reply,
  xcb_randr_list_provider_properties_reply_t as list_provider_properties_reply_t,
  xcb_randr_list_provider_properties_request_t as list_provider_properties_request_t,
  xcb_randr_list_provider_properties_sizeof as list_provider_properties_sizeof,
  xcb_randr_list_provider_properties_unchecked as list_provider_properties_unchecked,
  xcb_randr_mode_end as mode_end,
  xcb_randr_mode_flag_t as mode_flag_t,
  xcb_randr_mode_info_end as mode_info_end,
  xcb_randr_mode_info_iterator_t as mode_info_iterator_t,
  xcb_randr_mode_info_next as mode_info_next,
  xcb_randr_mode_info_t as mode_info_t,
  xcb_randr_mode_iterator_t as mode_iterator_t,
  xcb_randr_mode_next as mode_next,
  xcb_randr_mode_t as mode_t,
  xcb_randr_monitor_info_end as monitor_info_end,
  xcb_randr_monitor_info_iterator_t as monitor_info_iterator_t,
  xcb_randr_monitor_info_next as monitor_info_next,
  xcb_randr_monitor_info_outputs as monitor_info_outputs,
  xcb_randr_monitor_info_outputs_end as monitor_info_outputs_end,
  xcb_randr_monitor_info_outputs_length as monitor_info_outputs_length,
  xcb_randr_monitor_info_sizeof as monitor_info_sizeof,
  xcb_randr_monitor_info_t as monitor_info_t,
  xcb_randr_notify_data_end as notify_data_end,
  xcb_randr_notify_data_iterator_t as notify_data_iterator_t,
  xcb_randr_notify_data_next as notify_data_next,
  xcb_randr_notify_data_t as notify_data_t,
  xcb_randr_notify_event_t as notify_event_t,
  xcb_randr_notify_mask_t as notify_mask_t,
  xcb_randr_notify_t as notify_t,
  xcb_randr_output_change_end as output_change_end,
  xcb_randr_output_change_iterator_t as output_change_iterator_t,
  xcb_randr_output_change_next as output_change_next,
  xcb_randr_output_change_t as output_change_t,
  xcb_randr_output_end as output_end,
  xcb_randr_output_iterator_t as output_iterator_t,
  xcb_randr_output_next as output_next,
  xcb_randr_output_property_end as output_property_end,
  xcb_randr_output_property_iterator_t as output_property_iterator_t,
  xcb_randr_output_property_next as output_property_next,
  xcb_randr_output_property_t as output_property_t,
  xcb_randr_output_t as output_t,
  xcb_randr_provider_capability_t as provider_capability_t,
  xcb_randr_provider_change_end as provider_change_end,
  xcb_randr_provider_change_iterator_t as provider_change_iterator_t,
  xcb_randr_provider_change_next as provider_change_next,
  xcb_randr_provider_change_t as provider_change_t,
  xcb_randr_provider_end as provider_end,
  xcb_randr_provider_iterator_t as provider_iterator_t,
  xcb_randr_provider_next as provider_next,
  xcb_randr_provider_property_end as provider_property_end,
  xcb_randr_provider_property_iterator_t as provider_property_iterator_t,
  xcb_randr_provider_property_next as provider_property_next,
  xcb_randr_provider_property_t as provider_property_t,
  xcb_randr_provider_t as provider_t,
  xcb_randr_query_output_property as query_output_property,
  xcb_randr_query_output_property_cookie_t as query_output_property_cookie_t,
  xcb_randr_query_output_property_reply as query_output_property_reply,
  xcb_randr_query_output_property_reply_t as query_output_property_reply_t,
  xcb_randr_query_output_property_request_t as query_output_property_request_t,
  xcb_randr_query_output_property_sizeof as query_output_property_sizeof,
  xcb_randr_query_output_property_unchecked as query_output_property_unchecked,
  xcb_randr_query_output_property_valid_values as query_output_property_valid_values,
  xcb_randr_query_output_property_valid_values_end as query_output_property_valid_values_end,
  xcb_randr_query_output_property_valid_values_length as query_output_property_valid_values_length,
  xcb_randr_query_provider_property as query_provider_property,
  xcb_randr_query_provider_property_cookie_t as query_provider_property_cookie_t,
  xcb_randr_query_provider_property_reply as query_provider_property_reply,
  xcb_randr_query_provider_property_reply_t as query_provider_property_reply_t,
  xcb_randr_query_provider_property_request_t as query_provider_property_request_t,
  xcb_randr_query_provider_property_sizeof as query_provider_property_sizeof,
  xcb_randr_query_provider_property_unchecked as query_provider_property_unchecked,
  xcb_randr_query_provider_property_valid_values as query_provider_property_valid_values,
  xcb_randr_query_provider_property_valid_values_end as query_provider_property_valid_values_end,
  xcb_randr_query_provider_property_valid_values_length as query_provider_property_valid_values_length,
  xcb_randr_query_version as query_version,
  xcb_randr_query_version_cookie_t as query_version_cookie_t,
  xcb_randr_query_version_reply as query_version_reply,
  xcb_randr_query_version_reply_t as query_version_reply_t,
  xcb_randr_query_version_request_t as query_version_request_t,
  xcb_randr_query_version_unchecked as query_version_unchecked,
  xcb_randr_refresh_rates_end as refresh_rates_end,
  xcb_randr_refresh_rates_iterator_t as refresh_rates_iterator_t,
  xcb_randr_refresh_rates_next as refresh_rates_next,
  xcb_randr_refresh_rates_rates as refresh_rates_rates,
  xcb_randr_refresh_rates_rates_end as refresh_rates_rates_end,
  xcb_randr_refresh_rates_rates_length as refresh_rates_rates_length,
  xcb_randr_refresh_rates_sizeof as refresh_rates_sizeof,
  xcb_randr_refresh_rates_t as refresh_rates_t,
  xcb_randr_resource_change_end as resource_change_end,
  xcb_randr_resource_change_iterator_t as resource_change_iterator_t,
  xcb_randr_resource_change_next as resource_change_next,
  xcb_randr_resource_change_t as resource_change_t,
  xcb_randr_rotation_t as rotation_t,
  xcb_randr_screen_change_notify_event_t as screen_change_notify_event_t,
  xcb_randr_screen_size_end as screen_size_end,
  xcb_randr_screen_size_iterator_t as screen_size_iterator_t,
  xcb_randr_screen_size_next as screen_size_next,
  xcb_randr_screen_size_t as screen_size_t,
  xcb_randr_select_input as select_input,
  xcb_randr_select_input_checked as select_input_checked,
  xcb_randr_select_input_request_t as select_input_request_t,
  xcb_randr_set_config_t as set_config_t,
  xcb_randr_set_crtc_config as set_crtc_config,
  xcb_randr_set_crtc_config_cookie_t as set_crtc_config_cookie_t,
  xcb_randr_set_crtc_config_reply as set_crtc_config_reply,
  xcb_randr_set_crtc_config_reply_t as set_crtc_config_reply_t,
  xcb_randr_set_crtc_config_request_t as set_crtc_config_request_t,
  xcb_randr_set_crtc_config_sizeof as set_crtc_config_sizeof,
  xcb_randr_set_crtc_config_unchecked as set_crtc_config_unchecked,
  xcb_randr_set_crtc_gamma as set_crtc_gamma,
  xcb_randr_set_crtc_gamma_blue as set_crtc_gamma_blue,
  xcb_randr_set_crtc_gamma_blue_end as set_crtc_gamma_blue_end,
  xcb_randr_set_crtc_gamma_blue_length as set_crtc_gamma_blue_length,
  xcb_randr_set_crtc_gamma_checked as set_crtc_gamma_checked,
  xcb_randr_set_crtc_gamma_green as set_crtc_gamma_green,
  xcb_randr_set_crtc_gamma_green_end as set_crtc_gamma_green_end,
  xcb_randr_set_crtc_gamma_green_length as set_crtc_gamma_green_length,
  xcb_randr_set_crtc_gamma_red as set_crtc_gamma_red,
  xcb_randr_set_crtc_gamma_red_end as set_crtc_gamma_red_end,
  xcb_randr_set_crtc_gamma_red_length as set_crtc_gamma_red_length,
  xcb_randr_set_crtc_gamma_request_t as set_crtc_gamma_request_t,
  xcb_randr_set_crtc_gamma_sizeof as set_crtc_gamma_sizeof,
  xcb_randr_set_crtc_transform as set_crtc_transform,
  xcb_randr_set_crtc_transform_checked as set_crtc_transform_checked,
  xcb_randr_set_crtc_transform_filter_name as set_crtc_transform_filter_name,
  xcb_randr_set_crtc_transform_filter_name_end as set_crtc_transform_filter_name_end,
  xcb_randr_set_crtc_transform_filter_name_length as set_crtc_transform_filter_name_length,
  xcb_randr_set_crtc_transform_filter_params as set_crtc_transform_filter_params,
  xcb_randr_set_crtc_transform_filter_params_end as set_crtc_transform_filter_params_end,
  xcb_randr_set_crtc_transform_filter_params_length as set_crtc_transform_filter_params_length,
  xcb_randr_set_crtc_transform_request_t as set_crtc_transform_request_t,
  xcb_randr_set_crtc_transform_sizeof as set_crtc_transform_sizeof,
  xcb_randr_set_monitor as set_monitor,
  xcb_randr_set_monitor_checked as set_monitor_checked,
  xcb_randr_set_monitor_monitorinfo as set_monitor_monitorinfo,
  xcb_randr_set_monitor_request_t as set_monitor_request_t,
  xcb_randr_set_monitor_sizeof as set_monitor_sizeof,
  xcb_randr_set_output_primary as set_output_primary,
  xcb_randr_set_output_primary_checked as set_output_primary_checked,
  xcb_randr_set_output_primary_request_t as set_output_primary_request_t,
  xcb_randr_set_panning as set_panning,
  xcb_randr_set_panning_cookie_t as set_panning_cookie_t,
  xcb_randr_set_panning_reply as set_panning_reply,
  xcb_randr_set_panning_reply_t as set_panning_reply_t,
  xcb_randr_set_panning_request_t as set_panning_request_t,
  xcb_randr_set_panning_unchecked as set_panning_unchecked,
  xcb_randr_set_provider_offload_sink as set_provider_offload_sink,
  xcb_randr_set_provider_offload_sink_checked as set_provider_offload_sink_checked,
  xcb_randr_set_provider_offload_sink_request_t as set_provider_offload_sink_request_t,
  xcb_randr_set_provider_output_source as set_provider_output_source,
  xcb_randr_set_provider_output_source_checked as set_provider_output_source_checked,
  xcb_randr_set_provider_output_source_request_t as set_provider_output_source_request_t,
  xcb_randr_set_screen_config as set_screen_config,
  xcb_randr_set_screen_config_cookie_t as set_screen_config_cookie_t,
  xcb_randr_set_screen_config_reply as set_screen_config_reply,
  xcb_randr_set_screen_config_reply_t as set_screen_config_reply_t,
  xcb_randr_set_screen_config_request_t as set_screen_config_request_t,
  xcb_randr_set_screen_config_unchecked as set_screen_config_unchecked,
  xcb_randr_set_screen_size as set_screen_size,
  xcb_randr_set_screen_size_checked as set_screen_size_checked,
  xcb_randr_set_screen_size_request_t as set_screen_size_request_t,
  xcb_randr_transform_t as transform_t,
  XCB_RANDR_ADD_OUTPUT_MODE as ADD_OUTPUT_MODE,
  XCB_RANDR_BAD_CRTC as BAD_CRTC,
  XCB_RANDR_BAD_MODE as BAD_MODE,
  XCB_RANDR_BAD_OUTPUT as BAD_OUTPUT,
  XCB_RANDR_BAD_PROVIDER as BAD_PROVIDER,
  XCB_RANDR_CHANGE_OUTPUT_PROPERTY as CHANGE_OUTPUT_PROPERTY,
  XCB_RANDR_CHANGE_PROVIDER_PROPERTY as CHANGE_PROVIDER_PROPERTY,
  XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY as CONFIGURE_OUTPUT_PROPERTY,
  XCB_RANDR_CONFIGURE_PROVIDER_PROPERTY as CONFIGURE_PROVIDER_PROPERTY,
  XCB_RANDR_CREATE_LEASE as CREATE_LEASE,
  XCB_RANDR_CREATE_MODE as CREATE_MODE,
  XCB_RANDR_DELETE_MONITOR as DELETE_MONITOR,
  XCB_RANDR_DELETE_OUTPUT_MODE as DELETE_OUTPUT_MODE,
  XCB_RANDR_DELETE_OUTPUT_PROPERTY as DELETE_OUTPUT_PROPERTY,
  XCB_RANDR_DELETE_PROVIDER_PROPERTY as DELETE_PROVIDER_PROPERTY,
  XCB_RANDR_DESTROY_MODE as DESTROY_MODE,
  XCB_RANDR_FREE_LEASE as FREE_LEASE,
  XCB_RANDR_GET_CRTC_GAMMA as GET_CRTC_GAMMA,
  XCB_RANDR_GET_CRTC_GAMMA_SIZE as GET_CRTC_GAMMA_SIZE,
  XCB_RANDR_GET_CRTC_INFO as GET_CRTC_INFO,
  XCB_RANDR_GET_CRTC_TRANSFORM as GET_CRTC_TRANSFORM,
  XCB_RANDR_GET_MONITORS as GET_MONITORS,
  XCB_RANDR_GET_OUTPUT_INFO as GET_OUTPUT_INFO,
  XCB_RANDR_GET_OUTPUT_PRIMARY as GET_OUTPUT_PRIMARY,
  XCB_RANDR_GET_OUTPUT_PROPERTY as GET_OUTPUT_PROPERTY,
  XCB_RANDR_GET_PANNING as GET_PANNING,
  XCB_RANDR_GET_PROVIDERS as GET_PROVIDERS,
  XCB_RANDR_GET_PROVIDER_INFO as GET_PROVIDER_INFO,
  XCB_RANDR_GET_PROVIDER_PROPERTY as GET_PROVIDER_PROPERTY,
  XCB_RANDR_GET_SCREEN_INFO as GET_SCREEN_INFO,
  XCB_RANDR_GET_SCREEN_RESOURCES as GET_SCREEN_RESOURCES,
  XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT as GET_SCREEN_RESOURCES_CURRENT,
  XCB_RANDR_GET_SCREEN_SIZE_RANGE as GET_SCREEN_SIZE_RANGE,
  XCB_RANDR_LIST_OUTPUT_PROPERTIES as LIST_OUTPUT_PROPERTIES,
  XCB_RANDR_LIST_PROVIDER_PROPERTIES as LIST_PROVIDER_PROPERTIES,
  XCB_RANDR_MAJOR_VERSION as MAJOR_VERSION,
  XCB_RANDR_MINOR_VERSION as MINOR_VERSION,
  XCB_RANDR_NOTIFY as NOTIFY,
  XCB_RANDR_QUERY_OUTPUT_PROPERTY as QUERY_OUTPUT_PROPERTY,
  XCB_RANDR_QUERY_PROVIDER_PROPERTY as QUERY_PROVIDER_PROPERTY,
  XCB_RANDR_QUERY_VERSION as QUERY_VERSION,
  XCB_RANDR_SCREEN_CHANGE_NOTIFY as SCREEN_CHANGE_NOTIFY,
  XCB_RANDR_SELECT_INPUT as SELECT_INPUT,
  XCB_RANDR_SET_CRTC_CONFIG as SET_CRTC_CONFIG,
  XCB_RANDR_SET_CRTC_GAMMA as SET_CRTC_GAMMA,
  XCB_RANDR_SET_CRTC_TRANSFORM as SET_CRTC_TRANSFORM,
  XCB_RANDR_SET_MONITOR as SET_MONITOR,
  XCB_RANDR_SET_OUTPUT_PRIMARY as SET_OUTPUT_PRIMARY,
  XCB_RANDR_SET_PANNING as SET_PANNING,
  XCB_RANDR_SET_PROVIDER_OFFLOAD_SINK as SET_PROVIDER_OFFLOAD_SINK,
  XCB_RANDR_SET_PROVIDER_OUTPUT_SOURCE as SET_PROVIDER_OUTPUT_SOURCE,
  XCB_RANDR_SET_SCREEN_CONFIG as SET_SCREEN_CONFIG,
  XCB_RANDR_SET_SCREEN_SIZE as SET_SCREEN_SIZE,
};