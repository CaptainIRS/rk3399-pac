#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vendor_id_and_device_id: VendorIdAndDeviceId,
    command_and_status: CommandAndStatus,
    revision_id_and_class_code: RevisionIdAndClassCode,
    bist_header_type_latency_timer_and_cache_line_size_s:
        BistHeaderTypeLatencyTimerAndCacheLineSizeS,
    root_complex_base_address_0: RootComplexBaseAddress0,
    root_complex_base_address_1: RootComplexBaseAddress1,
    primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer:
        PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer,
    io_base_io_limit_secondary_status: IoBaseIoLimitSecondaryStatus,
    memory_base_memory_limit: MemoryBaseMemoryLimit,
    prefetchable_memory_base_prefetchable_memory_limit:
        PrefetchableMemoryBasePrefetchableMemoryLimit,
    prefetchable_base_upper: PrefetchableBaseUpper,
    prefetchable_limit_upper: PrefetchableLimitUpper,
    io_base_upper_io_limit_upper: IoBaseUpperIoLimitUpper,
    capabilities_pointer: CapabilitiesPointer,
    _reserved14: [u8; 0x04],
    interrupt_line_interrupt_pin_and_bridge_control: InterruptLineInterruptPinAndBridgeControl,
    _reserved15: [u8; 0x40],
    power_management_capabilities: PowerManagementCapabilities,
    power_management_control_status_report: PowerManagementControlStatusReport,
    _reserved17: [u8; 0x08],
    msi_control: MsiControl,
    msi_message_low_address: MsiMessageLowAddress,
    msi_message_high_address: MsiMessageHighAddress,
    msi_message_data: MsiMessageData,
    msi_mask: MsiMask,
    msi_pending_bits: MsiPendingBits,
    _reserved23: [u8; 0x08],
    msi_x_control: MsiXControl,
    msi_x_table_offset: MsiXTableOffset,
    msi_x_pending_interrupt: MsiXPendingInterrupt,
    _reserved26: [u8; 0x04],
    pci_express_capability_list: PciExpressCapabilityList,
    pci_express_device_capabilities: PciExpressDeviceCapabilities,
    pci_express_device_control_and_status: PciExpressDeviceControlAndStatus,
    link_capabilities: LinkCapabilities,
    link_control_and_status: LinkControlAndStatus,
    slot_capability: SlotCapability,
    slot_control_and_status: SlotControlAndStatus,
    root_control_and_capability: RootControlAndCapability,
    root_status: RootStatus,
    pci_express_device_capabilities_2: PciExpressDeviceCapabilities2,
    pci_express_device_control_and_status_2: PciExpressDeviceControlAndStatus2,
    link_capabilities_2: LinkCapabilities2,
    link_control_and_status_2: LinkControlAndStatus2,
    _reserved39: [u8; 0x0c],
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
    root_error_command: RootErrorCommand,
    root_error_status: RootErrorStatus,
    error_source_identification: ErrorSourceIdentification,
    _reserved53: [u8; 0x0148],
    tph_st_table_3: TphStTable3,
    _reserved54: [u8; 0x067c],
    l1_pm_substates_extended_capability_header: L1PmSubstatesExtendedCapabilityHeader,
    l1_pm_substates_capabilities: L1PmSubstatesCapabilities,
    l1_pm_substates_control_1: L1PmSubstatesControl1,
    l1_pm_substates_control_2: L1PmSubstatesControl2,
}
impl RegisterBlock {
    #[doc = "0x00 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn vendor_id_and_device_id(&self) -> &VendorIdAndDeviceId {
        &self.vendor_id_and_device_id
    }
    #[doc = "0x04 - Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn command_and_status(&self) -> &CommandAndStatus {
        &self.command_and_status
    }
    #[doc = "0x08 - Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn revision_id_and_class_code(&self) -> &RevisionIdAndClassCode {
        &self.revision_id_and_class_code
    }
    #[doc = "0x0c - BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &BistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x10 - Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn root_complex_base_address_0(&self) -> &RootComplexBaseAddress0 {
        &self.root_complex_base_address_0
    }
    #[doc = "0x14 - Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
    #[inline(always)]
    pub const fn root_complex_base_address_1(&self) -> &RootComplexBaseAddress1 {
        &self.root_complex_base_address_1
    }
    #[doc = "0x18 - Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented."]
    #[inline(always)]
    pub const fn primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer(
        &self,
    ) -> &PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer {
        &self.primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer
    }
    #[doc = "0x1c - IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn io_base_io_limit_secondary_status(&self) -> &IoBaseIoLimitSecondaryStatus {
        &self.io_base_io_limit_secondary_status
    }
    #[doc = "0x20 - Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub const fn memory_base_memory_limit(&self) -> &MemoryBaseMemoryLimit {
        &self.memory_base_memory_limit
    }
    #[doc = "0x24 - Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn prefetchable_memory_base_prefetchable_memory_limit(
        &self,
    ) -> &PrefetchableMemoryBasePrefetchableMemoryLimit {
        &self.prefetchable_memory_base_prefetchable_memory_limit
    }
    #[doc = "0x28 - Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn prefetchable_base_upper(&self) -> &PrefetchableBaseUpper {
        &self.prefetchable_base_upper
    }
    #[doc = "0x2c - Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn prefetchable_limit_upper(&self) -> &PrefetchableLimitUpper {
        &self.prefetchable_limit_upper
    }
    #[doc = "0x30 - IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub const fn io_base_upper_io_limit_upper(&self) -> &IoBaseUpperIoLimitUpper {
        &self.io_base_upper_io_limit_upper
    }
    #[doc = "0x34 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn capabilities_pointer(&self) -> &CapabilitiesPointer {
        &self.capabilities_pointer
    }
    #[doc = "0x3c - Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved"]
    #[inline(always)]
    pub const fn interrupt_line_interrupt_pin_and_bridge_control(
        &self,
    ) -> &InterruptLineInterruptPinAndBridgeControl {
        &self.interrupt_line_interrupt_pin_and_bridge_control
    }
    #[doc = "0x80 - Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
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
    #[doc = "0xa0 - MSI Mask Register RSVD"]
    #[inline(always)]
    pub const fn msi_mask(&self) -> &MsiMask {
        &self.msi_mask
    }
    #[doc = "0xa4 - MSI Pending Bits Register RSVD"]
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
    #[doc = "0xc8 - PCI Express Device Control and Status Register (no description)"]
    #[inline(always)]
    pub const fn pci_express_device_control_and_status(&self) -> &PciExpressDeviceControlAndStatus {
        &self.pci_express_device_control_and_status
    }
    #[doc = "0xcc - Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub const fn link_capabilities(&self) -> &LinkCapabilities {
        &self.link_capabilities
    }
    #[doc = "0xd0 - Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub const fn link_control_and_status(&self) -> &LinkControlAndStatus {
        &self.link_control_and_status
    }
    #[doc = "0xd4 - Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
    #[inline(always)]
    pub const fn slot_capability(&self) -> &SlotCapability {
        &self.slot_capability
    }
    #[doc = "0xd8 - Slot Control and Status Register (no description)"]
    #[inline(always)]
    pub const fn slot_control_and_status(&self) -> &SlotControlAndStatus {
        &self.slot_control_and_status
    }
    #[doc = "0xdc - Root Control and Capability Register Reserved"]
    #[inline(always)]
    pub const fn root_control_and_capability(&self) -> &RootControlAndCapability {
        &self.root_control_and_capability
    }
    #[doc = "0xe0 - Root Status Register Reserved"]
    #[inline(always)]
    pub const fn root_status(&self) -> &RootStatus {
        &self.root_status
    }
    #[doc = "0xe4 - PCI Express Device Capabilities 2 Register Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_capabilities_2(&self) -> &PciExpressDeviceCapabilities2 {
        &self.pci_express_device_capabilities_2
    }
    #[doc = "0xe8 - PCI Express Device Control and Status 2 Register (no description)"]
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
    #[doc = "0xf0 - Link Control and Status 2 Register Reserved"]
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
    #[doc = "0x104 - Uncorrectable Error Status Register Reserved"]
    #[inline(always)]
    pub const fn uncorrectable_error_status(&self) -> &UncorrectableErrorStatus {
        &self.uncorrectable_error_status
    }
    #[doc = "0x108 - Uncorrectable Error Mask Register Reserved"]
    #[inline(always)]
    pub const fn uncorrectable_error_mask(&self) -> &UncorrectableErrorMask {
        &self.uncorrectable_error_mask
    }
    #[doc = "0x10c - Uncorrectable Error Severity Register (no description)"]
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
    #[doc = "0x11c - Header Log Register 0 First Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn header_log_0(&self) -> &HeaderLog0 {
        &self.header_log_0
    }
    #[doc = "0x120 - Header Log Register 1 Second Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn header_log_1(&self) -> &HeaderLog1 {
        &self.header_log_1
    }
    #[doc = "0x124 - Header Log Register 2 Third Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn header_log_2(&self) -> &HeaderLog2 {
        &self.header_log_2
    }
    #[doc = "0x128 - Header Log Register 3 Fourth Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub const fn header_log_3(&self) -> &HeaderLog3 {
        &self.header_log_3
    }
    #[doc = "0x12c - Root Error Command Register Reserved"]
    #[inline(always)]
    pub const fn root_error_command(&self) -> &RootErrorCommand {
        &self.root_error_command
    }
    #[doc = "0x130 - Root Error Status Register Reserved"]
    #[inline(always)]
    pub const fn root_error_status(&self) -> &RootErrorStatus {
        &self.root_error_status
    }
    #[doc = "0x134 - Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
    #[inline(always)]
    pub const fn error_source_identification(&self) -> &ErrorSourceIdentification {
        &self.error_source_identification
    }
    #[doc = "0x280 - TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
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
#[doc = "TPH_ST_TABLE_3 (rw) register accessor: TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_st_table_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tph_st_table_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_st_table_3`]
module"]
#[doc(alias = "TPH_ST_TABLE_3")]
pub type TphStTable3 = crate::Reg<tph_st_table_3::TphStTable3Spec>;
#[doc = "TPH ST Table 3 This field is used for the upper 8 bits of the second Steering Tag when Extended TPH Requester support is enabled."]
pub mod tph_st_table_3;
#[doc = "VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_id_and_device_id`]
module"]
#[doc(alias = "VENDOR_ID_AND_DEVICE_ID")]
pub type VendorIdAndDeviceId = crate::Reg<vendor_id_and_device_id::VendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod vendor_id_and_device_id;
#[doc = "COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command_and_status`]
module"]
#[doc(alias = "COMMAND_AND_STATUS")]
pub type CommandAndStatus = crate::Reg<command_and_status::CommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod command_and_status;
#[doc = "REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision_id_and_class_code`]
module"]
#[doc(alias = "REVISION_ID_AND_CLASS_CODE")]
pub type RevisionIdAndClassCode =
    crate::Reg<revision_id_and_class_code::RevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. On power- up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod revision_id_and_class_code;
