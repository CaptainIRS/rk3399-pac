#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tsadc_user_con: TsadcUserCon,
    tsadc_auto_con: TsadcAutoCon,
    tsadc_int_en: TsadcIntEn,
    tsadc_int_pd: TsadcIntPd,
    _reserved4: [u8; 0x10],
    tsadc_data0: TsadcData0,
    tsadc_data1: TsadcData1,
    _reserved6: [u8; 0x08],
    tsadc_comp0_int: TsadcComp0Int,
    tsadc_comp1_int: TsadcComp1Int,
    _reserved8: [u8; 0x08],
    tsadc_comp0_shut: TsadcComp0Shut,
    tsadc_comp1_shut: TsadcComp1Shut,
    _reserved10: [u8; 0x18],
    tsadc_hight_int_debounce: TsadcHightIntDebounce,
    tsadc_hight_tshut_debounce: TsadcHightTshutDebounce,
    tsadc_auto_period: TsadcAutoPeriod,
    tsadc_auto_period_ht: TsadcAutoPeriodHt,
    _reserved14: [u8; 0x10],
    tsadc_comp0_low_int: TsadcComp0LowInt,
    tsadc_comp1_low_int: TsadcComp1LowInt,
}
impl RegisterBlock {
    #[doc = "0x00 - The control register of A/D Converter."]
    #[inline(always)]
    pub const fn tsadc_user_con(&self) -> &TsadcUserCon {
        &self.tsadc_user_con
    }
    #[doc = "0x04 - TSADC auto mode control register"]
    #[inline(always)]
    pub const fn tsadc_auto_con(&self) -> &TsadcAutoCon {
        &self.tsadc_auto_con
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn tsadc_int_en(&self) -> &TsadcIntEn {
        &self.tsadc_int_en
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn tsadc_int_pd(&self) -> &TsadcIntPd {
        &self.tsadc_int_pd
    }
    #[doc = "0x20 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn tsadc_data0(&self) -> &TsadcData0 {
        &self.tsadc_data0
    }
    #[doc = "0x24 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn tsadc_data1(&self) -> &TsadcData1 {
        &self.tsadc_data1
    }
    #[doc = "0x30 - TSADC high temperature level for source 0"]
    #[inline(always)]
    pub const fn tsadc_comp0_int(&self) -> &TsadcComp0Int {
        &self.tsadc_comp0_int
    }
    #[doc = "0x34 - TSADC high temperature level for source 1"]
    #[inline(always)]
    pub const fn tsadc_comp1_int(&self) -> &TsadcComp1Int {
        &self.tsadc_comp1_int
    }
    #[doc = "0x40 - TSADC high temperature level for source 0"]
    #[inline(always)]
    pub const fn tsadc_comp0_shut(&self) -> &TsadcComp0Shut {
        &self.tsadc_comp0_shut
    }
    #[doc = "0x44 - TSADC high temperature level for source 1"]
    #[inline(always)]
    pub const fn tsadc_comp1_shut(&self) -> &TsadcComp1Shut {
        &self.tsadc_comp1_shut
    }
    #[doc = "0x60 - high temperature debounce"]
    #[inline(always)]
    pub const fn tsadc_hight_int_debounce(&self) -> &TsadcHightIntDebounce {
        &self.tsadc_hight_int_debounce
    }
    #[doc = "0x64 - high temperature debounce"]
    #[inline(always)]
    pub const fn tsadc_hight_tshut_debounce(&self) -> &TsadcHightTshutDebounce {
        &self.tsadc_hight_tshut_debounce
    }
    #[doc = "0x68 - TSADC auto access period"]
    #[inline(always)]
    pub const fn tsadc_auto_period(&self) -> &TsadcAutoPeriod {
        &self.tsadc_auto_period
    }
    #[doc = "0x6c - TSADC auto access period when temperature is high"]
    #[inline(always)]
    pub const fn tsadc_auto_period_ht(&self) -> &TsadcAutoPeriodHt {
        &self.tsadc_auto_period_ht
    }
    #[doc = "0x80 - TSADC low temperature level for source 0"]
    #[inline(always)]
    pub const fn tsadc_comp0_low_int(&self) -> &TsadcComp0LowInt {
        &self.tsadc_comp0_low_int
    }
    #[doc = "0x84 - TSADC low temperature level for source 1"]
    #[inline(always)]
    pub const fn tsadc_comp1_low_int(&self) -> &TsadcComp1LowInt {
        &self.tsadc_comp1_low_int
    }
}
#[doc = "TSADC_USER_CON (rw) register accessor: The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_user_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_user_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_user_con`]
module"]
#[doc(alias = "TSADC_USER_CON")]
pub type TsadcUserCon = crate::Reg<tsadc_user_con::TsadcUserConSpec>;
#[doc = "The control register of A/D Converter."]
pub mod tsadc_user_con;
#[doc = "TSADC_AUTO_CON (rw) register accessor: TSADC auto mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_auto_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_auto_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_auto_con`]
module"]
#[doc(alias = "TSADC_AUTO_CON")]
pub type TsadcAutoCon = crate::Reg<tsadc_auto_con::TsadcAutoConSpec>;
#[doc = "TSADC auto mode control register"]
pub mod tsadc_auto_con;
#[doc = "TSADC_INT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_int_en`]
module"]
#[doc(alias = "TSADC_INT_EN")]
pub type TsadcIntEn = crate::Reg<tsadc_int_en::TsadcIntEnSpec>;
#[doc = ""]
pub mod tsadc_int_en;
#[doc = "TSADC_INT_PD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_int_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_int_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_int_pd`]
module"]
#[doc(alias = "TSADC_INT_PD")]
pub type TsadcIntPd = crate::Reg<tsadc_int_pd::TsadcIntPdSpec>;
#[doc = ""]
pub mod tsadc_int_pd;
#[doc = "TSADC_DATA0 (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_data0`]
module"]
#[doc(alias = "TSADC_DATA0")]
pub type TsadcData0 = crate::Reg<tsadc_data0::TsadcData0Spec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod tsadc_data0;
#[doc = "TSADC_DATA1 (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_data1`]
module"]
#[doc(alias = "TSADC_DATA1")]
pub type TsadcData1 = crate::Reg<tsadc_data1::TsadcData1Spec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod tsadc_data1;
#[doc = "TSADC_COMP0_INT (rw) register accessor: TSADC high temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp0_int`]
module"]
#[doc(alias = "TSADC_COMP0_INT")]
pub type TsadcComp0Int = crate::Reg<tsadc_comp0_int::TsadcComp0IntSpec>;
#[doc = "TSADC high temperature level for source 0"]
pub mod tsadc_comp0_int;
#[doc = "TSADC_COMP1_INT (rw) register accessor: TSADC high temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp1_int`]
module"]
#[doc(alias = "TSADC_COMP1_INT")]
pub type TsadcComp1Int = crate::Reg<tsadc_comp1_int::TsadcComp1IntSpec>;
#[doc = "TSADC high temperature level for source 1"]
pub mod tsadc_comp1_int;
#[doc = "TSADC_COMP0_SHUT (rw) register accessor: TSADC high temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp0_shut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp0_shut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp0_shut`]
module"]
#[doc(alias = "TSADC_COMP0_SHUT")]
pub type TsadcComp0Shut = crate::Reg<tsadc_comp0_shut::TsadcComp0ShutSpec>;
#[doc = "TSADC high temperature level for source 0"]
pub mod tsadc_comp0_shut;
#[doc = "TSADC_COMP1_SHUT (rw) register accessor: TSADC high temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp1_shut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp1_shut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp1_shut`]
module"]
#[doc(alias = "TSADC_COMP1_SHUT")]
pub type TsadcComp1Shut = crate::Reg<tsadc_comp1_shut::TsadcComp1ShutSpec>;
#[doc = "TSADC high temperature level for source 1"]
pub mod tsadc_comp1_shut;
#[doc = "TSADC_HIGHT_INT_DEBOUNCE (rw) register accessor: high temperature debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_hight_int_debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_hight_int_debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_hight_int_debounce`]
module"]
#[doc(alias = "TSADC_HIGHT_INT_DEBOUNCE")]
pub type TsadcHightIntDebounce = crate::Reg<tsadc_hight_int_debounce::TsadcHightIntDebounceSpec>;
#[doc = "high temperature debounce"]
pub mod tsadc_hight_int_debounce;
#[doc = "TSADC_HIGHT_TSHUT_DEBOUNCE (rw) register accessor: high temperature debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_hight_tshut_debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_hight_tshut_debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_hight_tshut_debounce`]
module"]
#[doc(alias = "TSADC_HIGHT_TSHUT_DEBOUNCE")]
pub type TsadcHightTshutDebounce =
    crate::Reg<tsadc_hight_tshut_debounce::TsadcHightTshutDebounceSpec>;
