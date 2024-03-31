#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    n_load_count0: NLoadCount0,
    n_load_count1: NLoadCount1,
    n_current_value0: NCurrentValue0,
    n_current_value1: NCurrentValue1,
    n_load_count2: NLoadCount2,
    n_load_count3: NLoadCount3,
    n_intstatus: NIntstatus,
    n_controlreg: NControlreg,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer n higher Load Count Register"]
    #[inline(always)]
    pub const fn n_load_count0(&self) -> &NLoadCount0 {
        &self.n_load_count0
    }
    #[doc = "0x04 - Timer n higher Load Count Register"]
    #[inline(always)]
    pub const fn n_load_count1(&self) -> &NLoadCount1 {
        &self.n_load_count1
    }
    #[doc = "0x08 - Timer n Current Value Register"]
    #[inline(always)]
    pub const fn n_current_value0(&self) -> &NCurrentValue0 {
        &self.n_current_value0
    }
    #[doc = "0x0c - Timer n Current Value Register"]
    #[inline(always)]
    pub const fn n_current_value1(&self) -> &NCurrentValue1 {
        &self.n_current_value1
    }
    #[doc = "0x10 - Timer n lower Load Count Register"]
    #[inline(always)]
    pub const fn n_load_count2(&self) -> &NLoadCount2 {
        &self.n_load_count2
    }
    #[doc = "0x14 - Timer n lower Load Count Register"]
    #[inline(always)]
    pub const fn n_load_count3(&self) -> &NLoadCount3 {
        &self.n_load_count3
    }
    #[doc = "0x18 - Timer Interrupt Stauts Register"]
    #[inline(always)]
    pub const fn n_intstatus(&self) -> &NIntstatus {
        &self.n_intstatus
    }
    #[doc = "0x1c - Timer n Control Register"]
    #[inline(always)]
    pub const fn n_controlreg(&self) -> &NControlreg {
        &self.n_controlreg
    }
}
#[doc = "n_LOAD_COUNT0 (rw) register accessor: Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_load_count0`]
module"]
#[doc(alias = "n_LOAD_COUNT0")]
pub type NLoadCount0 = crate::Reg<n_load_count0::NLoadCount0Spec>;
#[doc = "Timer n higher Load Count Register"]
pub mod n_load_count0;
#[doc = "n_LOAD_COUNT1 (rw) register accessor: Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_load_count1`]
module"]
#[doc(alias = "n_LOAD_COUNT1")]
pub type NLoadCount1 = crate::Reg<n_load_count1::NLoadCount1Spec>;
#[doc = "Timer n higher Load Count Register"]
pub mod n_load_count1;
#[doc = "n_CURRENT_VALUE0 (r) register accessor: Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_current_value0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_current_value0`]
module"]
#[doc(alias = "n_CURRENT_VALUE0")]
pub type NCurrentValue0 = crate::Reg<n_current_value0::NCurrentValue0Spec>;
#[doc = "Timer n Current Value Register"]
pub mod n_current_value0;
#[doc = "n_CURRENT_VALUE1 (r) register accessor: Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_current_value1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_current_value1`]
module"]
#[doc(alias = "n_CURRENT_VALUE1")]
pub type NCurrentValue1 = crate::Reg<n_current_value1::NCurrentValue1Spec>;
#[doc = "Timer n Current Value Register"]
pub mod n_current_value1;
#[doc = "n_LOAD_COUNT2 (rw) register accessor: Timer n lower Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_load_count2`]
module"]
#[doc(alias = "n_LOAD_COUNT2")]
pub type NLoadCount2 = crate::Reg<n_load_count2::NLoadCount2Spec>;
#[doc = "Timer n lower Load Count Register"]
pub mod n_load_count2;
#[doc = "n_LOAD_COUNT3 (rw) register accessor: Timer n lower Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_load_count3`]
module"]
#[doc(alias = "n_LOAD_COUNT3")]
pub type NLoadCount3 = crate::Reg<n_load_count3::NLoadCount3Spec>;
#[doc = "Timer n lower Load Count Register"]
pub mod n_load_count3;
#[doc = "n_INTSTATUS (rw) register accessor: Timer Interrupt Stauts Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_intstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_intstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_intstatus`]
module"]
#[doc(alias = "n_INTSTATUS")]
pub type NIntstatus = crate::Reg<n_intstatus::NIntstatusSpec>;
#[doc = "Timer Interrupt Stauts Register"]
pub mod n_intstatus;
#[doc = "n_CONTROLREG (rw) register accessor: Timer n Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_controlreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_controlreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_controlreg`]
module"]
#[doc(alias = "n_CONTROLREG")]
pub type NControlreg = crate::Reg<n_controlreg::NControlregSpec>;
#[doc = "Timer n Control Register"]
pub mod n_controlreg;
