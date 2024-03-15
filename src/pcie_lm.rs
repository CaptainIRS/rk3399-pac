#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    physical_layer_configuration_0: PhysicalLayerConfiguration0,
    physical_layer_configuration_1: PhysicalLayerConfiguration1,
    data_link_layer_timer_configuration: DataLinkLayerTimerConfiguration,
    receive_credit_limit_0_vc0: ReceiveCreditLimit0Vc0,
    receive_credit_limit_1_vc0: ReceiveCreditLimit1Vc0,
    transmit_credit_limit_0_vc0: TransmitCreditLimit0Vc0,
    transmit_credit_limit_1_vc0: TransmitCreditLimit1Vc0,
    transmit_credit_update_interval_configuration_0: TransmitCreditUpdateIntervalConfiguration0,
    transmit_credit_update_interval_configuration_1: TransmitCreditUpdateIntervalConfiguration1,
    l0s_timeout_limit: L0sTimeoutLimit,
    transmit_tlp_count: TransmitTlpCount,
    transmit_tlp_payload_dword_count: TransmitTlpPayloadDwordCount,
    receive_tlp_count: ReceiveTlpCount,
    receive_tlp_payload_dword_count: ReceiveTlpPayloadDwordCount,
    completion_timeout_limit_0: CompletionTimeoutLimit0,
    completion_timeout_limit_1: CompletionTimeoutLimit1,
    l1_state_re_entry_delay: L1StateReEntryDelay,
    vendor_id: VendorId,
    aspm_l1_entry_timeout_delay: AspmL1EntryTimeoutDelay,
    pme_turnoff_ack_delay: PmeTurnoffAckDelay,
    linkwidth_control: LinkwidthControl,
    _reserved21: [u8; 0x20],
    sris_control: SrisControl,
    _reserved22: [u8; 0x88],
    shadow_register_header_log_0: ShadowRegisterHeaderLog0,
    shadow_register_header_log_1: ShadowRegisterHeaderLog1,
    shadow_register_header_log_2: ShadowRegisterHeaderLog2,
    shadow_register_header_log_3: ShadowRegisterHeaderLog3,
    shadow_register_function_number: ShadowRegisterFunctionNumber,
    shadow_ur_error: ShadowUrError,
    _reserved28: [u8; 0xe8],
    negotiated_lane_map: NegotiatedLaneMap,
    receive_fts_count: ReceiveFtsCount,
    debug_mux_control: DebugMuxControl,
    local_error_and_status: LocalErrorAndStatus,
    local_interrupt_mask: LocalInterruptMask,
    lcrc_error_count: LcrcErrorCount,
    ecc_correctable_error_count: EccCorrectableErrorCount,
    ltr_snoop_no_snoop_latency: LtrSnoopNoSnoopLatency,
    ltr_message_generation_control: LtrMessageGenerationControl,
    pme_service_timeout_delay: PmeServiceTimeoutDelay,
    root_port_requestor_id: RootPortRequestorId,
    end_point_bus_and_device_number: EndPointBusAndDeviceNumber,
    _reserved40: [u8; 0x10],
    physical_function_bar_configuration_0: PhysicalFunctionBarConfiguration0,
    physical_function_bar_configuration_1: PhysicalFunctionBarConfiguration1,
    _reserved42: [u8; 0x38],
    virtual_function_bar_configuration_0: VirtualFunctionBarConfiguration0,
    virtual_function_bar_configuration_1: VirtualFunctionBarConfiguration1,
    _reserved44: [u8; 0x38],
    physical_function_configuration: PhysicalFunctionConfiguration,
    _reserved45: [u8; 0x3c],
    root_complex_bar_configuration: RootComplexBarConfiguration,
}
impl RegisterBlock {
    #[doc = "0x00 - Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode."]
    #[inline(always)]
    pub const fn physical_layer_configuration_0(&self) -> &PhysicalLayerConfiguration0 {
        &self.physical_layer_configuration_0
    }
    #[doc = "0x04 - Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
    #[inline(always)]
    pub const fn physical_layer_configuration_1(&self) -> &PhysicalLayerConfiguration1 {
        &self.physical_layer_configuration_1
    }
    #[doc = "0x08 - Data Link Layer Timer Configuration Register Reserved"]
    #[inline(always)]
    pub const fn data_link_layer_timer_configuration(&self) -> &DataLinkLayerTimerConfiguration {
        &self.data_link_layer_timer_configuration
    }
    #[doc = "0x0c - Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub const fn receive_credit_limit_0_vc0(&self) -> &ReceiveCreditLimit0Vc0 {
        &self.receive_credit_limit_0_vc0
    }
    #[doc = "0x10 - Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub const fn receive_credit_limit_1_vc0(&self) -> &ReceiveCreditLimit1Vc0 {
        &self.receive_credit_limit_1_vc0
    }
    #[doc = "0x14 - Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub const fn transmit_credit_limit_0_vc0(&self) -> &TransmitCreditLimit0Vc0 {
        &self.transmit_credit_limit_0_vc0
    }
    #[doc = "0x18 - Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub const fn transmit_credit_limit_1_vc0(&self) -> &TransmitCreditLimit1Vc0 {
        &self.transmit_credit_limit_1_vc0
    }
    #[doc = "0x1c - Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub const fn transmit_credit_update_interval_configuration_0(
        &self,
    ) -> &TransmitCreditUpdateIntervalConfiguration0 {
        &self.transmit_credit_update_interval_configuration_0
    }
    #[doc = "0x20 - Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub const fn transmit_credit_update_interval_configuration_1(
        &self,
    ) -> &TransmitCreditUpdateIntervalConfiguration1 {
        &self.transmit_credit_update_interval_configuration_1
    }
    #[doc = "0x24 - L0S Timeout Limit Register Reserved"]
    #[inline(always)]
    pub const fn l0s_timeout_limit(&self) -> &L0sTimeoutLimit {
        &self.l0s_timeout_limit
    }
    #[doc = "0x28 - Transmit TLP Count Register Count of TLPs transmitted"]
    #[inline(always)]
    pub const fn transmit_tlp_count(&self) -> &TransmitTlpCount {
        &self.transmit_tlp_count
    }
    #[doc = "0x2c - Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted"]
    #[inline(always)]
    pub const fn transmit_tlp_payload_dword_count(&self) -> &TransmitTlpPayloadDwordCount {
        &self.transmit_tlp_payload_dword_count
    }
    #[doc = "0x30 - Receive TLP Count Register Count of TLPs received"]
    #[inline(always)]
    pub const fn receive_tlp_count(&self) -> &ReceiveTlpCount {
        &self.receive_tlp_count
    }
    #[doc = "0x34 - Receive TLP Payload Dword Count Register Count of TLP payload Dwords received"]
    #[inline(always)]
    pub const fn receive_tlp_payload_dword_count(&self) -> &ReceiveTlpPayloadDwordCount {
        &self.receive_tlp_payload_dword_count
    }
    #[doc = "0x38 - Completion Timeout Limit Register 0 Reserved"]
    #[inline(always)]
    pub const fn completion_timeout_limit_0(&self) -> &CompletionTimeoutLimit0 {
        &self.completion_timeout_limit_0
    }
    #[doc = "0x3c - Completion Timeout Limit Register 1 Reserved"]
    #[inline(always)]
    pub const fn completion_timeout_limit_1(&self) -> &CompletionTimeoutLimit1 {
        &self.completion_timeout_limit_1
    }
    #[doc = "0x40 - L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns)."]
    #[inline(always)]
    pub const fn l1_state_re_entry_delay(&self) -> &L1StateReEntryDelay {
        &self.l1_state_re_entry_delay
    }
    #[doc = "0x44 - Vendor ID Register Subsystem Vendor ID"]
    #[inline(always)]
    pub const fn vendor_id(&self) -> &VendorId {
        &self.vendor_id
    }
    #[doc = "0x48 - ASPM L1 Entry Timeout Delay Register Reserved"]
    #[inline(always)]
    pub const fn aspm_l1_entry_timeout_delay(&self) -> &AspmL1EntryTimeoutDelay {
        &self.aspm_l1_entry_timeout_delay
    }
    #[doc = "0x4c - PME TurnOff Ack Delay Register Reserved"]
    #[inline(always)]
    pub const fn pme_turnoff_ack_delay(&self) -> &PmeTurnoffAckDelay {
        &self.pme_turnoff_ack_delay
    }
    #[doc = "0x50 - Linkwidth Control Register Reserved"]
    #[inline(always)]
    pub const fn linkwidth_control(&self) -> &LinkwidthControl {
        &self.linkwidth_control
    }
    #[doc = "0x74 - SRIS Control Register Reserved"]
    #[inline(always)]
    pub const fn sris_control(&self) -> &SrisControl {
        &self.sris_control
    }
    #[doc = "0x100 - Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn shadow_register_header_log_0(&self) -> &ShadowRegisterHeaderLog0 {
        &self.shadow_register_header_log_0
    }
    #[doc = "0x104 - Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn shadow_register_header_log_1(&self) -> &ShadowRegisterHeaderLog1 {
        &self.shadow_register_header_log_1
    }
    #[doc = "0x108 - Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn shadow_register_header_log_2(&self) -> &ShadowRegisterHeaderLog2 {
        &self.shadow_register_header_log_2
    }
    #[doc = "0x10c - Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
    #[inline(always)]
    pub const fn shadow_register_header_log_3(&self) -> &ShadowRegisterHeaderLog3 {
        &self.shadow_register_header_log_3
    }
    #[doc = "0x110 - Shadow register function number. Reserved"]
    #[inline(always)]
    pub const fn shadow_register_function_number(&self) -> &ShadowRegisterFunctionNumber {
        &self.shadow_register_function_number
    }
    #[doc = "0x114 - Shadow Register UR Error Reserved"]
    #[inline(always)]
    pub const fn shadow_ur_error(&self) -> &ShadowUrError {
        &self.shadow_ur_error
    }
    #[doc = "0x200 - Negotiated Lane Map Register Reserved"]
    #[inline(always)]
    pub const fn negotiated_lane_map(&self) -> &NegotiatedLaneMap {
        &self.negotiated_lane_map
    }
    #[doc = "0x204 - Receive FTS Count Register Reserved"]
    #[inline(always)]
    pub const fn receive_fts_count(&self) -> &ReceiveFtsCount {
        &self.receive_fts_count
    }
    #[doc = "0x208 - Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit."]
    #[inline(always)]
    pub const fn debug_mux_control(&self) -> &DebugMuxControl {
        &self.debug_mux_control
    }
    #[doc = "0x20c - Local Error and Status Register Reserved"]
    #[inline(always)]
    pub const fn local_error_and_status(&self) -> &LocalErrorAndStatus {
        &self.local_error_and_status
    }
    #[doc = "0x210 - Local Interrupt Mask Register Reserved"]
    #[inline(always)]
    pub const fn local_interrupt_mask(&self) -> &LocalInterruptMask {
        &self.local_interrupt_mask
    }
    #[doc = "0x214 - LCRC Error Count Register Reserved"]
    #[inline(always)]
    pub const fn lcrc_error_count(&self) -> &LcrcErrorCount {
        &self.lcrc_error_count
    }
    #[doc = "0x218 - ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it."]
    #[inline(always)]
    pub const fn ecc_correctable_error_count(&self) -> &EccCorrectableErrorCount {
        &self.ecc_correctable_error_count
    }
    #[doc = "0x21c - LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    pub const fn ltr_snoop_no_snoop_latency(&self) -> &LtrSnoopNoSnoopLatency {
        &self.ltr_snoop_no_snoop_latency
    }
    #[doc = "0x220 - LTR Message Generation Control Register RSVD"]
    #[inline(always)]
    pub const fn ltr_message_generation_control(&self) -> &LtrMessageGenerationControl {
        &self.ltr_message_generation_control
    }
    #[doc = "0x224 - PME Service Timeout Delay Register Reserved"]
    #[inline(always)]
    pub const fn pme_service_timeout_delay(&self) -> &PmeServiceTimeoutDelay {
        &self.pme_service_timeout_delay
    }
    #[doc = "0x228 - Root Port Requestor ID Register Reserved"]
    #[inline(always)]
    pub const fn root_port_requestor_id(&self) -> &RootPortRequestorId {
        &self.root_port_requestor_id
    }
    #[doc = "0x22c - End Point Bus and Device Number Register Reserved"]
    #[inline(always)]
    pub const fn end_point_bus_and_device_number(&self) -> &EndPointBusAndDeviceNumber {
        &self.end_point_bus_and_device_number
    }
    #[doc = "0x240 - Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub const fn physical_function_bar_configuration_0(
        &self,
    ) -> &PhysicalFunctionBarConfiguration0 {
        &self.physical_function_bar_configuration_0
    }
    #[doc = "0x244 - Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure."]
    #[inline(always)]
    pub const fn physical_function_bar_configuration_1(
        &self,
    ) -> &PhysicalFunctionBarConfiguration1 {
        &self.physical_function_bar_configuration_1
    }
    #[doc = "0x280 - Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub const fn virtual_function_bar_configuration_0(&self) -> &VirtualFunctionBarConfiguration0 {
        &self.virtual_function_bar_configuration_0
    }
    #[doc = "0x284 - Virtual Function BAR Configuration Register 1 Reserved"]
    #[inline(always)]
    pub const fn virtual_function_bar_configuration_1(&self) -> &VirtualFunctionBarConfiguration1 {
        &self.virtual_function_bar_configuration_1
    }
    #[doc = "0x2c0 - Physical Function Configuration Register Reserved"]
    #[inline(always)]
    pub const fn physical_function_configuration(&self) -> &PhysicalFunctionConfiguration {
        &self.physical_function_configuration
    }
    #[doc = "0x300 - Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
    #[inline(always)]
    pub const fn root_complex_bar_configuration(&self) -> &RootComplexBarConfiguration {
        &self.root_complex_bar_configuration
    }
}
#[doc = "PHYSICAL_LAYER_CONFIGURATION_0 (rw) register accessor: Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_layer_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_layer_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_layer_configuration_0`]
module"]
#[doc(alias = "PHYSICAL_LAYER_CONFIGURATION_0")]
pub type PhysicalLayerConfiguration0 =
    crate::Reg<physical_layer_configuration_0::PhysicalLayerConfiguration0Spec>;
