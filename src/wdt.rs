#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    torr: Torr,
    ccvr: Ccvr,
    crr: Crr,
    stat: Stat,
    eoi: Eoi,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Timeout range Register"]
    #[inline(always)]
    pub const fn torr(&self) -> &Torr {
        &self.torr
    }
    #[doc = "0x08 - Current counter value Register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &Ccvr {
        &self.ccvr
    }
    #[doc = "0x0c - Counter restart Register"]
    #[inline(always)]
    pub const fn crr(&self) -> &Crr {
        &self.crr
    }
    #[doc = "0x10 - Interrupt status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x14 - Interrupt clear Register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "TORR (rw) register accessor: Timeout range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`torr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`torr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@torr`]
module"]
#[doc(alias = "TORR")]
pub type Torr = crate::Reg<torr::TorrSpec>;
#[doc = "Timeout range Register"]
pub mod torr;
#[doc = "CCVR (r) register accessor: Current counter value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`]
module"]
#[doc(alias = "CCVR")]
pub type Ccvr = crate::Reg<ccvr::CcvrSpec>;
#[doc = "Current counter value Register"]
pub mod ccvr;
#[doc = "CRR (rw) register accessor: Counter restart Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crr`]
module"]
#[doc(alias = "CRR")]
pub type Crr = crate::Reg<crr::CrrSpec>;
#[doc = "Counter restart Register"]
pub mod crr;
#[doc = "STAT (r) register accessor: Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Interrupt status Register"]
pub mod stat;
#[doc = "EOI (r) register accessor: Interrupt clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eoi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "EOI")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "Interrupt clear Register"]
pub mod eoi;
