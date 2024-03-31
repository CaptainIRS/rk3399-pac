#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
    _reserved8: [u8; 0x10],
    srbr: Srbr,
    _reserved9: [u8; 0x38],
    sthr: Sthr,
    far: Far,
    tfr: Tfr,
    rfw: Rfw,
    usr: Usr,
    tfl: Tfl,
    rfl: Rfl,
    srr: Srr,
    srts: Srts,
    sbcr: Sbcr,
    sdmam: Sdmam,
    sfe: Sfe,
    srt: Srt,
    stet: Stet,
    htx: Htx,
    dmasa: Dmasa,
    _reserved25: [u8; 0x48],
    cpr: Cpr,
    ucv: Ucv,
    ctr: Ctr,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch (Low)"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch (High)"]
    #[inline(always)]
    pub const fn dlh(&self) -> &Dlh {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratchpad Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x30 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub const fn srbr(&self) -> &Srbr {
        &self.srbr
    }
    #[doc = "0x6c - Shadow Transmit Holding Register"]
    #[inline(always)]
    pub const fn sthr(&self) -> &Sthr {
        &self.sthr
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn far(&self) -> &Far {
        &self.far
    }
    #[doc = "0x74 - Transmit FIFO Read"]
    #[inline(always)]
    pub const fn tfr(&self) -> &Tfr {
        &self.tfr
    }
    #[doc = "0x78 - Receive FIFO Write"]
    #[inline(always)]
    pub const fn rfw(&self) -> &Rfw {
        &self.rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &Usr {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn tfl(&self) -> &Tfl {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn rfl(&self) -> &Rfl {
        &self.rfl
    }
    #[doc = "0x88 - Software Reset Register"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x8c - Shadow Request to Send"]
    #[inline(always)]
    pub const fn srts(&self) -> &Srts {
        &self.srts
    }
    #[doc = "0x90 - Shadow Break Control Register"]
    #[inline(always)]
    pub const fn sbcr(&self) -> &Sbcr {
        &self.sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode"]
    #[inline(always)]
    pub const fn sdmam(&self) -> &Sdmam {
        &self.sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable"]
    #[inline(always)]
    pub const fn sfe(&self) -> &Sfe {
        &self.sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger"]
    #[inline(always)]
    pub const fn srt(&self) -> &Srt {
        &self.srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger"]
    #[inline(always)]
    pub const fn stet(&self) -> &Stet {
        &self.stet
    }
    #[doc = "0xa4 - Halt TX"]
    #[inline(always)]
    pub const fn htx(&self) -> &Htx {
        &self.htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
    #[doc = "0xf4 - Component Parameter Register"]
    #[inline(always)]
    pub const fn cpr(&self) -> &Cpr {
        &self.cpr
    }
    #[doc = "0xf8 - UART Component Version"]
    #[inline(always)]
    pub const fn ucv(&self) -> &Ucv {
        &self.ucv
    }
    #[doc = "0xfc - Component Type Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
}
#[doc = "RBR (rw) register accessor: Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`]
module"]
#[doc(alias = "RBR")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "Receive Buffer Register"]
pub mod rbr;
#[doc = "THR (rw) register accessor: Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "DLL (rw) register accessor: Divisor Latch (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch (Low)"]
pub mod dll;
#[doc = "DLH (rw) register accessor: Divisor Latch (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`]
module"]
#[doc(alias = "DLH")]
pub type Dlh = crate::Reg<dlh::DlhSpec>;
#[doc = "Divisor Latch (High)"]
pub mod dlh;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IIR (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identification Register"]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "SCR (rw) register accessor: Scratchpad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratchpad Register"]
pub mod scr;
#[doc = "SRBR (r) register accessor: Shadow Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr`]
module"]
#[doc(alias = "SRBR")]
pub type Srbr = crate::Reg<srbr::SrbrSpec>;
#[doc = "Shadow Receive Buffer Register"]
pub mod srbr;
#[doc = "STHR (r) register accessor: Shadow Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr`]
module"]
#[doc(alias = "STHR")]
pub type Sthr = crate::Reg<sthr::SthrSpec>;
#[doc = "Shadow Transmit Holding Register"]
pub mod sthr;
#[doc = "FAR (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`]
module"]
#[doc(alias = "FAR")]
pub type Far = crate::Reg<far::FarSpec>;
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "TFR (r) register accessor: Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr`]
module"]
#[doc(alias = "TFR")]
pub type Tfr = crate::Reg<tfr::TfrSpec>;
#[doc = "Transmit FIFO Read"]
pub mod tfr;
#[doc = "RFW (w) register accessor: Receive FIFO Write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfw`]
module"]
#[doc(alias = "RFW")]
pub type Rfw = crate::Reg<rfw::RfwSpec>;
#[doc = "Receive FIFO Write"]
pub mod rfw;
#[doc = "USR (r) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`]
module"]
#[doc(alias = "USR")]
pub type Usr = crate::Reg<usr::UsrSpec>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "TFL (rw) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`]
module"]
#[doc(alias = "TFL")]
pub type Tfl = crate::Reg<tfl::TflSpec>;
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "RFL (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`]
module"]
#[doc(alias = "RFL")]
pub type Rfl = crate::Reg<rfl::RflSpec>;
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "SRR (w) register accessor: Software Reset Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
#[doc(alias = "SRR")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Software Reset Register"]
pub mod srr;
#[doc = "SRTS (rw) register accessor: Shadow Request to Send\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srts`]
module"]
#[doc(alias = "SRTS")]
pub type Srts = crate::Reg<srts::SrtsSpec>;
#[doc = "Shadow Request to Send"]
pub mod srts;
#[doc = "SBCR (rw) register accessor: Shadow Break Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcr`]
module"]
#[doc(alias = "SBCR")]
pub type Sbcr = crate::Reg<sbcr::SbcrSpec>;
#[doc = "Shadow Break Control Register"]
pub mod sbcr;
#[doc = "SDMAM (rw) register accessor: Shadow DMA Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmam`]
module"]
#[doc(alias = "SDMAM")]
pub type Sdmam = crate::Reg<sdmam::SdmamSpec>;
#[doc = "Shadow DMA Mode"]
pub mod sdmam;
#[doc = "SFE (rw) register accessor: Shadow FIFO Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfe`]
module"]
#[doc(alias = "SFE")]
pub type Sfe = crate::Reg<sfe::SfeSpec>;
#[doc = "Shadow FIFO Enable"]
pub mod sfe;
#[doc = "SRT (rw) register accessor: Shadow RCVR Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srt`]
module"]
#[doc(alias = "SRT")]
pub type Srt = crate::Reg<srt::SrtSpec>;
#[doc = "Shadow RCVR Trigger"]
pub mod srt;
#[doc = "STET (rw) register accessor: Shadow TX Empty Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stet`]
module"]
#[doc(alias = "STET")]
pub type Stet = crate::Reg<stet::StetSpec>;
#[doc = "Shadow TX Empty Trigger"]
pub mod stet;
#[doc = "HTX (rw) register accessor: Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`]
module"]
#[doc(alias = "HTX")]
pub type Htx = crate::Reg<htx::HtxSpec>;
#[doc = "Halt TX"]
pub mod htx;
#[doc = "DMASA (w) register accessor: DMA Software Acknowledge\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`]
module"]
#[doc(alias = "DMASA")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Software Acknowledge"]
pub mod dmasa;
#[doc = "CPR (r) register accessor: Component Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpr`]
module"]
#[doc(alias = "CPR")]
pub type Cpr = crate::Reg<cpr::CprSpec>;
#[doc = "Component Parameter Register"]
pub mod cpr;
#[doc = "UCV (r) register accessor: UART Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucv`]
module"]
#[doc(alias = "UCV")]
pub type Ucv = crate::Reg<ucv::UcvSpec>;
#[doc = "UART Component Version"]
pub mod ucv;
#[doc = "CTR (r) register accessor: Component Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Component Type Register"]
pub mod ctr;
