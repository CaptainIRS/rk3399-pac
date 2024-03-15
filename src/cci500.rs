#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cci500_sys_ctrl: Cci500SysCtrl,
    _reserved1: [u8; 0x04],
    cci500_secure_ctrl: Cci500SecureCtrl,
    cci500_status: Cci500Status,
    cci500_error_status: Cci500ErrorStatus,
    _reserved4: [u8; 0xec],
    cci500_pfmmon_ctrl: Cci500PfmmonCtrl,
    cci500_interface_monitor_ctrl: Cci500InterfaceMonitorCtrl,
    _reserved6: [u8; 0x0ef8],
    cci500_snoop_ctrl_s0: Cci500SnoopCtrlS0,
    cci500_shareable_override_s0: Cci500ShareableOverrideS0,
    _reserved8: [u8; 0xf8],
    cci500_rd_chan_qos_override_s0: Cci500RdChanQosOverrideS0,
    cci500_wr_chan_qos_override_s0: Cci500WrChanQosOverrideS0,
    _reserved10: [u8; 0x08],
    cci500_max_ot_s0: Cci500MaxOtS0,
    _reserved11: [u8; 0x0eec],
    cci500_snoop_ctrl_s1: Cci500SnoopCtrlS1,
    cci500_shareable_override_s1: Cci500ShareableOverrideS1,
    _reserved13: [u8; 0xf8],
    cci500_rd_chan_qos_override_s1: Cci500RdChanQosOverrideS1,
    cci500_wr_chan_qos_override_s1: Cci500WrChanQosOverrideS1,
    _reserved15: [u8; 0x08],
    cci500_max_ot_s1: Cci500MaxOtS1,
    _reserved16: [u8; 0x0008_deec],
    cci500_slave_interface_monitor_s0: Cci500SlaveInterfaceMonitorS0,
    cci500_slave_interface_monitor_s1: Cci500SlaveInterfaceMonitorS1,
    _reserved18: [u8; 0xf8],
    cci500_master_interface_monitor_m0: Cci500MasterInterfaceMonitorM0,
    cci500_master_interface_monitor_m1: Cci500MasterInterfaceMonitorM1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Override Register"]
    #[inline(always)]
    pub const fn cci500_sys_ctrl(&self) -> &Cci500SysCtrl {
        &self.cci500_sys_ctrl
    }
    #[doc = "0x08 - Secure Access Register"]
    #[inline(always)]
    pub const fn cci500_secure_ctrl(&self) -> &Cci500SecureCtrl {
        &self.cci500_secure_ctrl
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn cci500_status(&self) -> &Cci500Status {
        &self.cci500_status
    }
    #[doc = "0x10 - Imprecise Error Register"]
    #[inline(always)]
    pub const fn cci500_error_status(&self) -> &Cci500ErrorStatus {
        &self.cci500_error_status
    }
    #[doc = "0x100 - Performance Monitor Control Register (PMCR)"]
    #[inline(always)]
    pub const fn cci500_pfmmon_ctrl(&self) -> &Cci500PfmmonCtrl {
        &self.cci500_pfmmon_ctrl
    }
    #[doc = "0x104 - Snoop Control Register for slave interface x"]
    #[inline(always)]
    pub const fn cci500_interface_monitor_ctrl(&self) -> &Cci500InterfaceMonitorCtrl {
        &self.cci500_interface_monitor_ctrl
    }
    #[doc = "0x1000 - "]
    #[inline(always)]
    pub const fn cci500_snoop_ctrl_s0(&self) -> &Cci500SnoopCtrlS0 {
        &self.cci500_snoop_ctrl_s0
    }
    #[doc = "0x1004 - Shareable Override Register"]
    #[inline(always)]
    pub const fn cci500_shareable_override_s0(&self) -> &Cci500ShareableOverrideS0 {
        &self.cci500_shareable_override_s0
    }
    #[doc = "0x1100 - Read Channel QoS Value Override Register for slave interface x"]
    #[inline(always)]
    pub const fn cci500_rd_chan_qos_override_s0(&self) -> &Cci500RdChanQosOverrideS0 {
        &self.cci500_rd_chan_qos_override_s0
    }
    #[doc = "0x1104 - Write Channel QoS Value Override slave interface x"]
    #[inline(always)]
    pub const fn cci500_wr_chan_qos_override_s0(&self) -> &Cci500WrChanQosOverrideS0 {
        &self.cci500_wr_chan_qos_override_s0
    }
    #[doc = "0x1110 - Maximum Outstanding Transactions Registers"]
    #[inline(always)]
    pub const fn cci500_max_ot_s0(&self) -> &Cci500MaxOtS0 {
        &self.cci500_max_ot_s0
    }
    #[doc = "0x2000 - "]
    #[inline(always)]
    pub const fn cci500_snoop_ctrl_s1(&self) -> &Cci500SnoopCtrlS1 {
        &self.cci500_snoop_ctrl_s1
    }
    #[doc = "0x2004 - Shareable Override Register"]
    #[inline(always)]
    pub const fn cci500_shareable_override_s1(&self) -> &Cci500ShareableOverrideS1 {
        &self.cci500_shareable_override_s1
    }
    #[doc = "0x2100 - Read Channel QoS Value Override Register for slave interface x"]
    #[inline(always)]
    pub const fn cci500_rd_chan_qos_override_s1(&self) -> &Cci500RdChanQosOverrideS1 {
        &self.cci500_rd_chan_qos_override_s1
    }
    #[doc = "0x2104 - Write Channel QoS Value Override slave interface x"]
    #[inline(always)]
    pub const fn cci500_wr_chan_qos_override_s1(&self) -> &Cci500WrChanQosOverrideS1 {
        &self.cci500_wr_chan_qos_override_s1
    }
    #[doc = "0x2110 - Maximum Outstanding Transactions Registers"]
    #[inline(always)]
    pub const fn cci500_max_ot_s1(&self) -> &Cci500MaxOtS1 {
        &self.cci500_max_ot_s1
    }
    #[doc = "0x90000 - Slave Interface Monitor Registers"]
    #[inline(always)]
    pub const fn cci500_slave_interface_monitor_s0(&self) -> &Cci500SlaveInterfaceMonitorS0 {
        &self.cci500_slave_interface_monitor_s0
    }
    #[doc = "0x90004 - Slave Interface Monitor Registers"]
    #[inline(always)]
    pub const fn cci500_slave_interface_monitor_s1(&self) -> &Cci500SlaveInterfaceMonitorS1 {
        &self.cci500_slave_interface_monitor_s1
    }
    #[doc = "0x90100 - Master Interface Monitor Registers"]
    #[inline(always)]
    pub const fn cci500_master_interface_monitor_m0(&self) -> &Cci500MasterInterfaceMonitorM0 {
        &self.cci500_master_interface_monitor_m0
    }
    #[doc = "0x90104 - Master Interface Monitor Registers"]
    #[inline(always)]
    pub const fn cci500_master_interface_monitor_m1(&self) -> &Cci500MasterInterfaceMonitorM1 {
        &self.cci500_master_interface_monitor_m1
    }
}
#[doc = "CCI500_SYS_CTRL (rw) register accessor: Control Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_sys_ctrl`]
module"]
#[doc(alias = "CCI500_SYS_CTRL")]
pub type Cci500SysCtrl = crate::Reg<cci500_sys_ctrl::Cci500SysCtrlSpec>;
#[doc = "Control Override Register"]
pub mod cci500_sys_ctrl;
#[doc = "CCI500_SECURE_CTRL (rw) register accessor: Secure Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_secure_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_secure_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_secure_ctrl`]
module"]
#[doc(alias = "CCI500_SECURE_CTRL")]
pub type Cci500SecureCtrl = crate::Reg<cci500_secure_ctrl::Cci500SecureCtrlSpec>;
#[doc = "Secure Access Register"]
pub mod cci500_secure_ctrl;
#[doc = "CCI500_STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_status`]
module"]
#[doc(alias = "CCI500_STATUS")]
pub type Cci500Status = crate::Reg<cci500_status::Cci500StatusSpec>;
#[doc = "Status Register"]
pub mod cci500_status;
#[doc = "CCI500_ERROR_STATUS (rw) register accessor: Imprecise Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_error_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_error_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_error_status`]
module"]
#[doc(alias = "CCI500_ERROR_STATUS")]
pub type Cci500ErrorStatus = crate::Reg<cci500_error_status::Cci500ErrorStatusSpec>;
#[doc = "Imprecise Error Register"]
pub mod cci500_error_status;
#[doc = "CCI500_PFMMON_CTRL (rw) register accessor: Performance Monitor Control Register (PMCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_pfmmon_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_pfmmon_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_pfmmon_ctrl`]
module"]
#[doc(alias = "CCI500_PFMMON_CTRL")]
pub type Cci500PfmmonCtrl = crate::Reg<cci500_pfmmon_ctrl::Cci500PfmmonCtrlSpec>;
#[doc = "Performance Monitor Control Register (PMCR)"]
pub mod cci500_pfmmon_ctrl;
#[doc = "CCI500_INTERFACE_MONITOR_CTRL (rw) register accessor: Snoop Control Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_interface_monitor_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_interface_monitor_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_interface_monitor_ctrl`]
module"]
#[doc(alias = "CCI500_INTERFACE_MONITOR_CTRL")]
pub type Cci500InterfaceMonitorCtrl =
    crate::Reg<cci500_interface_monitor_ctrl::Cci500InterfaceMonitorCtrlSpec>;
