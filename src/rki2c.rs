#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: Con,
    clkdiv: Clkdiv,
    mrxaddr: Mrxaddr,
    mrxraddr: Mrxraddr,
    mtxcnt: Mtxcnt,
    mrxcnt: Mrxcnt,
    ien: Ien,
    ipd: Ipd,
    fcnt: Fcnt,
    scl_oe_db: SclOeDb,
    _reserved10: [u8; 0xd8],
    txdata0: Txdata0,
    txdata1: Txdata1,
    txdata2: Txdata2,
    txdata3: Txdata3,
    txdata4: Txdata4,
    txdata5: Txdata5,
    txdata6: Txdata6,
    txdata7: Txdata7,
    _reserved18: [u8; 0xe0],
    rxdata0: Rxdata0,
    rxdata1: Rxdata1,
    rxdata2: Rxdata2,
    rxdata3: Rxdata3,
    rxdata4: Rxdata4,
    rxdata5: Rxdata5,
    rxdata6: Rxdata6,
    rxdata7: Rxdata7,
    st: St,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x04 - clock divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x08 - the slave address accessed for master rx mode"]
    #[inline(always)]
    pub const fn mrxaddr(&self) -> &Mrxaddr {
        &self.mrxaddr
    }
    #[doc = "0x0c - the slave register address accessed for master rx mode"]
    #[inline(always)]
    pub const fn mrxraddr(&self) -> &Mrxraddr {
        &self.mrxraddr
    }
    #[doc = "0x10 - master transmit count"]
    #[inline(always)]
    pub const fn mtxcnt(&self) -> &Mtxcnt {
        &self.mtxcnt
    }
    #[doc = "0x14 - master rx count"]
    #[inline(always)]
    pub const fn mrxcnt(&self) -> &Mrxcnt {
        &self.mrxcnt
    }
    #[doc = "0x18 - interrupt enable register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x1c - interrupt pending register"]
    #[inline(always)]
    pub const fn ipd(&self) -> &Ipd {
        &self.ipd
    }
    #[doc = "0x20 - finished count"]
    #[inline(always)]
    pub const fn fcnt(&self) -> &Fcnt {
        &self.fcnt
    }
    #[doc = "0x24 - slave hold debounce configure register"]
    #[inline(always)]
    pub const fn scl_oe_db(&self) -> &SclOeDb {
        &self.scl_oe_db
    }
    #[doc = "0x100 - I2C tx data register 0"]
    #[inline(always)]
    pub const fn txdata0(&self) -> &Txdata0 {
        &self.txdata0
    }
    #[doc = "0x104 - I2C tx data register 1"]
    #[inline(always)]
    pub const fn txdata1(&self) -> &Txdata1 {
        &self.txdata1
    }
    #[doc = "0x108 - I2C tx data register 2"]
    #[inline(always)]
    pub const fn txdata2(&self) -> &Txdata2 {
        &self.txdata2
    }
    #[doc = "0x10c - I2C tx data register 3"]
    #[inline(always)]
    pub const fn txdata3(&self) -> &Txdata3 {
        &self.txdata3
    }
    #[doc = "0x110 - I2C tx data register 4"]
    #[inline(always)]
    pub const fn txdata4(&self) -> &Txdata4 {
        &self.txdata4
    }
    #[doc = "0x114 - I2C tx data register 5"]
    #[inline(always)]
    pub const fn txdata5(&self) -> &Txdata5 {
        &self.txdata5
    }
    #[doc = "0x118 - I2C tx data register 6"]
    #[inline(always)]
    pub const fn txdata6(&self) -> &Txdata6 {
        &self.txdata6
    }
    #[doc = "0x11c - I2C tx data register 7"]
    #[inline(always)]
    pub const fn txdata7(&self) -> &Txdata7 {
        &self.txdata7
    }
    #[doc = "0x200 - I2C rx data register 0"]
    #[inline(always)]
    pub const fn rxdata0(&self) -> &Rxdata0 {
        &self.rxdata0
    }
    #[doc = "0x204 - I2C rx data register 1"]
    #[inline(always)]
    pub const fn rxdata1(&self) -> &Rxdata1 {
        &self.rxdata1
    }
    #[doc = "0x208 - I2C rx data register 2"]
    #[inline(always)]
    pub const fn rxdata2(&self) -> &Rxdata2 {
        &self.rxdata2
    }
    #[doc = "0x20c - I2C rx data register 3"]
    #[inline(always)]
    pub const fn rxdata3(&self) -> &Rxdata3 {
        &self.rxdata3
    }
    #[doc = "0x210 - I2C rx data register 4"]
    #[inline(always)]
    pub const fn rxdata4(&self) -> &Rxdata4 {
        &self.rxdata4
    }
    #[doc = "0x214 - I2C rx data register 5"]
    #[inline(always)]
    pub const fn rxdata5(&self) -> &Rxdata5 {
        &self.rxdata5
    }
    #[doc = "0x218 - I2C rx data register 6"]
    #[inline(always)]
    pub const fn rxdata6(&self) -> &Rxdata6 {
        &self.rxdata6
    }
    #[doc = "0x21c - I2C rx data register 7"]
    #[inline(always)]
    pub const fn rxdata7(&self) -> &Rxdata7 {
        &self.rxdata7
    }
    #[doc = "0x220 - status debug register"]
    #[inline(always)]
    pub const fn st(&self) -> &St {
        &self.st
    }
}
#[doc = "CON (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "control register"]
#[path = "rki2c/con_.rs"]
pub mod con;
#[doc = "CLKDIV (rw) register accessor: clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "clock divider register"]
pub mod clkdiv;
#[doc = "MRXADDR (rw) register accessor: the slave address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrxaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrxaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrxaddr`]
module"]
#[doc(alias = "MRXADDR")]
pub type Mrxaddr = crate::Reg<mrxaddr::MrxaddrSpec>;
#[doc = "the slave address accessed for master rx mode"]
pub mod mrxaddr;
#[doc = "MRXRADDR (rw) register accessor: the slave register address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrxraddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrxraddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrxraddr`]
module"]
#[doc(alias = "MRXRADDR")]
pub type Mrxraddr = crate::Reg<mrxraddr::MrxraddrSpec>;
#[doc = "the slave register address accessed for master rx mode"]
pub mod mrxraddr;
#[doc = "MTXCNT (rw) register accessor: master transmit count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtxcnt`]
module"]
#[doc(alias = "MTXCNT")]
pub type Mtxcnt = crate::Reg<mtxcnt::MtxcntSpec>;
#[doc = "master transmit count"]
pub mod mtxcnt;
#[doc = "MRXCNT (rw) register accessor: master rx count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrxcnt`]
module"]
#[doc(alias = "MRXCNT")]
pub type Mrxcnt = crate::Reg<mrxcnt::MrxcntSpec>;
#[doc = "master rx count"]
pub mod mrxcnt;
#[doc = "IEN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "interrupt enable register"]
pub mod ien;
#[doc = "IPD (rw) register accessor: interrupt pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipd`]
module"]
#[doc(alias = "IPD")]
pub type Ipd = crate::Reg<ipd::IpdSpec>;
#[doc = "interrupt pending register"]
pub mod ipd;
#[doc = "FCNT (r) register accessor: finished count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcnt`]
module"]
#[doc(alias = "FCNT")]
pub type Fcnt = crate::Reg<fcnt::FcntSpec>;
#[doc = "finished count"]
pub mod fcnt;
#[doc = "SCL_OE_DB (rw) register accessor: slave hold debounce configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_oe_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_oe_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl_oe_db`]
module"]
#[doc(alias = "SCL_OE_DB")]
pub type SclOeDb = crate::Reg<scl_oe_db::SclOeDbSpec>;
#[doc = "slave hold debounce configure register"]
pub mod scl_oe_db;
#[doc = "TXDATA0 (rw) register accessor: I2C tx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata0`]
module"]
#[doc(alias = "TXDATA0")]
pub type Txdata0 = crate::Reg<txdata0::Txdata0Spec>;
#[doc = "I2C tx data register 0"]
pub mod txdata0;
#[doc = "TXDATA1 (rw) register accessor: I2C tx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata1`]
module"]
#[doc(alias = "TXDATA1")]
pub type Txdata1 = crate::Reg<txdata1::Txdata1Spec>;
#[doc = "I2C tx data register 1"]
pub mod txdata1;
#[doc = "TXDATA2 (rw) register accessor: I2C tx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata2`]
module"]
#[doc(alias = "TXDATA2")]
pub type Txdata2 = crate::Reg<txdata2::Txdata2Spec>;
#[doc = "I2C tx data register 2"]
pub mod txdata2;
#[doc = "TXDATA3 (rw) register accessor: I2C tx data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata3`]
module"]
#[doc(alias = "TXDATA3")]
pub type Txdata3 = crate::Reg<txdata3::Txdata3Spec>;
#[doc = "I2C tx data register 3"]
pub mod txdata3;
#[doc = "TXDATA4 (rw) register accessor: I2C tx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata4`]
module"]
#[doc(alias = "TXDATA4")]
pub type Txdata4 = crate::Reg<txdata4::Txdata4Spec>;
#[doc = "I2C tx data register 4"]
pub mod txdata4;
#[doc = "TXDATA5 (rw) register accessor: I2C tx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata5`]
module"]
#[doc(alias = "TXDATA5")]
pub type Txdata5 = crate::Reg<txdata5::Txdata5Spec>;
#[doc = "I2C tx data register 5"]
pub mod txdata5;
#[doc = "TXDATA6 (rw) register accessor: I2C tx data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata6`]
module"]
#[doc(alias = "TXDATA6")]
pub type Txdata6 = crate::Reg<txdata6::Txdata6Spec>;
#[doc = "I2C tx data register 6"]
pub mod txdata6;
#[doc = "TXDATA7 (rw) register accessor: I2C tx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata7`]
module"]
#[doc(alias = "TXDATA7")]
pub type Txdata7 = crate::Reg<txdata7::Txdata7Spec>;
#[doc = "I2C tx data register 7"]
pub mod txdata7;
#[doc = "RXDATA0 (r) register accessor: I2C rx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata0`]
module"]
#[doc(alias = "RXDATA0")]
pub type Rxdata0 = crate::Reg<rxdata0::Rxdata0Spec>;
#[doc = "I2C rx data register 0"]
pub mod rxdata0;
#[doc = "RXDATA1 (r) register accessor: I2C rx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata1`]
module"]
#[doc(alias = "RXDATA1")]
pub type Rxdata1 = crate::Reg<rxdata1::Rxdata1Spec>;
#[doc = "I2C rx data register 1"]
pub mod rxdata1;
#[doc = "RXDATA2 (r) register accessor: I2C rx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata2`]
module"]
#[doc(alias = "RXDATA2")]
pub type Rxdata2 = crate::Reg<rxdata2::Rxdata2Spec>;
#[doc = "I2C rx data register 2"]
pub mod rxdata2;
#[doc = "RXDATA3 (r) register accessor: I2C rx data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata3`]
module"]
#[doc(alias = "RXDATA3")]
pub type Rxdata3 = crate::Reg<rxdata3::Rxdata3Spec>;
#[doc = "I2C rx data register 3"]
pub mod rxdata3;
#[doc = "RXDATA4 (r) register accessor: I2C rx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata4`]
module"]
#[doc(alias = "RXDATA4")]
pub type Rxdata4 = crate::Reg<rxdata4::Rxdata4Spec>;
#[doc = "I2C rx data register 4"]
pub mod rxdata4;
#[doc = "RXDATA5 (r) register accessor: I2C rx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata5`]
module"]
#[doc(alias = "RXDATA5")]
pub type Rxdata5 = crate::Reg<rxdata5::Rxdata5Spec>;
#[doc = "I2C rx data register 5"]
pub mod rxdata5;
#[doc = "RXDATA6 (r) register accessor: I2C rx data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata6`]
module"]
#[doc(alias = "RXDATA6")]
pub type Rxdata6 = crate::Reg<rxdata6::Rxdata6Spec>;
#[doc = "I2C rx data register 6"]
pub mod rxdata6;
#[doc = "RXDATA7 (r) register accessor: I2C rx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata7`]
module"]
#[doc(alias = "RXDATA7")]
pub type Rxdata7 = crate::Reg<rxdata7::Rxdata7Spec>;
#[doc = "I2C rx data register 7"]
pub mod rxdata7;
#[doc = "ST (r) register accessor: status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st`]
module"]
#[doc(alias = "ST")]
pub type St = crate::Reg<st::StSpec>;
#[doc = "status debug register"]
pub mod st;