#[doc = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (rw) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_header_type_latency_timer_and_cache_line_size_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_header_type_latency_timer_and_cache_line_size_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type BistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < bist_header_type_latency_timer_and_cache_line_size_s :: BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "ROOT_COMPLEX_BASE_ADDRESS_0 (rw) register accessor: Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_complex_base_address_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_complex_base_address_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_complex_base_address_0`]
module"]
#[doc(alias = "ROOT_COMPLEX_BASE_ADDRESS_0")]
pub type RootComplexBaseAddress0 =
    crate::Reg<root_complex_base_address_0::RootComplexBaseAddress0Spec>;
#[doc = "Root Complex Base Address Register 0 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub mod root_complex_base_address_0;
#[doc = "ROOT_COMPLEX_BASE_ADDRESS_1 (rw) register accessor: Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_complex_base_address_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_complex_base_address_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_complex_base_address_1`]
module"]
#[doc(alias = "ROOT_COMPLEX_BASE_ADDRESS_1")]
pub type RootComplexBaseAddress1 =
    crate::Reg<root_complex_base_address_1::RootComplexBaseAddress1Spec>;
#[doc = "Root Complex Base Address Register 1 This field defines the base address of the memory address range. The number of implemented bits in this field determines the BAR aperture configured in Root Complex BAR Configuration Register. All other bits are not writeable, and are read as 0's."]
pub mod root_complex_base_address_1;
#[doc = "PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER (rw) register accessor: Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer`]
module"]
#[doc(
    alias = "PRIMARY_BUS_NUMBER_SECONDARY_BUS_NUMBER_SUBORDINATE_BUS_NUMBER_SECONDARY_LATENCY_TIMER"
)]
pub type PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimer = crate :: Reg < primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer :: PrimaryBusNumberSecondaryBusNumberSubordinateBusNumberSecondaryLatencyTimerSpec > ;
#[doc = "Primary Bus Number, Secondary Bus Number, Subordinate Bus Number, Secondary Latency Timer This field is not implemented."]
pub mod primary_bus_number_secondary_bus_number_subordinate_bus_number_secondary_latency_timer;
#[doc = "IO_BASE_IO_LIMIT_SECONDARY_STATUS (rw) register accessor: IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_base_io_limit_secondary_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_base_io_limit_secondary_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_base_io_limit_secondary_status`]
module"]
#[doc(alias = "IO_BASE_IO_LIMIT_SECONDARY_STATUS")]
pub type IoBaseIoLimitSecondaryStatus =
    crate::Reg<io_base_io_limit_secondary_status::IoBaseIoLimitSecondaryStatusSpec>;
