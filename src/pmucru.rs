#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ppll_con0: PpllCon0,
    ppll_con1: PpllCon1,
    ppll_con2: PpllCon2,
    ppll_con3: PpllCon3,
    ppll_con4: PpllCon4,
    ppll_con5: PpllCon5,
    _reserved6: [u8; 0x68],
    clksel_con0: ClkselCon0,
    clksel_con1: ClkselCon1,
    clksel_con2: ClkselCon2,
    clksel_con3: ClkselCon3,
    clksel_con4: ClkselCon4,
    clksel_con5: ClkselCon5,
    clkfrac_con0: ClkfracCon0,
    clkfrac_con1: ClkfracCon1,
    _reserved14: [u8; 0x60],
    clkgate_con0: ClkgateCon0,
    clkgate_con1: ClkgateCon1,
    clkgate_con2: ClkgateCon2,
    _reserved17: [u8; 0x04],
    softrst_con0: SoftrstCon0,
    softrst_con1: SoftrstCon1,
    _reserved19: [u8; 0x08],
    rstnhold_con0: RstnholdCon0,
    rstnhold_con1: RstnholdCon1,
    _reserved21: [u8; 0x08],
    gatedis_con0: GatedisCon0,
}
impl RegisterBlock {
    #[doc = "0x00 - PPLL configuration register0"]
    #[inline(always)]
    pub const fn ppll_con0(&self) -> &PpllCon0 {
        &self.ppll_con0
    }
    #[doc = "0x04 - PPLL configuration register1"]
    #[inline(always)]
    pub const fn ppll_con1(&self) -> &PpllCon1 {
        &self.ppll_con1
    }
    #[doc = "0x08 - PPLL configuration register2"]
    #[inline(always)]
    pub const fn ppll_con2(&self) -> &PpllCon2 {
        &self.ppll_con2
    }
    #[doc = "0x0c - PPLL configuration register3"]
    #[inline(always)]
    pub const fn ppll_con3(&self) -> &PpllCon3 {
        &self.ppll_con3
    }
    #[doc = "0x10 - PPLL configuration register4"]
    #[inline(always)]
    pub const fn ppll_con4(&self) -> &PpllCon4 {
        &self.ppll_con4
    }
    #[doc = "0x14 - PPLL configuration register5"]
    #[inline(always)]
    pub const fn ppll_con5(&self) -> &PpllCon5 {
        &self.ppll_con5
    }
    #[doc = "0x80 - Internal clock select and divide register0"]
    #[inline(always)]
    pub const fn clksel_con0(&self) -> &ClkselCon0 {
        &self.clksel_con0
    }
    #[doc = "0x84 - Internal clock select and divide register1"]
    #[inline(always)]
    pub const fn clksel_con1(&self) -> &ClkselCon1 {
        &self.clksel_con1
    }
    #[doc = "0x88 - Internal clock select and divide register2"]
    #[inline(always)]
    pub const fn clksel_con2(&self) -> &ClkselCon2 {
        &self.clksel_con2
    }
    #[doc = "0x8c - Internal clock select and divide register3"]
    #[inline(always)]
    pub const fn clksel_con3(&self) -> &ClkselCon3 {
        &self.clksel_con3
    }
    #[doc = "0x90 - Internal clock select and divide register4"]
    #[inline(always)]
    pub const fn clksel_con4(&self) -> &ClkselCon4 {
        &self.clksel_con4
    }
    #[doc = "0x94 - Internal clock select and divide register5"]
    #[inline(always)]
    pub const fn clksel_con5(&self) -> &ClkselCon5 {
        &self.clksel_con5
    }
    #[doc = "0x98 - Internal clock select and divide register6"]
    #[inline(always)]
    pub const fn clkfrac_con0(&self) -> &ClkfracCon0 {
        &self.clkfrac_con0
    }
    #[doc = "0x9c - Internal clock select and divide register7"]
    #[inline(always)]
    pub const fn clkfrac_con1(&self) -> &ClkfracCon1 {
        &self.clkfrac_con1
    }
    #[doc = "0x100 - Internal clock gating register0"]
    #[inline(always)]
    pub const fn clkgate_con0(&self) -> &ClkgateCon0 {
        &self.clkgate_con0
    }
    #[doc = "0x104 - Internal clock gating register1"]
    #[inline(always)]
    pub const fn clkgate_con1(&self) -> &ClkgateCon1 {
        &self.clkgate_con1
    }
    #[doc = "0x108 - Internal clock gating register2"]
    #[inline(always)]
    pub const fn clkgate_con2(&self) -> &ClkgateCon2 {
        &self.clkgate_con2
    }
    #[doc = "0x110 - Internal software reset control register0"]
    #[inline(always)]
    pub const fn softrst_con0(&self) -> &SoftrstCon0 {
        &self.softrst_con0
    }
    #[doc = "0x114 - Internal software reset control register1"]
    #[inline(always)]
    pub const fn softrst_con1(&self) -> &SoftrstCon1 {
        &self.softrst_con1
    }
    #[doc = "0x120 - Internal reset hold control register0"]
    #[inline(always)]
    pub const fn rstnhold_con0(&self) -> &RstnholdCon0 {
        &self.rstnhold_con0
    }
    #[doc = "0x124 - Internal reset hold control register1"]
    #[inline(always)]
    pub const fn rstnhold_con1(&self) -> &RstnholdCon1 {
        &self.rstnhold_con1
    }
    #[doc = "0x130 - Internal gate disable control register0"]
    #[inline(always)]
    pub const fn gatedis_con0(&self) -> &GatedisCon0 {
        &self.gatedis_con0
    }
}
#[doc = "PPLL_CON0 (rw) register accessor: PPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con0`]
module"]
#[doc(alias = "PPLL_CON0")]
pub type PpllCon0 = crate::Reg<ppll_con0::PpllCon0Spec>;
#[doc = "PPLL configuration register0"]
pub mod ppll_con0;
#[doc = "PPLL_CON1 (rw) register accessor: PPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con1`]
module"]
#[doc(alias = "PPLL_CON1")]
pub type PpllCon1 = crate::Reg<ppll_con1::PpllCon1Spec>;
#[doc = "PPLL configuration register1"]
pub mod ppll_con1;
#[doc = "PPLL_CON2 (rw) register accessor: PPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con2`]
module"]
#[doc(alias = "PPLL_CON2")]
pub type PpllCon2 = crate::Reg<ppll_con2::PpllCon2Spec>;
#[doc = "PPLL configuration register2"]
pub mod ppll_con2;
#[doc = "PPLL_CON3 (rw) register accessor: PPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con3`]
module"]
#[doc(alias = "PPLL_CON3")]
pub type PpllCon3 = crate::Reg<ppll_con3::PpllCon3Spec>;
#[doc = "PPLL configuration register3"]
pub mod ppll_con3;
#[doc = "PPLL_CON4 (rw) register accessor: PPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con4`]
module"]
#[doc(alias = "PPLL_CON4")]
pub type PpllCon4 = crate::Reg<ppll_con4::PpllCon4Spec>;
#[doc = "PPLL configuration register4"]
pub mod ppll_con4;
#[doc = "PPLL_CON5 (rw) register accessor: PPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppll_con5`]
module"]
#[doc(alias = "PPLL_CON5")]
pub type PpllCon5 = crate::Reg<ppll_con5::PpllCon5Spec>;
#[doc = "PPLL configuration register5"]
pub mod ppll_con5;
#[doc = "CLKSEL_CON0 (rw) register accessor: Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con0`]
module"]
#[doc(alias = "CLKSEL_CON0")]
pub type ClkselCon0 = crate::Reg<clksel_con0::ClkselCon0Spec>;
#[doc = "Internal clock select and divide register0"]
pub mod clksel_con0;
#[doc = "CLKSEL_CON1 (rw) register accessor: Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con1`]
module"]
#[doc(alias = "CLKSEL_CON1")]
pub type ClkselCon1 = crate::Reg<clksel_con1::ClkselCon1Spec>;
#[doc = "Internal clock select and divide register1"]
pub mod clksel_con1;
#[doc = "CLKSEL_CON2 (rw) register accessor: Internal clock select and divide register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con2`]
module"]
#[doc(alias = "CLKSEL_CON2")]
pub type ClkselCon2 = crate::Reg<clksel_con2::ClkselCon2Spec>;
#[doc = "Internal clock select and divide register2"]
pub mod clksel_con2;
#[doc = "CLKSEL_CON3 (rw) register accessor: Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con3`]
module"]
#[doc(alias = "CLKSEL_CON3")]
pub type ClkselCon3 = crate::Reg<clksel_con3::ClkselCon3Spec>;
#[doc = "Internal clock select and divide register3"]
pub mod clksel_con3;
#[doc = "CLKSEL_CON4 (rw) register accessor: Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con4`]
module"]
#[doc(alias = "CLKSEL_CON4")]
pub type ClkselCon4 = crate::Reg<clksel_con4::ClkselCon4Spec>;
#[doc = "Internal clock select and divide register4"]
pub mod clksel_con4;
#[doc = "CLKSEL_CON5 (rw) register accessor: Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con5`]
module"]
#[doc(alias = "CLKSEL_CON5")]
pub type ClkselCon5 = crate::Reg<clksel_con5::ClkselCon5Spec>;
#[doc = "Internal clock select and divide register5"]
pub mod clksel_con5;
#[doc = "CLKFRAC_CON0 (rw) register accessor: Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkfrac_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkfrac_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkfrac_con0`]
module"]
#[doc(alias = "CLKFRAC_CON0")]
pub type ClkfracCon0 = crate::Reg<clkfrac_con0::ClkfracCon0Spec>;
#[doc = "Internal clock select and divide register6"]
pub mod clkfrac_con0;
#[doc = "CLKFRAC_CON1 (rw) register accessor: Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkfrac_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkfrac_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkfrac_con1`]
module"]
#[doc(alias = "CLKFRAC_CON1")]
pub type ClkfracCon1 = crate::Reg<clkfrac_con1::ClkfracCon1Spec>;
#[doc = "Internal clock select and divide register7"]
pub mod clkfrac_con1;
#[doc = "CLKGATE_CON0 (rw) register accessor: Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con0`]
module"]
#[doc(alias = "CLKGATE_CON0")]
pub type ClkgateCon0 = crate::Reg<clkgate_con0::ClkgateCon0Spec>;
#[doc = "Internal clock gating register0"]
pub mod clkgate_con0;
#[doc = "CLKGATE_CON1 (rw) register accessor: Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con1`]
module"]
#[doc(alias = "CLKGATE_CON1")]
pub type ClkgateCon1 = crate::Reg<clkgate_con1::ClkgateCon1Spec>;
#[doc = "Internal clock gating register1"]
pub mod clkgate_con1;
#[doc = "CLKGATE_CON2 (rw) register accessor: Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con2`]
module"]
#[doc(alias = "CLKGATE_CON2")]
pub type ClkgateCon2 = crate::Reg<clkgate_con2::ClkgateCon2Spec>;
#[doc = "Internal clock gating register2"]
pub mod clkgate_con2;
#[doc = "SOFTRST_CON0 (rw) register accessor: Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con0`]
module"]
#[doc(alias = "SOFTRST_CON0")]
pub type SoftrstCon0 = crate::Reg<softrst_con0::SoftrstCon0Spec>;
#[doc = "Internal software reset control register0"]
pub mod softrst_con0;
#[doc = "SOFTRST_CON1 (rw) register accessor: Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con1`]
module"]
#[doc(alias = "SOFTRST_CON1")]
pub type SoftrstCon1 = crate::Reg<softrst_con1::SoftrstCon1Spec>;
#[doc = "Internal software reset control register1"]
pub mod softrst_con1;
#[doc = "RSTNHOLD_CON0 (rw) register accessor: Internal reset hold control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstnhold_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstnhold_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstnhold_con0`]
module"]
#[doc(alias = "RSTNHOLD_CON0")]
pub type RstnholdCon0 = crate::Reg<rstnhold_con0::RstnholdCon0Spec>;
#[doc = "Internal reset hold control register0"]
pub mod rstnhold_con0;
#[doc = "RSTNHOLD_CON1 (rw) register accessor: Internal reset hold control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstnhold_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstnhold_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstnhold_con1`]
module"]
#[doc(alias = "RSTNHOLD_CON1")]
pub type RstnholdCon1 = crate::Reg<rstnhold_con1::RstnholdCon1Spec>;
#[doc = "Internal reset hold control register1"]
pub mod rstnhold_con1;
#[doc = "GATEDIS_CON0 (rw) register accessor: Internal gate disable control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatedis_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatedis_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatedis_con0`]
module"]
#[doc(alias = "GATEDIS_CON0")]
pub type GatedisCon0 = crate::Reg<gatedis_con0::GatedisCon0Spec>;
#[doc = "Internal gate disable control register0"]
pub mod gatedis_con0;
