#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vendor_id: VendorId,
    product_id: ProductId,
    device_id: DeviceId,
    usbtypec_rev: UsbtypecRev,
    usbpd_rev_ver: UsbpdRevVer,
    pd_interface_rev: PdInterfaceRev,
    _reserved6: [u8; 0x08],
    alert: Alert,
    alert_mask: AlertMask,
    power_status_mask: PowerStatusMask,
    fault_status_mask: FaultStatusMask,
    _reserved10: [u8; 0x04],
    config_standard_output: ConfigStandardOutput,
    tcpc_control: TcpcControl,
    role_control: RoleControl,
    fault_control: FaultControl,
    power_control: PowerControl,
    cc_status: CcStatus,
    power_status: PowerStatus,
    fault_status: FaultStatus,
    _reserved18: [u8; 0x08],
    command: Command,
    device_capabilities_1: DeviceCapabilities1,
    device_capabilities_2: DeviceCapabilities2,
    standard_input_capabilities: StandardInputCapabilities,
    standard_output_capabilities: StandardOutputCapabilities,
    _reserved23: [u8; 0x0c],
    message_header_info: MessageHeaderInfo,
    receive_detect: ReceiveDetect,
    receive_byte_count: ReceiveByteCount,
    rx_buf_frame_type: RxBufFrameType,
    rx_buf_header_byte_10: RxBufHeaderByte10,
    rx_buf_obj_byte_3210: [RxBufObjByte3210; 7],
    transmit: Transmit,
    transmit_byte_count: TransmitByteCount,
    tx_buf_header_byte_10: TxBufHeaderByte10,
    tx_buf_obj_byte_3210: [TxBufObjByte3210; 7],
    vbus_voltage: VbusVoltage,
    vbus_sink_disconnect_threshold: VbusSinkDisconnectThreshold,
    vbus_stop_discharge_threshold: VbusStopDischargeThreshold,
    vbus_voltage_alarm_hi_cfg: VbusVoltageAlarmHiCfg,
    vbus_voltage_alarm_lo_cfg: VbusVoltageAlarmLoCfg,
    _reserved38: [u8; 0x0c],
    rx_err_cnt: RxErrCnt,
    phy_mux_ctrl: PhyMuxCtrl,
    cc_pd_test_dbg: CcPdTestDbg,
    sfc_reg_0: SfcReg0,
    sfc_reg_1: SfcReg1,
    sfc_reg_2: SfcReg2,
}
impl RegisterBlock {
    #[doc = "0x00 - Vendor ID Register"]
    #[inline(always)]
    pub const fn vendor_id(&self) -> &VendorId {
        &self.vendor_id
    }
    #[doc = "0x04 - Product ID Register"]
    #[inline(always)]
    pub const fn product_id(&self) -> &ProductId {
        &self.product_id
    }
    #[doc = "0x08 - Device ID Register"]
    #[inline(always)]
    pub const fn device_id(&self) -> &DeviceId {
        &self.device_id
    }
    #[doc = "0x0c - USB Type-C Revision Register"]
    #[inline(always)]
    pub const fn usbtypec_rev(&self) -> &UsbtypecRev {
        &self.usbtypec_rev
    }
    #[doc = "0x10 - USB PD Version Register"]
    #[inline(always)]
    pub const fn usbpd_rev_ver(&self) -> &UsbpdRevVer {
        &self.usbpd_rev_ver
    }
    #[doc = "0x14 - PD Interface Block Version Register"]
    #[inline(always)]
    pub const fn pd_interface_rev(&self) -> &PdInterfaceRev {
        &self.pd_interface_rev
    }
    #[doc = "0x20 - ALERT Status Register"]
    #[inline(always)]
    pub const fn alert(&self) -> &Alert {
        &self.alert
    }
    #[doc = "0x24 - ALERT Status Mask Register"]
    #[inline(always)]
    pub const fn alert_mask(&self) -> &AlertMask {
        &self.alert_mask
    }
    #[doc = "0x28 - Power Status Mask Register"]
    #[inline(always)]
    pub const fn power_status_mask(&self) -> &PowerStatusMask {
        &self.power_status_mask
    }
    #[doc = "0x2c - FAULT Status Mask Register"]
    #[inline(always)]
    pub const fn fault_status_mask(&self) -> &FaultStatusMask {
        &self.fault_status_mask
    }
    #[doc = "0x34 - Configure Standard Output Register"]
    #[inline(always)]
    pub const fn config_standard_output(&self) -> &ConfigStandardOutput {
        &self.config_standard_output
    }
    #[doc = "0x38 - TCPC Control Register"]
    #[inline(always)]
    pub const fn tcpc_control(&self) -> &TcpcControl {
        &self.tcpc_control
    }
    #[doc = "0x3c - Role Control Register"]
    #[inline(always)]
    pub const fn role_control(&self) -> &RoleControl {
        &self.role_control
    }
    #[doc = "0x40 - Fault Control Register"]
    #[inline(always)]
    pub const fn fault_control(&self) -> &FaultControl {
        &self.fault_control
    }
    #[doc = "0x44 - Power Control Register"]
    #[inline(always)]
    pub const fn power_control(&self) -> &PowerControl {
        &self.power_control
    }
    #[doc = "0x48 - CC Status Register"]
    #[inline(always)]
    pub const fn cc_status(&self) -> &CcStatus {
        &self.cc_status
    }
    #[doc = "0x4c - Power Status Register"]
    #[inline(always)]
    pub const fn power_status(&self) -> &PowerStatus {
        &self.power_status
    }
    #[doc = "0x50 - Fault Status Register"]
    #[inline(always)]
    pub const fn fault_status(&self) -> &FaultStatus {
        &self.fault_status
    }
    #[doc = "0x5c - Command Register"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x60 - Device Capabilities Register 1"]
    #[inline(always)]
    pub const fn device_capabilities_1(&self) -> &DeviceCapabilities1 {
        &self.device_capabilities_1
    }
    #[doc = "0x64 - Device Capabilities Register 2"]
    #[inline(always)]
    pub const fn device_capabilities_2(&self) -> &DeviceCapabilities2 {
        &self.device_capabilities_2
    }
    #[doc = "0x68 - Standard Input Capabilities Register"]
    #[inline(always)]
    pub const fn standard_input_capabilities(&self) -> &StandardInputCapabilities {
        &self.standard_input_capabilities
    }
    #[doc = "0x6c - Standard Output Capabilities Register"]
    #[inline(always)]
    pub const fn standard_output_capabilities(&self) -> &StandardOutputCapabilities {
        &self.standard_output_capabilities
    }
    #[doc = "0x7c - Message Header Information Register"]
    #[inline(always)]
    pub const fn message_header_info(&self) -> &MessageHeaderInfo {
        &self.message_header_info
    }
    #[doc = "0x80 - Receive Detect Register"]
    #[inline(always)]
    pub const fn receive_detect(&self) -> &ReceiveDetect {
        &self.receive_detect
    }
    #[doc = "0x84 - Receive Detect Register"]
    #[inline(always)]
    pub const fn receive_byte_count(&self) -> &ReceiveByteCount {
        &self.receive_byte_count
    }
    #[doc = "0x88 - RX Buffer Frame Type Register"]
    #[inline(always)]
    pub const fn rx_buf_frame_type(&self) -> &RxBufFrameType {
        &self.rx_buf_frame_type
    }
    #[doc = "0x8c - RX Buffer Message Header Byte Register"]
    #[inline(always)]
    pub const fn rx_buf_header_byte_10(&self) -> &RxBufHeaderByte10 {
        &self.rx_buf_header_byte_10
    }
    #[doc = "0x90..0xac - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj_byte_3210(&self, n: usize) -> &RxBufObjByte3210 {
        &self.rx_buf_obj_byte_3210[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xac - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub fn rx_buf_obj_byte_3210_iter(&self) -> impl Iterator<Item = &RxBufObjByte3210> {
        self.rx_buf_obj_byte_3210.iter()
    }
    #[doc = "0x90 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj1_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(0)
    }
    #[doc = "0x94 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj2_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(1)
    }
    #[doc = "0x98 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj3_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(2)
    }
    #[doc = "0x9c - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj4_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(3)
    }
    #[doc = "0xa0 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj5_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(4)
    }
    #[doc = "0xa4 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj6_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(5)
    }
    #[doc = "0xa8 - RX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn rx_buf_obj7_byte_3210(&self) -> &RxBufObjByte3210 {
        self.rx_buf_obj_byte_3210(6)
    }
    #[doc = "0xac - Transmit Register"]
    #[inline(always)]
    pub const fn transmit(&self) -> &Transmit {
        &self.transmit
    }
    #[doc = "0xb0 - Transmit Byte Count Register"]
    #[inline(always)]
    pub const fn transmit_byte_count(&self) -> &TransmitByteCount {
        &self.transmit_byte_count
    }
    #[doc = "0xb4 - Transmit Byte Count Register"]
    #[inline(always)]
    pub const fn tx_buf_header_byte_10(&self) -> &TxBufHeaderByte10 {
        &self.tx_buf_header_byte_10
    }
    #[doc = "0xb8..0xd4 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj_byte_3210(&self, n: usize) -> &TxBufObjByte3210 {
        &self.tx_buf_obj_byte_3210[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb8..0xd4 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub fn tx_buf_obj_byte_3210_iter(&self) -> impl Iterator<Item = &TxBufObjByte3210> {
        self.tx_buf_obj_byte_3210.iter()
    }
    #[doc = "0xb8 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj1_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(0)
    }
    #[doc = "0xbc - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj2_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(1)
    }
    #[doc = "0xc0 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj3_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(2)
    }
    #[doc = "0xc4 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj4_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(3)
    }
    #[doc = "0xc8 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj5_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(4)
    }
    #[doc = "0xcc - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj6_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(5)
    }
    #[doc = "0xd0 - TX Buffer OBJX Byte3~0 Register"]
    #[inline(always)]
    pub const fn tx_buf_obj7_byte_3210(&self) -> &TxBufObjByte3210 {
        self.tx_buf_obj_byte_3210(6)
    }
    #[doc = "0xd4 - Vbus Voltage Register"]
    #[inline(always)]
    pub const fn vbus_voltage(&self) -> &VbusVoltage {
        &self.vbus_voltage
    }
    #[doc = "0xd8 - Vbus Sink Disconnect Threshold Register"]
    #[inline(always)]
    pub const fn vbus_sink_disconnect_threshold(&self) -> &VbusSinkDisconnectThreshold {
        &self.vbus_sink_disconnect_threshold
    }
    #[doc = "0xdc - Vbus Stop Discharge Threshold Register"]
    #[inline(always)]
    pub const fn vbus_stop_discharge_threshold(&self) -> &VbusStopDischargeThreshold {
        &self.vbus_stop_discharge_threshold
    }
    #[doc = "0xe0 - Vbus Voltage Alarm High Cfg Register"]
    #[inline(always)]
    pub const fn vbus_voltage_alarm_hi_cfg(&self) -> &VbusVoltageAlarmHiCfg {
        &self.vbus_voltage_alarm_hi_cfg
    }
    #[doc = "0xe4 - Vbus Voltage Alarm Low Cfg Register"]
    #[inline(always)]
    pub const fn vbus_voltage_alarm_lo_cfg(&self) -> &VbusVoltageAlarmLoCfg {
        &self.vbus_voltage_alarm_lo_cfg
    }
    #[doc = "0xf4 - RX Error Counter Register"]
    #[inline(always)]
    pub const fn rx_err_cnt(&self) -> &RxErrCnt {
        &self.rx_err_cnt
    }
    #[doc = "0xf8 - PHY MUX Ctrl Register"]
    #[inline(always)]
    pub const fn phy_mux_ctrl(&self) -> &PhyMuxCtrl {
        &self.phy_mux_ctrl
    }
    #[doc = "0xfc - CC PD Test Debug Register"]
    #[inline(always)]
    pub const fn cc_pd_test_dbg(&self) -> &CcPdTestDbg {
        &self.cc_pd_test_dbg
    }
    #[doc = "0x100 - SFC Reg0 Register"]
    #[inline(always)]
    pub const fn sfc_reg_0(&self) -> &SfcReg0 {
        &self.sfc_reg_0
    }
    #[doc = "0x104 - SFC Reg1 Register"]
    #[inline(always)]
    pub const fn sfc_reg_1(&self) -> &SfcReg1 {
        &self.sfc_reg_1
    }
    #[doc = "0x108 - SFC Reg2 Register"]
    #[inline(always)]
    pub const fn sfc_reg_2(&self) -> &SfcReg2 {
        &self.sfc_reg_2
    }
}
#[doc = "VENDOR_ID (r) register accessor: Vendor ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor_id`]
module"]
#[doc(alias = "VENDOR_ID")]
pub type VendorId = crate::Reg<vendor_id::VendorIdSpec>;
#[doc = "Vendor ID Register"]
pub mod vendor_id;
#[doc = "PRODUCT_ID (r) register accessor: Product ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id`]
module"]
#[doc(alias = "PRODUCT_ID")]
pub type ProductId = crate::Reg<product_id::ProductIdSpec>;
#[doc = "Product ID Register"]
pub mod product_id;
#[doc = "DEVICE_ID (r) register accessor: Device ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id`]
module"]
#[doc(alias = "DEVICE_ID")]
pub type DeviceId = crate::Reg<device_id::DeviceIdSpec>;
#[doc = "Device ID Register"]
pub mod device_id;
#[doc = "USBTYPEC_REV (r) register accessor: USB Type-C Revision Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbtypec_rev::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbtypec_rev`]
module"]
#[doc(alias = "USBTYPEC_REV")]
pub type UsbtypecRev = crate::Reg<usbtypec_rev::UsbtypecRevSpec>;
#[doc = "USB Type-C Revision Register"]
pub mod usbtypec_rev;
#[doc = "USBPD_REV_VER (r) register accessor: USB PD Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpd_rev_ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpd_rev_ver`]
module"]
#[doc(alias = "USBPD_REV_VER")]
pub type UsbpdRevVer = crate::Reg<usbpd_rev_ver::UsbpdRevVerSpec>;
#[doc = "USB PD Version Register"]
pub mod usbpd_rev_ver;
#[doc = "PD_INTERFACE_REV (r) register accessor: PD Interface Block Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_interface_rev::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_interface_rev`]
module"]
#[doc(alias = "PD_INTERFACE_REV")]
pub type PdInterfaceRev = crate::Reg<pd_interface_rev::PdInterfaceRevSpec>;
#[doc = "PD Interface Block Version Register"]
pub mod pd_interface_rev;
#[doc = "ALERT (rw) register accessor: ALERT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alert::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alert::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alert`]
module"]
#[doc(alias = "ALERT")]
pub type Alert = crate::Reg<alert::AlertSpec>;
#[doc = "ALERT Status Register"]
pub mod alert;
#[doc = "ALERT_MASK (rw) register accessor: ALERT Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alert_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alert_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alert_mask`]
module"]
#[doc(alias = "ALERT_MASK")]
pub type AlertMask = crate::Reg<alert_mask::AlertMaskSpec>;
#[doc = "ALERT Status Mask Register"]
pub mod alert_mask;
#[doc = "POWER_STATUS_MASK (rw) register accessor: Power Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_status_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_status_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_status_mask`]
module"]
#[doc(alias = "POWER_STATUS_MASK")]
pub type PowerStatusMask = crate::Reg<power_status_mask::PowerStatusMaskSpec>;
#[doc = "Power Status Mask Register"]
pub mod power_status_mask;
#[doc = "FAULT_STATUS_MASK (rw) register accessor: FAULT Status Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_status_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_status_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_status_mask`]
module"]
#[doc(alias = "FAULT_STATUS_MASK")]
pub type FaultStatusMask = crate::Reg<fault_status_mask::FaultStatusMaskSpec>;
#[doc = "FAULT Status Mask Register"]
pub mod fault_status_mask;
#[doc = "CONFIG_STANDARD_OUTPUT (rw) register accessor: Configure Standard Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_standard_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_standard_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_standard_output`]
module"]
#[doc(alias = "CONFIG_STANDARD_OUTPUT")]
pub type ConfigStandardOutput = crate::Reg<config_standard_output::ConfigStandardOutputSpec>;
#[doc = "Configure Standard Output Register"]
pub mod config_standard_output;
#[doc = "TCPC_CONTROL (rw) register accessor: TCPC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcpc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcpc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcpc_control`]
module"]
#[doc(alias = "TCPC_CONTROL")]
pub type TcpcControl = crate::Reg<tcpc_control::TcpcControlSpec>;
#[doc = "TCPC Control Register"]
pub mod tcpc_control;
#[doc = "ROLE_CONTROL (rw) register accessor: Role Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`role_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`role_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@role_control`]
module"]
#[doc(alias = "ROLE_CONTROL")]
pub type RoleControl = crate::Reg<role_control::RoleControlSpec>;
#[doc = "Role Control Register"]
pub mod role_control;
#[doc = "FAULT_CONTROL (rw) register accessor: Fault Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_control`]
module"]
#[doc(alias = "FAULT_CONTROL")]
pub type FaultControl = crate::Reg<fault_control::FaultControlSpec>;
#[doc = "Fault Control Register"]
pub mod fault_control;
#[doc = "POWER_CONTROL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_control`]
module"]
#[doc(alias = "POWER_CONTROL")]
pub type PowerControl = crate::Reg<power_control::PowerControlSpec>;
#[doc = "Power Control Register"]
pub mod power_control;
#[doc = "CC_STATUS (r) register accessor: CC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_status`]
module"]
#[doc(alias = "CC_STATUS")]
pub type CcStatus = crate::Reg<cc_status::CcStatusSpec>;
#[doc = "CC Status Register"]
pub mod cc_status;
#[doc = "POWER_STATUS (r) register accessor: Power Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_status`]
module"]
#[doc(alias = "POWER_STATUS")]
pub type PowerStatus = crate::Reg<power_status::PowerStatusSpec>;
#[doc = "Power Status Register"]
pub mod power_status;
#[doc = "FAULT_STATUS (rw) register accessor: Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_status`]
module"]
#[doc(alias = "FAULT_STATUS")]
pub type FaultStatus = crate::Reg<fault_status::FaultStatusSpec>;
#[doc = "Fault Status Register"]
pub mod fault_status;
#[doc = "COMMAND (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command Register"]
pub mod command;
#[doc = "DEVICE_CAPABILITIES_1 (r) register accessor: Device Capabilities Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_capabilities_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_capabilities_1`]
module"]
#[doc(alias = "DEVICE_CAPABILITIES_1")]
pub type DeviceCapabilities1 = crate::Reg<device_capabilities_1::DeviceCapabilities1Spec>;
#[doc = "Device Capabilities Register 1"]
pub mod device_capabilities_1;
#[doc = "DEVICE_CAPABILITIES_2 (r) register accessor: Device Capabilities Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_capabilities_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_capabilities_2`]
module"]
#[doc(alias = "DEVICE_CAPABILITIES_2")]
pub type DeviceCapabilities2 = crate::Reg<device_capabilities_2::DeviceCapabilities2Spec>;
#[doc = "Device Capabilities Register 2"]
pub mod device_capabilities_2;
#[doc = "STANDARD_INPUT_CAPABILITIES (r) register accessor: Standard Input Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`standard_input_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@standard_input_capabilities`]
module"]
#[doc(alias = "STANDARD_INPUT_CAPABILITIES")]
pub type StandardInputCapabilities =
    crate::Reg<standard_input_capabilities::StandardInputCapabilitiesSpec>;