#[doc = "Physical Layer Configuration Register 0 When the core is operating as a Root Port, setting this to 1 causes the LTSSM to initiate a loopback and become the loopback master. This bit is not used in the EndPoint Mode."]
pub mod physical_layer_configuration_0;
#[doc = "PHYSICAL_LAYER_CONFIGURATION_1 (rw) register accessor: Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_layer_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_layer_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_layer_configuration_1`]
module"]
#[doc(alias = "PHYSICAL_LAYER_CONFIGURATION_1")]
pub type PhysicalLayerConfiguration1 =
    crate::Reg<physical_layer_configuration_1::PhysicalLayerConfiguration1Spec>;
#[doc = "Physical Layer Configuration Register 1 FTS count transmitted by the core in TS1/TS2 sequences during link training. This value must be set based on the time needed by the receiver to acquire sync while exiting from L0S state."]
pub mod physical_layer_configuration_1;
#[doc = "DATA_LINK_LAYER_TIMER_CONFIGURATION (rw) register accessor: Data Link Layer Timer Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_link_layer_timer_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_link_layer_timer_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_link_layer_timer_configuration`]
module"]
#[doc(alias = "DATA_LINK_LAYER_TIMER_CONFIGURATION")]
pub type DataLinkLayerTimerConfiguration =
    crate::Reg<data_link_layer_timer_configuration::DataLinkLayerTimerConfigurationSpec>;
