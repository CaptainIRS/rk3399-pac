#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vendor_id_and_device_id: VendorIdAndDeviceId,
    command_and_status: CommandAndStatus,
    revision_id_and_class_code: RevisionIdAndClassCode,
    bist_header_type_latency_timer_and_cache_line_size_s:
        BistHeaderTypeLatencyTimerAndCacheLineSizeS,
    base_address_0: BaseAddress0,
    base_address_1: BaseAddress1,
    base_address_2: BaseAddress2,
    base_address_3: BaseAddress3,
    base_address_4: BaseAddress4,
    base_address_5: BaseAddress5,
    _reserved10: [u8; 0x04],
    subsystem_vendor_id_and_subsystem_id: SubsystemVendorIdAndSubsystemId,
    _reserved11: [u8; 0x04],
    capabilities_pointer: CapabilitiesPointer,
    _reserved12: [u8; 0x04],
    interrupt_line_and_interrupt_pin: InterruptLineAndInterruptPin,
    _reserved13: [u8; 0x40],
    power_management_capabilities: PowerManagementCapabilities,
    power_management_control_status_report: PowerManagementControlStatusReport,
    _reserved15: [u8; 0x08],
    msi_control: MsiControl,
    msi_message_low_address: MsiMessageLowAddress,
    msi_message_high_address: MsiMessageHighAddress,
    msi_message_data: MsiMessageData,
    msi_mask: MsiMask,
    msi_pending_bits: MsiPendingBits,
    _reserved21: [u8; 0x08],
    msi_x_control: MsiXControl,
    msi_x_table_offset: MsiXTableOffset,
    msi_x_pending_interrupt: MsiXPendingInterrupt,
    _reserved24: [u8; 0x04],
    pci_express_capability_list: PciExpressCapabilityList,
    pci_express_device_capabilities: PciExpressDeviceCapabilities,
    pci_express_device_control_and_status: PciExpressDeviceControlAndStatus,
    link_capabilities: LinkCapabilities,
    link_control_and_status: LinkControlAndStatus,
    _reserved29: [u8; 0x10],
    pci_express_device_capabilities_2: PciExpressDeviceCapabilities2,
    pci_express_device_control_and_status_2: PciExpressDeviceControlAndStatus2,
    link_capabilities_2: LinkCapabilities2,
    link_control_and_status_2: LinkControlAndStatus2,
    _reserved33: [u8; 0x0c],
    advanced_error_reporting_aer_enhanced_capability_header:
        AdvancedErrorReportingAerEnhancedCapabilityHeader,
    uncorrectable_error_status: UncorrectableErrorStatus,
    uncorrectable_error_mask: UncorrectableErrorMask,
    uncorrectable_error_severity: UncorrectableErrorSeverity,
    correctable_error_status: CorrectableErrorStatus,
    correctable_error_mask: CorrectableErrorMask,
    advanced_error_capabilities_and_control: AdvancedErrorCapabilitiesAndControl,
    header_log_0: HeaderLog0,
    header_log_1: HeaderLog1,
    header_log_2: HeaderLog2,
    header_log_3: HeaderLog3,
    _reserved44: [u8; 0x14],
    ari_extended_capability_header: AriExtendedCapabilityHeader,
    ari_capability_and_ari_control: AriCapabilityAndAriControl,
    _reserved46: [u8; 0x18],
    power_budgeting_enhanced_capability_header: PowerBudgetingEnhancedCapabilityHeader,
    power_budgeting_data_select: PowerBudgetingDataSelect,
    power_budgeting_data: PowerBudgetingData,
    power_budget_capability: PowerBudgetCapability,
    _reserved50: [u8; 0x10],
    resizable_bar_extended_capability_header: ResizableBarExtendedCapabilityHeader,
    resizable_bar_capability_0: ResizableBarCapability0,
    resizable_bar_control_0: ResizableBarControl0,
    resizable_bar_capability_1: ResizableBarCapability1,
    resizable_bar_control_1: ResizableBarControl1,
    resizable_bar_capability_2: ResizableBarCapability2,
    resizable_bar_control_2: ResizableBarControl2,
    resizable_bar_capability_3: ResizableBarCapability3,
    resizable_bar_control_3: ResizableBarControl3,
    resizable_bar_capability_4: ResizableBarCapability4,
    resizable_bar_control_4: ResizableBarControl4,
    resizable_bar_capability_5: ResizableBarCapability5,
    resizable_bar_control_5: ResizableBarControl5,
    _reserved63: [u8; 0x04],
    latency_tolerance_reporting_ltr_extended_capability_header:
        LatencyToleranceReportingLtrExtendedCapabilityHeader,
    ltr_max_snoop_max_no_snoop_latency: LtrMaxSnoopMaxNoSnoopLatency,
    dpa_extended_capability_header: DpaExtendedCapabilityHeader,
    dpa_capability: DpaCapability,
    dpa_latency_indicator: DpaLatencyIndicator,
    dpa_control_and_status_s: DpaControlAndStatusS,
    dynamic_power_allocation_array_0: DynamicPowerAllocationArray0,
    dynamic_power_allocation_array_1: DynamicPowerAllocationArray1,
    _reserved71: [u8; 0x28],
    sr_iov_extended_capability_header: SrIovExtendedCapabilityHeader,
    sr_iov_capabilities: SrIovCapabilities,
    sr_iov_control_and_status_s: SrIovControlAndStatusS,
    initial_vfs_total_vfs: InitialVfsTotalVfs,
    function_dependency_link_numvfs: FunctionDependencyLinkNumvfs,
    vf_offset_stride: VfOffsetStride,
    vf_device_id: VfDeviceId,
    supported_page_sizes: SupportedPageSizes,
    system_page_size: SystemPageSize,
    vf_base_address_0: VfBaseAddress0,
    vf_base_address_1: VfBaseAddress1,
    vf_base_address_2: VfBaseAddress2,
    vf_base_address_3: VfBaseAddress3,
    vf_base_address_4: VfBaseAddress4,
    vf_base_address_5: VfBaseAddress5,
    vf_migration_state_array_offset: VfMigrationStateArrayOffset,
    _reserved87: [u8; 0x34],
    tph_requester_extended_capability_header: TphRequesterExtendedCapabilityHeader,
    tph_requester_capability: TphRequesterCapability,
    tph_requester_control: TphRequesterControl,
    tph_st_table_0: TphStTable0,
    tph_st_table_1: TphStTable1,
    tph_st_table_2: TphStTable2,
    tph_st_table_3: TphStTable3,
    _reserved94: [u8; 0x0670],
    l1_pm_substates_extended_capability_header: L1PmSubstatesExtendedCapabilityHeader,
    l1_pm_substates_capabilities: L1PmSubstatesCapabilities,
    l1_pm_substates_control_1: L1PmSubstatesControl1,
    l1_pm_substates_control_2: L1PmSubstatesControl2,
}
impl RegisterBlock {
    #[doc = "0x00 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn vendor_id_and_device_id(&self) -> &VendorIdAndDeviceId {
        &self.vendor_id_and_device_id
    }
    #[doc = "0x04 - Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position."]
    #[inline(always)]
    pub const fn command_and_status(&self) -> &CommandAndStatus {
        &self.command_and_status
    }
    #[doc = "0x08 - Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus"]
    #[inline(always)]
    pub const fn revision_id_and_class_code(&self) -> &RevisionIdAndClassCode {
        &self.revision_id_and_class_code
    }
    #[doc = "0x0c - BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus."]
    #[inline(always)]
    pub const fn bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &BistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x10 - Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn base_address_0(&self) -> &BaseAddress0 {
        &self.base_address_0
    }
    #[doc = "0x14 - Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn base_address_1(&self) -> &BaseAddress1 {
        &self.base_address_1
    }
    #[doc = "0x18 - Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn base_address_2(&self) -> &BaseAddress2 {
        &self.base_address_2
    }
    #[doc = "0x1c - Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn base_address_3(&self) -> &BaseAddress3 {
        &self.base_address_3
    }
    #[doc = "0x20 - Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn base_address_4(&self) -> &BaseAddress4 {
        &self.base_address_4
    }
    #[doc = "0x24 - Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn base_address_5(&self) -> &BaseAddress5 {
        &self.base_address_5
    }
    #[doc = "0x2c - Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn subsystem_vendor_id_and_subsystem_id(&self) -> &SubsystemVendorIdAndSubsystemId {
        &self.subsystem_vendor_id_and_subsystem_id
    }
    #[doc = "0x34 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn capabilities_pointer(&self) -> &CapabilitiesPointer {
        &self.capabilities_pointer
    }
    #[doc = "0x3c - Interrupt Line and Interrupt Pin Register Reserved"]
    #[inline(always)]
    pub const fn interrupt_line_and_interrupt_pin(&self) -> &InterruptLineAndInterruptPin {
        &self.interrupt_line_and_interrupt_pin
    }
    #[doc = "0x80 - Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
    #[inline(always)]
    pub const fn power_management_capabilities(&self) -> &PowerManagementCapabilities {
        &self.power_management_capabilities
    }
    #[doc = "0x84 - Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn power_management_control_status_report(
        &self,
    ) -> &PowerManagementControlStatusReport {
        &self.power_management_control_status_report
    }
    #[doc = "0x90 - MSI Control Register Reserved"]
    #[inline(always)]
    pub const fn msi_control(&self) -> &MsiControl {
        &self.msi_control
    }
    #[doc = "0x94 - MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn msi_message_low_address(&self) -> &MsiMessageLowAddress {
        &self.msi_message_low_address
    }
    #[doc = "0x98 - MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn msi_message_high_address(&self) -> &MsiMessageHighAddress {
        &self.msi_message_high_address
    }
    #[doc = "0x9c - MSI Message Data Register Hardwired to 0"]
    #[inline(always)]
    pub const fn msi_message_data(&self) -> &MsiMessageData {
        &self.msi_message_data
    }
    #[doc = "0xa0 - MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn msi_mask(&self) -> &MsiMask {
        &self.msi_mask
    }
    #[doc = "0xa4 - MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub const fn msi_pending_bits(&self) -> &MsiPendingBits {
        &self.msi_pending_bits
    }
    #[doc = "0xb0 - MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub const fn msi_x_control(&self) -> &MsiXControl {
        &self.msi_x_control
    }
    #[doc = "0xb4 - MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn msi_x_table_offset(&self) -> &MsiXTableOffset {
        &self.msi_x_table_offset
    }
    #[doc = "0xb8 - MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub const fn msi_x_pending_interrupt(&self) -> &MsiXPendingInterrupt {
        &self.msi_x_pending_interrupt
    }
    #[doc = "0xc0 - PCI Express Capability List Register Reserved"]
    #[inline(always)]
    pub const fn pci_express_capability_list(&self) -> &PciExpressCapabilityList {
        &self.pci_express_capability_list
    }
    #[doc = "0xc4 - PCI Express Device Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_capabilities(&self) -> &PciExpressDeviceCapabilities {
        &self.pci_express_device_capabilities
    }
    #[doc = "0xc8 - PCI Express Device Control and Status Register Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_control_and_status(&self) -> &PciExpressDeviceControlAndStatus {
        &self.pci_express_device_control_and_status
    }
    #[doc = "0xcc - Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
    #[inline(always)]
    pub const fn link_capabilities(&self) -> &LinkCapabilities {
        &self.link_capabilities
    }
    #[doc = "0xd0 - Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub const fn link_control_and_status(&self) -> &LinkControlAndStatus {
        &self.link_control_and_status
    }
    #[doc = "0xe4 - PCI Express Device Capabilities Register 2 Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_capabilities_2(&self) -> &PciExpressDeviceCapabilities2 {
        &self.pci_express_device_capabilities_2
    }
    #[doc = "0xe8 - PCI Express Device Control and Status Register 2 Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_control_and_status_2(
        &self,
    ) -> &PciExpressDeviceControlAndStatus2 {
        &self.pci_express_device_control_and_status_2
    }
    #[doc = "0xec - Link Capabilities Register 2 RSVD"]
    #[inline(always)]
    pub const fn link_capabilities_2(&self) -> &LinkCapabilities2 {
        &self.link_capabilities_2
    }
    #[doc = "0xf0 - Link Control and Status Register 2 Reserved"]
    #[inline(always)]
    pub const fn link_control_and_status_2(&self) -> &LinkControlAndStatus2 {
        &self.link_control_and_status_2
    }
    #[doc = "0x100 - Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn advanced_error_reporting_aer_enhanced_capability_header(
        &self,
    ) -> &AdvancedErrorReportingAerEnhancedCapabilityHeader {
        &self.advanced_error_reporting_aer_enhanced_capability_header
    }
    #[doc = "0x104 - Uncorrectable Error Status Register (no description)"]
    #[inline(always)]
    pub const fn uncorrectable_error_status(&self) -> &UncorrectableErrorStatus {
        &self.uncorrectable_error_status
    }
    #[doc = "0x108 - Uncorrectable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn uncorrectable_error_mask(&self) -> &UncorrectableErrorMask {
        &self.uncorrectable_error_mask
    }
    #[doc = "0x10c - Uncorrectable Error Severity Register Reserved"]
    #[inline(always)]
    pub const fn uncorrectable_error_severity(&self) -> &UncorrectableErrorSeverity {
        &self.uncorrectable_error_severity
    }
    #[doc = "0x110 - Correctable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn correctable_error_status(&self) -> &CorrectableErrorStatus {
        &self.correctable_error_status
    }
    #[doc = "0x114 - Correctable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn correctable_error_mask(&self) -> &CorrectableErrorMask {
        &self.correctable_error_mask
    }
    #[doc = "0x118 - Advanced Error Capabilities and Control Register Reserved"]
    #[inline(always)]
    pub const fn advanced_error_capabilities_and_control(
        &self,
    ) -> &AdvancedErrorCapabilitiesAndControl {
        &self.advanced_error_capabilities_and_control
    }
    #[doc = "0x11c - Header Log Register 0 First DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn header_log_0(&self) -> &HeaderLog0 {
        &self.header_log_0
    }
    #[doc = "0x120 - Header Log Register 1 Second DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn header_log_1(&self) -> &HeaderLog1 {
        &self.header_log_1
    }
    #[doc = "0x124 - Header Log Register 2 Third DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn header_log_2(&self) -> &HeaderLog2 {
        &self.header_log_2
    }
    #[doc = "0x128 - Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub const fn header_log_3(&self) -> &HeaderLog3 {
        &self.header_log_3
    }
    #[doc = "0x140 - ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn ari_extended_capability_header(&self) -> &AriExtendedCapabilityHeader {
        &self.ari_extended_capability_header
    }
    #[doc = "0x144 - ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0."]
    #[inline(always)]
    pub const fn ari_capability_and_ari_control(&self) -> &AriCapabilityAndAriControl {
        &self.ari_capability_and_ari_control
    }
    #[doc = "0x160 - Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn power_budgeting_enhanced_capability_header(
        &self,
    ) -> &PowerBudgetingEnhancedCapabilityHeader {
        &self.power_budgeting_enhanced_capability_header
    }
    #[doc = "0x164 - Power Budgeting Data Select Register (no description)"]
    #[inline(always)]
    pub const fn power_budgeting_data_select(&self) -> &PowerBudgetingDataSelect {
        &self.power_budgeting_data_select
    }
    #[doc = "0x168 - Power Budgeting Data Register Reserved"]
    #[inline(always)]
    pub const fn power_budgeting_data(&self) -> &PowerBudgetingData {
        &self.power_budgeting_data
    }
    #[doc = "0x16c - Power Budget Capability Register Reserved"]
    #[inline(always)]
    pub const fn power_budget_capability(&self) -> &PowerBudgetCapability {
        &self.power_budget_capability
    }
    #[doc = "0x180 - Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn resizable_bar_extended_capability_header(
        &self,
    ) -> &ResizableBarExtendedCapabilityHeader {
        &self.resizable_bar_extended_capability_header
    }
    #[doc = "0x184 - Resizable BAR Capability Register 0 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_0(&self) -> &ResizableBarCapability0 {
        &self.resizable_bar_capability_0
    }
    #[doc = "0x188 - Resizable BAR Control Register 0 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_0(&self) -> &ResizableBarControl0 {
        &self.resizable_bar_control_0
    }
    #[doc = "0x18c - Resizable BAR Capability Register 1 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_1(&self) -> &ResizableBarCapability1 {
        &self.resizable_bar_capability_1
    }
    #[doc = "0x190 - Resizable BAR Control Register 1 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_1(&self) -> &ResizableBarControl1 {
        &self.resizable_bar_control_1
    }
    #[doc = "0x194 - Resizable BAR Capability Register 2 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_2(&self) -> &ResizableBarCapability2 {
        &self.resizable_bar_capability_2
    }
    #[doc = "0x198 - Resizable BAR Control Register 2 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_2(&self) -> &ResizableBarControl2 {
        &self.resizable_bar_control_2
    }
    #[doc = "0x19c - Resizable BAR Capability Register 3 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_3(&self) -> &ResizableBarCapability3 {
        &self.resizable_bar_capability_3
    }
    #[doc = "0x1a0 - Resizable BAR Control Register 3 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_3(&self) -> &ResizableBarControl3 {
        &self.resizable_bar_control_3
    }
    #[doc = "0x1a4 - Resizable BAR Capability Register 4 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_4(&self) -> &ResizableBarCapability4 {
        &self.resizable_bar_capability_4
    }
    #[doc = "0x1a8 - Resizable BAR Control Register 4 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_4(&self) -> &ResizableBarControl4 {
        &self.resizable_bar_control_4
    }
    #[doc = "0x1ac - Resizable BAR Capability Register 5 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_capability_5(&self) -> &ResizableBarCapability5 {
        &self.resizable_bar_capability_5
    }
    #[doc = "0x1b0 - Resizable BAR Control Register 5 Reserved"]
    #[inline(always)]
    pub const fn resizable_bar_control_5(&self) -> &ResizableBarControl5 {
        &self.resizable_bar_control_5
    }
    #[doc = "0x1b8 - Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn latency_tolerance_reporting_ltr_extended_capability_header(
        &self,
    ) -> &LatencyToleranceReportingLtrExtendedCapabilityHeader {
        &self.latency_tolerance_reporting_ltr_extended_capability_header
    }
    #[doc = "0x1bc - LTR Max Snoop/Max No-Snoop Latency Register Reserved"]
    #[inline(always)]
    pub const fn ltr_max_snoop_max_no_snoop_latency(&self) -> &LtrMaxSnoopMaxNoSnoopLatency {
        &self.ltr_max_snoop_max_no_snoop_latency
    }
    #[doc = "0x1c0 - DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn dpa_extended_capability_header(&self) -> &DpaExtendedCapabilityHeader {
        &self.dpa_extended_capability_header
    }
    #[doc = "0x1c4 - DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register."]
    #[inline(always)]
    pub const fn dpa_capability(&self) -> &DpaCapability {
        &self.dpa_capability
    }
    #[doc = "0x1c8 - DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
    #[inline(always)]
    pub const fn dpa_latency_indicator(&self) -> &DpaLatencyIndicator {
        &self.dpa_latency_indicator
    }
    #[doc = "0x1cc - DPA Control and Status Registers Reserved"]
    #[inline(always)]
    pub const fn dpa_control_and_status_s(&self) -> &DpaControlAndStatusS {
        &self.dpa_control_and_status_s
    }
    #[doc = "0x1d0 - Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub const fn dynamic_power_allocation_array_0(&self) -> &DynamicPowerAllocationArray0 {
        &self.dynamic_power_allocation_array_0
    }
    #[doc = "0x1d4 - Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
    #[inline(always)]
    pub const fn dynamic_power_allocation_array_1(&self) -> &DynamicPowerAllocationArray1 {
        &self.dynamic_power_allocation_array_1
    }
    #[doc = "0x200 - SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn sr_iov_extended_capability_header(&self) -> &SrIovExtendedCapabilityHeader {
        &self.sr_iov_extended_capability_header
    }
    #[doc = "0x204 - SR-IOV Capabilities Register Reserved"]
    #[inline(always)]
    pub const fn sr_iov_capabilities(&self) -> &SrIovCapabilities {
        &self.sr_iov_capabilities
    }
    #[doc = "0x208 - SR-IOV Control and Status Registers Not implemented."]
    #[inline(always)]
    pub const fn sr_iov_control_and_status_s(&self) -> &SrIovControlAndStatusS {
        &self.sr_iov_control_and_status_s
    }
    #[doc = "0x20c - Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
    #[inline(always)]
    pub const fn initial_vfs_total_vfs(&self) -> &InitialVfsTotalVfs {
        &self.initial_vfs_total_vfs
    }
    #[doc = "0x210 - Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn function_dependency_link_numvfs(&self) -> &FunctionDependencyLinkNumvfs {
        &self.function_dependency_link_numvfs
    }
    #[doc = "0x214 - VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
    #[inline(always)]
    pub const fn vf_offset_stride(&self) -> &VfOffsetStride {
        &self.vf_offset_stride
    }
    #[doc = "0x218 - VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
    #[inline(always)]
    pub const fn vf_device_id(&self) -> &VfDeviceId {
        &self.vf_device_id
    }
    #[doc = "0x21c - Supported Page Sizes Register Reserved"]
    #[inline(always)]
    pub const fn supported_page_sizes(&self) -> &SupportedPageSizes {
        &self.supported_page_sizes
    }
    #[doc = "0x220 - System Page Size Register Reserved"]
    #[inline(always)]
    pub const fn system_page_size(&self) -> &SystemPageSize {
        &self.system_page_size
    }
    #[doc = "0x224 - VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn vf_base_address_0(&self) -> &VfBaseAddress0 {
        &self.vf_base_address_0
    }
    #[doc = "0x228 - VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn vf_base_address_1(&self) -> &VfBaseAddress1 {
        &self.vf_base_address_1
    }
    #[doc = "0x22c - VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn vf_base_address_2(&self) -> &VfBaseAddress2 {
        &self.vf_base_address_2
    }
    #[doc = "0x230 - VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn vf_base_address_3(&self) -> &VfBaseAddress3 {
        &self.vf_base_address_3
    }
    #[doc = "0x234 - VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
    #[inline(always)]
    pub const fn vf_base_address_4(&self) -> &VfBaseAddress4 {
        &self.vf_base_address_4
    }
    #[doc = "0x238 - VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn vf_base_address_5(&self) -> &VfBaseAddress5 {
        &self.vf_base_address_5
    }
    #[doc = "0x23c - VF Migration State Array Offset Register (no description)"]
    #[inline(always)]
    pub const fn vf_migration_state_array_offset(&self) -> &VfMigrationStateArrayOffset {
        &self.vf_migration_state_array_offset
    }
    #[doc = "0x274 - TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn tph_requester_extended_capability_header(
        &self,
    ) -> &TphRequesterExtendedCapabilityHeader {
        &self.tph_requester_extended_capability_header
    }
    #[doc = "0x278 - TPH Requester Capability Register Reserved"]
    #[inline(always)]
    pub const fn tph_requester_capability(&self) -> &TphRequesterCapability {
        &self.tph_requester_capability
    }
    #[doc = "0x27c - TPH Requester Control Register Reserved"]
    #[inline(always)]
    pub const fn tph_requester_control(&self) -> &TphRequesterControl {
        &self.tph_requester_control
    }
    #[doc = "0x280 - TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn tph_st_table_0(&self) -> &TphStTable0 {
        &self.tph_st_table_0
    }
    #[doc = "0x284 - TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn tph_st_table_1(&self) -> &TphStTable1 {
        &self.tph_st_table_1
    }
    #[doc = "0x288 - TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn tph_st_table_2(&self) -> &TphStTable2 {
        &self.tph_st_table_2
    }
    #[doc = "0x28c - TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
    #[inline(always)]
    pub const fn tph_st_table_3(&self) -> &TphStTable3 {
        &self.tph_st_table_3
    }
    #[doc = "0x900 - L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn l1_pm_substates_extended_capability_header(
        &self,
    ) -> &L1PmSubstatesExtendedCapabilityHeader {
        &self.l1_pm_substates_extended_capability_header
    }
    #[doc = "0x904 - L1 PM Substates Capabilities Register RSVD"]
    #[inline(always)]
    pub const fn l1_pm_substates_capabilities(&self) -> &L1PmSubstatesCapabilities {
        &self.l1_pm_substates_capabilities
    }
    #[doc = "0x908 - L1 PM Substates Control 1 Register (no description)"]
    #[inline(always)]
    pub const fn l1_pm_substates_control_1(&self) -> &L1PmSubstatesControl1 {
        &self.l1_pm_substates_control_1
    }
    #[doc = "0x90c - L1 PM Substates Control 2 Register RSVD"]
    #[inline(always)]
    pub const fn l1_pm_substates_control_2(&self) -> &L1PmSubstatesControl2 {
        &self.l1_pm_substates_control_2
    }
}
#[doc = "VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_id_and_device_id`]
module"]
#[doc(alias = "VENDOR_ID_AND_DEVICE_ID")]
pub type VendorIdAndDeviceId = crate::Reg<vendor_id_and_device_id::VendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus."]
pub mod vendor_id_and_device_id;
#[doc = "COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command_and_status`]
module"]
#[doc(alias = "COMMAND_AND_STATUS")]
pub type CommandAndStatus = crate::Reg<command_and_status::CommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus by writing a 1 into this bit position."]
pub mod command_and_status;
#[doc = "REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision_id_and_class_code`]
module"]
#[doc(alias = "REVISION_ID_AND_CLASS_CODE")]
pub type RevisionIdAndClassCode =
    crate::Reg<revision_id_and_class_code::RevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be rewritten independently for each Function from the local management bus"]
