#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_ctrlr0: SpiCtrlr0,
    spi_ctrlr1: SpiCtrlr1,
    spi_enr: SpiEnr,
    spi_ser: SpiSer,
    spi_baudr: SpiBaudr,
    spi_txftlr: SpiTxftlr,
    spi_rxftlr: SpiRxftlr,
    spi_txflr: SpiTxflr,
    spi_rxflr: SpiRxflr,
    spi_sr: SpiSr,
    spi_ipr: SpiIpr,
    spi_imr: SpiImr,
    spi_isr: SpiIsr,
    spi_risr: SpiRisr,
    spi_icr: SpiIcr,
    spi_dmacr: SpiDmacr,
    spi_dmatdlr: SpiDmatdlr,
    spi_dmardlr: SpiDmardlr,
    _reserved18: [u8; 0x03b8],
    spi_txdr: SpiTxdr,
    _reserved19: [u8; 0x03fc],
    spi_rxdr: SpiRxdr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    #[inline(always)]
    pub const fn spi_ctrlr0(&self) -> &SpiCtrlr0 {
        &self.spi_ctrlr0
    }
    #[doc = "0x04 - Control Register 1"]
    #[inline(always)]
    pub const fn spi_ctrlr1(&self) -> &SpiCtrlr1 {
        &self.spi_ctrlr1
    }
    #[doc = "0x08 - SPI Enable"]
    #[inline(always)]
    pub const fn spi_enr(&self) -> &SpiEnr {
        &self.spi_enr
    }
    #[doc = "0x0c - Slave Enable Register"]
    #[inline(always)]
    pub const fn spi_ser(&self) -> &SpiSer {
        &self.spi_ser
    }
    #[doc = "0x10 - Baud Rate Select"]
    #[inline(always)]
    pub const fn spi_baudr(&self) -> &SpiBaudr {
        &self.spi_baudr
    }
    #[doc = "0x14 - Transmit FIFO Threshold Level"]
    #[inline(always)]
    pub const fn spi_txftlr(&self) -> &SpiTxftlr {
        &self.spi_txftlr
    }
    #[doc = "0x18 - Receive FIFO Threshold Level"]
    #[inline(always)]
    pub const fn spi_rxftlr(&self) -> &SpiRxftlr {
        &self.spi_rxftlr
    }
    #[doc = "0x1c - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn spi_txflr(&self) -> &SpiTxflr {
        &self.spi_txflr
    }
    #[doc = "0x20 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn spi_rxflr(&self) -> &SpiRxflr {
        &self.spi_rxflr
    }
    #[doc = "0x24 - SPI Status"]
    #[inline(always)]
    pub const fn spi_sr(&self) -> &SpiSr {
        &self.spi_sr
    }
    #[doc = "0x28 - Interrupt Polarity"]
    #[inline(always)]
    pub const fn spi_ipr(&self) -> &SpiIpr {
        &self.spi_ipr
    }
    #[doc = "0x2c - Interrupt Mask"]
    #[inline(always)]
    pub const fn spi_imr(&self) -> &SpiImr {
        &self.spi_imr
    }
    #[doc = "0x30 - Interrupt Status"]
    #[inline(always)]
    pub const fn spi_isr(&self) -> &SpiIsr {
        &self.spi_isr
    }
    #[doc = "0x34 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn spi_risr(&self) -> &SpiRisr {
        &self.spi_risr
    }
    #[doc = "0x38 - Interrupt Clear"]
    #[inline(always)]
    pub const fn spi_icr(&self) -> &SpiIcr {
        &self.spi_icr
    }
    #[doc = "0x3c - DMA Control"]
    #[inline(always)]
    pub const fn spi_dmacr(&self) -> &SpiDmacr {
        &self.spi_dmacr
    }
    #[doc = "0x40 - DMA Transmit Data Level"]
    #[inline(always)]
    pub const fn spi_dmatdlr(&self) -> &SpiDmatdlr {
        &self.spi_dmatdlr
    }
    #[doc = "0x44 - DMA Receive Data Level"]
    #[inline(always)]
    pub const fn spi_dmardlr(&self) -> &SpiDmardlr {
        &self.spi_dmardlr
    }
    #[doc = "0x400 - Transmit FIFO Data"]
    #[inline(always)]
    pub const fn spi_txdr(&self) -> &SpiTxdr {
        &self.spi_txdr
    }
    #[doc = "0x800 - Receive FIFO Data"]
    #[inline(always)]
    pub const fn spi_rxdr(&self) -> &SpiRxdr {
        &self.spi_rxdr
    }
}
#[doc = "SPI_CTRLR0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrlr0`]
module"]
#[doc(alias = "SPI_CTRLR0")]
pub type SpiCtrlr0 = crate::Reg<spi_ctrlr0::SpiCtrlr0Spec>;
#[doc = "Control Register 0"]
pub mod spi_ctrlr0;
#[doc = "SPI_CTRLR1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrlr1`]
module"]
#[doc(alias = "SPI_CTRLR1")]
pub type SpiCtrlr1 = crate::Reg<spi_ctrlr1::SpiCtrlr1Spec>;
#[doc = "Control Register 1"]
pub mod spi_ctrlr1;
#[doc = "SPI_ENR (rw) register accessor: SPI Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_enr`]
module"]
#[doc(alias = "SPI_ENR")]
pub type SpiEnr = crate::Reg<spi_enr::SpiEnrSpec>;
#[doc = "SPI Enable"]
pub mod spi_enr;
#[doc = "SPI_SER (rw) register accessor: Slave Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ser`]
module"]
#[doc(alias = "SPI_SER")]
pub type SpiSer = crate::Reg<spi_ser::SpiSerSpec>;
#[doc = "Slave Enable Register"]
pub mod spi_ser;
#[doc = "SPI_BAUDR (rw) register accessor: Baud Rate Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_baudr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_baudr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_baudr`]
module"]
#[doc(alias = "SPI_BAUDR")]
pub type SpiBaudr = crate::Reg<spi_baudr::SpiBaudrSpec>;
#[doc = "Baud Rate Select"]
pub mod spi_baudr;
#[doc = "SPI_TXFTLR (rw) register accessor: Transmit FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txftlr`]
module"]
#[doc(alias = "SPI_TXFTLR")]
pub type SpiTxftlr = crate::Reg<spi_txftlr::SpiTxftlrSpec>;
#[doc = "Transmit FIFO Threshold Level"]
pub mod spi_txftlr;
#[doc = "SPI_RXFTLR (rw) register accessor: Receive FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxftlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxftlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxftlr`]
module"]
#[doc(alias = "SPI_RXFTLR")]
pub type SpiRxftlr = crate::Reg<spi_rxftlr::SpiRxftlrSpec>;
#[doc = "Receive FIFO Threshold Level"]
pub mod spi_rxftlr;
#[doc = "SPI_TXFLR (r) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txflr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txflr`]
module"]
#[doc(alias = "SPI_TXFLR")]
pub type SpiTxflr = crate::Reg<spi_txflr::SpiTxflrSpec>;
#[doc = "Transmit FIFO Level"]
pub mod spi_txflr;
#[doc = "SPI_RXFLR (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxflr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxflr`]
module"]
#[doc(alias = "SPI_RXFLR")]
pub type SpiRxflr = crate::Reg<spi_rxflr::SpiRxflrSpec>;
#[doc = "Receive FIFO Level"]
pub mod spi_rxflr;
#[doc = "SPI_SR (r) register accessor: SPI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sr`]
module"]
#[doc(alias = "SPI_SR")]
pub type SpiSr = crate::Reg<spi_sr::SpiSrSpec>;
#[doc = "SPI Status"]
pub mod spi_sr;
#[doc = "SPI_IPR (rw) register accessor: Interrupt Polarity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ipr`]
module"]
#[doc(alias = "SPI_IPR")]
pub type SpiIpr = crate::Reg<spi_ipr::SpiIprSpec>;
#[doc = "Interrupt Polarity"]
pub mod spi_ipr;
#[doc = "SPI_IMR (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_imr`]
module"]
#[doc(alias = "SPI_IMR")]
pub type SpiImr = crate::Reg<spi_imr::SpiImrSpec>;
#[doc = "Interrupt Mask"]
pub mod spi_imr;
#[doc = "SPI_ISR (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_isr`]
module"]
#[doc(alias = "SPI_ISR")]
pub type SpiIsr = crate::Reg<spi_isr::SpiIsrSpec>;
#[doc = "Interrupt Status"]
pub mod spi_isr;
#[doc = "SPI_RISR (r) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_risr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_risr`]
module"]
#[doc(alias = "SPI_RISR")]
pub type SpiRisr = crate::Reg<spi_risr::SpiRisrSpec>;
#[doc = "Raw Interrupt Status"]
pub mod spi_risr;
#[doc = "SPI_ICR (w) register accessor: Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_icr`]
module"]
#[doc(alias = "SPI_ICR")]
pub type SpiIcr = crate::Reg<spi_icr::SpiIcrSpec>;
#[doc = "Interrupt Clear"]
pub mod spi_icr;
#[doc = "SPI_DMACR (rw) register accessor: DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dmacr`]
module"]
#[doc(alias = "SPI_DMACR")]
pub type SpiDmacr = crate::Reg<spi_dmacr::SpiDmacrSpec>;
#[doc = "DMA Control"]
pub mod spi_dmacr;
#[doc = "SPI_DMATDLR (rw) register accessor: DMA Transmit Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dmatdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dmatdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dmatdlr`]
module"]
#[doc(alias = "SPI_DMATDLR")]
pub type SpiDmatdlr = crate::Reg<spi_dmatdlr::SpiDmatdlrSpec>;
#[doc = "DMA Transmit Data Level"]
pub mod spi_dmatdlr;
#[doc = "SPI_DMARDLR (rw) register accessor: DMA Receive Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dmardlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dmardlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dmardlr`]
module"]
#[doc(alias = "SPI_DMARDLR")]
pub type SpiDmardlr = crate::Reg<spi_dmardlr::SpiDmardlrSpec>;
#[doc = "DMA Receive Data Level"]
pub mod spi_dmardlr;
#[doc = "SPI_TXDR (w) register accessor: Transmit FIFO Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txdr`]
module"]
#[doc(alias = "SPI_TXDR")]
pub type SpiTxdr = crate::Reg<spi_txdr::SpiTxdrSpec>;
#[doc = "Transmit FIFO Data"]
pub mod spi_txdr;
#[doc = "SPI_RXDR (rw) register accessor: Receive FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxdr`]
module"]
#[doc(alias = "SPI_RXDR")]
pub type SpiRxdr = crate::Reg<spi_rxdr::SpiRxdrSpec>;
#[doc = "Receive FIFO Data"]
pub mod spi_rxdr;
