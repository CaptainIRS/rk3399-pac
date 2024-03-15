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
    expansion_rom_base_address: ExpansionRomBaseAddress,
    capabilities_pointer: CapabilitiesPointer,
    _reserved13: [u8; 0x04],
    interrupt_line_and_interrupt_pin: InterruptLineAndInterruptPin,
    _reserved14: [u8; 0x40],
    power_management_capabilities: PowerManagementCapabilities,
    power_management_control_status_report: PowerManagementControlStatusReport,
    _reserved16: [u8; 0x08],
    msi_control: MsiControl,
    msi_message_low_address: MsiMessageLowAddress,
    msi_message_high_address: MsiMessageHighAddress,
    msi_message_data: MsiMessageData,
    msi_mask: MsiMask,
    msi_pending_bits: MsiPendingBits,
    _reserved22: [u8; 0x08],
    msi_x_control: MsiXControl,
    msi_x_table_offset: MsiXTableOffset,
    msi_x_pending_interrupt: MsiXPendingInterrupt,
    _reserved25: [u8; 0x04],
    pci_express_capability_list: PciExpressCapabilityList,
    pci_express_device_capabilities: PciExpressDeviceCapabilities,
    pci_express_device_control_and_status: PciExpressDeviceControlAndStatus,
    link_capabilities: LinkCapabilities,
    _reserved29: [u8; 0x14],
    pci_express_device_capabilities_2: PciExpressDeviceCapabilities2,
    _reserved30: [u8; 0x18],
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
    _reserved41: [u8; 0x14],
    ari_extended_capability_header: AriExtendedCapabilityHeader,
    ari_capability_and_ari_control: AriCapabilityAndAriControl,
    _reserved43: [u8; 0x012c],
    tph_requester_enhanced_capability_header: TphRequesterEnhancedCapabilityHeader,
    tph_requester_capability: TphRequesterCapability,
    tph_requester_control: TphRequesterControl,
    tph_st_table_0: TphStTable0,
    tph_st_table_1: TphStTable1,
    tph_st_table_2: TphStTable2,
}
impl RegisterBlock {
    #[doc = "0x00 - Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus."]
    #[inline(always)]
    pub const fn vendor_id_and_device_id(&self) -> &VendorIdAndDeviceId {
        &self.vendor_id_and_device_id
    }
    #[doc = "0x04 - Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
    #[inline(always)]
    pub const fn command_and_status(&self) -> &CommandAndStatus {
        &self.command_and_status
    }
    #[doc = "0x08 - Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub const fn revision_id_and_class_code(&self) -> &RevisionIdAndClassCode {
        &self.revision_id_and_class_code
    }
    #[doc = "0x0c - BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved"]
    #[inline(always)]
    pub const fn bist_header_type_latency_timer_and_cache_line_size_s(
        &self,
    ) -> &BistHeaderTypeLatencyTimerAndCacheLineSizeS {
        &self.bist_header_type_latency_timer_and_cache_line_size_s
    }
    #[doc = "0x10 - Base Address Register 0 (no description)"]
    #[inline(always)]
    pub const fn base_address_0(&self) -> &BaseAddress0 {
        &self.base_address_0
    }
    #[doc = "0x14 - Base Address Register 1 (no description)"]
    #[inline(always)]
    pub const fn base_address_1(&self) -> &BaseAddress1 {
        &self.base_address_1
    }
    #[doc = "0x18 - Base Address Register 2 (no description)"]
    #[inline(always)]
    pub const fn base_address_2(&self) -> &BaseAddress2 {
        &self.base_address_2
    }
    #[doc = "0x1c - Base Address Register 3 (no description)"]
    #[inline(always)]
    pub const fn base_address_3(&self) -> &BaseAddress3 {
        &self.base_address_3
    }
    #[doc = "0x20 - Base Address Register 4 (no description)"]
    #[inline(always)]
    pub const fn base_address_4(&self) -> &BaseAddress4 {
        &self.base_address_4
    }
    #[doc = "0x24 - Base Address Register 5 (no description)"]
    #[inline(always)]
    pub const fn base_address_5(&self) -> &BaseAddress5 {
        &self.base_address_5
    }
    #[doc = "0x2c - Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub const fn subsystem_vendor_id_and_subsystem_id(&self) -> &SubsystemVendorIdAndSubsystemId {
        &self.subsystem_vendor_id_and_subsystem_id
    }
    #[doc = "0x30 - Expansion ROM Base Address Register (no description)"]
    #[inline(always)]
    pub const fn expansion_rom_base_address(&self) -> &ExpansionRomBaseAddress {
        &self.expansion_rom_base_address
    }
    #[doc = "0x34 - Capabilities Pointer Reserved"]
    #[inline(always)]
    pub const fn capabilities_pointer(&self) -> &CapabilitiesPointer {
        &self.capabilities_pointer
    }
    #[doc = "0x3c - Interrupt Line and Interrupt Pin Register (no description)"]
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
    #[doc = "0xe4 - PCI Express Device Capabilities Register 2 Reserved"]
    #[inline(always)]
    pub const fn pci_express_device_capabilities_2(&self) -> &PciExpressDeviceCapabilities2 {
        &self.pci_express_device_capabilities_2
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
    #[doc = "0x108 - Uncorrectable Error Mask Register (no description)"]
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
    #[doc = "0x114 - Correctable Error Mask Register (no description)"]
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
    #[doc = "0x144 - ARI Capability Register and ARI Control Register Reserved"]
    #[inline(always)]
    pub const fn ari_capability_and_ari_control(&self) -> &AriCapabilityAndAriControl {
        &self.ari_capability_and_ari_control
    }
    #[doc = "0x274 - TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub const fn tph_requester_enhanced_capability_header(
        &self,
    ) -> &TphRequesterEnhancedCapabilityHeader {
        &self.tph_requester_enhanced_capability_header
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
}
#[doc = "VENDOR_ID_AND_DEVICE_ID (r) register accessor: Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id_and_device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_id_and_device_id`]
module"]
#[doc(alias = "VENDOR_ID_AND_DEVICE_ID")]
pub type VendorIdAndDeviceId = crate::Reg<vendor_id_and_device_id::VendorIdAndDeviceIdSpec>;
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written independently for each Function from the local management bus."]
pub mod vendor_id_and_device_id;
#[doc = "COMMAND_AND_STATUS (rw) register accessor: Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command_and_status`]
module"]
#[doc(alias = "COMMAND_AND_STATUS")]
pub type CommandAndStatus = crate::Reg<command_and_status::CommandAndStatusSpec>;
#[doc = "Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
pub mod command_and_status;
#[doc = "REVISION_ID_AND_CLASS_CODE (r) register accessor: Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision_id_and_class_code::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision_id_and_class_code`]
module"]
#[doc(alias = "REVISION_ID_AND_CLASS_CODE")]
pub type RevisionIdAndClassCode =
    crate::Reg<revision_id_and_class_code::RevisionIdAndClassCodeSpec>;