pub mod revision_id_and_class_code;
#[doc = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (rw) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_header_type_latency_timer_and_cache_line_size_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_header_type_latency_timer_and_cache_line_size_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type BistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < bist_header_type_latency_timer_and_cache_line_size_s :: BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register.It can be accessed using local management bus."]
pub mod bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "BASE_ADDRESS_0 (rw) register accessor: Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_0`]
module"]
#[doc(alias = "BASE_ADDRESS_0")]
pub type BaseAddress0 = crate::Reg<base_address_0::BaseAddress0Spec>;
#[doc = "Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod base_address_0;
#[doc = "BASE_ADDRESS_1 (rw) register accessor: Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_1`]
module"]
#[doc(alias = "BASE_ADDRESS_1")]
pub type BaseAddress1 = crate::Reg<base_address_1::BaseAddress1Spec>;
#[doc = "Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod base_address_1;
#[doc = "BASE_ADDRESS_2 (rw) register accessor: Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_2`]
module"]
#[doc(alias = "BASE_ADDRESS_2")]
pub type BaseAddress2 = crate::Reg<base_address_2::BaseAddress2Spec>;
#[doc = "Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod base_address_2;
#[doc = "BASE_ADDRESS_3 (rw) register accessor: Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_3`]
module"]
#[doc(alias = "BASE_ADDRESS_3")]
pub type BaseAddress3 = crate::Reg<base_address_3::BaseAddress3Spec>;
#[doc = "Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod base_address_3;
#[doc = "BASE_ADDRESS_4 (rw) register accessor: Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_4`]
module"]
#[doc(alias = "BASE_ADDRESS_4")]
pub type BaseAddress4 = crate::Reg<base_address_4::BaseAddress4Spec>;
#[doc = "Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod base_address_4;
#[doc = "BASE_ADDRESS_5 (rw) register accessor: Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_address_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_5`]
module"]
#[doc(alias = "BASE_ADDRESS_5")]
pub type BaseAddress5 = crate::Reg<base_address_5::BaseAddress5Spec>;
#[doc = "Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod base_address_5;
#[doc = "SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID (r) register accessor: Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsystem_vendor_id_and_subsystem_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsystem_vendor_id_and_subsystem_id`]
module"]
#[doc(alias = "SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID")]
pub type SubsystemVendorIdAndSubsystemId =
    crate::Reg<subsystem_vendor_id_and_subsystem_id::SubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