#[doc = "Data Link Layer Timer Configuration Register Reserved"]
pub mod data_link_layer_timer_configuration;
#[doc = "RECEIVE_CREDIT_LIMIT_0_VC0 (rw) register accessor: Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_credit_limit_0_vc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_credit_limit_0_vc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_credit_limit_0_vc0`]
module"]
#[doc(alias = "RECEIVE_CREDIT_LIMIT_0_VC0")]
pub type ReceiveCreditLimit0Vc0 =
    crate::Reg<receive_credit_limit_0_vc0::ReceiveCreditLimit0Vc0Spec>;
#[doc = "Receive Credit Limit Register 0 VC0 Non-Posted payload credit limit advertised by the core for VC 0 (in units of 4 Dwords)."]
pub mod receive_credit_limit_0_vc0;
#[doc = "RECEIVE_CREDIT_LIMIT_1_VC0 (rw) register accessor: Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_credit_limit_1_vc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_credit_limit_1_vc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_credit_limit_1_vc0`]
module"]
#[doc(alias = "RECEIVE_CREDIT_LIMIT_1_VC0")]
pub type ReceiveCreditLimit1Vc0 =
    crate::Reg<receive_credit_limit_1_vc0::ReceiveCreditLimit1Vc0Spec>;
#[doc = "Receive Credit Limit Register 1 VC0 Completion header credit limit advertised by the core for VC 0 (in number of packets)."]
pub mod receive_credit_limit_1_vc0;
#[doc = "TRANSMIT_CREDIT_LIMIT_0_VC0 (r) register accessor: Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_credit_limit_0_vc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_credit_limit_0_vc0`]
module"]
#[doc(alias = "TRANSMIT_CREDIT_LIMIT_0_VC0")]
pub type TransmitCreditLimit0Vc0 =
    crate::Reg<transmit_credit_limit_0_vc0::TransmitCreditLimit0Vc0Spec>;
