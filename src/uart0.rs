#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uart: [u8; 0x04],
    _reserved_1_uart: [u8; 0x04],
    _reserved_2_uart: [u8; 0x04],
    uart_lcr: UartLcr,
    uart_mcr: UartMcr,
    uart_lsr: UartLsr,
    uart_msr: UartMsr,
    uart_scr: UartScr,
    _reserved8: [u8; 0x10],
    uart_srbr: UartSrbr,
    _reserved9: [u8; 0x38],
    uart_sthr: UartSthr,
    uart_far: UartFar,
    uart_tfr: UartTfr,
    uart_rfw: UartRfw,
    uart_usr: UartUsr,
    uart_tfl: UartTfl,
    uart_rfl: UartRfl,
    uart_srr: UartSrr,
    uart_srts: UartSrts,
    uart_sbcr: UartSbcr,
    uart_sdmam: UartSdmam,
    uart_sfe: UartSfe,
    uart_srt: UartSrt,
    uart_stet: UartStet,
    uart_htx: UartHtx,
    uart_dmasa: UartDmasa,
    _reserved25: [u8; 0x48],
    uart_cpr: UartCpr,
    uart_ucv: UartUcv,
    uart_ctr: UartCtr,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch (Low)"]
    #[inline(always)]
    pub const fn uart_dll(&self) -> &UartDll {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn uart_thr(&self) -> &UartThr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub const fn uart_rbr(&self) -> &UartRbr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uart_ier(&self) -> &UartIer {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch (High)"]
    #[inline(always)]
    pub const fn uart_dlh(&self) -> &UartDlh {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn uart_fcr(&self) -> &UartFcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn uart_iir(&self) -> &UartIir {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn uart_lcr(&self) -> &UartLcr {
        &self.uart_lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn uart_mcr(&self) -> &UartMcr {
        &self.uart_mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn uart_lsr(&self) -> &UartLsr {
        &self.uart_lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn uart_msr(&self) -> &UartMsr {
        &self.uart_msr
    }
    #[doc = "0x1c - Scratchpad Register"]
    #[inline(always)]
    pub const fn uart_scr(&self) -> &UartScr {
        &self.uart_scr
    }
    #[doc = "0x30 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub const fn uart_srbr(&self) -> &UartSrbr {
        &self.uart_srbr
    }
    #[doc = "0x6c - Shadow Transmit Holding Register"]
    #[inline(always)]
    pub const fn uart_sthr(&self) -> &UartSthr {
        &self.uart_sthr
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn uart_far(&self) -> &UartFar {
        &self.uart_far
    }
    #[doc = "0x74 - Transmit FIFO Read"]
    #[inline(always)]
    pub const fn uart_tfr(&self) -> &UartTfr {
        &self.uart_tfr
    }
    #[doc = "0x78 - Receive FIFO Write"]
    #[inline(always)]
    pub const fn uart_rfw(&self) -> &UartRfw {
        &self.uart_rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn uart_usr(&self) -> &UartUsr {
        &self.uart_usr
    }
    #[doc = "0x80 - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn uart_tfl(&self) -> &UartTfl {
        &self.uart_tfl
    }
    #[doc = "0x84 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn uart_rfl(&self) -> &UartRfl {
        &self.uart_rfl
    }
    #[doc = "0x88 - Software Reset Register"]
    #[inline(always)]
    pub const fn uart_srr(&self) -> &UartSrr {
        &self.uart_srr
    }
    #[doc = "0x8c - Shadow Request to Send"]
    #[inline(always)]
    pub const fn uart_srts(&self) -> &UartSrts {
        &self.uart_srts
    }
    #[doc = "0x90 - Shadow Break Control Register"]
    #[inline(always)]
    pub const fn uart_sbcr(&self) -> &UartSbcr {
        &self.uart_sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode"]
    #[inline(always)]
    pub const fn uart_sdmam(&self) -> &UartSdmam {
        &self.uart_sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable"]
    #[inline(always)]
    pub const fn uart_sfe(&self) -> &UartSfe {
        &self.uart_sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger"]
    #[inline(always)]
    pub const fn uart_srt(&self) -> &UartSrt {
        &self.uart_srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger"]
    #[inline(always)]
    pub const fn uart_stet(&self) -> &UartStet {
        &self.uart_stet
    }
    #[doc = "0xa4 - Halt TX"]
    #[inline(always)]
    pub const fn uart_htx(&self) -> &UartHtx {
        &self.uart_htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn uart_dmasa(&self) -> &UartDmasa {
        &self.uart_dmasa
    }
    #[doc = "0xf4 - Component Parameter Register"]
    #[inline(always)]
    pub const fn uart_cpr(&self) -> &UartCpr {
        &self.uart_cpr
    }
    #[doc = "0xf8 - UART Component Version"]
    #[inline(always)]
    pub const fn uart_ucv(&self) -> &UartUcv {
        &self.uart_ucv
    }
    #[doc = "0xfc - Component Type Register"]
    #[inline(always)]
    pub const fn uart_ctr(&self) -> &UartCtr {
        &self.uart_ctr
    }
}
#[doc = "UART_RBR (rw) register accessor: Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rbr`]
module"]
#[doc(alias = "UART_RBR")]
pub type UartRbr = crate::Reg<uart_rbr::UartRbrSpec>;
#[doc = "Receive Buffer Register"]
pub mod uart_rbr;
#[doc = "UART_THR (rw) register accessor: Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_thr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_thr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_thr`]
module"]
#[doc(alias = "UART_THR")]
pub type UartThr = crate::Reg<uart_thr::UartThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod uart_thr;
#[doc = "UART_DLL (rw) register accessor: Divisor Latch (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_dll`]
module"]
#[doc(alias = "UART_DLL")]
pub type UartDll = crate::Reg<uart_dll::UartDllSpec>;
#[doc = "Divisor Latch (Low)"]
pub mod uart_dll;
#[doc = "UART_DLH (rw) register accessor: Divisor Latch (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_dlh`]
module"]
#[doc(alias = "UART_DLH")]
pub type UartDlh = crate::Reg<uart_dlh::UartDlhSpec>;
#[doc = "Divisor Latch (High)"]
pub mod uart_dlh;
#[doc = "UART_IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ier`]
module"]
#[doc(alias = "UART_IER")]
pub type UartIer = crate::Reg<uart_ier::UartIerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod uart_ier;
#[doc = "UART_IIR (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_iir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_iir`]
module"]
#[doc(alias = "UART_IIR")]
pub type UartIir = crate::Reg<uart_iir::UartIirSpec>;
#[doc = "Interrupt Identification Register"]
pub mod uart_iir;
#[doc = "UART_FCR (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fcr`]
module"]
#[doc(alias = "UART_FCR")]
pub type UartFcr = crate::Reg<uart_fcr::UartFcrSpec>;
#[doc = "FIFO Control Register"]
pub mod uart_fcr;
#[doc = "UART_LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_lcr`]
module"]
#[doc(alias = "UART_LCR")]
pub type UartLcr = crate::Reg<uart_lcr::UartLcrSpec>;
#[doc = "Line Control Register"]
pub mod uart_lcr;
#[doc = "UART_MCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_mcr`]
module"]
#[doc(alias = "UART_MCR")]
pub type UartMcr = crate::Reg<uart_mcr::UartMcrSpec>;
#[doc = "Modem Control Register"]
pub mod uart_mcr;
#[doc = "UART_LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_lsr`]
module"]
#[doc(alias = "UART_LSR")]
pub type UartLsr = crate::Reg<uart_lsr::UartLsrSpec>;
#[doc = "Line Status Register"]
pub mod uart_lsr;
#[doc = "UART_MSR (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_msr`]
module"]
#[doc(alias = "UART_MSR")]
pub type UartMsr = crate::Reg<uart_msr::UartMsrSpec>;
#[doc = "Modem Status Register"]
pub mod uart_msr;
#[doc = "UART_SCR (rw) register accessor: Scratchpad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_scr`]
module"]
#[doc(alias = "UART_SCR")]
pub type UartScr = crate::Reg<uart_scr::UartScrSpec>;
#[doc = "Scratchpad Register"]
pub mod uart_scr;
#[doc = "UART_SRBR (r) register accessor: Shadow Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_srbr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_srbr`]
module"]
#[doc(alias = "UART_SRBR")]
pub type UartSrbr = crate::Reg<uart_srbr::UartSrbrSpec>;
#[doc = "Shadow Receive Buffer Register"]
pub mod uart_srbr;
#[doc = "UART_STHR (r) register accessor: Shadow Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sthr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sthr`]
module"]
#[doc(alias = "UART_STHR")]
pub type UartSthr = crate::Reg<uart_sthr::UartSthrSpec>;
#[doc = "Shadow Transmit Holding Register"]
pub mod uart_sthr;
#[doc = "UART_FAR (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_far`]
module"]
#[doc(alias = "UART_FAR")]
pub type UartFar = crate::Reg<uart_far::UartFarSpec>;
#[doc = "FIFO Access Register"]
pub mod uart_far;
#[doc = "UART_TFR (r) register accessor: Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tfr`]
module"]
#[doc(alias = "UART_TFR")]
pub type UartTfr = crate::Reg<uart_tfr::UartTfrSpec>;
#[doc = "Transmit FIFO Read"]
pub mod uart_tfr;
#[doc = "UART_RFW (w) register accessor: Receive FIFO Write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rfw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rfw`]
module"]
#[doc(alias = "UART_RFW")]
pub type UartRfw = crate::Reg<uart_rfw::UartRfwSpec>;
#[doc = "Receive FIFO Write"]
pub mod uart_rfw;
#[doc = "UART_USR (r) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_usr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_usr`]
module"]
#[doc(alias = "UART_USR")]
pub type UartUsr = crate::Reg<uart_usr::UartUsrSpec>;
#[doc = "UART Status Register"]
pub mod uart_usr;
#[doc = "UART_TFL (rw) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tfl`]
module"]
#[doc(alias = "UART_TFL")]
pub type UartTfl = crate::Reg<uart_tfl::UartTflSpec>;
#[doc = "Transmit FIFO Level"]
pub mod uart_tfl;
#[doc = "UART_RFL (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rfl`]
module"]
#[doc(alias = "UART_RFL")]
pub type UartRfl = crate::Reg<uart_rfl::UartRflSpec>;
#[doc = "Receive FIFO Level"]
pub mod uart_rfl;
#[doc = "UART_SRR (w) register accessor: Software Reset Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_srr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_srr`]
module"]
#[doc(alias = "UART_SRR")]
pub type UartSrr = crate::Reg<uart_srr::UartSrrSpec>;
#[doc = "Software Reset Register"]
pub mod uart_srr;
#[doc = "UART_SRTS (rw) register accessor: Shadow Request to Send\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_srts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_srts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_srts`]
module"]
#[doc(alias = "UART_SRTS")]
pub type UartSrts = crate::Reg<uart_srts::UartSrtsSpec>;
#[doc = "Shadow Request to Send"]
pub mod uart_srts;
#[doc = "UART_SBCR (rw) register accessor: Shadow Break Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_sbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sbcr`]
module"]
#[doc(alias = "UART_SBCR")]
pub type UartSbcr = crate::Reg<uart_sbcr::UartSbcrSpec>;
#[doc = "Shadow Break Control Register"]
pub mod uart_sbcr;
#[doc = "UART_SDMAM (rw) register accessor: Shadow DMA Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sdmam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_sdmam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sdmam`]
module"]
#[doc(alias = "UART_SDMAM")]
pub type UartSdmam = crate::Reg<uart_sdmam::UartSdmamSpec>;
#[doc = "Shadow DMA Mode"]
pub mod uart_sdmam;
#[doc = "UART_SFE (rw) register accessor: Shadow FIFO Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sfe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_sfe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sfe`]
module"]
#[doc(alias = "UART_SFE")]
pub type UartSfe = crate::Reg<uart_sfe::UartSfeSpec>;
#[doc = "Shadow FIFO Enable"]
pub mod uart_sfe;
#[doc = "UART_SRT (rw) register accessor: Shadow RCVR Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_srt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_srt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_srt`]
module"]
#[doc(alias = "UART_SRT")]
pub type UartSrt = crate::Reg<uart_srt::UartSrtSpec>;
#[doc = "Shadow RCVR Trigger"]
pub mod uart_srt;
#[doc = "UART_STET (rw) register accessor: Shadow TX Empty Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_stet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_stet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_stet`]
module"]
#[doc(alias = "UART_STET")]
pub type UartStet = crate::Reg<uart_stet::UartStetSpec>;
#[doc = "Shadow TX Empty Trigger"]
pub mod uart_stet;
#[doc = "UART_HTX (rw) register accessor: Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_htx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_htx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_htx`]
module"]
#[doc(alias = "UART_HTX")]
pub type UartHtx = crate::Reg<uart_htx::UartHtxSpec>;
#[doc = "Halt TX"]
pub mod uart_htx;
#[doc = "UART_DMASA (w) register accessor: DMA Software Acknowledge\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_dmasa::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_dmasa`]
module"]
#[doc(alias = "UART_DMASA")]
pub type UartDmasa = crate::Reg<uart_dmasa::UartDmasaSpec>;
#[doc = "DMA Software Acknowledge"]
pub mod uart_dmasa;
#[doc = "UART_CPR (r) register accessor: Component Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_cpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_cpr`]
module"]
#[doc(alias = "UART_CPR")]
pub type UartCpr = crate::Reg<uart_cpr::UartCprSpec>;
#[doc = "Component Parameter Register"]
pub mod uart_cpr;
#[doc = "UART_UCV (r) register accessor: UART Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ucv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ucv`]
module"]
#[doc(alias = "UART_UCV")]
pub type UartUcv = crate::Reg<uart_ucv::UartUcvSpec>;
#[doc = "UART Component Version"]
pub mod uart_ucv;
#[doc = "UART_CTR (r) register accessor: Component Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ctr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ctr`]
module"]
#[doc(alias = "UART_CTR")]
pub type UartCtr = crate::Reg<uart_ctr::UartCtrSpec>;
#[doc = "Component Type Register"]
pub mod uart_ctr;