pub mod subsystem_vendor_id_and_subsystem_id;
#[doc = "CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities_pointer`]
module"]
#[doc(alias = "CAPABILITIES_POINTER")]
pub type CapabilitiesPointer = crate::Reg<capabilities_pointer::CapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod capabilities_pointer;
#[doc = "INTERRUPT_LINE_AND_INTERRUPT_PIN (rw) register accessor: Interrupt Line and Interrupt Pin Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_line_and_interrupt_pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_line_and_interrupt_pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_line_and_interrupt_pin`]
module"]
#[doc(alias = "INTERRUPT_LINE_AND_INTERRUPT_PIN")]
pub type InterruptLineAndInterruptPin =
    crate::Reg<interrupt_line_and_interrupt_pin::InterruptLineAndInterruptPinSpec>;
#[doc = "Interrupt Line and Interrupt Pin Register Reserved"]
pub mod interrupt_line_and_interrupt_pin;
#[doc = "POWER_MANAGEMENT_CAPABILITIES (r) register accessor: Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_management_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_management_capabilities`]
module"]
#[doc(alias = "POWER_MANAGEMENT_CAPABILITIES")]
pub type PowerManagementCapabilities =
    crate::Reg<power_management_capabilities::PowerManagementCapabilitiesSpec>;
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
pub mod power_management_capabilities;
#[doc = "POWER_MANAGEMENT_CONTROL_STATUS_REPORT (rw) register accessor: Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_management_control_status_report::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_management_control_status_report::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_management_control_status_report`]
module"]
#[doc(alias = "POWER_MANAGEMENT_CONTROL_STATUS_REPORT")]
pub type PowerManagementControlStatusReport =
    crate::Reg<power_management_control_status_report::PowerManagementControlStatusReportSpec>;
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub mod power_management_control_status_report;
#[doc = "MSI_CONTROL (rw) register accessor: MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_control`]
module"]
#[doc(alias = "MSI_CONTROL")]
pub type MsiControl = crate::Reg<msi_control::MsiControlSpec>;
#[doc = "MSI Control Register Reserved"]
pub mod msi_control;
#[doc = "MSI_MESSAGE_LOW_ADDRESS (rw) register accessor: MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_message_low_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_message_low_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_message_low_address`]
module"]
#[doc(alias = "MSI_MESSAGE_LOW_ADDRESS")]
pub type MsiMessageLowAddress = crate::Reg<msi_message_low_address::MsiMessageLowAddressSpec>;
#[doc = "MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub mod msi_message_low_address;
#[doc = "MSI_MESSAGE_HIGH_ADDRESS (rw) register accessor: MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_message_high_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_message_high_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_message_high_address`]
module"]
#[doc(alias = "MSI_MESSAGE_HIGH_ADDRESS")]
pub type MsiMessageHighAddress = crate::Reg<msi_message_high_address::MsiMessageHighAddressSpec>;
#[doc = "MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub mod msi_message_high_address;
#[doc = "MSI_MESSAGE_DATA (rw) register accessor: MSI Message Data Register Hardwired to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_message_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_message_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_message_data`]
module"]
#[doc(alias = "MSI_MESSAGE_DATA")]
pub type MsiMessageData = crate::Reg<msi_message_data::MsiMessageDataSpec>;
#[doc = "MSI Message Data Register Hardwired to 0"]
pub mod msi_message_data;
#[doc = "MSI_MASK (rw) register accessor: MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_mask`]
module"]
#[doc(alias = "MSI_MASK")]
pub type MsiMask = crate::Reg<msi_mask::MsiMaskSpec>;
#[doc = "MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod msi_mask;
#[doc = "MSI_PENDING_BITS (r) register accessor: MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_pending_bits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_pending_bits`]
module"]
#[doc(alias = "MSI_PENDING_BITS")]
pub type MsiPendingBits = crate::Reg<msi_pending_bits::MsiPendingBitsSpec>;
#[doc = "MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub mod msi_pending_bits;
#[doc = "MSI_X_CONTROL (rw) register accessor: MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_x_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_x_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_x_control`]
module"]
#[doc(alias = "MSI_X_CONTROL")]
pub type MsiXControl = crate::Reg<msi_x_control::MsiXControlSpec>;
#[doc = "MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub mod msi_x_control;
#[doc = "MSI_X_TABLE_OFFSET (r) register accessor: MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_x_table_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_x_table_offset`]
module"]
#[doc(alias = "MSI_X_TABLE_OFFSET")]
pub type MsiXTableOffset = crate::Reg<msi_x_table_offset::MsiXTableOffsetSpec>;
#[doc = "MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod msi_x_table_offset;
#[doc = "MSI_X_PENDING_INTERRUPT (r) register accessor: MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_x_pending_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_x_pending_interrupt`]
module"]
#[doc(alias = "MSI_X_PENDING_INTERRUPT")]
pub type MsiXPendingInterrupt = crate::Reg<msi_x_pending_interrupt::MsiXPendingInterruptSpec>;
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub mod msi_x_pending_interrupt;
#[doc = "PCI_EXPRESS_CAPABILITY_LIST (r) register accessor: PCI Express Capability List Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_capability_list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_capability_list`]
module"]
#[doc(alias = "PCI_EXPRESS_CAPABILITY_LIST")]
pub type PciExpressCapabilityList =
    crate::Reg<pci_express_capability_list::PciExpressCapabilityListSpec>;