#[doc = "Transmit Credit Limit Register 0 VC0 Non-Posted payload credit limit received by the core for Link 0 (in units of 4 Dwords)."]
pub mod transmit_credit_limit_0_vc0;
#[doc = "TRANSMIT_CREDIT_LIMIT_1_VC0 (r) register accessor: Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_credit_limit_1_vc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_credit_limit_1_vc0`]
module"]
#[doc(alias = "TRANSMIT_CREDIT_LIMIT_1_VC0")]
pub type TransmitCreditLimit1Vc0 =
    crate::Reg<transmit_credit_limit_1_vc0::TransmitCreditLimit1Vc0Spec>;
#[doc = "Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets)."]
pub mod transmit_credit_limit_1_vc0;
#[doc = "TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0 (rw) register accessor: Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_credit_update_interval_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_credit_update_interval_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_credit_update_interval_configuration_0`]
module"]
#[doc(alias = "TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_0")]
pub type TransmitCreditUpdateIntervalConfiguration0 = crate::Reg<
    transmit_credit_update_interval_configuration_0::TransmitCreditUpdateIntervalConfiguration0Spec,
>;
#[doc = "Transmit Credit Update Interval Configuration Register 0 Minimum credit update interval for non-posted transactions. The core follows this minimum interval between issuing posted credit updates on the link. This is to limit the bandwidth use of credit updates. If new credit becomes available in the receive FIFO since the last update was sent, the core will issue a new update only after this interval has elapsed since the last update. The value is in units of 4 ns. This field is re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub mod transmit_credit_update_interval_configuration_0;
#[doc = "TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1 (rw) register accessor: Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_credit_update_interval_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_credit_update_interval_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_credit_update_interval_configuration_1`]
module"]
#[doc(alias = "TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1")]
pub type TransmitCreditUpdateIntervalConfiguration1 = crate::Reg<
    transmit_credit_update_interval_configuration_1::TransmitCreditUpdateIntervalConfiguration1Spec,