#[doc = "IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod io_base_io_limit_secondary_status;
#[doc = "MEMORY_BASE_MEMORY_LIMIT (rw) register accessor: Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memory_base_memory_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memory_base_memory_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memory_base_memory_limit`]
module"]
#[doc(alias = "MEMORY_BASE_MEMORY_LIMIT")]
pub type MemoryBaseMemoryLimit = crate::Reg<memory_base_memory_limit::MemoryBaseMemoryLimitSpec>;
#[doc = "Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub mod memory_base_memory_limit;
#[doc = "PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT (r) register accessor: Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_memory_base_prefetchable_memory_limit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefetchable_memory_base_prefetchable_memory_limit`]
module"]
#[doc(alias = "PREFETCHABLE_MEMORY_BASE_PREFETCHABLE_MEMORY_LIMIT")]
pub type PrefetchableMemoryBasePrefetchableMemoryLimit = crate :: Reg < prefetchable_memory_base_prefetchable_memory_limit :: PrefetchableMemoryBasePrefetchableMemoryLimitSpec > ;
#[doc = "Prefetchable Memory Base, Prefetchable Memory Limit This field can be read and written from the local management APB bus if prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod prefetchable_memory_base_prefetchable_memory_limit;
#[doc = "PREFETCHABLE_BASE_UPPER (r) register accessor: Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_base_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefetchable_base_upper`]
module"]
#[doc(alias = "PREFETCHABLE_BASE_UPPER")]
pub type PrefetchableBaseUpper = crate::Reg<prefetchable_base_upper::PrefetchableBaseUpperSpec>;
#[doc = "Prefetchable Base Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod prefetchable_base_upper;
#[doc = "PREFETCHABLE_LIMIT_UPPER (r) register accessor: Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefetchable_limit_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefetchable_limit_upper`]
module"]
#[doc(alias = "PREFETCHABLE_LIMIT_UPPER")]
pub type PrefetchableLimitUpper = crate::Reg<prefetchable_limit_upper::PrefetchableLimitUpperSpec>;
#[doc = "Prefetchable Limit Upper This field can be read and written from the local management APB bus if 64bit prefetchable memory is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod prefetchable_limit_upper;
#[doc = "IO_BASE_UPPER_IO_LIMIT_UPPER (r) register accessor: IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_base_upper_io_limit_upper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_base_upper_io_limit_upper`]
module"]
#[doc(alias = "IO_BASE_UPPER_IO_LIMIT_UPPER")]
pub type IoBaseUpperIoLimitUpper =
    crate::Reg<io_base_upper_io_limit_upper::IoBaseUpperIoLimitUpperSpec>;