#[doc = "Snoop Control Register for slave interface x"]
pub mod cci500_interface_monitor_ctrl;
#[doc = "CCI500_SNOOP_CTRL_S0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_snoop_ctrl_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_snoop_ctrl_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_snoop_ctrl_s0`]
module"]
#[doc(alias = "CCI500_SNOOP_CTRL_S0")]
pub type Cci500SnoopCtrlS0 = crate::Reg<cci500_snoop_ctrl_s0::Cci500SnoopCtrlS0Spec>;
#[doc = ""]
pub mod cci500_snoop_ctrl_s0;
#[doc = "CCI500_SHAREABLE_OVERRIDE_S0 (rw) register accessor: Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_shareable_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_shareable_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_shareable_override_s0`]
module"]
#[doc(alias = "CCI500_SHAREABLE_OVERRIDE_S0")]
pub type Cci500ShareableOverrideS0 =
    crate::Reg<cci500_shareable_override_s0::Cci500ShareableOverrideS0Spec>;
#[doc = "Shareable Override Register"]
pub mod cci500_shareable_override_s0;
#[doc = "CCI500_RD_CHAN_QOS_OVERRIDE_S0 (rw) register accessor: Read Channel QoS Value Override Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_rd_chan_qos_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_rd_chan_qos_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_rd_chan_qos_override_s0`]
module"]
#[doc(alias = "CCI500_RD_CHAN_QOS_OVERRIDE_S0")]
pub type Cci500RdChanQosOverrideS0 =
    crate::Reg<cci500_rd_chan_qos_override_s0::Cci500RdChanQosOverrideS0Spec>;