#[doc = "PCI Express Capability List Register Reserved"]
pub mod pci_express_capability_list;
#[doc = "PCI_EXPRESS_DEVICE_CAPABILITIES (r) register accessor: PCI Express Device Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_capabilities`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CAPABILITIES")]
pub type PciExpressDeviceCapabilities =
    crate::Reg<pci_express_device_capabilities::PciExpressDeviceCapabilitiesSpec>;
#[doc = "PCI Express Device Capabilities Register Reserved"]
pub mod pci_express_device_capabilities;
#[doc = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS (rw) register accessor: PCI Express Device Control and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pci_express_device_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_control_and_status`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS")]
pub type PciExpressDeviceControlAndStatus =
    crate::Reg<pci_express_device_control_and_status::PciExpressDeviceControlAndStatusSpec>;
#[doc = "PCI Express Device Control and Status Register Reserved"]
pub mod pci_express_device_control_and_status;
#[doc = "LINK_CAPABILITIES (r) register accessor: Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_capabilities`]
module"]
#[doc(alias = "LINK_CAPABILITIES")]
pub type LinkCapabilities = crate::Reg<link_capabilities::LinkCapabilitiesSpec>;
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device."]
pub mod link_capabilities;
#[doc = "LINK_CONTROL_AND_STATUS (rw) register accessor: Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_control_and_status`]
module"]
#[doc(alias = "LINK_CONTROL_AND_STATUS")]
pub type LinkControlAndStatus = crate::Reg<link_control_and_status::LinkControlAndStatusSpec>;
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub mod link_control_and_status;
#[doc = "PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PciExpressDeviceCapabilities2 =
    crate::Reg<pci_express_device_capabilities_2::PciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities Register 2 Reserved"]