#[doc = "IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub mod io_base_upper_io_limit_upper;
#[doc = "CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities_pointer`]
module"]
#[doc(alias = "CAPABILITIES_POINTER")]
pub type CapabilitiesPointer = crate::Reg<capabilities_pointer::CapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod capabilities_pointer;
#[doc = "INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL (rw) register accessor: Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_line_interrupt_pin_and_bridge_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_line_interrupt_pin_and_bridge_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_line_interrupt_pin_and_bridge_control`]
module"]
#[doc(alias = "INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL")]
pub type InterruptLineInterruptPinAndBridgeControl = crate::Reg<
    interrupt_line_interrupt_pin_and_bridge_control::InterruptLineInterruptPinAndBridgeControlSpec,
>;
#[doc = "Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved"]
pub mod interrupt_line_interrupt_pin_and_bridge_control;
#[doc = "POWER_MANAGEMENT_CAPABILITIES (r) register accessor: Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_management_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_management_capabilities`]
module"]
#[doc(alias = "POWER_MANAGEMENT_CAPABILITIES")]
pub type PowerManagementCapabilities =
    crate::Reg<power_management_capabilities::PowerManagementCapabilitiesSpec>;
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
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
#[doc = "MSI_MASK (rw) register accessor: MSI Mask Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_mask`]
module"]
#[doc(alias = "MSI_MASK")]
pub type MsiMask = crate::Reg<msi_mask::MsiMaskSpec>;
#[doc = "MSI Mask Register RSVD"]
pub mod msi_mask;
#[doc = "MSI_PENDING_BITS (r) register accessor: MSI Pending Bits Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_pending_bits::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msi_pending_bits`]
module"]
#[doc(alias = "MSI_PENDING_BITS")]
pub type MsiPendingBits = crate::Reg<msi_pending_bits::MsiPendingBitsSpec>;
#[doc = "MSI Pending Bits Register RSVD"]
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
#[doc = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS (rw) register accessor: PCI Express Device Control and Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pci_express_device_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_control_and_status`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS")]
pub type PciExpressDeviceControlAndStatus =
    crate::Reg<pci_express_device_control_and_status::PciExpressDeviceControlAndStatusSpec>;