#[doc = "Read Channel QoS Value Override Register for slave interface x"]
pub mod cci500_rd_chan_qos_override_s0;
#[doc = "CCI500_WR_CHAN_QOS_OVERRIDE_S0 (rw) register accessor: Write Channel QoS Value Override slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_wr_chan_qos_override_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_wr_chan_qos_override_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_wr_chan_qos_override_s0`]
module"]
#[doc(alias = "CCI500_WR_CHAN_QOS_OVERRIDE_S0")]
pub type Cci500WrChanQosOverrideS0 =
    crate::Reg<cci500_wr_chan_qos_override_s0::Cci500WrChanQosOverrideS0Spec>;
#[doc = "Write Channel QoS Value Override slave interface x"]
pub mod cci500_wr_chan_qos_override_s0;
#[doc = "CCI500_MAX_OT_S0 (rw) register accessor: Maximum Outstanding Transactions Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_max_ot_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_max_ot_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_max_ot_s0`]
module"]
#[doc(alias = "CCI500_MAX_OT_S0")]
pub type Cci500MaxOtS0 = crate::Reg<cci500_max_ot_s0::Cci500MaxOtS0Spec>;
#[doc = "Maximum Outstanding Transactions Registers"]
pub mod cci500_max_ot_s0;
#[doc = "CCI500_SNOOP_CTRL_S1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_snoop_ctrl_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_snoop_ctrl_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_snoop_ctrl_s1`]
module"]
#[doc(alias = "CCI500_SNOOP_CTRL_S1")]
pub type Cci500SnoopCtrlS1 = crate::Reg<cci500_snoop_ctrl_s1::Cci500SnoopCtrlS1Spec>;
#[doc = ""]
pub mod cci500_snoop_ctrl_s1;
#[doc = "CCI500_SHAREABLE_OVERRIDE_S1 (rw) register accessor: Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_shareable_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_shareable_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_shareable_override_s1`]
module"]
#[doc(alias = "CCI500_SHAREABLE_OVERRIDE_S1")]
pub type Cci500ShareableOverrideS1 =
    crate::Reg<cci500_shareable_override_s1::Cci500ShareableOverrideS1Spec>;
