#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dte_addr: DteAddr,
    status: Status,
    cmd: Cmd,
    page_fault_addr: PageFaultAddr,
    zap_one_line: ZapOneLine,
    int_rawstat: IntRawstat,
    int_clear: IntClear,
    int_mask: IntMask,
    int_status: IntStatus,
    auto_gating: AutoGating,
}
impl RegisterBlock {
    #[doc = "0x00 - MMU current page table address"]
    #[inline(always)]
    pub const fn dte_addr(&self) -> &DteAddr {
        &self.dte_addr
    }
    #[doc = "0x04 - MMU status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - MMU command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x0c - MMU logic address of last page fault register"]
    #[inline(always)]
    pub const fn page_fault_addr(&self) -> &PageFaultAddr {
        &self.page_fault_addr
    }
    #[doc = "0x10 - MMU zap cache line register"]
    #[inline(always)]
    pub const fn zap_one_line(&self) -> &ZapOneLine {
        &self.zap_one_line
    }
    #[doc = "0x14 - MMU raw interrupt status register"]
    #[inline(always)]
    pub const fn int_rawstat(&self) -> &IntRawstat {
        &self.int_rawstat
    }
    #[doc = "0x18 - MMU interrupt clear register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x1c - MMU interrupt mask register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x20 - MMU interrupt status register"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x24 - clock atuo gating register"]
    #[inline(always)]
    pub const fn auto_gating(&self) -> &AutoGating {
        &self.auto_gating
    }
}
#[doc = "DTE_ADDR (rw) register accessor: MMU current page table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dte_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dte_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dte_addr`]
module"]
#[doc(alias = "DTE_ADDR")]
pub type DteAddr = crate::Reg<dte_addr::DteAddrSpec>;
#[doc = "MMU current page table address"]
pub mod dte_addr;
#[doc = "STATUS (r) register accessor: MMU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "MMU status register"]
pub mod status;
#[doc = "CMD (rw) register accessor: MMU command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "MMU command register"]
pub mod cmd;
#[doc = "PAGE_FAULT_ADDR (r) register accessor: MMU logic address of last page fault register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`page_fault_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@page_fault_addr`]
module"]
#[doc(alias = "PAGE_FAULT_ADDR")]
pub type PageFaultAddr = crate::Reg<page_fault_addr::PageFaultAddrSpec>;
#[doc = "MMU logic address of last page fault register"]
pub mod page_fault_addr;
#[doc = "ZAP_ONE_LINE (rw) register accessor: MMU zap cache line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zap_one_line::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zap_one_line::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zap_one_line`]
module"]
#[doc(alias = "ZAP_ONE_LINE")]
pub type ZapOneLine = crate::Reg<zap_one_line::ZapOneLineSpec>;
#[doc = "MMU zap cache line register"]
pub mod zap_one_line;
#[doc = "INT_RAWSTAT (r) register accessor: MMU raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_rawstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_rawstat`]
module"]
#[doc(alias = "INT_RAWSTAT")]
pub type IntRawstat = crate::Reg<int_rawstat::IntRawstatSpec>;
#[doc = "MMU raw interrupt status register"]
pub mod int_rawstat;
#[doc = "INT_CLEAR (rw) register accessor: MMU interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`]
module"]
#[doc(alias = "INT_CLEAR")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "MMU interrupt clear register"]
pub mod int_clear;
#[doc = "INT_MASK (rw) register accessor: MMU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "MMU interrupt mask register"]
pub mod int_mask;
#[doc = "INT_STATUS (rw) register accessor: MMU interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "MMU interrupt status register"]
pub mod int_status;
#[doc = "AUTO_GATING (rw) register accessor: clock atuo gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_gating`]
module"]
#[doc(alias = "AUTO_GATING")]
pub type AutoGating = crate::Reg<auto_gating::AutoGatingSpec>;
#[doc = "clock atuo gating register"]
pub mod auto_gating;