pub mod pci_express_device_capabilities_2;
#[doc = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 (rw) register accessor: PCI Express Device Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pci_express_device_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_control_and_status_2`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2")]
pub type PciExpressDeviceControlAndStatus2 =
    crate::Reg<pci_express_device_control_and_status_2::PciExpressDeviceControlAndStatus2Spec>;
#[doc = "PCI Express Device Control and Status Register 2 Reserved"]
pub mod pci_express_device_control_and_status_2;
#[doc = "LINK_CAPABILITIES_2 (r) register accessor: Link Capabilities Register 2 RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_capabilities_2`]
module"]
#[doc(alias = "LINK_CAPABILITIES_2")]
pub type LinkCapabilities2 = crate::Reg<link_capabilities_2::LinkCapabilities2Spec>;
#[doc = "Link Capabilities Register 2 RSVD"]
pub mod link_capabilities_2;
#[doc = "LINK_CONTROL_AND_STATUS_2 (rw) register accessor: Link Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_control_and_status_2`]
module"]
#[doc(alias = "LINK_CONTROL_AND_STATUS_2")]
pub type LinkControlAndStatus2 = crate::Reg<link_control_and_status_2::LinkControlAndStatus2Spec>;
#[doc = "Link Control and Status Register 2 Reserved"]
pub mod link_control_and_status_2;
#[doc = "ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER (r) register accessor: Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced_error_reporting_aer_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advanced_error_reporting_aer_enhanced_capability_header`]
module"]
#[doc(alias = "ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER")]
pub type AdvancedErrorReportingAerEnhancedCapabilityHeader = crate :: Reg < advanced_error_reporting_aer_enhanced_capability_header :: AdvancedErrorReportingAerEnhancedCapabilityHeaderSpec > ;
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod advanced_error_reporting_aer_enhanced_capability_header;
#[doc = "UNCORRECTABLE_ERROR_STATUS (rw) register accessor: Uncorrectable Error Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_status`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_STATUS")]
pub type UncorrectableErrorStatus =
    crate::Reg<uncorrectable_error_status::UncorrectableErrorStatusSpec>;
#[doc = "Uncorrectable Error Status Register (no description)"]
pub mod uncorrectable_error_status;
#[doc = "UNCORRECTABLE_ERROR_MASK (rw) register accessor: Uncorrectable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_mask`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_MASK")]
pub type UncorrectableErrorMask = crate::Reg<uncorrectable_error_mask::UncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register Reserved"]
pub mod uncorrectable_error_mask;
#[doc = "UNCORRECTABLE_ERROR_SEVERITY (rw) register accessor: Uncorrectable Error Severity Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_severity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_severity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_severity`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_SEVERITY")]
pub type UncorrectableErrorSeverity =
    crate::Reg<uncorrectable_error_severity::UncorrectableErrorSeveritySpec>;
#[doc = "Uncorrectable Error Severity Register Reserved"]
pub mod uncorrectable_error_severity;
#[doc = "CORRECTABLE_ERROR_STATUS (rw) register accessor: Correctable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`correctable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@correctable_error_status`]
module"]
#[doc(alias = "CORRECTABLE_ERROR_STATUS")]
pub type CorrectableErrorStatus = crate::Reg<correctable_error_status::CorrectableErrorStatusSpec>;
#[doc = "Correctable Error Status Register Reserved"]
pub mod correctable_error_status;
#[doc = "CORRECTABLE_ERROR_MASK (rw) register accessor: Correctable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`correctable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@correctable_error_mask`]
module"]
#[doc(alias = "CORRECTABLE_ERROR_MASK")]
pub type CorrectableErrorMask = crate::Reg<correctable_error_mask::CorrectableErrorMaskSpec>;
#[doc = "Correctable Error Mask Register Reserved"]
pub mod correctable_error_mask;
#[doc = "ADVANCED_ERROR_CAPABILITIES_AND_CONTROL (rw) register accessor: Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced_error_capabilities_and_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`advanced_error_capabilities_and_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advanced_error_capabilities_and_control`]
module"]
#[doc(alias = "ADVANCED_ERROR_CAPABILITIES_AND_CONTROL")]
pub type AdvancedErrorCapabilitiesAndControl =
    crate::Reg<advanced_error_capabilities_and_control::AdvancedErrorCapabilitiesAndControlSpec>;
#[doc = "Advanced Error Capabilities and Control Register Reserved"]
pub mod advanced_error_capabilities_and_control;
#[doc = "HEADER_LOG_0 (r) register accessor: Header Log Register 0 First DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_0`]
module"]
#[doc(alias = "HEADER_LOG_0")]
pub type HeaderLog0 = crate::Reg<header_log_0::HeaderLog0Spec>;
#[doc = "Header Log Register 0 First DWORD of captured TLP header STICKY."]
pub mod header_log_0;
#[doc = "HEADER_LOG_1 (r) register accessor: Header Log Register 1 Second DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_1`]
module"]
#[doc(alias = "HEADER_LOG_1")]
pub type HeaderLog1 = crate::Reg<header_log_1::HeaderLog1Spec>;
#[doc = "Header Log Register 1 Second DWORD of captured TLP header STICKY."]
pub mod header_log_1;
#[doc = "HEADER_LOG_2 (r) register accessor: Header Log Register 2 Third DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_2`]
module"]
#[doc(alias = "HEADER_LOG_2")]
pub type HeaderLog2 = crate::Reg<header_log_2::HeaderLog2Spec>;
#[doc = "Header Log Register 2 Third DWORD of captured TLP header STICKY."]
pub mod header_log_2;
#[doc = "HEADER_LOG_3 (r) register accessor: Header Log Register 3 Fourth DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_3`]
module"]
#[doc(alias = "HEADER_LOG_3")]
pub type HeaderLog3 = crate::Reg<header_log_3::HeaderLog3Spec>;
#[doc = "Header Log Register 3 Fourth DWORD of captured TLP header STICKY."]
pub mod header_log_3;
#[doc = "ARI_EXTENDED_CAPABILITY_HEADER (r) register accessor: ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ari_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ari_extended_capability_header`]
module"]
#[doc(alias = "ARI_EXTENDED_CAPABILITY_HEADER")]
pub type AriExtendedCapabilityHeader =
    crate::Reg<ari_extended_capability_header::AriExtendedCapabilityHeaderSpec>;
