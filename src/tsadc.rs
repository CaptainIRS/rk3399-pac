#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    user_con: UserCon,
    auto_con: AutoCon,
    int_en: IntEn,
    int_pd: IntPd,
    _reserved4: [u8; 0x10],
    data0: Data0,
    data1: Data1,
    _reserved6: [u8; 0x08],
    comp0_int: Comp0Int,
    comp1_int: Comp1Int,
    _reserved8: [u8; 0x08],
    comp0_shut: Comp0Shut,
    comp1_shut: Comp1Shut,
    _reserved10: [u8; 0x18],
    hight_int_debounce: HightIntDebounce,
    hight_tshut_debounce: HightTshutDebounce,
    auto_period: AutoPeriod,
    auto_period_ht: AutoPeriodHt,
    _reserved14: [u8; 0x10],
    comp0_low_int: Comp0LowInt,
    comp1_low_int: Comp1LowInt,
}
impl RegisterBlock {
    #[doc = "0x00 - The control register of A/D Converter."]
    #[inline(always)]
    pub const fn user_con(&self) -> &UserCon {
        &self.user_con
    }
    #[doc = "0x04 - TSADC auto mode control register"]
    #[inline(always)]
    pub const fn auto_con(&self) -> &AutoCon {
        &self.auto_con
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn int_pd(&self) -> &IntPd {
        &self.int_pd
    }
    #[doc = "0x20 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x24 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x30 - TSADC high temperature level for source 0"]
    #[inline(always)]
    pub const fn comp0_int(&self) -> &Comp0Int {
        &self.comp0_int
    }
    #[doc = "0x34 - TSADC high temperature level for source 1"]
    #[inline(always)]
    pub const fn comp1_int(&self) -> &Comp1Int {
        &self.comp1_int
    }
    #[doc = "0x40 - TSADC high temperature level for source 0"]
    #[inline(always)]
    pub const fn comp0_shut(&self) -> &Comp0Shut {
        &self.comp0_shut
    }
    #[doc = "0x44 - TSADC high temperature level for source 1"]
    #[inline(always)]
    pub const fn comp1_shut(&self) -> &Comp1Shut {
        &self.comp1_shut
    }
    #[doc = "0x60 - high temperature debounce"]
    #[inline(always)]
    pub const fn hight_int_debounce(&self) -> &HightIntDebounce {
        &self.hight_int_debounce
    }
    #[doc = "0x64 - high temperature debounce"]
    #[inline(always)]
    pub const fn hight_tshut_debounce(&self) -> &HightTshutDebounce {
        &self.hight_tshut_debounce
    }
    #[doc = "0x68 - TSADC auto access period"]
    #[inline(always)]
    pub const fn auto_period(&self) -> &AutoPeriod {
        &self.auto_period
    }
    #[doc = "0x6c - TSADC auto access period when temperature is high"]
    #[inline(always)]
    pub const fn auto_period_ht(&self) -> &AutoPeriodHt {
        &self.auto_period_ht
    }
    #[doc = "0x80 - TSADC low temperature level for source 0"]
    #[inline(always)]
    pub const fn comp0_low_int(&self) -> &Comp0LowInt {
        &self.comp0_low_int
    }
    #[doc = "0x84 - TSADC low temperature level for source 1"]
    #[inline(always)]
    pub const fn comp1_low_int(&self) -> &Comp1LowInt {
        &self.comp1_low_int
    }
}
#[doc = "USER_CON (rw) register accessor: The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_con`]
module"]
#[doc(alias = "USER_CON")]
pub type UserCon = crate::Reg<user_con::UserConSpec>;
#[doc = "The control register of A/D Converter."]
pub mod user_con;
#[doc = "AUTO_CON (rw) register accessor: TSADC auto mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_con`]
module"]
#[doc(alias = "AUTO_CON")]
pub type AutoCon = crate::Reg<auto_con::AutoConSpec>;
#[doc = "TSADC auto mode control register"]
pub mod auto_con;
#[doc = "INT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = ""]
pub mod int_en;
#[doc = "INT_PD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_pd`]
module"]
#[doc(alias = "INT_PD")]
pub type IntPd = crate::Reg<int_pd::IntPdSpec>;
#[doc = ""]
pub mod int_pd;
#[doc = "DATA0 (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod data0;
#[doc = "DATA1 (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod data1;
#[doc = "COMP0_INT (rw) register accessor: TSADC high temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_int`]
module"]
#[doc(alias = "COMP0_INT")]
pub type Comp0Int = crate::Reg<comp0_int::Comp0IntSpec>;
#[doc = "TSADC high temperature level for source 0"]
pub mod comp0_int;
#[doc = "COMP1_INT (rw) register accessor: TSADC high temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_int`]
module"]
#[doc(alias = "COMP1_INT")]
pub type Comp1Int = crate::Reg<comp1_int::Comp1IntSpec>;
#[doc = "TSADC high temperature level for source 1"]
pub mod comp1_int;
#[doc = "COMP0_SHUT (rw) register accessor: TSADC high temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0_shut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0_shut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_shut`]
module"]
#[doc(alias = "COMP0_SHUT")]
pub type Comp0Shut = crate::Reg<comp0_shut::Comp0ShutSpec>;
#[doc = "TSADC high temperature level for source 0"]
pub mod comp0_shut;
#[doc = "COMP1_SHUT (rw) register accessor: TSADC high temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_shut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_shut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_shut`]
module"]
#[doc(alias = "COMP1_SHUT")]
pub type Comp1Shut = crate::Reg<comp1_shut::Comp1ShutSpec>;
#[doc = "TSADC high temperature level for source 1"]
pub mod comp1_shut;
#[doc = "HIGHT_INT_DEBOUNCE (rw) register accessor: high temperature debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hight_int_debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hight_int_debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hight_int_debounce`]
module"]
#[doc(alias = "HIGHT_INT_DEBOUNCE")]
pub type HightIntDebounce = crate::Reg<hight_int_debounce::HightIntDebounceSpec>;
#[doc = "high temperature debounce"]
pub mod hight_int_debounce;
#[doc = "HIGHT_TSHUT_DEBOUNCE (rw) register accessor: high temperature debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hight_tshut_debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hight_tshut_debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hight_tshut_debounce`]
module"]
#[doc(alias = "HIGHT_TSHUT_DEBOUNCE")]
pub type HightTshutDebounce = crate::Reg<hight_tshut_debounce::HightTshutDebounceSpec>;
#[doc = "high temperature debounce"]
pub mod hight_tshut_debounce;
#[doc = "AUTO_PERIOD (rw) register accessor: TSADC auto access period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_period`]
module"]
#[doc(alias = "AUTO_PERIOD")]
pub type AutoPeriod = crate::Reg<auto_period::AutoPeriodSpec>;
#[doc = "TSADC auto access period"]
pub mod auto_period;
#[doc = "AUTO_PERIOD_HT (rw) register accessor: TSADC auto access period when temperature is high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_period_ht::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_period_ht::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_period_ht`]
module"]
#[doc(alias = "AUTO_PERIOD_HT")]
pub type AutoPeriodHt = crate::Reg<auto_period_ht::AutoPeriodHtSpec>;
#[doc = "TSADC auto access period when temperature is high"]
pub mod auto_period_ht;
#[doc = "COMP0_LOW_INT (rw) register accessor: TSADC low temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0_low_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0_low_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_low_int`]
module"]
#[doc(alias = "COMP0_LOW_INT")]
pub type Comp0LowInt = crate::Reg<comp0_low_int::Comp0LowIntSpec>;
#[doc = "TSADC low temperature level for source 0"]
pub mod comp0_low_int;
#[doc = "COMP1_LOW_INT (rw) register accessor: TSADC low temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_low_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_low_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_low_int`]
module"]
#[doc(alias = "COMP1_LOW_INT")]
pub type Comp1LowInt = crate::Reg<comp1_low_int::Comp1LowIntSpec>;
#[doc = "TSADC low temperature level for source 1"]
pub mod comp1_low_int;
