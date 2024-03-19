#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcie_pf_vendor_id_and_device_id: PciePfVendorIdAndDeviceId,
    pcie_pf_command_and_status: PciePfCommandAndStatus,
    pcie_pf_revision_id_and_class_code: PciePfRevisionIdAndClassCode,
    pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s:
        PciePfBistHeaderTypeLatencyTimerAndCacheLineSizeS,
    pcie_pf_base_address_0: PciePfBaseAddress0,
    pcie_pf_base_address_1: PciePfBaseAddress1,
    pcie_pf_base_address_2: PciePfBaseAddress2,
    pcie_pf_base_address_3: PciePfBaseAddress3,
    pcie_pf_base_address_4: PciePfBaseAddress4,
    pcie_pf_base_address_5: PciePfBaseAddress5,
    _reserved10: [u8; 0x04],
    pcie_pf_subsystem_vendor_id_and_subsystem_id: PciePfSubsystemVendorIdAndSubsystemId,
    _reserved11: [u8; 0x04],
    pcie_pf_capabilities_pointer: PciePfCapabilitiesPointer,
    _reserved12: [u8; 0x04],
    pcie_pf_interrupt_line_and_interrupt_pin: PciePfInterruptLineAndInterruptPin,
    _reserved13: [u8; 0x40],
    pcie_pf_power_management_capabilities: PciePfPowerManagementCapabilities,
    pcie_pf_power_management_control_status_report: PciePfPowerManagementControlStatusReport,
    _reserved15: [u8; 0x08],
    pcie_pf_msi_control: PciePfMsiControl,
    pcie_pf_msi_message_low_address: PciePfMsiMessageLowAddress,
    pcie_pf_msi_message_high_address: PciePfMsiMessageHighAddress,
    pcie_pf_msi_message_data: PciePfMsiMessageData,
    pcie_pf_msi_mask: PciePfMsiMask,
    pcie_pf_msi_pending_bits: PciePfMsiPendingBits,
    _reserved21: [u8; 0x08],
    pcie_pf_msi_x_control: PciePfMsiXControl,
    pcie_pf_msi_x_table_offset: PciePfMsiXTableOffset,
    pcie_pf_msi_x_pending_interrupt: PciePfMsiXPendingInterrupt,
    _reserved24: [u8; 0x04],
    pcie_pf_pci_express_capability_list: PciePfPciExpressCapabilityList,
    pcie_pf_pci_express_device_capabilities: PciePfPciExpressDeviceCapabilities,
    pcie_pf_pci_express_device_control_and_status: PciePfPciExpressDeviceControlAndStatus,
    pcie_pf_link_capabilities: PciePfLinkCapabilities,
    pcie_pf_link_control_and_status: PciePfLinkControlAndStatus,
    _reserved29: [u8; 0x10],
    pcie_pf_pci_express_device_capabilities_2: PciePfPciExpressDeviceCapabilities2,
    pcie_pf_pci_express_device_control_and_status_2: PciePfPciExpressDeviceControlAndStatus2,
    pcie_pf_link_capabilities_2: PciePfLinkCapabilities2,
    pcie_pf_link_control_and_status_2: PciePfLinkControlAndStatus2,
    _reserved33: [u8; 0x0c],
    pcie_pf_advanced_error_reporting_aer_enhanced_capability_header:
        PciePfAdvancedErrorReportingAerEnhancedCapabilityHeader,
    pcie_pf_uncorrectable_error_status: PciePfUncorrectableErrorStatus,
    pcie_pf_uncorrectable_error_mask: PciePfUncorrectableErrorMask,
    pcie_pf_uncorrectable_error_severity: PciePfUncorrectableErrorSeverity,
    pcie_pf_correctable_error_status: PciePfCorrectableErrorStatus,
    pcie_pf_correctable_error_mask: PciePfCorrectableErrorMask,
    pcie_pf_advanced_error_capabilities_and_control: PciePfAdvancedErrorCapabilitiesAndControl,
    pcie_pf_header_log_0: PciePfHeaderLog0,
    pcie_pf_header_log_1: PciePfHeaderLog1,
    pcie_pf_header_log_2: PciePfHeaderLog2,
    pcie_pf_header_log_3: PciePfHeaderLog3,
    _reserved44: [u8; 0x14],
    pcie_pf_ari_extended_capability_header: PciePfAriExtendedCapabilityHeader,
    pcie_pf_ari_capability_and_ari_control: PciePfAriCapabilityAndAriControl,
    _reserved46: [u8; 0x18],
    pcie_pf_power_budgeting_enhanced_capability_header:
        PciePfPowerBudgetingEnhancedCapabilityHeader,
    pcie_pf_power_budgeting_data_select: PciePfPowerBudgetingDataSelect,
    pcie_pf_power_budgeting_data: PciePfPowerBudgetingData,
    pcie_pf_power_budget_capability: PciePfPowerBudgetCapability,
    _reserved50: [u8; 0x10],
    pcie_pf_resizable_bar_extended_capability_header: PciePfResizableBarExtendedCapabilityHeader,
    pcie_pf_resizable_bar_capability_0: PciePfResizableBarCapability0,
    pcie_pf_resizable_bar_control_0: PciePfResizableBarControl0,
    pcie_pf_resizable_bar_capability_1: PciePfResizableBarCapability1,
    pcie_pf_resizable_bar_control_1: PciePfResizableBarControl1,
    pcie_pf_resizable_bar_capability_2: PciePfResizableBarCapability2,
    pcie_pf_resizable_bar_control_2: PciePfResizableBarControl2,
    pcie_pf_resizable_bar_capability_3: PciePfResizableBarCapability3,
    pcie_pf_resizable_bar_control_3: PciePfResizableBarControl3,
    pcie_pf_resizable_bar_capability_4: PciePfResizableBarCapability4,
    pcie_pf_resizable_bar_control_4: PciePfResizableBarControl4,
    pcie_pf_resizable_bar_capability_5: PciePfResizableBarCapability5,
    pcie_pf_resizable_bar_control_5: PciePfResizableBarControl5,
    _reserved63: [u8; 0x04],
    pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header:
        PciePfLatencyToleranceReportingLtrExtendedCapabilityHeader,
    pcie_pf_ltr_max_snoop_max_no_snoop_latency: PciePfLtrMaxSnoopMaxNoSnoopLatency,
    pcie_pf_dpa_extended_capability_header: PciePfDpaExtendedCapabilityHeader,
    pcie_pf_dpa_capability: PciePfDpaCapability,
    pcie_pf_dpa_latency_indicator: PciePfDpaLatencyIndicator,
    pcie_pf_dpa_control_and_status_s: PciePfDpaControlAndStatusS,
    pcie_pf_dynamic_power_allocation_array_0: PciePfDynamicPowerAllocationArray0,
    pcie_pf_dynamic_power_allocation_array_1: PciePfDynamicPowerAllocationArray1,
    _reserved71: [u8; 0x28],
    pcie_pf_sr_iov_extended_capability_header: PciePfSrIovExtendedCapabilityHeader,
    pcie_pf_sr_iov_capabilities: PciePfSrIovCapabilities,
    pcie_pf_sr_iov_control_and_status_s: PciePfSrIovControlAndStatusS,
    pcie_pf_initial_vfs_total_vfs: PciePfInitialVfsTotalVfs,
    pcie_pf_function_dependency_link_numvfs: PciePfFunctionDependencyLinkNumvfs,
    pcie_pf_vf_offset_stride: PciePfVfOffsetStride,
    pcie_pf_vf_device_id: PciePfVfDeviceId,
    pcie_pf_supported_page_sizes: PciePfSupportedPageSizes,
    pcie_pf_system_page_size: PciePfSystemPageSize,
    pcie_pf_vf_base_address_0: PciePfVfBaseAddress0,
    pcie_pf_vf_base_address_1: PciePfVfBaseAddress1,
    pcie_pf_vf_base_address_2: PciePfVfBaseAddress2,
    pcie_pf_vf_base_address_3: PciePfVfBaseAddress3,
    pcie_pf_vf_base_address_4: PciePfVfBaseAddress4,
    pcie_pf_vf_base_address_5: PciePfVfBaseAddress5,
    pcie_pf_vf_migration_state_array_offset: PciePfVfMigrationStateArrayOffset,
    _reserved87: [u8; 0x34],
    pcie_pf_tph_requester_extended_capability_header: PciePfTphRequesterExtendedCapabilityHeader,
    pcie_pf_tph_requester_capability: PciePfTphRequesterCapability,
    pcie_pf_tph_requester_control: PciePfTphRequesterControl,
    pcie_pf_tph_st_table_0: PciePfTphStTable0,
    pcie_pf_tph_st_table_1: PciePfTphStTable1,
    pcie_pf_tph_st_table_2: PciePfTphStTable2,
    pcie_pf_tph_st_table_3: PciePfTphStTable3,
    _reserved94: [u8; 0x0670],
    pcie_pf_l1_pm_substates_extended_capability_header: PciePfL1PmSubstatesExtendedCapabilityHeader,
    pcie_pf_l1_pm_substates_capabilities: PciePfL1PmSubstatesCapabilities,
    pcie_pf_l1_pm_substates_control_1: PciePfL1PmSubstatesControl1,
    pcie_pf_l1_pm_substates_control_2: PciePfL1PmSubstatesControl2,
    _reserved98: [u8; 0xf6f0],
    pcie_vf_vendor_id_and_device_id: PcieVfVendorIdAndDeviceId,
    pcie_vf_command_and_status: PcieVfCommandAndStatus,
    pcie_vf_revision_id_and_class_code: PcieVfRevisionIdAndClassCode,
    pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s:
        PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeS,
    pcie_vf_base_address_0: PcieVfBaseAddress0,
    pcie_vf_base_address_1: PcieVfBaseAddress1,
    pcie_vf_base_address_2: PcieVfBaseAddress2,
    pcie_vf_base_address_3: PcieVfBaseAddress3,
    pcie_vf_base_address_4: PcieVfBaseAddress4,
    pcie_vf_base_address_5: PcieVfBaseAddress5,
    _reserved108: [u8; 0x04],
    pcie_vf_subsystem_vendor_id_and_subsystem_id: PcieVfSubsystemVendorIdAndSubsystemId,
    pcie_vf_expansion_rom_base_address: PcieVfExpansionRomBaseAddress,
    pcie_vf_capabilities_pointer: PcieVfCapabilitiesPointer,
    _reserved111: [u8; 0x04],
    pcie_vf_interrupt_line_and_interrupt_pin: PcieVfInterruptLineAndInterruptPin,
    _reserved112: [u8; 0x40],
    pcie_vf_power_management_capabilities: PcieVfPowerManagementCapabilities,
    pcie_vf_power_management_control_status_report: PcieVfPowerManagementControlStatusReport,
    _reserved114: [u8; 0x08],
    pcie_vf_msi_control: PcieVfMsiControl,
    pcie_vf_msi_message_low_address: PcieVfMsiMessageLowAddress,
    pcie_vf_msi_message_high_address: PcieVfMsiMessageHighAddress,
    pcie_vf_msi_message_data: PcieVfMsiMessageData,
    pcie_vf_msi_mask: PcieVfMsiMask,
    pcie_vf_msi_pending_bits: PcieVfMsiPendingBits,
    _reserved120: [u8; 0x08],
    pcie_vf_msi_x_control: PcieVfMsiXControl,
    pcie_vf_msi_x_table_offset: PcieVfMsiXTableOffset,
    pcie_vf_msi_x_pending_interrupt: PcieVfMsiXPendingInterrupt,
    _reserved123: [u8; 0x04],
    pcie_vf_pci_express_capability_list: PcieVfPciExpressCapabilityList,
    pcie_vf_pci_express_device_capabilities: PcieVfPciExpressDeviceCapabilities,
    pcie_vf_pci_express_device_control_and_status: PcieVfPciExpressDeviceControlAndStatus,
    pcie_vf_link_capabilities: PcieVfLinkCapabilities,
    _reserved127: [u8; 0x14],
    pcie_vf_pci_express_device_capabilities_2: PcieVfPciExpressDeviceCapabilities2,
    _reserved128: [u8; 0x18],
    pcie_vf_advanced_error_reporting_aer_enhanced_capability_header:
        PcieVfAdvancedErrorReportingAerEnhancedCapabilityHeader,
    pcie_vf_uncorrectable_error_status: PcieVfUncorrectableErrorStatus,
    pcie_vf_uncorrectable_error_mask: PcieVfUncorrectableErrorMask,
    pcie_vf_uncorrectable_error_severity: PcieVfUncorrectableErrorSeverity,
    pcie_vf_correctable_error_status: PcieVfCorrectableErrorStatus,
    pcie_vf_correctable_error_mask: PcieVfCorrectableErrorMask,
    pcie_vf_advanced_error_capabilities_and_control: PcieVfAdvancedErrorCapabilitiesAndControl,
    pcie_vf_header_log_0: PcieVfHeaderLog0,
    pcie_vf_header_log_1: PcieVfHeaderLog1,
    pcie_vf_header_log_2: PcieVfHeaderLog2,
    pcie_vf_header_log_3: PcieVfHeaderLog3,
    _reserved139: [u8; 0x14],
    pcie_vf_ari_extended_capability_header: PcieVfAriExtendedCapabilityHeader,
    pcie_vf_ari_capability_and_ari_control: PcieVfAriCapabilityAndAriControl,
    _reserved141: [u8; 0x012c],
    pcie_vf_tph_requester_enhanced_capability_header: PcieVfTphRequesterEnhancedCapabilityHeader,
    pcie_vf_tph_requester_capability: PcieVfTphRequesterCapability,
    pcie_vf_tph_requester_control: PcieVfTphRequesterControl,
    pcie_vf_tph_st_table_0: PcieVfTphStTable0,
    pcie_vf_tph_st_table_1: PcieVfTphStTable1,
    pcie_vf_tph_st_table_2: PcieVfTphStTable2,
    _reserved147: [u8; 0x000e_fd74],
    pcie_lm_physical_layer_configuration_0: PcieLmPhysicalLayerConfiguration0,
    pcie_lm_physical_layer_configuration_1: PcieLmPhysicalLayerConfiguration1,
    pcie_lm_data_link_layer_timer_configuration: PcieLmDataLinkLayerTimerConfiguration,
    pcie_lm_receive_credit_limit_0_vc0: PcieLmReceiveCreditLimit0Vc0,
    pcie_lm_receive_credit_limit_1_vc0: PcieLmReceiveCreditLimit1Vc0,
    pcie_lm_transmit_credit_limit_0_vc0: PcieLmTransmitCreditLimit0Vc0,
    pcie_lm_transmit_credit_limit_1_vc0: PcieLmTransmitCreditLimit1Vc0,
    pcie_lm_transmit_credit_update_interval_configuration_0:
        PcieLmTransmitCreditUpdateIntervalConfiguration0,
    pcie_lm_transmit_credit_update_interval_configuration_1:
        PcieLmTransmitCreditUpdateIntervalConfiguration1,
    pcie_lm_l0s_timeout_limit: PcieLmL0sTimeoutLimit,
    pcie_lm_transmit_tlp_count: PcieLmTransmitTlpCount,
    pcie_lm_transmit_tlp_payload_dword_count: PcieLmTransmitTlpPayloadDwordCount,
    pcie_lm_receive_tlp_count: PcieLmReceiveTlpCount,
    pcie_lm_receive_tlp_payload_dword_count: PcieLmReceiveTlpPayloadDwordCount,
    pcie_lm_completion_timeout_limit_0: PcieLmCompletionTimeoutLimit0,
    pcie_lm_completion_timeout_limit_1: PcieLmCompletionTimeoutLimit1,
    pcie_lm_l1_state_re_entry_delay: PcieLmL1StateReEntryDelay,
    pcie_lm_vendor_id: PcieLmVendorId,
    pcie_lm_aspm_l1_entry_timeout_delay: PcieLmAspmL1EntryTimeoutDelay,
    pcie_lm_pme_turnoff_ack_delay: PcieLmPmeTurnoffAckDelay,
    pcie_lm_linkwidth_control: PcieLmLinkwidthControl,
    _reserved168: [u8; 0x20],
    pcie_lm_sris_control: PcieLmSrisControl,
    _reserved169: [u8; 0x88],
    pcie_lm_shadow_register_header_log_0: PcieLmShadowRegisterHeaderLog0,
    pcie_lm_shadow_register_header_log_1: PcieLmShadowRegisterHeaderLog1,
    pcie_lm_shadow_register_header_log_2: PcieLmShadowRegisterHeaderLog2,
    pcie_lm_shadow_register_header_log_3: PcieLmShadowRegisterHeaderLog3,
    pcie_lm_shadow_register_function_number: PcieLmShadowRegisterFunctionNumber,
    pcie_lm_shadow_ur_error: PcieLmShadowUrError,
    _reserved175: [u8; 0xe8],
    pcie_lm_negotiated_lane_map: PcieLmNegotiatedLaneMap,
    pcie_lm_receive_fts_count: PcieLmReceiveFtsCount,
    pcie_lm_debug_mux_control: PcieLmDebugMuxControl,
    pcie_lm_local_error_and_status: PcieLmLocalErrorAndStatus,
    pcie_lm_local_interrupt_mask: PcieLmLocalInterruptMask,
    pcie_lm_lcrc_error_count: PcieLmLcrcErrorCount,
    pcie_lm_ecc_correctable_error_count: PcieLmEccCorrectableErrorCount,
    pcie_lm_ltr_snoop_no_snoop_latency: PcieLmLtrSnoopNoSnoopLatency,
    pcie_lm_ltr_message_generation_control: PcieLmLtrMessageGenerationControl,
    pcie_lm_pme_service_timeout_delay: PcieLmPmeServiceTimeoutDelay,
    pcie_lm_root_port_requestor_id: PcieLmRootPortRequestorId,
    pcie_lm_end_point_bus_and_device_number: PcieLmEndPointBusAndDeviceNumber,
    _reserved187: [u8; 0x10],
    pcie_lm_physical_function_bar_configuration_0: PcieLmPhysicalFunctionBarConfiguration0,
    pcie_lm_physical_function_bar_configuration_1: PcieLmPhysicalFunctionBarConfiguration1,
    _reserved189: [u8; 0x38],
    pcie_lm_virtual_function_bar_configuration_0: PcieLmVirtualFunctionBarConfiguration0,
    pcie_lm_virtual_function_bar_configuration_1: PcieLmVirtualFunctionBarConfiguration1,
    _reserved191: [u8; 0x38],
    pcie_lm_physical_function_configuration: PcieLmPhysicalFunctionConfiguration,
    _reserved192: [u8; 0x3c],
    pcie_lm_root_complex_bar_configuration: PcieLmRootComplexBarConfiguration,
    _reserved193: [u8; 0x000f_fcfc],
    pcie_rc_vendor_id_and_device_id: PcieRcVendorIdAndDeviceId,
    pcie_rc_command_and_status: PcieRcCommandAndStatus,
    pcie_rc_revision_id_and_class_code: PcieRcRevisionIdAndClassCode,
    pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s:
        PcieRcBistHeaderTypeLatencyTimerAndCacheLineSizeS,
    pcie_rc_root_complex_base_address_0: PcieRcRootComplexBaseAddress0,
    pcie_rc_root_complex_base_address_1: PcieRcRootComplexBaseAddress1,
    pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer:
        PcieRcPrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer,
    pcie_rc_io_base_io_limit_secondary_status: PcieRcIoBaseIoLimitSecondaryStatus,
    pcie_rc_memory_base_memory_limit: PcieRcMemoryBaseMemoryLimit,
    pcie_rc_prefetchable_memory_base_prefetchable_memory_limit:
        PcieRcPrefetchableMemoryBasePrefetchableMemoryLimit,
    pcie_rc_prefetchable_base_upper: PcieRcPrefetchableBaseUpper,
    pcie_rc_prefetchable_limit_upper: PcieRcPrefetchableLimitUpper,
    pcie_rc_io_base_upper_io_limit_upper: PcieRcIoBaseUpperIoLimitUpper,
    pcie_rc_capabilities_pointer: PcieRcCapabilitiesPointer,
    _reserved207: [u8; 0x04],
    pcie_rc_interrupt_line_interrupt_pin_and_bridge_control:
        PcieRcInterruptLineInterruptPinAndBridgeControl,
    _reserved208: [u8; 0x40],
    pcie_rc_power_management_capabilities: PcieRcPowerManagementCapabilities,
    pcie_rc_power_management_control_status_report: PcieRcPowerManagementControlStatusReport,
    _reserved210: [u8; 0x08],
    pcie_rc_msi_control: PcieRcMsiControl,
    pcie_rc_msi_message_low_address: PcieRcMsiMessageLowAddress,
    pcie_rc_msi_message_high_address: PcieRcMsiMessageHighAddress,
    pcie_rc_msi_message_data: PcieRcMsiMessageData,
    pcie_rc_msi_mask: PcieRcMsiMask,
    pcie_rc_msi_pending_bits: PcieRcMsiPendingBits,
    _reserved216: [u8; 0x08],
    pcie_rc_msi_x_control: PcieRcMsiXControl,
    pcie_rc_msi_x_table_offset: PcieRcMsiXTableOffset,
    pcie_rc_msi_x_pending_interrupt: PcieRcMsiXPendingInterrupt,
    _reserved219: [u8; 0x04],
    pcie_rc_pci_express_capability_list: PcieRcPciExpressCapabilityList,
    pcie_rc_pci_express_device_capabilities: PcieRcPciExpressDeviceCapabilities,
    pcie_rc_pci_express_device_control_and_status: PcieRcPciExpressDeviceControlAndStatus,
    pcie_rc_link_capabilities: PcieRcLinkCapabilities,
    pcie_rc_link_control_and_status: PcieRcLinkControlAndStatus,
    pcie_rc_slot_capability: PcieRcSlotCapability,
    pcie_rc_slot_control_and_status: PcieRcSlotControlAndStatus,
    pcie_rc_root_control_and_capability: PcieRcRootControlAndCapability,
    pcie_rc_root_status: PcieRcRootStatus,
    pcie_rc_pci_express_device_capabilities_2: PcieRcPciExpressDeviceCapabilities2,
    pcie_rc_pci_express_device_control_and_status_2: PcieRcPciExpressDeviceControlAndStatus2,
    pcie_rc_link_capabilities_2: PcieRcLinkCapabilities2,
    pcie_rc_link_control_and_status_2: PcieRcLinkControlAndStatus2,
    _reserved232: [u8; 0x0c],
    pcie_rc_advanced_error_reporting_aer_enhanced_capability_header:
        PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeader,
    pcie_rc_uncorrectable_error_status: PcieRcUncorrectableErrorStatus,
    pcie_rc_uncorrectable_error_mask: PcieRcUncorrectableErrorMask,
    pcie_rc_uncorrectable_error_severity: PcieRcUncorrectableErrorSeverity,
    pcie_rc_correctable_error_status: PcieRcCorrectableErrorStatus,
    pcie_rc_correctable_error_mask: PcieRcCorrectableErrorMask,
    pcie_rc_advanced_error_capabilities_and_control: PcieRcAdvancedErrorCapabilitiesAndControl,
    pcie_rc_header_log_0: PcieRcHeaderLog0,
    pcie_rc_header_log_1: PcieRcHeaderLog1,
    pcie_rc_header_log_2: PcieRcHeaderLog2,
    pcie_rc_header_log_3: PcieRcHeaderLog3,
    pcie_rc_root_error_command: PcieRcRootErrorCommand,
    pcie_rc_root_error_status: PcieRcRootErrorStatus,
    pcie_rc_error_source_identification: PcieRcErrorSourceIdentification,
    _reserved246: [u8; 0x0148],
    pcie_rc_tph_st_table_3: PcieRcTphStTable3,
    _reserved247: [u8; 0x067c],
    pcie_rc_l1_pm_substates_extended_capability_header: PcieRcL1PmSubstatesExtendedCapabilityHeader,
    pcie_rc_l1_pm_substates_capabilities: PcieRcL1PmSubstatesCapabilities,
    pcie_rc_l1_pm_substates_control_1: PcieRcL1PmSubstatesControl1,
    pcie_rc_l1_pm_substates_control_2: PcieRcL1PmSubstatesControl2,
    _reserved251: [u8; 0x001f_f6f0],
    pcie_at_ob_outbound_region_address_0: PcieAtObOutboundRegionAddress0,
    pcie_at_ob_outbound_region_address_1: PcieAtObOutboundRegionAddress1,
    pcie_at_ob_outbound_region_descriptor_0: PcieAtObOutboundRegionDescriptor0,
    pcie_at_ob_outbound_region_descriptor_1: PcieAtObOutboundRegionDescriptor1,
    pcie_at_ob_outbound_region_descriptor_2: PcieAtObOutboundRegionDescriptor2,
    pcie_at_ob_outbound_region_descriptor_3: PcieAtObOutboundRegionDescriptor3,
    _reserved257: [u8; 0x07e8],
    pcie_at_rp_ib_rp_inbound_bar_address_translation_0: PcieAtRpIbRpInboundBarAddressTranslation0,
    pcie_at_rp_ib_rp_inbound_bar_address_translation_1: PcieAtRpIbRpInboundBarAddressTranslation1,
    _reserved259: [u8; 0x1c],
    pcie_at_rp_ib_link_down_indication_bit: PcieAtRpIbLinkDownIndicationBit,
    pcie_at_ep_ib_ep_inbound_bar_address_translation_0: PcieAtEpIbEpInboundBarAddressTranslation0,
    pcie_at_ep_ib_ep_inbound_bar_address_translation_1: PcieAtEpIbEpInboundBarAddressTranslation1,
    _reserved262: [u8; 0x001f_f7d0],
    pcie_dma_channel_0_control: PcieDmaChannel0Control,
    pcie_dma_channel_0_start_pointer_lower: PcieDmaChannel0StartPointerLower,
    pcie_dma_channel_0_start_pointer_upper: PcieDmaChannel0StartPointerUpper,
    pcie_dma_channel_0_attribute_lower: PcieDmaChannel0AttributeLower,
    pcie_dma_channel_0_attribute_upper: PcieDmaChannel0AttributeUpper,
    pcie_dma_channel_1_control: PcieDmaChannel1Control,
    pcie_dma_channel_1_start_pointer_lower: PcieDmaChannel1StartPointerLower,
    pcie_dma_channel_1_start_pointer_upper: PcieDmaChannel1StartPointerUpper,
    pcie_dma_channel_1_attribute_lower: PcieDmaChannel1AttributeLower,
    pcie_dma_channel_1_attribute_upper: PcieDmaChannel1AttributeUpper,
    _reserved272: [u8; 0x78],
    pcie_dma_interrupt: PcieDmaInterrupt,
    pcie_dma_interrupt_enable: PcieDmaInterruptEnable,
    pcie_dma_interrupt_disable: PcieDmaInterruptDisable,
    pcie_dma_inbound_buffer_uncorrected_ecc_errors: PcieDmaInboundBufferUncorrectedEccErrors,
    pcie_dma_inbound_buffer_corrected_ecc_errors: PcieDmaInboundBufferCorrectedEccErrors,
    pcie_dma_outbound_buffer_uncorrected_ecc_errors: PcieDmaOutboundBufferUncorrectedEccErrors,
    pcie_dma_outbound_buffer_corrected_ecc_errors: PcieDmaOutboundBufferCorrectedEccErrors,
    _reserved279: [u8; 0x3c],
    pcie_dma_capability_and_version: PcieDmaCapabilityAndVersion,
    pcie_dma_configuration: PcieDmaConfiguration,
}
impl RegisterBlock {
    #[doc = "0x00 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_vendor_id_and_device_id(&self) -> &PciePfVendorIdAndDeviceId {
        &self.pcie_pf_vendor_id_and_device_id
    }
    #[doc = "0x04 - Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position."]
    #[inline(always)]
    pub const fn pcie_pf_command_and_status(&self) -> &PciePfCommandAndStatus {
        &self.pcie_pf_command_and_status
    }
    #[doc = "0x08 - Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus"]
    #[inline(always)]
    pub const fn pcie_pf_revision_id_and_class_code(&self) -> &PciePfRevisionIdAndClassCode {
        &self.pcie_pf_revision_id_and_class_code
    }
    #[doc = "0x0c - BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &PciePfBistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x10 - Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_0(&self) -> &PciePfBaseAddress0 {
        &self.pcie_pf_base_address_0
    }
    #[doc = "0x14 - Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_1(&self) -> &PciePfBaseAddress1 {
        &self.pcie_pf_base_address_1
    }
    #[doc = "0x18 - Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_2(&self) -> &PciePfBaseAddress2 {
        &self.pcie_pf_base_address_2
    }
    #[doc = "0x1c - Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_3(&self) -> &PciePfBaseAddress3 {
        &self.pcie_pf_base_address_3
    }
    #[doc = "0x20 - Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_4(&self) -> &PciePfBaseAddress4 {
        &self.pcie_pf_base_address_4
    }
    #[doc = "0x24 - Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_base_address_5(&self) -> &PciePfBaseAddress5 {
        &self.pcie_pf_base_address_5
    }
    #[doc = "0x2c - Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_subsystem_vendor_id_and_subsystem_id(
        &self,
    ) -> &PciePfSubsystemVendorIdAndSubsystemId {
        &self.pcie_pf_subsystem_vendor_id_and_subsystem_id
    }
    #[doc = "0x34 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_capabilities_pointer(&self) -> &PciePfCapabilitiesPointer {
        &self.pcie_pf_capabilities_pointer
    }
    #[doc = "0x3c - Interrupt Line and Interrupt Pin Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_interrupt_line_and_interrupt_pin(
        &self,
    ) -> &PciePfInterruptLineAndInterruptPin {
        &self.pcie_pf_interrupt_line_and_interrupt_pin
    }
    #[doc = "0x80 - Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_pf_power_management_capabilities(
        &self,
    ) -> &PciePfPowerManagementCapabilities {
        &self.pcie_pf_power_management_capabilities
    }
    #[doc = "0x84 - Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_pf_power_management_control_status_report(
        &self,
    ) -> &PciePfPowerManagementControlStatusReport {
        &self.pcie_pf_power_management_control_status_report
    }
    #[doc = "0x90 - MSI Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_msi_control(&self) -> &PciePfMsiControl {
        &self.pcie_pf_msi_control
    }
    #[doc = "0x94 - MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_msi_message_low_address(&self) -> &PciePfMsiMessageLowAddress {
        &self.pcie_pf_msi_message_low_address
    }
    #[doc = "0x98 - MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_msi_message_high_address(&self) -> &PciePfMsiMessageHighAddress {
        &self.pcie_pf_msi_message_high_address
    }
    #[doc = "0x9c - MSI Message Data Register Hardwired to 0"]
    #[inline(always)]
    pub const fn pcie_pf_msi_message_data(&self) -> &PciePfMsiMessageData {
        &self.pcie_pf_msi_message_data
    }
    #[doc = "0xa0 - MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn pcie_pf_msi_mask(&self) -> &PciePfMsiMask {
        &self.pcie_pf_msi_mask
    }
    #[doc = "0xa4 - MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn pcie_pf_msi_pending_bits(&self) -> &PciePfMsiPendingBits {
        &self.pcie_pf_msi_pending_bits
    }
    #[doc = "0xb0 - MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_msi_x_control(&self) -> &PciePfMsiXControl {
        &self.pcie_pf_msi_x_control
    }
    #[doc = "0xb4 - MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_pf_msi_x_table_offset(&self) -> &PciePfMsiXTableOffset {
        &self.pcie_pf_msi_x_table_offset
    }
    #[doc = "0xb8 - MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_pf_msi_x_pending_interrupt(&self) -> &PciePfMsiXPendingInterrupt {
        &self.pcie_pf_msi_x_pending_interrupt
    }
    #[doc = "0xc0 - PCI Express Capability List Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_pci_express_capability_list(&self) -> &PciePfPciExpressCapabilityList {
        &self.pcie_pf_pci_express_capability_list
    }
    #[doc = "0xc4 - PCI Express Device Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_pci_express_device_capabilities(
        &self,
    ) -> &PciePfPciExpressDeviceCapabilities {
        &self.pcie_pf_pci_express_device_capabilities
    }
    #[doc = "0xc8 - PCI Express Device Control and Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_pci_express_device_control_and_status(
        &self,
    ) -> &PciePfPciExpressDeviceControlAndStatus {
        &self.pcie_pf_pci_express_device_control_and_status
    }
    #[doc = "0xcc - Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
    #[inline(always)]
    pub const fn pcie_pf_link_capabilities(&self) -> &PciePfLinkCapabilities {
        &self.pcie_pf_link_capabilities
    }
    #[doc = "0xd0 - Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_pf_link_control_and_status(&self) -> &PciePfLinkControlAndStatus {
        &self.pcie_pf_link_control_and_status
    }
    #[doc = "0xe4 - PCI Express Device Capabilities Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_pci_express_device_capabilities_2(
        &self,
    ) -> &PciePfPciExpressDeviceCapabilities2 {
        &self.pcie_pf_pci_express_device_capabilities_2
    }
    #[doc = "0xe8 - PCI Express Device Control and Status Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_pci_express_device_control_and_status_2(
        &self,
    ) -> &PciePfPciExpressDeviceControlAndStatus2 {
        &self.pcie_pf_pci_express_device_control_and_status_2
    }
    #[doc = "0xec - Link Capabilities Register 2 RSVD"]
    #[inline(always)]
    pub const fn pcie_pf_link_capabilities_2(&self) -> &PciePfLinkCapabilities2 {
        &self.pcie_pf_link_capabilities_2
    }
    #[doc = "0xf0 - Link Control and Status Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_link_control_and_status_2(&self) -> &PciePfLinkControlAndStatus2 {
        &self.pcie_pf_link_control_and_status_2
    }
    #[doc = "0x100 - Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_advanced_error_reporting_aer_enhanced_capability_header(
        &self,
    ) -> &PciePfAdvancedErrorReportingAerEnhancedCapabilityHeader {
        &self.pcie_pf_advanced_error_reporting_aer_enhanced_capability_header
    }
    #[doc = "0x104 - Uncorrectable Error Status Register (no description)"]
    #[inline(always)]
    pub const fn pcie_pf_uncorrectable_error_status(&self) -> &PciePfUncorrectableErrorStatus {
        &self.pcie_pf_uncorrectable_error_status
    }
    #[doc = "0x108 - Uncorrectable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_uncorrectable_error_mask(&self) -> &PciePfUncorrectableErrorMask {
        &self.pcie_pf_uncorrectable_error_mask
    }
    #[doc = "0x10c - Uncorrectable Error Severity Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_uncorrectable_error_severity(&self) -> &PciePfUncorrectableErrorSeverity {
        &self.pcie_pf_uncorrectable_error_severity
    }
    #[doc = "0x110 - Correctable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_correctable_error_status(&self) -> &PciePfCorrectableErrorStatus {
        &self.pcie_pf_correctable_error_status
    }
    #[doc = "0x114 - Correctable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_correctable_error_mask(&self) -> &PciePfCorrectableErrorMask {
        &self.pcie_pf_correctable_error_mask
    }
    #[doc = "0x118 - Advanced Error Capabilities and Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_advanced_error_capabilities_and_control(
        &self,
    ) -> &PciePfAdvancedErrorCapabilitiesAndControl {
        &self.pcie_pf_advanced_error_capabilities_and_control
    }
    #[doc = "0x11c - Header Log Register 0 First DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_pf_header_log_0(&self) -> &PciePfHeaderLog0 {
        &self.pcie_pf_header_log_0
    }
    #[doc = "0x120 - Header Log Register 1 Second DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_pf_header_log_1(&self) -> &PciePfHeaderLog1 {
        &self.pcie_pf_header_log_1
    }
    #[doc = "0x124 - Header Log Register 2 Third DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_pf_header_log_2(&self) -> &PciePfHeaderLog2 {
        &self.pcie_pf_header_log_2
    }
    #[doc = "0x128 - Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_pf_header_log_3(&self) -> &PciePfHeaderLog3 {
        &self.pcie_pf_header_log_3
    }
    #[doc = "0x140 - ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_ari_extended_capability_header(
        &self,
    ) -> &PciePfAriExtendedCapabilityHeader {
        &self.pcie_pf_ari_extended_capability_header
    }
    #[doc = "0x144 - ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_pf_ari_capability_and_ari_control(
        &self,
    ) -> &PciePfAriCapabilityAndAriControl {
        &self.pcie_pf_ari_capability_and_ari_control
    }
    #[doc = "0x160 - Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_power_budgeting_enhanced_capability_header(
        &self,
    ) -> &PciePfPowerBudgetingEnhancedCapabilityHeader {
        &self.pcie_pf_power_budgeting_enhanced_capability_header
    }
    #[doc = "0x164 - Power Budgeting Data Select Register (no description)"]
    #[inline(always)]
    pub const fn pcie_pf_power_budgeting_data_select(&self) -> &PciePfPowerBudgetingDataSelect {
        &self.pcie_pf_power_budgeting_data_select
    }
    #[doc = "0x168 - Power Budgeting Data Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_power_budgeting_data(&self) -> &PciePfPowerBudgetingData {
        &self.pcie_pf_power_budgeting_data
    }
    #[doc = "0x16c - Power Budget Capability Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_power_budget_capability(&self) -> &PciePfPowerBudgetCapability {
        &self.pcie_pf_power_budget_capability
    }
    #[doc = "0x180 - Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_extended_capability_header(
        &self,
    ) -> &PciePfResizableBarExtendedCapabilityHeader {
        &self.pcie_pf_resizable_bar_extended_capability_header
    }
    #[doc = "0x184 - Resizable BAR Capability Register 0 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_0(&self) -> &PciePfResizableBarCapability0 {
        &self.pcie_pf_resizable_bar_capability_0
    }
    #[doc = "0x188 - Resizable BAR Control Register 0 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_0(&self) -> &PciePfResizableBarControl0 {
        &self.pcie_pf_resizable_bar_control_0
    }
    #[doc = "0x18c - Resizable BAR Capability Register 1 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_1(&self) -> &PciePfResizableBarCapability1 {
        &self.pcie_pf_resizable_bar_capability_1
    }
    #[doc = "0x190 - Resizable BAR Control Register 1 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_1(&self) -> &PciePfResizableBarControl1 {
        &self.pcie_pf_resizable_bar_control_1
    }
    #[doc = "0x194 - Resizable BAR Capability Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_2(&self) -> &PciePfResizableBarCapability2 {
        &self.pcie_pf_resizable_bar_capability_2
    }
    #[doc = "0x198 - Resizable BAR Control Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_2(&self) -> &PciePfResizableBarControl2 {
        &self.pcie_pf_resizable_bar_control_2
    }
    #[doc = "0x19c - Resizable BAR Capability Register 3 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_3(&self) -> &PciePfResizableBarCapability3 {
        &self.pcie_pf_resizable_bar_capability_3
    }
    #[doc = "0x1a0 - Resizable BAR Control Register 3 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_3(&self) -> &PciePfResizableBarControl3 {
        &self.pcie_pf_resizable_bar_control_3
    }
    #[doc = "0x1a4 - Resizable BAR Capability Register 4 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_4(&self) -> &PciePfResizableBarCapability4 {
        &self.pcie_pf_resizable_bar_capability_4
    }
    #[doc = "0x1a8 - Resizable BAR Control Register 4 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_4(&self) -> &PciePfResizableBarControl4 {
        &self.pcie_pf_resizable_bar_control_4
    }
    #[doc = "0x1ac - Resizable BAR Capability Register 5 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_capability_5(&self) -> &PciePfResizableBarCapability5 {
        &self.pcie_pf_resizable_bar_capability_5
    }
    #[doc = "0x1b0 - Resizable BAR Control Register 5 Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_resizable_bar_control_5(&self) -> &PciePfResizableBarControl5 {
        &self.pcie_pf_resizable_bar_control_5
    }
    #[doc = "0x1b8 - Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header(
        &self,
    ) -> &PciePfLatencyToleranceReportingLtrExtendedCapabilityHeader {
        &self.pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header
    }
    #[doc = "0x1bc - LTR Max Snoop/Max No-Snoop Latency Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_ltr_max_snoop_max_no_snoop_latency(
        &self,
    ) -> &PciePfLtrMaxSnoopMaxNoSnoopLatency {
        &self.pcie_pf_ltr_max_snoop_max_no_snoop_latency
    }
    #[doc = "0x1c0 - DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_dpa_extended_capability_header(
        &self,
    ) -> &PciePfDpaExtendedCapabilityHeader {
        &self.pcie_pf_dpa_extended_capability_header
    }
    #[doc = "0x1c4 - DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register."]
    #[inline(always)]
    pub const fn pcie_pf_dpa_capability(&self) -> &PciePfDpaCapability {
        &self.pcie_pf_dpa_capability
    }
    #[doc = "0x1c8 - DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
    #[inline(always)]
    pub const fn pcie_pf_dpa_latency_indicator(&self) -> &PciePfDpaLatencyIndicator {
        &self.pcie_pf_dpa_latency_indicator
    }
    #[doc = "0x1cc - DPA Control and Status Registers Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_dpa_control_and_status_s(&self) -> &PciePfDpaControlAndStatusS {
        &self.pcie_pf_dpa_control_and_status_s
    }
    #[doc = "0x1d0 - Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub const fn pcie_pf_dynamic_power_allocation_array_0(
        &self,
    ) -> &PciePfDynamicPowerAllocationArray0 {
        &self.pcie_pf_dynamic_power_allocation_array_0
    }
    #[doc = "0x1d4 - Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub const fn pcie_pf_dynamic_power_allocation_array_1(
        &self,
    ) -> &PciePfDynamicPowerAllocationArray1 {
        &self.pcie_pf_dynamic_power_allocation_array_1
    }
    #[doc = "0x200 - SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_sr_iov_extended_capability_header(
        &self,
    ) -> &PciePfSrIovExtendedCapabilityHeader {
        &self.pcie_pf_sr_iov_extended_capability_header
    }
    #[doc = "0x204 - SR-IOV Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_sr_iov_capabilities(&self) -> &PciePfSrIovCapabilities {
        &self.pcie_pf_sr_iov_capabilities
    }
    #[doc = "0x208 - SR-IOV Control and Status Registers Not implemented."]
    #[inline(always)]
    pub const fn pcie_pf_sr_iov_control_and_status_s(&self) -> &PciePfSrIovControlAndStatusS {
        &self.pcie_pf_sr_iov_control_and_status_s
    }
    #[doc = "0x20c - Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
    #[inline(always)]
    pub const fn pcie_pf_initial_vfs_total_vfs(&self) -> &PciePfInitialVfsTotalVfs {
        &self.pcie_pf_initial_vfs_total_vfs
    }
    #[doc = "0x210 - Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_function_dependency_link_numvfs(
        &self,
    ) -> &PciePfFunctionDependencyLinkNumvfs {
        &self.pcie_pf_function_dependency_link_numvfs
    }
    #[doc = "0x214 - VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
    #[inline(always)]
    pub const fn pcie_pf_vf_offset_stride(&self) -> &PciePfVfOffsetStride {
        &self.pcie_pf_vf_offset_stride
    }
    #[doc = "0x218 - VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
    #[inline(always)]
    pub const fn pcie_pf_vf_device_id(&self) -> &PciePfVfDeviceId {
        &self.pcie_pf_vf_device_id
    }
    #[doc = "0x21c - Supported Page Sizes Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_supported_page_sizes(&self) -> &PciePfSupportedPageSizes {
        &self.pcie_pf_supported_page_sizes
    }
    #[doc = "0x220 - System Page Size Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_system_page_size(&self) -> &PciePfSystemPageSize {
        &self.pcie_pf_system_page_size
    }
    #[doc = "0x224 - VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_0(&self) -> &PciePfVfBaseAddress0 {
        &self.pcie_pf_vf_base_address_0
    }
    #[doc = "0x228 - VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_1(&self) -> &PciePfVfBaseAddress1 {
        &self.pcie_pf_vf_base_address_1
    }
    #[doc = "0x22c - VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_2(&self) -> &PciePfVfBaseAddress2 {
        &self.pcie_pf_vf_base_address_2
    }
    #[doc = "0x230 - VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_3(&self) -> &PciePfVfBaseAddress3 {
        &self.pcie_pf_vf_base_address_3
    }
    #[doc = "0x234 - VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_4(&self) -> &PciePfVfBaseAddress4 {
        &self.pcie_pf_vf_base_address_4
    }
    #[doc = "0x238 - VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_pf_vf_base_address_5(&self) -> &PciePfVfBaseAddress5 {
        &self.pcie_pf_vf_base_address_5
    }
    #[doc = "0x23c - VF Migration State Array Offset Register (no description)"]
    #[inline(always)]
    pub const fn pcie_pf_vf_migration_state_array_offset(
        &self,
    ) -> &PciePfVfMigrationStateArrayOffset {
        &self.pcie_pf_vf_migration_state_array_offset
    }
    #[doc = "0x274 - TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_tph_requester_extended_capability_header(
        &self,
    ) -> &PciePfTphRequesterExtendedCapabilityHeader {
        &self.pcie_pf_tph_requester_extended_capability_header
    }
    #[doc = "0x278 - TPH Requester Capability Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_tph_requester_capability(&self) -> &PciePfTphRequesterCapability {
        &self.pcie_pf_tph_requester_capability
    }
    #[doc = "0x27c - TPH Requester Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_pf_tph_requester_control(&self) -> &PciePfTphRequesterControl {
        &self.pcie_pf_tph_requester_control
    }
    #[doc = "0x280 - TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_pf_tph_st_table_0(&self) -> &PciePfTphStTable0 {
        &self.pcie_pf_tph_st_table_0
    }
    #[doc = "0x284 - TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_pf_tph_st_table_1(&self) -> &PciePfTphStTable1 {
        &self.pcie_pf_tph_st_table_1
    }
    #[doc = "0x288 - TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_pf_tph_st_table_2(&self) -> &PciePfTphStTable2 {
        &self.pcie_pf_tph_st_table_2
    }
    #[doc = "0x28c - TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_pf_tph_st_table_3(&self) -> &PciePfTphStTable3 {
        &self.pcie_pf_tph_st_table_3
    }
    #[doc = "0x900 - L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_pf_l1_pm_substates_extended_capability_header(
        &self,
    ) -> &PciePfL1PmSubstatesExtendedCapabilityHeader {
        &self.pcie_pf_l1_pm_substates_extended_capability_header
    }
    #[doc = "0x904 - L1 PM Substates Capabilities Register RSVD"]
    #[inline(always)]
    pub const fn pcie_pf_l1_pm_substates_capabilities(&self) -> &PciePfL1PmSubstatesCapabilities {
        &self.pcie_pf_l1_pm_substates_capabilities
    }
    #[doc = "0x908 - L1 PM Substates Control 1 Register (no description)"]
    #[inline(always)]
    pub const fn pcie_pf_l1_pm_substates_control_1(&self) -> &PciePfL1PmSubstatesControl1 {
        &self.pcie_pf_l1_pm_substates_control_1
    }
    #[doc = "0x90c - L1 PM Substates Control 2 Register RSVD"]
    #[inline(always)]
    pub const fn pcie_pf_l1_pm_substates_control_2(&self) -> &PciePfL1PmSubstatesControl2 {
        &self.pcie_pf_l1_pm_substates_control_2
    }
    #[doc = "0x10000 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn pcie_vf_vendor_id_and_device_id(&self) -> &PcieVfVendorIdAndDeviceId {
        &self.pcie_vf_vendor_id_and_device_id
    }
    #[doc = "0x10004 - Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
    #[inline(always)]
    pub const fn pcie_vf_command_and_status(&self) -> &PcieVfCommandAndStatus {
        &self.pcie_vf_command_and_status
    }
    #[doc = "0x10008 - Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_vf_revision_id_and_class_code(&self) -> &PcieVfRevisionIdAndClassCode {
        &self.pcie_vf_revision_id_and_class_code
    }
    #[doc = "0x1000c - BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x10010 - Base Address Register 0 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_0(&self) -> &PcieVfBaseAddress0 {
        &self.pcie_vf_base_address_0
    }
    #[doc = "0x10014 - Base Address Register 1 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_1(&self) -> &PcieVfBaseAddress1 {
        &self.pcie_vf_base_address_1
    }
    #[doc = "0x10018 - Base Address Register 2 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_2(&self) -> &PcieVfBaseAddress2 {
        &self.pcie_vf_base_address_2
    }
    #[doc = "0x1001c - Base Address Register 3 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_3(&self) -> &PcieVfBaseAddress3 {
        &self.pcie_vf_base_address_3
    }
    #[doc = "0x10020 - Base Address Register 4 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_4(&self) -> &PcieVfBaseAddress4 {
        &self.pcie_vf_base_address_4
    }
    #[doc = "0x10024 - Base Address Register 5 (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_base_address_5(&self) -> &PcieVfBaseAddress5 {
        &self.pcie_vf_base_address_5
    }
    #[doc = "0x1002c - Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub const fn pcie_vf_subsystem_vendor_id_and_subsystem_id(
        &self,
    ) -> &PcieVfSubsystemVendorIdAndSubsystemId {
        &self.pcie_vf_subsystem_vendor_id_and_subsystem_id
    }
    #[doc = "0x10030 - Expansion ROM Base Address Register (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_expansion_rom_base_address(&self) -> &PcieVfExpansionRomBaseAddress {
        &self.pcie_vf_expansion_rom_base_address
    }
    #[doc = "0x10034 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_capabilities_pointer(&self) -> &PcieVfCapabilitiesPointer {
        &self.pcie_vf_capabilities_pointer
    }
    #[doc = "0x1003c - Interrupt Line and Interrupt Pin Register (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_interrupt_line_and_interrupt_pin(
        &self,
    ) -> &PcieVfInterruptLineAndInterruptPin {
        &self.pcie_vf_interrupt_line_and_interrupt_pin
    }
    #[doc = "0x10080 - Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_vf_power_management_capabilities(
        &self,
    ) -> &PcieVfPowerManagementCapabilities {
        &self.pcie_vf_power_management_capabilities
    }
    #[doc = "0x10084 - Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_vf_power_management_control_status_report(
        &self,
    ) -> &PcieVfPowerManagementControlStatusReport {
        &self.pcie_vf_power_management_control_status_report
    }
    #[doc = "0x10090 - MSI Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_msi_control(&self) -> &PcieVfMsiControl {
        &self.pcie_vf_msi_control
    }
    #[doc = "0x10094 - MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_vf_msi_message_low_address(&self) -> &PcieVfMsiMessageLowAddress {
        &self.pcie_vf_msi_message_low_address
    }
    #[doc = "0x10098 - MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_vf_msi_message_high_address(&self) -> &PcieVfMsiMessageHighAddress {
        &self.pcie_vf_msi_message_high_address
    }
    #[doc = "0x1009c - MSI Message Data Register Hardwired to 0"]
    #[inline(always)]
    pub const fn pcie_vf_msi_message_data(&self) -> &PcieVfMsiMessageData {
        &self.pcie_vf_msi_message_data
    }
    #[doc = "0x100a0 - MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn pcie_vf_msi_mask(&self) -> &PcieVfMsiMask {
        &self.pcie_vf_msi_mask
    }
    #[doc = "0x100a4 - MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn pcie_vf_msi_pending_bits(&self) -> &PcieVfMsiPendingBits {
        &self.pcie_vf_msi_pending_bits
    }
    #[doc = "0x100b0 - MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_vf_msi_x_control(&self) -> &PcieVfMsiXControl {
        &self.pcie_vf_msi_x_control
    }
    #[doc = "0x100b4 - MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_vf_msi_x_table_offset(&self) -> &PcieVfMsiXTableOffset {
        &self.pcie_vf_msi_x_table_offset
    }
    #[doc = "0x100b8 - MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_vf_msi_x_pending_interrupt(&self) -> &PcieVfMsiXPendingInterrupt {
        &self.pcie_vf_msi_x_pending_interrupt
    }
    #[doc = "0x100c0 - PCI Express Capability List Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_pci_express_capability_list(&self) -> &PcieVfPciExpressCapabilityList {
        &self.pcie_vf_pci_express_capability_list
    }
    #[doc = "0x100c4 - PCI Express Device Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_pci_express_device_capabilities(
        &self,
    ) -> &PcieVfPciExpressDeviceCapabilities {
        &self.pcie_vf_pci_express_device_capabilities
    }
    #[doc = "0x100c8 - PCI Express Device Control and Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_pci_express_device_control_and_status(
        &self,
    ) -> &PcieVfPciExpressDeviceControlAndStatus {
        &self.pcie_vf_pci_express_device_control_and_status
    }
    #[doc = "0x100cc - Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
    #[inline(always)]
    pub const fn pcie_vf_link_capabilities(&self) -> &PcieVfLinkCapabilities {
        &self.pcie_vf_link_capabilities
    }
    #[doc = "0x100e4 - PCI Express Device Capabilities Register 2 Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_pci_express_device_capabilities_2(
        &self,
    ) -> &PcieVfPciExpressDeviceCapabilities2 {
        &self.pcie_vf_pci_express_device_capabilities_2
    }
    #[doc = "0x10100 - Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_vf_advanced_error_reporting_aer_enhanced_capability_header(
        &self,
    ) -> &PcieVfAdvancedErrorReportingAerEnhancedCapabilityHeader {
        &self.pcie_vf_advanced_error_reporting_aer_enhanced_capability_header
    }
    #[doc = "0x10104 - Uncorrectable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_uncorrectable_error_status(&self) -> &PcieVfUncorrectableErrorStatus {
        &self.pcie_vf_uncorrectable_error_status
    }
    #[doc = "0x10108 - Uncorrectable Error Mask Register (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_uncorrectable_error_mask(&self) -> &PcieVfUncorrectableErrorMask {
        &self.pcie_vf_uncorrectable_error_mask
    }
    #[doc = "0x1010c - Uncorrectable Error Severity Register (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_uncorrectable_error_severity(&self) -> &PcieVfUncorrectableErrorSeverity {
        &self.pcie_vf_uncorrectable_error_severity
    }
    #[doc = "0x10110 - Correctable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_correctable_error_status(&self) -> &PcieVfCorrectableErrorStatus {
        &self.pcie_vf_correctable_error_status
    }
    #[doc = "0x10114 - Correctable Error Mask Register (no description)"]
    #[inline(always)]
    pub const fn pcie_vf_correctable_error_mask(&self) -> &PcieVfCorrectableErrorMask {
        &self.pcie_vf_correctable_error_mask
    }
    #[doc = "0x10118 - Advanced Error Capabilities and Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_advanced_error_capabilities_and_control(
        &self,
    ) -> &PcieVfAdvancedErrorCapabilitiesAndControl {
        &self.pcie_vf_advanced_error_capabilities_and_control
    }
    #[doc = "0x1011c - Header Log Register 0 First DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_vf_header_log_0(&self) -> &PcieVfHeaderLog0 {
        &self.pcie_vf_header_log_0
    }
    #[doc = "0x10120 - Header Log Register 1 Second DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_vf_header_log_1(&self) -> &PcieVfHeaderLog1 {
        &self.pcie_vf_header_log_1
    }
    #[doc = "0x10124 - Header Log Register 2 Third DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_vf_header_log_2(&self) -> &PcieVfHeaderLog2 {
        &self.pcie_vf_header_log_2
    }
    #[doc = "0x10128 - Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn pcie_vf_header_log_3(&self) -> &PcieVfHeaderLog3 {
        &self.pcie_vf_header_log_3
    }
    #[doc = "0x10140 - ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_vf_ari_extended_capability_header(
        &self,
    ) -> &PcieVfAriExtendedCapabilityHeader {
        &self.pcie_vf_ari_extended_capability_header
    }
    #[doc = "0x10144 - ARI Capability Register and ARI Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_ari_capability_and_ari_control(
        &self,
    ) -> &PcieVfAriCapabilityAndAriControl {
        &self.pcie_vf_ari_capability_and_ari_control
    }
    #[doc = "0x10274 - TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_vf_tph_requester_enhanced_capability_header(
        &self,
    ) -> &PcieVfTphRequesterEnhancedCapabilityHeader {
        &self.pcie_vf_tph_requester_enhanced_capability_header
    }
    #[doc = "0x10278 - TPH Requester Capability Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_tph_requester_capability(&self) -> &PcieVfTphRequesterCapability {
        &self.pcie_vf_tph_requester_capability
    }
    #[doc = "0x1027c - TPH Requester Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_vf_tph_requester_control(&self) -> &PcieVfTphRequesterControl {
        &self.pcie_vf_tph_requester_control
    }
    #[doc = "0x10280 - TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_vf_tph_st_table_0(&self) -> &PcieVfTphStTable0 {
        &self.pcie_vf_tph_st_table_0
    }
    #[doc = "0x10284 - TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_vf_tph_st_table_1(&self) -> &PcieVfTphStTable1 {
        &self.pcie_vf_tph_st_table_1
    }
    #[doc = "0x10288 - TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_vf_tph_st_table_2(&self) -> &PcieVfTphStTable2 {
        &self.pcie_vf_tph_st_table_2
    }
    #[doc = "0x100000 - Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode."]
    #[inline(always)]
    pub const fn pcie_lm_physical_layer_configuration_0(
        &self,
    ) -> &PcieLmPhysicalLayerConfiguration0 {
        &self.pcie_lm_physical_layer_configuration_0
    }
    #[doc = "0x100004 - Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    pub const fn pcie_lm_physical_layer_configuration_1(
        &self,
    ) -> &PcieLmPhysicalLayerConfiguration1 {
        &self.pcie_lm_physical_layer_configuration_1
    }
    #[doc = "0x100008 - Data Link Layer Timer Configuration Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_data_link_layer_timer_configuration(
        &self,
    ) -> &PcieLmDataLinkLayerTimerConfiguration {
        &self.pcie_lm_data_link_layer_timer_configuration
    }
    #[doc = "0x10000c - Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub const fn pcie_lm_receive_credit_limit_0_vc0(&self) -> &PcieLmReceiveCreditLimit0Vc0 {
        &self.pcie_lm_receive_credit_limit_0_vc0
    }
    #[doc = "0x100010 - Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub const fn pcie_lm_receive_credit_limit_1_vc0(&self) -> &PcieLmReceiveCreditLimit1Vc0 {
        &self.pcie_lm_receive_credit_limit_1_vc0
    }
    #[doc = "0x100014 - Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub const fn pcie_lm_transmit_credit_limit_0_vc0(&self) -> &PcieLmTransmitCreditLimit0Vc0 {
        &self.pcie_lm_transmit_credit_limit_0_vc0
    }
    #[doc = "0x100018 - Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub const fn pcie_lm_transmit_credit_limit_1_vc0(&self) -> &PcieLmTransmitCreditLimit1Vc0 {
        &self.pcie_lm_transmit_credit_limit_1_vc0
    }
    #[doc = "0x10001c - Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub const fn pcie_lm_transmit_credit_update_interval_configuration_0(
        &self,
    ) -> &PcieLmTransmitCreditUpdateIntervalConfiguration0 {
        &self.pcie_lm_transmit_credit_update_interval_configuration_0
    }
    #[doc = "0x100020 - Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub const fn pcie_lm_transmit_credit_update_interval_configuration_1(
        &self,
    ) -> &PcieLmTransmitCreditUpdateIntervalConfiguration1 {
        &self.pcie_lm_transmit_credit_update_interval_configuration_1
    }
    #[doc = "0x100024 - L0S Timeout Limit Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_l0s_timeout_limit(&self) -> &PcieLmL0sTimeoutLimit {
        &self.pcie_lm_l0s_timeout_limit
    }
    #[doc = "0x100028 - Transmit TLP Count Register Count of TLPs transmitted"]
    #[inline(always)]
    pub const fn pcie_lm_transmit_tlp_count(&self) -> &PcieLmTransmitTlpCount {
        &self.pcie_lm_transmit_tlp_count
    }
    #[doc = "0x10002c - Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted"]
    #[inline(always)]
    pub const fn pcie_lm_transmit_tlp_payload_dword_count(
        &self,
    ) -> &PcieLmTransmitTlpPayloadDwordCount {
        &self.pcie_lm_transmit_tlp_payload_dword_count
    }
    #[doc = "0x100030 - Receive TLP Count Register Count of TLPs received"]
    #[inline(always)]
    pub const fn pcie_lm_receive_tlp_count(&self) -> &PcieLmReceiveTlpCount {
        &self.pcie_lm_receive_tlp_count
    }
    #[doc = "0x100034 - Receive TLP Payload Dword Count Register Count of TLP payload Dwords received"]
    #[inline(always)]
    pub const fn pcie_lm_receive_tlp_payload_dword_count(
        &self,
    ) -> &PcieLmReceiveTlpPayloadDwordCount {
        &self.pcie_lm_receive_tlp_payload_dword_count
    }
    #[doc = "0x100038 - Completion Timeout Limit Register 0 Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_completion_timeout_limit_0(&self) -> &PcieLmCompletionTimeoutLimit0 {
        &self.pcie_lm_completion_timeout_limit_0
    }
    #[doc = "0x10003c - Completion Timeout Limit Register 1 Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_completion_timeout_limit_1(&self) -> &PcieLmCompletionTimeoutLimit1 {
        &self.pcie_lm_completion_timeout_limit_1
    }
    #[doc = "0x100040 - L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns)."]
    #[inline(always)]
    pub const fn pcie_lm_l1_state_re_entry_delay(&self) -> &PcieLmL1StateReEntryDelay {
        &self.pcie_lm_l1_state_re_entry_delay
    }
    #[doc = "0x100044 - Vendor ID Register Subsystem Vendor ID"]
    #[inline(always)]
    pub const fn pcie_lm_vendor_id(&self) -> &PcieLmVendorId {
        &self.pcie_lm_vendor_id
    }
    #[doc = "0x100048 - ASPM L1 Entry Timeout Delay Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_aspm_l1_entry_timeout_delay(&self) -> &PcieLmAspmL1EntryTimeoutDelay {
        &self.pcie_lm_aspm_l1_entry_timeout_delay
    }
    #[doc = "0x10004c - PME TurnOff Ack Delay Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_pme_turnoff_ack_delay(&self) -> &PcieLmPmeTurnoffAckDelay {
        &self.pcie_lm_pme_turnoff_ack_delay
    }
    #[doc = "0x100050 - Linkwidth Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_linkwidth_control(&self) -> &PcieLmLinkwidthControl {
        &self.pcie_lm_linkwidth_control
    }
    #[doc = "0x100074 - SRIS Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_sris_control(&self) -> &PcieLmSrisControl {
        &self.pcie_lm_sris_control
    }
    #[doc = "0x100100 - Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn pcie_lm_shadow_register_header_log_0(&self) -> &PcieLmShadowRegisterHeaderLog0 {
        &self.pcie_lm_shadow_register_header_log_0
    }
    #[doc = "0x100104 - Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn pcie_lm_shadow_register_header_log_1(&self) -> &PcieLmShadowRegisterHeaderLog1 {
        &self.pcie_lm_shadow_register_header_log_1
    }
    #[doc = "0x100108 - Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn pcie_lm_shadow_register_header_log_2(&self) -> &PcieLmShadowRegisterHeaderLog2 {
        &self.pcie_lm_shadow_register_header_log_2
    }
    #[doc = "0x10010c - Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn pcie_lm_shadow_register_header_log_3(&self) -> &PcieLmShadowRegisterHeaderLog3 {
        &self.pcie_lm_shadow_register_header_log_3
    }
    #[doc = "0x100110 - Shadow register function number. Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_shadow_register_function_number(
        &self,
    ) -> &PcieLmShadowRegisterFunctionNumber {
        &self.pcie_lm_shadow_register_function_number
    }
    #[doc = "0x100114 - Shadow Register UR Error Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_shadow_ur_error(&self) -> &PcieLmShadowUrError {
        &self.pcie_lm_shadow_ur_error
    }
    #[doc = "0x100200 - Negotiated Lane Map Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_negotiated_lane_map(&self) -> &PcieLmNegotiatedLaneMap {
        &self.pcie_lm_negotiated_lane_map
    }
    #[doc = "0x100204 - Receive FTS Count Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_receive_fts_count(&self) -> &PcieLmReceiveFtsCount {
        &self.pcie_lm_receive_fts_count
    }
    #[doc = "0x100208 - Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit."]
    #[inline(always)]
    pub const fn pcie_lm_debug_mux_control(&self) -> &PcieLmDebugMuxControl {
        &self.pcie_lm_debug_mux_control
    }
    #[doc = "0x10020c - Local Error and Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_local_error_and_status(&self) -> &PcieLmLocalErrorAndStatus {
        &self.pcie_lm_local_error_and_status
    }
    #[doc = "0x100210 - Local Interrupt Mask Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_local_interrupt_mask(&self) -> &PcieLmLocalInterruptMask {
        &self.pcie_lm_local_interrupt_mask
    }
    #[doc = "0x100214 - LCRC Error Count Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_lcrc_error_count(&self) -> &PcieLmLcrcErrorCount {
        &self.pcie_lm_lcrc_error_count
    }
    #[doc = "0x100218 - ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it."]
    #[inline(always)]
    pub const fn pcie_lm_ecc_correctable_error_count(&self) -> &PcieLmEccCorrectableErrorCount {
        &self.pcie_lm_ecc_correctable_error_count
    }
    #[doc = "0x10021c - LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    pub const fn pcie_lm_ltr_snoop_no_snoop_latency(&self) -> &PcieLmLtrSnoopNoSnoopLatency {
        &self.pcie_lm_ltr_snoop_no_snoop_latency
    }
    #[doc = "0x100220 - LTR Message Generation Control Register RSVD"]
    #[inline(always)]
    pub const fn pcie_lm_ltr_message_generation_control(
        &self,
    ) -> &PcieLmLtrMessageGenerationControl {
        &self.pcie_lm_ltr_message_generation_control
    }
    #[doc = "0x100224 - PME Service Timeout Delay Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_pme_service_timeout_delay(&self) -> &PcieLmPmeServiceTimeoutDelay {
        &self.pcie_lm_pme_service_timeout_delay
    }
    #[doc = "0x100228 - Root Port Requestor ID Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_root_port_requestor_id(&self) -> &PcieLmRootPortRequestorId {
        &self.pcie_lm_root_port_requestor_id
    }
    #[doc = "0x10022c - End Point Bus and Device Number Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_end_point_bus_and_device_number(
        &self,
    ) -> &PcieLmEndPointBusAndDeviceNumber {
        &self.pcie_lm_end_point_bus_and_device_number
    }
    #[doc = "0x100240 - Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_physical_function_bar_configuration_0(
        &self,
    ) -> &PcieLmPhysicalFunctionBarConfiguration0 {
        &self.pcie_lm_physical_function_bar_configuration_0
    }
    #[doc = "0x100244 - Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure."]
    #[inline(always)]
    pub const fn pcie_lm_physical_function_bar_configuration_1(
        &self,
    ) -> &PcieLmPhysicalFunctionBarConfiguration1 {
        &self.pcie_lm_physical_function_bar_configuration_1
    }
    #[doc = "0x100280 - Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_virtual_function_bar_configuration_0(
        &self,
    ) -> &PcieLmVirtualFunctionBarConfiguration0 {
        &self.pcie_lm_virtual_function_bar_configuration_0
    }
    #[doc = "0x100284 - Virtual Function BAR Configuration Register 1 Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_virtual_function_bar_configuration_1(
        &self,
    ) -> &PcieLmVirtualFunctionBarConfiguration1 {
        &self.pcie_lm_virtual_function_bar_configuration_1
    }
    #[doc = "0x1002c0 - Physical Function Configuration Register Reserved"]
    #[inline(always)]
    pub const fn pcie_lm_physical_function_configuration(
        &self,
    ) -> &PcieLmPhysicalFunctionConfiguration {
        &self.pcie_lm_physical_function_configuration
    }
    #[doc = "0x100300 - Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
    #[inline(always)]
    pub const fn pcie_lm_root_complex_bar_configuration(
        &self,
    ) -> &PcieLmRootComplexBarConfiguration {
        &self.pcie_lm_root_complex_bar_configuration
    }
    #[doc = "0x200000 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_vendor_id_and_device_id(&self) -> &PcieRcVendorIdAndDeviceId {
        &self.pcie_rc_vendor_id_and_device_id
    }
    #[doc = "0x200004 - Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_command_and_status(&self) -> &PcieRcCommandAndStatus {
        &self.pcie_rc_command_and_status
    }
    #[doc = "0x200008 - Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_revision_id_and_class_code(&self) -> &PcieRcRevisionIdAndClassCode {
        &self.pcie_rc_revision_id_and_class_code
    }
    #[doc = "0x20000c - BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &PcieRcBistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x200010 - Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_rc_root_complex_base_address_0(&self) -> &PcieRcRootComplexBaseAddress0 {
        &self.pcie_rc_root_complex_base_address_0
    }
    #[doc = "0x200014 - Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn pcie_rc_root_complex_base_address_1(&self) -> &PcieRcRootComplexBaseAddress1 {
        &self.pcie_rc_root_complex_base_address_1
    }
    #[doc = "0x200018 - Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented."]
    #[inline(always)]
    pub const fn pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer(
        &self,
    ) -> &PcieRcPrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer {
        & self . pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer
    }
    #[doc = "0x20001c - IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_io_base_io_limit_secondary_status(
        &self,
    ) -> &PcieRcIoBaseIoLimitSecondaryStatus {
        &self.pcie_rc_io_base_io_limit_secondary_status
    }
    #[doc = "0x200020 - Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub const fn pcie_rc_memory_base_memory_limit(&self) -> &PcieRcMemoryBaseMemoryLimit {
        &self.pcie_rc_memory_base_memory_limit
    }
    #[doc = "0x200024 - Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn pcie_rc_prefetchable_memory_base_prefetchable_memory_limit(
        &self,
    ) -> &PcieRcPrefetchableMemoryBasePrefetchableMemoryLimit {
        &self.pcie_rc_prefetchable_memory_base_prefetchable_memory_limit
    }
    #[doc = "0x200028 - Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn pcie_rc_prefetchable_base_upper(&self) -> &PcieRcPrefetchableBaseUpper {
        &self.pcie_rc_prefetchable_base_upper
    }
    #[doc = "0x20002c - Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn pcie_rc_prefetchable_limit_upper(&self) -> &PcieRcPrefetchableLimitUpper {
        &self.pcie_rc_prefetchable_limit_upper
    }
    #[doc = "0x200030 - IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn pcie_rc_io_base_upper_io_limit_upper(&self) -> &PcieRcIoBaseUpperIoLimitUpper {
        &self.pcie_rc_io_base_upper_io_limit_upper
    }
    #[doc = "0x200034 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_capabilities_pointer(&self) -> &PcieRcCapabilitiesPointer {
        &self.pcie_rc_capabilities_pointer
    }
    #[doc = "0x20003c - Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_interrupt_line_interrupt_pin_and_bridge_control(
        &self,
    ) -> &PcieRcInterruptLineInterruptPinAndBridgeControl {
        &self.pcie_rc_interrupt_line_interrupt_pin_and_bridge_control
    }
    #[doc = "0x200080 - Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_power_management_capabilities(
        &self,
    ) -> &PcieRcPowerManagementCapabilities {
        &self.pcie_rc_power_management_capabilities
    }
    #[doc = "0x200084 - Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn pcie_rc_power_management_control_status_report(
        &self,
    ) -> &PcieRcPowerManagementControlStatusReport {
        &self.pcie_rc_power_management_control_status_report
    }
    #[doc = "0x200090 - MSI Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_msi_control(&self) -> &PcieRcMsiControl {
        &self.pcie_rc_msi_control
    }
    #[doc = "0x200094 - MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_rc_msi_message_low_address(&self) -> &PcieRcMsiMessageLowAddress {
        &self.pcie_rc_msi_message_low_address
    }
    #[doc = "0x200098 - MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_rc_msi_message_high_address(&self) -> &PcieRcMsiMessageHighAddress {
        &self.pcie_rc_msi_message_high_address
    }
    #[doc = "0x20009c - MSI Message Data Register Hardwired to 0"]
    #[inline(always)]
    pub const fn pcie_rc_msi_message_data(&self) -> &PcieRcMsiMessageData {
        &self.pcie_rc_msi_message_data
    }
    #[doc = "0x2000a0 - MSI Mask Register RSVD"]
    #[inline(always)]
    pub const fn pcie_rc_msi_mask(&self) -> &PcieRcMsiMask {
        &self.pcie_rc_msi_mask
    }
    #[doc = "0x2000a4 - MSI Pending Bits Register RSVD"]
    #[inline(always)]
    pub const fn pcie_rc_msi_pending_bits(&self) -> &PcieRcMsiPendingBits {
        &self.pcie_rc_msi_pending_bits
    }
    #[doc = "0x2000b0 - MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn pcie_rc_msi_x_control(&self) -> &PcieRcMsiXControl {
        &self.pcie_rc_msi_x_control
    }
    #[doc = "0x2000b4 - MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_rc_msi_x_table_offset(&self) -> &PcieRcMsiXTableOffset {
        &self.pcie_rc_msi_x_table_offset
    }
    #[doc = "0x2000b8 - MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn pcie_rc_msi_x_pending_interrupt(&self) -> &PcieRcMsiXPendingInterrupt {
        &self.pcie_rc_msi_x_pending_interrupt
    }
    #[doc = "0x2000c0 - PCI Express Capability List Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_pci_express_capability_list(&self) -> &PcieRcPciExpressCapabilityList {
        &self.pcie_rc_pci_express_capability_list
    }
    #[doc = "0x2000c4 - PCI Express Device Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_pci_express_device_capabilities(
        &self,
    ) -> &PcieRcPciExpressDeviceCapabilities {
        &self.pcie_rc_pci_express_device_capabilities
    }
    #[doc = "0x2000c8 - PCI Express Device Control and Status Register (no description)"]
    #[inline(always)]
    pub const fn pcie_rc_pci_express_device_control_and_status(
        &self,
    ) -> &PcieRcPciExpressDeviceControlAndStatus {
        &self.pcie_rc_pci_express_device_control_and_status
    }
    #[doc = "0x2000cc - Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn pcie_rc_link_capabilities(&self) -> &PcieRcLinkCapabilities {
        &self.pcie_rc_link_capabilities
    }
    #[doc = "0x2000d0 - Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub const fn pcie_rc_link_control_and_status(&self) -> &PcieRcLinkControlAndStatus {
        &self.pcie_rc_link_control_and_status
    }
    #[doc = "0x2000d4 - Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
    #[inline(always)]
    pub const fn pcie_rc_slot_capability(&self) -> &PcieRcSlotCapability {
        &self.pcie_rc_slot_capability
    }
    #[doc = "0x2000d8 - Slot Control and Status Register (no description)"]
    #[inline(always)]
    pub const fn pcie_rc_slot_control_and_status(&self) -> &PcieRcSlotControlAndStatus {
        &self.pcie_rc_slot_control_and_status
    }
    #[doc = "0x2000dc - Root Control and Capability Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_root_control_and_capability(&self) -> &PcieRcRootControlAndCapability {
        &self.pcie_rc_root_control_and_capability
    }
    #[doc = "0x2000e0 - Root Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_root_status(&self) -> &PcieRcRootStatus {
        &self.pcie_rc_root_status
    }
    #[doc = "0x2000e4 - PCI Express Device Capabilities 2 Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_pci_express_device_capabilities_2(
        &self,
    ) -> &PcieRcPciExpressDeviceCapabilities2 {
        &self.pcie_rc_pci_express_device_capabilities_2
    }
    #[doc = "0x2000e8 - PCI Express Device Control and Status 2 Register (no description)"]
    #[inline(always)]
    pub const fn pcie_rc_pci_express_device_control_and_status_2(
        &self,
    ) -> &PcieRcPciExpressDeviceControlAndStatus2 {
        &self.pcie_rc_pci_express_device_control_and_status_2
    }
    #[doc = "0x2000ec - Link Capabilities Register 2 RSVD"]
    #[inline(always)]
    pub const fn pcie_rc_link_capabilities_2(&self) -> &PcieRcLinkCapabilities2 {
        &self.pcie_rc_link_capabilities_2
    }
    #[doc = "0x2000f0 - Link Control and Status 2 Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_link_control_and_status_2(&self) -> &PcieRcLinkControlAndStatus2 {
        &self.pcie_rc_link_control_and_status_2
    }
    #[doc = "0x200100 - Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_rc_advanced_error_reporting_aer_enhanced_capability_header(
        &self,
    ) -> &PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeader {
        &self.pcie_rc_advanced_error_reporting_aer_enhanced_capability_header
    }
    #[doc = "0x200104 - Uncorrectable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_uncorrectable_error_status(&self) -> &PcieRcUncorrectableErrorStatus {
        &self.pcie_rc_uncorrectable_error_status
    }
    #[doc = "0x200108 - Uncorrectable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_uncorrectable_error_mask(&self) -> &PcieRcUncorrectableErrorMask {
        &self.pcie_rc_uncorrectable_error_mask
    }
    #[doc = "0x20010c - Uncorrectable Error Severity Register (no description)"]
    #[inline(always)]
    pub const fn pcie_rc_uncorrectable_error_severity(&self) -> &PcieRcUncorrectableErrorSeverity {
        &self.pcie_rc_uncorrectable_error_severity
    }
    #[doc = "0x200110 - Correctable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_correctable_error_status(&self) -> &PcieRcCorrectableErrorStatus {
        &self.pcie_rc_correctable_error_status
    }
    #[doc = "0x200114 - Correctable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_correctable_error_mask(&self) -> &PcieRcCorrectableErrorMask {
        &self.pcie_rc_correctable_error_mask
    }
    #[doc = "0x200118 - Advanced Error Capabilities and Control Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_advanced_error_capabilities_and_control(
        &self,
    ) -> &PcieRcAdvancedErrorCapabilitiesAndControl {
        &self.pcie_rc_advanced_error_capabilities_and_control
    }
    #[doc = "0x20011c - Header Log Register 0 First Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn pcie_rc_header_log_0(&self) -> &PcieRcHeaderLog0 {
        &self.pcie_rc_header_log_0
    }
    #[doc = "0x200120 - Header Log Register 1 Second Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn pcie_rc_header_log_1(&self) -> &PcieRcHeaderLog1 {
        &self.pcie_rc_header_log_1
    }
    #[doc = "0x200124 - Header Log Register 2 Third Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn pcie_rc_header_log_2(&self) -> &PcieRcHeaderLog2 {
        &self.pcie_rc_header_log_2
    }
    #[doc = "0x200128 - Header Log Register 3 Fourth Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn pcie_rc_header_log_3(&self) -> &PcieRcHeaderLog3 {
        &self.pcie_rc_header_log_3
    }
    #[doc = "0x20012c - Root Error Command Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_root_error_command(&self) -> &PcieRcRootErrorCommand {
        &self.pcie_rc_root_error_command
    }
    #[doc = "0x200130 - Root Error Status Register Reserved"]
    #[inline(always)]
    pub const fn pcie_rc_root_error_status(&self) -> &PcieRcRootErrorStatus {
        &self.pcie_rc_root_error_status
    }
    #[doc = "0x200134 - Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
    #[inline(always)]
    pub const fn pcie_rc_error_source_identification(&self) -> &PcieRcErrorSourceIdentification {
        &self.pcie_rc_error_source_identification
    }
    #[doc = "0x200280 - TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn pcie_rc_tph_st_table_3(&self) -> &PcieRcTphStTable3 {
        &self.pcie_rc_tph_st_table_3
    }
    #[doc = "0x200900 - L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn pcie_rc_l1_pm_substates_extended_capability_header(
        &self,
    ) -> &PcieRcL1PmSubstatesExtendedCapabilityHeader {
        &self.pcie_rc_l1_pm_substates_extended_capability_header
    }
    #[doc = "0x200904 - L1 PM Substates Capabilities Register RSVD"]
    #[inline(always)]
    pub const fn pcie_rc_l1_pm_substates_capabilities(&self) -> &PcieRcL1PmSubstatesCapabilities {
        &self.pcie_rc_l1_pm_substates_capabilities
    }
    #[doc = "0x200908 - L1 PM Substates Control 1 Register (no description)"]
    #[inline(always)]
    pub const fn pcie_rc_l1_pm_substates_control_1(&self) -> &PcieRcL1PmSubstatesControl1 {
        &self.pcie_rc_l1_pm_substates_control_1
    }
    #[doc = "0x20090c - L1 PM Substates Control 2 Register RSVD"]
    #[inline(always)]
    pub const fn pcie_rc_l1_pm_substates_control_2(&self) -> &PcieRcL1PmSubstatesControl2 {
        &self.pcie_rc_l1_pm_substates_control_2
    }
    #[doc = "0x400000 - Outbound Region Address 0 Lower 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_address_0(&self) -> &PcieAtObOutboundRegionAddress0 {
        &self.pcie_at_ob_outbound_region_address_0
    }
    #[doc = "0x400004 - Outbound Region Address 1 Upper 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_address_1(&self) -> &PcieAtObOutboundRegionAddress1 {
        &self.pcie_at_ob_outbound_region_address_1
    }
    #[doc = "0x400008 - Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_descriptor_0(
        &self,
    ) -> &PcieAtObOutboundRegionDescriptor0 {
        &self.pcie_at_ob_outbound_region_descriptor_0
    }
    #[doc = "0x40000c - Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_descriptor_1(
        &self,
    ) -> &PcieAtObOutboundRegionDescriptor1 {
        &self.pcie_at_ob_outbound_region_descriptor_1
    }
    #[doc = "0x400010 - Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_descriptor_2(
        &self,
    ) -> &PcieAtObOutboundRegionDescriptor2 {
        &self.pcie_at_ob_outbound_region_descriptor_2
    }
    #[doc = "0x400014 - Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N"]
    #[inline(always)]
    pub const fn pcie_at_ob_outbound_region_descriptor_3(
        &self,
    ) -> &PcieAtObOutboundRegionDescriptor3 {
        &self.pcie_at_ob_outbound_region_descriptor_3
    }
    #[doc = "0x400800 - RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn pcie_at_rp_ib_rp_inbound_bar_address_translation_0(
        &self,
    ) -> &PcieAtRpIbRpInboundBarAddressTranslation0 {
        &self.pcie_at_rp_ib_rp_inbound_bar_address_translation_0
    }
    #[doc = "0x400804 - RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn pcie_at_rp_ib_rp_inbound_bar_address_translation_1(
        &self,
    ) -> &PcieAtRpIbRpInboundBarAddressTranslation1 {
        &self.pcie_at_rp_ib_rp_inbound_bar_address_translation_1
    }
    #[doc = "0x400824 - Link down indication bit RSVD"]
    #[inline(always)]
    pub const fn pcie_at_rp_ib_link_down_indication_bit(&self) -> &PcieAtRpIbLinkDownIndicationBit {
        &self.pcie_at_rp_ib_link_down_indication_bit
    }
    #[doc = "0x400828 - EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn pcie_at_ep_ib_ep_inbound_bar_address_translation_0(
        &self,
    ) -> &PcieAtEpIbEpInboundBarAddressTranslation0 {
        &self.pcie_at_ep_ib_ep_inbound_bar_address_translation_0
    }
    #[doc = "0x40082c - EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub const fn pcie_at_ep_ib_ep_inbound_bar_address_translation_1(
        &self,
    ) -> &PcieAtEpIbEpInboundBarAddressTranslation1 {
        &self.pcie_at_ep_ib_ep_inbound_bar_address_translation_1
    }
    #[doc = "0x600000 - PCIe DMA Channel 0 Control Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_control(&self) -> &PcieDmaChannel0Control {
        &self.pcie_dma_channel_0_control
    }
    #[doc = "0x600004 - PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_start_pointer_lower(
        &self,
    ) -> &PcieDmaChannel0StartPointerLower {
        &self.pcie_dma_channel_0_start_pointer_lower
    }
    #[doc = "0x600008 - PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_start_pointer_upper(
        &self,
    ) -> &PcieDmaChannel0StartPointerUpper {
        &self.pcie_dma_channel_0_start_pointer_upper
    }
    #[doc = "0x60000c - PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_attribute_lower(&self) -> &PcieDmaChannel0AttributeLower {
        &self.pcie_dma_channel_0_attribute_lower
    }
    #[doc = "0x600010 - PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_attribute_upper(&self) -> &PcieDmaChannel0AttributeUpper {
        &self.pcie_dma_channel_0_attribute_upper
    }
    #[doc = "0x600014 - PCIe DMA Channel 1 Control Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_control(&self) -> &PcieDmaChannel1Control {
        &self.pcie_dma_channel_1_control
    }
    #[doc = "0x600018 - PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_start_pointer_lower(
        &self,
    ) -> &PcieDmaChannel1StartPointerLower {
        &self.pcie_dma_channel_1_start_pointer_lower
    }
    #[doc = "0x60001c - PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_start_pointer_upper(
        &self,
    ) -> &PcieDmaChannel1StartPointerUpper {
        &self.pcie_dma_channel_1_start_pointer_upper
    }
    #[doc = "0x600020 - PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_attribute_lower(&self) -> &PcieDmaChannel1AttributeLower {
        &self.pcie_dma_channel_1_attribute_lower
    }
    #[doc = "0x600024 - PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_attribute_upper(&self) -> &PcieDmaChannel1AttributeUpper {
        &self.pcie_dma_channel_1_attribute_upper
    }
    #[doc = "0x6000a0 - PCIe DMA Interrupt Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt(&self) -> &PcieDmaInterrupt {
        &self.pcie_dma_interrupt
    }
    #[doc = "0x6000a4 - PCIe DMA Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt_enable(&self) -> &PcieDmaInterruptEnable {
        &self.pcie_dma_interrupt_enable
    }
    #[doc = "0x6000a8 - PCIe DMA Interrupt Disable Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt_disable(&self) -> &PcieDmaInterruptDisable {
        &self.pcie_dma_interrupt_disable
    }
    #[doc = "0x6000ac - PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_inbound_buffer_uncorrected_ecc_errors(
        &self,
    ) -> &PcieDmaInboundBufferUncorrectedEccErrors {
        &self.pcie_dma_inbound_buffer_uncorrected_ecc_errors
    }
    #[doc = "0x6000b0 - PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_inbound_buffer_corrected_ecc_errors(
        &self,
    ) -> &PcieDmaInboundBufferCorrectedEccErrors {
        &self.pcie_dma_inbound_buffer_corrected_ecc_errors
    }
    #[doc = "0x6000b4 - PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_outbound_buffer_uncorrected_ecc_errors(
        &self,
    ) -> &PcieDmaOutboundBufferUncorrectedEccErrors {
        &self.pcie_dma_outbound_buffer_uncorrected_ecc_errors
    }
    #[doc = "0x6000b8 - PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_outbound_buffer_corrected_ecc_errors(
        &self,
    ) -> &PcieDmaOutboundBufferCorrectedEccErrors {
        &self.pcie_dma_outbound_buffer_corrected_ecc_errors
    }
    #[doc = "0x6000f8 - PCIe DMA Capability and Version Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_capability_and_version(&self) -> &PcieDmaCapabilityAndVersion {
        &self.pcie_dma_capability_and_version
    }
    #[doc = "0x6000fc - PCIe DMA Configuration Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_configuration(&self) -> &PcieDmaConfiguration {
        &self.pcie_dma_configuration
    }
}
#[doc = "PCIE_PF_VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vendor_id_and_device_id`]
module"]
#[doc(alias = "PCIE_PF_VENDOR_ID_AND_DEVICE_ID")]
pub type PciePfVendorIdAndDeviceId =
    crate::Reg<pcie_pf_vendor_id_and_device_id::PciePfVendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus."]