#[doc = "Standard Input Capabilities Register"]
pub mod standard_input_capabilities;
#[doc = "STANDARD_OUTPUT_CAPABILITIES (r) register accessor: Standard Output Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`standard_output_capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@standard_output_capabilities`]
module"]
#[doc(alias = "STANDARD_OUTPUT_CAPABILITIES")]
pub type StandardOutputCapabilities =
    crate::Reg<standard_output_capabilities::StandardOutputCapabilitiesSpec>;
#[doc = "Standard Output Capabilities Register"]
pub mod standard_output_capabilities;
#[doc = "MESSAGE_HEADER_INFO (rw) register accessor: Message Header Information Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`message_header_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`message_header_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_header_info`]
module"]
#[doc(alias = "MESSAGE_HEADER_INFO")]
pub type MessageHeaderInfo = crate::Reg<message_header_info::MessageHeaderInfoSpec>;
#[doc = "Message Header Information Register"]
pub mod message_header_info;
#[doc = "RECEIVE_DETECT (rw) register accessor: Receive Detect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_detect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_detect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_detect`]
module"]
#[doc(alias = "RECEIVE_DETECT")]
pub type ReceiveDetect = crate::Reg<receive_detect::ReceiveDetectSpec>;
#[doc = "Receive Detect Register"]
pub mod receive_detect;
#[doc = "RECEIVE_BYTE_COUNT (r) register accessor: Receive Detect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_byte_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_byte_count`]
module"]
#[doc(alias = "RECEIVE_BYTE_COUNT")]
pub type ReceiveByteCount = crate::Reg<receive_byte_count::ReceiveByteCountSpec>;
#[doc = "Receive Detect Register"]
pub mod receive_byte_count;
#[doc = "RX_BUF_FRAME_TYPE (r) register accessor: RX Buffer Frame Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_frame_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_buf_frame_type`]
module"]
#[doc(alias = "RX_BUF_FRAME_TYPE")]
pub type RxBufFrameType = crate::Reg<rx_buf_frame_type::RxBufFrameTypeSpec>;
#[doc = "RX Buffer Frame Type Register"]
pub mod rx_buf_frame_type;
#[doc = "RX_BUF_HEADER_BYTE_10 (r) register accessor: RX Buffer Message Header Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_header_byte_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_buf_header_byte_10`]
module"]
#[doc(alias = "RX_BUF_HEADER_BYTE_10")]
pub type RxBufHeaderByte10 = crate::Reg<rx_buf_header_byte_10::RxBufHeaderByte10Spec>;
#[doc = "RX Buffer Message Header Byte Register"]
pub mod rx_buf_header_byte_10;
#[doc = "RX_BUF_OBJ_BYTE_3210 (r) register accessor: RX Buffer OBJX Byte3~0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_obj_byte_3210::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_buf_obj_byte_3210`]
module"]
#[doc(alias = "RX_BUF_OBJ_BYTE_3210")]
pub type RxBufObjByte3210 = crate::Reg<rx_buf_obj_byte_3210::RxBufObjByte3210Spec>;
#[doc = "RX Buffer OBJX Byte3~0 Register"]
pub mod rx_buf_obj_byte_3210;
#[doc = "TRANSMIT (rw) register accessor: Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit`]
module"]
#[doc(alias = "TRANSMIT")]
pub type Transmit = crate::Reg<transmit::TransmitSpec>;
#[doc = "Transmit Register"]
pub mod transmit;
#[doc = "TRANSMIT_BYTE_COUNT (rw) register accessor: Transmit Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_byte_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_byte_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_byte_count`]
module"]
#[doc(alias = "TRANSMIT_BYTE_COUNT")]
pub type TransmitByteCount = crate::Reg<transmit_byte_count::TransmitByteCountSpec>;
#[doc = "Transmit Byte Count Register"]
pub mod transmit_byte_count;
#[doc = "TX_BUF_HEADER_BYTE_10 (rw) register accessor: Transmit Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_buf_header_byte_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_buf_header_byte_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_buf_header_byte_10`]
module"]
#[doc(alias = "TX_BUF_HEADER_BYTE_10")]
pub type TxBufHeaderByte10 = crate::Reg<tx_buf_header_byte_10::TxBufHeaderByte10Spec>;
#[doc = "Transmit Byte Count Register"]
pub mod tx_buf_header_byte_10;
#[doc = "TX_BUF_OBJ_BYTE_3210 (rw) register accessor: TX Buffer OBJX Byte3~0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_buf_obj_byte_3210::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_buf_obj_byte_3210::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_buf_obj_byte_3210`]
module"]
#[doc(alias = "TX_BUF_OBJ_BYTE_3210")]
pub type TxBufObjByte3210 = crate::Reg<tx_buf_obj_byte_3210::TxBufObjByte3210Spec>;
#[doc = "TX Buffer OBJX Byte3~0 Register"]
pub mod tx_buf_obj_byte_3210;
#[doc = "VBUS_VOLTAGE (r) register accessor: Vbus Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_voltage::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_voltage`]
module"]
#[doc(alias = "VBUS_VOLTAGE")]
pub type VbusVoltage = crate::Reg<vbus_voltage::VbusVoltageSpec>;
#[doc = "Vbus Voltage Register"]
pub mod vbus_voltage;
#[doc = "VBUS_SINK_DISCONNECT_THRESHOLD (rw) register accessor: Vbus Sink Disconnect Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_sink_disconnect_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_sink_disconnect_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_sink_disconnect_threshold`]
module"]
#[doc(alias = "VBUS_SINK_DISCONNECT_THRESHOLD")]
pub type VbusSinkDisconnectThreshold =
    crate::Reg<vbus_sink_disconnect_threshold::VbusSinkDisconnectThresholdSpec>;