#[doc = "ARI Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod ari_extended_capability_header;
#[doc = "ARI_CAPABILITY_AND_ARI_CONTROL (r) register accessor: ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ari_capability_and_ari_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ari_capability_and_ari_control`]
module"]
#[doc(alias = "ARI_CAPABILITY_AND_ARI_CONTROL")]
pub type AriCapabilityAndAriControl =
    crate::Reg<ari_capability_and_ari_control::AriCapabilityAndAriControlSpec>;
#[doc = "ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0."]
pub mod ari_capability_and_ari_control;
#[doc = "POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER (r) register accessor: Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budgeting_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_budgeting_enhanced_capability_header`]
module"]
#[doc(alias = "POWER_BUDGETING_ENHANCED_CAPABILITY_HEADER")]
pub type PowerBudgetingEnhancedCapabilityHeader = crate::Reg<
    power_budgeting_enhanced_capability_header::PowerBudgetingEnhancedCapabilityHeaderSpec,
>;
#[doc = "Power Budgeting Enhanced Capability Header Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod power_budgeting_enhanced_capability_header;
#[doc = "POWER_BUDGETING_DATA_SELECT (rw) register accessor: Power Budgeting Data Select Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budgeting_data_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_budgeting_data_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_budgeting_data_select`]
module"]
#[doc(alias = "POWER_BUDGETING_DATA_SELECT")]
pub type PowerBudgetingDataSelect =
    crate::Reg<power_budgeting_data_select::PowerBudgetingDataSelectSpec>;
#[doc = "Power Budgeting Data Select Register (no description)"]
pub mod power_budgeting_data_select;
#[doc = "POWER_BUDGETING_DATA (r) register accessor: Power Budgeting Data Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budgeting_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_budgeting_data`]
module"]
#[doc(alias = "POWER_BUDGETING_DATA")]
pub type PowerBudgetingData = crate::Reg<power_budgeting_data::PowerBudgetingDataSpec>;
#[doc = "Power Budgeting Data Register Reserved"]
pub mod power_budgeting_data;
#[doc = "POWER_BUDGET_CAPABILITY (r) register accessor: Power Budget Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_budget_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_budget_capability`]
module"]
#[doc(alias = "POWER_BUDGET_CAPABILITY")]
pub type PowerBudgetCapability = crate::Reg<power_budget_capability::PowerBudgetCapabilitySpec>;
#[doc = "Power Budget Capability Register Reserved"]
pub mod power_budget_capability;
#[doc = "RESIZABLE_BAR_EXTENDED_CAPABILITY_HEADER (r) register accessor: Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_extended_capability_header`]
module"]
#[doc(alias = "RESIZABLE_BAR_EXTENDED_CAPABILITY_HEADER")]
pub type ResizableBarExtendedCapabilityHeader =
    crate::Reg<resizable_bar_extended_capability_header::ResizableBarExtendedCapabilityHeaderSpec>;
#[doc = "Resizable BAR Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod resizable_bar_extended_capability_header;
#[doc = "RESIZABLE_BAR_CAPABILITY_0 (r) register accessor: Resizable BAR Capability Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_0`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_0")]
pub type ResizableBarCapability0 =
    crate::Reg<resizable_bar_capability_0::ResizableBarCapability0Spec>;
#[doc = "Resizable BAR Capability Register 0 Reserved"]
pub mod resizable_bar_capability_0;
#[doc = "RESIZABLE_BAR_CONTROL_0 (r) register accessor: Resizable BAR Control Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_0`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_0")]
pub type ResizableBarControl0 = crate::Reg<resizable_bar_control_0::ResizableBarControl0Spec>;
#[doc = "Resizable BAR Control Register 0 Reserved"]
pub mod resizable_bar_control_0;
#[doc = "RESIZABLE_BAR_CAPABILITY_1 (r) register accessor: Resizable BAR Capability Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_1`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_1")]
pub type ResizableBarCapability1 =
    crate::Reg<resizable_bar_capability_1::ResizableBarCapability1Spec>;
#[doc = "Resizable BAR Capability Register 1 Reserved"]
pub mod resizable_bar_capability_1;
#[doc = "RESIZABLE_BAR_CONTROL_1 (r) register accessor: Resizable BAR Control Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_1`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_1")]
pub type ResizableBarControl1 = crate::Reg<resizable_bar_control_1::ResizableBarControl1Spec>;
#[doc = "Resizable BAR Control Register 1 Reserved"]
pub mod resizable_bar_control_1;
#[doc = "RESIZABLE_BAR_CAPABILITY_2 (r) register accessor: Resizable BAR Capability Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_2`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_2")]
pub type ResizableBarCapability2 =
    crate::Reg<resizable_bar_capability_2::ResizableBarCapability2Spec>;
#[doc = "Resizable BAR Capability Register 2 Reserved"]
pub mod resizable_bar_capability_2;
#[doc = "RESIZABLE_BAR_CONTROL_2 (r) register accessor: Resizable BAR Control Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_2`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_2")]
pub type ResizableBarControl2 = crate::Reg<resizable_bar_control_2::ResizableBarControl2Spec>;
#[doc = "Resizable BAR Control Register 2 Reserved"]
pub mod resizable_bar_control_2;
#[doc = "RESIZABLE_BAR_CAPABILITY_3 (r) register accessor: Resizable BAR Capability Register 3 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_3`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_3")]
pub type ResizableBarCapability3 =
    crate::Reg<resizable_bar_capability_3::ResizableBarCapability3Spec>;
#[doc = "Resizable BAR Capability Register 3 Reserved"]
pub mod resizable_bar_capability_3;
#[doc = "RESIZABLE_BAR_CONTROL_3 (r) register accessor: Resizable BAR Control Register 3 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_3`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_3")]
pub type ResizableBarControl3 = crate::Reg<resizable_bar_control_3::ResizableBarControl3Spec>;
#[doc = "Resizable BAR Control Register 3 Reserved"]
pub mod resizable_bar_control_3;
#[doc = "RESIZABLE_BAR_CAPABILITY_4 (r) register accessor: Resizable BAR Capability Register 4 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_4`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_4")]
pub type ResizableBarCapability4 =
    crate::Reg<resizable_bar_capability_4::ResizableBarCapability4Spec>;
#[doc = "Resizable BAR Capability Register 4 Reserved"]
pub mod resizable_bar_capability_4;
#[doc = "RESIZABLE_BAR_CONTROL_4 (r) register accessor: Resizable BAR Control Register 4 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_4`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_4")]
pub type ResizableBarControl4 = crate::Reg<resizable_bar_control_4::ResizableBarControl4Spec>;
#[doc = "Resizable BAR Control Register 4 Reserved"]
pub mod resizable_bar_control_4;
#[doc = "RESIZABLE_BAR_CAPABILITY_5 (r) register accessor: Resizable BAR Capability Register 5 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_capability_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_capability_5`]
module"]
#[doc(alias = "RESIZABLE_BAR_CAPABILITY_5")]
pub type ResizableBarCapability5 =
    crate::Reg<resizable_bar_capability_5::ResizableBarCapability5Spec>;
