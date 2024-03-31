#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wakeup_cfg0: WakeupCfg0,
    wakeup_cfg1: WakeupCfg1,
    wakeup_cfg2: WakeupCfg2,
    wakeup_cfg3: WakeupCfg3,
    wakeup_cfg4: WakeupCfg4,
    pwrdn_con: PwrdnCon,
    pwrdn_st: PwrdnSt,
    pll_con: PllCon,
    pwrmode_con: PwrmodeCon,
    sft_con: SftCon,
    int_con: IntCon,
    int_st: IntSt,
    gpio0_pos_int_con: Gpio0PosIntCon,
    gpio0_neg_int_con: Gpio0NegIntCon,
    gpio1_pos_int_con: Gpio1PosIntCon,
    gpio1_neg_int_con: Gpio1NegIntCon,
    gpio0_pos_int_st: Gpio0PosIntSt,
    gpio0_neg_int_st: Gpio0NegIntSt,
    gpio1_pos_int_st: Gpio1PosIntSt,
    gpio1_neg_int_st: Gpio1NegIntSt,
    pwrdn_inten: PwrdnInten,
    pwrdn_status: PwrdnStatus,
    wakeup_status: WakeupStatus,
    bus_clr: BusClr,
    bus_idle_req: BusIdleReq,
    bus_idle_st: BusIdleSt,
    bus_idle_ack: BusIdleAck,
    cci500_con: Cci500Con,
    adb400_con: Adb400Con,
    adb400_st: Adb400St,
    power_st: PowerSt,
    core_pwr_st: CorePwrSt,
    osc_cnt: OscCnt,
    plllock_cnt: PlllockCnt,
    pllrst_cnt: PllrstCnt,
    stable_cnt: StableCnt,
    ddrio_pwron_cnt: DdrioPwronCnt,
    wakeup_rst_clr_cnt: WakeupRstClrCnt,
    ddr_sref_st: DdrSrefSt,
    scu_l_pwrdn_cnt: ScuLPwrdnCnt,
    scu_l_pwrup_cnt: ScuLPwrupCnt,
    scu_b_pwrdn_cnt: ScuBPwrdnCnt,
    scu_b_pwrup_cnt: ScuBPwrupCnt,
    gpu_pwrdn_cnt: GpuPwrdnCnt,
    gpu_pwrup_cnt: GpuPwrupCnt,
    center_pwrdn_cnt: CenterPwrdnCnt,
    center_pwrup_cnt: CenterPwrupCnt,
    timeout_cnt: TimeoutCnt,
    cpu0apm_con: Cpu0apmCon,
    cpu1apm_con: Cpu1apmCon,
    cpu2apm_con: Cpu2apmCon,
    cpu3apm_con: Cpu3apmCon,
    cpu0bpm_con: Cpu0bpmCon,
    cpu1bpm_con: Cpu1bpmCon,
    noc_auto_ena: NocAutoEna,
    pwrdn_con1: PwrdnCon1,
    _reserved56: [u8; 0x10],
    sys_reg0: SysReg0,
    sys_reg1: SysReg1,
    sys_reg2: SysReg2,
    sys_reg3: SysReg3,
}
impl RegisterBlock {
    #[doc = "0x00 - pmu wakeup configure register 0"]
    #[inline(always)]
    pub const fn wakeup_cfg0(&self) -> &WakeupCfg0 {
        &self.wakeup_cfg0
    }
    #[doc = "0x04 - pmu wakeup configure register 1"]
    #[inline(always)]
    pub const fn wakeup_cfg1(&self) -> &WakeupCfg1 {
        &self.wakeup_cfg1
    }
    #[doc = "0x08 - pmu wakeup configure register 2"]
    #[inline(always)]
    pub const fn wakeup_cfg2(&self) -> &WakeupCfg2 {
        &self.wakeup_cfg2
    }
    #[doc = "0x0c - pmu wakeup configure register 3"]
    #[inline(always)]
    pub const fn wakeup_cfg3(&self) -> &WakeupCfg3 {
        &self.wakeup_cfg3
    }
    #[doc = "0x10 - pmu wakeup configure register 4"]
    #[inline(always)]
    pub const fn wakeup_cfg4(&self) -> &WakeupCfg4 {
        &self.wakeup_cfg4
    }
    #[doc = "0x14 - pmu power down configure register"]
    #[inline(always)]
    pub const fn pwrdn_con(&self) -> &PwrdnCon {
        &self.pwrdn_con
    }
    #[doc = "0x18 - pmu power down status register"]
    #[inline(always)]
    pub const fn pwrdn_st(&self) -> &PwrdnSt {
        &self.pwrdn_st
    }
    #[doc = "0x1c - PLL low power control register"]
    #[inline(always)]
    pub const fn pll_con(&self) -> &PllCon {
        &self.pll_con
    }
    #[doc = "0x20 - pmu power mode configure register of common resource"]
    #[inline(always)]
    pub const fn pwrmode_con(&self) -> &PwrmodeCon {
        &self.pwrmode_con
    }
    #[doc = "0x24 - pmu software configure register"]
    #[inline(always)]
    pub const fn sft_con(&self) -> &SftCon {
        &self.sft_con
    }
    #[doc = "0x28 - pmu interrupt configure register"]
    #[inline(always)]
    pub const fn int_con(&self) -> &IntCon {
        &self.int_con
    }
    #[doc = "0x2c - pmu interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    #[doc = "0x30 - pmu gpio0 posedge interrupt configure register"]
    #[inline(always)]
    pub const fn gpio0_pos_int_con(&self) -> &Gpio0PosIntCon {
        &self.gpio0_pos_int_con
    }
    #[doc = "0x34 - pmu gpio0 negedge interrupt configure register"]
    #[inline(always)]
    pub const fn gpio0_neg_int_con(&self) -> &Gpio0NegIntCon {
        &self.gpio0_neg_int_con
    }
    #[doc = "0x38 - pmu gpio1 posedge interrupt configure register"]
    #[inline(always)]
    pub const fn gpio1_pos_int_con(&self) -> &Gpio1PosIntCon {
        &self.gpio1_pos_int_con
    }
    #[doc = "0x3c - pmu gpio1 negedge interrupt configure register"]
    #[inline(always)]
    pub const fn gpio1_neg_int_con(&self) -> &Gpio1NegIntCon {
        &self.gpio1_neg_int_con
    }
    #[doc = "0x40 - pmu gpio0 posedge interrupt status register"]
    #[inline(always)]
    pub const fn gpio0_pos_int_st(&self) -> &Gpio0PosIntSt {
        &self.gpio0_pos_int_st
    }
    #[doc = "0x44 - pmu gpio0 negedge interrupt status register"]
    #[inline(always)]
    pub const fn gpio0_neg_int_st(&self) -> &Gpio0NegIntSt {
        &self.gpio0_neg_int_st
    }
    #[doc = "0x48 - pmu gpio1 posedge interrupt status register"]
    #[inline(always)]
    pub const fn gpio1_pos_int_st(&self) -> &Gpio1PosIntSt {
        &self.gpio1_pos_int_st
    }
    #[doc = "0x4c - pmu gpio1 negedge interrupt status register"]
    #[inline(always)]
    pub const fn gpio1_neg_int_st(&self) -> &Gpio1NegIntSt {
        &self.gpio1_neg_int_st
    }
    #[doc = "0x50 - pmu power down interrupt enable register"]
    #[inline(always)]
    pub const fn pwrdn_inten(&self) -> &PwrdnInten {
        &self.pwrdn_inten
    }
    #[doc = "0x54 - pmu power down interrupt status register"]
    #[inline(always)]
    pub const fn pwrdn_status(&self) -> &PwrdnStatus {
        &self.pwrdn_status
    }
    #[doc = "0x58 - pmu interrupt wakeup status register"]
    #[inline(always)]
    pub const fn wakeup_status(&self) -> &WakeupStatus {
        &self.wakeup_status
    }
    #[doc = "0x5c - pmu bus clear register"]
    #[inline(always)]
    pub const fn bus_clr(&self) -> &BusClr {
        &self.bus_clr
    }
    #[doc = "0x60 - pmu bus idle request register"]
    #[inline(always)]
    pub const fn bus_idle_req(&self) -> &BusIdleReq {
        &self.bus_idle_req
    }
    #[doc = "0x64 - pmu bus idle status register"]
    #[inline(always)]
    pub const fn bus_idle_st(&self) -> &BusIdleSt {
        &self.bus_idle_st
    }
    #[doc = "0x68 - pmu bus idle ack status register"]
    #[inline(always)]
    pub const fn bus_idle_ack(&self) -> &BusIdleAck {
        &self.bus_idle_ack
    }
    #[doc = "0x6c - CCI-500 low power control register"]
    #[inline(always)]
    pub const fn cci500_con(&self) -> &Cci500Con {
        &self.cci500_con
    }
    #[doc = "0x70 - adb-400 low power control register"]
    #[inline(always)]
    pub const fn adb400_con(&self) -> &Adb400Con {
        &self.adb400_con
    }
    #[doc = "0x74 - adb-400 low power status register"]
    #[inline(always)]
    pub const fn adb400_st(&self) -> &Adb400St {
        &self.adb400_st
    }
    #[doc = "0x78 - pmu power status register"]
    #[inline(always)]
    pub const fn power_st(&self) -> &PowerSt {
        &self.power_st
    }
    #[doc = "0x7c - pmu core power status register"]
    #[inline(always)]
    pub const fn core_pwr_st(&self) -> &CorePwrSt {
        &self.core_pwr_st
    }
    #[doc = "0x80 - pmu osc count register"]
    #[inline(always)]
    pub const fn osc_cnt(&self) -> &OscCnt {
        &self.osc_cnt
    }
    #[doc = "0x84 - pmu pll lock count register"]
    #[inline(always)]
    pub const fn plllock_cnt(&self) -> &PlllockCnt {
        &self.plllock_cnt
    }
    #[doc = "0x88 - pmu pll reset count register"]
    #[inline(always)]
    pub const fn pllrst_cnt(&self) -> &PllrstCnt {
        &self.pllrst_cnt
    }
    #[doc = "0x8c - pmu power stable count register"]
    #[inline(always)]
    pub const fn stable_cnt(&self) -> &StableCnt {
        &self.stable_cnt
    }
    #[doc = "0x90 - pmu ddrio power on count register"]
    #[inline(always)]
    pub const fn ddrio_pwron_cnt(&self) -> &DdrioPwronCnt {
        &self.ddrio_pwron_cnt
    }
    #[doc = "0x94 - pmu wakeup reset clear count register"]
    #[inline(always)]
    pub const fn wakeup_rst_clr_cnt(&self) -> &WakeupRstClrCnt {
        &self.wakeup_rst_clr_cnt
    }
    #[doc = "0x98 - pmu ddr self refresh status register"]
    #[inline(always)]
    pub const fn ddr_sref_st(&self) -> &DdrSrefSt {
        &self.ddr_sref_st
    }
    #[doc = "0x9c - pmu scu_l power down count register"]
    #[inline(always)]
    pub const fn scu_l_pwrdn_cnt(&self) -> &ScuLPwrdnCnt {
        &self.scu_l_pwrdn_cnt
    }
    #[doc = "0xa0 - pmu scu_l power up count register"]
    #[inline(always)]
    pub const fn scu_l_pwrup_cnt(&self) -> &ScuLPwrupCnt {
        &self.scu_l_pwrup_cnt
    }
    #[doc = "0xa4 - pmu scu_b power down count register"]
    #[inline(always)]
    pub const fn scu_b_pwrdn_cnt(&self) -> &ScuBPwrdnCnt {
        &self.scu_b_pwrdn_cnt
    }
    #[doc = "0xa8 - pmu scu_b power up count register"]
    #[inline(always)]
    pub const fn scu_b_pwrup_cnt(&self) -> &ScuBPwrupCnt {
        &self.scu_b_pwrup_cnt
    }
    #[doc = "0xac - pmu gpu power down count register"]
    #[inline(always)]
    pub const fn gpu_pwrdn_cnt(&self) -> &GpuPwrdnCnt {
        &self.gpu_pwrdn_cnt
    }
    #[doc = "0xb0 - pmu gpu power up count register"]
    #[inline(always)]
    pub const fn gpu_pwrup_cnt(&self) -> &GpuPwrupCnt {
        &self.gpu_pwrup_cnt
    }
    #[doc = "0xb4 - pmu center power down count register"]
    #[inline(always)]
    pub const fn center_pwrdn_cnt(&self) -> &CenterPwrdnCnt {
        &self.center_pwrdn_cnt
    }
    #[doc = "0xb8 - pmu center power up count register"]
    #[inline(always)]
    pub const fn center_pwrup_cnt(&self) -> &CenterPwrupCnt {
        &self.center_pwrup_cnt
    }
    #[doc = "0xbc - pmu timeout count register"]
    #[inline(always)]
    pub const fn timeout_cnt(&self) -> &TimeoutCnt {
        &self.timeout_cnt
    }
    #[doc = "0xc0 - pmu cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn cpu0apm_con(&self) -> &Cpu0apmCon {
        &self.cpu0apm_con
    }
    #[doc = "0xc4 - pmu cpu1 auto power down control register"]
    #[inline(always)]
    pub const fn cpu1apm_con(&self) -> &Cpu1apmCon {
        &self.cpu1apm_con
    }
    #[doc = "0xc8 - pmu cpu2 auto power down control register"]
    #[inline(always)]
    pub const fn cpu2apm_con(&self) -> &Cpu2apmCon {
        &self.cpu2apm_con
    }
    #[doc = "0xcc - pmu cpu3 auto power down control register"]
    #[inline(always)]
    pub const fn cpu3apm_con(&self) -> &Cpu3apmCon {
        &self.cpu3apm_con
    }
    #[doc = "0xd0 - pmu cluster_b cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn cpu0bpm_con(&self) -> &Cpu0bpmCon {
        &self.cpu0bpm_con
    }
    #[doc = "0xd4 - pmu cluster_b cpu0 auto power down control register"]
    #[inline(always)]
    pub const fn cpu1bpm_con(&self) -> &Cpu1bpmCon {
        &self.cpu1bpm_con
    }
    #[doc = "0xd8 - NOC auto domain clock gating disable enable register"]
    #[inline(always)]
    pub const fn noc_auto_ena(&self) -> &NocAutoEna {
        &self.noc_auto_ena
    }
    #[doc = "0xdc - pmu power down configure register1"]
    #[inline(always)]
    pub const fn pwrdn_con1(&self) -> &PwrdnCon1 {
        &self.pwrdn_con1
    }
    #[doc = "0xf0 - pmu system register 0"]
    #[inline(always)]
    pub const fn sys_reg0(&self) -> &SysReg0 {
        &self.sys_reg0
    }
    #[doc = "0xf4 - pmu system register 1"]
    #[inline(always)]
    pub const fn sys_reg1(&self) -> &SysReg1 {
        &self.sys_reg1
    }
    #[doc = "0xf8 - pmu system register 2"]
    #[inline(always)]
    pub const fn sys_reg2(&self) -> &SysReg2 {
        &self.sys_reg2
    }
    #[doc = "0xfc - pmu system register 3"]
    #[inline(always)]
    pub const fn sys_reg3(&self) -> &SysReg3 {
        &self.sys_reg3
    }
}
#[doc = "WAKEUP_CFG0 (rw) register accessor: pmu wakeup configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cfg0`]
module"]
#[doc(alias = "WAKEUP_CFG0")]
pub type WakeupCfg0 = crate::Reg<wakeup_cfg0::WakeupCfg0Spec>;
#[doc = "pmu wakeup configure register 0"]
pub mod wakeup_cfg0;
#[doc = "WAKEUP_CFG1 (rw) register accessor: pmu wakeup configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cfg1`]
module"]
#[doc(alias = "WAKEUP_CFG1")]
pub type WakeupCfg1 = crate::Reg<wakeup_cfg1::WakeupCfg1Spec>;
#[doc = "pmu wakeup configure register 1"]
pub mod wakeup_cfg1;
#[doc = "WAKEUP_CFG2 (rw) register accessor: pmu wakeup configure register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cfg2`]
module"]
#[doc(alias = "WAKEUP_CFG2")]
pub type WakeupCfg2 = crate::Reg<wakeup_cfg2::WakeupCfg2Spec>;
#[doc = "pmu wakeup configure register 2"]
pub mod wakeup_cfg2;
#[doc = "WAKEUP_CFG3 (rw) register accessor: pmu wakeup configure register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cfg3`]
module"]
#[doc(alias = "WAKEUP_CFG3")]
pub type WakeupCfg3 = crate::Reg<wakeup_cfg3::WakeupCfg3Spec>;
#[doc = "pmu wakeup configure register 3"]
pub mod wakeup_cfg3;
#[doc = "WAKEUP_CFG4 (rw) register accessor: pmu wakeup configure register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cfg4`]
module"]
#[doc(alias = "WAKEUP_CFG4")]
pub type WakeupCfg4 = crate::Reg<wakeup_cfg4::WakeupCfg4Spec>;
#[doc = "pmu wakeup configure register 4"]
pub mod wakeup_cfg4;
#[doc = "PWRDN_CON (rw) register accessor: pmu power down configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdn_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdn_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdn_con`]
module"]
#[doc(alias = "PWRDN_CON")]
pub type PwrdnCon = crate::Reg<pwrdn_con::PwrdnConSpec>;
#[doc = "pmu power down configure register"]
pub mod pwrdn_con;
#[doc = "PWRDN_ST (rw) register accessor: pmu power down status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdn_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdn_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdn_st`]
module"]
#[doc(alias = "PWRDN_ST")]
pub type PwrdnSt = crate::Reg<pwrdn_st::PwrdnStSpec>;
#[doc = "pmu power down status register"]
pub mod pwrdn_st;
#[doc = "PLL_CON (rw) register accessor: PLL low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_con`]
module"]
#[doc(alias = "PLL_CON")]
pub type PllCon = crate::Reg<pll_con::PllConSpec>;
#[doc = "PLL low power control register"]
pub mod pll_con;
#[doc = "PWRMODE_CON (rw) register accessor: pmu power mode configure register of common resource\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmode_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmode_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrmode_con`]
module"]
#[doc(alias = "PWRMODE_CON")]
pub type PwrmodeCon = crate::Reg<pwrmode_con::PwrmodeConSpec>;
#[doc = "pmu power mode configure register of common resource"]
pub mod pwrmode_con;
#[doc = "SFT_CON (rw) register accessor: pmu software configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sft_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sft_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sft_con`]
module"]
#[doc(alias = "SFT_CON")]
pub type SftCon = crate::Reg<sft_con::SftConSpec>;
#[doc = "pmu software configure register"]
pub mod sft_con;
#[doc = "INT_CON (rw) register accessor: pmu interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_con`]
module"]
#[doc(alias = "INT_CON")]
pub type IntCon = crate::Reg<int_con::IntConSpec>;
#[doc = "pmu interrupt configure register"]
pub mod int_con;
#[doc = "INT_ST (rw) register accessor: pmu interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`]
module"]
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::IntStSpec>;
#[doc = "pmu interrupt status register"]
pub mod int_st;
#[doc = "GPIO0_POS_INT_CON (rw) register accessor: pmu gpio0 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0_pos_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0_pos_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0_pos_int_con`]
module"]
#[doc(alias = "GPIO0_POS_INT_CON")]
pub type Gpio0PosIntCon = crate::Reg<gpio0_pos_int_con::Gpio0PosIntConSpec>;
#[doc = "pmu gpio0 posedge interrupt configure register"]
pub mod gpio0_pos_int_con;
#[doc = "GPIO0_NEG_INT_CON (rw) register accessor: pmu gpio0 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0_neg_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0_neg_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0_neg_int_con`]
module"]
#[doc(alias = "GPIO0_NEG_INT_CON")]
pub type Gpio0NegIntCon = crate::Reg<gpio0_neg_int_con::Gpio0NegIntConSpec>;
#[doc = "pmu gpio0 negedge interrupt configure register"]
pub mod gpio0_neg_int_con;
#[doc = "GPIO1_POS_INT_CON (rw) register accessor: pmu gpio1 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_pos_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_pos_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1_pos_int_con`]
module"]
#[doc(alias = "GPIO1_POS_INT_CON")]
pub type Gpio1PosIntCon = crate::Reg<gpio1_pos_int_con::Gpio1PosIntConSpec>;
#[doc = "pmu gpio1 posedge interrupt configure register"]
pub mod gpio1_pos_int_con;
#[doc = "GPIO1_NEG_INT_CON (rw) register accessor: pmu gpio1 negedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_neg_int_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_neg_int_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1_neg_int_con`]
module"]
#[doc(alias = "GPIO1_NEG_INT_CON")]
pub type Gpio1NegIntCon = crate::Reg<gpio1_neg_int_con::Gpio1NegIntConSpec>;
#[doc = "pmu gpio1 negedge interrupt configure register"]
pub mod gpio1_neg_int_con;
#[doc = "GPIO0_POS_INT_ST (rw) register accessor: pmu gpio0 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0_pos_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0_pos_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0_pos_int_st`]
module"]
#[doc(alias = "GPIO0_POS_INT_ST")]
pub type Gpio0PosIntSt = crate::Reg<gpio0_pos_int_st::Gpio0PosIntStSpec>;
#[doc = "pmu gpio0 posedge interrupt status register"]
pub mod gpio0_pos_int_st;
#[doc = "GPIO0_NEG_INT_ST (rw) register accessor: pmu gpio0 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0_neg_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0_neg_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0_neg_int_st`]
module"]
#[doc(alias = "GPIO0_NEG_INT_ST")]
pub type Gpio0NegIntSt = crate::Reg<gpio0_neg_int_st::Gpio0NegIntStSpec>;
#[doc = "pmu gpio0 negedge interrupt status register"]
pub mod gpio0_neg_int_st;
#[doc = "GPIO1_POS_INT_ST (rw) register accessor: pmu gpio1 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_pos_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_pos_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1_pos_int_st`]
module"]
#[doc(alias = "GPIO1_POS_INT_ST")]
pub type Gpio1PosIntSt = crate::Reg<gpio1_pos_int_st::Gpio1PosIntStSpec>;
#[doc = "pmu gpio1 posedge interrupt status register"]
pub mod gpio1_pos_int_st;
#[doc = "GPIO1_NEG_INT_ST (rw) register accessor: pmu gpio1 negedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_neg_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_neg_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1_neg_int_st`]
module"]
#[doc(alias = "GPIO1_NEG_INT_ST")]
pub type Gpio1NegIntSt = crate::Reg<gpio1_neg_int_st::Gpio1NegIntStSpec>;
#[doc = "pmu gpio1 negedge interrupt status register"]
pub mod gpio1_neg_int_st;
#[doc = "PWRDN_INTEN (rw) register accessor: pmu power down interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdn_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdn_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdn_inten`]
module"]
#[doc(alias = "PWRDN_INTEN")]
pub type PwrdnInten = crate::Reg<pwrdn_inten::PwrdnIntenSpec>;
#[doc = "pmu power down interrupt enable register"]
pub mod pwrdn_inten;
#[doc = "PWRDN_STATUS (rw) register accessor: pmu power down interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdn_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdn_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdn_status`]
module"]
#[doc(alias = "PWRDN_STATUS")]
pub type PwrdnStatus = crate::Reg<pwrdn_status::PwrdnStatusSpec>;
#[doc = "pmu power down interrupt status register"]
pub mod pwrdn_status;
#[doc = "WAKEUP_STATUS (rw) register accessor: pmu interrupt wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_status`]
module"]
#[doc(alias = "WAKEUP_STATUS")]
pub type WakeupStatus = crate::Reg<wakeup_status::WakeupStatusSpec>;
#[doc = "pmu interrupt wakeup status register"]
pub mod wakeup_status;
#[doc = "BUS_CLR (rw) register accessor: pmu bus clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clr`]
module"]
#[doc(alias = "BUS_CLR")]
pub type BusClr = crate::Reg<bus_clr::BusClrSpec>;
#[doc = "pmu bus clear register"]
pub mod bus_clr;
#[doc = "BUS_IDLE_REQ (rw) register accessor: pmu bus idle request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_idle_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_idle_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_idle_req`]
module"]
#[doc(alias = "BUS_IDLE_REQ")]
pub type BusIdleReq = crate::Reg<bus_idle_req::BusIdleReqSpec>;
#[doc = "pmu bus idle request register"]
pub mod bus_idle_req;
#[doc = "BUS_IDLE_ST (r) register accessor: pmu bus idle status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_idle_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_idle_st`]
module"]
#[doc(alias = "BUS_IDLE_ST")]
pub type BusIdleSt = crate::Reg<bus_idle_st::BusIdleStSpec>;
#[doc = "pmu bus idle status register"]
pub mod bus_idle_st;
#[doc = "BUS_IDLE_ACK (r) register accessor: pmu bus idle ack status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_idle_ack::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_idle_ack`]
module"]
#[doc(alias = "BUS_IDLE_ACK")]
pub type BusIdleAck = crate::Reg<bus_idle_ack::BusIdleAckSpec>;
#[doc = "pmu bus idle ack status register"]
pub mod bus_idle_ack;
#[doc = "CCI500_CON (rw) register accessor: CCI-500 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci500_con`]
module"]
#[doc(alias = "CCI500_CON")]
pub type Cci500Con = crate::Reg<cci500_con::Cci500ConSpec>;
#[doc = "CCI-500 low power control register"]
pub mod cci500_con;
#[doc = "ADB400_CON (rw) register accessor: adb-400 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adb400_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adb400_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adb400_con`]
module"]
#[doc(alias = "ADB400_CON")]
pub type Adb400Con = crate::Reg<adb400_con::Adb400ConSpec>;
#[doc = "adb-400 low power control register"]
pub mod adb400_con;
#[doc = "ADB400_ST (rw) register accessor: adb-400 low power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adb400_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adb400_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adb400_st`]
module"]
#[doc(alias = "ADB400_ST")]
pub type Adb400St = crate::Reg<adb400_st::Adb400StSpec>;
#[doc = "adb-400 low power status register"]
pub mod adb400_st;
#[doc = "POWER_ST (rw) register accessor: pmu power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_st`]
module"]
#[doc(alias = "POWER_ST")]
pub type PowerSt = crate::Reg<power_st::PowerStSpec>;
#[doc = "pmu power status register"]
pub mod power_st;
#[doc = "CORE_PWR_ST (r) register accessor: pmu core power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_pwr_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_pwr_st`]
module"]
#[doc(alias = "CORE_PWR_ST")]
pub type CorePwrSt = crate::Reg<core_pwr_st::CorePwrStSpec>;
#[doc = "pmu core power status register"]
pub mod core_pwr_st;
#[doc = "OSC_CNT (rw) register accessor: pmu osc count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc_cnt`]
module"]
#[doc(alias = "OSC_CNT")]
pub type OscCnt = crate::Reg<osc_cnt::OscCntSpec>;
#[doc = "pmu osc count register"]
pub mod osc_cnt;
#[doc = "PLLLOCK_CNT (rw) register accessor: pmu pll lock count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plllock_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plllock_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plllock_cnt`]
module"]
#[doc(alias = "PLLLOCK_CNT")]
pub type PlllockCnt = crate::Reg<plllock_cnt::PlllockCntSpec>;
#[doc = "pmu pll lock count register"]
pub mod plllock_cnt;
#[doc = "PLLRST_CNT (rw) register accessor: pmu pll reset count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllrst_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllrst_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllrst_cnt`]
module"]
#[doc(alias = "PLLRST_CNT")]
pub type PllrstCnt = crate::Reg<pllrst_cnt::PllrstCntSpec>;
#[doc = "pmu pll reset count register"]
pub mod pllrst_cnt;
#[doc = "STABLE_CNT (rw) register accessor: pmu power stable count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stable_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stable_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stable_cnt`]
module"]
#[doc(alias = "STABLE_CNT")]
pub type StableCnt = crate::Reg<stable_cnt::StableCntSpec>;
#[doc = "pmu power stable count register"]
pub mod stable_cnt;
#[doc = "DDRIO_PWRON_CNT (rw) register accessor: pmu ddrio power on count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrio_pwron_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrio_pwron_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrio_pwron_cnt`]
module"]
#[doc(alias = "DDRIO_PWRON_CNT")]
pub type DdrioPwronCnt = crate::Reg<ddrio_pwron_cnt::DdrioPwronCntSpec>;
#[doc = "pmu ddrio power on count register"]
pub mod ddrio_pwron_cnt;
#[doc = "WAKEUP_RST_CLR_CNT (rw) register accessor: pmu wakeup reset clear count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_rst_clr_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_rst_clr_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_rst_clr_cnt`]
module"]
#[doc(alias = "WAKEUP_RST_CLR_CNT")]
pub type WakeupRstClrCnt = crate::Reg<wakeup_rst_clr_cnt::WakeupRstClrCntSpec>;
#[doc = "pmu wakeup reset clear count register"]
pub mod wakeup_rst_clr_cnt;
#[doc = "DDR_SREF_ST (rw) register accessor: pmu ddr self refresh status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_sref_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_sref_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_sref_st`]
module"]
#[doc(alias = "DDR_SREF_ST")]
pub type DdrSrefSt = crate::Reg<ddr_sref_st::DdrSrefStSpec>;
#[doc = "pmu ddr self refresh status register"]
pub mod ddr_sref_st;
#[doc = "SCU_L_PWRDN_CNT (rw) register accessor: pmu scu_l power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_l_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_l_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu_l_pwrdn_cnt`]
module"]
#[doc(alias = "SCU_L_PWRDN_CNT")]
pub type ScuLPwrdnCnt = crate::Reg<scu_l_pwrdn_cnt::ScuLPwrdnCntSpec>;
#[doc = "pmu scu_l power down count register"]
pub mod scu_l_pwrdn_cnt;
#[doc = "SCU_L_PWRUP_CNT (rw) register accessor: pmu scu_l power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_l_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_l_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu_l_pwrup_cnt`]
module"]
#[doc(alias = "SCU_L_PWRUP_CNT")]
pub type ScuLPwrupCnt = crate::Reg<scu_l_pwrup_cnt::ScuLPwrupCntSpec>;
#[doc = "pmu scu_l power up count register"]
pub mod scu_l_pwrup_cnt;
#[doc = "SCU_B_PWRDN_CNT (rw) register accessor: pmu scu_b power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_b_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_b_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu_b_pwrdn_cnt`]
module"]
#[doc(alias = "SCU_B_PWRDN_CNT")]
pub type ScuBPwrdnCnt = crate::Reg<scu_b_pwrdn_cnt::ScuBPwrdnCntSpec>;
#[doc = "pmu scu_b power down count register"]
pub mod scu_b_pwrdn_cnt;
#[doc = "SCU_B_PWRUP_CNT (rw) register accessor: pmu scu_b power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scu_b_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scu_b_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu_b_pwrup_cnt`]
module"]
#[doc(alias = "SCU_B_PWRUP_CNT")]
pub type ScuBPwrupCnt = crate::Reg<scu_b_pwrup_cnt::ScuBPwrupCntSpec>;
#[doc = "pmu scu_b power up count register"]
pub mod scu_b_pwrup_cnt;
#[doc = "GPU_PWRDN_CNT (rw) register accessor: pmu gpu power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_pwrdn_cnt`]
module"]
#[doc(alias = "GPU_PWRDN_CNT")]
pub type GpuPwrdnCnt = crate::Reg<gpu_pwrdn_cnt::GpuPwrdnCntSpec>;
#[doc = "pmu gpu power down count register"]
pub mod gpu_pwrdn_cnt;
#[doc = "GPU_PWRUP_CNT (rw) register accessor: pmu gpu power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_pwrup_cnt`]
module"]
#[doc(alias = "GPU_PWRUP_CNT")]
pub type GpuPwrupCnt = crate::Reg<gpu_pwrup_cnt::GpuPwrupCntSpec>;
#[doc = "pmu gpu power up count register"]
pub mod gpu_pwrup_cnt;
#[doc = "CENTER_PWRDN_CNT (rw) register accessor: pmu center power down count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`center_pwrdn_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`center_pwrdn_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@center_pwrdn_cnt`]
module"]
#[doc(alias = "CENTER_PWRDN_CNT")]
pub type CenterPwrdnCnt = crate::Reg<center_pwrdn_cnt::CenterPwrdnCntSpec>;
#[doc = "pmu center power down count register"]
pub mod center_pwrdn_cnt;
#[doc = "CENTER_PWRUP_CNT (rw) register accessor: pmu center power up count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`center_pwrup_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`center_pwrup_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@center_pwrup_cnt`]
module"]
#[doc(alias = "CENTER_PWRUP_CNT")]
pub type CenterPwrupCnt = crate::Reg<center_pwrup_cnt::CenterPwrupCntSpec>;
#[doc = "pmu center power up count register"]
pub mod center_pwrup_cnt;
#[doc = "TIMEOUT_CNT (rw) register accessor: pmu timeout count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_cnt`]
module"]
#[doc(alias = "TIMEOUT_CNT")]
pub type TimeoutCnt = crate::Reg<timeout_cnt::TimeoutCntSpec>;
#[doc = "pmu timeout count register"]
pub mod timeout_cnt;
#[doc = "CPU0APM_CON (rw) register accessor: pmu cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu0apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu0apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0apm_con`]
module"]
#[doc(alias = "CPU0APM_CON")]
pub type Cpu0apmCon = crate::Reg<cpu0apm_con::Cpu0apmConSpec>;
#[doc = "pmu cpu0 auto power down control register"]
pub mod cpu0apm_con;
#[doc = "CPU1APM_CON (rw) register accessor: pmu cpu1 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu1apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu1apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1apm_con`]
module"]
#[doc(alias = "CPU1APM_CON")]
pub type Cpu1apmCon = crate::Reg<cpu1apm_con::Cpu1apmConSpec>;
#[doc = "pmu cpu1 auto power down control register"]
pub mod cpu1apm_con;
#[doc = "CPU2APM_CON (rw) register accessor: pmu cpu2 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu2apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu2apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu2apm_con`]
module"]
#[doc(alias = "CPU2APM_CON")]
pub type Cpu2apmCon = crate::Reg<cpu2apm_con::Cpu2apmConSpec>;
#[doc = "pmu cpu2 auto power down control register"]
pub mod cpu2apm_con;
#[doc = "CPU3APM_CON (rw) register accessor: pmu cpu3 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu3apm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu3apm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu3apm_con`]
module"]
#[doc(alias = "CPU3APM_CON")]
pub type Cpu3apmCon = crate::Reg<cpu3apm_con::Cpu3apmConSpec>;
#[doc = "pmu cpu3 auto power down control register"]
pub mod cpu3apm_con;
#[doc = "CPU0BPM_CON (rw) register accessor: pmu cluster_b cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu0bpm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu0bpm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0bpm_con`]
module"]
#[doc(alias = "CPU0BPM_CON")]
pub type Cpu0bpmCon = crate::Reg<cpu0bpm_con::Cpu0bpmConSpec>;
#[doc = "pmu cluster_b cpu0 auto power down control register"]
pub mod cpu0bpm_con;
#[doc = "CPU1BPM_CON (rw) register accessor: pmu cluster_b cpu0 auto power down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu1bpm_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu1bpm_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1bpm_con`]
module"]
#[doc(alias = "CPU1BPM_CON")]
pub type Cpu1bpmCon = crate::Reg<cpu1bpm_con::Cpu1bpmConSpec>;
#[doc = "pmu cluster_b cpu0 auto power down control register"]
pub mod cpu1bpm_con;
#[doc = "NOC_AUTO_ENA (rw) register accessor: NOC auto domain clock gating disable enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_auto_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_auto_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_auto_ena`]
module"]
#[doc(alias = "NOC_AUTO_ENA")]
pub type NocAutoEna = crate::Reg<noc_auto_ena::NocAutoEnaSpec>;
#[doc = "NOC auto domain clock gating disable enable register"]
pub mod noc_auto_ena;
#[doc = "PWRDN_CON1 (rw) register accessor: pmu power down configure register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdn_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdn_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdn_con1`]
module"]
#[doc(alias = "PWRDN_CON1")]
pub type PwrdnCon1 = crate::Reg<pwrdn_con1::PwrdnCon1Spec>;
#[doc = "pmu power down configure register1"]
pub mod pwrdn_con1;
#[doc = "SYS_REG0 (rw) register accessor: pmu system register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reg0`]
module"]
#[doc(alias = "SYS_REG0")]
pub type SysReg0 = crate::Reg<sys_reg0::SysReg0Spec>;
#[doc = "pmu system register 0"]
pub mod sys_reg0;
#[doc = "SYS_REG1 (rw) register accessor: pmu system register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reg1`]
module"]
#[doc(alias = "SYS_REG1")]
pub type SysReg1 = crate::Reg<sys_reg1::SysReg1Spec>;
#[doc = "pmu system register 1"]
pub mod sys_reg1;
#[doc = "SYS_REG2 (rw) register accessor: pmu system register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reg2`]
module"]
#[doc(alias = "SYS_REG2")]
pub type SysReg2 = crate::Reg<sys_reg2::SysReg2Spec>;
#[doc = "pmu system register 2"]
pub mod sys_reg2;
#[doc = "SYS_REG3 (rw) register accessor: pmu system register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reg3`]
module"]
#[doc(alias = "SYS_REG3")]
pub type SysReg3 = crate::Reg<sys_reg3::SysReg3Spec>;
#[doc = "pmu system register 3"]
pub mod sys_reg3;
