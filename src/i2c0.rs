#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rki2c_con: Rki2cCon,
    rki2c_clkdiv: Rki2cClkdiv,
    rki2c_mrxaddr: Rki2cMrxaddr,
    rki2c_mrxraddr: Rki2cMrxraddr,
    rki2c_mtxcnt: Rki2cMtxcnt,
    rki2c_mrxcnt: Rki2cMrxcnt,
    rki2c_ien: Rki2cIen,
    rki2c_ipd: Rki2cIpd,
    rki2c_fcnt: Rki2cFcnt,
    rki2c_scl_oe_db: Rki2cSclOeDb,
    _reserved10: [u8; 0xd8],
    rki2c_txdata0: Rki2cTxdata0,
    rki2c_txdata1: Rki2cTxdata1,
    rki2c_txdata2: Rki2cTxdata2,
    rki2c_txdata3: Rki2cTxdata3,
    rki2c_txdata4: Rki2cTxdata4,
    rki2c_txdata5: Rki2cTxdata5,
    rki2c_txdata6: Rki2cTxdata6,
    rki2c_txdata7: Rki2cTxdata7,
    _reserved18: [u8; 0xe0],
    rki2c_rxdata0: Rki2cRxdata0,
    rki2c_rxdata1: Rki2cRxdata1,
    rki2c_rxdata2: Rki2cRxdata2,
    rki2c_rxdata3: Rki2cRxdata3,
    rki2c_rxdata4: Rki2cRxdata4,
    rki2c_rxdata5: Rki2cRxdata5,
    rki2c_rxdata6: Rki2cRxdata6,
    rki2c_rxdata7: Rki2cRxdata7,
    rki2c_st: Rki2cSt,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn rki2c_con(&self) -> &Rki2cCon {
        &self.rki2c_con
    }
    #[doc = "0x04 - clock divider register"]
    #[inline(always)]
    pub const fn rki2c_clkdiv(&self) -> &Rki2cClkdiv {
        &self.rki2c_clkdiv
    }
    #[doc = "0x08 - the slave address accessed for master rx mode"]
    #[inline(always)]
    pub const fn rki2c_mrxaddr(&self) -> &Rki2cMrxaddr {
        &self.rki2c_mrxaddr
    }
    #[doc = "0x0c - the slave register address accessed for master rx mode"]
    #[inline(always)]
    pub const fn rki2c_mrxraddr(&self) -> &Rki2cMrxraddr {
        &self.rki2c_mrxraddr
    }
    #[doc = "0x10 - master transmit count"]
    #[inline(always)]
    pub const fn rki2c_mtxcnt(&self) -> &Rki2cMtxcnt {
        &self.rki2c_mtxcnt
    }
    #[doc = "0x14 - master rx count"]
    #[inline(always)]
    pub const fn rki2c_mrxcnt(&self) -> &Rki2cMrxcnt {
        &self.rki2c_mrxcnt
    }
    #[doc = "0x18 - interrupt enable register"]
    #[inline(always)]
    pub const fn rki2c_ien(&self) -> &Rki2cIen {
        &self.rki2c_ien
    }
    #[doc = "0x1c - interrupt pending register"]
    #[inline(always)]
    pub const fn rki2c_ipd(&self) -> &Rki2cIpd {
        &self.rki2c_ipd
    }
    #[doc = "0x20 - finished count"]
    #[inline(always)]
    pub const fn rki2c_fcnt(&self) -> &Rki2cFcnt {
        &self.rki2c_fcnt
    }
    #[doc = "0x24 - slave hold debounce configure register"]
    #[inline(always)]
    pub const fn rki2c_scl_oe_db(&self) -> &Rki2cSclOeDb {
        &self.rki2c_scl_oe_db
    }
    #[doc = "0x100 - I2C tx data register 0"]
    #[inline(always)]
    pub const fn rki2c_txdata0(&self) -> &Rki2cTxdata0 {
        &self.rki2c_txdata0
    }
    #[doc = "0x104 - I2C tx data register 1"]
    #[inline(always)]
    pub const fn rki2c_txdata1(&self) -> &Rki2cTxdata1 {
        &self.rki2c_txdata1
    }
    #[doc = "0x108 - I2C tx data register 2"]
    #[inline(always)]
    pub const fn rki2c_txdata2(&self) -> &Rki2cTxdata2 {
        &self.rki2c_txdata2
    }
    #[doc = "0x10c - I2C tx data register 3"]
    #[inline(always)]
    pub const fn rki2c_txdata3(&self) -> &Rki2cTxdata3 {
        &self.rki2c_txdata3
    }
    #[doc = "0x110 - I2C tx data register 4"]
    #[inline(always)]
    pub const fn rki2c_txdata4(&self) -> &Rki2cTxdata4 {
        &self.rki2c_txdata4
    }
    #[doc = "0x114 - I2C tx data register 5"]
    #[inline(always)]
    pub const fn rki2c_txdata5(&self) -> &Rki2cTxdata5 {
        &self.rki2c_txdata5
    }
    #[doc = "0x118 - I2C tx data register 6"]
    #[inline(always)]
    pub const fn rki2c_txdata6(&self) -> &Rki2cTxdata6 {
        &self.rki2c_txdata6
    }
    #[doc = "0x11c - I2C tx data register 7"]
    #[inline(always)]
    pub const fn rki2c_txdata7(&self) -> &Rki2cTxdata7 {
        &self.rki2c_txdata7
    }
    #[doc = "0x200 - I2C rx data register 0"]
    #[inline(always)]
    pub const fn rki2c_rxdata0(&self) -> &Rki2cRxdata0 {
        &self.rki2c_rxdata0
    }
    #[doc = "0x204 - I2C rx data register 1"]
    #[inline(always)]
    pub const fn rki2c_rxdata1(&self) -> &Rki2cRxdata1 {
        &self.rki2c_rxdata1
    }
    #[doc = "0x208 - I2C rx data register 2"]
    #[inline(always)]
    pub const fn rki2c_rxdata2(&self) -> &Rki2cRxdata2 {
        &self.rki2c_rxdata2
    }
    #[doc = "0x20c - I2C rx data register 3"]
    #[inline(always)]
    pub const fn rki2c_rxdata3(&self) -> &Rki2cRxdata3 {
        &self.rki2c_rxdata3
    }
    #[doc = "0x210 - I2C rx data register 4"]
    #[inline(always)]
    pub const fn rki2c_rxdata4(&self) -> &Rki2cRxdata4 {
        &self.rki2c_rxdata4
    }
    #[doc = "0x214 - I2C rx data register 5"]
    #[inline(always)]
    pub const fn rki2c_rxdata5(&self) -> &Rki2cRxdata5 {
        &self.rki2c_rxdata5
    }
    #[doc = "0x218 - I2C rx data register 6"]
    #[inline(always)]
    pub const fn rki2c_rxdata6(&self) -> &Rki2cRxdata6 {
        &self.rki2c_rxdata6
    }
    #[doc = "0x21c - I2C rx data register 7"]
    #[inline(always)]
    pub const fn rki2c_rxdata7(&self) -> &Rki2cRxdata7 {
        &self.rki2c_rxdata7
    }
    #[doc = "0x220 - status debug register"]
    #[inline(always)]
    pub const fn rki2c_st(&self) -> &Rki2cSt {
        &self.rki2c_st
    }
}
#[doc = "RKI2C_CON (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_con`]
module"]
#[doc(alias = "RKI2C_CON")]
pub type Rki2cCon = crate::Reg<rki2c_con::Rki2cConSpec>;
#[doc = "control register"]
pub mod rki2c_con;
#[doc = "RKI2C_CLKDIV (rw) register accessor: clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_clkdiv`]
module"]
#[doc(alias = "RKI2C_CLKDIV")]
pub type Rki2cClkdiv = crate::Reg<rki2c_clkdiv::Rki2cClkdivSpec>;
#[doc = "clock divider register"]
pub mod rki2c_clkdiv;
#[doc = "RKI2C_MRXADDR (rw) register accessor: the slave address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mrxaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mrxaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_mrxaddr`]
module"]
#[doc(alias = "RKI2C_MRXADDR")]
pub type Rki2cMrxaddr = crate::Reg<rki2c_mrxaddr::Rki2cMrxaddrSpec>;
#[doc = "the slave address accessed for master rx mode"]
pub mod rki2c_mrxaddr;
#[doc = "RKI2C_MRXRADDR (rw) register accessor: the slave register address accessed for master rx mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mrxraddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mrxraddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_mrxraddr`]
module"]
#[doc(alias = "RKI2C_MRXRADDR")]
pub type Rki2cMrxraddr = crate::Reg<rki2c_mrxraddr::Rki2cMrxraddrSpec>;
#[doc = "the slave register address accessed for master rx mode"]
pub mod rki2c_mrxraddr;
#[doc = "RKI2C_MTXCNT (rw) register accessor: master transmit count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mtxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mtxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_mtxcnt`]
module"]
#[doc(alias = "RKI2C_MTXCNT")]
pub type Rki2cMtxcnt = crate::Reg<rki2c_mtxcnt::Rki2cMtxcntSpec>;
#[doc = "master transmit count"]
pub mod rki2c_mtxcnt;
#[doc = "RKI2C_MRXCNT (rw) register accessor: master rx count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mrxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mrxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_mrxcnt`]
module"]
#[doc(alias = "RKI2C_MRXCNT")]
pub type Rki2cMrxcnt = crate::Reg<rki2c_mrxcnt::Rki2cMrxcntSpec>;
#[doc = "master rx count"]
pub mod rki2c_mrxcnt;
#[doc = "RKI2C_IEN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_ien`]
module"]
#[doc(alias = "RKI2C_IEN")]
pub type Rki2cIen = crate::Reg<rki2c_ien::Rki2cIenSpec>;
#[doc = "interrupt enable register"]
pub mod rki2c_ien;
#[doc = "RKI2C_IPD (rw) register accessor: interrupt pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_ipd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_ipd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_ipd`]
module"]
#[doc(alias = "RKI2C_IPD")]
pub type Rki2cIpd = crate::Reg<rki2c_ipd::Rki2cIpdSpec>;
#[doc = "interrupt pending register"]
pub mod rki2c_ipd;
#[doc = "RKI2C_FCNT (r) register accessor: finished count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_fcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_fcnt`]
module"]
#[doc(alias = "RKI2C_FCNT")]
pub type Rki2cFcnt = crate::Reg<rki2c_fcnt::Rki2cFcntSpec>;
#[doc = "finished count"]
pub mod rki2c_fcnt;
#[doc = "RKI2C_SCL_OE_DB (rw) register accessor: slave hold debounce configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_scl_oe_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_scl_oe_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_scl_oe_db`]
module"]
#[doc(alias = "RKI2C_SCL_OE_DB")]
pub type Rki2cSclOeDb = crate::Reg<rki2c_scl_oe_db::Rki2cSclOeDbSpec>;
#[doc = "slave hold debounce configure register"]
pub mod rki2c_scl_oe_db;
#[doc = "RKI2C_TXDATA0 (rw) register accessor: I2C tx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata0`]
module"]
#[doc(alias = "RKI2C_TXDATA0")]
pub type Rki2cTxdata0 = crate::Reg<rki2c_txdata0::Rki2cTxdata0Spec>;
#[doc = "I2C tx data register 0"]
pub mod rki2c_txdata0;
#[doc = "RKI2C_TXDATA1 (rw) register accessor: I2C tx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata1`]
module"]
#[doc(alias = "RKI2C_TXDATA1")]
pub type Rki2cTxdata1 = crate::Reg<rki2c_txdata1::Rki2cTxdata1Spec>;
#[doc = "I2C tx data register 1"]
pub mod rki2c_txdata1;
#[doc = "RKI2C_TXDATA2 (rw) register accessor: I2C tx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata2`]
module"]
#[doc(alias = "RKI2C_TXDATA2")]
pub type Rki2cTxdata2 = crate::Reg<rki2c_txdata2::Rki2cTxdata2Spec>;
#[doc = "I2C tx data register 2"]
pub mod rki2c_txdata2;
#[doc = "RKI2C_TXDATA3 (rw) register accessor: I2C tx data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata3`]
module"]
#[doc(alias = "RKI2C_TXDATA3")]
pub type Rki2cTxdata3 = crate::Reg<rki2c_txdata3::Rki2cTxdata3Spec>;
#[doc = "I2C tx data register 3"]
pub mod rki2c_txdata3;
#[doc = "RKI2C_TXDATA4 (rw) register accessor: I2C tx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata4`]
module"]
#[doc(alias = "RKI2C_TXDATA4")]
pub type Rki2cTxdata4 = crate::Reg<rki2c_txdata4::Rki2cTxdata4Spec>;
#[doc = "I2C tx data register 4"]
pub mod rki2c_txdata4;
#[doc = "RKI2C_TXDATA5 (rw) register accessor: I2C tx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata5`]
module"]
#[doc(alias = "RKI2C_TXDATA5")]
pub type Rki2cTxdata5 = crate::Reg<rki2c_txdata5::Rki2cTxdata5Spec>;
#[doc = "I2C tx data register 5"]
pub mod rki2c_txdata5;
#[doc = "RKI2C_TXDATA6 (rw) register accessor: I2C tx data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata6`]
module"]
#[doc(alias = "RKI2C_TXDATA6")]
pub type Rki2cTxdata6 = crate::Reg<rki2c_txdata6::Rki2cTxdata6Spec>;
#[doc = "I2C tx data register 6"]
pub mod rki2c_txdata6;
#[doc = "RKI2C_TXDATA7 (rw) register accessor: I2C tx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_txdata7`]
module"]
#[doc(alias = "RKI2C_TXDATA7")]
pub type Rki2cTxdata7 = crate::Reg<rki2c_txdata7::Rki2cTxdata7Spec>;
#[doc = "I2C tx data register 7"]
pub mod rki2c_txdata7;
#[doc = "RKI2C_RXDATA0 (r) register accessor: I2C rx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata0`]
module"]
#[doc(alias = "RKI2C_RXDATA0")]
pub type Rki2cRxdata0 = crate::Reg<rki2c_rxdata0::Rki2cRxdata0Spec>;
#[doc = "I2C rx data register 0"]
pub mod rki2c_rxdata0;
#[doc = "RKI2C_RXDATA1 (r) register accessor: I2C rx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata1`]
module"]
#[doc(alias = "RKI2C_RXDATA1")]
pub type Rki2cRxdata1 = crate::Reg<rki2c_rxdata1::Rki2cRxdata1Spec>;
#[doc = "I2C rx data register 1"]
pub mod rki2c_rxdata1;
#[doc = "RKI2C_RXDATA2 (r) register accessor: I2C rx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata2`]
module"]
#[doc(alias = "RKI2C_RXDATA2")]
pub type Rki2cRxdata2 = crate::Reg<rki2c_rxdata2::Rki2cRxdata2Spec>;
#[doc = "I2C rx data register 2"]
pub mod rki2c_rxdata2;
#[doc = "RKI2C_RXDATA3 (r) register accessor: I2C rx data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata3`]
module"]
#[doc(alias = "RKI2C_RXDATA3")]
pub type Rki2cRxdata3 = crate::Reg<rki2c_rxdata3::Rki2cRxdata3Spec>;
#[doc = "I2C rx data register 3"]
pub mod rki2c_rxdata3;
#[doc = "RKI2C_RXDATA4 (r) register accessor: I2C rx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata4`]
module"]
#[doc(alias = "RKI2C_RXDATA4")]
pub type Rki2cRxdata4 = crate::Reg<rki2c_rxdata4::Rki2cRxdata4Spec>;
#[doc = "I2C rx data register 4"]
pub mod rki2c_rxdata4;
#[doc = "RKI2C_RXDATA5 (r) register accessor: I2C rx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata5`]
module"]
#[doc(alias = "RKI2C_RXDATA5")]
pub type Rki2cRxdata5 = crate::Reg<rki2c_rxdata5::Rki2cRxdata5Spec>;
#[doc = "I2C rx data register 5"]
pub mod rki2c_rxdata5;
#[doc = "RKI2C_RXDATA6 (r) register accessor: I2C rx data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata6`]
module"]
#[doc(alias = "RKI2C_RXDATA6")]
pub type Rki2cRxdata6 = crate::Reg<rki2c_rxdata6::Rki2cRxdata6Spec>;
#[doc = "I2C rx data register 6"]
pub mod rki2c_rxdata6;
#[doc = "RKI2C_RXDATA7 (r) register accessor: I2C rx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_rxdata7`]
module"]
#[doc(alias = "RKI2C_RXDATA7")]
pub type Rki2cRxdata7 = crate::Reg<rki2c_rxdata7::Rki2cRxdata7Spec>;
#[doc = "I2C rx data register 7"]
pub mod rki2c_rxdata7;
#[doc = "RKI2C_ST (r) register accessor: status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rki2c_st`]
module"]
#[doc(alias = "RKI2C_ST")]
pub type Rki2cSt = crate::Reg<rki2c_st::Rki2cStSpec>;
#[doc = "status debug register"]
pub mod rki2c_st;
