#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlr0: Ctrlr0,
    ctrlr1: Ctrlr1,
    enr: Enr,
    ser: Ser,
    baudr: Baudr,
    txftlr: Txftlr,
    rxftlr: Rxftlr,
    txflr: Txflr,
    rxflr: Rxflr,
    sr: Sr,
    ipr: Ipr,
    imr: Imr,
    isr: Isr,
    risr: Risr,
    icr: Icr,
    dmacr: Dmacr,
    dmatdlr: Dmatdlr,
    dmardlr: Dmardlr,
    _reserved18: [u8; 0x03b8],
    txdr: Txdr,
    _reserved19: [u8; 0x03fc],
    rxdr: Rxdr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    #[inline(always)]
    pub const fn ctrlr0(&self) -> &Ctrlr0 {
        &self.ctrlr0
    }
    #[doc = "0x04 - Control Register 1"]
    #[inline(always)]
    pub const fn ctrlr1(&self) -> &Ctrlr1 {
        &self.ctrlr1
    }
    #[doc = "0x08 - SPI Enable"]
    #[inline(always)]
    pub const fn enr(&self) -> &Enr {
        &self.enr
    }
    #[doc = "0x0c - Slave Enable Register"]
    #[inline(always)]
    pub const fn ser(&self) -> &Ser {
        &self.ser
    }
    #[doc = "0x10 - Baud Rate Select"]
    #[inline(always)]
    pub const fn baudr(&self) -> &Baudr {
        &self.baudr
    }
    #[doc = "0x14 - Transmit FIFO Threshold Level"]
    #[inline(always)]
    pub const fn txftlr(&self) -> &Txftlr {
        &self.txftlr
    }
    #[doc = "0x18 - Receive FIFO Threshold Level"]
    #[inline(always)]
    pub const fn rxftlr(&self) -> &Rxftlr {
        &self.rxftlr
    }
    #[doc = "0x1c - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn txflr(&self) -> &Txflr {
        &self.txflr
    }
    #[doc = "0x20 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &Rxflr {
        &self.rxflr
    }
    #[doc = "0x24 - SPI Status"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x28 - Interrupt Polarity"]
    #[inline(always)]
    pub const fn ipr(&self) -> &Ipr {
        &self.ipr
    }
    #[doc = "0x2c - Interrupt Mask"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Interrupt Status"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x34 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn risr(&self) -> &Risr {
        &self.risr
    }
    #[doc = "0x38 - Interrupt Clear"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x3c - DMA Control"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x40 - DMA Transmit Data Level"]
    #[inline(always)]
    pub const fn dmatdlr(&self) -> &Dmatdlr {
        &self.dmatdlr
    }
    #[doc = "0x44 - DMA Receive Data Level"]
    #[inline(always)]
    pub const fn dmardlr(&self) -> &Dmardlr {
        &self.dmardlr
    }
    #[doc = "0x400 - Transmit FIFO Data"]
    #[inline(always)]
    pub const fn txdr(&self) -> &Txdr {
        &self.txdr
    }
    #[doc = "0x800 - Receive FIFO Data"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &Rxdr {
        &self.rxdr
    }
}
#[doc = "CTRLR0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlr0`]
module"]
#[doc(alias = "CTRLR0")]
pub type Ctrlr0 = crate::Reg<ctrlr0::Ctrlr0Spec>;
#[doc = "Control Register 0"]
pub mod ctrlr0;
#[doc = "CTRLR1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlr1`]
module"]
#[doc(alias = "CTRLR1")]
pub type Ctrlr1 = crate::Reg<ctrlr1::Ctrlr1Spec>;
#[doc = "Control Register 1"]
pub mod ctrlr1;
#[doc = "ENR (rw) register accessor: SPI Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr`]
module"]
#[doc(alias = "ENR")]
pub type Enr = crate::Reg<enr::EnrSpec>;
#[doc = "SPI Enable"]
pub mod enr;
#[doc = "SER (rw) register accessor: Slave Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`]
module"]
#[doc(alias = "SER")]
pub type Ser = crate::Reg<ser::SerSpec>;
#[doc = "Slave Enable Register"]
pub mod ser;
#[doc = "BAUDR (rw) register accessor: Baud Rate Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudr`]
module"]
#[doc(alias = "BAUDR")]
pub type Baudr = crate::Reg<baudr::BaudrSpec>;
#[doc = "Baud Rate Select"]
pub mod baudr;
#[doc = "TXFTLR (rw) register accessor: Transmit FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txftlr`]
module"]
#[doc(alias = "TXFTLR")]
pub type Txftlr = crate::Reg<txftlr::TxftlrSpec>;
#[doc = "Transmit FIFO Threshold Level"]
pub mod txftlr;
#[doc = "RXFTLR (rw) register accessor: Receive FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxftlr`]
module"]
#[doc(alias = "RXFTLR")]
pub type Rxftlr = crate::Reg<rxftlr::RxftlrSpec>;
#[doc = "Receive FIFO Threshold Level"]
pub mod rxftlr;
#[doc = "TXFLR (r) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txflr`]
module"]
#[doc(alias = "TXFLR")]
pub type Txflr = crate::Reg<txflr::TxflrSpec>;
#[doc = "Transmit FIFO Level"]
pub mod txflr;
#[doc = "RXFLR (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxflr`]
module"]
#[doc(alias = "RXFLR")]
pub type Rxflr = crate::Reg<rxflr::RxflrSpec>;
#[doc = "Receive FIFO Level"]
pub mod rxflr;
#[doc = "SR (r) register accessor: SPI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SPI Status"]
pub mod sr;
#[doc = "IPR (rw) register accessor: Interrupt Polarity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr`]
module"]
#[doc(alias = "IPR")]
pub type Ipr = crate::Reg<ipr::IprSpec>;
#[doc = "Interrupt Polarity"]
pub mod ipr;
#[doc = "IMR (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status"]
pub mod isr;
#[doc = "RISR (r) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`risr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@risr`]
module"]
#[doc(alias = "RISR")]
pub type Risr = crate::Reg<risr::RisrSpec>;
#[doc = "Raw Interrupt Status"]
pub mod risr;
#[doc = "ICR (w) register accessor: Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Control"]
pub mod dmacr;
#[doc = "DMATDLR (rw) register accessor: DMA Transmit Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdlr`]
module"]
#[doc(alias = "DMATDLR")]
pub type Dmatdlr = crate::Reg<dmatdlr::DmatdlrSpec>;
#[doc = "DMA Transmit Data Level"]
pub mod dmatdlr;
#[doc = "DMARDLR (rw) register accessor: DMA Receive Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmardlr`]
module"]
#[doc(alias = "DMARDLR")]
pub type Dmardlr = crate::Reg<dmardlr::DmardlrSpec>;
#[doc = "DMA Receive Data Level"]
pub mod dmardlr;
#[doc = "TXDR (w) register accessor: Transmit FIFO Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
#[doc(alias = "TXDR")]
pub type Txdr = crate::Reg<txdr::TxdrSpec>;
#[doc = "Transmit FIFO Data"]
pub mod txdr;
#[doc = "RXDR (rw) register accessor: Receive FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
#[doc(alias = "RXDR")]
pub type Rxdr = crate::Reg<rxdr::RxdrSpec>;
#[doc = "Receive FIFO Data"]
pub mod rxdr;