#[doc = "Revision ID and Class Code Register Identifies the function of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub mod revision_id_and_class_code;
#[doc = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S (r) register accessor: BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_header_type_latency_timer_and_cache_line_size_s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_header_type_latency_timer_and_cache_line_size_s`]
module"]
#[doc(alias = "BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S")]
pub type BistHeaderTypeLatencyTimerAndCacheLineSizeS = crate :: Reg < bist_header_type_latency_timer_and_cache_line_size_s :: BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec > ;
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved"]
pub mod bist_header_type_latency_timer_and_cache_line_size_s;
#[doc = "BASE_ADDRESS_0 (r) register accessor: Base Address Register 0 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_0`]
module"]
#[doc(alias = "BASE_ADDRESS_0")]
pub type BaseAddress0 = crate::Reg<base_address_0::BaseAddress0Spec>;
#[doc = "Base Address Register 0 (no description)"]
pub mod base_address_0;
#[doc = "BASE_ADDRESS_1 (r) register accessor: Base Address Register 1 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_1`]
module"]
#[doc(alias = "BASE_ADDRESS_1")]
pub type BaseAddress1 = crate::Reg<base_address_1::BaseAddress1Spec>;
#[doc = "Base Address Register 1 (no description)"]
pub mod base_address_1;
#[doc = "BASE_ADDRESS_2 (r) register accessor: Base Address Register 2 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_2`]
module"]
#[doc(alias = "BASE_ADDRESS_2")]
pub type BaseAddress2 = crate::Reg<base_address_2::BaseAddress2Spec>;
#[doc = "Base Address Register 2 (no description)"]
pub mod base_address_2;
#[doc = "BASE_ADDRESS_3 (r) register accessor: Base Address Register 3 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_3`]
module"]
#[doc(alias = "BASE_ADDRESS_3")]
pub type BaseAddress3 = crate::Reg<base_address_3::BaseAddress3Spec>;
#[doc = "Base Address Register 3 (no description)"]
pub mod base_address_3;
#[doc = "BASE_ADDRESS_4 (r) register accessor: Base Address Register 4 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_4`]
module"]
#[doc(alias = "BASE_ADDRESS_4")]
pub type BaseAddress4 = crate::Reg<base_address_4::BaseAddress4Spec>;
#[doc = "Base Address Register 4 (no description)"]
pub mod base_address_4;
#[doc = "BASE_ADDRESS_5 (r) register accessor: Base Address Register 5 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_address_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address_5`]
module"]
#[doc(alias = "BASE_ADDRESS_5")]
pub type BaseAddress5 = crate::Reg<base_address_5::BaseAddress5Spec>;
#[doc = "Base Address Register 5 (no description)"]
pub mod base_address_5;
#[doc = "SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID (r) register accessor: Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsystem_vendor_id_and_subsystem_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsystem_vendor_id_and_subsystem_id`]
module"]
#[doc(alias = "SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID")]
pub type SubsystemVendorIdAndSubsystemId =
    crate::Reg<subsystem_vendor_id_and_subsystem_id::SubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub mod subsystem_vendor_id_and_subsystem_id;
