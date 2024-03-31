#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    saddr: Saddr,
    blksiz: Blksiz,
    blkcnt: Blkcnt,
    arg: Arg,
    transmod: Transmod,
    cmd: Cmd,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    buffer: Buffer,
    prests: Prests,
    hostctrl1: Hostctrl1,
    pwrctrl: Pwrctrl,
    blkgapctrl: Blkgapctrl,
    _reserved15: [u8; 0x01],
    clkctrl: Clkctrl,
    timeout: Timeout,
    swrst: Swrst,
    norintsts: Norintsts,
    errintsts: Errintsts,
    norintstsena: Norintstsena,
    errintstsena: Errintstsena,
    norintsigena: Norintsigena,
    errintsigena: Errintsigena,
    acmderrsts: Acmderrsts,
    hostctrl2: Hostctrl2,
    cap: Cap,
    _reserved27: [u8; 0x08],
    feacmd: Feacmd,
    feerrint: Feerrint,
    admaerrsts: Admaerrsts,
    _reserved30: [u8; 0x02],
    admaaddr: Admaaddr,
    pvalinit: Pvalinit,
    pvalds: Pvalds,
    pvalhs: Pvalhs,
    pvalsdr12: Pvalsdr12,
    pvalsdr25: Pvalsdr25,
    pvalsdr50: Pvalsdr50,
    pvalsdr104: Pvalsdr104,
    pvalddr50: Pvalddr50,
    boottimeout: Boottimeout,
    pvalhs400: Pvalhs400,
    _reserved41: [u8; 0x02],
    vendor: Vendor,
    _reserved42: [u8; 0x82],
    slotintsts: Slotintsts,
    version: Version,
    _reserved44: [u8; 0x0100],
    cqver: Cqver,
    cqcap: Cqcap,
    cqcfg: Cqcfg,
    cqctrl: Cqctrl,
    cqintsts: Cqintsts,
    cqintstsena: Cqintstsena,
    cqintsigena: Cqintsigena,
    cqintcoal: Cqintcoal,
    cqtdlba: Cqtdlba,
    cqtdlbau: Cqtdlbau,
    cqtdb: Cqtdb,
    cqtdbn: Cqtdbn,
    cqdqsts: Cqdqsts,
    cqdpt: Cqdpt,
    cqtclr: Cqtclr,
    _reserved59: [u8; 0x04],
    cqssc1: Cqssc1,
    cqssc2: Cqssc2,
    cqcrdt: Cqcrdt,
    _reserved62: [u8; 0x04],
    cqrmem: Cqrmem,
    cqtei: Cqtei,
    cqcri: Cqcri,
    cqcra: Cqcra,
}
impl RegisterBlock {
    #[doc = "0x00 - System address/ Argument 2 register"]
    #[inline(always)]
    pub const fn saddr(&self) -> &Saddr {
        &self.saddr
    }
    #[doc = "0x04 - Block size register"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x06 - Block count register"]
    #[inline(always)]
    pub const fn blkcnt(&self) -> &Blkcnt {
        &self.blkcnt
    }
    #[doc = "0x08 - Argument register"]
    #[inline(always)]
    pub const fn arg(&self) -> &Arg {
        &self.arg
    }
    #[doc = "0x0c - Transfer mode register"]
    #[inline(always)]
    pub const fn transmod(&self) -> &Transmod {
        &self.transmod
    }
    #[doc = "0x0e - Command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - Response register bit \\[31:0\\]"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x14 - Response register bit \\[63:32\\]"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x18 - Response register bit \\[95:64\\]"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x1c - Response register bit \\[127:98\\]"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x20 - Buffer data port register"]
    #[inline(always)]
    pub const fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    #[doc = "0x24 - Present state register"]
    #[inline(always)]
    pub const fn prests(&self) -> &Prests {
        &self.prests
    }
    #[doc = "0x28 - Host control 1 register"]
    #[inline(always)]
    pub const fn hostctrl1(&self) -> &Hostctrl1 {
        &self.hostctrl1
    }
    #[doc = "0x29 - Power control register"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    #[doc = "0x2a - Block gap control register"]
    #[inline(always)]
    pub const fn blkgapctrl(&self) -> &Blkgapctrl {
        &self.blkgapctrl
    }
    #[doc = "0x2c - Clock control Register"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x2e - Timeout control register"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x2f - Software reset register"]
    #[inline(always)]
    pub const fn swrst(&self) -> &Swrst {
        &self.swrst
    }
    #[doc = "0x30 - Normal interrupt status register"]
    #[inline(always)]
    pub const fn norintsts(&self) -> &Norintsts {
        &self.norintsts
    }
    #[doc = "0x32 - Error interrupt status register"]
    #[inline(always)]
    pub const fn errintsts(&self) -> &Errintsts {
        &self.errintsts
    }
    #[doc = "0x34 - Normal interrupt status enable register"]
    #[inline(always)]
    pub const fn norintstsena(&self) -> &Norintstsena {
        &self.norintstsena
    }
    #[doc = "0x36 - Error interrupt status enable register"]
    #[inline(always)]
    pub const fn errintstsena(&self) -> &Errintstsena {
        &self.errintstsena
    }
    #[doc = "0x38 - Normal interrupt signal enable register"]
    #[inline(always)]
    pub const fn norintsigena(&self) -> &Norintsigena {
        &self.norintsigena
    }
    #[doc = "0x3a - Error interrupt signal enable register"]
    #[inline(always)]
    pub const fn errintsigena(&self) -> &Errintsigena {
        &self.errintsigena
    }
    #[doc = "0x3c - Auto CMD error status register"]
    #[inline(always)]
    pub const fn acmderrsts(&self) -> &Acmderrsts {
        &self.acmderrsts
    }
    #[doc = "0x3e - Host Control 2 Register"]
    #[inline(always)]
    pub const fn hostctrl2(&self) -> &Hostctrl2 {
        &self.hostctrl2
    }
    #[doc = "0x40..0x48 - Capabilities register"]
    #[inline(always)]
    pub const fn cap(&self) -> &Cap {
        &self.cap
    }
    #[doc = "0x50 - Force event register for Auto CMD error status"]
    #[inline(always)]
    pub const fn feacmd(&self) -> &Feacmd {
        &self.feacmd
    }
    #[doc = "0x52 - Force event register for error interrupt status"]
    #[inline(always)]
    pub const fn feerrint(&self) -> &Feerrint {
        &self.feerrint
    }
    #[doc = "0x54 - ADMA error status register"]
    #[inline(always)]
    pub const fn admaerrsts(&self) -> &Admaerrsts {
        &self.admaerrsts
    }
    #[doc = "0x58..0x60 - ADMA system address register"]
    #[inline(always)]
    pub const fn admaaddr(&self) -> &Admaaddr {
        &self.admaaddr
    }
    #[doc = "0x60 - Preset value register for Initialization"]
    #[inline(always)]
    pub const fn pvalinit(&self) -> &Pvalinit {
        &self.pvalinit
    }
    #[doc = "0x62 - Preset value register for Default Speed"]
    #[inline(always)]
    pub const fn pvalds(&self) -> &Pvalds {
        &self.pvalds
    }
    #[doc = "0x64 - Preset value register for High Speed"]
    #[inline(always)]
    pub const fn pvalhs(&self) -> &Pvalhs {
        &self.pvalhs
    }
    #[doc = "0x66 - Preset value register for SDR12"]
    #[inline(always)]
    pub const fn pvalsdr12(&self) -> &Pvalsdr12 {
        &self.pvalsdr12
    }
    #[doc = "0x68 - Preset value register for SDR25"]
    #[inline(always)]
    pub const fn pvalsdr25(&self) -> &Pvalsdr25 {
        &self.pvalsdr25
    }
    #[doc = "0x6a - Preset value register for SDR50"]
    #[inline(always)]
    pub const fn pvalsdr50(&self) -> &Pvalsdr50 {
        &self.pvalsdr50
    }
    #[doc = "0x6c - Preset value register for SDR104"]
    #[inline(always)]
    pub const fn pvalsdr104(&self) -> &Pvalsdr104 {
        &self.pvalsdr104
    }
    #[doc = "0x6e - Preset value register for DDR50"]
    #[inline(always)]
    pub const fn pvalddr50(&self) -> &Pvalddr50 {
        &self.pvalddr50
    }
    #[doc = "0x70 - Boot timeout control register"]
    #[inline(always)]
    pub const fn boottimeout(&self) -> &Boottimeout {
        &self.boottimeout
    }
    #[doc = "0x74 - Preset value register for HS400"]
    #[inline(always)]
    pub const fn pvalhs400(&self) -> &Pvalhs400 {
        &self.pvalhs400
    }
    #[doc = "0x78 - Vendor register"]
    #[inline(always)]
    pub const fn vendor(&self) -> &Vendor {
        &self.vendor
    }
    #[doc = "0xfc - Slot interrupt status register"]
    #[inline(always)]
    pub const fn slotintsts(&self) -> &Slotintsts {
        &self.slotintsts
    }
    #[doc = "0xfe - Host controller version register"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x200 - Command queueing version register"]
    #[inline(always)]
    pub const fn cqver(&self) -> &Cqver {
        &self.cqver
    }
    #[doc = "0x204 - Command queueing capabilities register"]
    #[inline(always)]
    pub const fn cqcap(&self) -> &Cqcap {
        &self.cqcap
    }
    #[doc = "0x208 - Command queueing configuration register"]
    #[inline(always)]
    pub const fn cqcfg(&self) -> &Cqcfg {
        &self.cqcfg
    }
    #[doc = "0x20c - Command queueing control register"]
    #[inline(always)]
    pub const fn cqctrl(&self) -> &Cqctrl {
        &self.cqctrl
    }
    #[doc = "0x210 - Command queueing interrupt status register"]
    #[inline(always)]
    pub const fn cqintsts(&self) -> &Cqintsts {
        &self.cqintsts
    }
    #[doc = "0x214 - Command queueing interrupt status enable register"]
    #[inline(always)]
    pub const fn cqintstsena(&self) -> &Cqintstsena {
        &self.cqintstsena
    }
    #[doc = "0x218 - Command queueing interrupt signal enable register"]
    #[inline(always)]
    pub const fn cqintsigena(&self) -> &Cqintsigena {
        &self.cqintsigena
    }
    #[doc = "0x21c - Command queueing interrupt coalescing register"]
    #[inline(always)]
    pub const fn cqintcoal(&self) -> &Cqintcoal {
        &self.cqintcoal
    }
    #[doc = "0x220 - Command queueing task descriptor list base address register"]
    #[inline(always)]
    pub const fn cqtdlba(&self) -> &Cqtdlba {
        &self.cqtdlba
    }
    #[doc = "0x224 - Command queueing task descriptor list base address upper 32bits register"]
    #[inline(always)]
    pub const fn cqtdlbau(&self) -> &Cqtdlbau {
        &self.cqtdlbau
    }
    #[doc = "0x228 - Command queueing task doorbell register"]
    #[inline(always)]
    pub const fn cqtdb(&self) -> &Cqtdb {
        &self.cqtdb
    }
    #[doc = "0x22c - Command queueing task doorbell notification register"]
    #[inline(always)]
    pub const fn cqtdbn(&self) -> &Cqtdbn {
        &self.cqtdbn
    }
    #[doc = "0x230 - Command queueing device queue status register"]
    #[inline(always)]
    pub const fn cqdqsts(&self) -> &Cqdqsts {
        &self.cqdqsts
    }
    #[doc = "0x234 - Command queueing device pending tasks register"]
    #[inline(always)]
    pub const fn cqdpt(&self) -> &Cqdpt {
        &self.cqdpt
    }
    #[doc = "0x238 - Command queueing task clear register"]
    #[inline(always)]
    pub const fn cqtclr(&self) -> &Cqtclr {
        &self.cqtclr
    }
    #[doc = "0x240 - Command queueing send status configuration register 1"]
    #[inline(always)]
    pub const fn cqssc1(&self) -> &Cqssc1 {
        &self.cqssc1
    }
    #[doc = "0x244 - Command queueing send status configuration register 2"]
    #[inline(always)]
    pub const fn cqssc2(&self) -> &Cqssc2 {
        &self.cqssc2
    }
    #[doc = "0x248 - Command queueing command response for direct-command task register"]
    #[inline(always)]
    pub const fn cqcrdt(&self) -> &Cqcrdt {
        &self.cqcrdt
    }
    #[doc = "0x250 - Command queueing response mode error mask register"]
    #[inline(always)]
    pub const fn cqrmem(&self) -> &Cqrmem {
        &self.cqrmem
    }
    #[doc = "0x254 - Command queueing task error information register"]
    #[inline(always)]
    pub const fn cqtei(&self) -> &Cqtei {
        &self.cqtei
    }
    #[doc = "0x258 - Command queueing command response index register"]
    #[inline(always)]
    pub const fn cqcri(&self) -> &Cqcri {
        &self.cqcri
    }
    #[doc = "0x25c - Command queueing command response argument register"]
    #[inline(always)]
    pub const fn cqcra(&self) -> &Cqcra {
        &self.cqcra
    }
}
#[doc = "SADDR (rw) register accessor: System address/ Argument 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr`]
module"]
#[doc(alias = "SADDR")]
pub type Saddr = crate::Reg<saddr::SaddrSpec>;
#[doc = "System address/ Argument 2 register"]
pub mod saddr;
#[doc = "BLKSIZ (rw) register accessor: Block size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`]
module"]
#[doc(alias = "BLKSIZ")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "Block size register"]
pub mod blksiz;
#[doc = "BLKCNT (rw) register accessor: Block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkcnt`]
module"]
#[doc(alias = "BLKCNT")]
pub type Blkcnt = crate::Reg<blkcnt::BlkcntSpec>;
#[doc = "Block count register"]
pub mod blkcnt;
#[doc = "ARG (rw) register accessor: Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`]
module"]
#[doc(alias = "ARG")]
pub type Arg = crate::Reg<arg::ArgSpec>;
#[doc = "Argument register"]
pub mod arg;
#[doc = "TRANSMOD (rw) register accessor: Transfer mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmod`]
module"]
#[doc(alias = "TRANSMOD")]
pub type Transmod = crate::Reg<transmod::TransmodSpec>;
#[doc = "Transfer mode register"]
pub mod transmod;
#[doc = "CMD (rw) register accessor: Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "RESP0 (rw) register accessor: Response register bit \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`]
module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "Response register bit \\[31:0\\]"]
pub mod resp0;
#[doc = "RESP1 (rw) register accessor: Response register bit \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`]
module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "Response register bit \\[63:32\\]"]
pub mod resp1;
#[doc = "RESP2 (rw) register accessor: Response register bit \\[95:64\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Response register bit \\[95:64\\]"]
pub mod resp2;
#[doc = "RESP3 (rw) register accessor: Response register bit \\[127:98\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "Response register bit \\[127:98\\]"]
pub mod resp3;
#[doc = "BUFFER (rw) register accessor: Buffer data port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer`]
module"]
#[doc(alias = "BUFFER")]
pub type Buffer = crate::Reg<buffer::BufferSpec>;
#[doc = "Buffer data port register"]
pub mod buffer;
#[doc = "PRESTS (rw) register accessor: Present state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prests::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prests::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prests`]
module"]
#[doc(alias = "PRESTS")]
pub type Prests = crate::Reg<prests::PrestsSpec>;
#[doc = "Present state register"]
pub mod prests;
#[doc = "HOSTCTRL1 (rw) register accessor: Host control 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostctrl1`]
module"]
#[doc(alias = "HOSTCTRL1")]
pub type Hostctrl1 = crate::Reg<hostctrl1::Hostctrl1Spec>;
#[doc = "Host control 1 register"]
pub mod hostctrl1;
#[doc = "PWRCTRL (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`]
module"]
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PwrctrlSpec>;
#[doc = "Power control register"]
pub mod pwrctrl;
#[doc = "BLKGAPCTRL (rw) register accessor: Block gap control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkgapctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkgapctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkgapctrl`]
module"]
#[doc(alias = "BLKGAPCTRL")]
pub type Blkgapctrl = crate::Reg<blkgapctrl::BlkgapctrlSpec>;
#[doc = "Block gap control register"]
pub mod blkgapctrl;
#[doc = "CLKCTRL (rw) register accessor: Clock control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock control Register"]
pub mod clkctrl;
#[doc = "TIMEOUT (rw) register accessor: Timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout control register"]
pub mod timeout;
#[doc = "SWRST (rw) register accessor: Software reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst`]
module"]
#[doc(alias = "SWRST")]
pub type Swrst = crate::Reg<swrst::SwrstSpec>;
#[doc = "Software reset register"]
pub mod swrst;
#[doc = "NORINTSTS (rw) register accessor: Normal interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`norintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`norintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintsts`]
module"]
#[doc(alias = "NORINTSTS")]
pub type Norintsts = crate::Reg<norintsts::NorintstsSpec>;
#[doc = "Normal interrupt status register"]
pub mod norintsts;
#[doc = "ERRINTSTS (rw) register accessor: Error interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintsts`]
module"]
#[doc(alias = "ERRINTSTS")]
pub type Errintsts = crate::Reg<errintsts::ErrintstsSpec>;
#[doc = "Error interrupt status register"]
pub mod errintsts;
#[doc = "NORINTSTSENA (rw) register accessor: Normal interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`norintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`norintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintstsena`]
module"]
#[doc(alias = "NORINTSTSENA")]
pub type Norintstsena = crate::Reg<norintstsena::NorintstsenaSpec>;
#[doc = "Normal interrupt status enable register"]
pub mod norintstsena;
#[doc = "ERRINTSTSENA (rw) register accessor: Error interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintstsena`]
module"]
#[doc(alias = "ERRINTSTSENA")]
pub type Errintstsena = crate::Reg<errintstsena::ErrintstsenaSpec>;
#[doc = "Error interrupt status enable register"]
pub mod errintstsena;
#[doc = "NORINTSIGENA (rw) register accessor: Normal interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`norintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`norintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintsigena`]
module"]
#[doc(alias = "NORINTSIGENA")]
pub type Norintsigena = crate::Reg<norintsigena::NorintsigenaSpec>;
#[doc = "Normal interrupt signal enable register"]
pub mod norintsigena;
#[doc = "ERRINTSIGENA (rw) register accessor: Error interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintsigena`]
module"]
#[doc(alias = "ERRINTSIGENA")]
pub type Errintsigena = crate::Reg<errintsigena::ErrintsigenaSpec>;
#[doc = "Error interrupt signal enable register"]
pub mod errintsigena;
#[doc = "ACMDERRSTS (r) register accessor: Auto CMD error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmderrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmderrsts`]
module"]
#[doc(alias = "ACMDERRSTS")]
pub type Acmderrsts = crate::Reg<acmderrsts::AcmderrstsSpec>;
#[doc = "Auto CMD error status register"]
pub mod acmderrsts;
#[doc = "HOSTCTRL2 (rw) register accessor: Host Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostctrl2`]
module"]
#[doc(alias = "HOSTCTRL2")]
pub type Hostctrl2 = crate::Reg<hostctrl2::Hostctrl2Spec>;
#[doc = "Host Control 2 Register"]
pub mod hostctrl2;
#[doc = "CAP (rw) register accessor: Capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "CAP")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Capabilities register"]
pub mod cap;
#[doc = "FEACMD (w) register accessor: Force event register for Auto CMD error status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feacmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feacmd`]
module"]
#[doc(alias = "FEACMD")]
pub type Feacmd = crate::Reg<feacmd::FeacmdSpec>;
#[doc = "Force event register for Auto CMD error status"]
pub mod feacmd;
#[doc = "FEERRINT (rw) register accessor: Force event register for error interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feerrint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feerrint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feerrint`]
module"]
#[doc(alias = "FEERRINT")]
pub type Feerrint = crate::Reg<feerrint::FeerrintSpec>;
#[doc = "Force event register for error interrupt status"]
pub mod feerrint;
#[doc = "ADMAERRSTS (r) register accessor: ADMA error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admaerrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admaerrsts`]
module"]
#[doc(alias = "ADMAERRSTS")]
pub type Admaerrsts = crate::Reg<admaerrsts::AdmaerrstsSpec>;
#[doc = "ADMA error status register"]
pub mod admaerrsts;
#[doc = "ADMAADDR (rw) register accessor: ADMA system address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admaaddr`]
module"]
#[doc(alias = "ADMAADDR")]
pub type Admaaddr = crate::Reg<admaaddr::AdmaaddrSpec>;
#[doc = "ADMA system address register"]
pub mod admaaddr;
#[doc = "PVALINIT (r) register accessor: Preset value register for Initialization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalinit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalinit`]
module"]
#[doc(alias = "PVALINIT")]
pub type Pvalinit = crate::Reg<pvalinit::PvalinitSpec>;
#[doc = "Preset value register for Initialization"]
pub mod pvalinit;
#[doc = "PVALDS (r) register accessor: Preset value register for Default Speed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalds`]
module"]
#[doc(alias = "PVALDS")]
pub type Pvalds = crate::Reg<pvalds::PvaldsSpec>;
#[doc = "Preset value register for Default Speed"]
pub mod pvalds;
#[doc = "PVALHS (r) register accessor: Preset value register for High Speed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalhs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalhs`]
module"]
#[doc(alias = "PVALHS")]
pub type Pvalhs = crate::Reg<pvalhs::PvalhsSpec>;
#[doc = "Preset value register for High Speed"]
pub mod pvalhs;
#[doc = "PVALSDR12 (r) register accessor: Preset value register for SDR12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalsdr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalsdr12`]
module"]
#[doc(alias = "PVALSDR12")]
pub type Pvalsdr12 = crate::Reg<pvalsdr12::Pvalsdr12Spec>;
#[doc = "Preset value register for SDR12"]
pub mod pvalsdr12;
#[doc = "PVALSDR25 (r) register accessor: Preset value register for SDR25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalsdr25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalsdr25`]
module"]
#[doc(alias = "PVALSDR25")]
pub type Pvalsdr25 = crate::Reg<pvalsdr25::Pvalsdr25Spec>;
#[doc = "Preset value register for SDR25"]
pub mod pvalsdr25;
#[doc = "PVALSDR50 (r) register accessor: Preset value register for SDR50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalsdr50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalsdr50`]
module"]
#[doc(alias = "PVALSDR50")]
pub type Pvalsdr50 = crate::Reg<pvalsdr50::Pvalsdr50Spec>;
#[doc = "Preset value register for SDR50"]
pub mod pvalsdr50;
#[doc = "PVALSDR104 (r) register accessor: Preset value register for SDR104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalsdr104::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalsdr104`]
module"]
#[doc(alias = "PVALSDR104")]
pub type Pvalsdr104 = crate::Reg<pvalsdr104::Pvalsdr104Spec>;
#[doc = "Preset value register for SDR104"]
pub mod pvalsdr104;
#[doc = "PVALDDR50 (r) register accessor: Preset value register for DDR50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalddr50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalddr50`]
module"]
#[doc(alias = "PVALDDR50")]
pub type Pvalddr50 = crate::Reg<pvalddr50::Pvalddr50Spec>;
#[doc = "Preset value register for DDR50"]
pub mod pvalddr50;
#[doc = "BOOTTIMEOUT (rw) register accessor: Boot timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boottimeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boottimeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boottimeout`]
module"]
#[doc(alias = "BOOTTIMEOUT")]
pub type Boottimeout = crate::Reg<boottimeout::BoottimeoutSpec>;
#[doc = "Boot timeout control register"]
pub mod boottimeout;
#[doc = "PVALHS400 (r) register accessor: Preset value register for HS400\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvalhs400::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvalhs400`]
module"]
#[doc(alias = "PVALHS400")]
pub type Pvalhs400 = crate::Reg<pvalhs400::Pvalhs400Spec>;
#[doc = "Preset value register for HS400"]
pub mod pvalhs400;
#[doc = "VENDOR (rw) register accessor: Vendor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vendor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendor`]
module"]
#[doc(alias = "VENDOR")]
pub type Vendor = crate::Reg<vendor::VendorSpec>;
#[doc = "Vendor register"]
pub mod vendor;
#[doc = "SLOTINTSTS (r) register accessor: Slot interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotintsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotintsts`]
module"]
#[doc(alias = "SLOTINTSTS")]
pub type Slotintsts = crate::Reg<slotintsts::SlotintstsSpec>;
#[doc = "Slot interrupt status register"]
pub mod slotintsts;
#[doc = "VERSION (r) register accessor: Host controller version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "Host controller version register"]
pub mod version;
#[doc = "CQVER (r) register accessor: Command queueing version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqver`]
module"]
#[doc(alias = "CQVER")]
pub type Cqver = crate::Reg<cqver::CqverSpec>;
#[doc = "Command queueing version register"]
pub mod cqver;
#[doc = "CQCAP (r) register accessor: Command queueing capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcap`]
module"]
#[doc(alias = "CQCAP")]
pub type Cqcap = crate::Reg<cqcap::CqcapSpec>;
#[doc = "Command queueing capabilities register"]
pub mod cqcap;
#[doc = "CQCFG (rw) register accessor: Command queueing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg`]
module"]
#[doc(alias = "CQCFG")]
pub type Cqcfg = crate::Reg<cqcfg::CqcfgSpec>;
#[doc = "Command queueing configuration register"]
pub mod cqcfg;
#[doc = "CQCTRL (rw) register accessor: Command queueing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqctrl`]
module"]
#[doc(alias = "CQCTRL")]
pub type Cqctrl = crate::Reg<cqctrl::CqctrlSpec>;
#[doc = "Command queueing control register"]
pub mod cqctrl;
#[doc = "CQINTSTS (rw) register accessor: Command queueing interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqintsts`]
module"]
#[doc(alias = "CQINTSTS")]
pub type Cqintsts = crate::Reg<cqintsts::CqintstsSpec>;
#[doc = "Command queueing interrupt status register"]
pub mod cqintsts;
#[doc = "CQINTSTSENA (rw) register accessor: Command queueing interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqintstsena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqintstsena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqintstsena`]
module"]
#[doc(alias = "CQINTSTSENA")]
pub type Cqintstsena = crate::Reg<cqintstsena::CqintstsenaSpec>;
#[doc = "Command queueing interrupt status enable register"]
pub mod cqintstsena;
#[doc = "CQINTSIGENA (rw) register accessor: Command queueing interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqintsigena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqintsigena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqintsigena`]
module"]
#[doc(alias = "CQINTSIGENA")]
pub type Cqintsigena = crate::Reg<cqintsigena::CqintsigenaSpec>;
#[doc = "Command queueing interrupt signal enable register"]
pub mod cqintsigena;
#[doc = "CQINTCOAL (rw) register accessor: Command queueing interrupt coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqintcoal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqintcoal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqintcoal`]
module"]
#[doc(alias = "CQINTCOAL")]
pub type Cqintcoal = crate::Reg<cqintcoal::CqintcoalSpec>;
#[doc = "Command queueing interrupt coalescing register"]
pub mod cqintcoal;
#[doc = "CQTDLBA (rw) register accessor: Command queueing task descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdlba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdlba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdlba`]
module"]
#[doc(alias = "CQTDLBA")]
pub type Cqtdlba = crate::Reg<cqtdlba::CqtdlbaSpec>;
#[doc = "Command queueing task descriptor list base address register"]
pub mod cqtdlba;
#[doc = "CQTDLBAU (rw) register accessor: Command queueing task descriptor list base address upper 32bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdlbau::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdlbau::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdlbau`]
module"]
#[doc(alias = "CQTDLBAU")]
pub type Cqtdlbau = crate::Reg<cqtdlbau::CqtdlbauSpec>;
#[doc = "Command queueing task descriptor list base address upper 32bits register"]
pub mod cqtdlbau;
#[doc = "CQTDB (rw) register accessor: Command queueing task doorbell register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdb`]
module"]
#[doc(alias = "CQTDB")]
pub type Cqtdb = crate::Reg<cqtdb::CqtdbSpec>;
#[doc = "Command queueing task doorbell register"]
pub mod cqtdb;
#[doc = "CQTDBN (rw) register accessor: Command queueing task doorbell notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdbn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdbn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdbn`]
module"]
#[doc(alias = "CQTDBN")]
pub type Cqtdbn = crate::Reg<cqtdbn::CqtdbnSpec>;
#[doc = "Command queueing task doorbell notification register"]
pub mod cqtdbn;
#[doc = "CQDQSTS (r) register accessor: Command queueing device queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdqsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdqsts`]
module"]
#[doc(alias = "CQDQSTS")]
pub type Cqdqsts = crate::Reg<cqdqsts::CqdqstsSpec>;
#[doc = "Command queueing device queue status register"]
pub mod cqdqsts;
#[doc = "CQDPT (r) register accessor: Command queueing device pending tasks register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdpt`]
module"]
#[doc(alias = "CQDPT")]
pub type Cqdpt = crate::Reg<cqdpt::CqdptSpec>;
#[doc = "Command queueing device pending tasks register"]
pub mod cqdpt;
#[doc = "CQTCLR (rw) register accessor: Command queueing task clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtclr`]
module"]
#[doc(alias = "CQTCLR")]
pub type Cqtclr = crate::Reg<cqtclr::CqtclrSpec>;
#[doc = "Command queueing task clear register"]
pub mod cqtclr;
#[doc = "CQSSC1 (rw) register accessor: Command queueing send status configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc1`]
module"]
#[doc(alias = "CQSSC1")]
pub type Cqssc1 = crate::Reg<cqssc1::Cqssc1Spec>;
#[doc = "Command queueing send status configuration register 1"]
pub mod cqssc1;
#[doc = "CQSSC2 (rw) register accessor: Command queueing send status configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc2`]
module"]
#[doc(alias = "CQSSC2")]
pub type Cqssc2 = crate::Reg<cqssc2::Cqssc2Spec>;
#[doc = "Command queueing send status configuration register 2"]
pub mod cqssc2;
#[doc = "CQCRDT (r) register accessor: Command queueing command response for direct-command task register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcrdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcrdt`]
module"]
#[doc(alias = "CQCRDT")]
pub type Cqcrdt = crate::Reg<cqcrdt::CqcrdtSpec>;
#[doc = "Command queueing command response for direct-command task register"]
pub mod cqcrdt;
#[doc = "CQRMEM (r) register accessor: Command queueing response mode error mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqrmem::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqrmem`]
module"]
#[doc(alias = "CQRMEM")]
pub type Cqrmem = crate::Reg<cqrmem::CqrmemSpec>;
#[doc = "Command queueing response mode error mask register"]
pub mod cqrmem;
#[doc = "CQTEI (r) register accessor: Command queueing task error information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtei::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtei`]
module"]
#[doc(alias = "CQTEI")]
pub type Cqtei = crate::Reg<cqtei::CqteiSpec>;
#[doc = "Command queueing task error information register"]
pub mod cqtei;
#[doc = "CQCRI (r) register accessor: Command queueing command response index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcri::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcri`]
module"]
#[doc(alias = "CQCRI")]
pub type Cqcri = crate::Reg<cqcri::CqcriSpec>;
#[doc = "Command queueing command response index register"]
pub mod cqcri;
#[doc = "CQCRA (r) register accessor: Command queueing command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcra::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcra`]
module"]
#[doc(alias = "CQCRA")]
pub type Cqcra = crate::Reg<cqcra::CqcraSpec>;
#[doc = "Command queueing command response argument register"]
pub mod cqcra;
