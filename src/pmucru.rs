#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmucru_ppll_con0: PmucruPpllCon0,
    pmucru_ppll_con1: PmucruPpllCon1,
    pmucru_ppll_con2: PmucruPpllCon2,
    pmucru_ppll_con3: PmucruPpllCon3,
    pmucru_ppll_con4: PmucruPpllCon4,
    pmucru_ppll_con5: PmucruPpllCon5,
    _reserved6: [u8; 0x68],
    pmucru_clksel_con0: PmucruClkselCon0,
    pmucru_clksel_con1: PmucruClkselCon1,
    pmucru_clksel_con2: PmucruClkselCon2,
    pmucru_clksel_con3: PmucruClkselCon3,
    pmucru_clksel_con4: PmucruClkselCon4,
    pmucru_clksel_con5: PmucruClkselCon5,
    pmucru_clkfrac_con0: PmucruClkfracCon0,
    pmucru_clkfrac_con1: PmucruClkfracCon1,
    _reserved14: [u8; 0x60],
    pmucru_clkgate_con0: PmucruClkgateCon0,
    pmucru_clkgate_con1: PmucruClkgateCon1,
    pmucru_clkgate_con2: PmucruClkgateCon2,
    _reserved17: [u8; 0x04],
    pmucru_softrst_con0: PmucruSoftrstCon0,
    pmucru_softrst_con1: PmucruSoftrstCon1,
    _reserved19: [u8; 0x08],
    pmucru_rstnhold_con0: PmucruRstnholdCon0,
    pmucru_rstnhold_con1: PmucruRstnholdCon1,
    _reserved21: [u8; 0x08],
    pmucru_gatedis_con0: PmucruGatedisCon0,
}
impl RegisterBlock {
    #[doc = "0x00 - PPLL configuration register0"]
    #[inline(always)]
    pub const fn pmucru_ppll_con0(&self) -> &PmucruPpllCon0 {
        &self.pmucru_ppll_con0
    }
    #[doc = "0x04 - PPLL configuration register1"]
    #[inline(always)]
    pub const fn pmucru_ppll_con1(&self) -> &PmucruPpllCon1 {
        &self.pmucru_ppll_con1
    }
    #[doc = "0x08 - PPLL configuration register2"]
    #[inline(always)]
    pub const fn pmucru_ppll_con2(&self) -> &PmucruPpllCon2 {
        &self.pmucru_ppll_con2
    }
    #[doc = "0x0c - PPLL configuration register3"]
    #[inline(always)]
    pub const fn pmucru_ppll_con3(&self) -> &PmucruPpllCon3 {
        &self.pmucru_ppll_con3
    }
    #[doc = "0x10 - PPLL configuration register4"]
    #[inline(always)]
    pub const fn pmucru_ppll_con4(&self) -> &PmucruPpllCon4 {
        &self.pmucru_ppll_con4
    }
    #[doc = "0x14 - PPLL configuration register5"]
    #[inline(always)]
    pub const fn pmucru_ppll_con5(&self) -> &PmucruPpllCon5 {
        &self.pmucru_ppll_con5
    }
    #[doc = "0x80 - Internal clock select and divide register0"]
    #[inline(always)]
    pub const fn pmucru_clksel_con0(&self) -> &PmucruClkselCon0 {
        &self.pmucru_clksel_con0
    }
    #[doc = "0x84 - Internal clock select and divide register1"]
    #[inline(always)]
    pub const fn pmucru_clksel_con1(&self) -> &PmucruClkselCon1 {
        &self.pmucru_clksel_con1
    }
    #[doc = "0x88 - Internal clock select and divide register2"]
    #[inline(always)]
    pub const fn pmucru_clksel_con2(&self) -> &PmucruClkselCon2 {
        &self.pmucru_clksel_con2
    }
    #[doc = "0x8c - Internal clock select and divide register3"]
    #[inline(always)]
    pub const fn pmucru_clksel_con3(&self) -> &PmucruClkselCon3 {
        &self.pmucru_clksel_con3
    }
    #[doc = "0x90 - Internal clock select and divide register4"]
    #[inline(always)]
    pub const fn pmucru_clksel_con4(&self) -> &PmucruClkselCon4 {
        &self.pmucru_clksel_con4
    }
    #[doc = "0x94 - Internal clock select and divide register5"]
    #[inline(always)]
    pub const fn pmucru_clksel_con5(&self) -> &PmucruClkselCon5 {
        &self.pmucru_clksel_con5
    }
    #[doc = "0x98 - Internal clock select and divide register6"]
    #[inline(always)]
    pub const fn pmucru_clkfrac_con0(&self) -> &PmucruClkfracCon0 {
        &self.pmucru_clkfrac_con0
    }
    #[doc = "0x9c - Internal clock select and divide register7"]
    #[inline(always)]
    pub const fn pmucru_clkfrac_con1(&self) -> &PmucruClkfracCon1 {
        &self.pmucru_clkfrac_con1
    }
    #[doc = "0x100 - Internal clock gating register0"]
    #[inline(always)]
    pub const fn pmucru_clkgate_con0(&self) -> &PmucruClkgateCon0 {
        &self.pmucru_clkgate_con0
    }
    #[doc = "0x104 - Internal clock gating register1"]
    #[inline(always)]
    pub const fn pmucru_clkgate_con1(&self) -> &PmucruClkgateCon1 {
        &self.pmucru_clkgate_con1
    }
    #[doc = "0x108 - Internal clock gating register2"]
    #[inline(always)]
    pub const fn pmucru_clkgate_con2(&self) -> &PmucruClkgateCon2 {
        &self.pmucru_clkgate_con2
    }
    #[doc = "0x110 - Internal software reset control register0"]
    #[inline(always)]
    pub const fn pmucru_softrst_con0(&self) -> &PmucruSoftrstCon0 {
        &self.pmucru_softrst_con0
    }
    #[doc = "0x114 - Internal software reset control register1"]
    #[inline(always)]
    pub const fn pmucru_softrst_con1(&self) -> &PmucruSoftrstCon1 {
        &self.pmucru_softrst_con1
    }
    #[doc = "0x120 - Internal reset hold control register0"]
    #[inline(always)]
    pub const fn pmucru_rstnhold_con0(&self) -> &PmucruRstnholdCon0 {
        &self.pmucru_rstnhold_con0
    }
    #[doc = "0x124 - Internal reset hold control register1"]
    #[inline(always)]
    pub const fn pmucru_rstnhold_con1(&self) -> &PmucruRstnholdCon1 {
        &self.pmucru_rstnhold_con1
    }
    #[doc = "0x130 - Internal gate disable control register0"]
    #[inline(always)]
    pub const fn pmucru_gatedis_con0(&self) -> &PmucruGatedisCon0 {
        &self.pmucru_gatedis_con0
    }
}
#[doc = "PMUCRU_PPLL_CON0 (rw) register accessor: PPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con0`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON0")]
pub type PmucruPpllCon0 = crate::Reg<pmucru_ppll_con0::PmucruPpllCon0Spec>;
#[doc = "PPLL configuration register0"]
pub mod pmucru_ppll_con0;
#[doc = "PMUCRU_PPLL_CON1 (rw) register accessor: PPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con1`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON1")]
pub type PmucruPpllCon1 = crate::Reg<pmucru_ppll_con1::PmucruPpllCon1Spec>;
#[doc = "PPLL configuration register1"]
pub mod pmucru_ppll_con1;
#[doc = "PMUCRU_PPLL_CON2 (rw) register accessor: PPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con2`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON2")]
pub type PmucruPpllCon2 = crate::Reg<pmucru_ppll_con2::PmucruPpllCon2Spec>;
#[doc = "PPLL configuration register2"]
pub mod pmucru_ppll_con2;
#[doc = "PMUCRU_PPLL_CON3 (rw) register accessor: PPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con3`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON3")]
pub type PmucruPpllCon3 = crate::Reg<pmucru_ppll_con3::PmucruPpllCon3Spec>;
#[doc = "PPLL configuration register3"]
pub mod pmucru_ppll_con3;
#[doc = "PMUCRU_PPLL_CON4 (rw) register accessor: PPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con4`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON4")]
pub type PmucruPpllCon4 = crate::Reg<pmucru_ppll_con4::PmucruPpllCon4Spec>;
#[doc = "PPLL configuration register4"]
pub mod pmucru_ppll_con4;
#[doc = "PMUCRU_PPLL_CON5 (rw) register accessor: PPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_ppll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_ppll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_ppll_con5`]
module"]
#[doc(alias = "PMUCRU_PPLL_CON5")]
pub type PmucruPpllCon5 = crate::Reg<pmucru_ppll_con5::PmucruPpllCon5Spec>;
#[doc = "PPLL configuration register5"]
pub mod pmucru_ppll_con5;
#[doc = "PMUCRU_CLKSEL_CON0 (rw) register accessor: Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con0`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON0")]
pub type PmucruClkselCon0 = crate::Reg<pmucru_clksel_con0::PmucruClkselCon0Spec>;
#[doc = "Internal clock select and divide register0"]
pub mod pmucru_clksel_con0;
#[doc = "PMUCRU_CLKSEL_CON1 (rw) register accessor: Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con1`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON1")]
pub type PmucruClkselCon1 = crate::Reg<pmucru_clksel_con1::PmucruClkselCon1Spec>;
#[doc = "Internal clock select and divide register1"]
pub mod pmucru_clksel_con1;
#[doc = "PMUCRU_CLKSEL_CON2 (rw) register accessor: Internal clock select and divide register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con2`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON2")]
pub type PmucruClkselCon2 = crate::Reg<pmucru_clksel_con2::PmucruClkselCon2Spec>;
#[doc = "Internal clock select and divide register2"]
pub mod pmucru_clksel_con2;
#[doc = "PMUCRU_CLKSEL_CON3 (rw) register accessor: Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con3`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON3")]
pub type PmucruClkselCon3 = crate::Reg<pmucru_clksel_con3::PmucruClkselCon3Spec>;
#[doc = "Internal clock select and divide register3"]
pub mod pmucru_clksel_con3;
#[doc = "PMUCRU_CLKSEL_CON4 (rw) register accessor: Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con4`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON4")]
pub type PmucruClkselCon4 = crate::Reg<pmucru_clksel_con4::PmucruClkselCon4Spec>;
#[doc = "Internal clock select and divide register4"]
pub mod pmucru_clksel_con4;
#[doc = "PMUCRU_CLKSEL_CON5 (rw) register accessor: Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clksel_con5`]
module"]
#[doc(alias = "PMUCRU_CLKSEL_CON5")]
pub type PmucruClkselCon5 = crate::Reg<pmucru_clksel_con5::PmucruClkselCon5Spec>;
#[doc = "Internal clock select and divide register5"]
pub mod pmucru_clksel_con5;
#[doc = "PMUCRU_CLKFRAC_CON0 (rw) register accessor: Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkfrac_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkfrac_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clkfrac_con0`]
module"]
#[doc(alias = "PMUCRU_CLKFRAC_CON0")]
pub type PmucruClkfracCon0 = crate::Reg<pmucru_clkfrac_con0::PmucruClkfracCon0Spec>;
#[doc = "Internal clock select and divide register6"]
pub mod pmucru_clkfrac_con0;
#[doc = "PMUCRU_CLKFRAC_CON1 (rw) register accessor: Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkfrac_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkfrac_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clkfrac_con1`]
module"]
#[doc(alias = "PMUCRU_CLKFRAC_CON1")]
pub type PmucruClkfracCon1 = crate::Reg<pmucru_clkfrac_con1::PmucruClkfracCon1Spec>;
#[doc = "Internal clock select and divide register7"]
pub mod pmucru_clkfrac_con1;
#[doc = "PMUCRU_CLKGATE_CON0 (rw) register accessor: Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clkgate_con0`]
module"]
#[doc(alias = "PMUCRU_CLKGATE_CON0")]
pub type PmucruClkgateCon0 = crate::Reg<pmucru_clkgate_con0::PmucruClkgateCon0Spec>;
#[doc = "Internal clock gating register0"]
pub mod pmucru_clkgate_con0;
#[doc = "PMUCRU_CLKGATE_CON1 (rw) register accessor: Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clkgate_con1`]
module"]
#[doc(alias = "PMUCRU_CLKGATE_CON1")]
pub type PmucruClkgateCon1 = crate::Reg<pmucru_clkgate_con1::PmucruClkgateCon1Spec>;
#[doc = "Internal clock gating register1"]
pub mod pmucru_clkgate_con1;
#[doc = "PMUCRU_CLKGATE_CON2 (rw) register accessor: Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_clkgate_con2`]
module"]
#[doc(alias = "PMUCRU_CLKGATE_CON2")]
pub type PmucruClkgateCon2 = crate::Reg<pmucru_clkgate_con2::PmucruClkgateCon2Spec>;
#[doc = "Internal clock gating register2"]
pub mod pmucru_clkgate_con2;
#[doc = "PMUCRU_SOFTRST_CON0 (rw) register accessor: Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_softrst_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_softrst_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_softrst_con0`]
module"]
#[doc(alias = "PMUCRU_SOFTRST_CON0")]
pub type PmucruSoftrstCon0 = crate::Reg<pmucru_softrst_con0::PmucruSoftrstCon0Spec>;
#[doc = "Internal software reset control register0"]
pub mod pmucru_softrst_con0;
#[doc = "PMUCRU_SOFTRST_CON1 (rw) register accessor: Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_softrst_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_softrst_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_softrst_con1`]
module"]
#[doc(alias = "PMUCRU_SOFTRST_CON1")]
pub type PmucruSoftrstCon1 = crate::Reg<pmucru_softrst_con1::PmucruSoftrstCon1Spec>;
#[doc = "Internal software reset control register1"]
pub mod pmucru_softrst_con1;
#[doc = "PMUCRU_RSTNHOLD_CON0 (rw) register accessor: Internal reset hold control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_rstnhold_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_rstnhold_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_rstnhold_con0`]
module"]
#[doc(alias = "PMUCRU_RSTNHOLD_CON0")]
pub type PmucruRstnholdCon0 = crate::Reg<pmucru_rstnhold_con0::PmucruRstnholdCon0Spec>;
#[doc = "Internal reset hold control register0"]
pub mod pmucru_rstnhold_con0;
#[doc = "PMUCRU_RSTNHOLD_CON1 (rw) register accessor: Internal reset hold control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_rstnhold_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_rstnhold_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_rstnhold_con1`]
module"]
#[doc(alias = "PMUCRU_RSTNHOLD_CON1")]
pub type PmucruRstnholdCon1 = crate::Reg<pmucru_rstnhold_con1::PmucruRstnholdCon1Spec>;
#[doc = "Internal reset hold control register1"]
pub mod pmucru_rstnhold_con1;
#[doc = "PMUCRU_GATEDIS_CON0 (rw) register accessor: Internal gate disable control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_gatedis_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_gatedis_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucru_gatedis_con0`]
module"]
#[doc(alias = "PMUCRU_GATEDIS_CON0")]
pub type PmucruGatedisCon0 = crate::Reg<pmucru_gatedis_con0::PmucruGatedisCon0Spec>;
#[doc = "Internal gate disable control register0"]
pub mod pmucru_gatedis_con0;