#[doc = "EXPANSION_ROM_BASE_ADDRESS (r) register accessor: Expansion ROM Base Address Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`expansion_rom_base_address::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@expansion_rom_base_address`]
module"]
#[doc(alias = "EXPANSION_ROM_BASE_ADDRESS")]
pub type ExpansionRomBaseAddress =
    crate::Reg<expansion_rom_base_address::ExpansionRomBaseAddressSpec>;
#[doc = "Expansion ROM Base Address Register (no description)"]
pub mod expansion_rom_base_address;
#[doc = "CAPABILITIES_POINTER (r) register accessor: Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_pointer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities_pointer`]
module"]
#[doc(alias = "CAPABILITIES_POINTER")]
pub type CapabilitiesPointer = crate::Reg<capabilities_pointer::CapabilitiesPointerSpec>;
#[doc = "Capabilities Pointer Reserved"]
pub mod capabilities_pointer;
#[doc = "INTERRUPT_LINE_AND_INTERRUPT_PIN (r) register accessor: Interrupt Line and Interrupt Pin Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_line_and_interrupt_pin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_line_and_interrupt_pin`]
module"]
#[doc(alias = "INTERRUPT_LINE_AND_INTERRUPT_PIN")]
pub type InterruptLineAndInterruptPin =
    crate::Reg<interrupt_line_and_interrupt_pin::InterruptLineAndInterruptPinSpec>;
#[doc = "Interrupt Line and Interrupt Pin Register (no description)"]
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
#[doc = "PCI_EXPRESS_DEVICE_CAPABILITIES_2 (r) register accessor: PCI Express Device Capabilities Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pci_express_device_capabilities_2`]
module"]
#[doc(alias = "PCI_EXPRESS_DEVICE_CAPABILITIES_2")]
pub type PciExpressDeviceCapabilities2 =
    crate::Reg<pci_express_device_capabilities_2::PciExpressDeviceCapabilities2Spec>;
#[doc = "PCI Express Device Capabilities Register 2 Reserved"]
pub mod pci_express_device_capabilities_2;
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
#[doc = "UNCORRECTABLE_ERROR_MASK (r) register accessor: Uncorrectable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_mask::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_mask`]
module"]
#[doc(alias = "UNCORRECTABLE_ERROR_MASK")]
pub type UncorrectableErrorMask = crate::Reg<uncorrectable_error_mask::UncorrectableErrorMaskSpec>;
#[doc = "Uncorrectable Error Mask Register (no description)"]
pub mod uncorrectable_error_mask;
#[doc = "UNCORRECTABLE_ERROR_SEVERITY (r) register accessor: Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_severity::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uncorrectable_error_severity`]
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
#[doc = "CORRECTABLE_ERROR_MASK (r) register accessor: Correctable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_mask::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@correctable_error_mask`]
module"]
#[doc(alias = "CORRECTABLE_ERROR_MASK")]
pub type CorrectableErrorMask = crate::Reg<correctable_error_mask::CorrectableErrorMaskSpec>;
#[doc = "Correctable Error Mask Register (no description)"]
pub mod correctable_error_mask;
#[doc = "ADVANCED_ERROR_CAPABILITIES_AND_CONTROL (r) register accessor: Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced_error_capabilities_and_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advanced_error_capabilities_and_control`]
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
#[doc = "ARI_CAPABILITY_AND_ARI_CONTROL (r) register accessor: ARI Capability Register and ARI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ari_capability_and_ari_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ari_capability_and_ari_control`]
module"]
#[doc(alias = "ARI_CAPABILITY_AND_ARI_CONTROL")]
pub type AriCapabilityAndAriControl =
    crate::Reg<ari_capability_and_ari_control::AriCapabilityAndAriControlSpec>;
#[doc = "ARI Capability Register and ARI Control Register Reserved"]
pub mod ari_capability_and_ari_control;
#[doc = "TPH_REQUESTER_ENHANCED_CAPABILITY_HEADER (r) register accessor: TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_requester_enhanced_capability_header::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tph_requester_enhanced_capability_header`]
module"]
#[doc(alias = "TPH_REQUESTER_ENHANCED_CAPABILITY_HEADER")]
pub type TphRequesterEnhancedCapabilityHeader =
    crate::Reg<tph_requester_enhanced_capability_header::TphRequesterEnhancedCapabilityHeaderSpec>;
#[doc = "TPH Requester Enhanced Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub mod tph_requester_enhanced_capability_header;
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
