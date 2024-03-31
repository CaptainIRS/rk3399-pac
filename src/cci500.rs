#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_ctrl: SysCtrl,
    _reserved1: [u8; 0x04],
    secure_ctrl: SecureCtrl,
    status: Status,
    error_status: ErrorStatus,
    _reserved4: [u8; 0xec],
    pfmmon_ctrl: PfmmonCtrl,
    interface_monitor_ctrl: InterfaceMonitorCtrl,
    _reserved6: [u8; 0x0ef8],
    snoop_ctrl_s0: SnoopCtrlS0,
    shareable_override_s0: ShareableOverrideS0,
    _reserved8: [u8; 0xf8],
    rd_chan_qos_override_s0: RdChanQosOverrideS0,
    wr_chan_qos_override_s0: WrChanQosOverrideS0,
    _reserved10: [u8; 0x08],
    max_ot_s0: MaxOtS0,
    _reserved11: [u8; 0x0eec],
    snoop_ctrl_s1: SnoopCtrlS1,
    shareable_override_s1: ShareableOverrideS1,
    _reserved13: [u8; 0xf8],
    rd_chan_qos_override_s1: RdChanQosOverrideS1,
    wr_chan_qos_override_s1: WrChanQosOverrideS1,
    _reserved15: [u8; 0x08],
    max_ot_s1: MaxOtS1,
    _reserved16: [u8; 0x0008_deec],
    slave_interface_monitor_s0: SlaveInterfaceMonitorS0,
    slave_interface_monitor_s1: SlaveInterfaceMonitorS1,
    _reserved18: [u8; 0xf8],
    master_interface_monitor_m0: MasterInterfaceMonitorM0,
    master_interface_monitor_m1: MasterInterfaceMonitorM1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Override Register"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SysCtrl {
        &self.sys_ctrl
    }
    #[doc = "0x08 - Secure Access Register"]
    #[inline(always)]
    pub const fn secure_ctrl(&self) -> &SecureCtrl {
        &self.secure_ctrl
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Imprecise Error Register"]
    #[inline(always)]
    pub const fn error_status(&self) -> &ErrorStatus {
        &self.error_status
    }
    #[doc = "0x100 - Performance Monitor Control Register (PMCR)"]
    #[inline(always)]
    pub const fn pfmmon_ctrl(&self) -> &PfmmonCtrl {
        &self.pfmmon_ctrl
    }
    #[doc = "0x104 - Snoop Control Register for slave interface x"]
    #[inline(always)]
    pub const fn interface_monitor_ctrl(&self) -> &InterfaceMonitorCtrl {
        &self.interface_monitor_ctrl
    }
    #[doc = "0x1000 - "]
    #[inline(always)]
    pub const fn snoop_ctrl_s0(&self) -> &SnoopCtrlS0 {
        &self.snoop_ctrl_s0
    }
    #[doc = "0x1004 - Shareable Override Register"]
    #[inline(always)]
    pub const fn shareable_override_s0(&self) -> &ShareableOverrideS0 {
        &self.shareable_override_s0
    }
    #[doc = "0x1100 - Read Channel QoS Value Override Register for slave interface x"]
    #[inline(always)]
    pub const fn rd_chan_qos_override_s0(&self) -> &RdChanQosOverrideS0 {
        &self.rd_chan_qos_override_s0
    }
    #[doc = "0x1104 - Write Channel QoS Value Override slave interface x"]
    #[inline(always)]
    pub const fn wr_chan_qos_override_s0(&self) -> &WrChanQosOverrideS0 {
        &self.wr_chan_qos_override_s0
    }
    #[doc = "0x1110 - Maximum Outstanding Transactions Registers"]
    #[inline(always)]
    pub const fn max_ot_s0(&self) -> &MaxOtS0 {
        &self.max_ot_s0
    }
    #[doc = "0x2000 - "]
    #[inline(always)]
    pub const fn snoop_ctrl_s1(&self) -> &SnoopCtrlS1 {
        &self.snoop_ctrl_s1
    }
    #[doc = "0x2004 - Shareable Override Register"]
    #[inline(always)]
    pub const fn shareable_override_s1(&self) -> &ShareableOverrideS1 {
        &self.shareable_override_s1
    }
    #[doc = "0x2100 - Read Channel QoS Value Override Register for slave interface x"]
    #[inline(always)]
    pub const fn rd_chan_qos_override_s1(&self) -> &RdChanQosOverrideS1 {
        &self.rd_chan_qos_override_s1
    }
    #[doc = "0x2104 - Write Channel QoS Value Override slave interface x"]
    #[inline(always)]
    pub const fn wr_chan_qos_override_s1(&self) -> &WrChanQosOverrideS1 {
        &self.wr_chan_qos_override_s1
    }
    #[doc = "0x2110 - Maximum Outstanding Transactions Registers"]
    #[inline(always)]
    pub const fn max_ot_s1(&self) -> &MaxOtS1 {
        &self.max_ot_s1
    }
    #[doc = "0x90000 - Slave Interface Monitor Registers"]
    #[inline(always)]
    pub const fn slave_interface_monitor_s0(&self) -> &SlaveInterfaceMonitorS0 {
        &self.slave_interface_monitor_s0
    }
    #[doc = "0x90004 - Slave Interface Monitor Registers"]
    #[inline(always)]
    pub const fn slave_interface_monitor_s1(&self) -> &SlaveInterfaceMonitorS1 {
        &self.slave_interface_monitor_s1
    }
    #[doc = "0x90100 - Master Interface Monitor Registers"]
    #[inline(always)]
    pub const fn master_interface_monitor_m0(&self) -> &MasterInterfaceMonitorM0 {
        &self.master_interface_monitor_m0
    }
    #[doc = "0x90104 - Master Interface Monitor Registers"]
    #[inline(always)]
    pub const fn master_interface_monitor_m1(&self) -> &MasterInterfaceMonitorM1 {
        &self.master_interface_monitor_m1
    }
}
#[doc = "SYS_CTRL (rw) register accessor: Control Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`]
module"]
#[doc(alias = "SYS_CTRL")]
pub type SysCtrl = crate::Reg<sys_ctrl::SysCtrlSpec>;
#[doc = "Control Override Register"]
pub mod sys_ctrl;
#[doc = "SECURE_CTRL (rw) register accessor: Secure Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_ctrl`]
module"]
#[doc(alias = "SECURE_CTRL")]
pub type SecureCtrl = crate::Reg<secure_ctrl::SecureCtrlSpec>;
#[doc = "Secure Access Register"]
pub mod secure_ctrl;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ERROR_STATUS (rw) register accessor: Imprecise Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status`]
module"]
#[doc(alias = "ERROR_STATUS")]
pub type ErrorStatus = crate::Reg<error_status::ErrorStatusSpec>;
#[doc = "Imprecise Error Register"]
pub mod error_status;
#[doc = "PFMMON_CTRL (rw) register accessor: Performance Monitor Control Register (PMCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfmmon_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfmmon_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfmmon_ctrl`]
module"]
#[doc(alias = "PFMMON_CTRL")]
pub type PfmmonCtrl = crate::Reg<pfmmon_ctrl::PfmmonCtrlSpec>;
#[doc = "Performance Monitor Control Register (PMCR)"]
pub mod pfmmon_ctrl;
#[doc = "INTERFACE_MONITOR_CTRL (rw) register accessor: Snoop Control Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_monitor_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interface_monitor_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_monitor_ctrl`]
module"]
#[doc(alias = "INTERFACE_MONITOR_CTRL")]
pub type InterfaceMonitorCtrl = crate::Reg<interface_monitor_ctrl::InterfaceMonitorCtrlSpec>;
#[doc = "Snoop Control Register for slave interface x"]
pub mod interface_monitor_ctrl;
#[doc = "SNOOP_CTRL_S0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snoop_ctrl_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snoop_ctrl_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snoop_ctrl_s0`]
module"]
#[doc(alias = "SNOOP_CTRL_S0")]
pub type SnoopCtrlS0 = crate::Reg<snoop_ctrl_s0::SnoopCtrlS0Spec>;
#[doc = ""]
pub mod snoop_ctrl_s0;
#[doc = "SHAREABLE_OVERRIDE_S0 (rw) register accessor: Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shareable_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shareable_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shareable_override_s0`]
module"]
#[doc(alias = "SHAREABLE_OVERRIDE_S0")]
pub type ShareableOverrideS0 = crate::Reg<shareable_override_s0::ShareableOverrideS0Spec>;
#[doc = "Shareable Override Register"]
pub mod shareable_override_s0;
#[doc = "RD_CHAN_QOS_OVERRIDE_S0 (rw) register accessor: Read Channel QoS Value Override Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_chan_qos_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_chan_qos_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_chan_qos_override_s0`]
module"]
#[doc(alias = "RD_CHAN_QOS_OVERRIDE_S0")]
pub type RdChanQosOverrideS0 = crate::Reg<rd_chan_qos_override_s0::RdChanQosOverrideS0Spec>;
#[doc = "Read Channel QoS Value Override Register for slave interface x"]
pub mod rd_chan_qos_override_s0;
#[doc = "WR_CHAN_QOS_OVERRIDE_S0 (rw) register accessor: Write Channel QoS Value Override slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_chan_qos_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_chan_qos_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_chan_qos_override_s0`]
module"]
#[doc(alias = "WR_CHAN_QOS_OVERRIDE_S0")]
pub type WrChanQosOverrideS0 = crate::Reg<wr_chan_qos_override_s0::WrChanQosOverrideS0Spec>;
#[doc = "Write Channel QoS Value Override slave interface x"]
pub mod wr_chan_qos_override_s0;
#[doc = "MAX_OT_S0 (rw) register accessor: Maximum Outstanding Transactions Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_ot_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`max_ot_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_ot_s0`]
module"]
#[doc(alias = "MAX_OT_S0")]
pub type MaxOtS0 = crate::Reg<max_ot_s0::MaxOtS0Spec>;
#[doc = "Maximum Outstanding Transactions Registers"]
pub mod max_ot_s0;
#[doc = "SNOOP_CTRL_S1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snoop_ctrl_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snoop_ctrl_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snoop_ctrl_s1`]
module"]
#[doc(alias = "SNOOP_CTRL_S1")]
pub type SnoopCtrlS1 = crate::Reg<snoop_ctrl_s1::SnoopCtrlS1Spec>;
#[doc = ""]
pub mod snoop_ctrl_s1;
#[doc = "SHAREABLE_OVERRIDE_S1 (rw) register accessor: Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shareable_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shareable_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shareable_override_s1`]
module"]
#[doc(alias = "SHAREABLE_OVERRIDE_S1")]
pub type ShareableOverrideS1 = crate::Reg<shareable_override_s1::ShareableOverrideS1Spec>;
#[doc = "Shareable Override Register"]
pub mod shareable_override_s1;
#[doc = "RD_CHAN_QOS_OVERRIDE_S1 (rw) register accessor: Read Channel QoS Value Override Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_chan_qos_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_chan_qos_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_chan_qos_override_s1`]
module"]
#[doc(alias = "RD_CHAN_QOS_OVERRIDE_S1")]
pub type RdChanQosOverrideS1 = crate::Reg<rd_chan_qos_override_s1::RdChanQosOverrideS1Spec>;
#[doc = "Read Channel QoS Value Override Register for slave interface x"]
pub mod rd_chan_qos_override_s1;
#[doc = "WR_CHAN_QOS_OVERRIDE_S1 (rw) register accessor: Write Channel QoS Value Override slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_chan_qos_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_chan_qos_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_chan_qos_override_s1`]
module"]
#[doc(alias = "WR_CHAN_QOS_OVERRIDE_S1")]
pub type WrChanQosOverrideS1 = crate::Reg<wr_chan_qos_override_s1::WrChanQosOverrideS1Spec>;
#[doc = "Write Channel QoS Value Override slave interface x"]
pub mod wr_chan_qos_override_s1;
#[doc = "MAX_OT_S1 (rw) register accessor: Maximum Outstanding Transactions Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_ot_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`max_ot_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_ot_s1`]
module"]
#[doc(alias = "MAX_OT_S1")]
pub type MaxOtS1 = crate::Reg<max_ot_s1::MaxOtS1Spec>;
#[doc = "Maximum Outstanding Transactions Registers"]
pub mod max_ot_s1;
#[doc = "SLAVE_INTERFACE_MONITOR_S0 (rw) register accessor: Slave Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_interface_monitor_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_interface_monitor_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_interface_monitor_s0`]
module"]
#[doc(alias = "SLAVE_INTERFACE_MONITOR_S0")]
pub type SlaveInterfaceMonitorS0 =
    crate::Reg<slave_interface_monitor_s0::SlaveInterfaceMonitorS0Spec>;