>;
#[doc = "Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub mod transmit_credit_update_interval_configuration_1;
#[doc = "L0S_TIMEOUT_LIMIT (rw) register accessor: L0S Timeout Limit Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0s_timeout_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0s_timeout_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0s_timeout_limit`]
module"]
#[doc(alias = "L0S_TIMEOUT_LIMIT")]
pub type L0sTimeoutLimit = crate::Reg<l0s_timeout_limit::L0sTimeoutLimitSpec>;
#[doc = "L0S Timeout Limit Register Reserved"]
pub mod l0s_timeout_limit;
#[doc = "TRANSMIT_TLP_COUNT (rw) register accessor: Transmit TLP Count Register Count of TLPs transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_tlp_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_tlp_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_tlp_count`]
module"]
#[doc(alias = "TRANSMIT_TLP_COUNT")]
pub type TransmitTlpCount = crate::Reg<transmit_tlp_count::TransmitTlpCountSpec>;
#[doc = "Transmit TLP Count Register Count of TLPs transmitted"]
pub mod transmit_tlp_count;
#[doc = "TRANSMIT_TLP_PAYLOAD_DWORD_COUNT (rw) register accessor: Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_tlp_payload_dword_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_tlp_payload_dword_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_tlp_payload_dword_count`]
module"]
#[doc(alias = "TRANSMIT_TLP_PAYLOAD_DWORD_COUNT")]
pub type TransmitTlpPayloadDwordCount =
    crate::Reg<transmit_tlp_payload_dword_count::TransmitTlpPayloadDwordCountSpec>;
#[doc = "Transmit TLP Payload Dword Count Register Count of TLPs payload Dwords transmitted"]
pub mod transmit_tlp_payload_dword_count;
#[doc = "RECEIVE_TLP_COUNT (rw) register accessor: Receive TLP Count Register Count of TLPs received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_tlp_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_tlp_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_tlp_count`]
module"]
#[doc(alias = "RECEIVE_TLP_COUNT")]
pub type ReceiveTlpCount = crate::Reg<receive_tlp_count::ReceiveTlpCountSpec>;
#[doc = "Receive TLP Count Register Count of TLPs received"]
pub mod receive_tlp_count;
#[doc = "RECEIVE_TLP_PAYLOAD_DWORD_COUNT (rw) register accessor: Receive TLP Payload Dword Count Register Count of TLP payload Dwords received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_tlp_payload_dword_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_tlp_payload_dword_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_tlp_payload_dword_count`]
module"]
#[doc(alias = "RECEIVE_TLP_PAYLOAD_DWORD_COUNT")]
pub type ReceiveTlpPayloadDwordCount =
    crate::Reg<receive_tlp_payload_dword_count::ReceiveTlpPayloadDwordCountSpec>;
#[doc = "Receive TLP Payload Dword Count Register Count of TLP payload Dwords received"]
pub mod receive_tlp_payload_dword_count;
#[doc = "COMPLETION_TIMEOUT_LIMIT_0 (rw) register accessor: Completion Timeout Limit Register 0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`completion_timeout_limit_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`completion_timeout_limit_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@completion_timeout_limit_0`]
module"]
#[doc(alias = "COMPLETION_TIMEOUT_LIMIT_0")]
pub type CompletionTimeoutLimit0 =
    crate::Reg<completion_timeout_limit_0::CompletionTimeoutLimit0Spec>;
#[doc = "Completion Timeout Limit Register 0 Reserved"]
pub mod completion_timeout_limit_0;
#[doc = "COMPLETION_TIMEOUT_LIMIT_1 (rw) register accessor: Completion Timeout Limit Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`completion_timeout_limit_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`completion_timeout_limit_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@completion_timeout_limit_1`]
module"]
#[doc(alias = "COMPLETION_TIMEOUT_LIMIT_1")]
pub type CompletionTimeoutLimit1 =
    crate::Reg<completion_timeout_limit_1::CompletionTimeoutLimit1Spec>;
#[doc = "Completion Timeout Limit Register 1 Reserved"]
pub mod completion_timeout_limit_1;
#[doc = "L1_STATE_RE_ENTRY_DELAY (rw) register accessor: L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_state_re_entry_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_state_re_entry_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_state_re_entry_delay`]
module"]
#[doc(alias = "L1_STATE_RE_ENTRY_DELAY")]
pub type L1StateReEntryDelay = crate::Reg<l1_state_re_entry_delay::L1StateReEntryDelaySpec>;
#[doc = "L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns)."]
pub mod l1_state_re_entry_delay;
#[doc = "VENDOR_ID (rw) register accessor: Vendor ID Register Subsystem Vendor ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vendor_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_id`]
module"]
#[doc(alias = "VENDOR_ID")]
pub type VendorId = crate::Reg<vendor_id::VendorIdSpec>;
#[doc = "Vendor ID Register Subsystem Vendor ID"]
pub mod vendor_id;
#[doc = "ASPM_L1_ENTRY_TIMEOUT_DELAY (rw) register accessor: ASPM L1 Entry Timeout Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aspm_l1_entry_timeout_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aspm_l1_entry_timeout_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aspm_l1_entry_timeout_delay`]
module"]
#[doc(alias = "ASPM_L1_ENTRY_TIMEOUT_DELAY")]
pub type AspmL1EntryTimeoutDelay =
    crate::Reg<aspm_l1_entry_timeout_delay::AspmL1EntryTimeoutDelaySpec>;
