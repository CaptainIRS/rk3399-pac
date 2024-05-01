#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: Version,
    size: Size,
    status: Status,
    _reserved3: [u8; 0x04],
    command: Command,
    clear_page: ClearPage,
    max_reads: MaxReads,
    enable: Enable,
    perfcnt_src0: PerfcntSrc0,
    perfcnt_val0: PerfcntVal0,
    perfcnt_src1: PerfcntSrc1,
    perfcnt_val1: PerfcntVal1,
}
impl RegisterBlock {
    #[doc = "0x00 - VERSION register"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x04 - L2 cache SIZE"]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Command setting register"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x14 - clear page register"]
    #[inline(always)]
    pub const fn clear_page(&self) -> &ClearPage {
        &self.clear_page
    }
    #[doc = "0x18 - maximum read register"]
    #[inline(always)]
    pub const fn max_reads(&self) -> &MaxReads {
        &self.max_reads
    }
    #[doc = "0x1c - enables cacheable accesses and cache read allocation"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x20 - performance counter 0 source register"]
    #[inline(always)]
    pub const fn perfcnt_src0(&self) -> &PerfcntSrc0 {
        &self.perfcnt_src0
    }
    #[doc = "0x24 - performance counter 0 value register"]
    #[inline(always)]
    pub const fn perfcnt_val0(&self) -> &PerfcntVal0 {
        &self.perfcnt_val0
    }
    #[doc = "0x28 - performance counter 0 source register"]
    #[inline(always)]
    pub const fn perfcnt_src1(&self) -> &PerfcntSrc1 {
        &self.perfcnt_src1
    }
    #[doc = "0x2c - performance counter 1 value register"]
    #[inline(always)]
    pub const fn perfcnt_val1(&self) -> &PerfcntVal1 {
        &self.perfcnt_val1
    }
}
#[doc = "VERSION (r) register accessor: VERSION register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "VERSION register"]
pub mod version;
#[doc = "SIZE (r) register accessor: L2 cache SIZE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size`]
module"]
#[doc(alias = "SIZE")]
pub type Size = crate::Reg<size::SizeSpec>;
#[doc = "L2 cache SIZE"]
pub mod size;
#[doc = "STATUS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "COMMAND (rw) register accessor: Command setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command setting register"]
pub mod command;
#[doc = "CLEAR_PAGE (w) register accessor: clear page register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_page::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_page`]
module"]
#[doc(alias = "CLEAR_PAGE")]
pub type ClearPage = crate::Reg<clear_page::ClearPageSpec>;
#[doc = "clear page register"]
pub mod clear_page;
#[doc = "MAX_READS (rw) register accessor: maximum read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_reads::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`max_reads::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_reads`]
module"]
#[doc(alias = "MAX_READS")]
pub type MaxReads = crate::Reg<max_reads::MaxReadsSpec>;
#[doc = "maximum read register"]
pub mod max_reads;
#[doc = "ENABLE (rw) register accessor: enables cacheable accesses and cache read allocation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "enables cacheable accesses and cache read allocation"]
pub mod enable;
#[doc = "PERFCNT_SRC0 (rw) register accessor: performance counter 0 source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_src0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_src0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfcnt_src0`]
module"]
#[doc(alias = "PERFCNT_SRC0")]
pub type PerfcntSrc0 = crate::Reg<perfcnt_src0::PerfcntSrc0Spec>;
#[doc = "performance counter 0 source register"]
pub mod perfcnt_src0;
#[doc = "PERFCNT_VAL0 (rw) register accessor: performance counter 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_val0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_val0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfcnt_val0`]
module"]
#[doc(alias = "PERFCNT_VAL0")]
pub type PerfcntVal0 = crate::Reg<perfcnt_val0::PerfcntVal0Spec>;
#[doc = "performance counter 0 value register"]
pub mod perfcnt_val0;
#[doc = "PERFCNT_SRC1 (rw) register accessor: performance counter 0 source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_src1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_src1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfcnt_src1`]
module"]
#[doc(alias = "PERFCNT_SRC1")]
pub type PerfcntSrc1 = crate::Reg<perfcnt_src1::PerfcntSrc1Spec>;
#[doc = "performance counter 0 source register"]
pub mod perfcnt_src1;
#[doc = "PERFCNT_VAL1 (rw) register accessor: performance counter 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_val1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_val1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfcnt_val1`]
module"]
#[doc(alias = "PERFCNT_VAL1")]
pub type PerfcntVal1 = crate::Reg<perfcnt_val1::PerfcntVal1Spec>;
#[doc = "performance counter 1 value register"]
pub mod perfcnt_val1;