#[doc = "high temperature debounce"]
pub mod tsadc_hight_tshut_debounce;
#[doc = "TSADC_AUTO_PERIOD (rw) register accessor: TSADC auto access period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_auto_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_auto_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_auto_period`]
module"]
#[doc(alias = "TSADC_AUTO_PERIOD")]
pub type TsadcAutoPeriod = crate::Reg<tsadc_auto_period::TsadcAutoPeriodSpec>;
#[doc = "TSADC auto access period"]
pub mod tsadc_auto_period;
#[doc = "TSADC_AUTO_PERIOD_HT (rw) register accessor: TSADC auto access period when temperature is high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_auto_period_ht::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_auto_period_ht::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_auto_period_ht`]
module"]
#[doc(alias = "TSADC_AUTO_PERIOD_HT")]
pub type TsadcAutoPeriodHt = crate::Reg<tsadc_auto_period_ht::TsadcAutoPeriodHtSpec>;
#[doc = "TSADC auto access period when temperature is high"]
pub mod tsadc_auto_period_ht;
#[doc = "TSADC_COMP0_LOW_INT (rw) register accessor: TSADC low temperature level for source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp0_low_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp0_low_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp0_low_int`]
module"]
#[doc(alias = "TSADC_COMP0_LOW_INT")]
pub type TsadcComp0LowInt = crate::Reg<tsadc_comp0_low_int::TsadcComp0LowIntSpec>;
#[doc = "TSADC low temperature level for source 0"]
pub mod tsadc_comp0_low_int;
#[doc = "TSADC_COMP1_LOW_INT (rw) register accessor: TSADC low temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp1_low_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp1_low_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_comp1_low_int`]
module"]
#[doc(alias = "TSADC_COMP1_LOW_INT")]
pub type TsadcComp1LowInt = crate::Reg<tsadc_comp1_low_int::TsadcComp1LowIntSpec>;
#[doc = "TSADC low temperature level for source 1"]
pub mod tsadc_comp1_low_int;