#[doc = "PCI Express Device Control and Status Register (no description)"]
pub mod pci_express_device_control_and_status;
#[doc = "LINK_CAPABILITIES (r) register accessor: Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_capabilities`]
module"]
#[doc(alias = "LINK_CAPABILITIES")]
pub type LinkCapabilities = crate::Reg<link_capabilities::LinkCapabilitiesSpec>;
#[doc = "Link Capabilities Register Specifies the port number assigned to the PCI Express link connected to this device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub mod link_capabilities;
#[doc = "LINK_CONTROL_AND_STATUS (rw) register accessor: Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_control_and_status`]
module"]
#[doc(alias = "LINK_CONTROL_AND_STATUS")]
pub type LinkControlAndStatus = crate::Reg<link_control_and_status::LinkControlAndStatusSpec>;
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub mod link_control_and_status;
#[doc = "SLOT_CAPABILITY (rw) register accessor: Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_capability::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slot_capability::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slot_capability`]
module"]
#[doc(alias = "SLOT_CAPABILITY")]
pub type SlotCapability = crate::Reg<slot_capability::SlotCapabilitySpec>;
#[doc = "Slot Capability Register This field indicates the physical slot number attached to this Port. This field must be hardware initialized to a value that assigns a slot number that is unique within the chassis, regardless of the form factor associated with the slot. This field must be initialized to zero for Ports connected to devices that are either integrated on the system board or integrated within the same silicon as the Switch device or Root Port."]
pub mod slot_capability;
#[doc = "SLOT_CONTROL_AND_STATUS (rw) register accessor: Slot Control and Status Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_control_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slot_control_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slot_control_and_status`]
module"]
#[doc(alias = "SLOT_CONTROL_AND_STATUS")]
pub type SlotControlAndStatus = crate::Reg<slot_control_and_status::SlotControlAndStatusSpec>;
#[doc = "Slot Control and Status Register (no description)"]
pub mod slot_control_and_status;
#[doc = "ROOT_CONTROL_AND_CAPABILITY (rw) register accessor: Root Control and Capability Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_control_and_capability::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_control_and_capability::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_control_and_capability`]
module"]
#[doc(alias = "ROOT_CONTROL_AND_CAPABILITY")]
pub type RootControlAndCapability =
    crate::Reg<root_control_and_capability::RootControlAndCapabilitySpec>;
