#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdt_cr: WdtCr,
    wdt_torr: WdtTorr,
    wdt_ccvr: WdtCcvr,
    wdt_crr: WdtCrr,
    wdt_stat: WdtStat,
    wdt_eoi: WdtEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn wdt_cr(&self) -> &WdtCr {
        &self.wdt_cr
    }
    #[doc = "0x04 - Timeout range Register"]
    #[inline(always)]
    pub const fn wdt_torr(&self) -> &WdtTorr {
        &self.wdt_torr
    }
    #[doc = "0x08 - Current counter value Register"]
    #[inline(always)]
    pub const fn wdt_ccvr(&self) -> &WdtCcvr {
        &self.wdt_ccvr
    }
    #[doc = "0x0c - Counter restart Register"]
    #[inline(always)]
    pub const fn wdt_crr(&self) -> &WdtCrr {
        &self.wdt_crr
    }
    #[doc = "0x10 - Interrupt status Register"]
    #[inline(always)]
    pub const fn wdt_stat(&self) -> &WdtStat {
        &self.wdt_stat
    }
    #[doc = "0x14 - Interrupt clear Register"]
    #[inline(always)]
    pub const fn wdt_eoi(&self) -> &WdtEoi {
        &self.wdt_eoi
    }
}
#[doc = "WDT_CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_cr`]
module"]
#[doc(alias = "WDT_CR")]
pub type WdtCr = crate::Reg<wdt_cr::WdtCrSpec>;
#[doc = "Control Register"]
pub mod wdt_cr;
#[doc = "WDT_TORR (rw) register accessor: Timeout range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_torr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_torr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_torr`]
module"]
#[doc(alias = "WDT_TORR")]
pub type WdtTorr = crate::Reg<wdt_torr::WdtTorrSpec>;
#[doc = "Timeout range Register"]
pub mod wdt_torr;
#[doc = "WDT_CCVR (r) register accessor: Current counter value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_ccvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_ccvr`]
module"]
#[doc(alias = "WDT_CCVR")]
pub type WdtCcvr = crate::Reg<wdt_ccvr::WdtCcvrSpec>;
#[doc = "Current counter value Register"]
pub mod wdt_ccvr;
#[doc = "WDT_CRR (rw) register accessor: Counter restart Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_crr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_crr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_crr`]
module"]
#[doc(alias = "WDT_CRR")]
pub type WdtCrr = crate::Reg<wdt_crr::WdtCrrSpec>;
#[doc = "Counter restart Register"]
pub mod wdt_crr;
#[doc = "WDT_STAT (r) register accessor: Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_stat`]
module"]
#[doc(alias = "WDT_STAT")]
pub type WdtStat = crate::Reg<wdt_stat::WdtStatSpec>;
#[doc = "Interrupt status Register"]
pub mod wdt_stat;
#[doc = "WDT_EOI (r) register accessor: Interrupt clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_eoi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_eoi`]
module"]
#[doc(alias = "WDT_EOI")]
pub type WdtEoi = crate::Reg<wdt_eoi::WdtEoiSpec>;
#[doc = "Interrupt clear Register"]
pub mod wdt_eoi;