pub mod pcie_pf_vendor_id_and_device_id;
#[doc = "PCIE_PF_COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_command_and_status`]
module"]
#[doc(alias = "PCIE_PF_COMMAND_AND_STATUS")]
pub type PciePfCommandAndStatus =
    crate::Reg<pcie_pf_command_and_status::PciePfCommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position."]
pub mod pcie_pf_command_and_status;
#[doc = "PCIE_PF_REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_revision_id_and_class_code`]
module"]
#[doc(alias = "PCIE_PF_REVISION_ID_AND_CLASS_CODE")]
pub type PciePfRevisionIdAndClassCode =
    crate::Reg<pcie_pf_revision_id_and_class_code::PciePfRevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus"]
pub mod pcie_pf_revision_id_and_class_code;
#[doc = "PCIE_PF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (rw) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "PCIE_PF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type PciePfBistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s :: PciePfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus."]
pub mod pcie_pf_bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "PCIE_PF_BASE_ADDRESS_0 (rw) register accessor: Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_0`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_0")]
pub type PciePfBaseAddress0 = crate::Reg<pcie_pf_base_address_0::PciePfBaseAddress0Spec>;
#[doc = "Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_base_address_0;
#[doc = "PCIE_PF_BASE_ADDRESS_1 (rw) register accessor: Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_1`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_1")]
pub type PciePfBaseAddress1 = crate::Reg<pcie_pf_base_address_1::PciePfBaseAddress1Spec>;
#[doc = "Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_base_address_1;
#[doc = "PCIE_PF_BASE_ADDRESS_2 (rw) register accessor: Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_2`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_2")]
pub type PciePfBaseAddress2 = crate::Reg<pcie_pf_base_address_2::PciePfBaseAddress2Spec>;
#[doc = "Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_base_address_2;
#[doc = "PCIE_PF_BASE_ADDRESS_3 (rw) register accessor: Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_3`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_3")]
pub type PciePfBaseAddress3 = crate::Reg<pcie_pf_base_address_3::PciePfBaseAddress3Spec>;
#[doc = "Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_base_address_3;
#[doc = "PCIE_PF_BASE_ADDRESS_4 (rw) register accessor: Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_4`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_4")]
pub type PciePfBaseAddress4 = crate::Reg<pcie_pf_base_address_4::PciePfBaseAddress4Spec>;
#[doc = "Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_base_address_4;
#[doc = "PCIE_PF_BASE_ADDRESS_5 (rw) register accessor: Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_base_address_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_base_address_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_base_address_5`]
module"]
#[doc(alias = "PCIE_PF_BASE_ADDRESS_5")]
pub type PciePfBaseAddress5 = crate::Reg<pcie_pf_base_address_5::PciePfBaseAddress5Spec>;
#[doc = "Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_base_address_5;
#[doc = "PCIE_PF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID (r) register accessor: Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_subsystem_vendor_id_and_subsystem_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_subsystem_vendor_id_and_subsystem_id`]
module"]
#[doc(alias = "PCIE_PF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID")]
pub type PciePfSubsystemVendorIdAndSubsystemId = crate::Reg<
    pcie_pf_subsystem_vendor_id_and_subsystem_id::PciePfSubsystemVendorIdAndSubsystemIdSpec,
