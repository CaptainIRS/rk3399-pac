#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmugrf_gpio0a_iomux: PmugrfGpio0aIomux,
    pmugrf_gpio0b_iomux: PmugrfGpio0bIomux,
    _reserved2: [u8; 0x08],
    pmugrf_gpio1a_iomux: PmugrfGpio1aIomux,
    pmugrf_gpio1b_iomux: PmugrfGpio1bIomux,
    pmugrf_gpio1c_iomux: PmugrfGpio1cIomux,
    pmugrf_gpio1d_iomux: PmugrfGpio1dIomux,
    _reserved6: [u8; 0x20],
    pmugrf_gpio0a_p: PmugrfGpio0aP,
    pmugrf_gpio0b_p: PmugrfGpio0bP,
    _reserved8: [u8; 0x08],
    pmugrf_gpio1a_p: PmugrfGpio1aP,
    pmugrf_gpio1b_p: PmugrfGpio1bP,
    pmugrf_gpio1c_p: PmugrfGpio1cP,
    pmugrf_gpio1d_p: PmugrfGpio1dP,
    _reserved12: [u8; 0x20],
    pmugrf_gpio0a_e: PmugrfGpio0aE,
    _reserved13: [u8; 0x04],
    pmugrf_gpio0b_e: PmugrfGpio0bE,
    _reserved14: [u8; 0x14],
    pmugrf_gpio1a_e: PmugrfGpio1aE,
    _reserved15: [u8; 0x04],
    pmugrf_gpio1b_e: PmugrfGpio1bE,
    _reserved16: [u8; 0x04],
    pmugrf_gpio1c_e: PmugrfGpio1cE,
    _reserved17: [u8; 0x04],
    pmugrf_gpio1d_e: PmugrfGpio1dE,
    _reserved18: [u8; 0x44],
    pmugrf_gpio0l_sr: PmugrfGpio0lSr,
    _reserved19: [u8; 0x04],
    pmugrf_gpio1l_sr: PmugrfGpio1lSr,
    pmugrf_gpio1h_sr: PmugrfGpio1hSr,
    _reserved21: [u8; 0x10],
    pmugrf_gpio0a_smt: PmugrfGpio0aSmt,
    pmugrf_gpio0b_smt: PmugrfGpio0bSmt,
    _reserved23: [u8; 0x08],
    pmugrf_gpio1a_smt: PmugrfGpio1aSmt,
    pmugrf_gpio1b_smt: PmugrfGpio1bSmt,
    pmugrf_gpio1c_smt: PmugrfGpio1cSmt,
    pmugrf_gpio1d_smt: PmugrfGpio1dSmt,
    _reserved27: [u8; 0x20],
    pmugrf_gpio0l_he: PmugrfGpio0lHe,
    _reserved28: [u8; 0x04],
    pmugrf_gpio1l_he: PmugrfGpio1lHe,
    pmugrf_gpio1h_he: PmugrfGpio1hHe,
    _reserved30: [u8; 0x10],
    pmugrf_soc_con0: PmugrfSocCon0,
    _reserved31: [u8; 0x24],
    pmugrf_soc_con10: PmugrfSocCon10,
    pmugrf_soc_con11: PmugrfSocCon11,
    _reserved33: [u8; 0x90],
    pmugrf_pmupvtm_con0: PmugrfPmupvtmCon0,
    pmugrf_pmupvtm_con1: PmugrfPmupvtmCon1,
    pmugrf_pmupvtm_status0: PmugrfPmupvtmStatus0,
    pmugrf_pmupvtm_status1: PmugrfPmupvtmStatus1,
    pmugrf_osc_e: PmugrfOscE,
    _reserved38: [u8; 0xac],
    pmugrf_os_reg0: PmugrfOsReg0,
    pmugrf_os_reg1: PmugrfOsReg1,
    pmugrf_os_reg2: PmugrfOsReg2,
    pmugrf_os_reg3: PmugrfOsReg3,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO0A iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0a_iomux(&self) -> &PmugrfGpio0aIomux {
        &self.pmugrf_gpio0a_iomux
    }
    #[doc = "0x04 - GPIO0B iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0b_iomux(&self) -> &PmugrfGpio0bIomux {
        &self.pmugrf_gpio0b_iomux
    }
    #[doc = "0x10 - GPIO1A iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1a_iomux(&self) -> &PmugrfGpio1aIomux {
        &self.pmugrf_gpio1a_iomux
    }
    #[doc = "0x14 - GPIO1B iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1b_iomux(&self) -> &PmugrfGpio1bIomux {
        &self.pmugrf_gpio1b_iomux
    }
    #[doc = "0x18 - GPIO1C iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1c_iomux(&self) -> &PmugrfGpio1cIomux {
        &self.pmugrf_gpio1c_iomux
    }
    #[doc = "0x1c - GPIO1D iomux control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1d_iomux(&self) -> &PmugrfGpio1dIomux {
        &self.pmugrf_gpio1d_iomux
    }
    #[doc = "0x40 - GPIO0A PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0a_p(&self) -> &PmugrfGpio0aP {
        &self.pmugrf_gpio0a_p
    }
    #[doc = "0x44 - GPIO0B PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0b_p(&self) -> &PmugrfGpio0bP {
        &self.pmugrf_gpio0b_p
    }
    #[doc = "0x50 - GPIO1A PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1a_p(&self) -> &PmugrfGpio1aP {
        &self.pmugrf_gpio1a_p
    }
    #[doc = "0x54 - GPIO1B PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1b_p(&self) -> &PmugrfGpio1bP {
        &self.pmugrf_gpio1b_p
    }
    #[doc = "0x58 - GPIO1C PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1c_p(&self) -> &PmugrfGpio1cP {
        &self.pmugrf_gpio1c_p
    }
    #[doc = "0x5c - GPIO0D PU/PD control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1d_p(&self) -> &PmugrfGpio1dP {
        &self.pmugrf_gpio1d_p
    }
    #[doc = "0x80 - GPIO0A drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0a_e(&self) -> &PmugrfGpio0aE {
        &self.pmugrf_gpio0a_e
    }
    #[doc = "0x88 - GPIO0D drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0b_e(&self) -> &PmugrfGpio0bE {
        &self.pmugrf_gpio0b_e
    }
    #[doc = "0xa0 - GPIO1A drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1a_e(&self) -> &PmugrfGpio1aE {
        &self.pmugrf_gpio1a_e
    }
    #[doc = "0xa8 - GPIO1D drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1b_e(&self) -> &PmugrfGpio1bE {
        &self.pmugrf_gpio1b_e
    }
    #[doc = "0xb0 - GPIO1C drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1c_e(&self) -> &PmugrfGpio1cE {
        &self.pmugrf_gpio1c_e
    }
    #[doc = "0xb8 - GPIO1D drive strength control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1d_e(&self) -> &PmugrfGpio1dE {
        &self.pmugrf_gpio1d_e
    }
    #[doc = "0x100 - GPIO0 A/B SR control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0l_sr(&self) -> &PmugrfGpio0lSr {
        &self.pmugrf_gpio0l_sr
    }
    #[doc = "0x108 - GPIO1 A/B SR control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1l_sr(&self) -> &PmugrfGpio1lSr {
        &self.pmugrf_gpio1l_sr
    }
    #[doc = "0x10c - GPIO1C/D SR control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1h_sr(&self) -> &PmugrfGpio1hSr {
        &self.pmugrf_gpio1h_sr
    }
    #[doc = "0x120 - GPIO0A smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0a_smt(&self) -> &PmugrfGpio0aSmt {
        &self.pmugrf_gpio0a_smt
    }
    #[doc = "0x124 - GPIO0B smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0b_smt(&self) -> &PmugrfGpio0bSmt {
        &self.pmugrf_gpio0b_smt
    }
    #[doc = "0x130 - GPIO1A smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1a_smt(&self) -> &PmugrfGpio1aSmt {
        &self.pmugrf_gpio1a_smt
    }
    #[doc = "0x134 - GPIO1B smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1b_smt(&self) -> &PmugrfGpio1bSmt {
        &self.pmugrf_gpio1b_smt
    }
    #[doc = "0x138 - GPIO1C smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1c_smt(&self) -> &PmugrfGpio1cSmt {
        &self.pmugrf_gpio1c_smt
    }
    #[doc = "0x13c - GPIO1D smit control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1d_smt(&self) -> &PmugrfGpio1dSmt {
        &self.pmugrf_gpio1d_smt
    }
    #[doc = "0x160 - GPIO0 A/B HE control"]
    #[inline(always)]
    pub const fn pmugrf_gpio0l_he(&self) -> &PmugrfGpio0lHe {
        &self.pmugrf_gpio0l_he
    }
    #[doc = "0x168 - GPIO1 A/B HE control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1l_he(&self) -> &PmugrfGpio1lHe {
        &self.pmugrf_gpio1l_he
    }
    #[doc = "0x16c - GPIO1C/D HE control"]
    #[inline(always)]
    pub const fn pmugrf_gpio1h_he(&self) -> &PmugrfGpio1hHe {
        &self.pmugrf_gpio1h_he
    }
    #[doc = "0x180 - SoC control register 0"]
    #[inline(always)]
    pub const fn pmugrf_soc_con0(&self) -> &PmugrfSocCon0 {
        &self.pmugrf_soc_con0
    }
    #[doc = "0x1a8 - SoC control register 10"]
    #[inline(always)]
    pub const fn pmugrf_soc_con10(&self) -> &PmugrfSocCon10 {
        &self.pmugrf_soc_con10
    }
    #[doc = "0x1ac - SoC control register 11"]
    #[inline(always)]
    pub const fn pmugrf_soc_con11(&self) -> &PmugrfSocCon11 {
        &self.pmugrf_soc_con11
    }
    #[doc = "0x240 - pmu pvtm configuration register0"]
    #[inline(always)]
    pub const fn pmugrf_pmupvtm_con0(&self) -> &PmugrfPmupvtmCon0 {
        &self.pmugrf_pmupvtm_con0
    }
    #[doc = "0x244 - pmu pvtm configuration register1"]
    #[inline(always)]
    pub const fn pmugrf_pmupvtm_con1(&self) -> &PmugrfPmupvtmCon1 {
        &self.pmugrf_pmupvtm_con1
    }
    #[doc = "0x248 - pmu pvtm status register"]
    #[inline(always)]
    pub const fn pmugrf_pmupvtm_status0(&self) -> &PmugrfPmupvtmStatus0 {
        &self.pmugrf_pmupvtm_status0
    }
    #[doc = "0x24c - pmu pvtm status register"]
    #[inline(always)]
    pub const fn pmugrf_pmupvtm_status1(&self) -> &PmugrfPmupvtmStatus1 {
        &self.pmugrf_pmupvtm_status1
    }
    #[doc = "0x250 - OSC control register"]
    #[inline(always)]
    pub const fn pmugrf_osc_e(&self) -> &PmugrfOscE {
        &self.pmugrf_osc_e
    }
    #[doc = "0x300 - os register"]
    #[inline(always)]
    pub const fn pmugrf_os_reg0(&self) -> &PmugrfOsReg0 {
        &self.pmugrf_os_reg0
    }
    #[doc = "0x304 - os register"]
    #[inline(always)]
    pub const fn pmugrf_os_reg1(&self) -> &PmugrfOsReg1 {
        &self.pmugrf_os_reg1
    }
    #[doc = "0x308 - os register"]
    #[inline(always)]
    pub const fn pmugrf_os_reg2(&self) -> &PmugrfOsReg2 {
        &self.pmugrf_os_reg2
    }
    #[doc = "0x30c - os register"]
    #[inline(always)]
    pub const fn pmugrf_os_reg3(&self) -> &PmugrfOsReg3 {
        &self.pmugrf_os_reg3
    }
}
#[doc = "PMUGRF_GPIO0A_IOMUX (rw) register accessor: GPIO0A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0a_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO0A_IOMUX")]
pub type PmugrfGpio0aIomux = crate::Reg<pmugrf_gpio0a_iomux::PmugrfGpio0aIomuxSpec>;
#[doc = "GPIO0A iomux control"]
pub mod pmugrf_gpio0a_iomux;
#[doc = "PMUGRF_GPIO0B_IOMUX (rw) register accessor: GPIO0B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0b_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO0B_IOMUX")]
pub type PmugrfGpio0bIomux = crate::Reg<pmugrf_gpio0b_iomux::PmugrfGpio0bIomuxSpec>;
#[doc = "GPIO0B iomux control"]
pub mod pmugrf_gpio0b_iomux;
#[doc = "PMUGRF_GPIO1A_IOMUX (rw) register accessor: GPIO1A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1a_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO1A_IOMUX")]
pub type PmugrfGpio1aIomux = crate::Reg<pmugrf_gpio1a_iomux::PmugrfGpio1aIomuxSpec>;
#[doc = "GPIO1A iomux control"]
pub mod pmugrf_gpio1a_iomux;
#[doc = "PMUGRF_GPIO1B_IOMUX (rw) register accessor: GPIO1B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1b_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO1B_IOMUX")]
pub type PmugrfGpio1bIomux = crate::Reg<pmugrf_gpio1b_iomux::PmugrfGpio1bIomuxSpec>;
#[doc = "GPIO1B iomux control"]
pub mod pmugrf_gpio1b_iomux;
#[doc = "PMUGRF_GPIO1C_IOMUX (rw) register accessor: GPIO1C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1c_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO1C_IOMUX")]
pub type PmugrfGpio1cIomux = crate::Reg<pmugrf_gpio1c_iomux::PmugrfGpio1cIomuxSpec>;
#[doc = "GPIO1C iomux control"]
pub mod pmugrf_gpio1c_iomux;
#[doc = "PMUGRF_GPIO1D_IOMUX (rw) register accessor: GPIO1D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1d_iomux`]
module"]
#[doc(alias = "PMUGRF_GPIO1D_IOMUX")]
pub type PmugrfGpio1dIomux = crate::Reg<pmugrf_gpio1d_iomux::PmugrfGpio1dIomuxSpec>;
#[doc = "GPIO1D iomux control"]
pub mod pmugrf_gpio1d_iomux;
#[doc = "PMUGRF_GPIO0A_P (rw) register accessor: GPIO0A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0a_p`]
module"]
#[doc(alias = "PMUGRF_GPIO0A_P")]
pub type PmugrfGpio0aP = crate::Reg<pmugrf_gpio0a_p::PmugrfGpio0aPSpec>;
#[doc = "GPIO0A PU/PD control"]
pub mod pmugrf_gpio0a_p;
#[doc = "PMUGRF_GPIO0B_P (rw) register accessor: GPIO0B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0b_p`]
module"]
#[doc(alias = "PMUGRF_GPIO0B_P")]
pub type PmugrfGpio0bP = crate::Reg<pmugrf_gpio0b_p::PmugrfGpio0bPSpec>;
#[doc = "GPIO0B PU/PD control"]
pub mod pmugrf_gpio0b_p;
#[doc = "PMUGRF_GPIO1A_P (rw) register accessor: GPIO1A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1a_p`]
module"]
#[doc(alias = "PMUGRF_GPIO1A_P")]
pub type PmugrfGpio1aP = crate::Reg<pmugrf_gpio1a_p::PmugrfGpio1aPSpec>;
#[doc = "GPIO1A PU/PD control"]
pub mod pmugrf_gpio1a_p;
#[doc = "PMUGRF_GPIO1B_P (rw) register accessor: GPIO1B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1b_p`]
module"]
#[doc(alias = "PMUGRF_GPIO1B_P")]
pub type PmugrfGpio1bP = crate::Reg<pmugrf_gpio1b_p::PmugrfGpio1bPSpec>;
#[doc = "GPIO1B PU/PD control"]
pub mod pmugrf_gpio1b_p;
#[doc = "PMUGRF_GPIO1C_P (rw) register accessor: GPIO1C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1c_p`]
module"]
#[doc(alias = "PMUGRF_GPIO1C_P")]
pub type PmugrfGpio1cP = crate::Reg<pmugrf_gpio1c_p::PmugrfGpio1cPSpec>;
#[doc = "GPIO1C PU/PD control"]
pub mod pmugrf_gpio1c_p;
#[doc = "PMUGRF_GPIO1D_P (rw) register accessor: GPIO0D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1d_p`]
module"]
#[doc(alias = "PMUGRF_GPIO1D_P")]
pub type PmugrfGpio1dP = crate::Reg<pmugrf_gpio1d_p::PmugrfGpio1dPSpec>;
#[doc = "GPIO0D PU/PD control"]
pub mod pmugrf_gpio1d_p;
#[doc = "PMUGRF_GPIO0A_E (rw) register accessor: GPIO0A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0a_e`]
module"]
#[doc(alias = "PMUGRF_GPIO0A_E")]
pub type PmugrfGpio0aE = crate::Reg<pmugrf_gpio0a_e::PmugrfGpio0aESpec>;
#[doc = "GPIO0A drive strength control"]
pub mod pmugrf_gpio0a_e;
#[doc = "PMUGRF_GPIO0B_E (rw) register accessor: GPIO0D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0b_e`]
module"]
#[doc(alias = "PMUGRF_GPIO0B_E")]
pub type PmugrfGpio0bE = crate::Reg<pmugrf_gpio0b_e::PmugrfGpio0bESpec>;
#[doc = "GPIO0D drive strength control"]
pub mod pmugrf_gpio0b_e;
#[doc = "PMUGRF_GPIO1A_E (rw) register accessor: GPIO1A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1a_e`]
module"]
#[doc(alias = "PMUGRF_GPIO1A_E")]
pub type PmugrfGpio1aE = crate::Reg<pmugrf_gpio1a_e::PmugrfGpio1aESpec>;
#[doc = "GPIO1A drive strength control"]
pub mod pmugrf_gpio1a_e;
#[doc = "PMUGRF_GPIO1B_E (rw) register accessor: GPIO1D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1b_e`]
module"]
#[doc(alias = "PMUGRF_GPIO1B_E")]
pub type PmugrfGpio1bE = crate::Reg<pmugrf_gpio1b_e::PmugrfGpio1bESpec>;
#[doc = "GPIO1D drive strength control"]
pub mod pmugrf_gpio1b_e;
#[doc = "PMUGRF_GPIO1C_E (rw) register accessor: GPIO1C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1c_e`]
module"]
#[doc(alias = "PMUGRF_GPIO1C_E")]
pub type PmugrfGpio1cE = crate::Reg<pmugrf_gpio1c_e::PmugrfGpio1cESpec>;
#[doc = "GPIO1C drive strength control"]
pub mod pmugrf_gpio1c_e;
#[doc = "PMUGRF_GPIO1D_E (rw) register accessor: GPIO1D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1d_e`]
module"]
#[doc(alias = "PMUGRF_GPIO1D_E")]
pub type PmugrfGpio1dE = crate::Reg<pmugrf_gpio1d_e::PmugrfGpio1dESpec>;
#[doc = "GPIO1D drive strength control"]
pub mod pmugrf_gpio1d_e;
#[doc = "PMUGRF_GPIO0L_SR (rw) register accessor: GPIO0 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0l_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0l_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0l_sr`]
module"]
#[doc(alias = "PMUGRF_GPIO0L_SR")]
pub type PmugrfGpio0lSr = crate::Reg<pmugrf_gpio0l_sr::PmugrfGpio0lSrSpec>;
#[doc = "GPIO0 A/B SR control"]
pub mod pmugrf_gpio0l_sr;
#[doc = "PMUGRF_GPIO1L_SR (rw) register accessor: GPIO1 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1l_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1l_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1l_sr`]
module"]
#[doc(alias = "PMUGRF_GPIO1L_SR")]
pub type PmugrfGpio1lSr = crate::Reg<pmugrf_gpio1l_sr::PmugrfGpio1lSrSpec>;
#[doc = "GPIO1 A/B SR control"]
pub mod pmugrf_gpio1l_sr;
#[doc = "PMUGRF_GPIO1H_SR (rw) register accessor: GPIO1C/D SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1h_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1h_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1h_sr`]
module"]
#[doc(alias = "PMUGRF_GPIO1H_SR")]
pub type PmugrfGpio1hSr = crate::Reg<pmugrf_gpio1h_sr::PmugrfGpio1hSrSpec>;
#[doc = "GPIO1C/D SR control"]
pub mod pmugrf_gpio1h_sr;
#[doc = "PMUGRF_GPIO0A_SMT (rw) register accessor: GPIO0A smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0a_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO0A_SMT")]
pub type PmugrfGpio0aSmt = crate::Reg<pmugrf_gpio0a_smt::PmugrfGpio0aSmtSpec>;
#[doc = "GPIO0A smit control"]
pub mod pmugrf_gpio0a_smt;
#[doc = "PMUGRF_GPIO0B_SMT (rw) register accessor: GPIO0B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0b_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO0B_SMT")]
pub type PmugrfGpio0bSmt = crate::Reg<pmugrf_gpio0b_smt::PmugrfGpio0bSmtSpec>;
#[doc = "GPIO0B smit control"]
pub mod pmugrf_gpio0b_smt;
#[doc = "PMUGRF_GPIO1A_SMT (rw) register accessor: GPIO1A smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1a_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO1A_SMT")]
pub type PmugrfGpio1aSmt = crate::Reg<pmugrf_gpio1a_smt::PmugrfGpio1aSmtSpec>;
#[doc = "GPIO1A smit control"]
pub mod pmugrf_gpio1a_smt;
#[doc = "PMUGRF_GPIO1B_SMT (rw) register accessor: GPIO1B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1b_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO1B_SMT")]
pub type PmugrfGpio1bSmt = crate::Reg<pmugrf_gpio1b_smt::PmugrfGpio1bSmtSpec>;
#[doc = "GPIO1B smit control"]
pub mod pmugrf_gpio1b_smt;
#[doc = "PMUGRF_GPIO1C_SMT (rw) register accessor: GPIO1C smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1c_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO1C_SMT")]
pub type PmugrfGpio1cSmt = crate::Reg<pmugrf_gpio1c_smt::PmugrfGpio1cSmtSpec>;
#[doc = "GPIO1C smit control"]
pub mod pmugrf_gpio1c_smt;
#[doc = "PMUGRF_GPIO1D_SMT (rw) register accessor: GPIO1D smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1d_smt`]
module"]
#[doc(alias = "PMUGRF_GPIO1D_SMT")]
pub type PmugrfGpio1dSmt = crate::Reg<pmugrf_gpio1d_smt::PmugrfGpio1dSmtSpec>;
#[doc = "GPIO1D smit control"]
pub mod pmugrf_gpio1d_smt;
#[doc = "PMUGRF_GPIO0L_HE (rw) register accessor: GPIO0 A/B HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0l_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0l_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio0l_he`]
module"]
#[doc(alias = "PMUGRF_GPIO0L_HE")]
pub type PmugrfGpio0lHe = crate::Reg<pmugrf_gpio0l_he::PmugrfGpio0lHeSpec>;
#[doc = "GPIO0 A/B HE control"]
pub mod pmugrf_gpio0l_he;
#[doc = "PMUGRF_GPIO1L_HE (rw) register accessor: GPIO1 A/B HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1l_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1l_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1l_he`]
module"]
#[doc(alias = "PMUGRF_GPIO1L_HE")]
pub type PmugrfGpio1lHe = crate::Reg<pmugrf_gpio1l_he::PmugrfGpio1lHeSpec>;
#[doc = "GPIO1 A/B HE control"]
pub mod pmugrf_gpio1l_he;
#[doc = "PMUGRF_GPIO1H_HE (rw) register accessor: GPIO1C/D HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1h_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1h_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_gpio1h_he`]
module"]
#[doc(alias = "PMUGRF_GPIO1H_HE")]
pub type PmugrfGpio1hHe = crate::Reg<pmugrf_gpio1h_he::PmugrfGpio1hHeSpec>;
#[doc = "GPIO1C/D HE control"]
pub mod pmugrf_gpio1h_he;
#[doc = "PMUGRF_SOC_CON0 (rw) register accessor: SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_soc_con0`]
module"]
#[doc(alias = "PMUGRF_SOC_CON0")]
pub type PmugrfSocCon0 = crate::Reg<pmugrf_soc_con0::PmugrfSocCon0Spec>;
#[doc = "SoC control register 0"]
pub mod pmugrf_soc_con0;
#[doc = "PMUGRF_SOC_CON10 (rw) register accessor: SoC control register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_soc_con10`]
module"]
#[doc(alias = "PMUGRF_SOC_CON10")]
pub type PmugrfSocCon10 = crate::Reg<pmugrf_soc_con10::PmugrfSocCon10Spec>;
#[doc = "SoC control register 10"]
pub mod pmugrf_soc_con10;
#[doc = "PMUGRF_SOC_CON11 (rw) register accessor: SoC control register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_soc_con11`]
module"]
#[doc(alias = "PMUGRF_SOC_CON11")]
pub type PmugrfSocCon11 = crate::Reg<pmugrf_soc_con11::PmugrfSocCon11Spec>;
#[doc = "SoC control register 11"]
pub mod pmugrf_soc_con11;
#[doc = "PMUGRF_PMUPVTM_CON0 (rw) register accessor: pmu pvtm configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_pmupvtm_con0`]
module"]
#[doc(alias = "PMUGRF_PMUPVTM_CON0")]
pub type PmugrfPmupvtmCon0 = crate::Reg<pmugrf_pmupvtm_con0::PmugrfPmupvtmCon0Spec>;
#[doc = "pmu pvtm configuration register0"]
pub mod pmugrf_pmupvtm_con0;
#[doc = "PMUGRF_PMUPVTM_CON1 (rw) register accessor: pmu pvtm configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_pmupvtm_con1`]
module"]
#[doc(alias = "PMUGRF_PMUPVTM_CON1")]
pub type PmugrfPmupvtmCon1 = crate::Reg<pmugrf_pmupvtm_con1::PmugrfPmupvtmCon1Spec>;
#[doc = "pmu pvtm configuration register1"]
pub mod pmugrf_pmupvtm_con1;
#[doc = "PMUGRF_PMUPVTM_STATUS0 (rw) register accessor: pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_pmupvtm_status0`]
module"]
#[doc(alias = "PMUGRF_PMUPVTM_STATUS0")]
pub type PmugrfPmupvtmStatus0 = crate::Reg<pmugrf_pmupvtm_status0::PmugrfPmupvtmStatus0Spec>;
#[doc = "pmu pvtm status register"]
pub mod pmugrf_pmupvtm_status0;
#[doc = "PMUGRF_PMUPVTM_STATUS1 (rw) register accessor: pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_pmupvtm_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_pmupvtm_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_pmupvtm_status1`]
module"]
#[doc(alias = "PMUGRF_PMUPVTM_STATUS1")]
pub type PmugrfPmupvtmStatus1 = crate::Reg<pmugrf_pmupvtm_status1::PmugrfPmupvtmStatus1Spec>;
#[doc = "pmu pvtm status register"]
pub mod pmugrf_pmupvtm_status1;
#[doc = "PMUGRF_OSC_E (rw) register accessor: OSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_osc_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_osc_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_osc_e`]
module"]
#[doc(alias = "PMUGRF_OSC_E")]
pub type PmugrfOscE = crate::Reg<pmugrf_osc_e::PmugrfOscESpec>;
#[doc = "OSC control register"]
pub mod pmugrf_osc_e;
#[doc = "PMUGRF_OS_REG0 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_os_reg0`]
module"]
#[doc(alias = "PMUGRF_OS_REG0")]
pub type PmugrfOsReg0 = crate::Reg<pmugrf_os_reg0::PmugrfOsReg0Spec>;
#[doc = "os register"]
pub mod pmugrf_os_reg0;
#[doc = "PMUGRF_OS_REG1 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_os_reg1`]
module"]
#[doc(alias = "PMUGRF_OS_REG1")]
pub type PmugrfOsReg1 = crate::Reg<pmugrf_os_reg1::PmugrfOsReg1Spec>;
#[doc = "os register"]
pub mod pmugrf_os_reg1;
#[doc = "PMUGRF_OS_REG2 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_os_reg2`]
module"]
#[doc(alias = "PMUGRF_OS_REG2")]
pub type PmugrfOsReg2 = crate::Reg<pmugrf_os_reg2::PmugrfOsReg2Spec>;
#[doc = "os register"]
pub mod pmugrf_os_reg2;
#[doc = "PMUGRF_OS_REG3 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_os_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_os_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmugrf_os_reg3`]
module"]
#[doc(alias = "PMUGRF_OS_REG3")]
pub type PmugrfOsReg3 = crate::Reg<pmugrf_os_reg3::PmugrfOsReg3Spec>;
#[doc = "os register"]
pub mod pmugrf_os_reg3;
