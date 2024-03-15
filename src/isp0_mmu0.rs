#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmu_dte_addr: MmuDteAddr,
    mmu_status: MmuStatus,
    mmu_cmd: MmuCmd,
    mmu_page_fault_addr: MmuPageFaultAddr,
    mmu_zap_one_line: MmuZapOneLine,
    mmu_int_rawstat: MmuIntRawstat,
    mmu_int_clear: MmuIntClear,
    mmu_int_mask: MmuIntMask,
    mmu_int_status: MmuIntStatus,
    mmu_auto_gating: MmuAutoGating,
}
impl RegisterBlock {
    #[doc = "0x00 - MMU current page table address"]
    #[inline(always)]
    pub const fn mmu_dte_addr(&self) -> &MmuDteAddr {
        &self.mmu_dte_addr
    }
    #[doc = "0x04 - MMU status register"]
    #[inline(always)]
    pub const fn mmu_status(&self) -> &MmuStatus {
        &self.mmu_status
    }
    #[doc = "0x08 - MMU command register"]
    #[inline(always)]
    pub const fn mmu_cmd(&self) -> &MmuCmd {
        &self.mmu_cmd
    }
    #[doc = "0x0c - MMU logic address of last page fault register"]
    #[inline(always)]
    pub const fn mmu_page_fault_addr(&self) -> &MmuPageFaultAddr {
        &self.mmu_page_fault_addr
    }
    #[doc = "0x10 - MMU zap cache line register"]
    #[inline(always)]
    pub const fn mmu_zap_one_line(&self) -> &MmuZapOneLine {
        &self.mmu_zap_one_line
    }
    #[doc = "0x14 - MMU raw interrupt status register"]
    #[inline(always)]
    pub const fn mmu_int_rawstat(&self) -> &MmuIntRawstat {
        &self.mmu_int_rawstat
    }
    #[doc = "0x18 - MMU interrupt clear register"]
    #[inline(always)]
    pub const fn mmu_int_clear(&self) -> &MmuIntClear {
        &self.mmu_int_clear
    }
    #[doc = "0x1c - MMU interrupt mask register"]
    #[inline(always)]
    pub const fn mmu_int_mask(&self) -> &MmuIntMask {
        &self.mmu_int_mask
    }
    #[doc = "0x20 - MMU interrupt status register"]
    #[inline(always)]
    pub const fn mmu_int_status(&self) -> &MmuIntStatus {
        &self.mmu_int_status
    }
    #[doc = "0x24 - clock atuo gating register"]
    #[inline(always)]
    pub const fn mmu_auto_gating(&self) -> &MmuAutoGating {
        &self.mmu_auto_gating
    }
}
#[doc = "MMU_DTE_ADDR (rw) register accessor: MMU current page table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_dte_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_dte_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_dte_addr`]
module"]
#[doc(alias = "MMU_DTE_ADDR")]
pub type MmuDteAddr = crate::Reg<mmu_dte_addr::MmuDteAddrSpec>;
#[doc = "MMU current page table address"]
pub mod mmu_dte_addr;
#[doc = "MMU_STATUS (r) register accessor: MMU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_status`]
module"]
#[doc(alias = "MMU_STATUS")]
pub type MmuStatus = crate::Reg<mmu_status::MmuStatusSpec>;
#[doc = "MMU status register"]
pub mod mmu_status;
#[doc = "MMU_CMD (rw) register accessor: MMU command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_cmd`]
module"]
#[doc(alias = "MMU_CMD")]
pub type MmuCmd = crate::Reg<mmu_cmd::MmuCmdSpec>;
#[doc = "MMU command register"]
pub mod mmu_cmd;
#[doc = "MMU_PAGE_FAULT_ADDR (r) register accessor: MMU logic address of last page fault register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_page_fault_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_page_fault_addr`]
module"]
#[doc(alias = "MMU_PAGE_FAULT_ADDR")]
pub type MmuPageFaultAddr = crate::Reg<mmu_page_fault_addr::MmuPageFaultAddrSpec>;
#[doc = "MMU logic address of last page fault register"]
pub mod mmu_page_fault_addr;
#[doc = "MMU_ZAP_ONE_LINE (rw) register accessor: MMU zap cache line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_zap_one_line::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_zap_one_line::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_zap_one_line`]
module"]
#[doc(alias = "MMU_ZAP_ONE_LINE")]
pub type MmuZapOneLine = crate::Reg<mmu_zap_one_line::MmuZapOneLineSpec>;
#[doc = "MMU zap cache line register"]
pub mod mmu_zap_one_line;
#[doc = "MMU_INT_RAWSTAT (r) register accessor: MMU raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_int_rawstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_int_rawstat`]
module"]
#[doc(alias = "MMU_INT_RAWSTAT")]
pub type MmuIntRawstat = crate::Reg<mmu_int_rawstat::MmuIntRawstatSpec>;
#[doc = "MMU raw interrupt status register"]
pub mod mmu_int_rawstat;
#[doc = "MMU_INT_CLEAR (rw) register accessor: MMU interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_int_clear`]
module"]
#[doc(alias = "MMU_INT_CLEAR")]
pub type MmuIntClear = crate::Reg<mmu_int_clear::MmuIntClearSpec>;
#[doc = "MMU interrupt clear register"]
pub mod mmu_int_clear;
#[doc = "MMU_INT_MASK (rw) register accessor: MMU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_int_mask`]
module"]
#[doc(alias = "MMU_INT_MASK")]
pub type MmuIntMask = crate::Reg<mmu_int_mask::MmuIntMaskSpec>;
#[doc = "MMU interrupt mask register"]
pub mod mmu_int_mask;
#[doc = "MMU_INT_STATUS (rw) register accessor: MMU interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_int_status`]
module"]
#[doc(alias = "MMU_INT_STATUS")]
pub type MmuIntStatus = crate::Reg<mmu_int_status::MmuIntStatusSpec>;
#[doc = "MMU interrupt status register"]
pub mod mmu_int_status;
#[doc = "MMU_AUTO_GATING (rw) register accessor: clock atuo gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_auto_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_auto_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_auto_gating`]
module"]
#[doc(alias = "MMU_AUTO_GATING")]
pub type MmuAutoGating = crate::Reg<mmu_auto_gating::MmuAutoGatingSpec>;
#[doc = "clock atuo gating register"]
pub mod mmu_auto_gating;