#[doc = "ASPM L1 Entry Timeout Delay Register Reserved"]
pub mod aspm_l1_entry_timeout_delay;
#[doc = "PME_TURNOFF_ACK_DELAY (rw) register accessor: PME TurnOff Ack Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pme_turnoff_ack_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pme_turnoff_ack_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pme_turnoff_ack_delay`]
module"]
#[doc(alias = "PME_TURNOFF_ACK_DELAY")]
pub type PmeTurnoffAckDelay = crate::Reg<pme_turnoff_ack_delay::PmeTurnoffAckDelaySpec>;
#[doc = "PME TurnOff Ack Delay Register Reserved"]
pub mod pme_turnoff_ack_delay;
#[doc = "LINKWIDTH_CONTROL (rw) register accessor: Linkwidth Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linkwidth_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkwidth_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkwidth_control`]
module"]
#[doc(alias = "LINKWIDTH_CONTROL")]
pub type LinkwidthControl = crate::Reg<linkwidth_control::LinkwidthControlSpec>;
#[doc = "Linkwidth Control Register Reserved"]
pub mod linkwidth_control;
#[doc = "SRIS_CONTROL (rw) register accessor: SRIS Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sris_control`]
module"]
#[doc(alias = "SRIS_CONTROL")]
pub type SrisControl = crate::Reg<sris_control::SrisControlSpec>;
#[doc = "SRIS Control Register Reserved"]
pub mod sris_control;
#[doc = "SHADOW_REGISTER_HEADER_LOG_0 (rw) register accessor: Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_register_header_log_0`]
module"]
#[doc(alias = "SHADOW_REGISTER_HEADER_LOG_0")]
pub type ShadowRegisterHeaderLog0 =
    crate::Reg<shadow_register_header_log_0::ShadowRegisterHeaderLog0Spec>;
#[doc = "Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
pub mod shadow_register_header_log_0;
#[doc = "SHADOW_REGISTER_HEADER_LOG_1 (rw) register accessor: Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_register_header_log_1`]
module"]
#[doc(alias = "SHADOW_REGISTER_HEADER_LOG_1")]
pub type ShadowRegisterHeaderLog1 =
    crate::Reg<shadow_register_header_log_1::ShadowRegisterHeaderLog1Spec>;
#[doc = "Shadow register header log 1 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[63:32\\]
value of the TLP header."]
pub mod shadow_register_header_log_1;
#[doc = "SHADOW_REGISTER_HEADER_LOG_2 (rw) register accessor: Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_register_header_log_2`]
module"]
#[doc(alias = "SHADOW_REGISTER_HEADER_LOG_2")]
pub type ShadowRegisterHeaderLog2 =
    crate::Reg<shadow_register_header_log_2::ShadowRegisterHeaderLog2Spec>;
#[doc = "Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
pub mod shadow_register_header_log_2;
#[doc = "SHADOW_REGISTER_HEADER_LOG_3 (rw) register accessor: Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_register_header_log_3`]
module"]
#[doc(alias = "SHADOW_REGISTER_HEADER_LOG_3")]
pub type ShadowRegisterHeaderLog3 =
    crate::Reg<shadow_register_header_log_3::ShadowRegisterHeaderLog3Spec>;
#[doc = "Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
pub mod shadow_register_header_log_3;
#[doc = "SHADOW_REGISTER_FUNCTION_NUMBER (rw) register accessor: Shadow register function number. Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_function_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_function_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_register_function_number`]
module"]
#[doc(alias = "SHADOW_REGISTER_FUNCTION_NUMBER")]
pub type ShadowRegisterFunctionNumber =
    crate::Reg<shadow_register_function_number::ShadowRegisterFunctionNumberSpec>;