#[doc = "Shareable Override Register"]
pub mod cci500_shareable_override_s1;
#[doc = "CCI500_RD_CHAN_QOS_OVERRIDE_S1 (rw) register accessor: Read Channel QoS Value Override Register for slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_rd_chan_qos_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_rd_chan_qos_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_rd_chan_qos_override_s1`]
module"]
#[doc(alias = "CCI500_RD_CHAN_QOS_OVERRIDE_S1")]
pub type Cci500RdChanQosOverrideS1 =
    crate::Reg<cci500_rd_chan_qos_override_s1::Cci500RdChanQosOverrideS1Spec>;
#[doc = "Read Channel QoS Value Override Register for slave interface x"]
pub mod cci500_rd_chan_qos_override_s1;
#[doc = "CCI500_WR_CHAN_QOS_OVERRIDE_S1 (rw) register accessor: Write Channel QoS Value Override slave interface x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_wr_chan_qos_override_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_wr_chan_qos_override_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_wr_chan_qos_override_s1`]
module"]
#[doc(alias = "CCI500_WR_CHAN_QOS_OVERRIDE_S1")]
pub type Cci500WrChanQosOverrideS1 =
    crate::Reg<cci500_wr_chan_qos_override_s1::Cci500WrChanQosOverrideS1Spec>;
#[doc = "Write Channel QoS Value Override slave interface x"]
pub mod cci500_wr_chan_qos_override_s1;
#[doc = "CCI500_MAX_OT_S1 (rw) register accessor: Maximum Outstanding Transactions Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_max_ot_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_max_ot_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_max_ot_s1`]
module"]
#[doc(alias = "CCI500_MAX_OT_S1")]
pub type Cci500MaxOtS1 = crate::Reg<cci500_max_ot_s1::Cci500MaxOtS1Spec>;
#[doc = "Maximum Outstanding Transactions Registers"]
pub mod cci500_max_ot_s1;
#[doc = "CCI500_SLAVE_INTERFACE_MONITOR_S0 (rw) register accessor: Slave Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_slave_interface_monitor_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_slave_interface_monitor_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_slave_interface_monitor_s0`]
module"]
#[doc(alias = "CCI500_SLAVE_INTERFACE_MONITOR_S0")]
pub type Cci500SlaveInterfaceMonitorS0 =
    crate::Reg<cci500_slave_interface_monitor_s0::Cci500SlaveInterfaceMonitorS0Spec>;
#[doc = "Slave Interface Monitor Registers"]
pub mod cci500_slave_interface_monitor_s0;
#[doc = "CCI500_SLAVE_INTERFACE_MONITOR_S1 (rw) register accessor: Slave Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_slave_interface_monitor_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_slave_interface_monitor_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_slave_interface_monitor_s1`]
module"]
#[doc(alias = "CCI500_SLAVE_INTERFACE_MONITOR_S1")]
pub type Cci500SlaveInterfaceMonitorS1 =
    crate::Reg<cci500_slave_interface_monitor_s1::Cci500SlaveInterfaceMonitorS1Spec>;
#[doc = "Slave Interface Monitor Registers"]
pub mod cci500_slave_interface_monitor_s1;
#[doc = "CCI500_MASTER_INTERFACE_MONITOR_M0 (rw) register accessor: Master Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_master_interface_monitor_m0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_master_interface_monitor_m0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_master_interface_monitor_m0`]
module"]
#[doc(alias = "CCI500_MASTER_INTERFACE_MONITOR_M0")]
pub type Cci500MasterInterfaceMonitorM0 =
    crate::Reg<cci500_master_interface_monitor_m0::Cci500MasterInterfaceMonitorM0Spec>;
#[doc = "Master Interface Monitor Registers"]
pub mod cci500_master_interface_monitor_m0;
#[doc = "CCI500_MASTER_INTERFACE_MONITOR_M1 (rw) register accessor: Master Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_master_interface_monitor_m1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_master_interface_monitor_m1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_master_interface_monitor_m1`]
module"]
#[doc(alias = "CCI500_MASTER_INTERFACE_MONITOR_M1")]
pub type Cci500MasterInterfaceMonitorM1 =
    crate::Reg<cci500_master_interface_monitor_m1::Cci500MasterInterfaceMonitorM1Spec>;
#[doc = "Master Interface Monitor Registers"]
pub mod cci500_master_interface_monitor_m1;