#[doc = "Root Control and Capability Register Reserved"]
pub mod root_control_and_capability;
#[doc = "ROOT_STATUS (rw) register accessor: Root Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_status`]
module"]
#[doc(alias = "ROOT_STATUS")]
pub type RootStatus = crate::Reg<root_status::RootStatusSpec>;
#[doc = "Root Status Register Reserved"]
pub mod root_status;
#[doc = "PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities 2 Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PciExpressDeviceCapabilities2 =
    crate::Reg<pci_express_device_capabilities_2::PciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities 2 Register Reserved"]
pub mod pci_express_device_capabilities_2;
#[doc = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 (rw) register accessor: PCI Express Device Control and Status 2 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pci_express_device_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_control_and_status_2`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2")]
pub type PciExpressDeviceControlAndStatus2 =
    crate::Reg<pci_express_device_control_and_status_2::PciExpressDeviceControlAndStatus2Spec>;
#[doc = "PCI Express Device Control and Status 2 Register (no description)"]
pub mod pci_express_device_control_and_status_2;
#[doc = "LINK_CAPABILITIES_2 (r) register accessor: Link Capabilities Register 2 RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_capabilities_2`]
module"]
#[doc(alias = "LINK_CAPABILITIES_2")]
pub type LinkCapabilities2 = crate::Reg<link_capabilities_2::LinkCapabilities2Spec>;
#[doc = "Link Capabilities Register 2 RSVD"]
pub mod link_capabilities_2;
#[doc = "LINK_CONTROL_AND_STATUS_2 (rw) register accessor: Link Control and Status 2 Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_control_and_status_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_control_and_status_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link_control_and_status_2`]
module"]
#[doc(alias = "LINK_CONTROL_AND_STATUS_2")]
pub type LinkControlAndStatus2 = crate::Reg<link_control_and_status_2::LinkControlAndStatus2Spec>;
#[doc = "Link Control and Status 2 Register Reserved"]
pub mod link_control_and_status_2;
#[doc = "ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER (r) register accessor: Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced_error_reporting_aer_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advanced_error_reporting_aer_enhanced_capability_header`]
module"]
#[doc(alias = "ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER")]
pub type AdvancedErrorReportingAerEnhancedCapabilityHeader = crate :: Reg < advanced_error_reporting_aer_enhanced_capability_header :: AdvancedErrorReportingAerEnhancedCapabilityHeaderSpec > ;
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod advanced_error_reporting_aer_enhanced_capability_header;
#[doc = "UNCORRECTABLE_ERROR_STATUS (rw) register accessor: Uncorrectable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_status`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_STATUS")]
pub type UncorrectableErrorStatus =
    crate::Reg<uncorrectable_error_status::UncorrectableErrorStatusSpec>;
