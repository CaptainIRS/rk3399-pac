#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer_n_load_count0: TimerNLoadCount0,
    timer_n_load_count1: TimerNLoadCount1,
    timer_n_current_value0: TimerNCurrentValue0,
    timer_n_current_value1: TimerNCurrentValue1,
    timer_n_load_count2: TimerNLoadCount2,
    timer_n_load_count3: TimerNLoadCount3,
    timer_n_intstatus: TimerNIntstatus,
    timer_n_controlreg: TimerNControlreg,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer n higher Load Count Register"]
    #[inline(always)]
    pub const fn timer_n_load_count0(&self) -> &TimerNLoadCount0 {
        &self.timer_n_load_count0
    }
    #[doc = "0x04 - Timer n higher Load Count Register"]
    #[inline(always)]
    pub const fn timer_n_load_count1(&self) -> &TimerNLoadCount1 {
        &self.timer_n_load_count1
    }
    #[doc = "0x08 - Timer n Current Value Register"]
    #[inline(always)]
    pub const fn timer_n_current_value0(&self) -> &TimerNCurrentValue0 {
        &self.timer_n_current_value0
    }
    #[doc = "0x0c - Timer n Current Value Register"]
    #[inline(always)]
    pub const fn timer_n_current_value1(&self) -> &TimerNCurrentValue1 {
        &self.timer_n_current_value1
    }
    #[doc = "0x10 - Timer n lower Load Count Register"]
    #[inline(always)]
    pub const fn timer_n_load_count2(&self) -> &TimerNLoadCount2 {
        &self.timer_n_load_count2
    }
    #[doc = "0x14 - Timer n lower Load Count Register"]
    #[inline(always)]
    pub const fn timer_n_load_count3(&self) -> &TimerNLoadCount3 {
        &self.timer_n_load_count3
    }
    #[doc = "0x18 - Timer Interrupt Stauts Register"]
    #[inline(always)]
    pub const fn timer_n_intstatus(&self) -> &TimerNIntstatus {
        &self.timer_n_intstatus
    }
    #[doc = "0x1c - Timer n Control Register"]
    #[inline(always)]
    pub const fn timer_n_controlreg(&self) -> &TimerNControlreg {
        &self.timer_n_controlreg
    }
}
#[doc = "TIMER_n_LOAD_COUNT0 (rw) register accessor: Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_load_count0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_load_count0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_load_count0`]
module"]
#[doc(alias = "TIMER_n_LOAD_COUNT0")]
pub type TimerNLoadCount0 = crate::Reg<timer_n_load_count0::TimerNLoadCount0Spec>;
#[doc = "Timer n higher Load Count Register"]
pub mod timer_n_load_count0;
#[doc = "TIMER_n_LOAD_COUNT1 (rw) register accessor: Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_load_count1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_load_count1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_load_count1`]
module"]
#[doc(alias = "TIMER_n_LOAD_COUNT1")]
pub type TimerNLoadCount1 = crate::Reg<timer_n_load_count1::TimerNLoadCount1Spec>;
#[doc = "Timer n higher Load Count Register"]
pub mod timer_n_load_count1;
#[doc = "TIMER_n_CURRENT_VALUE0 (r) register accessor: Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_current_value0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_current_value0`]
module"]
#[doc(alias = "TIMER_n_CURRENT_VALUE0")]
pub type TimerNCurrentValue0 = crate::Reg<timer_n_current_value0::TimerNCurrentValue0Spec>;
#[doc = "Timer n Current Value Register"]
pub mod timer_n_current_value0;
#[doc = "TIMER_n_CURRENT_VALUE1 (r) register accessor: Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_current_value1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_current_value1`]
module"]
#[doc(alias = "TIMER_n_CURRENT_VALUE1")]
pub type TimerNCurrentValue1 = crate::Reg<timer_n_current_value1::TimerNCurrentValue1Spec>;
#[doc = "Timer n Current Value Register"]
pub mod timer_n_current_value1;
#[doc = "TIMER_n_LOAD_COUNT2 (rw) register accessor: Timer n lower Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_load_count2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_load_count2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_load_count2`]
module"]
#[doc(alias = "TIMER_n_LOAD_COUNT2")]
pub type TimerNLoadCount2 = crate::Reg<timer_n_load_count2::TimerNLoadCount2Spec>;
#[doc = "Timer n lower Load Count Register"]
pub mod timer_n_load_count2;
#[doc = "TIMER_n_LOAD_COUNT3 (rw) register accessor: Timer n lower Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_load_count3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_load_count3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_load_count3`]
module"]
#[doc(alias = "TIMER_n_LOAD_COUNT3")]
pub type TimerNLoadCount3 = crate::Reg<timer_n_load_count3::TimerNLoadCount3Spec>;
#[doc = "Timer n lower Load Count Register"]
pub mod timer_n_load_count3;
#[doc = "TIMER_n_INTSTATUS (rw) register accessor: Timer Interrupt Stauts Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_intstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_intstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_intstatus`]
module"]
#[doc(alias = "TIMER_n_INTSTATUS")]
pub type TimerNIntstatus = crate::Reg<timer_n_intstatus::TimerNIntstatusSpec>;
#[doc = "Timer Interrupt Stauts Register"]
pub mod timer_n_intstatus;
#[doc = "TIMER_n_CONTROLREG (rw) register accessor: Timer n Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_controlreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_controlreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_n_controlreg`]
module"]
#[doc(alias = "TIMER_n_CONTROLREG")]
pub type TimerNControlreg = crate::Reg<timer_n_controlreg::TimerNControlregSpec>;
#[doc = "Timer n Control Register"]
pub mod timer_n_controlreg;
