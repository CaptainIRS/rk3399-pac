#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txcr: Txcr,
    rxcr: Rxcr,
    ckr: Ckr,
    txfifolr: Txfifolr,
    dmacr: Dmacr,
    intcr: Intcr,
    intsr: Intsr,
    xfer: Xfer,
    clr: Clr,
    txdr: Txdr,
    rxdr: Rxdr,
    rxfifolr: Rxfifolr,
}
impl RegisterBlock {
    #[doc = "0x00 - transmit operation control register"]
    #[inline(always)]
    pub const fn txcr(&self) -> &Txcr {
        &self.txcr
    }
    #[doc = "0x04 - receive operation control register"]
    #[inline(always)]
    pub const fn rxcr(&self) -> &Rxcr {
        &self.rxcr
    }
    #[doc = "0x08 - clock generation register"]
    #[inline(always)]
    pub const fn ckr(&self) -> &Ckr {
        &self.ckr
    }
    #[doc = "0x0c - TX FIFO level register"]
    #[inline(always)]
    pub const fn txfifolr(&self) -> &Txfifolr {
        &self.txfifolr
    }
    #[doc = "0x10 - DMA control register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x14 - interrupt control register"]
    #[inline(always)]
    pub const fn intcr(&self) -> &Intcr {
        &self.intcr
    }
    #[doc = "0x18 - interrupt status register"]
    #[inline(always)]
    pub const fn intsr(&self) -> &Intsr {
        &self.intsr
    }
    #[doc = "0x1c - Transfer Start Register"]
    #[inline(always)]
    pub const fn xfer(&self) -> &Xfer {
        &self.xfer
    }
    #[doc = "0x20 - SCLK domain logic clear Register"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x24 - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn txdr(&self) -> &Txdr {
        &self.txdr
    }
    #[doc = "0x28 - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &Rxdr {
        &self.rxdr
    }
    #[doc = "0x2c - RX FIFO level register"]
    #[inline(always)]
    pub const fn rxfifolr(&self) -> &Rxfifolr {
        &self.rxfifolr
    }
}
#[doc = "TXCR (rw) register accessor: transmit operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcr`]
module"]
#[doc(alias = "TXCR")]
pub type Txcr = crate::Reg<txcr::TxcrSpec>;
#[doc = "transmit operation control register"]
pub mod txcr;
#[doc = "RXCR (rw) register accessor: receive operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcr`]
module"]
#[doc(alias = "RXCR")]
pub type Rxcr = crate::Reg<rxcr::RxcrSpec>;
#[doc = "receive operation control register"]
pub mod rxcr;
#[doc = "CKR (rw) register accessor: clock generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckr`]
module"]
#[doc(alias = "CKR")]
pub type Ckr = crate::Reg<ckr::CkrSpec>;
#[doc = "clock generation register"]
pub mod ckr;
#[doc = "TXFIFOLR (r) register accessor: TX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifolr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifolr`]
module"]
#[doc(alias = "TXFIFOLR")]
pub type Txfifolr = crate::Reg<txfifolr::TxfifolrSpec>;
#[doc = "TX FIFO level register"]
pub mod txfifolr;
#[doc = "DMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "INTCR (rw) register accessor: interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcr`]
module"]
#[doc(alias = "INTCR")]
pub type Intcr = crate::Reg<intcr::IntcrSpec>;
#[doc = "interrupt control register"]
pub mod intcr;
#[doc = "INTSR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsr`]
module"]
#[doc(alias = "INTSR")]
pub type Intsr = crate::Reg<intsr::IntsrSpec>;
#[doc = "interrupt status register"]
pub mod intsr;
#[doc = "XFER (rw) register accessor: Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfer`]
module"]
#[doc(alias = "XFER")]
pub type Xfer = crate::Reg<xfer::XferSpec>;
#[doc = "Transfer Start Register"]
pub mod xfer;
#[doc = "CLR (rw) register accessor: SCLK domain logic clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "SCLK domain logic clear Register"]
pub mod clr;
#[doc = "TXDR (w) register accessor: Transmit FIFO Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
#[doc(alias = "TXDR")]
pub type Txdr = crate::Reg<txdr::TxdrSpec>;
#[doc = "Transmit FIFO Data Register"]
pub mod txdr;
#[doc = "RXDR (r) register accessor: Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
#[doc(alias = "RXDR")]
pub type Rxdr = crate::Reg<rxdr::RxdrSpec>;
#[doc = "Receive FIFO Data Register"]
pub mod rxdr;
#[doc = "RXFIFOLR (r) register accessor: RX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifolr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifolr`]
module"]
#[doc(alias = "RXFIFOLR")]
pub type Rxfifolr = crate::Reg<rxfifolr::RxfifolrSpec>;
#[doc = "RX FIFO level register"]
pub mod rxfifolr;