>;
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
pub mod pcie_pf_subsystem_vendor_id_and_subsystem_id;
#[doc = "PCIE_PF_CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_capabilities_pointer`]
module"]
#[doc(alias = "PCIE_PF_CAPABILITIES_POINTER")]
pub type PciePfCapabilitiesPointer =
    crate::Reg<pcie_pf_capabilities_pointer::PciePfCapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod pcie_pf_capabilities_pointer;
#[doc = "PCIE_PF_INTERRUPT_LINE_AND_INTERRUPT_PIN (rw) register accessor: Interrupt Line and Interrupt Pin Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_interrupt_line_and_interrupt_pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_interrupt_line_and_interrupt_pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_interrupt_line_and_interrupt_pin`]
module"]
#[doc(alias = "PCIE_PF_INTERRUPT_LINE_AND_INTERRUPT_PIN")]
pub type PciePfInterruptLineAndInterruptPin =
    crate::Reg<pcie_pf_interrupt_line_and_interrupt_pin::PciePfInterruptLineAndInterruptPinSpec>;
#[doc = "Interrupt Line and Interrupt Pin Register Reserved"]
pub mod pcie_pf_interrupt_line_and_interrupt_pin;
#[doc = "PCIE_PF_POWER_MANAGEMENT_CAPABILITIES (r) register accessor: Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_management_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_management_capabilities`]
module"]
#[doc(alias = "PCIE_PF_POWER_MANAGEMENT_CAPABILITIES")]
pub type PciePfPowerManagementCapabilities =
    crate::Reg<pcie_pf_power_management_capabilities::PciePfPowerManagementCapabilitiesSpec>;
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
pub mod pcie_pf_power_management_capabilities;
#[doc = "PCIE_PF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT (rw) register accessor: Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_management_control_status_report::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_power_management_control_status_report::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_management_control_status_report`]
module"]
#[doc(alias = "PCIE_PF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT")]
pub type PciePfPowerManagementControlStatusReport = crate::Reg<
    pcie_pf_power_management_control_status_report::PciePfPowerManagementControlStatusReportSpec,