#[doc = "Shadow register function number. Reserved"]
pub mod shadow_register_function_number;
#[doc = "SHADOW_UR_ERROR (rw) register accessor: Shadow Register UR Error Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_ur_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_ur_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow_ur_error`]
module"]
#[doc(alias = "SHADOW_UR_ERROR")]
pub type ShadowUrError = crate::Reg<shadow_ur_error::ShadowUrErrorSpec>;
#[doc = "Shadow Register UR Error Reserved"]
pub mod shadow_ur_error;
#[doc = "NEGOTIATED_LANE_MAP (r) register accessor: Negotiated Lane Map Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`negotiated_lane_map::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@negotiated_lane_map`]
module"]
#[doc(alias = "NEGOTIATED_LANE_MAP")]
pub type NegotiatedLaneMap = crate::Reg<negotiated_lane_map::NegotiatedLaneMapSpec>;
#[doc = "Negotiated Lane Map Register Reserved"]
pub mod negotiated_lane_map;
#[doc = "RECEIVE_FTS_COUNT (r) register accessor: Receive FTS Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_fts_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_fts_count`]
module"]
#[doc(alias = "RECEIVE_FTS_COUNT")]
pub type ReceiveFtsCount = crate::Reg<receive_fts_count::ReceiveFtsCountSpec>;
#[doc = "Receive FTS Count Register Reserved"]
pub mod receive_fts_count;
#[doc = "DEBUG_MUX_CONTROL (rw) register accessor: Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_mux_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_mux_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_mux_control`]
module"]
#[doc(alias = "DEBUG_MUX_CONTROL")]
pub type DebugMuxControl = crate::Reg<debug_mux_control::DebugMuxControlSpec>;
#[doc = "Debug Mux Control Register Setting this bit to 0 causes all the enabled Functions to report an error when a Type-1 configuration access is received by the core, targeted at any Function. Setting it to 1 limits the error reporting to the type-0 Function whose number matches with the Function number specified in the request. If the Function number in the request refers to an unimplemented or disabled Function, all enabled Functions report the error regardless of the setting of this bit."]
pub mod debug_mux_control;
#[doc = "LOCAL_ERROR_AND_STATUS (rw) register accessor: Local Error and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`local_error_and_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`local_error_and_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@local_error_and_status`]
module"]
#[doc(alias = "LOCAL_ERROR_AND_STATUS")]
pub type LocalErrorAndStatus = crate::Reg<local_error_and_status::LocalErrorAndStatusSpec>;
#[doc = "Local Error and Status Register Reserved"]
pub mod local_error_and_status;
#[doc = "LOCAL_INTERRUPT_MASK (rw) register accessor: Local Interrupt Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`local_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`local_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@local_interrupt_mask`]
module"]
#[doc(alias = "LOCAL_INTERRUPT_MASK")]
pub type LocalInterruptMask = crate::Reg<local_interrupt_mask::LocalInterruptMaskSpec>;
#[doc = "Local Interrupt Mask Register Reserved"]
pub mod local_interrupt_mask;
#[doc = "LCRC_ERROR_COUNT (rw) register accessor: LCRC Error Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrc_error_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrc_error_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrc_error_count`]
module"]
#[doc(alias = "LCRC_ERROR_COUNT")]
pub type LcrcErrorCount = crate::Reg<lcrc_error_count::LcrcErrorCountSpec>;
#[doc = "LCRC Error Count Register Reserved"]
pub mod lcrc_error_count;
#[doc = "ECC_CORRECTABLE_ERROR_COUNT (rw) register accessor: ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_correctable_error_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_correctable_error_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_correctable_error_count`]
module"]
#[doc(alias = "ECC_CORRECTABLE_ERROR_COUNT")]
pub type EccCorrectableErrorCount =
    crate::Reg<ecc_correctable_error_count::EccCorrectableErrorCountSpec>;
#[doc = "ECC Correctable Error Count Register Number of correctable errors detected while reading from the TPH Steering Tag RAM. This is an 8-bit saturating counter that can be cleared by writing all 1s into it."]
pub mod ecc_correctable_error_count;
#[doc = "LTR_SNOOP_NO_SNOOP_LATENCY (rw) register accessor: LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr_snoop_no_snoop_latency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr_snoop_no_snoop_latency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr_snoop_no_snoop_latency`]
module"]
#[doc(alias = "LTR_SNOOP_NO_SNOOP_LATENCY")]
pub type LtrSnoopNoSnoopLatency =
    crate::Reg<ltr_snoop_no_snoop_latency::LtrSnoopNoSnoopLatencySpec>;
#[doc = "LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
pub mod ltr_snoop_no_snoop_latency;
#[doc = "LTR_MESSAGE_GENERATION_CONTROL (rw) register accessor: LTR Message Generation Control Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr_message_generation_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr_message_generation_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr_message_generation_control`]
module"]
#[doc(alias = "LTR_MESSAGE_GENERATION_CONTROL")]
pub type LtrMessageGenerationControl =
    crate::Reg<ltr_message_generation_control::LtrMessageGenerationControlSpec>;