#[doc = "Resizable BAR Capability Register 5 Reserved"]
pub mod resizable_bar_capability_5;
#[doc = "RESIZABLE_BAR_CONTROL_5 (r) register accessor: Resizable BAR Control Register 5 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resizable_bar_control_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resizable_bar_control_5`]
module"]
#[doc(alias = "RESIZABLE_BAR_CONTROL_5")]
pub type ResizableBarControl5 = crate::Reg<resizable_bar_control_5::ResizableBarControl5Spec>;
#[doc = "Resizable BAR Control Register 5 Reserved"]
pub mod resizable_bar_control_5;
#[doc = "LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER (r) register accessor: Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`latency_tolerance_reporting_ltr_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@latency_tolerance_reporting_ltr_extended_capability_header`]
module"]
#[doc(alias = "LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER")]
pub type LatencyToleranceReportingLtrExtendedCapabilityHeader = crate :: Reg < latency_tolerance_reporting_ltr_extended_capability_header :: LatencyToleranceReportingLtrExtendedCapabilityHeaderSpec > ;
#[doc = "Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod latency_tolerance_reporting_ltr_extended_capability_header;
#[doc = "LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY (rw) register accessor: LTR Max Snoop/Max No-Snoop Latency Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr_max_snoop_max_no_snoop_latency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr_max_snoop_max_no_snoop_latency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr_max_snoop_max_no_snoop_latency`]
module"]
#[doc(alias = "LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY")]
pub type LtrMaxSnoopMaxNoSnoopLatency =
    crate::Reg<ltr_max_snoop_max_no_snoop_latency::LtrMaxSnoopMaxNoSnoopLatencySpec>;
#[doc = "LTR Max Snoop/Max No-Snoop Latency Register Reserved"]
pub mod ltr_max_snoop_max_no_snoop_latency;
#[doc = "DPA_EXTENDED_CAPABILITY_HEADER (r) register accessor: DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_extended_capability_header`]
module"]
#[doc(alias = "DPA_EXTENDED_CAPABILITY_HEADER")]
pub type DpaExtendedCapabilityHeader =
    crate::Reg<dpa_extended_capability_header::DpaExtendedCapabilityHeaderSpec>;
#[doc = "DPA Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod dpa_extended_capability_header;
#[doc = "DPA_CAPABILITY (r) register accessor: DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_capability`]
module"]
#[doc(alias = "DPA_CAPABILITY")]
pub type DpaCapability = crate::Reg<dpa_capability::DpaCapabilitySpec>;
#[doc = "DPA Capability Register Specifies the second of the two transition latency values for the substates. The unit of latency is specified by the Transition Latency Unit field of this register."]
pub mod dpa_capability;
#[doc = "DPA_LATENCY_INDICATOR (r) register accessor: DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_latency_indicator::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_latency_indicator`]
module"]
#[doc(alias = "DPA_LATENCY_INDICATOR")]
pub type DpaLatencyIndicator = crate::Reg<dpa_latency_indicator::DpaLatencyIndicatorSpec>;
#[doc = "DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
pub mod dpa_latency_indicator;
#[doc = "DPA_CONTROL_AND_STATUS_S (rw) register accessor: DPA Control and Status Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_control_and_status_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpa_control_and_status_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_control_and_status_s`]
module"]
#[doc(alias = "DPA_CONTROL_AND_STATUS_S")]
pub type DpaControlAndStatusS = crate::Reg<dpa_control_and_status_s::DpaControlAndStatusSSpec>;
#[doc = "DPA Control and Status Registers Reserved"]
pub mod dpa_control_and_status_s;
#[doc = "DYNAMIC_POWER_ALLOCATION_ARRAY_0 (r) register accessor: Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dynamic_power_allocation_array_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dynamic_power_allocation_array_0`]
module"]
#[doc(alias = "DYNAMIC_POWER_ALLOCATION_ARRAY_0")]
pub type DynamicPowerAllocationArray0 =
    crate::Reg<dynamic_power_allocation_array_0::DynamicPowerAllocationArray0Spec>;
#[doc = "Dynamic Power Allocation Array Register 0 This field contains the power allocation for the DPA substate #3 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub mod dynamic_power_allocation_array_0;
#[doc = "DYNAMIC_POWER_ALLOCATION_ARRAY_1 (r) register accessor: Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dynamic_power_allocation_array_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dynamic_power_allocation_array_1`]
module"]
#[doc(alias = "DYNAMIC_POWER_ALLOCATION_ARRAY_1")]
pub type DynamicPowerAllocationArray1 =
    crate::Reg<dynamic_power_allocation_array_1::DynamicPowerAllocationArray1Spec>;
#[doc = "Dynamic Power Allocation Array Register 1 This field contains the power allocation for the DPA substate #7 covered by this register. This value, when multiplied by the Power Allocation Scale programmed in the DPA Capability Register, provides the power associated with the corresponding substate in Watts."]
pub mod dynamic_power_allocation_array_1;
#[doc = "SR_IOV_EXTENDED_CAPABILITY_HEADER (r) register accessor: SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_iov_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_iov_extended_capability_header`]
module"]
#[doc(alias = "SR_IOV_EXTENDED_CAPABILITY_HEADER")]
pub type SrIovExtendedCapabilityHeader =
    crate::Reg<sr_iov_extended_capability_header::SrIovExtendedCapabilityHeaderSpec>;
#[doc = "SR-IOV Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod sr_iov_extended_capability_header;
#[doc = "SR_IOV_CAPABILITIES (r) register accessor: SR-IOV Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_iov_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_iov_capabilities`]
module"]
#[doc(alias = "SR_IOV_CAPABILITIES")]
pub type SrIovCapabilities = crate::Reg<sr_iov_capabilities::SrIovCapabilitiesSpec>;
#[doc = "SR-IOV Capabilities Register Reserved"]
pub mod sr_iov_capabilities;
#[doc = "SR_IOV_CONTROL_AND_STATUS_S (rw) register accessor: SR-IOV Control and Status Registers Not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_iov_control_and_status_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_iov_control_and_status_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr_iov_control_and_status_s`]
module"]
#[doc(alias = "SR_IOV_CONTROL_AND_STATUS_S")]
pub type SrIovControlAndStatusS =
    crate::Reg<sr_iov_control_and_status_s::SrIovControlAndStatusSSpec>;
#[doc = "SR-IOV Control and Status Registers Not implemented."]
pub mod sr_iov_control_and_status_s;
#[doc = "INITIAL_VFS_TOTAL_VFS (r) register accessor: Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`initial_vfs_total_vfs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initial_vfs_total_vfs`]
module"]
#[doc(alias = "INITIAL_VFS_TOTAL_VFS")]
pub type InitialVfsTotalVfs = crate::Reg<initial_vfs_total_vfs::InitialVfsTotalVfsSpec>;
#[doc = "Initial VFs/Total VFs Register This field contains the total number of VFs per PF. Its default setting is identical to that of InitialVFs. This field can be modified using local management registers."]
pub mod initial_vfs_total_vfs;
#[doc = "FUNCTION_DEPENDENCY_LINK_NUMVFS (rw) register accessor: Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function_dependency_link_numvfs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function_dependency_link_numvfs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function_dependency_link_numvfs`]
module"]
#[doc(alias = "FUNCTION_DEPENDENCY_LINK_NUMVFS")]
pub type FunctionDependencyLinkNumvfs =
    crate::Reg<function_dependency_link_numvfs::FunctionDependencyLinkNumvfsSpec>;