#[doc = "Slave Interface Monitor Registers"]
pub mod slave_interface_monitor_s0;
#[doc = "SLAVE_INTERFACE_MONITOR_S1 (rw) register accessor: Slave Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_interface_monitor_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_interface_monitor_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_interface_monitor_s1`]
module"]
#[doc(alias = "SLAVE_INTERFACE_MONITOR_S1")]
pub type SlaveInterfaceMonitorS1 =
    crate::Reg<slave_interface_monitor_s1::SlaveInterfaceMonitorS1Spec>;
#[doc = "Slave Interface Monitor Registers"]
pub mod slave_interface_monitor_s1;
#[doc = "MASTER_INTERFACE_MONITOR_M0 (rw) register accessor: Master Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`master_interface_monitor_m0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`master_interface_monitor_m0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_interface_monitor_m0`]
module"]
#[doc(alias = "MASTER_INTERFACE_MONITOR_M0")]
pub type MasterInterfaceMonitorM0 =
    crate::Reg<master_interface_monitor_m0::MasterInterfaceMonitorM0Spec>;
#[doc = "Master Interface Monitor Registers"]
pub mod master_interface_monitor_m0;
#[doc = "MASTER_INTERFACE_MONITOR_M1 (rw) register accessor: Master Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`master_interface_monitor_m1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`master_interface_monitor_m1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_interface_monitor_m1`]
module"]
#[doc(alias = "MASTER_INTERFACE_MONITOR_M1")]
pub type MasterInterfaceMonitorM1 =
    crate::Reg<master_interface_monitor_m1::MasterInterfaceMonitorM1Spec>;
#[doc = "Master Interface Monitor Registers"]
pub mod master_interface_monitor_m1;