>;
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub mod pcie_pf_power_management_control_status_report;
#[doc = "PCIE_PF_MSI_CONTROL (rw) register accessor: MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_control`]
module"]
#[doc(alias = "PCIE_PF_MSI_CONTROL")]
pub type PciePfMsiControl = crate::Reg<pcie_pf_msi_control::PciePfMsiControlSpec>;
#[doc = "MSI Control Register Reserved"]
pub mod pcie_pf_msi_control;
#[doc = "PCIE_PF_MSI_MESSAGE_LOW_ADDRESS (rw) register accessor: MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_message_low_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_message_low_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_message_low_address`]
module"]
#[doc(alias = "PCIE_PF_MSI_MESSAGE_LOW_ADDRESS")]
pub type PciePfMsiMessageLowAddress =
    crate::Reg<pcie_pf_msi_message_low_address::PciePfMsiMessageLowAddressSpec>;
#[doc = "MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub mod pcie_pf_msi_message_low_address;
#[doc = "PCIE_PF_MSI_MESSAGE_HIGH_ADDRESS (rw) register accessor: MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_message_high_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_message_high_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_message_high_address`]
module"]
#[doc(alias = "PCIE_PF_MSI_MESSAGE_HIGH_ADDRESS")]
pub type PciePfMsiMessageHighAddress =
    crate::Reg<pcie_pf_msi_message_high_address::PciePfMsiMessageHighAddressSpec>;
#[doc = "MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub mod pcie_pf_msi_message_high_address;
#[doc = "PCIE_PF_MSI_MESSAGE_DATA (rw) register accessor: MSI Message Data Register Hardwired to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_message_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_message_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_message_data`]
module"]
#[doc(alias = "PCIE_PF_MSI_MESSAGE_DATA")]
pub type PciePfMsiMessageData = crate::Reg<pcie_pf_msi_message_data::PciePfMsiMessageDataSpec>;
#[doc = "MSI Message Data Register Hardwired to 0"]
pub mod pcie_pf_msi_message_data;
#[doc = "PCIE_PF_MSI_MASK (rw) register accessor: MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_mask`]
module"]
#[doc(alias = "PCIE_PF_MSI_MASK")]
pub type PciePfMsiMask = crate::Reg<pcie_pf_msi_mask::PciePfMsiMaskSpec>;
#[doc = "MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod pcie_pf_msi_mask;
#[doc = "PCIE_PF_MSI_PENDING_BITS (r) register accessor: MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_pending_bits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_pending_bits`]
module"]
#[doc(alias = "PCIE_PF_MSI_PENDING_BITS")]
pub type PciePfMsiPendingBits = crate::Reg<pcie_pf_msi_pending_bits::PciePfMsiPendingBitsSpec>;
#[doc = "MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod pcie_pf_msi_pending_bits;
#[doc = "PCIE_PF_MSI_X_CONTROL (rw) register accessor: MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_x_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_x_control`]
module"]
#[doc(alias = "PCIE_PF_MSI_X_CONTROL")]
pub type PciePfMsiXControl = crate::Reg<pcie_pf_msi_x_control::PciePfMsiXControlSpec>;
#[doc = "MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub mod pcie_pf_msi_x_control;
#[doc = "PCIE_PF_MSI_X_TABLE_OFFSET (r) register accessor: MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_table_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_x_table_offset`]
module"]
#[doc(alias = "PCIE_PF_MSI_X_TABLE_OFFSET")]
pub type PciePfMsiXTableOffset = crate::Reg<pcie_pf_msi_x_table_offset::PciePfMsiXTableOffsetSpec>;
#[doc = "MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_pf_msi_x_table_offset;
#[doc = "PCIE_PF_MSI_X_PENDING_INTERRUPT (r) register accessor: MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_pending_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_msi_x_pending_interrupt`]
module"]
#[doc(alias = "PCIE_PF_MSI_X_PENDING_INTERRUPT")]
pub type PciePfMsiXPendingInterrupt =
    crate::Reg<pcie_pf_msi_x_pending_interrupt::PciePfMsiXPendingInterruptSpec>;
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_pf_msi_x_pending_interrupt;
#[doc = "PCIE_PF_PCI_EXPRESS_CAPABILITY_LIST (r) register accessor: PCI Express Capability List Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_capability_list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_pci_express_capability_list`]
module"]
#[doc(alias = "PCIE_PF_PCI_EXPRESS_CAPABILITY_LIST")]
pub type PciePfPciExpressCapabilityList =
    crate::Reg<pcie_pf_pci_express_capability_list::PciePfPciExpressCapabilityListSpec>;
#[doc = "PCI Express Capability List Register Reserved"]
pub mod pcie_pf_pci_express_capability_list;
#[doc = "PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES (r) register accessor: PCI Express Device Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_pci_express_device_capabilities`]
module"]
#[doc(alias = "PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES")]
pub type PciePfPciExpressDeviceCapabilities =
    crate::Reg<pcie_pf_pci_express_device_capabilities::PciePfPciExpressDeviceCapabilitiesSpec>;
#[doc = "PCI Express Device Capabilities Register Reserved"]
pub mod pcie_pf_pci_express_device_capabilities;
#[doc = "PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS (rw) register accessor: PCI Express Device Control and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_pci_express_device_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_pci_express_device_control_and_status`]
module"]
#[doc(alias = "PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS")]
pub type PciePfPciExpressDeviceControlAndStatus = crate::Reg<
    pcie_pf_pci_express_device_control_and_status::PciePfPciExpressDeviceControlAndStatusSpec,
>;
#[doc = "PCI Express Device Control and Status Register Reserved"]
pub mod pcie_pf_pci_express_device_control_and_status;
#[doc = "PCIE_PF_LINK_CAPABILITIES (r) register accessor: Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_link_capabilities`]
module"]
#[doc(alias = "PCIE_PF_LINK_CAPABILITIES")]
pub type PciePfLinkCapabilities = crate::Reg<pcie_pf_link_capabilities::PciePfLinkCapabilitiesSpec>;
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
pub mod pcie_pf_link_capabilities;
#[doc = "PCIE_PF_LINK_CONTROL_AND_STATUS (rw) register accessor: Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_link_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_link_control_and_status`]
module"]
#[doc(alias = "PCIE_PF_LINK_CONTROL_AND_STATUS")]
pub type PciePfLinkControlAndStatus =
    crate::Reg<pcie_pf_link_control_and_status::PciePfLinkControlAndStatusSpec>;
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub mod pcie_pf_link_control_and_status;
#[doc = "PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCIE_PF_PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PciePfPciExpressDeviceCapabilities2 =
    crate::Reg<pcie_pf_pci_express_device_capabilities_2::PciePfPciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities Register 2 Reserved"]
pub mod pcie_pf_pci_express_device_capabilities_2;
#[doc = "PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 (rw) register accessor: PCI Express Device Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_pci_express_device_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_pci_express_device_control_and_status_2`]
module"]
#[doc(alias = "PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2")]
pub type PciePfPciExpressDeviceControlAndStatus2 = crate::Reg<
    pcie_pf_pci_express_device_control_and_status_2::PciePfPciExpressDeviceControlAndStatus2Spec,
>;
#[doc = "PCI Express Device Control and Status Register 2 Reserved"]
pub mod pcie_pf_pci_express_device_control_and_status_2;
#[doc = "PCIE_PF_LINK_CAPABILITIES_2 (r) register accessor: Link Capabilities Register 2 RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_link_capabilities_2`]
module"]
#[doc(alias = "PCIE_PF_LINK_CAPABILITIES_2")]
pub type PciePfLinkCapabilities2 =
    crate::Reg<pcie_pf_link_capabilities_2::PciePfLinkCapabilities2Spec>;
#[doc = "Link Capabilities Register 2 RSVD"]
pub mod pcie_pf_link_capabilities_2;
#[doc = "PCIE_PF_LINK_CONTROL_AND_STATUS_2 (rw) register accessor: Link Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_link_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_link_control_and_status_2`]
module"]
#[doc(alias = "PCIE_PF_LINK_CONTROL_AND_STATUS_2")]
pub type PciePfLinkControlAndStatus2 =
    crate::Reg<pcie_pf_link_control_and_status_2::PciePfLinkControlAndStatus2Spec>;
#[doc = "Link Control and Status Register 2 Reserved"]
pub mod pcie_pf_link_control_and_status_2;
#[doc = "PCIE_PF_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER (r) register accessor: Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_advanced_error_reporting_aer_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_advanced_error_reporting_aer_enhanced_capability_header`]
module"]
#[doc(alias = "PCIE_PF_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER")]
pub type PciePfAdvancedErrorReportingAerEnhancedCapabilityHeader = crate :: Reg < pcie_pf_advanced_error_reporting_aer_enhanced_capability_header :: PciePfAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec > ;
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_advanced_error_reporting_aer_enhanced_capability_header;
#[doc = "PCIE_PF_UNCORRECTABLE_ERROR_STATUS (rw) register accessor: Uncorrectable Error Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_uncorrectable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_uncorrectable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_uncorrectable_error_status`]
module"]
#[doc(alias = "PCIE_PF_UNCORRECTABLE_ERROR_STATUS")]
pub type PciePfUncorrectableErrorStatus =
    crate::Reg<pcie_pf_uncorrectable_error_status::PciePfUncorrectableErrorStatusSpec>;
#[doc = "Uncorrectable Error Status Register (no description)"]
pub mod pcie_pf_uncorrectable_error_status;
#[doc = "PCIE_PF_UNCORRECTABLE_ERROR_MASK (rw) register accessor: Uncorrectable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_uncorrectable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_uncorrectable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_uncorrectable_error_mask`]
module"]
#[doc(alias = "PCIE_PF_UNCORRECTABLE_ERROR_MASK")]
pub type PciePfUncorrectableErrorMask =
    crate::Reg<pcie_pf_uncorrectable_error_mask::PciePfUncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register Reserved"]
pub mod pcie_pf_uncorrectable_error_mask;
#[doc = "PCIE_PF_UNCORRECTABLE_ERROR_SEVERITY (rw) register accessor: Uncorrectable Error Severity Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_uncorrectable_error_severity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_uncorrectable_error_severity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_uncorrectable_error_severity`]
module"]
#[doc(alias = "PCIE_PF_UNCORRECTABLE_ERROR_SEVERITY")]
pub type PciePfUncorrectableErrorSeverity =
    crate::Reg<pcie_pf_uncorrectable_error_severity::PciePfUncorrectableErrorSeveritySpec>;
#[doc = "Uncorrectable Error Severity Register Reserved"]
pub mod pcie_pf_uncorrectable_error_severity;
#[doc = "PCIE_PF_CORRECTABLE_ERROR_STATUS (rw) register accessor: Correctable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_correctable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_correctable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_correctable_error_status`]
module"]
#[doc(alias = "PCIE_PF_CORRECTABLE_ERROR_STATUS")]
pub type PciePfCorrectableErrorStatus =
    crate::Reg<pcie_pf_correctable_error_status::PciePfCorrectableErrorStatusSpec>;
#[doc = "Correctable Error Status Register Reserved"]
pub mod pcie_pf_correctable_error_status;
#[doc = "PCIE_PF_CORRECTABLE_ERROR_MASK (rw) register accessor: Correctable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_correctable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_correctable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_correctable_error_mask`]
module"]
#[doc(alias = "PCIE_PF_CORRECTABLE_ERROR_MASK")]
pub type PciePfCorrectableErrorMask =
    crate::Reg<pcie_pf_correctable_error_mask::PciePfCorrectableErrorMaskSpec>;
#[doc = "Correctable Error Mask Register Reserved"]
pub mod pcie_pf_correctable_error_mask;
#[doc = "PCIE_PF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL (rw) register accessor: Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_advanced_error_capabilities_and_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_advanced_error_capabilities_and_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_advanced_error_capabilities_and_control`]
module"]
#[doc(alias = "PCIE_PF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL")]
pub type PciePfAdvancedErrorCapabilitiesAndControl = crate::Reg<
    pcie_pf_advanced_error_capabilities_and_control::PciePfAdvancedErrorCapabilitiesAndControlSpec,
>;
#[doc = "Advanced Error Capabilities and Control Register Reserved"]
pub mod pcie_pf_advanced_error_capabilities_and_control;
#[doc = "PCIE_PF_HEADER_LOG_0 (r) register accessor: Header Log Register 0 First DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_header_log_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_header_log_0`]
module"]
#[doc(alias = "PCIE_PF_HEADER_LOG_0")]
pub type PciePfHeaderLog0 = crate::Reg<pcie_pf_header_log_0::PciePfHeaderLog0Spec>;
#[doc = "Header Log Register 0 First DWORD of captured TLP header STICKY."]
pub mod pcie_pf_header_log_0;
#[doc = "PCIE_PF_HEADER_LOG_1 (r) register accessor: Header Log Register 1 Second DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_header_log_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_header_log_1`]
module"]
#[doc(alias = "PCIE_PF_HEADER_LOG_1")]
pub type PciePfHeaderLog1 = crate::Reg<pcie_pf_header_log_1::PciePfHeaderLog1Spec>;
#[doc = "Header Log Register 1 Second DWORD of captured TLP header STICKY."]
pub mod pcie_pf_header_log_1;
#[doc = "PCIE_PF_HEADER_LOG_2 (r) register accessor: Header Log Register 2 Third DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_header_log_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_header_log_2`]
module"]
#[doc(alias = "PCIE_PF_HEADER_LOG_2")]
pub type PciePfHeaderLog2 = crate::Reg<pcie_pf_header_log_2::PciePfHeaderLog2Spec>;
#[doc = "Header Log Register 2 Third DWORD of captured TLP header STICKY."]
pub mod pcie_pf_header_log_2;
#[doc = "PCIE_PF_HEADER_LOG_3 (r) register accessor: Header Log Register 3 Fourth DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_header_log_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_header_log_3`]
module"]
#[doc(alias = "PCIE_PF_HEADER_LOG_3")]
pub type PciePfHeaderLog3 = crate::Reg<pcie_pf_header_log_3::PciePfHeaderLog3Spec>;
#[doc = "Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
pub mod pcie_pf_header_log_3;
#[doc = "PCIE_PF_ARI_EXTENDED_CAPABILITY_HEADER (r) register accessor: ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ari_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_ari_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_ARI_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfAriExtendedCapabilityHeader =
    crate::Reg<pcie_pf_ari_extended_capability_header::PciePfAriExtendedCapabilityHeaderSpec>;
#[doc = "ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_ari_extended_capability_header;
#[doc = "PCIE_PF_ARI_CAPABILITY_AND_ARI_CONTROL (r) register accessor: ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ari_capability_and_ari_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_ari_capability_and_ari_control`]
module"]
#[doc(alias = "PCIE_PF_ARI_CAPABILITY_AND_ARI_CONTROL")]
pub type PciePfAriCapabilityAndAriControl =
    crate::Reg<pcie_pf_ari_capability_and_ari_control::PciePfAriCapabilityAndAriControlSpec>;
#[doc = "ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0."]
pub mod pcie_pf_ari_capability_and_ari_control;
#[doc = "PCIE_PF_POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER (r) register accessor: Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_budgeting_enhanced_capability_header`]
module"]
#[doc(alias = "PCIE_PF_POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER")]
pub type PciePfPowerBudgetingEnhancedCapabilityHeader = crate :: Reg < pcie_pf_power_budgeting_enhanced_capability_header :: PciePfPowerBudgetingEnhancedCapabilityHeaderSpec > ;
#[doc = "Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_power_budgeting_enhanced_capability_header;
#[doc = "PCIE_PF_POWER_BUDGETING_DATA_SELECT (rw) register accessor: Power Budgeting Data Select Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_data_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_power_budgeting_data_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_budgeting_data_select`]
module"]
#[doc(alias = "PCIE_PF_POWER_BUDGETING_DATA_SELECT")]
pub type PciePfPowerBudgetingDataSelect =
    crate::Reg<pcie_pf_power_budgeting_data_select::PciePfPowerBudgetingDataSelectSpec>;
#[doc = "Power Budgeting Data Select Register (no description)"]
pub mod pcie_pf_power_budgeting_data_select;
#[doc = "PCIE_PF_POWER_BUDGETING_DATA (r) register accessor: Power Budgeting Data Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budgeting_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_budgeting_data`]
module"]
#[doc(alias = "PCIE_PF_POWER_BUDGETING_DATA")]
pub type PciePfPowerBudgetingData =
    crate::Reg<pcie_pf_power_budgeting_data::PciePfPowerBudgetingDataSpec>;
#[doc = "Power Budgeting Data Register Reserved"]
pub mod pcie_pf_power_budgeting_data;
#[doc = "PCIE_PF_POWER_BUDGET_CAPABILITY (r) register accessor: Power Budget Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_power_budget_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_power_budget_capability`]
module"]
#[doc(alias = "PCIE_PF_POWER_BUDGET_CAPABILITY")]
pub type PciePfPowerBudgetCapability =
    crate::Reg<pcie_pf_power_budget_capability::PciePfPowerBudgetCapabilitySpec>;