#[doc = "LTR Message Generation Control Register RSVD"]
pub mod ltr_message_generation_control;
#[doc = "PME_SERVICE_TIMEOUT_DELAY (rw) register accessor: PME Service Timeout Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pme_service_timeout_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pme_service_timeout_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pme_service_timeout_delay`]
module"]
#[doc(alias = "PME_SERVICE_TIMEOUT_DELAY")]
pub type PmeServiceTimeoutDelay = crate::Reg<pme_service_timeout_delay::PmeServiceTimeoutDelaySpec>;
#[doc = "PME Service Timeout Delay Register Reserved"]
pub mod pme_service_timeout_delay;
#[doc = "ROOT_PORT_REQUESTOR_ID (rw) register accessor: Root Port Requestor ID Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_port_requestor_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_port_requestor_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_port_requestor_id`]
module"]
#[doc(alias = "ROOT_PORT_REQUESTOR_ID")]
pub type RootPortRequestorId = crate::Reg<root_port_requestor_id::RootPortRequestorIdSpec>;
#[doc = "Root Port Requestor ID Register Reserved"]
pub mod root_port_requestor_id;
#[doc = "END_POINT_BUS_AND_DEVICE_NUMBER (r) register accessor: End Point Bus and Device Number Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`end_point_bus_and_device_number::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end_point_bus_and_device_number`]
module"]
#[doc(alias = "END_POINT_BUS_AND_DEVICE_NUMBER")]
pub type EndPointBusAndDeviceNumber =
    crate::Reg<end_point_bus_and_device_number::EndPointBusAndDeviceNumberSpec>;
#[doc = "End Point Bus and Device Number Register Reserved"]
pub mod end_point_bus_and_device_number;
#[doc = "PHYSICAL_FUNCTION_BAR_CONFIGURATION_0 (rw) register accessor: Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_function_bar_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_function_bar_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_function_bar_configuration_0`]
module"]
#[doc(alias = "PHYSICAL_FUNCTION_BAR_CONFIGURATION_0")]
pub type PhysicalFunctionBarConfiguration0 =
    crate::Reg<physical_function_bar_configuration_0::PhysicalFunctionBarConfiguration0Spec>;
#[doc = "Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub mod physical_function_bar_configuration_0;
#[doc = "PHYSICAL_FUNCTION_BAR_CONFIGURATION_1 (rw) register accessor: Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_function_bar_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_function_bar_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_function_bar_configuration_1`]
module"]
#[doc(alias = "PHYSICAL_FUNCTION_BAR_CONFIGURATION_1")]
pub type PhysicalFunctionBarConfiguration1 =
    crate::Reg<physical_function_bar_configuration_1::PhysicalFunctionBarConfiguration1Spec>;
#[doc = "Physical Function BAR Configuration Register 1 Setting this bit to 1 enables the Resizable BAR Capability in the PCI Express Configuration Space of the associated Function. When the Resizable BAR Capability is enabled, the apertures of the memory BARs of the corresponding Function are no longer selected by the fields in this register, but by the setting of the registers in the Resizable BAR Capability Structure."]
pub mod physical_function_bar_configuration_1;
#[doc = "VIRTUAL_FUNCTION_BAR_CONFIGURATION_0 (rw) register accessor: Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`virtual_function_bar_configuration_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`virtual_function_bar_configuration_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@virtual_function_bar_configuration_0`]
module"]
#[doc(alias = "VIRTUAL_FUNCTION_BAR_CONFIGURATION_0")]
pub type VirtualFunctionBarConfiguration0 =
    crate::Reg<virtual_function_bar_configuration_0::VirtualFunctionBarConfiguration0Spec>;
#[doc = "Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub mod virtual_function_bar_configuration_0;
#[doc = "VIRTUAL_FUNCTION_BAR_CONFIGURATION_1 (rw) register accessor: Virtual Function BAR Configuration Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`virtual_function_bar_configuration_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`virtual_function_bar_configuration_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@virtual_function_bar_configuration_1`]
module"]
#[doc(alias = "VIRTUAL_FUNCTION_BAR_CONFIGURATION_1")]
pub type VirtualFunctionBarConfiguration1 =
    crate::Reg<virtual_function_bar_configuration_1::VirtualFunctionBarConfiguration1Spec>;
#[doc = "Virtual Function BAR Configuration Register 1 Reserved"]
pub mod virtual_function_bar_configuration_1;
#[doc = "PHYSICAL_FUNCTION_CONFIGURATION (r) register accessor: Physical Function Configuration Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_function_configuration::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_function_configuration`]
module"]
#[doc(alias = "PHYSICAL_FUNCTION_CONFIGURATION")]
pub type PhysicalFunctionConfiguration =
    crate::Reg<physical_function_configuration::PhysicalFunctionConfigurationSpec>;
#[doc = "Physical Function Configuration Register Reserved"]
pub mod physical_function_configuration;
#[doc = "ROOT_COMPLEX_BAR_CONFIGURATION (rw) register accessor: Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_complex_bar_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_complex_bar_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_complex_bar_configuration`]
module"]
#[doc(alias = "ROOT_COMPLEX_BAR_CONFIGURATION")]
pub type RootComplexBarConfiguration =
    crate::Reg<root_complex_bar_configuration::RootComplexBarConfigurationSpec>;
#[doc = "Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
pub mod root_complex_bar_configuration;
