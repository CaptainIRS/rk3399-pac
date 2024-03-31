#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio0a_iomux: Gpio0aIomux,
    gpio0b_iomux: Gpio0bIomux,
    _reserved2: [u8; 0x08],
    gpio1a_iomux: Gpio1aIomux,
    gpio1b_iomux: Gpio1bIomux,
    gpio1c_iomux: Gpio1cIomux,
    gpio1d_iomux: Gpio1dIomux,
    _reserved6: [u8; 0x20],
    gpio0a_p: Gpio0aP,
    gpio0b_p: Gpio0bP,
    _reserved8: [u8; 0x08],
    gpio1a_p: Gpio1aP,
    gpio1b_p: Gpio1bP,
    gpio1c_p: Gpio1cP,
    gpio1d_p: Gpio1dP,
    _reserved12: [u8; 0x20],
    gpio0a_e: Gpio0aE,
    _reserved13: [u8; 0x04],
    gpio0b_e: Gpio0bE,
    _reserved14: [u8; 0x14],
    gpio1a_e: Gpio1aE,
    _reserved15: [u8; 0x04],
    gpio1b_e: Gpio1bE,
    _reserved16: [u8; 0x04],
    gpio1c_e: Gpio1cE,
    _reserved17: [u8; 0x04],
    gpio1d_e: Gpio1dE,
    _reserved18: [u8; 0x44],
    gpio0l_sr: Gpio0lSr,
    _reserved19: [u8; 0x04],
    gpio1l_sr: Gpio1lSr,
    gpio1h_sr: Gpio1hSr,
    _reserved21: [u8; 0x10],
    gpio0a_smt: Gpio0aSmt,
    gpio0b_smt: Gpio0bSmt,
    _reserved23: [u8; 0x08],
    gpio1a_smt: Gpio1aSmt,
    gpio1b_smt: Gpio1bSmt,
    gpio1c_smt: Gpio1cSmt,
    gpio1d_smt: Gpio1dSmt,
    _reserved27: [u8; 0x20],
    gpio0l_he: Gpio0lHe,
    _reserved28: [u8; 0x04],
    gpio1l_he: Gpio1lHe,
    gpio1h_he: Gpio1hHe,
    _reserved30: [u8; 0x10],
    soc_con0: SocCon0,
    _reserved31: [u8; 0x24],
    soc_con10: SocCon10,
    soc_con11: SocCon11,
    _reserved33: [u8; 0x90],
    pmupvtm_con0: PmupvtmCon0,
    pmupvtm_con1: PmupvtmCon1,
    pmupvtm_status0: PmupvtmStatus0,
    pmupvtm_status1: PmupvtmStatus1,
    osc_e: OscE,
    _reserved38: [u8; 0xac],
    os_reg0: OsReg0,
    os_reg1: OsReg1,
    os_reg2: OsReg2,
    os_reg3: OsReg3,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO0A iomux control"]
    #[inline(always)]
    pub const fn gpio0a_iomux(&self) -> &Gpio0aIomux {
        &self.gpio0a_iomux
    }
    #[doc = "0x04 - GPIO0B iomux control"]
    #[inline(always)]
    pub const fn gpio0b_iomux(&self) -> &Gpio0bIomux {
        &self.gpio0b_iomux
    }
    #[doc = "0x10 - GPIO1A iomux control"]
    #[inline(always)]
    pub const fn gpio1a_iomux(&self) -> &Gpio1aIomux {
        &self.gpio1a_iomux
    }
    #[doc = "0x14 - GPIO1B iomux control"]
    #[inline(always)]
    pub const fn gpio1b_iomux(&self) -> &Gpio1bIomux {
        &self.gpio1b_iomux
    }
    #[doc = "0x18 - GPIO1C iomux control"]
    #[inline(always)]
    pub const fn gpio1c_iomux(&self) -> &Gpio1cIomux {
        &self.gpio1c_iomux
    }
    #[doc = "0x1c - GPIO1D iomux control"]
    #[inline(always)]
    pub const fn gpio1d_iomux(&self) -> &Gpio1dIomux {
        &self.gpio1d_iomux
    }
    #[doc = "0x40 - GPIO0A PU/PD control"]
    #[inline(always)]
    pub const fn gpio0a_p(&self) -> &Gpio0aP {
        &self.gpio0a_p
    }
    #[doc = "0x44 - GPIO0B PU/PD control"]
    #[inline(always)]
    pub const fn gpio0b_p(&self) -> &Gpio0bP {
        &self.gpio0b_p
    }
    #[doc = "0x50 - GPIO1A PU/PD control"]
    #[inline(always)]
    pub const fn gpio1a_p(&self) -> &Gpio1aP {
        &self.gpio1a_p
    }
    #[doc = "0x54 - GPIO1B PU/PD control"]
    #[inline(always)]
    pub const fn gpio1b_p(&self) -> &Gpio1bP {
        &self.gpio1b_p
    }
    #[doc = "0x58 - GPIO1C PU/PD control"]
    #[inline(always)]
    pub const fn gpio1c_p(&self) -> &Gpio1cP {
        &self.gpio1c_p
    }
    #[doc = "0x5c - GPIO0D PU/PD control"]
    #[inline(always)]
    pub const fn gpio1d_p(&self) -> &Gpio1dP {
        &self.gpio1d_p
    }
    #[doc = "0x80 - GPIO0A drive strength control"]
    #[inline(always)]
    pub const fn gpio0a_e(&self) -> &Gpio0aE {
        &self.gpio0a_e
    }
    #[doc = "0x88 - GPIO0D drive strength control"]
    #[inline(always)]
    pub const fn gpio0b_e(&self) -> &Gpio0bE {
        &self.gpio0b_e
    }
    #[doc = "0xa0 - GPIO1A drive strength control"]
    #[inline(always)]
    pub const fn gpio1a_e(&self) -> &Gpio1aE {
        &self.gpio1a_e
    }
    #[doc = "0xa8 - GPIO1D drive strength control"]
    #[inline(always)]
    pub const fn gpio1b_e(&self) -> &Gpio1bE {
        &self.gpio1b_e
    }
    #[doc = "0xb0 - GPIO1C drive strength control"]
    #[inline(always)]
    pub const fn gpio1c_e(&self) -> &Gpio1cE {
        &self.gpio1c_e
    }
    #[doc = "0xb8 - GPIO1D drive strength control"]
    #[inline(always)]
    pub const fn gpio1d_e(&self) -> &Gpio1dE {
        &self.gpio1d_e
    }
    #[doc = "0x100 - GPIO0 A/B SR control"]
    #[inline(always)]
    pub const fn gpio0l_sr(&self) -> &Gpio0lSr {
        &self.gpio0l_sr
    }
    #[doc = "0x108 - GPIO1 A/B SR control"]
    #[inline(always)]
    pub const fn gpio1l_sr(&self) -> &Gpio1lSr {
        &self.gpio1l_sr
    }
    #[doc = "0x10c - GPIO1C/D SR control"]
    #[inline(always)]
    pub const fn gpio1h_sr(&self) -> &Gpio1hSr {
        &self.gpio1h_sr
    }
    #[doc = "0x120 - GPIO0A smit control"]
    #[inline(always)]
    pub const fn gpio0a_smt(&self) -> &Gpio0aSmt {
        &self.gpio0a_smt
    }
    #[doc = "0x124 - GPIO0B smit control"]
    #[inline(always)]
    pub const fn gpio0b_smt(&self) -> &Gpio0bSmt {
        &self.gpio0b_smt
    }
    #[doc = "0x130 - GPIO1A smit control"]
    #[inline(always)]
    pub const fn gpio1a_smt(&self) -> &Gpio1aSmt {
        &self.gpio1a_smt
    }
    #[doc = "0x134 - GPIO1B smit control"]
    #[inline(always)]
    pub const fn gpio1b_smt(&self) -> &Gpio1bSmt {
        &self.gpio1b_smt
    }
    #[doc = "0x138 - GPIO1C smit control"]
    #[inline(always)]
    pub const fn gpio1c_smt(&self) -> &Gpio1cSmt {
        &self.gpio1c_smt
    }
    #[doc = "0x13c - GPIO1D smit control"]
    #[inline(always)]
    pub const fn gpio1d_smt(&self) -> &Gpio1dSmt {
        &self.gpio1d_smt
    }
    #[doc = "0x160 - GPIO0 A/B HE control"]
    #[inline(always)]
    pub const fn gpio0l_he(&self) -> &Gpio0lHe {
        &self.gpio0l_he
    }
    #[doc = "0x168 - GPIO1 A/B HE control"]
    #[inline(always)]
    pub const fn gpio1l_he(&self) -> &Gpio1lHe {
        &self.gpio1l_he
    }
    #[doc = "0x16c - GPIO1C/D HE control"]
    #[inline(always)]
    pub const fn gpio1h_he(&self) -> &Gpio1hHe {
        &self.gpio1h_he
    }
    #[doc = "0x180 - SoC control register 0"]
    #[inline(always)]
    pub const fn soc_con0(&self) -> &SocCon0 {
        &self.soc_con0
    }
    #[doc = "0x1a8 - SoC control register 10"]
    #[inline(always)]
    pub const fn soc_con10(&self) -> &SocCon10 {
        &self.soc_con10
    }
    #[doc = "0x1ac - SoC control register 11"]
    #[inline(always)]
    pub const fn soc_con11(&self) -> &SocCon11 {
        &self.soc_con11
    }
    #[doc = "0x240 - pmu pvtm configuration register0"]
    #[inline(always)]
    pub const fn pmupvtm_con0(&self) -> &PmupvtmCon0 {
        &self.pmupvtm_con0
    }
    #[doc = "0x244 - pmu pvtm configuration register1"]
    #[inline(always)]
    pub const fn pmupvtm_con1(&self) -> &PmupvtmCon1 {
        &self.pmupvtm_con1
    }
    #[doc = "0x248 - pmu pvtm status register"]
    #[inline(always)]
    pub const fn pmupvtm_status0(&self) -> &PmupvtmStatus0 {
        &self.pmupvtm_status0
    }
    #[doc = "0x24c - pmu pvtm status register"]
    #[inline(always)]
    pub const fn pmupvtm_status1(&self) -> &PmupvtmStatus1 {
        &self.pmupvtm_status1
    }
    #[doc = "0x250 - OSC control register"]
    #[inline(always)]
    pub const fn osc_e(&self) -> &OscE {
        &self.osc_e
    }
    #[doc = "0x300 - os register"]
    #[inline(always)]
    pub const fn os_reg0(&self) -> &OsReg0 {
        &self.os_reg0
    }
    #[doc = "0x304 - os register"]
    #[inline(always)]
    pub const fn os_reg1(&self) -> &OsReg1 {
        &self.os_reg1
    }
    #[doc = "0x308 - os register"]
    #[inline(always)]
    pub const fn os_reg2(&self) -> &OsReg2 {
        &self.os_reg2
    }
    #[doc = "0x30c - os register"]
    #[inline(always)]
    pub const fn os_reg3(&self) -> &OsReg3 {
        &self.os_reg3
    }
}
#[doc = "GPIO0A_IOMUX (rw) register accessor: GPIO0A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a_iomux`]
module"]
#[doc(alias = "GPIO0A_IOMUX")]
pub type Gpio0aIomux = crate::Reg<gpio0a_iomux::Gpio0aIomuxSpec>;
#[doc = "GPIO0A iomux control"]
pub mod gpio0a_iomux;
#[doc = "GPIO0B_IOMUX (rw) register accessor: GPIO0B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b_iomux`]
module"]
#[doc(alias = "GPIO0B_IOMUX")]
pub type Gpio0bIomux = crate::Reg<gpio0b_iomux::Gpio0bIomuxSpec>;
#[doc = "GPIO0B iomux control"]
pub mod gpio0b_iomux;
#[doc = "GPIO1A_IOMUX (rw) register accessor: GPIO1A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a_iomux`]
module"]
#[doc(alias = "GPIO1A_IOMUX")]
pub type Gpio1aIomux = crate::Reg<gpio1a_iomux::Gpio1aIomuxSpec>;
#[doc = "GPIO1A iomux control"]
pub mod gpio1a_iomux;
#[doc = "GPIO1B_IOMUX (rw) register accessor: GPIO1B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b_iomux`]
module"]
#[doc(alias = "GPIO1B_IOMUX")]
pub type Gpio1bIomux = crate::Reg<gpio1b_iomux::Gpio1bIomuxSpec>;
#[doc = "GPIO1B iomux control"]
pub mod gpio1b_iomux;
#[doc = "GPIO1C_IOMUX (rw) register accessor: GPIO1C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c_iomux`]
module"]
#[doc(alias = "GPIO1C_IOMUX")]
pub type Gpio1cIomux = crate::Reg<gpio1c_iomux::Gpio1cIomuxSpec>;
#[doc = "GPIO1C iomux control"]
pub mod gpio1c_iomux;
#[doc = "GPIO1D_IOMUX (rw) register accessor: GPIO1D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d_iomux`]
module"]
#[doc(alias = "GPIO1D_IOMUX")]
pub type Gpio1dIomux = crate::Reg<gpio1d_iomux::Gpio1dIomuxSpec>;
#[doc = "GPIO1D iomux control"]
pub mod gpio1d_iomux;
#[doc = "GPIO0A_P (rw) register accessor: GPIO0A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a_p`]
module"]
#[doc(alias = "GPIO0A_P")]
pub type Gpio0aP = crate::Reg<gpio0a_p::Gpio0aPSpec>;
#[doc = "GPIO0A PU/PD control"]
pub mod gpio0a_p;
#[doc = "GPIO0B_P (rw) register accessor: GPIO0B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b_p`]
module"]
#[doc(alias = "GPIO0B_P")]
pub type Gpio0bP = crate::Reg<gpio0b_p::Gpio0bPSpec>;
#[doc = "GPIO0B PU/PD control"]
pub mod gpio0b_p;
#[doc = "GPIO1A_P (rw) register accessor: GPIO1A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a_p`]
module"]
#[doc(alias = "GPIO1A_P")]
pub type Gpio1aP = crate::Reg<gpio1a_p::Gpio1aPSpec>;
#[doc = "GPIO1A PU/PD control"]
pub mod gpio1a_p;
#[doc = "GPIO1B_P (rw) register accessor: GPIO1B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b_p`]
module"]
#[doc(alias = "GPIO1B_P")]
pub type Gpio1bP = crate::Reg<gpio1b_p::Gpio1bPSpec>;
#[doc = "GPIO1B PU/PD control"]
pub mod gpio1b_p;
#[doc = "GPIO1C_P (rw) register accessor: GPIO1C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c_p`]
module"]
#[doc(alias = "GPIO1C_P")]
pub type Gpio1cP = crate::Reg<gpio1c_p::Gpio1cPSpec>;
#[doc = "GPIO1C PU/PD control"]
pub mod gpio1c_p;
#[doc = "GPIO1D_P (rw) register accessor: GPIO0D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d_p`]
module"]
#[doc(alias = "GPIO1D_P")]
pub type Gpio1dP = crate::Reg<gpio1d_p::Gpio1dPSpec>;
#[doc = "GPIO0D PU/PD control"]
pub mod gpio1d_p;
#[doc = "GPIO0A_E (rw) register accessor: GPIO0A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a_e`]
module"]
#[doc(alias = "GPIO0A_E")]
pub type Gpio0aE = crate::Reg<gpio0a_e::Gpio0aESpec>;
#[doc = "GPIO0A drive strength control"]
pub mod gpio0a_e;
#[doc = "GPIO0B_E (rw) register accessor: GPIO0D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b_e`]
module"]
#[doc(alias = "GPIO0B_E")]
pub type Gpio0bE = crate::Reg<gpio0b_e::Gpio0bESpec>;
#[doc = "GPIO0D drive strength control"]
pub mod gpio0b_e;
#[doc = "GPIO1A_E (rw) register accessor: GPIO1A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a_e`]
module"]
#[doc(alias = "GPIO1A_E")]
pub type Gpio1aE = crate::Reg<gpio1a_e::Gpio1aESpec>;
#[doc = "GPIO1A drive strength control"]
pub mod gpio1a_e;
#[doc = "GPIO1B_E (rw) register accessor: GPIO1D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b_e`]
module"]
#[doc(alias = "GPIO1B_E")]
pub type Gpio1bE = crate::Reg<gpio1b_e::Gpio1bESpec>;
#[doc = "GPIO1D drive strength control"]
pub mod gpio1b_e;
#[doc = "GPIO1C_E (rw) register accessor: GPIO1C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c_e`]
module"]
#[doc(alias = "GPIO1C_E")]
pub type Gpio1cE = crate::Reg<gpio1c_e::Gpio1cESpec>;
#[doc = "GPIO1C drive strength control"]
pub mod gpio1c_e;
#[doc = "GPIO1D_E (rw) register accessor: GPIO1D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d_e`]
module"]
#[doc(alias = "GPIO1D_E")]
pub type Gpio1dE = crate::Reg<gpio1d_e::Gpio1dESpec>;
#[doc = "GPIO1D drive strength control"]
pub mod gpio1d_e;
#[doc = "GPIO0L_SR (rw) register accessor: GPIO0 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0l_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0l_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0l_sr`]
module"]
#[doc(alias = "GPIO0L_SR")]
pub type Gpio0lSr = crate::Reg<gpio0l_sr::Gpio0lSrSpec>;
#[doc = "GPIO0 A/B SR control"]
pub mod gpio0l_sr;
#[doc = "GPIO1L_SR (rw) register accessor: GPIO1 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1l_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1l_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1l_sr`]
module"]
#[doc(alias = "GPIO1L_SR")]
pub type Gpio1lSr = crate::Reg<gpio1l_sr::Gpio1lSrSpec>;
#[doc = "GPIO1 A/B SR control"]
pub mod gpio1l_sr;
#[doc = "GPIO1H_SR (rw) register accessor: GPIO1C/D SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1h_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1h_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1h_sr`]
module"]
#[doc(alias = "GPIO1H_SR")]
pub type Gpio1hSr = crate::Reg<gpio1h_sr::Gpio1hSrSpec>;
#[doc = "GPIO1C/D SR control"]
pub mod gpio1h_sr;
#[doc = "GPIO0A_SMT (rw) register accessor: GPIO0A smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a_smt`]
module"]
#[doc(alias = "GPIO0A_SMT")]
pub type Gpio0aSmt = crate::Reg<gpio0a_smt::Gpio0aSmtSpec>;
#[doc = "GPIO0A smit control"]
pub mod gpio0a_smt;
#[doc = "GPIO0B_SMT (rw) register accessor: GPIO0B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b_smt`]
module"]
#[doc(alias = "GPIO0B_SMT")]
pub type Gpio0bSmt = crate::Reg<gpio0b_smt::Gpio0bSmtSpec>;
#[doc = "GPIO0B smit control"]
pub mod gpio0b_smt;
#[doc = "GPIO1A_SMT (rw) register accessor: GPIO1A smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a_smt`]
module"]
#[doc(alias = "GPIO1A_SMT")]
pub type Gpio1aSmt = crate::Reg<gpio1a_smt::Gpio1aSmtSpec>;
#[doc = "GPIO1A smit control"]
pub mod gpio1a_smt;
#[doc = "GPIO1B_SMT (rw) register accessor: GPIO1B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b_smt`]
module"]
#[doc(alias = "GPIO1B_SMT")]
pub type Gpio1bSmt = crate::Reg<gpio1b_smt::Gpio1bSmtSpec>;
#[doc = "GPIO1B smit control"]
pub mod gpio1b_smt;
#[doc = "GPIO1C_SMT (rw) register accessor: GPIO1C smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c_smt`]
module"]
#[doc(alias = "GPIO1C_SMT")]
pub type Gpio1cSmt = crate::Reg<gpio1c_smt::Gpio1cSmtSpec>;
#[doc = "GPIO1C smit control"]
pub mod gpio1c_smt;
#[doc = "GPIO1D_SMT (rw) register accessor: GPIO1D smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d_smt`]
module"]
#[doc(alias = "GPIO1D_SMT")]
pub type Gpio1dSmt = crate::Reg<gpio1d_smt::Gpio1dSmtSpec>;
#[doc = "GPIO1D smit control"]
pub mod gpio1d_smt;
#[doc = "GPIO0L_HE (rw) register accessor: GPIO0 A/B HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0l_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0l_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0l_he`]
module"]
#[doc(alias = "GPIO0L_HE")]
pub type Gpio0lHe = crate::Reg<gpio0l_he::Gpio0lHeSpec>;
#[doc = "GPIO0 A/B HE control"]
pub mod gpio0l_he;
#[doc = "GPIO1L_HE (rw) register accessor: GPIO1 A/B HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1l_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1l_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1l_he`]
module"]
#[doc(alias = "GPIO1L_HE")]
pub type Gpio1lHe = crate::Reg<gpio1l_he::Gpio1lHeSpec>;
#[doc = "GPIO1 A/B HE control"]
pub mod gpio1l_he;
#[doc = "GPIO1H_HE (rw) register accessor: GPIO1C/D HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1h_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1h_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1h_he`]
module"]
#[doc(alias = "GPIO1H_HE")]
pub type Gpio1hHe = crate::Reg<gpio1h_he::Gpio1hHeSpec>;
#[doc = "GPIO1C/D HE control"]
pub mod gpio1h_he;
#[doc = "SOC_CON0 (rw) register accessor: SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con0`]
module"]
#[doc(alias = "SOC_CON0")]
pub type SocCon0 = crate::Reg<soc_con0::SocCon0Spec>;
#[doc = "SoC control register 0"]
pub mod soc_con0;
#[doc = "SOC_CON10 (rw) register accessor: SoC control register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con10`]
module"]
#[doc(alias = "SOC_CON10")]
pub type SocCon10 = crate::Reg<soc_con10::SocCon10Spec>;
#[doc = "SoC control register 10"]
pub mod soc_con10;
#[doc = "SOC_CON11 (rw) register accessor: SoC control register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con11`]
module"]
#[doc(alias = "SOC_CON11")]
pub type SocCon11 = crate::Reg<soc_con11::SocCon11Spec>;
#[doc = "SoC control register 11"]
pub mod soc_con11;
#[doc = "PMUPVTM_CON0 (rw) register accessor: pmu pvtm configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmupvtm_con0`]
module"]
#[doc(alias = "PMUPVTM_CON0")]
pub type PmupvtmCon0 = crate::Reg<pmupvtm_con0::PmupvtmCon0Spec>;
#[doc = "pmu pvtm configuration register0"]
pub mod pmupvtm_con0;
#[doc = "PMUPVTM_CON1 (rw) register accessor: pmu pvtm configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmupvtm_con1`]
module"]
#[doc(alias = "PMUPVTM_CON1")]
pub type PmupvtmCon1 = crate::Reg<pmupvtm_con1::PmupvtmCon1Spec>;
#[doc = "pmu pvtm configuration register1"]
pub mod pmupvtm_con1;
#[doc = "PMUPVTM_STATUS0 (rw) register accessor: pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmupvtm_status0`]
module"]
#[doc(alias = "PMUPVTM_STATUS0")]
pub type PmupvtmStatus0 = crate::Reg<pmupvtm_status0::PmupvtmStatus0Spec>;
#[doc = "pmu pvtm status register"]
pub mod pmupvtm_status0;
#[doc = "PMUPVTM_STATUS1 (rw) register accessor: pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmupvtm_status1`]
module"]
#[doc(alias = "PMUPVTM_STATUS1")]
pub type PmupvtmStatus1 = crate::Reg<pmupvtm_status1::PmupvtmStatus1Spec>;
#[doc = "pmu pvtm status register"]
pub mod pmupvtm_status1;
#[doc = "OSC_E (rw) register accessor: OSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc_e`]
module"]
#[doc(alias = "OSC_E")]
pub type OscE = crate::Reg<osc_e::OscESpec>;
#[doc = "OSC control register"]
pub mod osc_e;
#[doc = "OS_REG0 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os_reg0`]
module"]
#[doc(alias = "OS_REG0")]
pub type OsReg0 = crate::Reg<os_reg0::OsReg0Spec>;
#[doc = "os register"]
pub mod os_reg0;
#[doc = "OS_REG1 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os_reg1`]
module"]
#[doc(alias = "OS_REG1")]
pub type OsReg1 = crate::Reg<os_reg1::OsReg1Spec>;
#[doc = "os register"]
pub mod os_reg1;
#[doc = "OS_REG2 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os_reg2`]
module"]
#[doc(alias = "OS_REG2")]
pub type OsReg2 = crate::Reg<os_reg2::OsReg2Spec>;
#[doc = "os register"]
pub mod os_reg2;
#[doc = "OS_REG3 (rw) register accessor: os register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os_reg3`]
module"]
#[doc(alias = "OS_REG3")]
pub type OsReg3 = crate::Reg<os_reg3::OsReg3Spec>;
#[doc = "os register"]
pub mod os_reg3;