#[doc = "Power Budget Capability Register Reserved"]
pub mod pcie_pf_power_budget_capability;
#[doc = "PCIE_PF_RESIZABLE_BAR_EXTENDED_CAPABILITY_HEADER (r) register accessor: Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfResizableBarExtendedCapabilityHeader = crate :: Reg < pcie_pf_resizable_bar_extended_capability_header :: PciePfResizableBarExtendedCapabilityHeaderSpec > ;
#[doc = "Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_resizable_bar_extended_capability_header;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_0 (r) register accessor: Resizable BAR Capability Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_0`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_0")]
pub type PciePfResizableBarCapability0 =
    crate::Reg<pcie_pf_resizable_bar_capability_0::PciePfResizableBarCapability0Spec>;
#[doc = "Resizable BAR Capability Register 0 Reserved"]
pub mod pcie_pf_resizable_bar_capability_0;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_0 (r) register accessor: Resizable BAR Control Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_0`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_0")]
pub type PciePfResizableBarControl0 =
    crate::Reg<pcie_pf_resizable_bar_control_0::PciePfResizableBarControl0Spec>;
#[doc = "Resizable BAR Control Register 0 Reserved"]
pub mod pcie_pf_resizable_bar_control_0;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_1 (r) register accessor: Resizable BAR Capability Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_1`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_1")]
pub type PciePfResizableBarCapability1 =
    crate::Reg<pcie_pf_resizable_bar_capability_1::PciePfResizableBarCapability1Spec>;
#[doc = "Resizable BAR Capability Register 1 Reserved"]
pub mod pcie_pf_resizable_bar_capability_1;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_1 (r) register accessor: Resizable BAR Control Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_1`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_1")]
pub type PciePfResizableBarControl1 =
    crate::Reg<pcie_pf_resizable_bar_control_1::PciePfResizableBarControl1Spec>;
#[doc = "Resizable BAR Control Register 1 Reserved"]
pub mod pcie_pf_resizable_bar_control_1;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_2 (r) register accessor: Resizable BAR Capability Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_2`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_2")]
pub type PciePfResizableBarCapability2 =
    crate::Reg<pcie_pf_resizable_bar_capability_2::PciePfResizableBarCapability2Spec>;
#[doc = "Resizable BAR Capability Register 2 Reserved"]
pub mod pcie_pf_resizable_bar_capability_2;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_2 (r) register accessor: Resizable BAR Control Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_2`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_2")]
pub type PciePfResizableBarControl2 =
    crate::Reg<pcie_pf_resizable_bar_control_2::PciePfResizableBarControl2Spec>;
#[doc = "Resizable BAR Control Register 2 Reserved"]
pub mod pcie_pf_resizable_bar_control_2;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_3 (r) register accessor: Resizable BAR Capability Register 3 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_3`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_3")]
pub type PciePfResizableBarCapability3 =
    crate::Reg<pcie_pf_resizable_bar_capability_3::PciePfResizableBarCapability3Spec>;
#[doc = "Resizable BAR Capability Register 3 Reserved"]
pub mod pcie_pf_resizable_bar_capability_3;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_3 (r) register accessor: Resizable BAR Control Register 3 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_3`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_3")]
pub type PciePfResizableBarControl3 =
    crate::Reg<pcie_pf_resizable_bar_control_3::PciePfResizableBarControl3Spec>;
#[doc = "Resizable BAR Control Register 3 Reserved"]
pub mod pcie_pf_resizable_bar_control_3;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_4 (r) register accessor: Resizable BAR Capability Register 4 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_4`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_4")]
pub type PciePfResizableBarCapability4 =
    crate::Reg<pcie_pf_resizable_bar_capability_4::PciePfResizableBarCapability4Spec>;
#[doc = "Resizable BAR Capability Register 4 Reserved"]
pub mod pcie_pf_resizable_bar_capability_4;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_4 (r) register accessor: Resizable BAR Control Register 4 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_4`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_4")]
pub type PciePfResizableBarControl4 =
    crate::Reg<pcie_pf_resizable_bar_control_4::PciePfResizableBarControl4Spec>;
#[doc = "Resizable BAR Control Register 4 Reserved"]
pub mod pcie_pf_resizable_bar_control_4;
#[doc = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_5 (r) register accessor: Resizable BAR Capability Register 5 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_capability_5`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CAPABILITY_5")]
pub type PciePfResizableBarCapability5 =
    crate::Reg<pcie_pf_resizable_bar_capability_5::PciePfResizableBarCapability5Spec>;
#[doc = "Resizable BAR Capability Register 5 Reserved"]
pub mod pcie_pf_resizable_bar_capability_5;
#[doc = "PCIE_PF_RESIZABLE_BAR_CONTROL_5 (r) register accessor: Resizable BAR Control Register 5 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_resizable_bar_control_5`]
module"]
#[doc(alias = "PCIE_PF_RESIZABLE_BAR_CONTROL_5")]
pub type PciePfResizableBarControl5 =
    crate::Reg<pcie_pf_resizable_bar_control_5::PciePfResizableBarControl5Spec>;
#[doc = "Resizable BAR Control Register 5 Reserved"]
pub mod pcie_pf_resizable_bar_control_5;
#[doc = "PCIE_PF_LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER (r) register accessor: Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfLatencyToleranceReportingLtrExtendedCapabilityHeader = crate :: Reg < pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header :: PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec > ;
#[doc = "Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header;
#[doc = "PCIE_PF_LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY (rw) register accessor: LTR Max Snoop/Max No-Snoop Latency Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_ltr_max_snoop_max_no_snoop_latency`]
module"]
#[doc(alias = "PCIE_PF_LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY")]
pub type PciePfLtrMaxSnoopMaxNoSnoopLatency =
    crate::Reg<pcie_pf_ltr_max_snoop_max_no_snoop_latency::PciePfLtrMaxSnoopMaxNoSnoopLatencySpec>;
#[doc = "LTR Max Snoop/Max No-Snoop Latency Register Reserved"]
pub mod pcie_pf_ltr_max_snoop_max_no_snoop_latency;
#[doc = "PCIE_PF_DPA_EXTENDED_CAPABILITY_HEADER (r) register accessor: DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dpa_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_DPA_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfDpaExtendedCapabilityHeader =
    crate::Reg<pcie_pf_dpa_extended_capability_header::PciePfDpaExtendedCapabilityHeaderSpec>;
#[doc = "DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_dpa_extended_capability_header;
#[doc = "PCIE_PF_DPA_CAPABILITY (r) register accessor: DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dpa_capability`]
module"]
#[doc(alias = "PCIE_PF_DPA_CAPABILITY")]
pub type PciePfDpaCapability = crate::Reg<pcie_pf_dpa_capability::PciePfDpaCapabilitySpec>;
#[doc = "DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register."]
pub mod pcie_pf_dpa_capability;
#[doc = "PCIE_PF_DPA_LATENCY_INDICATOR (r) register accessor: DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_latency_indicator::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dpa_latency_indicator`]
module"]
#[doc(alias = "PCIE_PF_DPA_LATENCY_INDICATOR")]
pub type PciePfDpaLatencyIndicator =
    crate::Reg<pcie_pf_dpa_latency_indicator::PciePfDpaLatencyIndicatorSpec>;
#[doc = "DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
pub mod pcie_pf_dpa_latency_indicator;
#[doc = "PCIE_PF_DPA_CONTROL_AND_STATUS_S (rw) register accessor: DPA Control and Status Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_control_and_status_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_dpa_control_and_status_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dpa_control_and_status_s`]
module"]
#[doc(alias = "PCIE_PF_DPA_CONTROL_AND_STATUS_S")]
pub type PciePfDpaControlAndStatusS =
    crate::Reg<pcie_pf_dpa_control_and_status_s::PciePfDpaControlAndStatusSSpec>;
#[doc = "DPA Control and Status Registers Reserved"]
pub mod pcie_pf_dpa_control_and_status_s;
#[doc = "PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_0 (r) register accessor: Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dynamic_power_allocation_array_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dynamic_power_allocation_array_0`]
module"]
#[doc(alias = "PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_0")]
pub type PciePfDynamicPowerAllocationArray0 =
    crate::Reg<pcie_pf_dynamic_power_allocation_array_0::PciePfDynamicPowerAllocationArray0Spec>;
#[doc = "Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub mod pcie_pf_dynamic_power_allocation_array_0;
#[doc = "PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_1 (r) register accessor: Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dynamic_power_allocation_array_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_dynamic_power_allocation_array_1`]
module"]
#[doc(alias = "PCIE_PF_DYNAMIC_POWER_ALLOCATION_ARRAY_1")]
pub type PciePfDynamicPowerAllocationArray1 =
    crate::Reg<pcie_pf_dynamic_power_allocation_array_1::PciePfDynamicPowerAllocationArray1Spec>;
#[doc = "Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub mod pcie_pf_dynamic_power_allocation_array_1;
#[doc = "PCIE_PF_SR_IOV_EXTENDED_CAPABILITY_HEADER (r) register accessor: SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_sr_iov_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_SR_IOV_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfSrIovExtendedCapabilityHeader =
    crate::Reg<pcie_pf_sr_iov_extended_capability_header::PciePfSrIovExtendedCapabilityHeaderSpec>;
#[doc = "SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_sr_iov_extended_capability_header;
#[doc = "PCIE_PF_SR_IOV_CAPABILITIES (r) register accessor: SR-IOV Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_sr_iov_capabilities`]
module"]
#[doc(alias = "PCIE_PF_SR_IOV_CAPABILITIES")]
pub type PciePfSrIovCapabilities =
    crate::Reg<pcie_pf_sr_iov_capabilities::PciePfSrIovCapabilitiesSpec>;
#[doc = "SR-IOV Capabilities Register Reserved"]
pub mod pcie_pf_sr_iov_capabilities;
#[doc = "PCIE_PF_SR_IOV_CONTROL_AND_STATUS_S (rw) register accessor: SR-IOV Control and Status Registers Not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_control_and_status_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_sr_iov_control_and_status_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_sr_iov_control_and_status_s`]
module"]
#[doc(alias = "PCIE_PF_SR_IOV_CONTROL_AND_STATUS_S")]
pub type PciePfSrIovControlAndStatusS =
    crate::Reg<pcie_pf_sr_iov_control_and_status_s::PciePfSrIovControlAndStatusSSpec>;
#[doc = "SR-IOV Control and Status Registers Not implemented."]
pub mod pcie_pf_sr_iov_control_and_status_s;
#[doc = "PCIE_PF_INITIAL_VFS_TOTAL_VFS (r) register accessor: Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_initial_vfs_total_vfs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_initial_vfs_total_vfs`]
module"]
#[doc(alias = "PCIE_PF_INITIAL_VFS_TOTAL_VFS")]
pub type PciePfInitialVfsTotalVfs =
    crate::Reg<pcie_pf_initial_vfs_total_vfs::PciePfInitialVfsTotalVfsSpec>;
#[doc = "Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
pub mod pcie_pf_initial_vfs_total_vfs;
#[doc = "PCIE_PF_FUNCTION_DEPENDENCY_LINK_NUMVFS (rw) register accessor: Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_function_dependency_link_numvfs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_function_dependency_link_numvfs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_function_dependency_link_numvfs`]
module"]
#[doc(alias = "PCIE_PF_FUNCTION_DEPENDENCY_LINK_NUMVFS")]
pub type PciePfFunctionDependencyLinkNumvfs =
    crate::Reg<pcie_pf_function_dependency_link_numvfs::PciePfFunctionDependencyLinkNumvfsSpec>;
#[doc = "Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
pub mod pcie_pf_function_dependency_link_numvfs;
#[doc = "PCIE_PF_VF_OFFSET_STRIDE (r) register accessor: VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_offset_stride::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_offset_stride`]
module"]
#[doc(alias = "PCIE_PF_VF_OFFSET_STRIDE")]
pub type PciePfVfOffsetStride = crate::Reg<pcie_pf_vf_offset_stride::PciePfVfOffsetStrideSpec>;
#[doc = "VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
pub mod pcie_pf_vf_offset_stride;
#[doc = "PCIE_PF_VF_DEVICE_ID (r) register accessor: VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_device_id`]
module"]
#[doc(alias = "PCIE_PF_VF_DEVICE_ID")]
pub type PciePfVfDeviceId = crate::Reg<pcie_pf_vf_device_id::PciePfVfDeviceIdSpec>;
#[doc = "VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
pub mod pcie_pf_vf_device_id;
#[doc = "PCIE_PF_SUPPORTED_PAGE_SIZES (r) register accessor: Supported Page Sizes Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_supported_page_sizes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_supported_page_sizes`]
module"]
#[doc(alias = "PCIE_PF_SUPPORTED_PAGE_SIZES")]
pub type PciePfSupportedPageSizes =
    crate::Reg<pcie_pf_supported_page_sizes::PciePfSupportedPageSizesSpec>;
#[doc = "Supported Page Sizes Register Reserved"]
pub mod pcie_pf_supported_page_sizes;
#[doc = "PCIE_PF_SYSTEM_PAGE_SIZE (rw) register accessor: System Page Size Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_system_page_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_system_page_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_system_page_size`]
module"]
#[doc(alias = "PCIE_PF_SYSTEM_PAGE_SIZE")]
pub type PciePfSystemPageSize = crate::Reg<pcie_pf_system_page_size::PciePfSystemPageSizeSpec>;
#[doc = "System Page Size Register Reserved"]
pub mod pcie_pf_system_page_size;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_0 (rw) register accessor: VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_0`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_0")]
pub type PciePfVfBaseAddress0 = crate::Reg<pcie_pf_vf_base_address_0::PciePfVfBaseAddress0Spec>;
#[doc = "VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_vf_base_address_0;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_1 (rw) register accessor: VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_1`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_1")]
pub type PciePfVfBaseAddress1 = crate::Reg<pcie_pf_vf_base_address_1::PciePfVfBaseAddress1Spec>;
#[doc = "VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_vf_base_address_1;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_2 (rw) register accessor: VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_2`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_2")]
pub type PciePfVfBaseAddress2 = crate::Reg<pcie_pf_vf_base_address_2::PciePfVfBaseAddress2Spec>;
#[doc = "VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_vf_base_address_2;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_3 (rw) register accessor: VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_3`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_3")]
pub type PciePfVfBaseAddress3 = crate::Reg<pcie_pf_vf_base_address_3::PciePfVfBaseAddress3Spec>;
#[doc = "VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_vf_base_address_3;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_4 (rw) register accessor: VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_4`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_4")]
pub type PciePfVfBaseAddress4 = crate::Reg<pcie_pf_vf_base_address_4::PciePfVfBaseAddress4Spec>;
#[doc = "VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod pcie_pf_vf_base_address_4;
#[doc = "PCIE_PF_VF_BASE_ADDRESS_5 (rw) register accessor: VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_base_address_5`]
module"]
#[doc(alias = "PCIE_PF_VF_BASE_ADDRESS_5")]
pub type PciePfVfBaseAddress5 = crate::Reg<pcie_pf_vf_base_address_5::PciePfVfBaseAddress5Spec>;
#[doc = "VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod pcie_pf_vf_base_address_5;
#[doc = "PCIE_PF_VF_MIGRATION_STATE_ARRAY_OFFSET (r) register accessor: VF Migration State Array Offset Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_migration_state_array_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_vf_migration_state_array_offset`]
module"]
#[doc(alias = "PCIE_PF_VF_MIGRATION_STATE_ARRAY_OFFSET")]
pub type PciePfVfMigrationStateArrayOffset =
    crate::Reg<pcie_pf_vf_migration_state_array_offset::PciePfVfMigrationStateArrayOffsetSpec>;
#[doc = "VF Migration State Array Offset Register (no description)"]
pub mod pcie_pf_vf_migration_state_array_offset;
#[doc = "PCIE_PF_TPH_REQUESTER_EXTENDED_CAPABILITY_HEADER (r) register accessor: TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_requester_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_requester_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_TPH_REQUESTER_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfTphRequesterExtendedCapabilityHeader = crate :: Reg < pcie_pf_tph_requester_extended_capability_header :: PciePfTphRequesterExtendedCapabilityHeaderSpec > ;
#[doc = "TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_tph_requester_extended_capability_header;
#[doc = "PCIE_PF_TPH_REQUESTER_CAPABILITY (r) register accessor: TPH Requester Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_requester_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_requester_capability`]
module"]
#[doc(alias = "PCIE_PF_TPH_REQUESTER_CAPABILITY")]
pub type PciePfTphRequesterCapability =
    crate::Reg<pcie_pf_tph_requester_capability::PciePfTphRequesterCapabilitySpec>;
#[doc = "TPH Requester Capability Register Reserved"]
pub mod pcie_pf_tph_requester_capability;
#[doc = "PCIE_PF_TPH_REQUESTER_CONTROL (rw) register accessor: TPH Requester Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_requester_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_requester_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_requester_control`]
module"]
#[doc(alias = "PCIE_PF_TPH_REQUESTER_CONTROL")]
pub type PciePfTphRequesterControl =
    crate::Reg<pcie_pf_tph_requester_control::PciePfTphRequesterControlSpec>;
#[doc = "TPH Requester Control Register Reserved"]
pub mod pcie_pf_tph_requester_control;
#[doc = "PCIE_PF_TPH_ST_TABLE_0 (rw) register accessor: TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_st_table_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_st_table_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_st_table_0`]
module"]
#[doc(alias = "PCIE_PF_TPH_ST_TABLE_0")]
pub type PciePfTphStTable0 = crate::Reg<pcie_pf_tph_st_table_0::PciePfTphStTable0Spec>;
#[doc = "TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_pf_tph_st_table_0;
#[doc = "PCIE_PF_TPH_ST_TABLE_1 (rw) register accessor: TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_st_table_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_st_table_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_st_table_1`]
module"]
#[doc(alias = "PCIE_PF_TPH_ST_TABLE_1")]
pub type PciePfTphStTable1 = crate::Reg<pcie_pf_tph_st_table_1::PciePfTphStTable1Spec>;
#[doc = "TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_pf_tph_st_table_1;
#[doc = "PCIE_PF_TPH_ST_TABLE_2 (rw) register accessor: TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_st_table_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_st_table_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_st_table_2`]
module"]
#[doc(alias = "PCIE_PF_TPH_ST_TABLE_2")]
pub type PciePfTphStTable2 = crate::Reg<pcie_pf_tph_st_table_2::PciePfTphStTable2Spec>;
#[doc = "TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_pf_tph_st_table_2;
#[doc = "PCIE_PF_TPH_ST_TABLE_3 (rw) register accessor: TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_st_table_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_st_table_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_tph_st_table_3`]
module"]
#[doc(alias = "PCIE_PF_TPH_ST_TABLE_3")]
pub type PciePfTphStTable3 = crate::Reg<pcie_pf_tph_st_table_3::PciePfTphStTable3Spec>;
#[doc = "TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_pf_tph_st_table_3;
#[doc = "PCIE_PF_L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER (r) register accessor: L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_l1_pm_substates_extended_capability_header`]
module"]
#[doc(alias = "PCIE_PF_L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER")]
pub type PciePfL1PmSubstatesExtendedCapabilityHeader = crate :: Reg < pcie_pf_l1_pm_substates_extended_capability_header :: PciePfL1PmSubstatesExtendedCapabilityHeaderSpec > ;
#[doc = "L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_pf_l1_pm_substates_extended_capability_header;
#[doc = "PCIE_PF_L1_PM_SUBSTATES_CAPABILITIES (r) register accessor: L1 PM Substates Capabilities Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_l1_pm_substates_capabilities`]
module"]
#[doc(alias = "PCIE_PF_L1_PM_SUBSTATES_CAPABILITIES")]
pub type PciePfL1PmSubstatesCapabilities =
    crate::Reg<pcie_pf_l1_pm_substates_capabilities::PciePfL1PmSubstatesCapabilitiesSpec>;
#[doc = "L1 PM Substates Capabilities Register RSVD"]
pub mod pcie_pf_l1_pm_substates_capabilities;
#[doc = "PCIE_PF_L1_PM_SUBSTATES_CONTROL_1 (rw) register accessor: L1 PM Substates Control 1 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_control_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_l1_pm_substates_control_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_l1_pm_substates_control_1`]
module"]
#[doc(alias = "PCIE_PF_L1_PM_SUBSTATES_CONTROL_1")]
pub type PciePfL1PmSubstatesControl1 =
    crate::Reg<pcie_pf_l1_pm_substates_control_1::PciePfL1PmSubstatesControl1Spec>;
#[doc = "L1 PM Substates Control 1 Register (no description)"]
pub mod pcie_pf_l1_pm_substates_control_1;
#[doc = "PCIE_PF_L1_PM_SUBSTATES_CONTROL_2 (rw) register accessor: L1 PM Substates Control 2 Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_control_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_l1_pm_substates_control_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_pf_l1_pm_substates_control_2`]
module"]
#[doc(alias = "PCIE_PF_L1_PM_SUBSTATES_CONTROL_2")]
pub type PciePfL1PmSubstatesControl2 =
    crate::Reg<pcie_pf_l1_pm_substates_control_2::PciePfL1PmSubstatesControl2Spec>;
#[doc = "L1 PM Substates Control 2 Register RSVD"]
pub mod pcie_pf_l1_pm_substates_control_2;
#[doc = "PCIE_VF_VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_vendor_id_and_device_id`]
module"]
#[doc(alias = "PCIE_VF_VENDOR_ID_AND_DEVICE_ID")]
pub type PcieVfVendorIdAndDeviceId =
    crate::Reg<pcie_vf_vendor_id_and_device_id::PcieVfVendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus."]
pub mod pcie_vf_vendor_id_and_device_id;
#[doc = "PCIE_VF_COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_command_and_status`]
module"]
#[doc(alias = "PCIE_VF_COMMAND_AND_STATUS")]
pub type PcieVfCommandAndStatus =
    crate::Reg<pcie_vf_command_and_status::PcieVfCommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
pub mod pcie_vf_command_and_status;
#[doc = "PCIE_VF_REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_revision_id_and_class_code`]
module"]
#[doc(alias = "PCIE_VF_REVISION_ID_AND_CLASS_CODE")]
pub type PcieVfRevisionIdAndClassCode =
    crate::Reg<pcie_vf_revision_id_and_class_code::PcieVfRevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub mod pcie_vf_revision_id_and_class_code;
#[doc = "PCIE_VF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (r) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "PCIE_VF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s :: PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved"]
pub mod pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "PCIE_VF_BASE_ADDRESS_0 (r) register accessor: Base Address Register 0 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_0`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_0")]
pub type PcieVfBaseAddress0 = crate::Reg<pcie_vf_base_address_0::PcieVfBaseAddress0Spec>;
#[doc = "Base Address Register 0 (no description)"]
pub mod pcie_vf_base_address_0;
#[doc = "PCIE_VF_BASE_ADDRESS_1 (r) register accessor: Base Address Register 1 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_1`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_1")]
pub type PcieVfBaseAddress1 = crate::Reg<pcie_vf_base_address_1::PcieVfBaseAddress1Spec>;
#[doc = "Base Address Register 1 (no description)"]
pub mod pcie_vf_base_address_1;
#[doc = "PCIE_VF_BASE_ADDRESS_2 (r) register accessor: Base Address Register 2 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_2`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_2")]
pub type PcieVfBaseAddress2 = crate::Reg<pcie_vf_base_address_2::PcieVfBaseAddress2Spec>;
#[doc = "Base Address Register 2 (no description)"]
pub mod pcie_vf_base_address_2;
#[doc = "PCIE_VF_BASE_ADDRESS_3 (r) register accessor: Base Address Register 3 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_3`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_3")]
pub type PcieVfBaseAddress3 = crate::Reg<pcie_vf_base_address_3::PcieVfBaseAddress3Spec>;
#[doc = "Base Address Register 3 (no description)"]
pub mod pcie_vf_base_address_3;
#[doc = "PCIE_VF_BASE_ADDRESS_4 (r) register accessor: Base Address Register 4 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_4`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_4")]
pub type PcieVfBaseAddress4 = crate::Reg<pcie_vf_base_address_4::PcieVfBaseAddress4Spec>;
#[doc = "Base Address Register 4 (no description)"]
pub mod pcie_vf_base_address_4;
#[doc = "PCIE_VF_BASE_ADDRESS_5 (r) register accessor: Base Address Register 5 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_base_address_5`]
module"]
#[doc(alias = "PCIE_VF_BASE_ADDRESS_5")]
pub type PcieVfBaseAddress5 = crate::Reg<pcie_vf_base_address_5::PcieVfBaseAddress5Spec>;
#[doc = "Base Address Register 5 (no description)"]
pub mod pcie_vf_base_address_5;
#[doc = "PCIE_VF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID (r) register accessor: Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_subsystem_vendor_id_and_subsystem_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_subsystem_vendor_id_and_subsystem_id`]
module"]
#[doc(alias = "PCIE_VF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID")]
pub type PcieVfSubsystemVendorIdAndSubsystemId = crate::Reg<
    pcie_vf_subsystem_vendor_id_and_subsystem_id::PcieVfSubsystemVendorIdAndSubsystemIdSpec,
>;
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub mod pcie_vf_subsystem_vendor_id_and_subsystem_id;
#[doc = "PCIE_VF_EXPANSION_ROM_BASE_ADDRESS (r) register accessor: Expansion ROM Base Address Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_expansion_rom_base_address::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_expansion_rom_base_address`]
module"]
#[doc(alias = "PCIE_VF_EXPANSION_ROM_BASE_ADDRESS")]
pub type PcieVfExpansionRomBaseAddress =
    crate::Reg<pcie_vf_expansion_rom_base_address::PcieVfExpansionRomBaseAddressSpec>;
#[doc = "Expansion ROM Base Address Register (no description)"]
pub mod pcie_vf_expansion_rom_base_address;
#[doc = "PCIE_VF_CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_capabilities_pointer`]
module"]
#[doc(alias = "PCIE_VF_CAPABILITIES_POINTER")]
pub type PcieVfCapabilitiesPointer =
    crate::Reg<pcie_vf_capabilities_pointer::PcieVfCapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod pcie_vf_capabilities_pointer;
#[doc = "PCIE_VF_INTERRUPT_LINE_AND_INTERRUPT_PIN (r) register accessor: Interrupt Line and Interrupt Pin Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_interrupt_line_and_interrupt_pin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_interrupt_line_and_interrupt_pin`]
module"]
#[doc(alias = "PCIE_VF_INTERRUPT_LINE_AND_INTERRUPT_PIN")]
pub type PcieVfInterruptLineAndInterruptPin =
    crate::Reg<pcie_vf_interrupt_line_and_interrupt_pin::PcieVfInterruptLineAndInterruptPinSpec>;
#[doc = "Interrupt Line and Interrupt Pin Register (no description)"]
pub mod pcie_vf_interrupt_line_and_interrupt_pin;
#[doc = "PCIE_VF_POWER_MANAGEMENT_CAPABILITIES (r) register accessor: Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_power_management_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_power_management_capabilities`]
module"]
#[doc(alias = "PCIE_VF_POWER_MANAGEMENT_CAPABILITIES")]
pub type PcieVfPowerManagementCapabilities =
    crate::Reg<pcie_vf_power_management_capabilities::PcieVfPowerManagementCapabilitiesSpec>;
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
pub mod pcie_vf_power_management_capabilities;
#[doc = "PCIE_VF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT (rw) register accessor: Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_power_management_control_status_report::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_power_management_control_status_report::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_power_management_control_status_report`]
module"]
#[doc(alias = "PCIE_VF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT")]
pub type PcieVfPowerManagementControlStatusReport = crate::Reg<
    pcie_vf_power_management_control_status_report::PcieVfPowerManagementControlStatusReportSpec,
