#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2s_txcr: I2sTxcr,
    i2s_rxcr: I2sRxcr,
    i2s_ckr: I2sCkr,
    i2s_txfifolr: I2sTxfifolr,
    i2s_dmacr: I2sDmacr,
    i2s_intcr: I2sIntcr,
    i2s_intsr: I2sIntsr,
    i2s_xfer: I2sXfer,
    i2s_clr: I2sClr,
    i2s_txdr: I2sTxdr,
    i2s_rxdr: I2sRxdr,
    i2s_rxfifolr: I2sRxfifolr,
}
impl RegisterBlock {
    #[doc = "0x00 - transmit operation control register"]
    #[inline(always)]
    pub const fn i2s_txcr(&self) -> &I2sTxcr {
        &self.i2s_txcr
    }
    #[doc = "0x04 - receive operation control register"]
    #[inline(always)]
    pub const fn i2s_rxcr(&self) -> &I2sRxcr {
        &self.i2s_rxcr
    }
    #[doc = "0x08 - clock generation register"]
    #[inline(always)]
    pub const fn i2s_ckr(&self) -> &I2sCkr {
        &self.i2s_ckr
    }
    #[doc = "0x0c - TX FIFO level register"]
    #[inline(always)]
    pub const fn i2s_txfifolr(&self) -> &I2sTxfifolr {
        &self.i2s_txfifolr
    }
    #[doc = "0x10 - DMA control register"]
    #[inline(always)]
    pub const fn i2s_dmacr(&self) -> &I2sDmacr {
        &self.i2s_dmacr
    }
    #[doc = "0x14 - interrupt control register"]
    #[inline(always)]
    pub const fn i2s_intcr(&self) -> &I2sIntcr {
        &self.i2s_intcr
    }
    #[doc = "0x18 - interrupt status register"]
    #[inline(always)]
    pub const fn i2s_intsr(&self) -> &I2sIntsr {
        &self.i2s_intsr
    }
    #[doc = "0x1c - Transfer Start Register"]
    #[inline(always)]
    pub const fn i2s_xfer(&self) -> &I2sXfer {
        &self.i2s_xfer
    }
    #[doc = "0x20 - SCLK domain logic clear Register"]
    #[inline(always)]
    pub const fn i2s_clr(&self) -> &I2sClr {
        &self.i2s_clr
    }
    #[doc = "0x24 - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn i2s_txdr(&self) -> &I2sTxdr {
        &self.i2s_txdr
    }
    #[doc = "0x28 - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn i2s_rxdr(&self) -> &I2sRxdr {
        &self.i2s_rxdr
    }
    #[doc = "0x2c - RX FIFO level register"]
    #[inline(always)]
    pub const fn i2s_rxfifolr(&self) -> &I2sRxfifolr {
        &self.i2s_rxfifolr
    }
}
#[doc = "I2S_TXCR (rw) register accessor: transmit operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_txcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_txcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_txcr`]
module"]
#[doc(alias = "I2S_TXCR")]
pub type I2sTxcr = crate::Reg<i2s_txcr::I2sTxcrSpec>;
#[doc = "transmit operation control register"]
pub mod i2s_txcr;
#[doc = "I2S_RXCR (rw) register accessor: receive operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_rxcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rxcr`]
module"]
#[doc(alias = "I2S_RXCR")]
pub type I2sRxcr = crate::Reg<i2s_rxcr::I2sRxcrSpec>;
#[doc = "receive operation control register"]
pub mod i2s_rxcr;
#[doc = "I2S_CKR (rw) register accessor: clock generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_ckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_ckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_ckr`]
module"]
#[doc(alias = "I2S_CKR")]
pub type I2sCkr = crate::Reg<i2s_ckr::I2sCkrSpec>;
#[doc = "clock generation register"]
pub mod i2s_ckr;
#[doc = "I2S_TXFIFOLR (r) register accessor: TX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_txfifolr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_txfifolr`]
module"]
#[doc(alias = "I2S_TXFIFOLR")]
pub type I2sTxfifolr = crate::Reg<i2s_txfifolr::I2sTxfifolrSpec>;
#[doc = "TX FIFO level register"]
pub mod i2s_txfifolr;
#[doc = "I2S_DMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_dmacr`]
module"]
#[doc(alias = "I2S_DMACR")]
pub type I2sDmacr = crate::Reg<i2s_dmacr::I2sDmacrSpec>;
#[doc = "DMA control register"]
pub mod i2s_dmacr;
#[doc = "I2S_INTCR (rw) register accessor: interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_intcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_intcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_intcr`]
module"]
#[doc(alias = "I2S_INTCR")]
pub type I2sIntcr = crate::Reg<i2s_intcr::I2sIntcrSpec>;
#[doc = "interrupt control register"]
pub mod i2s_intcr;
#[doc = "I2S_INTSR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_intsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_intsr`]
module"]
#[doc(alias = "I2S_INTSR")]
pub type I2sIntsr = crate::Reg<i2s_intsr::I2sIntsrSpec>;
#[doc = "interrupt status register"]
pub mod i2s_intsr;
#[doc = "I2S_XFER (rw) register accessor: Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_xfer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_xfer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_xfer`]
module"]
#[doc(alias = "I2S_XFER")]
pub type I2sXfer = crate::Reg<i2s_xfer::I2sXferSpec>;
#[doc = "Transfer Start Register"]
pub mod i2s_xfer;
#[doc = "I2S_CLR (rw) register accessor: SCLK domain logic clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_clr`]
module"]
#[doc(alias = "I2S_CLR")]
pub type I2sClr = crate::Reg<i2s_clr::I2sClrSpec>;
#[doc = "SCLK domain logic clear Register"]
pub mod i2s_clr;
#[doc = "I2S_TXDR (w) register accessor: Transmit FIFO Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_txdr`]
module"]
#[doc(alias = "I2S_TXDR")]
pub type I2sTxdr = crate::Reg<i2s_txdr::I2sTxdrSpec>;
#[doc = "Transmit FIFO Data Register"]
pub mod i2s_txdr;
#[doc = "I2S_RXDR (r) register accessor: Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rxdr`]
module"]
#[doc(alias = "I2S_RXDR")]
pub type I2sRxdr = crate::Reg<i2s_rxdr::I2sRxdrSpec>;
#[doc = "Receive FIFO Data Register"]
pub mod i2s_rxdr;
#[doc = "I2S_RXFIFOLR (r) register accessor: RX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxfifolr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rxfifolr`]
module"]
#[doc(alias = "I2S_RXFIFOLR")]
pub type I2sRxfifolr = crate::Reg<i2s_rxfifolr::I2sRxfifolrSpec>;
#[doc = "RX FIFO level register"]
pub mod i2s_rxfifolr;