#[doc = "Function Dependency Link/NumVFs Register This field is used to specify dependencies between PFs. It can be modified independently for each Function from the local management bus."]
pub mod function_dependency_link_numvfs;
#[doc = "VF_OFFSET_STRIDE (r) register accessor: VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_offset_stride::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_offset_stride`]
module"]
#[doc(alias = "VF_OFFSET_STRIDE")]
pub type VfOffsetStride = crate::Reg<vf_offset_stride::VfOffsetStrideSpec>;
#[doc = "VF Offset/Stride Register Stride value used to assign RIDs for VFs. The stride value is hardwired to 1 for all Physical Functions."]
pub mod vf_offset_stride;
#[doc = "VF_DEVICE_ID (r) register accessor: VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_device_id`]
module"]
#[doc(alias = "VF_DEVICE_ID")]
pub type VfDeviceId = crate::Reg<vf_device_id::VfDeviceIdSpec>;
#[doc = "VF Device ID Register VF device id assigned to the device. Its default value is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
pub mod vf_device_id;
#[doc = "SUPPORTED_PAGE_SIZES (r) register accessor: Supported Page Sizes Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`supported_page_sizes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supported_page_sizes`]
module"]
#[doc(alias = "SUPPORTED_PAGE_SIZES")]
pub type SupportedPageSizes = crate::Reg<supported_page_sizes::SupportedPageSizesSpec>;
#[doc = "Supported Page Sizes Register Reserved"]
pub mod supported_page_sizes;
#[doc = "SYSTEM_PAGE_SIZE (rw) register accessor: System Page Size Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_page_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_page_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_page_size`]
module"]
#[doc(alias = "SYSTEM_PAGE_SIZE")]
pub type SystemPageSize = crate::Reg<system_page_size::SystemPageSizeSpec>;
#[doc = "System Page Size Register Reserved"]
pub mod system_page_size;
#[doc = "VF_BASE_ADDRESS_0 (rw) register accessor: VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_0`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_0")]
pub type VfBaseAddress0 = crate::Reg<vf_base_address_0::VfBaseAddress0Spec>;
#[doc = "VF Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod vf_base_address_0;
#[doc = "VF_BASE_ADDRESS_1 (rw) register accessor: VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_1`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_1")]
pub type VfBaseAddress1 = crate::Reg<vf_base_address_1::VfBaseAddress1Spec>;
#[doc = "VF Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod vf_base_address_1;
#[doc = "VF_BASE_ADDRESS_2 (rw) register accessor: VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_2`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_2")]
pub type VfBaseAddress2 = crate::Reg<vf_base_address_2::VfBaseAddress2Spec>;
#[doc = "VF Base Address Register 2 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod vf_base_address_2;
#[doc = "VF_BASE_ADDRESS_3 (rw) register accessor: VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_3`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_3")]
pub type VfBaseAddress3 = crate::Reg<vf_base_address_3::VfBaseAddress3Spec>;
#[doc = "VF Base Address Register 3 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod vf_base_address_3;
#[doc = "VF_BASE_ADDRESS_4 (rw) register accessor: VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_4`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_4")]
pub type VfBaseAddress4 = crate::Reg<vf_base_address_4::VfBaseAddress4Spec>;
#[doc = "VF Base Address Register 4 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in BAR Configuration Registers of the associated Physical Function."]
pub mod vf_base_address_4;
#[doc = "VF_BASE_ADDRESS_5 (rw) register accessor: VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_base_address_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vf_base_address_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_base_address_5`]
module"]
#[doc(alias = "VF_BASE_ADDRESS_5")]
pub type VfBaseAddress5 = crate::Reg<vf_base_address_5::VfBaseAddress5Spec>;
#[doc = "VF Base Address Register 5 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture setting of BAR Configuration Registers of the associated Physical Function. All other bits are not writeable, and are read as 0's."]
pub mod vf_base_address_5;
#[doc = "VF_MIGRATION_STATE_ARRAY_OFFSET (r) register accessor: VF Migration State Array Offset Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_migration_state_array_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vf_migration_state_array_offset`]
module"]
#[doc(alias = "VF_MIGRATION_STATE_ARRAY_OFFSET")]
pub type VfMigrationStateArrayOffset =
    crate::Reg<vf_migration_state_array_offset::VfMigrationStateArrayOffsetSpec>;
#[doc = "VF Migration State Array Offset Register (no description)"]
pub mod vf_migration_state_array_offset;
#[doc = "TPH_REQUESTER_EXTENDED_CAPABILITY_HEADER (r) register accessor: TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_requester_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_requester_extended_capability_header`]
module"]
#[doc(alias = "TPH_REQUESTER_EXTENDED_CAPABILITY_HEADER")]
pub type TphRequesterExtendedCapabilityHeader =
    crate::Reg<tph_requester_extended_capability_header::TphRequesterExtendedCapabilityHeaderSpec>;
#[doc = "TPH Requester Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod tph_requester_extended_capability_header;
#[doc = "TPH_REQUESTER_CAPABILITY (r) register accessor: TPH Requester Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_requester_capability::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_requester_capability`]
module"]
#[doc(alias = "TPH_REQUESTER_CAPABILITY")]
pub type TphRequesterCapability = crate::Reg<tph_requester_capability::TphRequesterCapabilitySpec>;
#[doc = "TPH Requester Capability Register Reserved"]
pub mod tph_requester_capability;
#[doc = "TPH_REQUESTER_CONTROL (rw) register accessor: TPH Requester Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_requester_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_requester_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_requester_control`]
module"]
#[doc(alias = "TPH_REQUESTER_CONTROL")]
pub type TphRequesterControl = crate::Reg<tph_requester_control::TphRequesterControlSpec>;
#[doc = "TPH Requester Control Register Reserved"]
pub mod tph_requester_control;
#[doc = "TPH_ST_TABLE_0 (rw) register accessor: TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_st_table_0`]
module"]
#[doc(alias = "TPH_ST_TABLE_0")]
pub type TphStTable0 = crate::Reg<tph_st_table_0::TphStTable0Spec>;
#[doc = "TPH ST Table 0 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod tph_st_table_0;
#[doc = "TPH_ST_TABLE_1 (rw) register accessor: TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_st_table_1`]
module"]
#[doc(alias = "TPH_ST_TABLE_1")]
pub type TphStTable1 = crate::Reg<tph_st_table_1::TphStTable1Spec>;
#[doc = "TPH ST Table 1 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod tph_st_table_1;
#[doc = "TPH_ST_TABLE_2 (rw) register accessor: TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_st_table_2`]
module"]
#[doc(alias = "TPH_ST_TABLE_2")]
pub type TphStTable2 = crate::Reg<tph_st_table_2::TphStTable2Spec>;
#[doc = "TPH ST Table 2 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod tph_st_table_2;
#[doc = "TPH_ST_TABLE_3 (rw) register accessor: TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_st_table_3`]
module"]
#[doc(alias = "TPH_ST_TABLE_3")]
pub type TphStTable3 = crate::Reg<tph_st_table_3::TphStTable3Spec>;
#[doc = "TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod tph_st_table_3;
#[doc = "L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER (r) register accessor: L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_pm_substates_extended_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_pm_substates_extended_capability_header`]
module"]
#[doc(alias = "L1_PM_SUBSTATES_EXTENDED_CAPABILITY_HEADER")]
pub type L1PmSubstatesExtendedCapabilityHeader = crate::Reg<
    l1_pm_substates_extended_capability_header::L1PmSubstatesExtendedCapabilityHeaderSpec,
>;
#[doc = "L1 PM Substates Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod l1_pm_substates_extended_capability_header;
#[doc = "L1_PM_SUBSTATES_CAPABILITIES (r) register accessor: L1 PM Substates Capabilities Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_pm_substates_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_pm_substates_capabilities`]
module"]
#[doc(alias = "L1_PM_SUBSTATES_CAPABILITIES")]
pub type L1PmSubstatesCapabilities =
    crate::Reg<l1_pm_substates_capabilities::L1PmSubstatesCapabilitiesSpec>;
#[doc = "L1 PM Substates Capabilities Register RSVD"]
pub mod l1_pm_substates_capabilities;
#[doc = "L1_PM_SUBSTATES_CONTROL_1 (rw) register accessor: L1 PM Substates Control 1 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_pm_substates_control_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_pm_substates_control_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_pm_substates_control_1`]
module"]
#[doc(alias = "L1_PM_SUBSTATES_CONTROL_1")]
pub type L1PmSubstatesControl1 = crate::Reg<l1_pm_substates_control_1::L1PmSubstatesControl1Spec>;
#[doc = "L1 PM Substates Control 1 Register (no description)"]
pub mod l1_pm_substates_control_1;
#[doc = "L1_PM_SUBSTATES_CONTROL_2 (rw) register accessor: L1 PM Substates Control 2 Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_pm_substates_control_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_pm_substates_control_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_pm_substates_control_2`]
module"]
#[doc(alias = "L1_PM_SUBSTATES_CONTROL_2")]
pub type L1PmSubstatesControl2 = crate::Reg<l1_pm_substates_control_2::L1PmSubstatesControl2Spec>;
#[doc = "L1 PM Substates Control 2 Register RSVD"]
pub mod l1_pm_substates_control_2;