>;
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub mod pcie_vf_power_management_control_status_report;
#[doc = "PCIE_VF_MSI_CONTROL (rw) register accessor: MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_control`]
module"]
#[doc(alias = "PCIE_VF_MSI_CONTROL")]
pub type PcieVfMsiControl = crate::Reg<pcie_vf_msi_control::PcieVfMsiControlSpec>;
#[doc = "MSI Control Register Reserved"]
pub mod pcie_vf_msi_control;
#[doc = "PCIE_VF_MSI_MESSAGE_LOW_ADDRESS (rw) register accessor: MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_message_low_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_message_low_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_message_low_address`]
module"]
#[doc(alias = "PCIE_VF_MSI_MESSAGE_LOW_ADDRESS")]
pub type PcieVfMsiMessageLowAddress =
    crate::Reg<pcie_vf_msi_message_low_address::PcieVfMsiMessageLowAddressSpec>;
#[doc = "MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub mod pcie_vf_msi_message_low_address;
#[doc = "PCIE_VF_MSI_MESSAGE_HIGH_ADDRESS (rw) register accessor: MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_message_high_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_message_high_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_message_high_address`]
module"]
#[doc(alias = "PCIE_VF_MSI_MESSAGE_HIGH_ADDRESS")]
pub type PcieVfMsiMessageHighAddress =
    crate::Reg<pcie_vf_msi_message_high_address::PcieVfMsiMessageHighAddressSpec>;
#[doc = "MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub mod pcie_vf_msi_message_high_address;
#[doc = "PCIE_VF_MSI_MESSAGE_DATA (rw) register accessor: MSI Message Data Register Hardwired to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_message_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_message_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_message_data`]
module"]
#[doc(alias = "PCIE_VF_MSI_MESSAGE_DATA")]
pub type PcieVfMsiMessageData = crate::Reg<pcie_vf_msi_message_data::PcieVfMsiMessageDataSpec>;
#[doc = "MSI Message Data Register Hardwired to 0"]
pub mod pcie_vf_msi_message_data;
#[doc = "PCIE_VF_MSI_MASK (rw) register accessor: MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_mask`]
module"]
#[doc(alias = "PCIE_VF_MSI_MASK")]
pub type PcieVfMsiMask = crate::Reg<pcie_vf_msi_mask::PcieVfMsiMaskSpec>;
#[doc = "MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod pcie_vf_msi_mask;
#[doc = "PCIE_VF_MSI_PENDING_BITS (r) register accessor: MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_pending_bits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_pending_bits`]
module"]
#[doc(alias = "PCIE_VF_MSI_PENDING_BITS")]
pub type PcieVfMsiPendingBits = crate::Reg<pcie_vf_msi_pending_bits::PcieVfMsiPendingBitsSpec>;
#[doc = "MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod pcie_vf_msi_pending_bits;
#[doc = "PCIE_VF_MSI_X_CONTROL (rw) register accessor: MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_x_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_x_control`]
module"]
#[doc(alias = "PCIE_VF_MSI_X_CONTROL")]
pub type PcieVfMsiXControl = crate::Reg<pcie_vf_msi_x_control::PcieVfMsiXControlSpec>;
#[doc = "MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub mod pcie_vf_msi_x_control;
#[doc = "PCIE_VF_MSI_X_TABLE_OFFSET (r) register accessor: MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_table_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_x_table_offset`]
module"]
#[doc(alias = "PCIE_VF_MSI_X_TABLE_OFFSET")]
pub type PcieVfMsiXTableOffset = crate::Reg<pcie_vf_msi_x_table_offset::PcieVfMsiXTableOffsetSpec>;
#[doc = "MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_vf_msi_x_table_offset;
#[doc = "PCIE_VF_MSI_X_PENDING_INTERRUPT (r) register accessor: MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_pending_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_msi_x_pending_interrupt`]
module"]
#[doc(alias = "PCIE_VF_MSI_X_PENDING_INTERRUPT")]
pub type PcieVfMsiXPendingInterrupt =
    crate::Reg<pcie_vf_msi_x_pending_interrupt::PcieVfMsiXPendingInterruptSpec>;
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_vf_msi_x_pending_interrupt;
#[doc = "PCIE_VF_PCI_EXPRESS_CAPABILITY_LIST (r) register accessor: PCI Express Capability List Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_capability_list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_pci_express_capability_list`]
module"]
#[doc(alias = "PCIE_VF_PCI_EXPRESS_CAPABILITY_LIST")]
pub type PcieVfPciExpressCapabilityList =
    crate::Reg<pcie_vf_pci_express_capability_list::PcieVfPciExpressCapabilityListSpec>;
#[doc = "PCI Express Capability List Register Reserved"]
pub mod pcie_vf_pci_express_capability_list;
#[doc = "PCIE_VF_PCI_EXPRESS_DEVICE_CAPABILITIES (r) register accessor: PCI Express Device Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_device_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_pci_express_device_capabilities`]
module"]
#[doc(alias = "PCIE_VF_PCI_EXPRESS_DEVICE_CAPABILITIES")]
pub type PcieVfPciExpressDeviceCapabilities =
    crate::Reg<pcie_vf_pci_express_device_capabilities::PcieVfPciExpressDeviceCapabilitiesSpec>;
#[doc = "PCI Express Device Capabilities Register Reserved"]
pub mod pcie_vf_pci_express_device_capabilities;
#[doc = "PCIE_VF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS (rw) register accessor: PCI Express Device Control and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_device_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_pci_express_device_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_pci_express_device_control_and_status`]
module"]
#[doc(alias = "PCIE_VF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS")]
pub type PcieVfPciExpressDeviceControlAndStatus = crate::Reg<
    pcie_vf_pci_express_device_control_and_status::PcieVfPciExpressDeviceControlAndStatusSpec,
>;
#[doc = "PCI Express Device Control and Status Register Reserved"]
pub mod pcie_vf_pci_express_device_control_and_status;
#[doc = "PCIE_VF_LINK_CAPABILITIES (r) register accessor: Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_link_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_link_capabilities`]
module"]
#[doc(alias = "PCIE_VF_LINK_CAPABILITIES")]
pub type PcieVfLinkCapabilities = crate::Reg<pcie_vf_link_capabilities::PcieVfLinkCapabilitiesSpec>;
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
pub mod pcie_vf_link_capabilities;
#[doc = "PCIE_VF_PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCIE_VF_PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PcieVfPciExpressDeviceCapabilities2 =
    crate::Reg<pcie_vf_pci_express_device_capabilities_2::PcieVfPciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities Register 2 Reserved"]
pub mod pcie_vf_pci_express_device_capabilities_2;
#[doc = "PCIE_VF_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER (r) register accessor: Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_advanced_error_reporting_aer_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_advanced_error_reporting_aer_enhanced_capability_header`]
module"]
#[doc(alias = "PCIE_VF_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER")]
pub type PcieVfAdvancedErrorReportingAerEnhancedCapabilityHeader = crate :: Reg < pcie_vf_advanced_error_reporting_aer_enhanced_capability_header :: PcieVfAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec > ;
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_vf_advanced_error_reporting_aer_enhanced_capability_header;
#[doc = "PCIE_VF_UNCORRECTABLE_ERROR_STATUS (rw) register accessor: Uncorrectable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_uncorrectable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_uncorrectable_error_status`]
module"]
#[doc(alias = "PCIE_VF_UNCORRECTABLE_ERROR_STATUS")]
pub type PcieVfUncorrectableErrorStatus =
    crate::Reg<pcie_vf_uncorrectable_error_status::PcieVfUncorrectableErrorStatusSpec>;
#[doc = "Uncorrectable Error Status Register Reserved"]
pub mod pcie_vf_uncorrectable_error_status;
#[doc = "PCIE_VF_UNCORRECTABLE_ERROR_MASK (r) register accessor: Uncorrectable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_mask::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_uncorrectable_error_mask`]
module"]
#[doc(alias = "PCIE_VF_UNCORRECTABLE_ERROR_MASK")]
pub type PcieVfUncorrectableErrorMask =
    crate::Reg<pcie_vf_uncorrectable_error_mask::PcieVfUncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register (no description)"]
pub mod pcie_vf_uncorrectable_error_mask;
#[doc = "PCIE_VF_UNCORRECTABLE_ERROR_SEVERITY (r) register accessor: Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_severity::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_uncorrectable_error_severity`]
module"]
#[doc(alias = "PCIE_VF_UNCORRECTABLE_ERROR_SEVERITY")]
pub type PcieVfUncorrectableErrorSeverity =
    crate::Reg<pcie_vf_uncorrectable_error_severity::PcieVfUncorrectableErrorSeveritySpec>;
#[doc = "Uncorrectable Error Severity Register (no description)"]
pub mod pcie_vf_uncorrectable_error_severity;
#[doc = "PCIE_VF_CORRECTABLE_ERROR_STATUS (rw) register accessor: Correctable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_correctable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_correctable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_correctable_error_status`]
module"]
#[doc(alias = "PCIE_VF_CORRECTABLE_ERROR_STATUS")]
pub type PcieVfCorrectableErrorStatus =
    crate::Reg<pcie_vf_correctable_error_status::PcieVfCorrectableErrorStatusSpec>;
#[doc = "Correctable Error Status Register Reserved"]
pub mod pcie_vf_correctable_error_status;
#[doc = "PCIE_VF_CORRECTABLE_ERROR_MASK (r) register accessor: Correctable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_correctable_error_mask::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_correctable_error_mask`]
module"]
#[doc(alias = "PCIE_VF_CORRECTABLE_ERROR_MASK")]
pub type PcieVfCorrectableErrorMask =
    crate::Reg<pcie_vf_correctable_error_mask::PcieVfCorrectableErrorMaskSpec>;
#[doc = "Correctable Error Mask Register (no description)"]
pub mod pcie_vf_correctable_error_mask;
#[doc = "PCIE_VF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL (r) register accessor: Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_advanced_error_capabilities_and_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_advanced_error_capabilities_and_control`]
module"]
#[doc(alias = "PCIE_VF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL")]
pub type PcieVfAdvancedErrorCapabilitiesAndControl = crate::Reg<
    pcie_vf_advanced_error_capabilities_and_control::PcieVfAdvancedErrorCapabilitiesAndControlSpec,
>;
#[doc = "Advanced Error Capabilities and Control Register Reserved"]
pub mod pcie_vf_advanced_error_capabilities_and_control;
#[doc = "PCIE_VF_HEADER_LOG_0 (r) register accessor: Header Log Register 0 First DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_header_log_0`]
module"]
#[doc(alias = "PCIE_VF_HEADER_LOG_0")]
pub type PcieVfHeaderLog0 = crate::Reg<pcie_vf_header_log_0::PcieVfHeaderLog0Spec>;
#[doc = "Header Log Register 0 First DWORD of captured TLP header STICKY."]
pub mod pcie_vf_header_log_0;
#[doc = "PCIE_VF_HEADER_LOG_1 (r) register accessor: Header Log Register 1 Second DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_header_log_1`]
module"]
#[doc(alias = "PCIE_VF_HEADER_LOG_1")]
pub type PcieVfHeaderLog1 = crate::Reg<pcie_vf_header_log_1::PcieVfHeaderLog1Spec>;
#[doc = "Header Log Register 1 Second DWORD of captured TLP header STICKY."]
pub mod pcie_vf_header_log_1;
#[doc = "PCIE_VF_HEADER_LOG_2 (r) register accessor: Header Log Register 2 Third DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_header_log_2`]
module"]
#[doc(alias = "PCIE_VF_HEADER_LOG_2")]
pub type PcieVfHeaderLog2 = crate::Reg<pcie_vf_header_log_2::PcieVfHeaderLog2Spec>;
#[doc = "Header Log Register 2 Third DWORD of captured TLP header STICKY."]
pub mod pcie_vf_header_log_2;
#[doc = "PCIE_VF_HEADER_LOG_3 (r) register accessor: Header Log Register 3 Fourth DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_header_log_3`]
module"]
#[doc(alias = "PCIE_VF_HEADER_LOG_3")]
pub type PcieVfHeaderLog3 = crate::Reg<pcie_vf_header_log_3::PcieVfHeaderLog3Spec>;
#[doc = "Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
pub mod pcie_vf_header_log_3;
#[doc = "PCIE_VF_ARI_EXTENDED_CAPABILITY_HEADER (r) register accessor: ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_ari_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_ari_extended_capability_header`]
module"]
#[doc(alias = "PCIE_VF_ARI_EXTENDED_CAPABILITY_HEADER")]
pub type PcieVfAriExtendedCapabilityHeader =
    crate::Reg<pcie_vf_ari_extended_capability_header::PcieVfAriExtendedCapabilityHeaderSpec>;
#[doc = "ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_vf_ari_extended_capability_header;
#[doc = "PCIE_VF_ARI_CAPABILITY_AND_ARI_CONTROL (r) register accessor: ARI Capability Register and ARI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_ari_capability_and_ari_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_ari_capability_and_ari_control`]
module"]
#[doc(alias = "PCIE_VF_ARI_CAPABILITY_AND_ARI_CONTROL")]
pub type PcieVfAriCapabilityAndAriControl =
    crate::Reg<pcie_vf_ari_capability_and_ari_control::PcieVfAriCapabilityAndAriControlSpec>;
#[doc = "ARI Capability Register and ARI Control Register Reserved"]
pub mod pcie_vf_ari_capability_and_ari_control;
#[doc = "PCIE_VF_TPH_REQUESTER_ENHANCED_CAPABILITY_HEADER (r) register accessor: TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_requester_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_requester_enhanced_capability_header`]
module"]
#[doc(alias = "PCIE_VF_TPH_REQUESTER_ENHANCED_CAPABILITY_HEADER")]
pub type PcieVfTphRequesterEnhancedCapabilityHeader = crate :: Reg < pcie_vf_tph_requester_enhanced_capability_header :: PcieVfTphRequesterEnhancedCapabilityHeaderSpec > ;
#[doc = "TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_vf_tph_requester_enhanced_capability_header;
#[doc = "PCIE_VF_TPH_REQUESTER_CAPABILITY (r) register accessor: TPH Requester Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_requester_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_requester_capability`]
module"]
#[doc(alias = "PCIE_VF_TPH_REQUESTER_CAPABILITY")]
pub type PcieVfTphRequesterCapability =
    crate::Reg<pcie_vf_tph_requester_capability::PcieVfTphRequesterCapabilitySpec>;
#[doc = "TPH Requester Capability Register Reserved"]
pub mod pcie_vf_tph_requester_capability;
#[doc = "PCIE_VF_TPH_REQUESTER_CONTROL (rw) register accessor: TPH Requester Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_requester_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_requester_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_requester_control`]
module"]
#[doc(alias = "PCIE_VF_TPH_REQUESTER_CONTROL")]
pub type PcieVfTphRequesterControl =
    crate::Reg<pcie_vf_tph_requester_control::PcieVfTphRequesterControlSpec>;
#[doc = "TPH Requester Control Register Reserved"]
pub mod pcie_vf_tph_requester_control;
#[doc = "PCIE_VF_TPH_ST_TABLE_0 (rw) register accessor: TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_st_table_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_st_table_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_st_table_0`]
module"]
#[doc(alias = "PCIE_VF_TPH_ST_TABLE_0")]
pub type PcieVfTphStTable0 = crate::Reg<pcie_vf_tph_st_table_0::PcieVfTphStTable0Spec>;
#[doc = "TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_vf_tph_st_table_0;
#[doc = "PCIE_VF_TPH_ST_TABLE_1 (rw) register accessor: TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_st_table_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_st_table_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_st_table_1`]
module"]
#[doc(alias = "PCIE_VF_TPH_ST_TABLE_1")]
pub type PcieVfTphStTable1 = crate::Reg<pcie_vf_tph_st_table_1::PcieVfTphStTable1Spec>;
#[doc = "TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_vf_tph_st_table_1;
#[doc = "PCIE_VF_TPH_ST_TABLE_2 (rw) register accessor: TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_st_table_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_st_table_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_vf_tph_st_table_2`]
module"]
#[doc(alias = "PCIE_VF_TPH_ST_TABLE_2")]
pub type PcieVfTphStTable2 = crate::Reg<pcie_vf_tph_st_table_2::PcieVfTphStTable2Spec>;
#[doc = "TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_vf_tph_st_table_2;
#[doc = "PCIE_RC_TPH_ST_TABLE_3 (rw) register accessor: TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_tph_st_table_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_tph_st_table_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_tph_st_table_3`]
module"]
#[doc(alias = "PCIE_RC_TPH_ST_TABLE_3")]
pub type PcieRcTphStTable3 = crate::Reg<pcie_rc_tph_st_table_3::PcieRcTphStTable3Spec>;
#[doc = "TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod pcie_rc_tph_st_table_3;
#[doc = "PCIE_RC_VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_vendor_id_and_device_id`]
module"]
#[doc(alias = "PCIE_RC_VENDOR_ID_AND_DEVICE_ID")]
pub type PcieRcVendorIdAndDeviceId =
    crate::Reg<pcie_rc_vendor_id_and_device_id::PcieRcVendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_vendor_id_and_device_id;
#[doc = "PCIE_RC_COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_command_and_status`]
module"]
#[doc(alias = "PCIE_RC_COMMAND_AND_STATUS")]
pub type PcieRcCommandAndStatus =
    crate::Reg<pcie_rc_command_and_status::PcieRcCommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_command_and_status;
#[doc = "PCIE_RC_REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_revision_id_and_class_code`]
module"]
#[doc(alias = "PCIE_RC_REVISION_ID_AND_CLASS_CODE")]
pub type PcieRcRevisionIdAndClassCode =
    crate::Reg<pcie_rc_revision_id_and_class_code::PcieRcRevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_revision_id_and_class_code;
#[doc = "PCIE_RC_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (rw) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "PCIE_RC_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type PcieRcBistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s :: PcieRcBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_0 (rw) register accessor: Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_complex_base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_complex_base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_complex_base_address_0`]
module"]
#[doc(alias = "PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_0")]
pub type PcieRcRootComplexBaseAddress0 =
    crate::Reg<pcie_rc_root_complex_base_address_0::PcieRcRootComplexBaseAddress0Spec>;
#[doc = "Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub mod pcie_rc_root_complex_base_address_0;
#[doc = "PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_1 (rw) register accessor: Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_complex_base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_complex_base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_complex_base_address_1`]
module"]
#[doc(alias = "PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_1")]
pub type PcieRcRootComplexBaseAddress1 =
    crate::Reg<pcie_rc_root_complex_base_address_1::PcieRcRootComplexBaseAddress1Spec>;
#[doc = "Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub mod pcie_rc_root_complex_base_address_1;
#[doc = "PCIE_RC_PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER (rw) register accessor: Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer`]
module"]
#[doc(
    alias = "PCIE_RC_PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER"
)]
pub type PcieRcPrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer = crate :: Reg < pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer :: PcieRcPrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec > ;
#[doc = "Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented."]
pub mod pcie_rc_primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer;
#[doc = "PCIE_RC_IO_BASE_IO_LIMIT_SECONDARY_STATUS (rw) register accessor: IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_io_base_io_limit_secondary_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_io_base_io_limit_secondary_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_io_base_io_limit_secondary_status`]
module"]
#[doc(alias = "PCIE_RC_IO_BASE_IO_LIMIT_SECONDARY_STATUS")]
pub type PcieRcIoBaseIoLimitSecondaryStatus =
    crate::Reg<pcie_rc_io_base_io_limit_secondary_status::PcieRcIoBaseIoLimitSecondaryStatusSpec>;
#[doc = "IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_io_base_io_limit_secondary_status;
#[doc = "PCIE_RC_MEMORY_BASE_MEMORY_LIMIT (rw) register accessor: Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_memory_base_memory_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_memory_base_memory_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_memory_base_memory_limit`]
module"]
#[doc(alias = "PCIE_RC_MEMORY_BASE_MEMORY_LIMIT")]
pub type PcieRcMemoryBaseMemoryLimit =
    crate::Reg<pcie_rc_memory_base_memory_limit::PcieRcMemoryBaseMemoryLimitSpec>;
#[doc = "Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub mod pcie_rc_memory_base_memory_limit;
#[doc = "PCIE_RC_PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT (r) register accessor: Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_memory_base_prefetchable_memory_limit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_prefetchable_memory_base_prefetchable_memory_limit`]
module"]
#[doc(alias = "PCIE_RC_PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT")]
pub type PcieRcPrefetchableMemoryBasePrefetchableMemoryLimit = crate :: Reg < pcie_rc_prefetchable_memory_base_prefetchable_memory_limit :: PcieRcPrefetchableMemoryBasePrefetchableMemoryLimitSpec > ;
#[doc = "Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod pcie_rc_prefetchable_memory_base_prefetchable_memory_limit;
#[doc = "PCIE_RC_PREFETCHABLE_BASE_UPPER (r) register accessor: Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_base_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_prefetchable_base_upper`]
module"]
#[doc(alias = "PCIE_RC_PREFETCHABLE_BASE_UPPER")]
pub type PcieRcPrefetchableBaseUpper =
    crate::Reg<pcie_rc_prefetchable_base_upper::PcieRcPrefetchableBaseUpperSpec>;
#[doc = "Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod pcie_rc_prefetchable_base_upper;
#[doc = "PCIE_RC_PREFETCHABLE_LIMIT_UPPER (r) register accessor: Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_prefetchable_limit_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_prefetchable_limit_upper`]
module"]
#[doc(alias = "PCIE_RC_PREFETCHABLE_LIMIT_UPPER")]
pub type PcieRcPrefetchableLimitUpper =
    crate::Reg<pcie_rc_prefetchable_limit_upper::PcieRcPrefetchableLimitUpperSpec>;
#[doc = "Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod pcie_rc_prefetchable_limit_upper;
#[doc = "PCIE_RC_IO_BASE_UPPER_IO_LIMIT_UPPER (r) register accessor: IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_io_base_upper_io_limit_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_io_base_upper_io_limit_upper`]
module"]
#[doc(alias = "PCIE_RC_IO_BASE_UPPER_IO_LIMIT_UPPER")]
pub type PcieRcIoBaseUpperIoLimitUpper =
    crate::Reg<pcie_rc_io_base_upper_io_limit_upper::PcieRcIoBaseUpperIoLimitUpperSpec>;
#[doc = "IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod pcie_rc_io_base_upper_io_limit_upper;
#[doc = "PCIE_RC_CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_capabilities_pointer`]
module"]
#[doc(alias = "PCIE_RC_CAPABILITIES_POINTER")]
pub type PcieRcCapabilitiesPointer =
    crate::Reg<pcie_rc_capabilities_pointer::PcieRcCapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod pcie_rc_capabilities_pointer;
#[doc = "PCIE_RC_INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL (rw) register accessor: Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_interrupt_line_interrupt_pin_and_bridge_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_interrupt_line_interrupt_pin_and_bridge_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_interrupt_line_interrupt_pin_and_bridge_control`]
module"]
#[doc(alias = "PCIE_RC_INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL")]
pub type PcieRcInterruptLineInterruptPinAndBridgeControl = crate :: Reg < pcie_rc_interrupt_line_interrupt_pin_and_bridge_control :: PcieRcInterruptLineInterruptPinAndBridgeControlSpec > ;
#[doc = "Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved"]
pub mod pcie_rc_interrupt_line_interrupt_pin_and_bridge_control;
#[doc = "PCIE_RC_POWER_MANAGEMENT_CAPABILITIES (r) register accessor: Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_power_management_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_power_management_capabilities`]
module"]
#[doc(alias = "PCIE_RC_POWER_MANAGEMENT_CAPABILITIES")]
pub type PcieRcPowerManagementCapabilities =
    crate::Reg<pcie_rc_power_management_capabilities::PcieRcPowerManagementCapabilitiesSpec>;
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_power_management_capabilities;
#[doc = "PCIE_RC_POWER_MANAGEMENT_CONTROL_STATUS_REPORT (rw) register accessor: Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_power_management_control_status_report::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_power_management_control_status_report::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_power_management_control_status_report`]
module"]
#[doc(alias = "PCIE_RC_POWER_MANAGEMENT_CONTROL_STATUS_REPORT")]
pub type PcieRcPowerManagementControlStatusReport = crate::Reg<
    pcie_rc_power_management_control_status_report::PcieRcPowerManagementControlStatusReportSpec,
>;
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub mod pcie_rc_power_management_control_status_report;
#[doc = "PCIE_RC_MSI_CONTROL (rw) register accessor: MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_control`]
module"]
#[doc(alias = "PCIE_RC_MSI_CONTROL")]
pub type PcieRcMsiControl = crate::Reg<pcie_rc_msi_control::PcieRcMsiControlSpec>;
#[doc = "MSI Control Register Reserved"]
pub mod pcie_rc_msi_control;
#[doc = "PCIE_RC_MSI_MESSAGE_LOW_ADDRESS (rw) register accessor: MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_message_low_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_message_low_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_message_low_address`]
module"]
#[doc(alias = "PCIE_RC_MSI_MESSAGE_LOW_ADDRESS")]
pub type PcieRcMsiMessageLowAddress =
    crate::Reg<pcie_rc_msi_message_low_address::PcieRcMsiMessageLowAddressSpec>;
#[doc = "MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub mod pcie_rc_msi_message_low_address;
#[doc = "PCIE_RC_MSI_MESSAGE_HIGH_ADDRESS (rw) register accessor: MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_message_high_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_message_high_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_message_high_address`]
module"]
#[doc(alias = "PCIE_RC_MSI_MESSAGE_HIGH_ADDRESS")]
pub type PcieRcMsiMessageHighAddress =
    crate::Reg<pcie_rc_msi_message_high_address::PcieRcMsiMessageHighAddressSpec>;
#[doc = "MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub mod pcie_rc_msi_message_high_address;
#[doc = "PCIE_RC_MSI_MESSAGE_DATA (rw) register accessor: MSI Message Data Register Hardwired to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_message_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_message_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_message_data`]
module"]
#[doc(alias = "PCIE_RC_MSI_MESSAGE_DATA")]
pub type PcieRcMsiMessageData = crate::Reg<pcie_rc_msi_message_data::PcieRcMsiMessageDataSpec>;
#[doc = "MSI Message Data Register Hardwired to 0"]
pub mod pcie_rc_msi_message_data;
#[doc = "PCIE_RC_MSI_MASK (rw) register accessor: MSI Mask Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_mask`]
module"]
#[doc(alias = "PCIE_RC_MSI_MASK")]
pub type PcieRcMsiMask = crate::Reg<pcie_rc_msi_mask::PcieRcMsiMaskSpec>;
#[doc = "MSI Mask Register RSVD"]
pub mod pcie_rc_msi_mask;
#[doc = "PCIE_RC_MSI_PENDING_BITS (r) register accessor: MSI Pending Bits Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_pending_bits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_pending_bits`]
module"]
#[doc(alias = "PCIE_RC_MSI_PENDING_BITS")]
pub type PcieRcMsiPendingBits = crate::Reg<pcie_rc_msi_pending_bits::PcieRcMsiPendingBitsSpec>;
#[doc = "MSI Pending Bits Register RSVD"]
pub mod pcie_rc_msi_pending_bits;
#[doc = "PCIE_RC_MSI_X_CONTROL (rw) register accessor: MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_x_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_x_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_x_control`]
module"]
#[doc(alias = "PCIE_RC_MSI_X_CONTROL")]
pub type PcieRcMsiXControl = crate::Reg<pcie_rc_msi_x_control::PcieRcMsiXControlSpec>;
#[doc = "MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub mod pcie_rc_msi_x_control;
#[doc = "PCIE_RC_MSI_X_TABLE_OFFSET (r) register accessor: MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_x_table_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_x_table_offset`]
module"]
#[doc(alias = "PCIE_RC_MSI_X_TABLE_OFFSET")]
pub type PcieRcMsiXTableOffset = crate::Reg<pcie_rc_msi_x_table_offset::PcieRcMsiXTableOffsetSpec>;
#[doc = "MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_rc_msi_x_table_offset;
#[doc = "PCIE_RC_MSI_X_PENDING_INTERRUPT (r) register accessor: MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_x_pending_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_msi_x_pending_interrupt`]
module"]
#[doc(alias = "PCIE_RC_MSI_X_PENDING_INTERRUPT")]
pub type PcieRcMsiXPendingInterrupt =
    crate::Reg<pcie_rc_msi_x_pending_interrupt::PcieRcMsiXPendingInterruptSpec>;
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod pcie_rc_msi_x_pending_interrupt;
#[doc = "PCIE_RC_PCI_EXPRESS_CAPABILITY_LIST (r) register accessor: PCI Express Capability List Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_capability_list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_pci_express_capability_list`]
module"]
#[doc(alias = "PCIE_RC_PCI_EXPRESS_CAPABILITY_LIST")]
pub type PcieRcPciExpressCapabilityList =
    crate::Reg<pcie_rc_pci_express_capability_list::PcieRcPciExpressCapabilityListSpec>;
