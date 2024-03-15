#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmu_wakeup_cfg0: PmuWakeupCfg0,
    pmu_wakeup_cfg1: PmuWakeupCfg1,
    pmu_wakeup_cfg2: PmuWakeupCfg2,
    pmu_wakeup_cfg3: PmuWakeupCfg3,
    pmu_wakeup_cfg4: PmuWakeupCfg4,
    pmu_pwrdn_con: PmuPwrdnCon,
    pmu_pwrdn_st: PmuPwrdnSt,
    pmu_pll_con: PmuPllCon,
    pmu_pwrmode_con: PmuPwrmodeCon,
    pmu_sft_con: PmuSftCon,
    pmu_int_con: PmuIntCon,
    pmu_int_st: PmuIntSt,
    pmu_gpio0_pos_int_con: PmuGpio0PosIntCon,
    pmu_gpio0_neg_int_con: PmuGpio0NegIntCon,
    pmu_gpio1_pos_int_con: PmuGpio1PosIntCon,
    pmu_gpio1_neg_int_con: PmuGpio1NegIntCon,
    pmu_gpio0_pos_int_st: PmuGpio0PosIntSt,
    pmu_gpio0_neg_int_st: PmuGpio0NegIntSt,
    pmu_gpio1_pos_int_st: PmuGpio1PosIntSt,
    pmu_gpio1_neg_int_st: PmuGpio1NegIntSt,
    pmu_pwrdn_inten: PmuPwrdnInten,
    pmu_pwrdn_status: PmuPwrdnStatus,
    pmu_wakeup_status: PmuWakeupStatus,
    pmu_bus_clr: PmuBusClr,
    pmu_bus_idle_req: PmuBusIdleReq,
    pmu_bus_idle_st: PmuBusIdleSt,
    pmu_bus_idle_ack: PmuBusIdleAck,
    pmu_cci500_con: PmuCci500Con,
    pmu_adb400_con: PmuAdb400Con,
    pmu_adb400_st: PmuAdb400St,
    pmu_power_st: PmuPowerSt,
    pmu_core_pwr_st: PmuCorePwrSt,
    pmu_osc_cnt: PmuOscCnt,
    pmu_plllock_cnt: PmuPlllockCnt,
    pmu_pllrst_cnt: PmuPllrstCnt,
    pmu_stable_cnt: PmuStableCnt,
    pmu_ddrio_pwron_cnt: PmuDdrioPwronCnt,
    pmu_wakeup_rst_clr_cnt: PmuWakeupRstClrCnt,
    pmu_ddr_sref_st: PmuDdrSrefSt,
    pmu_scu_l_pwrdn_cnt: PmuScuLPwrdnCnt,
    pmu_scu_l_pwrup_cnt: PmuScuLPwrupCnt,
    pmu_scu_b_pwrdn_cnt: PmuScuBPwrdnCnt,
    pmu_scu_b_pwrup_cnt: PmuScuBPwrupCnt,
    pmu_gpu_pwrdn_cnt: PmuGpuPwrdnCnt,
    pmu_gpu_pwrup_cnt: PmuGpuPwrupCnt,
    pmu_center_pwrdn_cnt: PmuCenterPwrdnCnt,
    pmu_center_pwrup_cnt: PmuCenterPwrupCnt,
    pmu_timeout_cnt: PmuTimeoutCnt,
    pmu_cpu0apm_con: PmuCpu0apmCon,
    pmu_cpu1apm_con: PmuCpu1apmCon,
    pmu_cpu2apm_con: PmuCpu2apmCon,
    pmu_cpu3apm_con: PmuCpu3apmCon,
    pmu_cpu0bpm_con: PmuCpu0bpmCon,
    pmu_cpu1bpm_con: PmuCpu1bpmCon,
    pmu_noc_auto_ena: PmuNocAutoEna,
    pmu_pwrdn_con1: PmuPwrdnCon1,
    _reserved56: [u8; 0x10],
    pmu_sys_reg0: PmuSysReg0,
    pmu_sys_reg1: PmuSysReg1,
    pmu_sys_reg2: PmuSysReg2,
    pmu_sys_reg3: PmuSysReg3,
}
impl RegisterBlock {
    #[doc = "0x00 - pmu wakeup configure register 0"]
    #[inline(always)]
    pub const fn pmu_wakeup_cfg0(&self) -> &PmuWakeupCfg0 {
        &self.pmu_wakeup_cfg0
    }
    #[doc = "0x04 - pmu wakeup configure register 1"]
    #[inline(always)]
    pub const fn pmu_wakeup_cfg1(&self) -> &PmuWakeupCfg1 {
        &self.pmu_wakeup_cfg1
    }
    #[doc = "0x08 - pmu wakeup configure register 2"]
    #[inline(always)]
    pub const fn pmu_wakeup_cfg2(&self) -> &PmuWakeupCfg2 {
        &self.pmu_wakeup_cfg2
    }
    #[doc = "0x0c - pmu wakeup configure register 3"]
    #[inline(always)]
    pub const fn pmu_wakeup_cfg3(&self) -> &PmuWakeupCfg3 {
        &self.pmu_wakeup_cfg3
    }
    #[doc = "0x10 - pmu wakeup configure register 4"]
    #[inline(always)]
    pub const fn pmu_wakeup_cfg4(&self) -> &PmuWakeupCfg4 {
        &self.pmu_wakeup_cfg4
    }
    #[doc = "0x14 - pmu power down configure register"]
    #[inline(always)]
    pub const fn pmu_pwrdn_con(&self) -> &PmuPwrdnCon {
        &self.pmu_pwrdn_con
    }
    #[doc = "0x18 - pmu power down status register"]
    #[inline(always)]
    pub const fn pmu_pwrdn_st(&self) -> &PmuPwrdnSt {
        &self.pmu_pwrdn_st
    }
    #[doc = "0x1c - PLL low power control register"]
    #[inline(always)]
    pub const fn pmu_pll_con(&self) -> &PmuPllCon {
        &self.pmu_pll_con
    }
    #[doc = "0x20 - pmu power mode configure register of common resource"]
    #[inline(always)]
    pub const fn pmu_pwrmode_con(&self) -> &PmuPwrmodeCon {
        &self.pmu_pwrmode_con
    }
    #[doc = "0x24 - pmu software configure register"]
    #[inline(always)]
    pub const fn pmu_sft_con(&self) -> &PmuSftCon {
        &self.pmu_sft_con
    }
    #[doc = "0x28 - pmu interrupt configure register"]
    #[inline(always)]
    pub const fn pmu_int_con(&self) -> &PmuIntCon {
        &self.pmu_int_con
    }
    #[doc = "0x2c - pmu interrupt status register"]
    #[inline(always)]
    pub const fn pmu_int_st(&self) -> &PmuIntSt {
        &self.pmu_int_st
    }
    #[doc = "0x30 - pmu gpio0 posedge interrupt configure register"]
    #[inline(always)]
    pub const fn pmu_gpio0_pos_int_con(&self) -> &PmuGpio0PosIntCon {
        &self.pmu_gpio0_pos_int_con
    }
    #[doc = "0x34 - pmu gpio0 negedge interrupt configure register"]
    #[inline(always)]
    pub const fn pmu_gpio0_neg_int_con(&self) -> &PmuGpio0NegIntCon {
        &self.pmu_gpio0_neg_int_con
    }
    #[doc = "0x38 - pmu gpio1 posedge interrupt configure register"]
    #[inline(always)]
    pub const fn pmu_gpio1_pos_int_con(&self) -> &PmuGpio1PosIntCon {
        &self.pmu_gpio1_pos_int_con
    }
    #[doc = "0x3c - pmu gpio1 negedge interrupt configure register"]
    #[inline(always)]
    pub const fn pmu_gpio1_neg_int_con(&self) -> &PmuGpio1NegIntCon {
        &self.pmu_gpio1_neg_int_con
    }
    #[doc = "0x40 - pmu gpio0 posedge interrupt status register"]
    #[inline(always)]
    pub const fn pmu_gpio0_pos_int_st(&self) -> &PmuGpio0PosIntSt {
        &self.pmu_gpio0_pos_int_st
    }
    #[doc = "0x44 - pmu gpio0 negedge interrupt status register"]
    #[inline(always)]
    pub const fn pmu_gpio0_neg_int_st(&self) -> &PmuGpio0NegIntSt {
        &self.pmu_gpio0_neg_int_st
    }
    #[doc = "0x48 - pmu gpio1 posedge interrupt status register"]
    #[inline(always)]
    pub const fn pmu_gpio1_pos_int_st(&self) -> &PmuGpio1PosIntSt {
        &self.pmu_gpio1_pos_int_st
    }
    #[doc = "0x4c - pmu gpio1 negedge interrupt status register"]
    #[inline(always)]
    pub const fn pmu_gpio1_neg_int_st(&self) -> &PmuGpio1NegIntSt {
        &self.pmu_gpio1_neg_int_st
    }
    #[doc = "0x50 - pmu power down interrupt enable register"]
    #[inline(always)]
    pub const fn pmu_pwrdn_inten(&self) -> &PmuPwrdnInten {
        &self.pmu_pwrdn_inten
    }
    #[doc = "0x54 - pmu power down interrupt status register"]
    #[inline(always)]
    pub const fn pmu_pwrdn_status(&self) -> &PmuPwrdnStatus {
        &self.pmu_pwrdn_status
    }
    #[doc = "0x58 - pmu interrupt wakeup status register"]
    #[inline(always)]
    pub const fn pmu_wakeup_status(&self) -> &PmuWakeupStatus {
        &self.pmu_wakeup_status
    }
    #[doc = "0x5c - pmu bus clear register"]
    #[inline(always)]
    pub const fn pmu_bus_clr(&self) -> &PmuBusClr {
        &self.pmu_bus_clr
    }
    #[doc = "0x60 - pmu bus idle request register"]
    #[inline(always)]
    pub const fn pmu_bus_idle_req(&self) -> &PmuBusIdleReq {
        &self.pmu_bus_idle_req
    }
    #[doc = "0x64 - pmu bus idle status register"]
    #[inline(always)]
    pub const fn pmu_bus_idle_st(&self) -> &PmuBusIdleSt {
        &self.pmu_bus_idle_st
    }
    #[doc = "0x68 - pmu bus idle ack status register"]
    #[inline(always)]
    pub const fn pmu_bus_idle_ack(&self) -> &PmuBusIdleAck {
        &self.pmu_bus_idle_ack
    }
    #[doc = "0x6c - CCI-500 low power control register"]
    #[inline(always)]
    pub const fn pmu_cci500_con(&self) -> &PmuCci500Con {
        &self.pmu_cci500_con
    }
    #[doc = "0x70 - adb-400 low power control register"]
    #[inline(always)]
    pub const fn pmu_adb400_con(&self) -> &PmuAdb400Con {
        &self.pmu_adb400_con
    }
    #[doc = "0x74 - adb-400 low power status register"]
    #[inline(always)]
    pub const fn pmu_adb400_st(&self) -> &PmuAdb400St {
        &self.pmu_adb400_st
    }
    #[doc = "0x78 - pmu power status register"]
    #[inline(always)]
    pub const fn pmu_power_st(&self) -> &PmuPowerSt {
        &self.pmu_power_st
    }
    #[doc = "0x7c - pmu core power status register"]
    #[inline(always)]
    pub const fn pmu_core_pwr_st(&self) -> &PmuCorePwrSt {
        &self.pmu_core_pwr_st
    }
    #[doc = "0x80 - pmu osc count register"]
    #[inline(always)]
    pub const fn pmu_osc_cnt(&self) -> &PmuOscCnt {
        &self.pmu_osc_cnt
    }
    #[doc = "0x84 - pmu pll lock count register"]
    #[inline(always)]
    pub const fn pmu_plllock_cnt(&self) -> &PmuPlllockCnt {
        &self.pmu_plllock_cnt
    }
    #[doc = "0x88 - pmu pll reset count register"]
    #[inline(always)]
    pub const fn pmu_pllrst_cnt(&self) -> &PmuPllrstCnt {
        &self.pmu_pllrst_cnt
    }
    #[doc = "0x8c - pmu power stable count register"]
    #[inline(always)]
    pub const fn pmu_stable_cnt(&self) -> &PmuStableCnt {
        &self.pmu_stable_cnt
    }
    #[doc = "0x90 - pmu ddrio power on count register"]
    #[inline(always)]
    pub const fn pmu_ddrio_pwron_cnt(&self) -> &PmuDdrioPwronCnt {
        &self.pmu_ddrio_pwron_cnt
    }
    #[doc = "0x94 - pmu wakeup reset clear count register"]
    #[inline(always)]
    pub const fn pmu_wakeup_rst_clr_cnt(&self) -> &PmuWakeupRstClrCnt {
        &self.pmu_wakeup_rst_clr_cnt
    }
    #[doc = "0x98 - pmu ddr self refresh status register"]
    #[inline(always)]
    pub const fn pmu_ddr_sref_st(&self) -> &PmuDdrSrefSt {
        &self.pmu_ddr_sref_st
    }
    #[doc = "0x9c - pmu scu_l power down count register"]
    #[inline(always)]
    pub const fn pmu_scu_l_pwrdn_cnt(&self) -> &PmuScuLPwrdnCnt {
        &self.pmu_scu_l_pwrdn_cnt
    }
    #[doc = "0xa0 - pmu scu_l power up count register"]
    #[inline(always)]
    pub const fn pmu_scu_l_pwrup_cnt(&self) -> &PmuScuLPwrupCnt {
        &self.pmu_scu_l_pwrup_cnt
    }
    #[doc = "0xa4 - pmu scu_b power down count register"]
    #[inline(always)]
    pub const fn pmu_scu_b_pwrdn_cnt(&self) -> &PmuScuBPwrdnCnt {
        &self.pmu_scu_b_pwrdn_cnt
    }
    #[doc = "0xa8 - pmu scu_b power up count register"]
    #[inline(always)]
    pub const fn pmu_scu_b_pwrup_cnt(&self) -> &PmuScuBPwrupCnt {
        &self.pmu_scu_b_pwrup_cnt
    }
    #[doc = "0xac - pmu gpu power down count register"]
    #[inline(always)]
    pub const fn pmu_gpu_pwrdn_cnt(&self) -> &PmuGpuPwrdnCnt {
        &self.pmu_gpu_pwrdn_cnt
    }
    #[doc = "0xb0 - pmu gpu power up count register"]
    #[inline(always)]
    pub const fn pmu_gpu_pwrup_cnt(&self) -> &PmuGpuPwrupCnt {
        &self.pmu_gpu_pwrup_cnt
    }
    #[doc = "0xb4 - pmu center power down count register"]
    #[inline(always)]
    pub const fn pmu_center_pwrdn_cnt(&self) -> &PmuCenterPwrdnCnt {
        &self.pmu_center_pwrdn_cnt
    }
    #[doc = "0xb8 - pmu center power up count register"]
    #[inline(always)]
    pub const fn pmu_center_pwrup_cnt(&self) -> &PmuCenterPwrupCnt {
        &self.pmu_center_pwrup_cnt
    }
    #[doc = "0xbc - pmu timeout count register"]
    #[inline(always)]
    pub const fn pmu_timeout_cnt(&self) -> &PmuTimeoutCnt {
        &self.pmu_timeout_cnt
    }
    #[doc = "0xc0 - pmu cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu0apm_con(&self) -> &PmuCpu0apmCon {
        &self.pmu_cpu0apm_con
    }
    #[doc = "0xc4 - pmu cpu1 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu1apm_con(&self) -> &PmuCpu1apmCon {
        &self.pmu_cpu1apm_con
    }
    #[doc = "0xc8 - pmu cpu2 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu2apm_con(&self) -> &PmuCpu2apmCon {
        &self.pmu_cpu2apm_con
    }
    #[doc = "0xcc - pmu cpu3 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu3apm_con(&self) -> &PmuCpu3apmCon {
        &self.pmu_cpu3apm_con
    }
    #[doc = "0xd0 - pmu cluster_b cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu0bpm_con(&self) -> &PmuCpu0bpmCon {
        &self.pmu_cpu0bpm_con
    }
    #[doc = "0xd4 - pmu cluster_b cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn pmu_cpu1bpm_con(&self) -> &PmuCpu1bpmCon {
        &self.pmu_cpu1bpm_con
    }
    #[doc = "0xd8 - NOC auto domain clock gating disable enable register"]
    #[inline(always)]
    pub const fn pmu_noc_auto_ena(&self) -> &PmuNocAutoEna {
        &self.pmu_noc_auto_ena
    }
    #[doc = "0xdc - pmu power down configure register1"]
    #[inline(always)]
    pub const fn pmu_pwrdn_con1(&self) -> &PmuPwrdnCon1 {
        &self.pmu_pwrdn_con1
    }
    #[doc = "0xf0 - pmu system register 0"]
    #[inline(always)]
    pub const fn pmu_sys_reg0(&self) -> &PmuSysReg0 {
        &self.pmu_sys_reg0
    }
    #[doc = "0xf4 - pmu system register 1"]
    #[inline(always)]
    pub const fn pmu_sys_reg1(&self) -> &PmuSysReg1 {
        &self.pmu_sys_reg1
    }
    #[doc = "0xf8 - pmu system register 2"]
    #[inline(always)]
    pub const fn pmu_sys_reg2(&self) -> &PmuSysReg2 {
        &self.pmu_sys_reg2
    }
    #[doc = "0xfc - pmu system register 3"]
    #[inline(always)]
    pub const fn pmu_sys_reg3(&self) -> &PmuSysReg3 {
        &self.pmu_sys_reg3
    }
}
#[doc = "PMU_WAKEUP_CFG0 (rw) register accessor: pmu wakeup configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_cfg0`]
module"]
#[doc(alias = "PMU_WAKEUP_CFG0")]
pub type PmuWakeupCfg0 = crate::Reg<pmu_wakeup_cfg0::PmuWakeupCfg0Spec>;
#[doc = "pmu wakeup configure register 0"]
pub mod pmu_wakeup_cfg0;
#[doc = "PMU_WAKEUP_CFG1 (rw) register accessor: pmu wakeup configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_cfg1`]
module"]
#[doc(alias = "PMU_WAKEUP_CFG1")]
pub type PmuWakeupCfg1 = crate::Reg<pmu_wakeup_cfg1::PmuWakeupCfg1Spec>;
#[doc = "pmu wakeup configure register 1"]
pub mod pmu_wakeup_cfg1;
#[doc = "PMU_WAKEUP_CFG2 (rw) register accessor: pmu wakeup configure register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_cfg2`]
module"]
#[doc(alias = "PMU_WAKEUP_CFG2")]
pub type PmuWakeupCfg2 = crate::Reg<pmu_wakeup_cfg2::PmuWakeupCfg2Spec>;
#[doc = "pmu wakeup configure register 2"]
pub mod pmu_wakeup_cfg2;
#[doc = "PMU_WAKEUP_CFG3 (rw) register accessor: pmu wakeup configure register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_cfg3`]
module"]
#[doc(alias = "PMU_WAKEUP_CFG3")]
pub type PmuWakeupCfg3 = crate::Reg<pmu_wakeup_cfg3::PmuWakeupCfg3Spec>;
#[doc = "pmu wakeup configure register 3"]
pub mod pmu_wakeup_cfg3;
#[doc = "PMU_WAKEUP_CFG4 (rw) register accessor: pmu wakeup configure register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_cfg4`]
module"]
#[doc(alias = "PMU_WAKEUP_CFG4")]
pub type PmuWakeupCfg4 = crate::Reg<pmu_wakeup_cfg4::PmuWakeupCfg4Spec>;
#[doc = "pmu wakeup configure register 4"]
pub mod pmu_wakeup_cfg4;
#[doc = "PMU_PWRDN_CON (rw) register accessor: pmu power down configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrdn_con`]
module"]
#[doc(alias = "PMU_PWRDN_CON")]
pub type PmuPwrdnCon = crate::Reg<pmu_pwrdn_con::PmuPwrdnConSpec>;
#[doc = "pmu power down configure register"]
pub mod pmu_pwrdn_con;
#[doc = "PMU_PWRDN_ST (rw) register accessor: pmu power down status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrdn_st`]
module"]
#[doc(alias = "PMU_PWRDN_ST")]
pub type PmuPwrdnSt = crate::Reg<pmu_pwrdn_st::PmuPwrdnStSpec>;
#[doc = "pmu power down status register"]
pub mod pmu_pwrdn_st;
#[doc = "PMU_PLL_CON (rw) register accessor: PLL low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pll_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pll_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pll_con`]
module"]
#[doc(alias = "PMU_PLL_CON")]
pub type PmuPllCon = crate::Reg<pmu_pll_con::PmuPllConSpec>;
#[doc = "PLL low power control register"]
pub mod pmu_pll_con;
#[doc = "PMU_PWRMODE_CON (rw) register accessor: pmu power mode configure register of common resource\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrmode_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrmode_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrmode_con`]
module"]
#[doc(alias = "PMU_PWRMODE_CON")]
pub type PmuPwrmodeCon = crate::Reg<pmu_pwrmode_con::PmuPwrmodeConSpec>;
#[doc = "pmu power mode configure register of common resource"]
pub mod pmu_pwrmode_con;
#[doc = "PMU_SFT_CON (rw) register accessor: pmu software configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sft_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sft_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_sft_con`]
module"]
#[doc(alias = "PMU_SFT_CON")]
pub type PmuSftCon = crate::Reg<pmu_sft_con::PmuSftConSpec>;
#[doc = "pmu software configure register"]
pub mod pmu_sft_con;
#[doc = "PMU_INT_CON (rw) register accessor: pmu interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_int_con`]
module"]
#[doc(alias = "PMU_INT_CON")]
pub type PmuIntCon = crate::Reg<pmu_int_con::PmuIntConSpec>;
#[doc = "pmu interrupt configure register"]
pub mod pmu_int_con;
#[doc = "PMU_INT_ST (rw) register accessor: pmu interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_int_st`]
module"]
#[doc(alias = "PMU_INT_ST")]
pub type PmuIntSt = crate::Reg<pmu_int_st::PmuIntStSpec>;
#[doc = "pmu interrupt status register"]
pub mod pmu_int_st;
#[doc = "PMU_GPIO0_POS_INT_CON (rw) register accessor: pmu gpio0 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_pos_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_pos_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio0_pos_int_con`]
module"]
#[doc(alias = "PMU_GPIO0_POS_INT_CON")]
pub type PmuGpio0PosIntCon = crate::Reg<pmu_gpio0_pos_int_con::PmuGpio0PosIntConSpec>;
#[doc = "pmu gpio0 posedge interrupt configure register"]
pub mod pmu_gpio0_pos_int_con;
#[doc = "PMU_GPIO0_NEG_INT_CON (rw) register accessor: pmu gpio0 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_neg_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_neg_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio0_neg_int_con`]
module"]
#[doc(alias = "PMU_GPIO0_NEG_INT_CON")]
pub type PmuGpio0NegIntCon = crate::Reg<pmu_gpio0_neg_int_con::PmuGpio0NegIntConSpec>;
#[doc = "pmu gpio0 negedge interrupt configure register"]
pub mod pmu_gpio0_neg_int_con;
#[doc = "PMU_GPIO1_POS_INT_CON (rw) register accessor: pmu gpio1 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio1_pos_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio1_pos_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio1_pos_int_con`]
module"]
#[doc(alias = "PMU_GPIO1_POS_INT_CON")]
pub type PmuGpio1PosIntCon = crate::Reg<pmu_gpio1_pos_int_con::PmuGpio1PosIntConSpec>;
#[doc = "pmu gpio1 posedge interrupt configure register"]
pub mod pmu_gpio1_pos_int_con;
#[doc = "PMU_GPIO1_NEG_INT_CON (rw) register accessor: pmu gpio1 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio1_neg_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio1_neg_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio1_neg_int_con`]
module"]
#[doc(alias = "PMU_GPIO1_NEG_INT_CON")]
pub type PmuGpio1NegIntCon = crate::Reg<pmu_gpio1_neg_int_con::PmuGpio1NegIntConSpec>;
#[doc = "pmu gpio1 negedge interrupt configure register"]
pub mod pmu_gpio1_neg_int_con;
#[doc = "PMU_GPIO0_POS_INT_ST (rw) register accessor: pmu gpio0 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_pos_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_pos_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio0_pos_int_st`]
module"]
#[doc(alias = "PMU_GPIO0_POS_INT_ST")]
pub type PmuGpio0PosIntSt = crate::Reg<pmu_gpio0_pos_int_st::PmuGpio0PosIntStSpec>;
#[doc = "pmu gpio0 posedge interrupt status register"]
pub mod pmu_gpio0_pos_int_st;
#[doc = "PMU_GPIO0_NEG_INT_ST (rw) register accessor: pmu gpio0 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_neg_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_neg_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio0_neg_int_st`]
module"]
#[doc(alias = "PMU_GPIO0_NEG_INT_ST")]
pub type PmuGpio0NegIntSt = crate::Reg<pmu_gpio0_neg_int_st::PmuGpio0NegIntStSpec>;
#[doc = "pmu gpio0 negedge interrupt status register"]
pub mod pmu_gpio0_neg_int_st;
#[doc = "PMU_GPIO1_POS_INT_ST (rw) register accessor: pmu gpio1 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio1_pos_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio1_pos_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio1_pos_int_st`]
module"]
#[doc(alias = "PMU_GPIO1_POS_INT_ST")]
pub type PmuGpio1PosIntSt = crate::Reg<pmu_gpio1_pos_int_st::PmuGpio1PosIntStSpec>;
#[doc = "pmu gpio1 posedge interrupt status register"]
pub mod pmu_gpio1_pos_int_st;
#[doc = "PMU_GPIO1_NEG_INT_ST (rw) register accessor: pmu gpio1 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio1_neg_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio1_neg_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpio1_neg_int_st`]
module"]
#[doc(alias = "PMU_GPIO1_NEG_INT_ST")]
pub type PmuGpio1NegIntSt = crate::Reg<pmu_gpio1_neg_int_st::PmuGpio1NegIntStSpec>;
#[doc = "pmu gpio1 negedge interrupt status register"]
pub mod pmu_gpio1_neg_int_st;
#[doc = "PMU_PWRDN_INTEN (rw) register accessor: pmu power down interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrdn_inten`]
module"]
#[doc(alias = "PMU_PWRDN_INTEN")]
pub type PmuPwrdnInten = crate::Reg<pmu_pwrdn_inten::PmuPwrdnIntenSpec>;
#[doc = "pmu power down interrupt enable register"]
pub mod pmu_pwrdn_inten;
#[doc = "PMU_PWRDN_STATUS (rw) register accessor: pmu power down interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrdn_status`]
module"]
#[doc(alias = "PMU_PWRDN_STATUS")]
pub type PmuPwrdnStatus = crate::Reg<pmu_pwrdn_status::PmuPwrdnStatusSpec>;
#[doc = "pmu power down interrupt status register"]
pub mod pmu_pwrdn_status;
#[doc = "PMU_WAKEUP_STATUS (rw) register accessor: pmu interrupt wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_status`]
module"]
#[doc(alias = "PMU_WAKEUP_STATUS")]
pub type PmuWakeupStatus = crate::Reg<pmu_wakeup_status::PmuWakeupStatusSpec>;
#[doc = "pmu interrupt wakeup status register"]
pub mod pmu_wakeup_status;
#[doc = "PMU_BUS_CLR (rw) register accessor: pmu bus clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_bus_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_bus_clr`]
module"]
#[doc(alias = "PMU_BUS_CLR")]
pub type PmuBusClr = crate::Reg<pmu_bus_clr::PmuBusClrSpec>;
#[doc = "pmu bus clear register"]
pub mod pmu_bus_clr;
#[doc = "PMU_BUS_IDLE_REQ (rw) register accessor: pmu bus idle request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_idle_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_bus_idle_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_bus_idle_req`]
module"]
#[doc(alias = "PMU_BUS_IDLE_REQ")]
pub type PmuBusIdleReq = crate::Reg<pmu_bus_idle_req::PmuBusIdleReqSpec>;
#[doc = "pmu bus idle request register"]
pub mod pmu_bus_idle_req;
#[doc = "PMU_BUS_IDLE_ST (r) register accessor: pmu bus idle status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_idle_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_bus_idle_st`]
module"]
#[doc(alias = "PMU_BUS_IDLE_ST")]
pub type PmuBusIdleSt = crate::Reg<pmu_bus_idle_st::PmuBusIdleStSpec>;
#[doc = "pmu bus idle status register"]
pub mod pmu_bus_idle_st;
#[doc = "PMU_BUS_IDLE_ACK (r) register accessor: pmu bus idle ack status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_idle_ack::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_bus_idle_ack`]
module"]
#[doc(alias = "PMU_BUS_IDLE_ACK")]
pub type PmuBusIdleAck = crate::Reg<pmu_bus_idle_ack::PmuBusIdleAckSpec>;
#[doc = "pmu bus idle ack status register"]
pub mod pmu_bus_idle_ack;
#[doc = "PMU_CCI500_CON (rw) register accessor: CCI-500 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cci500_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cci500_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cci500_con`]
module"]
#[doc(alias = "PMU_CCI500_CON")]
pub type PmuCci500Con = crate::Reg<pmu_cci500_con::PmuCci500ConSpec>;
#[doc = "CCI-500 low power control register"]
pub mod pmu_cci500_con;
#[doc = "PMU_ADB400_CON (rw) register accessor: adb-400 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_adb400_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_adb400_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_adb400_con`]
module"]
#[doc(alias = "PMU_ADB400_CON")]
pub type PmuAdb400Con = crate::Reg<pmu_adb400_con::PmuAdb400ConSpec>;
#[doc = "adb-400 low power control register"]
pub mod pmu_adb400_con;
#[doc = "PMU_ADB400_ST (rw) register accessor: adb-400 low power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_adb400_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_adb400_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_adb400_st`]
module"]
#[doc(alias = "PMU_ADB400_ST")]
pub type PmuAdb400St = crate::Reg<pmu_adb400_st::PmuAdb400StSpec>;
#[doc = "adb-400 low power status register"]
pub mod pmu_adb400_st;
#[doc = "PMU_POWER_ST (rw) register accessor: pmu power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_power_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_power_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_power_st`]
module"]
#[doc(alias = "PMU_POWER_ST")]
pub type PmuPowerSt = crate::Reg<pmu_power_st::PmuPowerStSpec>;
#[doc = "pmu power status register"]
pub mod pmu_power_st;
#[doc = "PMU_CORE_PWR_ST (r) register accessor: pmu core power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_core_pwr_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_core_pwr_st`]
module"]
#[doc(alias = "PMU_CORE_PWR_ST")]
pub type PmuCorePwrSt = crate::Reg<pmu_core_pwr_st::PmuCorePwrStSpec>;
#[doc = "pmu core power status register"]
pub mod pmu_core_pwr_st;
#[doc = "PMU_OSC_CNT (rw) register accessor: pmu osc count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_osc_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_osc_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_osc_cnt`]
module"]
#[doc(alias = "PMU_OSC_CNT")]
pub type PmuOscCnt = crate::Reg<pmu_osc_cnt::PmuOscCntSpec>;
#[doc = "pmu osc count register"]
pub mod pmu_osc_cnt;
#[doc = "PMU_PLLLOCK_CNT (rw) register accessor: pmu pll lock count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_plllock_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_plllock_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_plllock_cnt`]
module"]
#[doc(alias = "PMU_PLLLOCK_CNT")]
pub type PmuPlllockCnt = crate::Reg<pmu_plllock_cnt::PmuPlllockCntSpec>;
#[doc = "pmu pll lock count register"]
pub mod pmu_plllock_cnt;
#[doc = "PMU_PLLRST_CNT (rw) register accessor: pmu pll reset count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pllrst_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pllrst_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pllrst_cnt`]
module"]
#[doc(alias = "PMU_PLLRST_CNT")]
pub type PmuPllrstCnt = crate::Reg<pmu_pllrst_cnt::PmuPllrstCntSpec>;
#[doc = "pmu pll reset count register"]
pub mod pmu_pllrst_cnt;
#[doc = "PMU_STABLE_CNT (rw) register accessor: pmu power stable count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_stable_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_stable_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_stable_cnt`]
module"]
#[doc(alias = "PMU_STABLE_CNT")]
pub type PmuStableCnt = crate::Reg<pmu_stable_cnt::PmuStableCntSpec>;
#[doc = "pmu power stable count register"]
pub mod pmu_stable_cnt;
#[doc = "PMU_DDRIO_PWRON_CNT (rw) register accessor: pmu ddrio power on count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_ddrio_pwron_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_ddrio_pwron_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_ddrio_pwron_cnt`]
module"]
#[doc(alias = "PMU_DDRIO_PWRON_CNT")]
pub type PmuDdrioPwronCnt = crate::Reg<pmu_ddrio_pwron_cnt::PmuDdrioPwronCntSpec>;
#[doc = "pmu ddrio power on count register"]
pub mod pmu_ddrio_pwron_cnt;
#[doc = "PMU_WAKEUP_RST_CLR_CNT (rw) register accessor: pmu wakeup reset clear count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_wakeup_rst_clr_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_wakeup_rst_clr_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_wakeup_rst_clr_cnt`]
module"]
#[doc(alias = "PMU_WAKEUP_RST_CLR_CNT")]
pub type PmuWakeupRstClrCnt = crate::Reg<pmu_wakeup_rst_clr_cnt::PmuWakeupRstClrCntSpec>;
#[doc = "pmu wakeup reset clear count register"]
pub mod pmu_wakeup_rst_clr_cnt;
#[doc = "PMU_DDR_SREF_ST (rw) register accessor: pmu ddr self refresh status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_ddr_sref_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_ddr_sref_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_ddr_sref_st`]
module"]
#[doc(alias = "PMU_DDR_SREF_ST")]
pub type PmuDdrSrefSt = crate::Reg<pmu_ddr_sref_st::PmuDdrSrefStSpec>;
#[doc = "pmu ddr self refresh status register"]
pub mod pmu_ddr_sref_st;
#[doc = "PMU_SCU_L_PWRDN_CNT (rw) register accessor: pmu scu_l power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_scu_l_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_scu_l_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_scu_l_pwrdn_cnt`]
module"]
#[doc(alias = "PMU_SCU_L_PWRDN_CNT")]
pub type PmuScuLPwrdnCnt = crate::Reg<pmu_scu_l_pwrdn_cnt::PmuScuLPwrdnCntSpec>;
#[doc = "pmu scu_l power down count register"]
pub mod pmu_scu_l_pwrdn_cnt;
#[doc = "PMU_SCU_L_PWRUP_CNT (rw) register accessor: pmu scu_l power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_scu_l_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_scu_l_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_scu_l_pwrup_cnt`]
module"]
#[doc(alias = "PMU_SCU_L_PWRUP_CNT")]
pub type PmuScuLPwrupCnt = crate::Reg<pmu_scu_l_pwrup_cnt::PmuScuLPwrupCntSpec>;
#[doc = "pmu scu_l power up count register"]
pub mod pmu_scu_l_pwrup_cnt;
#[doc = "PMU_SCU_B_PWRDN_CNT (rw) register accessor: pmu scu_b power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_scu_b_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_scu_b_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_scu_b_pwrdn_cnt`]
module"]
#[doc(alias = "PMU_SCU_B_PWRDN_CNT")]
pub type PmuScuBPwrdnCnt = crate::Reg<pmu_scu_b_pwrdn_cnt::PmuScuBPwrdnCntSpec>;
#[doc = "pmu scu_b power down count register"]
pub mod pmu_scu_b_pwrdn_cnt;
#[doc = "PMU_SCU_B_PWRUP_CNT (rw) register accessor: pmu scu_b power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_scu_b_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_scu_b_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_scu_b_pwrup_cnt`]
module"]
#[doc(alias = "PMU_SCU_B_PWRUP_CNT")]
pub type PmuScuBPwrupCnt = crate::Reg<pmu_scu_b_pwrup_cnt::PmuScuBPwrupCntSpec>;
#[doc = "pmu scu_b power up count register"]
pub mod pmu_scu_b_pwrup_cnt;
#[doc = "PMU_GPU_PWRDN_CNT (rw) register accessor: pmu gpu power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpu_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpu_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpu_pwrdn_cnt`]
module"]
#[doc(alias = "PMU_GPU_PWRDN_CNT")]
pub type PmuGpuPwrdnCnt = crate::Reg<pmu_gpu_pwrdn_cnt::PmuGpuPwrdnCntSpec>;
#[doc = "pmu gpu power down count register"]
pub mod pmu_gpu_pwrdn_cnt;
#[doc = "PMU_GPU_PWRUP_CNT (rw) register accessor: pmu gpu power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpu_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpu_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_gpu_pwrup_cnt`]
module"]
#[doc(alias = "PMU_GPU_PWRUP_CNT")]
pub type PmuGpuPwrupCnt = crate::Reg<pmu_gpu_pwrup_cnt::PmuGpuPwrupCntSpec>;
#[doc = "pmu gpu power up count register"]
pub mod pmu_gpu_pwrup_cnt;
#[doc = "PMU_CENTER_PWRDN_CNT (rw) register accessor: pmu center power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_center_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_center_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_center_pwrdn_cnt`]
module"]
#[doc(alias = "PMU_CENTER_PWRDN_CNT")]
pub type PmuCenterPwrdnCnt = crate::Reg<pmu_center_pwrdn_cnt::PmuCenterPwrdnCntSpec>;
#[doc = "pmu center power down count register"]
pub mod pmu_center_pwrdn_cnt;
#[doc = "PMU_CENTER_PWRUP_CNT (rw) register accessor: pmu center power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_center_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_center_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_center_pwrup_cnt`]
module"]
#[doc(alias = "PMU_CENTER_PWRUP_CNT")]
pub type PmuCenterPwrupCnt = crate::Reg<pmu_center_pwrup_cnt::PmuCenterPwrupCntSpec>;
#[doc = "pmu center power up count register"]
pub mod pmu_center_pwrup_cnt;
#[doc = "PMU_TIMEOUT_CNT (rw) register accessor: pmu timeout count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_timeout_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_timeout_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_timeout_cnt`]
module"]
#[doc(alias = "PMU_TIMEOUT_CNT")]
pub type PmuTimeoutCnt = crate::Reg<pmu_timeout_cnt::PmuTimeoutCntSpec>;
#[doc = "pmu timeout count register"]
pub mod pmu_timeout_cnt;
#[doc = "PMU_CPU0APM_CON (rw) register accessor: pmu cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu0apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu0apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu0apm_con`]
module"]
#[doc(alias = "PMU_CPU0APM_CON")]
pub type PmuCpu0apmCon = crate::Reg<pmu_cpu0apm_con::PmuCpu0apmConSpec>;
#[doc = "pmu cpu0 auto power down control register"]
pub mod pmu_cpu0apm_con;
#[doc = "PMU_CPU1APM_CON (rw) register accessor: pmu cpu1 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu1apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu1apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu1apm_con`]
module"]
#[doc(alias = "PMU_CPU1APM_CON")]
pub type PmuCpu1apmCon = crate::Reg<pmu_cpu1apm_con::PmuCpu1apmConSpec>;
#[doc = "pmu cpu1 auto power down control register"]
pub mod pmu_cpu1apm_con;
#[doc = "PMU_CPU2APM_CON (rw) register accessor: pmu cpu2 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu2apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu2apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu2apm_con`]
module"]
#[doc(alias = "PMU_CPU2APM_CON")]
pub type PmuCpu2apmCon = crate::Reg<pmu_cpu2apm_con::PmuCpu2apmConSpec>;
#[doc = "pmu cpu2 auto power down control register"]
pub mod pmu_cpu2apm_con;
#[doc = "PMU_CPU3APM_CON (rw) register accessor: pmu cpu3 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu3apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu3apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu3apm_con`]
module"]
#[doc(alias = "PMU_CPU3APM_CON")]
pub type PmuCpu3apmCon = crate::Reg<pmu_cpu3apm_con::PmuCpu3apmConSpec>;
#[doc = "pmu cpu3 auto power down control register"]
pub mod pmu_cpu3apm_con;
#[doc = "PMU_CPU0BPM_CON (rw) register accessor: pmu cluster_b cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu0bpm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu0bpm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu0bpm_con`]
module"]
#[doc(alias = "PMU_CPU0BPM_CON")]
pub type PmuCpu0bpmCon = crate::Reg<pmu_cpu0bpm_con::PmuCpu0bpmConSpec>;
#[doc = "pmu cluster_b cpu0 auto power down control register"]
pub mod pmu_cpu0bpm_con;
#[doc = "PMU_CPU1BPM_CON (rw) register accessor: pmu cluster_b cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_cpu1bpm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_cpu1bpm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_cpu1bpm_con`]
module"]
#[doc(alias = "PMU_CPU1BPM_CON")]
pub type PmuCpu1bpmCon = crate::Reg<pmu_cpu1bpm_con::PmuCpu1bpmConSpec>;
#[doc = "pmu cluster_b cpu0 auto power down control register"]
pub mod pmu_cpu1bpm_con;
#[doc = "PMU_NOC_AUTO_ENA (rw) register accessor: NOC auto domain clock gating disable enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_noc_auto_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_noc_auto_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_noc_auto_ena`]
module"]
#[doc(alias = "PMU_NOC_AUTO_ENA")]
pub type PmuNocAutoEna = crate::Reg<pmu_noc_auto_ena::PmuNocAutoEnaSpec>;
#[doc = "NOC auto domain clock gating disable enable register"]
pub mod pmu_noc_auto_ena;
#[doc = "PMU_PWRDN_CON1 (rw) register accessor: pmu power down configure register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_pwrdn_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_pwrdn_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_pwrdn_con1`]
module"]
#[doc(alias = "PMU_PWRDN_CON1")]
pub type PmuPwrdnCon1 = crate::Reg<pmu_pwrdn_con1::PmuPwrdnCon1Spec>;
#[doc = "pmu power down configure register1"]
pub mod pmu_pwrdn_con1;
#[doc = "PMU_SYS_REG0 (rw) register accessor: pmu system register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_sys_reg0`]
module"]
#[doc(alias = "PMU_SYS_REG0")]
pub type PmuSysReg0 = crate::Reg<pmu_sys_reg0::PmuSysReg0Spec>;
#[doc = "pmu system register 0"]
pub mod pmu_sys_reg0;
#[doc = "PMU_SYS_REG1 (rw) register accessor: pmu system register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_sys_reg1`]
module"]
#[doc(alias = "PMU_SYS_REG1")]
pub type PmuSysReg1 = crate::Reg<pmu_sys_reg1::PmuSysReg1Spec>;
#[doc = "pmu system register 1"]
pub mod pmu_sys_reg1;
#[doc = "PMU_SYS_REG2 (rw) register accessor: pmu system register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_sys_reg2`]
module"]
#[doc(alias = "PMU_SYS_REG2")]
pub type PmuSysReg2 = crate::Reg<pmu_sys_reg2::PmuSysReg2Spec>;
#[doc = "pmu system register 2"]
pub mod pmu_sys_reg2;
#[doc = "PMU_SYS_REG3 (rw) register accessor: pmu system register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_sys_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_sys_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_sys_reg3`]
module"]
#[doc(alias = "PMU_SYS_REG3")]
pub type PmuSysReg3 = crate::Reg<pmu_sys_reg3::PmuSysReg3Spec>;
#[doc = "pmu system register 3"]
pub mod pmu_sys_reg3;