#[doc = "Vbus Sink Disconnect Threshold Register"]
pub mod vbus_sink_disconnect_threshold;
#[doc = "VBUS_STOP_DISCHARGE_THRESHOLD (rw) register accessor: Vbus Stop Discharge Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_stop_discharge_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_stop_discharge_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_stop_discharge_threshold`]
module"]
#[doc(alias = "VBUS_STOP_DISCHARGE_THRESHOLD")]
pub type VbusStopDischargeThreshold =
    crate::Reg<vbus_stop_discharge_threshold::VbusStopDischargeThresholdSpec>;
#[doc = "Vbus Stop Discharge Threshold Register"]
pub mod vbus_stop_discharge_threshold;
#[doc = "VBUS_VOLTAGE_ALARM_HI_CFG (rw) register accessor: Vbus Voltage Alarm High Cfg Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_voltage_alarm_hi_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_voltage_alarm_hi_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_voltage_alarm_hi_cfg`]
module"]
#[doc(alias = "VBUS_VOLTAGE_ALARM_HI_CFG")]
pub type VbusVoltageAlarmHiCfg = crate::Reg<vbus_voltage_alarm_hi_cfg::VbusVoltageAlarmHiCfgSpec>;
#[doc = "Vbus Voltage Alarm High Cfg Register"]
pub mod vbus_voltage_alarm_hi_cfg;
#[doc = "VBUS_VOLTAGE_ALARM_LO_CFG (rw) register accessor: Vbus Voltage Alarm Low Cfg Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_voltage_alarm_lo_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_voltage_alarm_lo_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_voltage_alarm_lo_cfg`]
module"]
#[doc(alias = "VBUS_VOLTAGE_ALARM_LO_CFG")]
pub type VbusVoltageAlarmLoCfg = crate::Reg<vbus_voltage_alarm_lo_cfg::VbusVoltageAlarmLoCfgSpec>;
#[doc = "Vbus Voltage Alarm Low Cfg Register"]
pub mod vbus_voltage_alarm_lo_cfg;
#[doc = "RX_ERR_CNT (rw) register accessor: RX Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_cnt`]
module"]
#[doc(alias = "RX_ERR_CNT")]
pub type RxErrCnt = crate::Reg<rx_err_cnt::RxErrCntSpec>;
#[doc = "RX Error Counter Register"]
pub mod rx_err_cnt;
#[doc = "PHY_MUX_CTRL (rw) register accessor: PHY MUX Ctrl Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_mux_ctrl`]
module"]
#[doc(alias = "PHY_MUX_CTRL")]
pub type PhyMuxCtrl = crate::Reg<phy_mux_ctrl::PhyMuxCtrlSpec>;
#[doc = "PHY MUX Ctrl Register"]
pub mod phy_mux_ctrl;
#[doc = "CC_PD_TEST_DBG (rw) register accessor: CC PD Test Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_pd_test_dbg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_pd_test_dbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_pd_test_dbg`]
module"]
#[doc(alias = "CC_PD_TEST_DBG")]
pub type CcPdTestDbg = crate::Reg<cc_pd_test_dbg::CcPdTestDbgSpec>;
#[doc = "CC PD Test Debug Register"]
pub mod cc_pd_test_dbg;
#[doc = "SFC_REG_0 (rw) register accessor: SFC Reg0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfc_reg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfc_reg_0`]
module"]
#[doc(alias = "SFC_REG_0")]
pub type SfcReg0 = crate::Reg<sfc_reg_0::SfcReg0Spec>;
#[doc = "SFC Reg0 Register"]
pub mod sfc_reg_0;
#[doc = "SFC_REG_1 (rw) register accessor: SFC Reg1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfc_reg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfc_reg_1`]
module"]
#[doc(alias = "SFC_REG_1")]
pub type SfcReg1 = crate::Reg<sfc_reg_1::SfcReg1Spec>;
#[doc = "SFC Reg1 Register"]
pub mod sfc_reg_1;
#[doc = "SFC_REG_2 (r) register accessor: SFC Reg2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfc_reg_2`]
module"]
#[doc(alias = "SFC_REG_2")]
pub type SfcReg2 = crate::Reg<sfc_reg_2::SfcReg2Spec>;
#[doc = "SFC Reg2 Register"]
pub mod sfc_reg_2;