#[doc = "PCI Express Capability List Register Reserved"]
pub mod pcie_rc_pci_express_capability_list;
#[doc = "PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES (r) register accessor: PCI Express Device Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_pci_express_device_capabilities`]
module"]
#[doc(alias = "PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES")]
pub type PcieRcPciExpressDeviceCapabilities =
    crate::Reg<pcie_rc_pci_express_device_capabilities::PcieRcPciExpressDeviceCapabilitiesSpec>;
#[doc = "PCI Express Device Capabilities Register Reserved"]
pub mod pcie_rc_pci_express_device_capabilities;
#[doc = "PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS (rw) register accessor: PCI Express Device Control and Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_pci_express_device_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_pci_express_device_control_and_status`]
module"]
#[doc(alias = "PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS")]
pub type PcieRcPciExpressDeviceControlAndStatus = crate::Reg<
    pcie_rc_pci_express_device_control_and_status::PcieRcPciExpressDeviceControlAndStatusSpec,
>;
#[doc = "PCI Express Device Control and Status Register (no description)"]
pub mod pcie_rc_pci_express_device_control_and_status;
#[doc = "PCIE_RC_LINK_CAPABILITIES (r) register accessor: Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_link_capabilities`]
module"]
#[doc(alias = "PCIE_RC_LINK_CAPABILITIES")]
pub type PcieRcLinkCapabilities = crate::Reg<pcie_rc_link_capabilities::PcieRcLinkCapabilitiesSpec>;
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod pcie_rc_link_capabilities;
#[doc = "PCIE_RC_LINK_CONTROL_AND_STATUS (rw) register accessor: Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_link_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_link_control_and_status`]
module"]
#[doc(alias = "PCIE_RC_LINK_CONTROL_AND_STATUS")]
pub type PcieRcLinkControlAndStatus =
    crate::Reg<pcie_rc_link_control_and_status::PcieRcLinkControlAndStatusSpec>;
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub mod pcie_rc_link_control_and_status;
#[doc = "PCIE_RC_SLOT_CAPABILITY (rw) register accessor: Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_capability::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_capability::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_slot_capability`]
module"]
#[doc(alias = "PCIE_RC_SLOT_CAPABILITY")]
pub type PcieRcSlotCapability = crate::Reg<pcie_rc_slot_capability::PcieRcSlotCapabilitySpec>;
#[doc = "Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
pub mod pcie_rc_slot_capability;
#[doc = "PCIE_RC_SLOT_CONTROL_AND_STATUS (rw) register accessor: Slot Control and Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_slot_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_slot_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_slot_control_and_status`]
module"]
#[doc(alias = "PCIE_RC_SLOT_CONTROL_AND_STATUS")]
pub type PcieRcSlotControlAndStatus =
    crate::Reg<pcie_rc_slot_control_and_status::PcieRcSlotControlAndStatusSpec>;
#[doc = "Slot Control and Status Register (no description)"]
pub mod pcie_rc_slot_control_and_status;
#[doc = "PCIE_RC_ROOT_CONTROL_AND_CAPABILITY (rw) register accessor: Root Control and Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_control_and_capability::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_control_and_capability::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_control_and_capability`]
module"]
#[doc(alias = "PCIE_RC_ROOT_CONTROL_AND_CAPABILITY")]
pub type PcieRcRootControlAndCapability =
    crate::Reg<pcie_rc_root_control_and_capability::PcieRcRootControlAndCapabilitySpec>;
#[doc = "Root Control and Capability Register Reserved"]
pub mod pcie_rc_root_control_and_capability;
#[doc = "PCIE_RC_ROOT_STATUS (rw) register accessor: Root Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_status`]
module"]
#[doc(alias = "PCIE_RC_ROOT_STATUS")]
pub type PcieRcRootStatus = crate::Reg<pcie_rc_root_status::PcieRcRootStatusSpec>;
#[doc = "Root Status Register Reserved"]
pub mod pcie_rc_root_status;
#[doc = "PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities 2 Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCIE_RC_PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PcieRcPciExpressDeviceCapabilities2 =
    crate::Reg<pcie_rc_pci_express_device_capabilities_2::PcieRcPciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities 2 Register Reserved"]
pub mod pcie_rc_pci_express_device_capabilities_2;
#[doc = "PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 (rw) register accessor: PCI Express Device Control and Status 2 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_device_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_pci_express_device_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_pci_express_device_control_and_status_2`]
module"]
#[doc(alias = "PCIE_RC_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2")]
pub type PcieRcPciExpressDeviceControlAndStatus2 = crate::Reg<
    pcie_rc_pci_express_device_control_and_status_2::PcieRcPciExpressDeviceControlAndStatus2Spec,
>;
#[doc = "PCI Express Device Control and Status 2 Register (no description)"]
pub mod pcie_rc_pci_express_device_control_and_status_2;
#[doc = "PCIE_RC_LINK_CAPABILITIES_2 (r) register accessor: Link Capabilities Register 2 RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_link_capabilities_2`]
module"]
#[doc(alias = "PCIE_RC_LINK_CAPABILITIES_2")]
pub type PcieRcLinkCapabilities2 =
    crate::Reg<pcie_rc_link_capabilities_2::PcieRcLinkCapabilities2Spec>;
#[doc = "Link Capabilities Register 2 RSVD"]
pub mod pcie_rc_link_capabilities_2;
#[doc = "PCIE_RC_LINK_CONTROL_AND_STATUS_2 (rw) register accessor: Link Control and Status 2 Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_link_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_link_control_and_status_2`]
module"]
#[doc(alias = "PCIE_RC_LINK_CONTROL_AND_STATUS_2")]
pub type PcieRcLinkControlAndStatus2 =
    crate::Reg<pcie_rc_link_control_and_status_2::PcieRcLinkControlAndStatus2Spec>;
#[doc = "Link Control and Status 2 Register Reserved"]
pub mod pcie_rc_link_control_and_status_2;
#[doc = "PCIE_RC_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER (r) register accessor: Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_advanced_error_reporting_aer_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_advanced_error_reporting_aer_enhanced_capability_header`]
module"]
#[doc(alias = "PCIE_RC_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER")]
pub type PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeader = crate :: Reg < pcie_rc_advanced_error_reporting_aer_enhanced_capability_header :: PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec > ;
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_rc_advanced_error_reporting_aer_enhanced_capability_header;
#[doc = "PCIE_RC_UNCORRECTABLE_ERROR_STATUS (rw) register accessor: Uncorrectable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_uncorrectable_error_status`]
module"]
#[doc(alias = "PCIE_RC_UNCORRECTABLE_ERROR_STATUS")]
pub type PcieRcUncorrectableErrorStatus =
    crate::Reg<pcie_rc_uncorrectable_error_status::PcieRcUncorrectableErrorStatusSpec>;
#[doc = "Uncorrectable Error Status Register Reserved"]
pub mod pcie_rc_uncorrectable_error_status;
#[doc = "PCIE_RC_UNCORRECTABLE_ERROR_MASK (rw) register accessor: Uncorrectable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_uncorrectable_error_mask`]
module"]
#[doc(alias = "PCIE_RC_UNCORRECTABLE_ERROR_MASK")]
pub type PcieRcUncorrectableErrorMask =
    crate::Reg<pcie_rc_uncorrectable_error_mask::PcieRcUncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register Reserved"]
pub mod pcie_rc_uncorrectable_error_mask;
#[doc = "PCIE_RC_UNCORRECTABLE_ERROR_SEVERITY (rw) register accessor: Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_severity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_severity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_uncorrectable_error_severity`]
module"]
#[doc(alias = "PCIE_RC_UNCORRECTABLE_ERROR_SEVERITY")]
pub type PcieRcUncorrectableErrorSeverity =
    crate::Reg<pcie_rc_uncorrectable_error_severity::PcieRcUncorrectableErrorSeveritySpec>;
#[doc = "Uncorrectable Error Severity Register (no description)"]
pub mod pcie_rc_uncorrectable_error_severity;
#[doc = "PCIE_RC_CORRECTABLE_ERROR_STATUS (rw) register accessor: Correctable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_correctable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_correctable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_correctable_error_status`]
module"]
#[doc(alias = "PCIE_RC_CORRECTABLE_ERROR_STATUS")]
pub type PcieRcCorrectableErrorStatus =
    crate::Reg<pcie_rc_correctable_error_status::PcieRcCorrectableErrorStatusSpec>;
#[doc = "Correctable Error Status Register Reserved"]
pub mod pcie_rc_correctable_error_status;
#[doc = "PCIE_RC_CORRECTABLE_ERROR_MASK (rw) register accessor: Correctable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_correctable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_correctable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_correctable_error_mask`]
module"]
#[doc(alias = "PCIE_RC_CORRECTABLE_ERROR_MASK")]
pub type PcieRcCorrectableErrorMask =
    crate::Reg<pcie_rc_correctable_error_mask::PcieRcCorrectableErrorMaskSpec>;
#[doc = "Correctable Error Mask Register Reserved"]
pub mod pcie_rc_correctable_error_mask;
#[doc = "PCIE_RC_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL (rw) register accessor: Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_advanced_error_capabilities_and_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_advanced_error_capabilities_and_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_advanced_error_capabilities_and_control`]
module"]
#[doc(alias = "PCIE_RC_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL")]
pub type PcieRcAdvancedErrorCapabilitiesAndControl = crate::Reg<
    pcie_rc_advanced_error_capabilities_and_control::PcieRcAdvancedErrorCapabilitiesAndControlSpec,
>;
#[doc = "Advanced Error Capabilities and Control Register Reserved"]
pub mod pcie_rc_advanced_error_capabilities_and_control;
#[doc = "PCIE_RC_HEADER_LOG_0 (r) register accessor: Header Log Register 0 First Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_header_log_0`]
module"]
#[doc(alias = "PCIE_RC_HEADER_LOG_0")]
pub type PcieRcHeaderLog0 = crate::Reg<pcie_rc_header_log_0::PcieRcHeaderLog0Spec>;
#[doc = "Header Log Register 0 First Dword of captured TLP header. STICKY."]
pub mod pcie_rc_header_log_0;
#[doc = "PCIE_RC_HEADER_LOG_1 (r) register accessor: Header Log Register 1 Second Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_header_log_1`]
module"]
#[doc(alias = "PCIE_RC_HEADER_LOG_1")]
pub type PcieRcHeaderLog1 = crate::Reg<pcie_rc_header_log_1::PcieRcHeaderLog1Spec>;
#[doc = "Header Log Register 1 Second Dword of captured TLP header. STICKY."]
pub mod pcie_rc_header_log_1;
#[doc = "PCIE_RC_HEADER_LOG_2 (r) register accessor: Header Log Register 2 Third Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_header_log_2`]
module"]
#[doc(alias = "PCIE_RC_HEADER_LOG_2")]
pub type PcieRcHeaderLog2 = crate::Reg<pcie_rc_header_log_2::PcieRcHeaderLog2Spec>;
#[doc = "Header Log Register 2 Third Dword of captured TLP header. STICKY."]
pub mod pcie_rc_header_log_2;
#[doc = "PCIE_RC_HEADER_LOG_3 (r) register accessor: Header Log Register 3 Fourth Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_header_log_3`]
module"]
#[doc(alias = "PCIE_RC_HEADER_LOG_3")]
pub type PcieRcHeaderLog3 = crate::Reg<pcie_rc_header_log_3::PcieRcHeaderLog3Spec>;
#[doc = "Header Log Register 3 Fourth Dword of captured TLP header. STICKY."]
pub mod pcie_rc_header_log_3;
#[doc = "PCIE_RC_ROOT_ERROR_COMMAND (rw) register accessor: Root Error Command Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_error_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_error_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_error_command`]
module"]
#[doc(alias = "PCIE_RC_ROOT_ERROR_COMMAND")]
pub type PcieRcRootErrorCommand =
    crate::Reg<pcie_rc_root_error_command::PcieRcRootErrorCommandSpec>;
#[doc = "Root Error Command Register Reserved"]
pub mod pcie_rc_root_error_command;
#[doc = "PCIE_RC_ROOT_ERROR_STATUS (rw) register accessor: Root Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_root_error_status`]
module"]
#[doc(alias = "PCIE_RC_ROOT_ERROR_STATUS")]
pub type PcieRcRootErrorStatus = crate::Reg<pcie_rc_root_error_status::PcieRcRootErrorStatusSpec>;
#[doc = "Root Error Status Register Reserved"]
pub mod pcie_rc_root_error_status;
#[doc = "PCIE_RC_ERROR_SOURCE_IDENTIFICATION (r) register accessor: Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_error_source_identification::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_error_source_identification`]
module"]
#[doc(alias = "PCIE_RC_ERROR_SOURCE_IDENTIFICATION")]
pub type PcieRcErrorSourceIdentification =
    crate::Reg<pcie_rc_error_source_identification::PcieRcErrorSourceIdentificationSpec>;
#[doc = "Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
pub mod pcie_rc_error_source_identification;
#[doc = "PCIE_RC_L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER (r) register accessor: L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_l1_pm_substates_extended_capability_header`]
module"]
#[doc(alias = "PCIE_RC_L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER")]
pub type PcieRcL1PmSubstatesExtendedCapabilityHeader = crate :: Reg < pcie_rc_l1_pm_substates_extended_capability_header :: PcieRcL1PmSubstatesExtendedCapabilityHeaderSpec > ;
#[doc = "L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod pcie_rc_l1_pm_substates_extended_capability_header;
#[doc = "PCIE_RC_L1_PM_SUBSTATES_CAPABILITIES (r) register accessor: L1 PM Substates Capabilities Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_l1_pm_substates_capabilities`]
module"]
#[doc(alias = "PCIE_RC_L1_PM_SUBSTATES_CAPABILITIES")]
pub type PcieRcL1PmSubstatesCapabilities =
    crate::Reg<pcie_rc_l1_pm_substates_capabilities::PcieRcL1PmSubstatesCapabilitiesSpec>;
#[doc = "L1 PM Substates Capabilities Register RSVD"]
pub mod pcie_rc_l1_pm_substates_capabilities;
#[doc = "PCIE_RC_L1_PM_SUBSTATES_CONTROL_1 (rw) register accessor: L1 PM Substates Control 1 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_control_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_l1_pm_substates_control_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_l1_pm_substates_control_1`]
module"]
#[doc(alias = "PCIE_RC_L1_PM_SUBSTATES_CONTROL_1")]
pub type PcieRcL1PmSubstatesControl1 =
    crate::Reg<pcie_rc_l1_pm_substates_control_1::PcieRcL1PmSubstatesControl1Spec>;
#[doc = "L1 PM Substates Control 1 Register (no description)"]
pub mod pcie_rc_l1_pm_substates_control_1;
#[doc = "PCIE_RC_L1_PM_SUBSTATES_CONTROL_2 (rw) register accessor: L1 PM Substates Control 2 Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_control_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_l1_pm_substates_control_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_rc_l1_pm_substates_control_2`]
module"]
#[doc(alias = "PCIE_RC_L1_PM_SUBSTATES_CONTROL_2")]
pub type PcieRcL1PmSubstatesControl2 =
    crate::Reg<pcie_rc_l1_pm_substates_control_2::PcieRcL1PmSubstatesControl2Spec>;
#[doc = "L1 PM Substates Control 2 Register RSVD"]
pub mod pcie_rc_l1_pm_substates_control_2;
#[doc = "PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_0 (rw) register accessor: Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_layer_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_layer_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_physical_layer_configuration_0`]
module"]
#[doc(alias = "PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_0")]
pub type PcieLmPhysicalLayerConfiguration0 =
    crate::Reg<pcie_lm_physical_layer_configuration_0::PcieLmPhysicalLayerConfiguration0Spec>;
#[doc = "Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode."]
pub mod pcie_lm_physical_layer_configuration_0;
#[doc = "PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_1 (rw) register accessor: Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_layer_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_layer_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_physical_layer_configuration_1`]
module"]
#[doc(alias = "PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_1")]
pub type PcieLmPhysicalLayerConfiguration1 =
    crate::Reg<pcie_lm_physical_layer_configuration_1::PcieLmPhysicalLayerConfiguration1Spec>;
#[doc = "Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub mod pcie_lm_physical_layer_configuration_1;
#[doc = "PCIE_LM_DATA_LINK_LAYER_TIMER_CONFIGURATION (rw) register accessor: Data Link Layer Timer Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_data_link_layer_timer_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_data_link_layer_timer_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_data_link_layer_timer_configuration`]
module"]
#[doc(alias = "PCIE_LM_DATA_LINK_LAYER_TIMER_CONFIGURATION")]
pub type PcieLmDataLinkLayerTimerConfiguration = crate::Reg<
    pcie_lm_data_link_layer_timer_configuration::PcieLmDataLinkLayerTimerConfigurationSpec,
>;
#[doc = "Data Link Layer Timer Configuration Register Reserved"]
pub mod pcie_lm_data_link_layer_timer_configuration;
#[doc = "PCIE_LM_RECEIVE_CREDIT_LIMIT_0_VC0 (rw) register accessor: Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_credit_limit_0_vc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_credit_limit_0_vc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_receive_credit_limit_0_vc0`]
module"]
#[doc(alias = "PCIE_LM_RECEIVE_CREDIT_LIMIT_0_VC0")]
pub type PcieLmReceiveCreditLimit0Vc0 =
    crate::Reg<pcie_lm_receive_credit_limit_0_vc0::PcieLmReceiveCreditLimit0Vc0Spec>;
#[doc = "Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
pub mod pcie_lm_receive_credit_limit_0_vc0;
#[doc = "PCIE_LM_RECEIVE_CREDIT_LIMIT_1_VC0 (rw) register accessor: Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_credit_limit_1_vc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_credit_limit_1_vc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_receive_credit_limit_1_vc0`]
module"]
#[doc(alias = "PCIE_LM_RECEIVE_CREDIT_LIMIT_1_VC0")]
pub type PcieLmReceiveCreditLimit1Vc0 =
    crate::Reg<pcie_lm_receive_credit_limit_1_vc0::PcieLmReceiveCreditLimit1Vc0Spec>;
#[doc = "Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
pub mod pcie_lm_receive_credit_limit_1_vc0;
#[doc = "PCIE_LM_TRANSMIT_CREDIT_LIMIT_0_VC0 (r) register accessor: Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_limit_0_vc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_credit_limit_0_vc0`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_CREDIT_LIMIT_0_VC0")]
pub type PcieLmTransmitCreditLimit0Vc0 =
    crate::Reg<pcie_lm_transmit_credit_limit_0_vc0::PcieLmTransmitCreditLimit0Vc0Spec>;
#[doc = "Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords)."]
pub mod pcie_lm_transmit_credit_limit_0_vc0;
#[doc = "PCIE_LM_TRANSMIT_CREDIT_LIMIT_1_VC0 (r) register accessor: Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_limit_1_vc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_credit_limit_1_vc0`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_CREDIT_LIMIT_1_VC0")]
pub type PcieLmTransmitCreditLimit1Vc0 =
    crate::Reg<pcie_lm_transmit_credit_limit_1_vc0::PcieLmTransmitCreditLimit1Vc0Spec>;
#[doc = "Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets)."]
pub mod pcie_lm_transmit_credit_limit_1_vc0;
#[doc = "PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0 (rw) register accessor: Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_update_interval_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_credit_update_interval_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_credit_update_interval_configuration_0`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0")]
pub type PcieLmTransmitCreditUpdateIntervalConfiguration0 = crate :: Reg < pcie_lm_transmit_credit_update_interval_configuration_0 :: PcieLmTransmitCreditUpdateIntervalConfiguration0Spec > ;
#[doc = "Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub mod pcie_lm_transmit_credit_update_interval_configuration_0;
#[doc = "PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1 (rw) register accessor: Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_update_interval_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_credit_update_interval_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_credit_update_interval_configuration_1`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1")]
pub type PcieLmTransmitCreditUpdateIntervalConfiguration1 = crate :: Reg < pcie_lm_transmit_credit_update_interval_configuration_1 :: PcieLmTransmitCreditUpdateIntervalConfiguration1Spec > ;
#[doc = "Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub mod pcie_lm_transmit_credit_update_interval_configuration_1;
#[doc = "PCIE_LM_L0S_TIMEOUT_LIMIT (rw) register accessor: L0S Timeout Limit Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_l0s_timeout_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_l0s_timeout_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_l0s_timeout_limit`]
module"]
#[doc(alias = "PCIE_LM_L0S_TIMEOUT_LIMIT")]
pub type PcieLmL0sTimeoutLimit = crate::Reg<pcie_lm_l0s_timeout_limit::PcieLmL0sTimeoutLimitSpec>;
#[doc = "L0S Timeout Limit Register Reserved"]
pub mod pcie_lm_l0s_timeout_limit;
#[doc = "PCIE_LM_TRANSMIT_TLP_COUNT (rw) register accessor: Transmit TLP Count Register Count of TLPs transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_tlp_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_tlp_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_tlp_count`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_TLP_COUNT")]
pub type PcieLmTransmitTlpCount =
    crate::Reg<pcie_lm_transmit_tlp_count::PcieLmTransmitTlpCountSpec>;
#[doc = "Transmit TLP Count Register Count of TLPs transmitted"]
pub mod pcie_lm_transmit_tlp_count;
#[doc = "PCIE_LM_TRANSMIT_TLP_PAYLOAD_DWORD_COUNT (rw) register accessor: Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_tlp_payload_dword_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_tlp_payload_dword_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_transmit_tlp_payload_dword_count`]
module"]
#[doc(alias = "PCIE_LM_TRANSMIT_TLP_PAYLOAD_DWORD_COUNT")]
pub type PcieLmTransmitTlpPayloadDwordCount =
    crate::Reg<pcie_lm_transmit_tlp_payload_dword_count::PcieLmTransmitTlpPayloadDwordCountSpec>;
#[doc = "Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted"]
pub mod pcie_lm_transmit_tlp_payload_dword_count;
#[doc = "PCIE_LM_RECEIVE_TLP_COUNT (rw) register accessor: Receive TLP Count Register Count of TLPs received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_tlp_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_tlp_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_receive_tlp_count`]
module"]
#[doc(alias = "PCIE_LM_RECEIVE_TLP_COUNT")]
pub type PcieLmReceiveTlpCount = crate::Reg<pcie_lm_receive_tlp_count::PcieLmReceiveTlpCountSpec>;
#[doc = "Receive TLP Count Register Count of TLPs received"]
pub mod pcie_lm_receive_tlp_count;
#[doc = "PCIE_LM_RECEIVE_TLP_PAYLOAD_DWORD_COUNT (rw) register accessor: Receive TLP Payload Dword Count Register Count of TLP payload Dwords received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_tlp_payload_dword_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_tlp_payload_dword_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_receive_tlp_payload_dword_count`]
module"]
#[doc(alias = "PCIE_LM_RECEIVE_TLP_PAYLOAD_DWORD_COUNT")]
pub type PcieLmReceiveTlpPayloadDwordCount =
    crate::Reg<pcie_lm_receive_tlp_payload_dword_count::PcieLmReceiveTlpPayloadDwordCountSpec>;
#[doc = "Receive TLP Payload Dword Count Register Count of TLP payload Dwords received"]
pub mod pcie_lm_receive_tlp_payload_dword_count;
#[doc = "PCIE_LM_COMPLETION_TIMEOUT_LIMIT_0 (rw) register accessor: Completion Timeout Limit Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_completion_timeout_limit_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_completion_timeout_limit_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_completion_timeout_limit_0`]
module"]
#[doc(alias = "PCIE_LM_COMPLETION_TIMEOUT_LIMIT_0")]
pub type PcieLmCompletionTimeoutLimit0 =
    crate::Reg<pcie_lm_completion_timeout_limit_0::PcieLmCompletionTimeoutLimit0Spec>;
#[doc = "Completion Timeout Limit Register 0 Reserved"]
pub mod pcie_lm_completion_timeout_limit_0;
#[doc = "PCIE_LM_COMPLETION_TIMEOUT_LIMIT_1 (rw) register accessor: Completion Timeout Limit Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_completion_timeout_limit_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_completion_timeout_limit_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_completion_timeout_limit_1`]
module"]
#[doc(alias = "PCIE_LM_COMPLETION_TIMEOUT_LIMIT_1")]
pub type PcieLmCompletionTimeoutLimit1 =
    crate::Reg<pcie_lm_completion_timeout_limit_1::PcieLmCompletionTimeoutLimit1Spec>;
#[doc = "Completion Timeout Limit Register 1 Reserved"]
pub mod pcie_lm_completion_timeout_limit_1;
#[doc = "PCIE_LM_L1_STATE_RE_ENTRY_DELAY (rw) register accessor: L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_l1_state_re_entry_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_l1_state_re_entry_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_l1_state_re_entry_delay`]
module"]
#[doc(alias = "PCIE_LM_L1_STATE_RE_ENTRY_DELAY")]
pub type PcieLmL1StateReEntryDelay =
    crate::Reg<pcie_lm_l1_state_re_entry_delay::PcieLmL1StateReEntryDelaySpec>;
#[doc = "L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns)."]
pub mod pcie_lm_l1_state_re_entry_delay;
#[doc = "PCIE_LM_VENDOR_ID (rw) register accessor: Vendor ID Register Subsystem Vendor ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_vendor_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_vendor_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_vendor_id`]
module"]
#[doc(alias = "PCIE_LM_VENDOR_ID")]
pub type PcieLmVendorId = crate::Reg<pcie_lm_vendor_id::PcieLmVendorIdSpec>;
#[doc = "Vendor ID Register Subsystem Vendor ID"]
pub mod pcie_lm_vendor_id;
#[doc = "PCIE_LM_ASPM_L1_ENTRY_TIMEOUT_DELAY (rw) register accessor: ASPM L1 Entry Timeout Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_aspm_l1_entry_timeout_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_aspm_l1_entry_timeout_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_aspm_l1_entry_timeout_delay`]
module"]
#[doc(alias = "PCIE_LM_ASPM_L1_ENTRY_TIMEOUT_DELAY")]
pub type PcieLmAspmL1EntryTimeoutDelay =
    crate::Reg<pcie_lm_aspm_l1_entry_timeout_delay::PcieLmAspmL1EntryTimeoutDelaySpec>;