#[doc = "Uncorrectable Error Status Register Reserved"]
pub mod uncorrectable_error_status;
#[doc = "UNCORRECTABLE_ERROR_MASK (rw) register accessor: Uncorrectable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_mask`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_MASK")]
pub type UncorrectableErrorMask = crate::Reg<uncorrectable_error_mask::UncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register Reserved"]
pub mod uncorrectable_error_mask;
#[doc = "UNCORRECTABLE_ERROR_SEVERITY (rw) register accessor: Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_severity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_severity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_severity`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_SEVERITY")]
pub type UncorrectableErrorSeverity =
    crate::Reg<uncorrectable_error_severity::UncorrectableErrorSeveritySpec>;
#[doc = "Uncorrectable Error Severity Register (no description)"]
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
#[doc = "HEADER_LOG_0 (r) register accessor: Header Log Register 0 First Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_0`]
module"]
#[doc(alias = "HEADER_LOG_0")]
pub type HeaderLog0 = crate::Reg<header_log_0::HeaderLog0Spec>;
#[doc = "Header Log Register 0 First Dword of captured TLP header. STICKY."]
pub mod header_log_0;
#[doc = "HEADER_LOG_1 (r) register accessor: Header Log Register 1 Second Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_1`]
module"]
#[doc(alias = "HEADER_LOG_1")]
pub type HeaderLog1 = crate::Reg<header_log_1::HeaderLog1Spec>;
#[doc = "Header Log Register 1 Second Dword of captured TLP header. STICKY."]
pub mod header_log_1;
#[doc = "HEADER_LOG_2 (r) register accessor: Header Log Register 2 Third Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_2`]
module"]
#[doc(alias = "HEADER_LOG_2")]
pub type HeaderLog2 = crate::Reg<header_log_2::HeaderLog2Spec>;
#[doc = "Header Log Register 2 Third Dword of captured TLP header. STICKY."]
pub mod header_log_2;
#[doc = "HEADER_LOG_3 (r) register accessor: Header Log Register 3 Fourth Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`header_log_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header_log_3`]
module"]
#[doc(alias = "HEADER_LOG_3")]
pub type HeaderLog3 = crate::Reg<header_log_3::HeaderLog3Spec>;
#[doc = "Header Log Register 3 Fourth Dword of captured TLP header. STICKY."]
pub mod header_log_3;
#[doc = "ROOT_ERROR_COMMAND (rw) register accessor: Root Error Command Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_error_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_error_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_error_command`]
module"]
#[doc(alias = "ROOT_ERROR_COMMAND")]
pub type RootErrorCommand = crate::Reg<root_error_command::RootErrorCommandSpec>;
#[doc = "Root Error Command Register Reserved"]
pub mod root_error_command;
#[doc = "ROOT_ERROR_STATUS (rw) register accessor: Root Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_error_status`]
module"]
#[doc(alias = "ROOT_ERROR_STATUS")]
pub type RootErrorStatus = crate::Reg<root_error_status::RootErrorStatusSpec>;
#[doc = "Root Error Status Register Reserved"]
pub mod root_error_status;
#[doc = "ERROR_SOURCE_IDENTIFICATION (r) register accessor: Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_source_identification::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_source_identification`]
module"]
#[doc(alias = "ERROR_SOURCE_IDENTIFICATION")]
pub type ErrorSourceIdentification =
    crate::Reg<error_source_identification::ErrorSourceIdentificationSpec>;
#[doc = "Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
pub mod error_source_identification;
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
