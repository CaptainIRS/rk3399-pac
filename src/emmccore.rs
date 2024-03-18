#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    emmccore_saddr: EmmccoreSaddr,
    emmccore_blksiz: EmmccoreBlksiz,
    emmccore_blkcnt: EmmccoreBlkcnt,
    emmccore_arg: EmmccoreArg,
    emmccore_transmod: EmmccoreTransmod,
    emmccore_cmd: EmmccoreCmd,
    emmccore_resp0: EmmccoreResp0,
    emmccore_resp1: EmmccoreResp1,
    emmccore_resp2: EmmccoreResp2,
    emmccore_resp3: EmmccoreResp3,
    emmccore_buffer: EmmccoreBuffer,
    emmccore_prests: EmmccorePrests,
    emmccore_hostctrl1: EmmccoreHostctrl1,
    emmccore_pwrctrl: EmmccorePwrctrl,
    emmccore_blkgapctrl: EmmccoreBlkgapctrl,
    _reserved15: [u8; 0x01],
    emmccore_clkctrl: EmmccoreClkctrl,
    emmccore_timeout: EmmccoreTimeout,
    emmccore_swrst: EmmccoreSwrst,
    emmccore_norintsts: EmmccoreNorintsts,
    emmccore_errintsts: EmmccoreErrintsts,
    emmccore_norintstsena: EmmccoreNorintstsena,
    emmccore_errintstsena: EmmccoreErrintstsena,
    emmccore_norintsigena: EmmccoreNorintsigena,
    emmccore_errintsigena: EmmccoreErrintsigena,
    emmccore_acmderrsts: EmmccoreAcmderrsts,
    emmccore_hostctrl2: EmmccoreHostctrl2,
    emmccore_cap: EmmccoreCap,
    _reserved27: [u8; 0x08],
    emmccore_feacmd: EmmccoreFeacmd,
    emmccore_feerrint: EmmccoreFeerrint,
    emmccore_admaerrsts: EmmccoreAdmaerrsts,
    _reserved30: [u8; 0x02],
    emmccore_admaaddr: EmmccoreAdmaaddr,
    emmccore_pvalinit: EmmccorePvalinit,
    emmccore_pvalds: EmmccorePvalds,
    emmccore_pvalhs: EmmccorePvalhs,
    emmccore_pvalsdr12: EmmccorePvalsdr12,
    emmccore_pvalsdr25: EmmccorePvalsdr25,
    emmccore_pvalsdr50: EmmccorePvalsdr50,
    emmccore_pvalsdr104: EmmccorePvalsdr104,
    emmccore_pvalddr50: EmmccorePvalddr50,
    emmccore_boottimeout: EmmccoreBoottimeout,
    emmccore_pvalhs400: EmmccorePvalhs400,
    _reserved41: [u8; 0x02],
    emmccore_vendor: EmmccoreVendor,
    _reserved42: [u8; 0x82],
    emmccore_slotintsts: EmmccoreSlotintsts,
    emmccore_version: EmmccoreVersion,
    _reserved44: [u8; 0x0100],
    emmccore_cqver: EmmccoreCqver,
    emmccore_cqcap: EmmccoreCqcap,
    emmccore_cqcfg: EmmccoreCqcfg,
    emmccore_cqctrl: EmmccoreCqctrl,
    emmccore_cqintsts: EmmccoreCqintsts,
    emmccore_cqintstsena: EmmccoreCqintstsena,
    emmccore_cqintsigena: EmmccoreCqintsigena,
    emmccore_cqintcoal: EmmccoreCqintcoal,
    emmccore_cqtdlba: EmmccoreCqtdlba,
    emmccore_cqtdlbau: EmmccoreCqtdlbau,
    emmccore_cqtdb: EmmccoreCqtdb,
    emmccore_cqtdbn: EmmccoreCqtdbn,
    emmccore_cqdqsts: EmmccoreCqdqsts,
    emmccore_cqdpt: EmmccoreCqdpt,
    emmccore_cqtclr: EmmccoreCqtclr,
    _reserved59: [u8; 0x04],
    emmccore_cqssc1: EmmccoreCqssc1,
    emmccore_cqssc2: EmmccoreCqssc2,
    emmccore_cqcrdt: EmmccoreCqcrdt,
    _reserved62: [u8; 0x04],
    emmccore_cqrmem: EmmccoreCqrmem,
    emmccore_cqtei: EmmccoreCqtei,
    emmccore_cqcri: EmmccoreCqcri,
    emmccore_cqcra: EmmccoreCqcra,
}
impl RegisterBlock {
    #[doc = "0x00 - System address/ Argument 2 register"]
    #[inline(always)]
    pub const fn emmccore_saddr(&self) -> &EmmccoreSaddr {
        &self.emmccore_saddr
    }
    #[doc = "0x04 - Block size register"]
    #[inline(always)]
    pub const fn emmccore_blksiz(&self) -> &EmmccoreBlksiz {
        &self.emmccore_blksiz
    }
    #[doc = "0x06 - Block count register"]
    #[inline(always)]
    pub const fn emmccore_blkcnt(&self) -> &EmmccoreBlkcnt {
        &self.emmccore_blkcnt
    }
    #[doc = "0x08 - Argument register"]
    #[inline(always)]
    pub const fn emmccore_arg(&self) -> &EmmccoreArg {
        &self.emmccore_arg
    }
    #[doc = "0x0c - Transfer mode register"]
    #[inline(always)]
    pub const fn emmccore_transmod(&self) -> &EmmccoreTransmod {
        &self.emmccore_transmod
    }
    #[doc = "0x0e - Command register"]
    #[inline(always)]
    pub const fn emmccore_cmd(&self) -> &EmmccoreCmd {
        &self.emmccore_cmd
    }
    #[doc = "0x10 - Response register bit \\[31:0\\]"]
    #[inline(always)]
    pub const fn emmccore_resp0(&self) -> &EmmccoreResp0 {
        &self.emmccore_resp0
    }
    #[doc = "0x14 - Response register bit \\[63:32\\]"]
    #[inline(always)]
    pub const fn emmccore_resp1(&self) -> &EmmccoreResp1 {
        &self.emmccore_resp1
    }
    #[doc = "0x18 - Response register bit \\[95:64\\]"]
    #[inline(always)]
    pub const fn emmccore_resp2(&self) -> &EmmccoreResp2 {
        &self.emmccore_resp2
    }
    #[doc = "0x1c - Response register bit \\[127:98\\]"]
    #[inline(always)]
    pub const fn emmccore_resp3(&self) -> &EmmccoreResp3 {
        &self.emmccore_resp3
    }
    #[doc = "0x20 - Buffer data port register"]
    #[inline(always)]
    pub const fn emmccore_buffer(&self) -> &EmmccoreBuffer {
        &self.emmccore_buffer
    }
    #[doc = "0x24 - Present state register"]
    #[inline(always)]
    pub const fn emmccore_prests(&self) -> &EmmccorePrests {
        &self.emmccore_prests
    }
    #[doc = "0x28 - Host control 1 register"]
    #[inline(always)]
    pub const fn emmccore_hostctrl1(&self) -> &EmmccoreHostctrl1 {
        &self.emmccore_hostctrl1
    }
    #[doc = "0x29 - Power control register"]
    #[inline(always)]
    pub const fn emmccore_pwrctrl(&self) -> &EmmccorePwrctrl {
        &self.emmccore_pwrctrl
    }
    #[doc = "0x2a - Block gap control register"]
    #[inline(always)]
    pub const fn emmccore_blkgapctrl(&self) -> &EmmccoreBlkgapctrl {
        &self.emmccore_blkgapctrl
    }
    #[doc = "0x2c - Clock control Register"]
    #[inline(always)]
    pub const fn emmccore_clkctrl(&self) -> &EmmccoreClkctrl {
        &self.emmccore_clkctrl
    }
    #[doc = "0x2e - Timeout control register"]
    #[inline(always)]
    pub const fn emmccore_timeout(&self) -> &EmmccoreTimeout {
        &self.emmccore_timeout
    }
    #[doc = "0x2f - Software reset register"]
    #[inline(always)]
    pub const fn emmccore_swrst(&self) -> &EmmccoreSwrst {
        &self.emmccore_swrst
    }
    #[doc = "0x30 - Normal interrupt status register"]
    #[inline(always)]
    pub const fn emmccore_norintsts(&self) -> &EmmccoreNorintsts {
        &self.emmccore_norintsts
    }
    #[doc = "0x32 - Error interrupt status register"]
    #[inline(always)]
    pub const fn emmccore_errintsts(&self) -> &EmmccoreErrintsts {
        &self.emmccore_errintsts
    }
    #[doc = "0x34 - Normal interrupt status enable register"]
    #[inline(always)]
    pub const fn emmccore_norintstsena(&self) -> &EmmccoreNorintstsena {
        &self.emmccore_norintstsena
    }
    #[doc = "0x36 - Error interrupt status enable register"]
    #[inline(always)]
    pub const fn emmccore_errintstsena(&self) -> &EmmccoreErrintstsena {
        &self.emmccore_errintstsena
    }
    #[doc = "0x38 - Normal interrupt signal enable register"]
    #[inline(always)]
    pub const fn emmccore_norintsigena(&self) -> &EmmccoreNorintsigena {
        &self.emmccore_norintsigena
    }
    #[doc = "0x3a - Error interrupt signal enable register"]
    #[inline(always)]
    pub const fn emmccore_errintsigena(&self) -> &EmmccoreErrintsigena {
        &self.emmccore_errintsigena
    }
    #[doc = "0x3c - Auto CMD error status register"]
    #[inline(always)]
    pub const fn emmccore_acmderrsts(&self) -> &EmmccoreAcmderrsts {
        &self.emmccore_acmderrsts
    }
    #[doc = "0x3e - Host Control 2 Register"]
    #[inline(always)]
    pub const fn emmccore_hostctrl2(&self) -> &EmmccoreHostctrl2 {
        &self.emmccore_hostctrl2
    }
    #[doc = "0x40..0x48 - Capabilities register"]
    #[inline(always)]
    pub const fn emmccore_cap(&self) -> &EmmccoreCap {
        &self.emmccore_cap
    }
    #[doc = "0x50 - Force event register for Auto CMD error status"]
    #[inline(always)]
    pub const fn emmccore_feacmd(&self) -> &EmmccoreFeacmd {
        &self.emmccore_feacmd
    }
    #[doc = "0x52 - Force event register for error interrupt status"]
    #[inline(always)]
    pub const fn emmccore_feerrint(&self) -> &EmmccoreFeerrint {
        &self.emmccore_feerrint
    }
    #[doc = "0x54 - ADMA error status register"]
    #[inline(always)]
    pub const fn emmccore_admaerrsts(&self) -> &EmmccoreAdmaerrsts {
        &self.emmccore_admaerrsts
    }
    #[doc = "0x58..0x60 - ADMA system address register"]
    #[inline(always)]
    pub const fn emmccore_admaaddr(&self) -> &EmmccoreAdmaaddr {
        &self.emmccore_admaaddr
    }
    #[doc = "0x60 - Preset value register for Initialization"]
    #[inline(always)]
    pub const fn emmccore_pvalinit(&self) -> &EmmccorePvalinit {
        &self.emmccore_pvalinit
    }
    #[doc = "0x62 - Preset value register for Default Speed"]
    #[inline(always)]
    pub const fn emmccore_pvalds(&self) -> &EmmccorePvalds {
        &self.emmccore_pvalds
    }
    #[doc = "0x64 - Preset value register for High Speed"]
    #[inline(always)]
    pub const fn emmccore_pvalhs(&self) -> &EmmccorePvalhs {
        &self.emmccore_pvalhs
    }
    #[doc = "0x66 - Preset value register for SDR12"]
    #[inline(always)]
    pub const fn emmccore_pvalsdr12(&self) -> &EmmccorePvalsdr12 {
        &self.emmccore_pvalsdr12
    }
    #[doc = "0x68 - Preset value register for SDR25"]
    #[inline(always)]
    pub const fn emmccore_pvalsdr25(&self) -> &EmmccorePvalsdr25 {
        &self.emmccore_pvalsdr25
    }
    #[doc = "0x6a - Preset value register for SDR50"]
    #[inline(always)]
    pub const fn emmccore_pvalsdr50(&self) -> &EmmccorePvalsdr50 {
        &self.emmccore_pvalsdr50
    }
    #[doc = "0x6c - Preset value register for SDR104"]
    #[inline(always)]
    pub const fn emmccore_pvalsdr104(&self) -> &EmmccorePvalsdr104 {
        &self.emmccore_pvalsdr104
    }
    #[doc = "0x6e - Preset value register for DDR50"]
    #[inline(always)]
    pub const fn emmccore_pvalddr50(&self) -> &EmmccorePvalddr50 {
        &self.emmccore_pvalddr50
    }
    #[doc = "0x70 - Boot timeout control register"]
    #[inline(always)]
    pub const fn emmccore_boottimeout(&self) -> &EmmccoreBoottimeout {
        &self.emmccore_boottimeout
    }
    #[doc = "0x74 - Preset value register for HS400"]
    #[inline(always)]
    pub const fn emmccore_pvalhs400(&self) -> &EmmccorePvalhs400 {
        &self.emmccore_pvalhs400
    }
    #[doc = "0x78 - Vendor register"]
    #[inline(always)]
    pub const fn emmccore_vendor(&self) -> &EmmccoreVendor {
        &self.emmccore_vendor
    }
    #[doc = "0xfc - Slot interrupt status register"]
    #[inline(always)]
    pub const fn emmccore_slotintsts(&self) -> &EmmccoreSlotintsts {
        &self.emmccore_slotintsts
    }
    #[doc = "0xfe - Host controller version register"]
    #[inline(always)]
    pub const fn emmccore_version(&self) -> &EmmccoreVersion {
        &self.emmccore_version
    }
    #[doc = "0x200 - Command queueing version register"]
    #[inline(always)]
    pub const fn emmccore_cqver(&self) -> &EmmccoreCqver {
        &self.emmccore_cqver
    }
    #[doc = "0x204 - Command queueing capabilities register"]
    #[inline(always)]
    pub const fn emmccore_cqcap(&self) -> &EmmccoreCqcap {
        &self.emmccore_cqcap
    }
    #[doc = "0x208 - Command queueing configuration register"]
    #[inline(always)]
    pub const fn emmccore_cqcfg(&self) -> &EmmccoreCqcfg {
        &self.emmccore_cqcfg
    }
    #[doc = "0x20c - Command queueing control register"]
    #[inline(always)]
    pub const fn emmccore_cqctrl(&self) -> &EmmccoreCqctrl {
        &self.emmccore_cqctrl
    }
    #[doc = "0x210 - Command queueing interrupt status register"]
    #[inline(always)]
    pub const fn emmccore_cqintsts(&self) -> &EmmccoreCqintsts {
        &self.emmccore_cqintsts
    }
    #[doc = "0x214 - Command queueing interrupt status enable register"]
    #[inline(always)]
    pub const fn emmccore_cqintstsena(&self) -> &EmmccoreCqintstsena {
        &self.emmccore_cqintstsena
    }
    #[doc = "0x218 - Command queueing interrupt signal enable register"]
    #[inline(always)]
    pub const fn emmccore_cqintsigena(&self) -> &EmmccoreCqintsigena {
        &self.emmccore_cqintsigena
    }
    #[doc = "0x21c - Command queueing interrupt coalescing register"]
    #[inline(always)]
    pub const fn emmccore_cqintcoal(&self) -> &EmmccoreCqintcoal {
        &self.emmccore_cqintcoal
    }
    #[doc = "0x220 - Command queueing task descriptor list base address register"]
    #[inline(always)]
    pub const fn emmccore_cqtdlba(&self) -> &EmmccoreCqtdlba {
        &self.emmccore_cqtdlba
    }
    #[doc = "0x224 - Command queueing task descriptor list base address upper 32bits register"]
    #[inline(always)]
    pub const fn emmccore_cqtdlbau(&self) -> &EmmccoreCqtdlbau {
        &self.emmccore_cqtdlbau
    }
    #[doc = "0x228 - Command queueing task doorbell register"]
    #[inline(always)]
    pub const fn emmccore_cqtdb(&self) -> &EmmccoreCqtdb {
        &self.emmccore_cqtdb
    }
    #[doc = "0x22c - Command queueing task doorbell notification register"]
    #[inline(always)]
    pub const fn emmccore_cqtdbn(&self) -> &EmmccoreCqtdbn {
        &self.emmccore_cqtdbn
    }
    #[doc = "0x230 - Command queueing device queue status register"]
    #[inline(always)]
    pub const fn emmccore_cqdqsts(&self) -> &EmmccoreCqdqsts {
        &self.emmccore_cqdqsts
    }
    #[doc = "0x234 - Command queueing device pending tasks register"]
    #[inline(always)]
    pub const fn emmccore_cqdpt(&self) -> &EmmccoreCqdpt {
        &self.emmccore_cqdpt
    }
    #[doc = "0x238 - Command queueing task clear register"]
    #[inline(always)]
    pub const fn emmccore_cqtclr(&self) -> &EmmccoreCqtclr {
        &self.emmccore_cqtclr
    }
    #[doc = "0x240 - Command queueing send status configuration register 1"]
    #[inline(always)]
    pub const fn emmccore_cqssc1(&self) -> &EmmccoreCqssc1 {
        &self.emmccore_cqssc1
    }
    #[doc = "0x244 - Command queueing send status configuration register 2"]
    #[inline(always)]
    pub const fn emmccore_cqssc2(&self) -> &EmmccoreCqssc2 {
        &self.emmccore_cqssc2
    }
    #[doc = "0x248 - Command queueing command response for direct-command task register"]
    #[inline(always)]
    pub const fn emmccore_cqcrdt(&self) -> &EmmccoreCqcrdt {
        &self.emmccore_cqcrdt
    }
    #[doc = "0x250 - Command queueing response mode error mask register"]
    #[inline(always)]
    pub const fn emmccore_cqrmem(&self) -> &EmmccoreCqrmem {
        &self.emmccore_cqrmem
    }
    #[doc = "0x254 - Command queueing task error information register"]
    #[inline(always)]
    pub const fn emmccore_cqtei(&self) -> &EmmccoreCqtei {
        &self.emmccore_cqtei
    }
    #[doc = "0x258 - Command queueing command response index register"]
    #[inline(always)]
    pub const fn emmccore_cqcri(&self) -> &EmmccoreCqcri {
        &self.emmccore_cqcri
    }
    #[doc = "0x25c - Command queueing command response argument register"]
    #[inline(always)]
    pub const fn emmccore_cqcra(&self) -> &EmmccoreCqcra {
        &self.emmccore_cqcra
    }
}
#[doc = "EMMCCORE_SADDR (rw) register accessor: System address/ Argument 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_saddr`]
module"]
#[doc(alias = "EMMCCORE_SADDR")]
pub type EmmccoreSaddr = crate::Reg<emmccore_saddr::EmmccoreSaddrSpec>;
#[doc = "System address/ Argument 2 register"]
pub mod emmccore_saddr;
#[doc = "EMMCCORE_BLKSIZ (rw) register accessor: Block size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_blksiz`]
module"]
#[doc(alias = "EMMCCORE_BLKSIZ")]
pub type EmmccoreBlksiz = crate::Reg<emmccore_blksiz::EmmccoreBlksizSpec>;
#[doc = "Block size register"]
pub mod emmccore_blksiz;
#[doc = "EMMCCORE_BLKCNT (rw) register accessor: Block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_blkcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_blkcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_blkcnt`]
module"]
#[doc(alias = "EMMCCORE_BLKCNT")]
pub type EmmccoreBlkcnt = crate::Reg<emmccore_blkcnt::EmmccoreBlkcntSpec>;
#[doc = "Block count register"]
pub mod emmccore_blkcnt;
#[doc = "EMMCCORE_ARG (rw) register accessor: Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_arg`]
module"]
#[doc(alias = "EMMCCORE_ARG")]
pub type EmmccoreArg = crate::Reg<emmccore_arg::EmmccoreArgSpec>;
#[doc = "Argument register"]
pub mod emmccore_arg;
#[doc = "EMMCCORE_TRANSMOD (rw) register accessor: Transfer mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_transmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_transmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_transmod`]
module"]
#[doc(alias = "EMMCCORE_TRANSMOD")]
pub type EmmccoreTransmod = crate::Reg<emmccore_transmod::EmmccoreTransmodSpec>;
#[doc = "Transfer mode register"]
pub mod emmccore_transmod;
#[doc = "EMMCCORE_CMD (rw) register accessor: Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cmd`]
module"]
#[doc(alias = "EMMCCORE_CMD")]
pub type EmmccoreCmd = crate::Reg<emmccore_cmd::EmmccoreCmdSpec>;
#[doc = "Command register"]
pub mod emmccore_cmd;
#[doc = "EMMCCORE_RESP0 (rw) register accessor: Response register bit \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_resp0`]
module"]
#[doc(alias = "EMMCCORE_RESP0")]
pub type EmmccoreResp0 = crate::Reg<emmccore_resp0::EmmccoreResp0Spec>;
#[doc = "Response register bit \\[31:0\\]"]
pub mod emmccore_resp0;
#[doc = "EMMCCORE_RESP1 (rw) register accessor: Response register bit \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_resp1`]
module"]
#[doc(alias = "EMMCCORE_RESP1")]
pub type EmmccoreResp1 = crate::Reg<emmccore_resp1::EmmccoreResp1Spec>;
#[doc = "Response register bit \\[63:32\\]"]
pub mod emmccore_resp1;
#[doc = "EMMCCORE_RESP2 (rw) register accessor: Response register bit \\[95:64\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_resp2`]
module"]
#[doc(alias = "EMMCCORE_RESP2")]
pub type EmmccoreResp2 = crate::Reg<emmccore_resp2::EmmccoreResp2Spec>;
#[doc = "Response register bit \\[95:64\\]"]
pub mod emmccore_resp2;
#[doc = "EMMCCORE_RESP3 (rw) register accessor: Response register bit \\[127:98\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_resp3`]
module"]
#[doc(alias = "EMMCCORE_RESP3")]
pub type EmmccoreResp3 = crate::Reg<emmccore_resp3::EmmccoreResp3Spec>;
#[doc = "Response register bit \\[127:98\\]"]
pub mod emmccore_resp3;
#[doc = "EMMCCORE_BUFFER (rw) register accessor: Buffer data port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_buffer`]
module"]
#[doc(alias = "EMMCCORE_BUFFER")]
pub type EmmccoreBuffer = crate::Reg<emmccore_buffer::EmmccoreBufferSpec>;
#[doc = "Buffer data port register"]
pub mod emmccore_buffer;
#[doc = "EMMCCORE_PRESTS (rw) register accessor: Present state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_prests::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_prests::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_prests`]
module"]
#[doc(alias = "EMMCCORE_PRESTS")]
pub type EmmccorePrests = crate::Reg<emmccore_prests::EmmccorePrestsSpec>;
#[doc = "Present state register"]
pub mod emmccore_prests;
#[doc = "EMMCCORE_HOSTCTRL1 (rw) register accessor: Host control 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_hostctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_hostctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_hostctrl1`]
module"]
#[doc(alias = "EMMCCORE_HOSTCTRL1")]
pub type EmmccoreHostctrl1 = crate::Reg<emmccore_hostctrl1::EmmccoreHostctrl1Spec>;
#[doc = "Host control 1 register"]
pub mod emmccore_hostctrl1;
#[doc = "EMMCCORE_PWRCTRL (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pwrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_pwrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pwrctrl`]
module"]
#[doc(alias = "EMMCCORE_PWRCTRL")]
pub type EmmccorePwrctrl = crate::Reg<emmccore_pwrctrl::EmmccorePwrctrlSpec>;
#[doc = "Power control register"]
pub mod emmccore_pwrctrl;
#[doc = "EMMCCORE_BLKGAPCTRL (rw) register accessor: Block gap control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_blkgapctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_blkgapctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_blkgapctrl`]
module"]
#[doc(alias = "EMMCCORE_BLKGAPCTRL")]
pub type EmmccoreBlkgapctrl = crate::Reg<emmccore_blkgapctrl::EmmccoreBlkgapctrlSpec>;
#[doc = "Block gap control register"]
pub mod emmccore_blkgapctrl;
#[doc = "EMMCCORE_CLKCTRL (rw) register accessor: Clock control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_clkctrl`]
module"]
#[doc(alias = "EMMCCORE_CLKCTRL")]
pub type EmmccoreClkctrl = crate::Reg<emmccore_clkctrl::EmmccoreClkctrlSpec>;
#[doc = "Clock control Register"]
pub mod emmccore_clkctrl;
#[doc = "EMMCCORE_TIMEOUT (rw) register accessor: Timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_timeout`]
module"]
#[doc(alias = "EMMCCORE_TIMEOUT")]
pub type EmmccoreTimeout = crate::Reg<emmccore_timeout::EmmccoreTimeoutSpec>;
#[doc = "Timeout control register"]
pub mod emmccore_timeout;
#[doc = "EMMCCORE_SWRST (rw) register accessor: Software reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_swrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_swrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_swrst`]
module"]
#[doc(alias = "EMMCCORE_SWRST")]
pub type EmmccoreSwrst = crate::Reg<emmccore_swrst::EmmccoreSwrstSpec>;
#[doc = "Software reset register"]
pub mod emmccore_swrst;
#[doc = "EMMCCORE_NORINTSTS (rw) register accessor: Normal interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_norintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_norintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_norintsts`]
module"]
#[doc(alias = "EMMCCORE_NORINTSTS")]
pub type EmmccoreNorintsts = crate::Reg<emmccore_norintsts::EmmccoreNorintstsSpec>;
#[doc = "Normal interrupt status register"]
pub mod emmccore_norintsts;
#[doc = "EMMCCORE_ERRINTSTS (rw) register accessor: Error interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_errintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_errintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_errintsts`]
module"]
#[doc(alias = "EMMCCORE_ERRINTSTS")]
pub type EmmccoreErrintsts = crate::Reg<emmccore_errintsts::EmmccoreErrintstsSpec>;
#[doc = "Error interrupt status register"]
pub mod emmccore_errintsts;
#[doc = "EMMCCORE_NORINTSTSENA (rw) register accessor: Normal interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_norintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_norintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_norintstsena`]
module"]
#[doc(alias = "EMMCCORE_NORINTSTSENA")]
pub type EmmccoreNorintstsena = crate::Reg<emmccore_norintstsena::EmmccoreNorintstsenaSpec>;
#[doc = "Normal interrupt status enable register"]
pub mod emmccore_norintstsena;
#[doc = "EMMCCORE_ERRINTSTSENA (rw) register accessor: Error interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_errintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_errintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_errintstsena`]
module"]
#[doc(alias = "EMMCCORE_ERRINTSTSENA")]
pub type EmmccoreErrintstsena = crate::Reg<emmccore_errintstsena::EmmccoreErrintstsenaSpec>;
#[doc = "Error interrupt status enable register"]
pub mod emmccore_errintstsena;
#[doc = "EMMCCORE_NORINTSIGENA (rw) register accessor: Normal interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_norintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_norintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_norintsigena`]
module"]
#[doc(alias = "EMMCCORE_NORINTSIGENA")]
pub type EmmccoreNorintsigena = crate::Reg<emmccore_norintsigena::EmmccoreNorintsigenaSpec>;
#[doc = "Normal interrupt signal enable register"]
pub mod emmccore_norintsigena;
#[doc = "EMMCCORE_ERRINTSIGENA (rw) register accessor: Error interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_errintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_errintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_errintsigena`]
module"]
#[doc(alias = "EMMCCORE_ERRINTSIGENA")]
pub type EmmccoreErrintsigena = crate::Reg<emmccore_errintsigena::EmmccoreErrintsigenaSpec>;
#[doc = "Error interrupt signal enable register"]
pub mod emmccore_errintsigena;
#[doc = "EMMCCORE_ACMDERRSTS (r) register accessor: Auto CMD error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_acmderrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_acmderrsts`]
module"]
#[doc(alias = "EMMCCORE_ACMDERRSTS")]
pub type EmmccoreAcmderrsts = crate::Reg<emmccore_acmderrsts::EmmccoreAcmderrstsSpec>;
#[doc = "Auto CMD error status register"]
pub mod emmccore_acmderrsts;
#[doc = "EMMCCORE_HOSTCTRL2 (rw) register accessor: Host Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_hostctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_hostctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_hostctrl2`]
module"]
#[doc(alias = "EMMCCORE_HOSTCTRL2")]
pub type EmmccoreHostctrl2 = crate::Reg<emmccore_hostctrl2::EmmccoreHostctrl2Spec>;
#[doc = "Host Control 2 Register"]
pub mod emmccore_hostctrl2;
#[doc = "EMMCCORE_CAP (rw) register accessor: Capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cap`]
module"]
#[doc(alias = "EMMCCORE_CAP")]
pub type EmmccoreCap = crate::Reg<emmccore_cap::EmmccoreCapSpec>;
#[doc = "Capabilities register"]
pub mod emmccore_cap;
#[doc = "EMMCCORE_FEACMD (w) register accessor: Force event register for Auto CMD error status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_feacmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_feacmd`]
module"]
#[doc(alias = "EMMCCORE_FEACMD")]
pub type EmmccoreFeacmd = crate::Reg<emmccore_feacmd::EmmccoreFeacmdSpec>;
#[doc = "Force event register for Auto CMD error status"]
pub mod emmccore_feacmd;
#[doc = "EMMCCORE_FEERRINT (rw) register accessor: Force event register for error interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_feerrint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_feerrint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_feerrint`]
module"]
#[doc(alias = "EMMCCORE_FEERRINT")]
pub type EmmccoreFeerrint = crate::Reg<emmccore_feerrint::EmmccoreFeerrintSpec>;
#[doc = "Force event register for error interrupt status"]
pub mod emmccore_feerrint;
#[doc = "EMMCCORE_ADMAERRSTS (r) register accessor: ADMA error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_admaerrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_admaerrsts`]
module"]
#[doc(alias = "EMMCCORE_ADMAERRSTS")]
pub type EmmccoreAdmaerrsts = crate::Reg<emmccore_admaerrsts::EmmccoreAdmaerrstsSpec>;
#[doc = "ADMA error status register"]
pub mod emmccore_admaerrsts;
#[doc = "EMMCCORE_ADMAADDR (rw) register accessor: ADMA system address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_admaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_admaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_admaaddr`]
module"]
#[doc(alias = "EMMCCORE_ADMAADDR")]
pub type EmmccoreAdmaaddr = crate::Reg<emmccore_admaaddr::EmmccoreAdmaaddrSpec>;
#[doc = "ADMA system address register"]
pub mod emmccore_admaaddr;
#[doc = "EMMCCORE_PVALINIT (r) register accessor: Preset value register for Initialization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalinit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalinit`]
module"]
#[doc(alias = "EMMCCORE_PVALINIT")]
pub type EmmccorePvalinit = crate::Reg<emmccore_pvalinit::EmmccorePvalinitSpec>;
#[doc = "Preset value register for Initialization"]
pub mod emmccore_pvalinit;
#[doc = "EMMCCORE_PVALDS (r) register accessor: Preset value register for Default Speed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalds`]
module"]
#[doc(alias = "EMMCCORE_PVALDS")]
pub type EmmccorePvalds = crate::Reg<emmccore_pvalds::EmmccorePvaldsSpec>;
#[doc = "Preset value register for Default Speed"]
pub mod emmccore_pvalds;
#[doc = "EMMCCORE_PVALHS (r) register accessor: Preset value register for High Speed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalhs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalhs`]
module"]
#[doc(alias = "EMMCCORE_PVALHS")]
pub type EmmccorePvalhs = crate::Reg<emmccore_pvalhs::EmmccorePvalhsSpec>;
#[doc = "Preset value register for High Speed"]
pub mod emmccore_pvalhs;
#[doc = "EMMCCORE_PVALSDR12 (r) register accessor: Preset value register for SDR12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalsdr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalsdr12`]
module"]
#[doc(alias = "EMMCCORE_PVALSDR12")]
pub type EmmccorePvalsdr12 = crate::Reg<emmccore_pvalsdr12::EmmccorePvalsdr12Spec>;
#[doc = "Preset value register for SDR12"]
pub mod emmccore_pvalsdr12;
#[doc = "EMMCCORE_PVALSDR25 (r) register accessor: Preset value register for SDR25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalsdr25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalsdr25`]
module"]
#[doc(alias = "EMMCCORE_PVALSDR25")]
pub type EmmccorePvalsdr25 = crate::Reg<emmccore_pvalsdr25::EmmccorePvalsdr25Spec>;
#[doc = "Preset value register for SDR25"]
pub mod emmccore_pvalsdr25;
#[doc = "EMMCCORE_PVALSDR50 (r) register accessor: Preset value register for SDR50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalsdr50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalsdr50`]
module"]
#[doc(alias = "EMMCCORE_PVALSDR50")]
pub type EmmccorePvalsdr50 = crate::Reg<emmccore_pvalsdr50::EmmccorePvalsdr50Spec>;
#[doc = "Preset value register for SDR50"]
pub mod emmccore_pvalsdr50;
#[doc = "EMMCCORE_PVALSDR104 (r) register accessor: Preset value register for SDR104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalsdr104::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalsdr104`]
module"]
#[doc(alias = "EMMCCORE_PVALSDR104")]
pub type EmmccorePvalsdr104 = crate::Reg<emmccore_pvalsdr104::EmmccorePvalsdr104Spec>;
#[doc = "Preset value register for SDR104"]
pub mod emmccore_pvalsdr104;
#[doc = "EMMCCORE_PVALDDR50 (r) register accessor: Preset value register for DDR50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalddr50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalddr50`]
module"]
#[doc(alias = "EMMCCORE_PVALDDR50")]
pub type EmmccorePvalddr50 = crate::Reg<emmccore_pvalddr50::EmmccorePvalddr50Spec>;
#[doc = "Preset value register for DDR50"]
pub mod emmccore_pvalddr50;
#[doc = "EMMCCORE_BOOTTIMEOUT (rw) register accessor: Boot timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_boottimeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_boottimeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_boottimeout`]
module"]
#[doc(alias = "EMMCCORE_BOOTTIMEOUT")]
pub type EmmccoreBoottimeout = crate::Reg<emmccore_boottimeout::EmmccoreBoottimeoutSpec>;
#[doc = "Boot timeout control register"]
pub mod emmccore_boottimeout;
#[doc = "EMMCCORE_PVALHS400 (r) register accessor: Preset value register for HS400\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalhs400::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_pvalhs400`]
module"]
#[doc(alias = "EMMCCORE_PVALHS400")]
pub type EmmccorePvalhs400 = crate::Reg<emmccore_pvalhs400::EmmccorePvalhs400Spec>;
#[doc = "Preset value register for HS400"]
pub mod emmccore_pvalhs400;
#[doc = "EMMCCORE_VENDOR (rw) register accessor: Vendor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_vendor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_vendor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_vendor`]
module"]
#[doc(alias = "EMMCCORE_VENDOR")]
pub type EmmccoreVendor = crate::Reg<emmccore_vendor::EmmccoreVendorSpec>;
#[doc = "Vendor register"]
pub mod emmccore_vendor;
#[doc = "EMMCCORE_SLOTINTSTS (r) register accessor: Slot interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_slotintsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_slotintsts`]
module"]
#[doc(alias = "EMMCCORE_SLOTINTSTS")]
pub type EmmccoreSlotintsts = crate::Reg<emmccore_slotintsts::EmmccoreSlotintstsSpec>;
#[doc = "Slot interrupt status register"]
pub mod emmccore_slotintsts;
#[doc = "EMMCCORE_VERSION (r) register accessor: Host controller version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_version`]
module"]
#[doc(alias = "EMMCCORE_VERSION")]
pub type EmmccoreVersion = crate::Reg<emmccore_version::EmmccoreVersionSpec>;
#[doc = "Host controller version register"]
pub mod emmccore_version;
#[doc = "EMMCCORE_CQVER (r) register accessor: Command queueing version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqver`]
module"]
#[doc(alias = "EMMCCORE_CQVER")]
pub type EmmccoreCqver = crate::Reg<emmccore_cqver::EmmccoreCqverSpec>;
#[doc = "Command queueing version register"]
pub mod emmccore_cqver;
#[doc = "EMMCCORE_CQCAP (r) register accessor: Command queueing capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqcap`]
module"]
#[doc(alias = "EMMCCORE_CQCAP")]
pub type EmmccoreCqcap = crate::Reg<emmccore_cqcap::EmmccoreCqcapSpec>;
#[doc = "Command queueing capabilities register"]
pub mod emmccore_cqcap;
#[doc = "EMMCCORE_CQCFG (rw) register accessor: Command queueing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqcfg`]
module"]
#[doc(alias = "EMMCCORE_CQCFG")]
pub type EmmccoreCqcfg = crate::Reg<emmccore_cqcfg::EmmccoreCqcfgSpec>;
#[doc = "Command queueing configuration register"]
pub mod emmccore_cqcfg;
#[doc = "EMMCCORE_CQCTRL (rw) register accessor: Command queueing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqctrl`]
module"]
#[doc(alias = "EMMCCORE_CQCTRL")]
pub type EmmccoreCqctrl = crate::Reg<emmccore_cqctrl::EmmccoreCqctrlSpec>;
#[doc = "Command queueing control register"]
pub mod emmccore_cqctrl;
#[doc = "EMMCCORE_CQINTSTS (rw) register accessor: Command queueing interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqintsts`]
module"]
#[doc(alias = "EMMCCORE_CQINTSTS")]
pub type EmmccoreCqintsts = crate::Reg<emmccore_cqintsts::EmmccoreCqintstsSpec>;
#[doc = "Command queueing interrupt status register"]
pub mod emmccore_cqintsts;
#[doc = "EMMCCORE_CQINTSTSENA (rw) register accessor: Command queueing interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqintstsena`]
module"]
#[doc(alias = "EMMCCORE_CQINTSTSENA")]
pub type EmmccoreCqintstsena = crate::Reg<emmccore_cqintstsena::EmmccoreCqintstsenaSpec>;
#[doc = "Command queueing interrupt status enable register"]
pub mod emmccore_cqintstsena;
#[doc = "EMMCCORE_CQINTSIGENA (rw) register accessor: Command queueing interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqintsigena`]
module"]
#[doc(alias = "EMMCCORE_CQINTSIGENA")]
pub type EmmccoreCqintsigena = crate::Reg<emmccore_cqintsigena::EmmccoreCqintsigenaSpec>;
#[doc = "Command queueing interrupt signal enable register"]
pub mod emmccore_cqintsigena;
#[doc = "EMMCCORE_CQINTCOAL (rw) register accessor: Command queueing interrupt coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintcoal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintcoal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqintcoal`]
module"]
#[doc(alias = "EMMCCORE_CQINTCOAL")]
pub type EmmccoreCqintcoal = crate::Reg<emmccore_cqintcoal::EmmccoreCqintcoalSpec>;
#[doc = "Command queueing interrupt coalescing register"]
pub mod emmccore_cqintcoal;
#[doc = "EMMCCORE_CQTDLBA (rw) register accessor: Command queueing task descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdlba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdlba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtdlba`]
module"]
#[doc(alias = "EMMCCORE_CQTDLBA")]
pub type EmmccoreCqtdlba = crate::Reg<emmccore_cqtdlba::EmmccoreCqtdlbaSpec>;
#[doc = "Command queueing task descriptor list base address register"]
pub mod emmccore_cqtdlba;
#[doc = "EMMCCORE_CQTDLBAU (rw) register accessor: Command queueing task descriptor list base address upper 32bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdlbau::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdlbau::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtdlbau`]
module"]
#[doc(alias = "EMMCCORE_CQTDLBAU")]
pub type EmmccoreCqtdlbau = crate::Reg<emmccore_cqtdlbau::EmmccoreCqtdlbauSpec>;
#[doc = "Command queueing task descriptor list base address upper 32bits register"]
pub mod emmccore_cqtdlbau;
#[doc = "EMMCCORE_CQTDB (rw) register accessor: Command queueing task doorbell register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtdb`]
module"]
#[doc(alias = "EMMCCORE_CQTDB")]
pub type EmmccoreCqtdb = crate::Reg<emmccore_cqtdb::EmmccoreCqtdbSpec>;
#[doc = "Command queueing task doorbell register"]
pub mod emmccore_cqtdb;
#[doc = "EMMCCORE_CQTDBN (rw) register accessor: Command queueing task doorbell notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtdbn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtdbn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtdbn`]
module"]
#[doc(alias = "EMMCCORE_CQTDBN")]
pub type EmmccoreCqtdbn = crate::Reg<emmccore_cqtdbn::EmmccoreCqtdbnSpec>;
#[doc = "Command queueing task doorbell notification register"]
pub mod emmccore_cqtdbn;
#[doc = "EMMCCORE_CQDQSTS (r) register accessor: Command queueing device queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqdqsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqdqsts`]
module"]
#[doc(alias = "EMMCCORE_CQDQSTS")]
pub type EmmccoreCqdqsts = crate::Reg<emmccore_cqdqsts::EmmccoreCqdqstsSpec>;
#[doc = "Command queueing device queue status register"]
pub mod emmccore_cqdqsts;
#[doc = "EMMCCORE_CQDPT (r) register accessor: Command queueing device pending tasks register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqdpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqdpt`]
module"]
#[doc(alias = "EMMCCORE_CQDPT")]
pub type EmmccoreCqdpt = crate::Reg<emmccore_cqdpt::EmmccoreCqdptSpec>;
#[doc = "Command queueing device pending tasks register"]
pub mod emmccore_cqdpt;
#[doc = "EMMCCORE_CQTCLR (rw) register accessor: Command queueing task clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqtclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtclr`]
module"]
#[doc(alias = "EMMCCORE_CQTCLR")]
pub type EmmccoreCqtclr = crate::Reg<emmccore_cqtclr::EmmccoreCqtclrSpec>;
#[doc = "Command queueing task clear register"]
pub mod emmccore_cqtclr;
#[doc = "EMMCCORE_CQSSC1 (rw) register accessor: Command queueing send status configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqssc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqssc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqssc1`]
module"]
#[doc(alias = "EMMCCORE_CQSSC1")]
pub type EmmccoreCqssc1 = crate::Reg<emmccore_cqssc1::EmmccoreCqssc1Spec>;
#[doc = "Command queueing send status configuration register 1"]
pub mod emmccore_cqssc1;
#[doc = "EMMCCORE_CQSSC2 (rw) register accessor: Command queueing send status configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqssc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqssc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqssc2`]
module"]
#[doc(alias = "EMMCCORE_CQSSC2")]
pub type EmmccoreCqssc2 = crate::Reg<emmccore_cqssc2::EmmccoreCqssc2Spec>;
#[doc = "Command queueing send status configuration register 2"]
pub mod emmccore_cqssc2;
#[doc = "EMMCCORE_CQCRDT (r) register accessor: Command queueing command response for direct-command task register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcrdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqcrdt`]
module"]
#[doc(alias = "EMMCCORE_CQCRDT")]
pub type EmmccoreCqcrdt = crate::Reg<emmccore_cqcrdt::EmmccoreCqcrdtSpec>;
#[doc = "Command queueing command response for direct-command task register"]
pub mod emmccore_cqcrdt;
#[doc = "EMMCCORE_CQRMEM (r) register accessor: Command queueing response mode error mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqrmem::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqrmem`]
module"]
#[doc(alias = "EMMCCORE_CQRMEM")]
pub type EmmccoreCqrmem = crate::Reg<emmccore_cqrmem::EmmccoreCqrmemSpec>;
#[doc = "Command queueing response mode error mask register"]
pub mod emmccore_cqrmem;
#[doc = "EMMCCORE_CQTEI (r) register accessor: Command queueing task error information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtei::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqtei`]
module"]
#[doc(alias = "EMMCCORE_CQTEI")]
pub type EmmccoreCqtei = crate::Reg<emmccore_cqtei::EmmccoreCqteiSpec>;
#[doc = "Command queueing task error information register"]
pub mod emmccore_cqtei;
#[doc = "EMMCCORE_CQCRI (r) register accessor: Command queueing command response index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcri::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqcri`]
module"]
#[doc(alias = "EMMCCORE_CQCRI")]
pub type EmmccoreCqcri = crate::Reg<emmccore_cqcri::EmmccoreCqcriSpec>;
#[doc = "Command queueing command response index register"]
pub mod emmccore_cqcri;
#[doc = "EMMCCORE_CQCRA (r) register accessor: Command queueing command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqcra::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_cqcra`]
module"]
#[doc(alias = "EMMCCORE_CQCRA")]
pub type EmmccoreCqcra = crate::Reg<emmccore_cqcra::EmmccoreCqcraSpec>;
#[doc = "Command queueing command response argument register"]
pub mod emmccore_cqcra;