#[doc = "ASPM L1 Entry Timeout Delay Register Reserved"]
pub mod pcie_lm_aspm_l1_entry_timeout_delay;
#[doc = "PCIE_LM_PME_TURNOFF_ACK_DELAY (rw) register accessor: PME TurnOff Ack Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_pme_turnoff_ack_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_pme_turnoff_ack_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_pme_turnoff_ack_delay`]
module"]
#[doc(alias = "PCIE_LM_PME_TURNOFF_ACK_DELAY")]
pub type PcieLmPmeTurnoffAckDelay =
    crate::Reg<pcie_lm_pme_turnoff_ack_delay::PcieLmPmeTurnoffAckDelaySpec>;
#[doc = "PME TurnOff Ack Delay Register Reserved"]
pub mod pcie_lm_pme_turnoff_ack_delay;
#[doc = "PCIE_LM_LINKWIDTH_CONTROL (rw) register accessor: Linkwidth Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_linkwidth_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_linkwidth_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_linkwidth_control`]
module"]
#[doc(alias = "PCIE_LM_LINKWIDTH_CONTROL")]
pub type PcieLmLinkwidthControl = crate::Reg<pcie_lm_linkwidth_control::PcieLmLinkwidthControlSpec>;
#[doc = "Linkwidth Control Register Reserved"]
pub mod pcie_lm_linkwidth_control;
#[doc = "PCIE_LM_SRIS_CONTROL (rw) register accessor: SRIS Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_sris_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_sris_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_sris_control`]
module"]
#[doc(alias = "PCIE_LM_SRIS_CONTROL")]
pub type PcieLmSrisControl = crate::Reg<pcie_lm_sris_control::PcieLmSrisControlSpec>;
#[doc = "SRIS Control Register Reserved"]
pub mod pcie_lm_sris_control;
#[doc = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_0 (rw) register accessor: Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_register_header_log_0`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_0")]
pub type PcieLmShadowRegisterHeaderLog0 =
    crate::Reg<pcie_lm_shadow_register_header_log_0::PcieLmShadowRegisterHeaderLog0Spec>;
#[doc = "Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
pub mod pcie_lm_shadow_register_header_log_0;
#[doc = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_1 (rw) register accessor: Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_register_header_log_1`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_1")]
pub type PcieLmShadowRegisterHeaderLog1 =
    crate::Reg<pcie_lm_shadow_register_header_log_1::PcieLmShadowRegisterHeaderLog1Spec>;
#[doc = "Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header."]
pub mod pcie_lm_shadow_register_header_log_1;
#[doc = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_2 (rw) register accessor: Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_register_header_log_2`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_2")]
pub type PcieLmShadowRegisterHeaderLog2 =
    crate::Reg<pcie_lm_shadow_register_header_log_2::PcieLmShadowRegisterHeaderLog2Spec>;
#[doc = "Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
pub mod pcie_lm_shadow_register_header_log_2;
#[doc = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_3 (rw) register accessor: Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_register_header_log_3`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_REGISTER_HEADER_LOG_3")]
pub type PcieLmShadowRegisterHeaderLog3 =
    crate::Reg<pcie_lm_shadow_register_header_log_3::PcieLmShadowRegisterHeaderLog3Spec>;
#[doc = "Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
pub mod pcie_lm_shadow_register_header_log_3;
#[doc = "PCIE_LM_SHADOW_REGISTER_FUNCTION_NUMBER (rw) register accessor: Shadow register function number. Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_function_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_function_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_register_function_number`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_REGISTER_FUNCTION_NUMBER")]
pub type PcieLmShadowRegisterFunctionNumber =
    crate::Reg<pcie_lm_shadow_register_function_number::PcieLmShadowRegisterFunctionNumberSpec>;
#[doc = "Shadow register function number. Reserved"]
pub mod pcie_lm_shadow_register_function_number;
#[doc = "PCIE_LM_SHADOW_UR_ERROR (rw) register accessor: Shadow Register UR Error Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_ur_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_ur_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_shadow_ur_error`]
module"]
#[doc(alias = "PCIE_LM_SHADOW_UR_ERROR")]
pub type PcieLmShadowUrError = crate::Reg<pcie_lm_shadow_ur_error::PcieLmShadowUrErrorSpec>;
#[doc = "Shadow Register UR Error Reserved"]
pub mod pcie_lm_shadow_ur_error;
#[doc = "PCIE_LM_NEGOTIATED_LANE_MAP (r) register accessor: Negotiated Lane Map Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_negotiated_lane_map::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_negotiated_lane_map`]
module"]
#[doc(alias = "PCIE_LM_NEGOTIATED_LANE_MAP")]
pub type PcieLmNegotiatedLaneMap =
    crate::Reg<pcie_lm_negotiated_lane_map::PcieLmNegotiatedLaneMapSpec>;
#[doc = "Negotiated Lane Map Register Reserved"]
pub mod pcie_lm_negotiated_lane_map;
#[doc = "PCIE_LM_RECEIVE_FTS_COUNT (r) register accessor: Receive FTS Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_fts_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_receive_fts_count`]
module"]
#[doc(alias = "PCIE_LM_RECEIVE_FTS_COUNT")]
pub type PcieLmReceiveFtsCount = crate::Reg<pcie_lm_receive_fts_count::PcieLmReceiveFtsCountSpec>;
#[doc = "Receive FTS Count Register Reserved"]
pub mod pcie_lm_receive_fts_count;
#[doc = "PCIE_LM_DEBUG_MUX_CONTROL (rw) register accessor: Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_debug_mux_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_debug_mux_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_debug_mux_control`]
module"]
#[doc(alias = "PCIE_LM_DEBUG_MUX_CONTROL")]
pub type PcieLmDebugMuxControl = crate::Reg<pcie_lm_debug_mux_control::PcieLmDebugMuxControlSpec>;
#[doc = "Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit."]
pub mod pcie_lm_debug_mux_control;
#[doc = "PCIE_LM_LOCAL_ERROR_AND_STATUS (rw) register accessor: Local Error and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_local_error_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_local_error_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_local_error_and_status`]
module"]
#[doc(alias = "PCIE_LM_LOCAL_ERROR_AND_STATUS")]
pub type PcieLmLocalErrorAndStatus =
    crate::Reg<pcie_lm_local_error_and_status::PcieLmLocalErrorAndStatusSpec>;
#[doc = "Local Error and Status Register Reserved"]
pub mod pcie_lm_local_error_and_status;
#[doc = "PCIE_LM_LOCAL_INTERRUPT_MASK (rw) register accessor: Local Interrupt Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_local_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_local_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_local_interrupt_mask`]
module"]
#[doc(alias = "PCIE_LM_LOCAL_INTERRUPT_MASK")]
pub type PcieLmLocalInterruptMask =
    crate::Reg<pcie_lm_local_interrupt_mask::PcieLmLocalInterruptMaskSpec>;
#[doc = "Local Interrupt Mask Register Reserved"]
pub mod pcie_lm_local_interrupt_mask;
#[doc = "PCIE_LM_LCRC_ERROR_COUNT (rw) register accessor: LCRC Error Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_lcrc_error_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_lcrc_error_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_lcrc_error_count`]
module"]
#[doc(alias = "PCIE_LM_LCRC_ERROR_COUNT")]
pub type PcieLmLcrcErrorCount = crate::Reg<pcie_lm_lcrc_error_count::PcieLmLcrcErrorCountSpec>;
#[doc = "LCRC Error Count Register Reserved"]
pub mod pcie_lm_lcrc_error_count;
#[doc = "PCIE_LM_ECC_CORRECTABLE_ERROR_COUNT (rw) register accessor: ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ecc_correctable_error_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ecc_correctable_error_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_ecc_correctable_error_count`]
module"]
#[doc(alias = "PCIE_LM_ECC_CORRECTABLE_ERROR_COUNT")]
pub type PcieLmEccCorrectableErrorCount =
    crate::Reg<pcie_lm_ecc_correctable_error_count::PcieLmEccCorrectableErrorCountSpec>;
#[doc = "ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it."]
pub mod pcie_lm_ecc_correctable_error_count;
#[doc = "PCIE_LM_LTR_SNOOP_NO_SNOOP_LATENCY (rw) register accessor: LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ltr_snoop_no_snoop_latency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ltr_snoop_no_snoop_latency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_ltr_snoop_no_snoop_latency`]
module"]
#[doc(alias = "PCIE_LM_LTR_SNOOP_NO_SNOOP_LATENCY")]
pub type PcieLmLtrSnoopNoSnoopLatency =
    crate::Reg<pcie_lm_ltr_snoop_no_snoop_latency::PcieLmLtrSnoopNoSnoopLatencySpec>;
#[doc = "LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
pub mod pcie_lm_ltr_snoop_no_snoop_latency;
#[doc = "PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL (rw) register accessor: LTR Message Generation Control Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ltr_message_generation_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ltr_message_generation_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_ltr_message_generation_control`]
module"]
#[doc(alias = "PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL")]
pub type PcieLmLtrMessageGenerationControl =
    crate::Reg<pcie_lm_ltr_message_generation_control::PcieLmLtrMessageGenerationControlSpec>;
#[doc = "LTR Message Generation Control Register RSVD"]
pub mod pcie_lm_ltr_message_generation_control;
#[doc = "PCIE_LM_PME_SERVICE_TIMEOUT_DELAY (rw) register accessor: PME Service Timeout Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_pme_service_timeout_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_pme_service_timeout_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_pme_service_timeout_delay`]
module"]
#[doc(alias = "PCIE_LM_PME_SERVICE_TIMEOUT_DELAY")]
pub type PcieLmPmeServiceTimeoutDelay =
    crate::Reg<pcie_lm_pme_service_timeout_delay::PcieLmPmeServiceTimeoutDelaySpec>;
#[doc = "PME Service Timeout Delay Register Reserved"]
pub mod pcie_lm_pme_service_timeout_delay;
#[doc = "PCIE_LM_ROOT_PORT_REQUESTOR_ID (rw) register accessor: Root Port Requestor ID Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_root_port_requestor_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_root_port_requestor_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_root_port_requestor_id`]
module"]
#[doc(alias = "PCIE_LM_ROOT_PORT_REQUESTOR_ID")]
pub type PcieLmRootPortRequestorId =
    crate::Reg<pcie_lm_root_port_requestor_id::PcieLmRootPortRequestorIdSpec>;
#[doc = "Root Port Requestor ID Register Reserved"]
pub mod pcie_lm_root_port_requestor_id;
#[doc = "PCIE_LM_END_POINT_BUS_AND_DEVICE_NUMBER (r) register accessor: End Point Bus and Device Number Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_end_point_bus_and_device_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_end_point_bus_and_device_number`]
module"]
#[doc(alias = "PCIE_LM_END_POINT_BUS_AND_DEVICE_NUMBER")]
pub type PcieLmEndPointBusAndDeviceNumber =
    crate::Reg<pcie_lm_end_point_bus_and_device_number::PcieLmEndPointBusAndDeviceNumberSpec>;
#[doc = "End Point Bus and Device Number Register Reserved"]
pub mod pcie_lm_end_point_bus_and_device_number;
#[doc = "PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_0 (rw) register accessor: Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_function_bar_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_function_bar_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_physical_function_bar_configuration_0`]
module"]
#[doc(alias = "PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_0")]
pub type PcieLmPhysicalFunctionBarConfiguration0 = crate::Reg<
    pcie_lm_physical_function_bar_configuration_0::PcieLmPhysicalFunctionBarConfiguration0Spec,
>;
#[doc = "Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub mod pcie_lm_physical_function_bar_configuration_0;
#[doc = "PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_1 (rw) register accessor: Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_function_bar_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_function_bar_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_physical_function_bar_configuration_1`]
module"]
#[doc(alias = "PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_1")]
pub type PcieLmPhysicalFunctionBarConfiguration1 = crate::Reg<
    pcie_lm_physical_function_bar_configuration_1::PcieLmPhysicalFunctionBarConfiguration1Spec,
>;
#[doc = "Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure."]
pub mod pcie_lm_physical_function_bar_configuration_1;
#[doc = "PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_0 (rw) register accessor: Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_virtual_function_bar_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_virtual_function_bar_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_virtual_function_bar_configuration_0`]
module"]
#[doc(alias = "PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_0")]
pub type PcieLmVirtualFunctionBarConfiguration0 = crate::Reg<
    pcie_lm_virtual_function_bar_configuration_0::PcieLmVirtualFunctionBarConfiguration0Spec,
>;
#[doc = "Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub mod pcie_lm_virtual_function_bar_configuration_0;
#[doc = "PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_1 (rw) register accessor: Virtual Function BAR Configuration Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_virtual_function_bar_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_virtual_function_bar_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_virtual_function_bar_configuration_1`]
module"]
#[doc(alias = "PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_1")]
pub type PcieLmVirtualFunctionBarConfiguration1 = crate::Reg<
    pcie_lm_virtual_function_bar_configuration_1::PcieLmVirtualFunctionBarConfiguration1Spec,
>;
#[doc = "Virtual Function BAR Configuration Register 1 Reserved"]
pub mod pcie_lm_virtual_function_bar_configuration_1;
#[doc = "PCIE_LM_PHYSICAL_FUNCTION_CONFIGURATION (r) register accessor: Physical Function Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_function_configuration::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_physical_function_configuration`]
module"]
#[doc(alias = "PCIE_LM_PHYSICAL_FUNCTION_CONFIGURATION")]
pub type PcieLmPhysicalFunctionConfiguration =
    crate::Reg<pcie_lm_physical_function_configuration::PcieLmPhysicalFunctionConfigurationSpec>;
#[doc = "Physical Function Configuration Register Reserved"]
pub mod pcie_lm_physical_function_configuration;
#[doc = "PCIE_LM_ROOT_COMPLEX_BAR_CONFIGURATION (rw) register accessor: Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_root_complex_bar_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_root_complex_bar_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_lm_root_complex_bar_configuration`]
module"]
#[doc(alias = "PCIE_LM_ROOT_COMPLEX_BAR_CONFIGURATION")]
pub type PcieLmRootComplexBarConfiguration =
    crate::Reg<pcie_lm_root_complex_bar_configuration::PcieLmRootComplexBarConfigurationSpec>;
#[doc = "Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
pub mod pcie_lm_root_complex_bar_configuration;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_0 (rw) register accessor: Outbound Region Address 0 Lower 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_address_0`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_0")]
pub type PcieAtObOutboundRegionAddress0 =
    crate::Reg<pcie_at_ob_outbound_region_address_0::PcieAtObOutboundRegionAddress0Spec>;
#[doc = "Outbound Region Address 0 Lower 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_address_0;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_1 (rw) register accessor: Outbound Region Address 1 Upper 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_address_1`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_1")]
pub type PcieAtObOutboundRegionAddress1 =
    crate::Reg<pcie_at_ob_outbound_region_address_1::PcieAtObOutboundRegionAddress1Spec>;
#[doc = "Outbound Region Address 1 Upper 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_address_1;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_0 (rw) register accessor: Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_descriptor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_descriptor_0`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_0")]
pub type PcieAtObOutboundRegionDescriptor0 =
    crate::Reg<pcie_at_ob_outbound_region_descriptor_0::PcieAtObOutboundRegionDescriptor0Spec>;
#[doc = "Outbound Region Descriptor 0 Lowest 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_descriptor_0;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_1 (rw) register accessor: Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_descriptor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_descriptor_1`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_1")]
pub type PcieAtObOutboundRegionDescriptor1 =
    crate::Reg<pcie_at_ob_outbound_region_descriptor_1::PcieAtObOutboundRegionDescriptor1Spec>;
#[doc = "Outbound Region Descriptor 1 Lower middle 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_descriptor_1;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_2 (rw) register accessor: Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_descriptor_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_descriptor_2`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_2")]
pub type PcieAtObOutboundRegionDescriptor2 =
    crate::Reg<pcie_at_ob_outbound_region_descriptor_2::PcieAtObOutboundRegionDescriptor2Spec>;
#[doc = "Outbound Region Descriptor 2 Upper middle 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_descriptor_2;
#[doc = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_3 (r) register accessor: Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_descriptor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ob_outbound_region_descriptor_3`]
module"]
#[doc(alias = "PCIE_AT_OB_OUTBOUND_REGION_DESCRIPTOR_3")]
pub type PcieAtObOutboundRegionDescriptor3 =
    crate::Reg<pcie_at_ob_outbound_region_descriptor_3::PcieAtObOutboundRegionDescriptor3Spec>;
#[doc = "Outbound Region Descriptor 3 Upmost 32-bits of Address Register for region N"]
pub mod pcie_at_ob_outbound_region_descriptor_3;
#[doc = "PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_0 (rw) register accessor: RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_rp_ib_rp_inbound_bar_address_translation_0`]
module"]
#[doc(alias = "PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_0")]
pub type PcieAtRpIbRpInboundBarAddressTranslation0 = crate :: Reg < pcie_at_rp_ib_rp_inbound_bar_address_translation_0 :: PcieAtRpIbRpInboundBarAddressTranslation0Spec > ;
#[doc = "RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N"]
pub mod pcie_at_rp_ib_rp_inbound_bar_address_translation_0;
#[doc = "PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_1 (rw) register accessor: RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_rp_ib_rp_inbound_bar_address_translation_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_rp_ib_rp_inbound_bar_address_translation_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_rp_ib_rp_inbound_bar_address_translation_1`]
module"]
#[doc(alias = "PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_1")]
pub type PcieAtRpIbRpInboundBarAddressTranslation1 = crate :: Reg < pcie_at_rp_ib_rp_inbound_bar_address_translation_1 :: PcieAtRpIbRpInboundBarAddressTranslation1Spec > ;
#[doc = "RP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
pub mod pcie_at_rp_ib_rp_inbound_bar_address_translation_1;
#[doc = "PCIE_AT_RP_IB_LINK_DOWN_INDICATION_BIT (rw) register accessor: Link down indication bit RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_rp_ib_link_down_indication_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_rp_ib_link_down_indication_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_rp_ib_link_down_indication_bit`]
module"]
#[doc(alias = "PCIE_AT_RP_IB_LINK_DOWN_INDICATION_BIT")]
pub type PcieAtRpIbLinkDownIndicationBit =
    crate::Reg<pcie_at_rp_ib_link_down_indication_bit::PcieAtRpIbLinkDownIndicationBitSpec>;
#[doc = "Link down indication bit RSVD"]
pub mod pcie_at_rp_ib_link_down_indication_bit;
#[doc = "PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_0 (rw) register accessor: EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ep_ib_ep_inbound_bar_address_translation_0`]
module"]
#[doc(alias = "PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_0")]
pub type PcieAtEpIbEpInboundBarAddressTranslation0 = crate :: Reg < pcie_at_ep_ib_ep_inbound_bar_address_translation_0 :: PcieAtEpIbEpInboundBarAddressTranslation0Spec > ;
#[doc = "EP Inbound BAR Address Translation 0 Bits \\[31:0\\]
of Address Register for BAR N"]
pub mod pcie_at_ep_ib_ep_inbound_bar_address_translation_0;
#[doc = "PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_1 (rw) register accessor: EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ep_ib_ep_inbound_bar_address_translation_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ep_ib_ep_inbound_bar_address_translation_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_at_ep_ib_ep_inbound_bar_address_translation_1`]
module"]
#[doc(alias = "PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_1")]
pub type PcieAtEpIbEpInboundBarAddressTranslation1 = crate :: Reg < pcie_at_ep_ib_ep_inbound_bar_address_translation_1 :: PcieAtEpIbEpInboundBarAddressTranslation1Spec > ;
#[doc = "EP Inbound BAR Address Translation 1 Bits \\[63:32\\]
of Address Register for BAR N"]
pub mod pcie_at_ep_ib_ep_inbound_bar_address_translation_1;
#[doc = "PCIE_DMA_CHANNEL_0_CONTROL (rw) register accessor: PCIe DMA Channel 0 Control Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_control`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_CONTROL")]
pub type PcieDmaChannel0Control =
    crate::Reg<pcie_dma_channel_0_control::PcieDmaChannel0ControlSpec>;
#[doc = "PCIe DMA Channel 0 Control Register Reserved for future use"]
pub mod pcie_dma_channel_0_control;
#[doc = "PCIE_DMA_CHANNEL_0_START_POINTER_LOWER (rw) register accessor: PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_start_pointer_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_START_POINTER_LOWER")]
pub type PcieDmaChannel0StartPointerLower =
    crate::Reg<pcie_dma_channel_0_start_pointer_lower::PcieDmaChannel0StartPointerLowerSpec>;
#[doc = "PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_0_start_pointer_lower;
#[doc = "PCIE_DMA_CHANNEL_0_START_POINTER_UPPER (rw) register accessor: PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_start_pointer_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_START_POINTER_UPPER")]
pub type PcieDmaChannel0StartPointerUpper =
    crate::Reg<pcie_dma_channel_0_start_pointer_upper::PcieDmaChannel0StartPointerUpperSpec>;
#[doc = "PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_0_start_pointer_upper;
#[doc = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_LOWER (rw) register accessor: PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_attribute_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_attribute_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_attribute_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_LOWER")]
pub type PcieDmaChannel0AttributeLower =
    crate::Reg<pcie_dma_channel_0_attribute_lower::PcieDmaChannel0AttributeLowerSpec>;
#[doc = "PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_0_attribute_lower;
#[doc = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER (rw) register accessor: PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_attribute_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_attribute_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_attribute_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER")]
pub type PcieDmaChannel0AttributeUpper =
    crate::Reg<pcie_dma_channel_0_attribute_upper::PcieDmaChannel0AttributeUpperSpec>;
#[doc = "PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_0_attribute_upper;
#[doc = "PCIE_DMA_CHANNEL_1_CONTROL (rw) register accessor: PCIe DMA Channel 1 Control Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_control`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_CONTROL")]
pub type PcieDmaChannel1Control =
    crate::Reg<pcie_dma_channel_1_control::PcieDmaChannel1ControlSpec>;
#[doc = "PCIe DMA Channel 1 Control Register Reserved for future use"]
pub mod pcie_dma_channel_1_control;
#[doc = "PCIE_DMA_CHANNEL_1_START_POINTER_LOWER (rw) register accessor: PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_start_pointer_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_start_pointer_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_start_pointer_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_START_POINTER_LOWER")]
pub type PcieDmaChannel1StartPointerLower =
    crate::Reg<pcie_dma_channel_1_start_pointer_lower::PcieDmaChannel1StartPointerLowerSpec>;
#[doc = "PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_1_start_pointer_lower;
#[doc = "PCIE_DMA_CHANNEL_1_START_POINTER_UPPER (rw) register accessor: PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_start_pointer_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_start_pointer_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_start_pointer_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_START_POINTER_UPPER")]
pub type PcieDmaChannel1StartPointerUpper =
    crate::Reg<pcie_dma_channel_1_start_pointer_upper::PcieDmaChannel1StartPointerUpperSpec>;
#[doc = "PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_1_start_pointer_upper;
#[doc = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER (rw) register accessor: PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_attribute_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_attribute_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_attribute_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER")]
pub type PcieDmaChannel1AttributeLower =
    crate::Reg<pcie_dma_channel_1_attribute_lower::PcieDmaChannel1AttributeLowerSpec>;
#[doc = "PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_1_attribute_lower;
#[doc = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_UPPER (rw) register accessor: PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_attribute_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_attribute_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_attribute_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_UPPER")]
pub type PcieDmaChannel1AttributeUpper =
    crate::Reg<pcie_dma_channel_1_attribute_upper::PcieDmaChannel1AttributeUpperSpec>;
#[doc = "PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_1_attribute_upper;
#[doc = "PCIE_DMA_INTERRUPT (rw) register accessor: PCIe DMA Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT")]
pub type PcieDmaInterrupt = crate::Reg<pcie_dma_interrupt::PcieDmaInterruptSpec>;
#[doc = "PCIe DMA Interrupt Register"]
pub mod pcie_dma_interrupt;
#[doc = "PCIE_DMA_INTERRUPT_ENABLE (rw) register accessor: PCIe DMA Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt_enable`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT_ENABLE")]
pub type PcieDmaInterruptEnable = crate::Reg<pcie_dma_interrupt_enable::PcieDmaInterruptEnableSpec>;
#[doc = "PCIe DMA Interrupt Enable Register"]
pub mod pcie_dma_interrupt_enable;
#[doc = "PCIE_DMA_INTERRUPT_DISABLE (rw) register accessor: PCIe DMA Interrupt Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt_disable`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT_DISABLE")]
pub type PcieDmaInterruptDisable =
    crate::Reg<pcie_dma_interrupt_disable::PcieDmaInterruptDisableSpec>;
#[doc = "PCIe DMA Interrupt Disable Register"]
pub mod pcie_dma_interrupt_disable;
#[doc = "PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_uncorrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_inbound_buffer_uncorrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS")]
pub type PcieDmaInboundBufferUncorrectedEccErrors = crate::Reg<
    pcie_dma_inbound_buffer_uncorrected_ecc_errors::PcieDmaInboundBufferUncorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use"]
pub mod pcie_dma_inbound_buffer_uncorrected_ecc_errors;
#[doc = "PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_corrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_inbound_buffer_corrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS")]
pub type PcieDmaInboundBufferCorrectedEccErrors = crate::Reg<
    pcie_dma_inbound_buffer_corrected_ecc_errors::PcieDmaInboundBufferCorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use"]
pub mod pcie_dma_inbound_buffer_corrected_ecc_errors;
#[doc = "PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_outbound_buffer_uncorrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_outbound_buffer_uncorrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS")]
pub type PcieDmaOutboundBufferUncorrectedEccErrors = crate::Reg<
    pcie_dma_outbound_buffer_uncorrected_ecc_errors::PcieDmaOutboundBufferUncorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use"]
pub mod pcie_dma_outbound_buffer_uncorrected_ecc_errors;
#[doc = "PCIE_DMA_OUTBOUND_BUFFER_CORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_outbound_buffer_corrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_outbound_buffer_corrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_OUTBOUND_BUFFER_CORRECTED_ECC_ERRORS")]
pub type PcieDmaOutboundBufferCorrectedEccErrors = crate::Reg<
    pcie_dma_outbound_buffer_corrected_ecc_errors::PcieDmaOutboundBufferCorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use"]
pub mod pcie_dma_outbound_buffer_corrected_ecc_errors;
#[doc = "PCIE_DMA_CAPABILITY_AND_VERSION (r) register accessor: PCIe DMA Capability and Version Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_capability_and_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_capability_and_version`]
module"]
#[doc(alias = "PCIE_DMA_CAPABILITY_AND_VERSION")]
pub type PcieDmaCapabilityAndVersion =
    crate::Reg<pcie_dma_capability_and_version::PcieDmaCapabilityAndVersionSpec>;
#[doc = "PCIe DMA Capability and Version Register Reserved for future use"]
pub mod pcie_dma_capability_and_version;
#[doc = "PCIE_DMA_CONFIGURATION (r) register accessor: PCIe DMA Configuration Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_configuration::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_configuration`]
module"]
#[doc(alias = "PCIE_DMA_CONFIGURATION")]
pub type PcieDmaConfiguration = crate::Reg<pcie_dma_configuration::PcieDmaConfigurationSpec>;
#[doc = "PCIe DMA Configuration Register Reserved for future use"]
pub mod pcie_dma_configuration;
