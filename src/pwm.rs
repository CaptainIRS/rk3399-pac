#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwm_pwm0_cnt: PwmPwm0Cnt,
    pwm_pwm0_period_hpr: PwmPwm0PeriodHpr,
    pwm_pwm0_duty_lpr: PwmPwm0DutyLpr,
    pwm_pwm0_ctrl: PwmPwm0Ctrl,
    pwm_pwm1_cnt: PwmPwm1Cnt,
    pwm_pwm1_period_hpr: PwmPwm1PeriodHpr,
    pwm_pwm1_duty_lpr: PwmPwm1DutyLpr,
    pwm_pwm1_ctrl: PwmPwm1Ctrl,
    pwm_pwm2_cnt: PwmPwm2Cnt,
    pwm_pwm2_period_hpr: PwmPwm2PeriodHpr,
    pwm_pwm2_duty_lpr: PwmPwm2DutyLpr,
    pwm_pwm2_ctrl: PwmPwm2Ctrl,
    pwm_pwm3_cnt: PwmPwm3Cnt,
    pwm_pwm3_period_hpr: PwmPwm3PeriodHpr,
    pwm_pwm3_duty_lpr: PwmPwm3DutyLpr,
    pwm_pwm3_ctrl: PwmPwm3Ctrl,
    pwm_intsts: PwmIntsts,
    pwm_int_en: PwmIntEn,
    _reserved18: [u8; 0x08],
    pwm_pwm_fifo_ctrl: PwmPwmFifoCtrl,
    pwm_pwm_fifo_intsts: PwmPwmFifoIntsts,
    pwm_pwm_fifo_toutthr: PwmPwmFifoToutthr,
    _reserved21: [u8; 0x04],
    pwm_pwm_fifo: [PwmPwmFifo; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Channel 0 Counter Register"]
    #[inline(always)]
    pub const fn pwm_pwm0_cnt(&self) -> &PwmPwm0Cnt {
        &self.pwm_pwm0_cnt
    }
    #[doc = "0x04 - PWM Channel 0 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm0_period_hpr(&self) -> &PwmPwm0PeriodHpr {
        &self.pwm_pwm0_period_hpr
    }
    #[doc = "0x08 - PWM Channel 0 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm0_duty_lpr(&self) -> &PwmPwm0DutyLpr {
        &self.pwm_pwm0_duty_lpr
    }
    #[doc = "0x0c - PWM Channel 0 Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm0_ctrl(&self) -> &PwmPwm0Ctrl {
        &self.pwm_pwm0_ctrl
    }
    #[doc = "0x10 - PWM Channel 1 Counter Register"]
    #[inline(always)]
    pub const fn pwm_pwm1_cnt(&self) -> &PwmPwm1Cnt {
        &self.pwm_pwm1_cnt
    }
    #[doc = "0x14 - PWM Channel 1 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm1_period_hpr(&self) -> &PwmPwm1PeriodHpr {
        &self.pwm_pwm1_period_hpr
    }
    #[doc = "0x18 - PWM Channel 1 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm1_duty_lpr(&self) -> &PwmPwm1DutyLpr {
        &self.pwm_pwm1_duty_lpr
    }
    #[doc = "0x1c - PWM Channel 1 Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm1_ctrl(&self) -> &PwmPwm1Ctrl {
        &self.pwm_pwm1_ctrl
    }
    #[doc = "0x20 - PWM Channel 2 Counter Register"]
    #[inline(always)]
    pub const fn pwm_pwm2_cnt(&self) -> &PwmPwm2Cnt {
        &self.pwm_pwm2_cnt
    }
    #[doc = "0x24 - PWM Channel 2 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm2_period_hpr(&self) -> &PwmPwm2PeriodHpr {
        &self.pwm_pwm2_period_hpr
    }
    #[doc = "0x28 - PWM Channel 2 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm2_duty_lpr(&self) -> &PwmPwm2DutyLpr {
        &self.pwm_pwm2_duty_lpr
    }
    #[doc = "0x2c - PWM Channel 2 Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm2_ctrl(&self) -> &PwmPwm2Ctrl {
        &self.pwm_pwm2_ctrl
    }
    #[doc = "0x30 - PWM Channel 3 Counter Register"]
    #[inline(always)]
    pub const fn pwm_pwm3_cnt(&self) -> &PwmPwm3Cnt {
        &self.pwm_pwm3_cnt
    }
    #[doc = "0x34 - PWM Channel 3 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm3_period_hpr(&self) -> &PwmPwm3PeriodHpr {
        &self.pwm_pwm3_period_hpr
    }
    #[doc = "0x38 - PWM Channel 3 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm_pwm3_duty_lpr(&self) -> &PwmPwm3DutyLpr {
        &self.pwm_pwm3_duty_lpr
    }
    #[doc = "0x3c - PWM Channel 3 Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm3_ctrl(&self) -> &PwmPwm3Ctrl {
        &self.pwm_pwm3_ctrl
    }
    #[doc = "0x40 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn pwm_intsts(&self) -> &PwmIntsts {
        &self.pwm_intsts
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pwm_int_en(&self) -> &PwmIntEn {
        &self.pwm_int_en
    }
    #[doc = "0x50 - PWM Channel 3 FIFO Mode Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm_fifo_ctrl(&self) -> &PwmPwmFifoCtrl {
        &self.pwm_pwm_fifo_ctrl
    }
    #[doc = "0x54 - FIFO Interrupts Status Register"]
    #[inline(always)]
    pub const fn pwm_pwm_fifo_intsts(&self) -> &PwmPwmFifoIntsts {
        &self.pwm_pwm_fifo_intsts
    }
    #[doc = "0x58 - FIFO Timeout Threshold Register"]
    #[inline(always)]
    pub const fn pwm_pwm_fifo_toutthr(&self) -> &PwmPwmFifoToutthr {
        &self.pwm_pwm_fifo_toutthr
    }
    #[doc = "0x60..0x80 - FIFO Register"]
    #[inline(always)]
    pub const fn pwm_pwm_fifo(&self, n: usize) -> &PwmPwmFifo {
        &self.pwm_pwm_fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - FIFO Register"]
    #[inline(always)]
    pub fn pwm_pwm_fifo_iter(&self) -> impl Iterator<Item = &PwmPwmFifo> {
        self.pwm_pwm_fifo.iter()
    }
}
#[doc = "PWM_PWM0_CNT (r) register accessor: PWM Channel 0 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm0_cnt`]
module"]
#[doc(alias = "PWM_PWM0_CNT")]
pub type PwmPwm0Cnt = crate::Reg<pwm_pwm0_cnt::PwmPwm0CntSpec>;
#[doc = "PWM Channel 0 Counter Register"]
pub mod pwm_pwm0_cnt;
#[doc = "PWM_PWM0_PERIOD_HPR (rw) register accessor: PWM Channel 0 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm0_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm0_period_hpr`]
module"]
#[doc(alias = "PWM_PWM0_PERIOD_HPR")]
pub type PwmPwm0PeriodHpr = crate::Reg<pwm_pwm0_period_hpr::PwmPwm0PeriodHprSpec>;
#[doc = "PWM Channel 0 Period Register/High Polarity Capture Register"]
pub mod pwm_pwm0_period_hpr;
#[doc = "PWM_PWM0_DUTY_LPR (rw) register accessor: PWM Channel 0 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm0_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm0_duty_lpr`]
module"]
#[doc(alias = "PWM_PWM0_DUTY_LPR")]
pub type PwmPwm0DutyLpr = crate::Reg<pwm_pwm0_duty_lpr::PwmPwm0DutyLprSpec>;
#[doc = "PWM Channel 0 Duty Register/Low Polarity Capture Register"]
pub mod pwm_pwm0_duty_lpr;
#[doc = "PWM_PWM0_CTRL (rw) register accessor: PWM Channel 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm0_ctrl`]
module"]
#[doc(alias = "PWM_PWM0_CTRL")]
pub type PwmPwm0Ctrl = crate::Reg<pwm_pwm0_ctrl::PwmPwm0CtrlSpec>;
#[doc = "PWM Channel 0 Control Register"]
pub mod pwm_pwm0_ctrl;
#[doc = "PWM_PWM1_CNT (r) register accessor: PWM Channel 1 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm1_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm1_cnt`]
module"]
#[doc(alias = "PWM_PWM1_CNT")]
pub type PwmPwm1Cnt = crate::Reg<pwm_pwm1_cnt::PwmPwm1CntSpec>;
#[doc = "PWM Channel 1 Counter Register"]
pub mod pwm_pwm1_cnt;
#[doc = "PWM_PWM1_PERIOD_HPR (rw) register accessor: PWM Channel 1 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm1_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm1_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm1_period_hpr`]
module"]
#[doc(alias = "PWM_PWM1_PERIOD_HPR")]
pub type PwmPwm1PeriodHpr = crate::Reg<pwm_pwm1_period_hpr::PwmPwm1PeriodHprSpec>;
#[doc = "PWM Channel 1 Period Register/High Polarity Capture Register"]
pub mod pwm_pwm1_period_hpr;
#[doc = "PWM_PWM1_DUTY_LPR (rw) register accessor: PWM Channel 1 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm1_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm1_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm1_duty_lpr`]
module"]
#[doc(alias = "PWM_PWM1_DUTY_LPR")]
pub type PwmPwm1DutyLpr = crate::Reg<pwm_pwm1_duty_lpr::PwmPwm1DutyLprSpec>;
#[doc = "PWM Channel 1 Duty Register/Low Polarity Capture Register"]
pub mod pwm_pwm1_duty_lpr;
#[doc = "PWM_PWM1_CTRL (rw) register accessor: PWM Channel 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm1_ctrl`]
module"]
#[doc(alias = "PWM_PWM1_CTRL")]
pub type PwmPwm1Ctrl = crate::Reg<pwm_pwm1_ctrl::PwmPwm1CtrlSpec>;
#[doc = "PWM Channel 1 Control Register"]
pub mod pwm_pwm1_ctrl;
#[doc = "PWM_PWM2_CNT (r) register accessor: PWM Channel 2 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm2_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm2_cnt`]
module"]
#[doc(alias = "PWM_PWM2_CNT")]
pub type PwmPwm2Cnt = crate::Reg<pwm_pwm2_cnt::PwmPwm2CntSpec>;
#[doc = "PWM Channel 2 Counter Register"]
pub mod pwm_pwm2_cnt;
#[doc = "PWM_PWM2_PERIOD_HPR (rw) register accessor: PWM Channel 2 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm2_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm2_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm2_period_hpr`]
module"]
#[doc(alias = "PWM_PWM2_PERIOD_HPR")]
pub type PwmPwm2PeriodHpr = crate::Reg<pwm_pwm2_period_hpr::PwmPwm2PeriodHprSpec>;
#[doc = "PWM Channel 2 Period Register/High Polarity Capture Register"]
pub mod pwm_pwm2_period_hpr;
#[doc = "PWM_PWM2_DUTY_LPR (rw) register accessor: PWM Channel 2 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm2_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm2_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm2_duty_lpr`]
module"]
#[doc(alias = "PWM_PWM2_DUTY_LPR")]
pub type PwmPwm2DutyLpr = crate::Reg<pwm_pwm2_duty_lpr::PwmPwm2DutyLprSpec>;
#[doc = "PWM Channel 2 Duty Register/Low Polarity Capture Register"]
pub mod pwm_pwm2_duty_lpr;
#[doc = "PWM_PWM2_CTRL (rw) register accessor: PWM Channel 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm2_ctrl`]
module"]
#[doc(alias = "PWM_PWM2_CTRL")]
pub type PwmPwm2Ctrl = crate::Reg<pwm_pwm2_ctrl::PwmPwm2CtrlSpec>;
#[doc = "PWM Channel 2 Control Register"]
pub mod pwm_pwm2_ctrl;
#[doc = "PWM_PWM3_CNT (r) register accessor: PWM Channel 3 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm3_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm3_cnt`]
module"]
#[doc(alias = "PWM_PWM3_CNT")]
pub type PwmPwm3Cnt = crate::Reg<pwm_pwm3_cnt::PwmPwm3CntSpec>;
#[doc = "PWM Channel 3 Counter Register"]
pub mod pwm_pwm3_cnt;
#[doc = "PWM_PWM3_PERIOD_HPR (rw) register accessor: PWM Channel 3 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm3_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm3_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm3_period_hpr`]
module"]
#[doc(alias = "PWM_PWM3_PERIOD_HPR")]
pub type PwmPwm3PeriodHpr = crate::Reg<pwm_pwm3_period_hpr::PwmPwm3PeriodHprSpec>;
#[doc = "PWM Channel 3 Period Register/High Polarity Capture Register"]
pub mod pwm_pwm3_period_hpr;
#[doc = "PWM_PWM3_DUTY_LPR (rw) register accessor: PWM Channel 3 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm3_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm3_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm3_duty_lpr`]
module"]
#[doc(alias = "PWM_PWM3_DUTY_LPR")]
pub type PwmPwm3DutyLpr = crate::Reg<pwm_pwm3_duty_lpr::PwmPwm3DutyLprSpec>;
#[doc = "PWM Channel 3 Duty Register/Low Polarity Capture Register"]
pub mod pwm_pwm3_duty_lpr;
#[doc = "PWM_PWM3_CTRL (rw) register accessor: PWM Channel 3 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm3_ctrl`]
module"]
#[doc(alias = "PWM_PWM3_CTRL")]
pub type PwmPwm3Ctrl = crate::Reg<pwm_pwm3_ctrl::PwmPwm3CtrlSpec>;
#[doc = "PWM Channel 3 Control Register"]
pub mod pwm_pwm3_ctrl;
#[doc = "PWM_INTSTS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_intsts`]
module"]
#[doc(alias = "PWM_INTSTS")]
pub type PwmIntsts = crate::Reg<pwm_intsts::PwmIntstsSpec>;
#[doc = "Interrupt Status Register"]
pub mod pwm_intsts;
#[doc = "PWM_INT_EN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_int_en`]
module"]
#[doc(alias = "PWM_INT_EN")]
pub type PwmIntEn = crate::Reg<pwm_int_en::PwmIntEnSpec>;
#[doc = "Interrupt Enable Register"]
pub mod pwm_int_en;
#[doc = "PWM_PWM_FIFO_CTRL (rw) register accessor: PWM Channel 3 FIFO Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_fifo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_fifo_ctrl`]
module"]
#[doc(alias = "PWM_PWM_FIFO_CTRL")]
pub type PwmPwmFifoCtrl = crate::Reg<pwm_pwm_fifo_ctrl::PwmPwmFifoCtrlSpec>;
#[doc = "PWM Channel 3 FIFO Mode Control Register"]
pub mod pwm_pwm_fifo_ctrl;
#[doc = "PWM_PWM_FIFO_INTSTS (rw) register accessor: FIFO Interrupts Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_fifo_intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_fifo_intsts`]
module"]
#[doc(alias = "PWM_PWM_FIFO_INTSTS")]
pub type PwmPwmFifoIntsts = crate::Reg<pwm_pwm_fifo_intsts::PwmPwmFifoIntstsSpec>;
#[doc = "FIFO Interrupts Status Register"]
pub mod pwm_pwm_fifo_intsts;
#[doc = "PWM_PWM_FIFO_TOUTTHR (r) register accessor: FIFO Timeout Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_toutthr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_fifo_toutthr`]
module"]
#[doc(alias = "PWM_PWM_FIFO_TOUTTHR")]
pub type PwmPwmFifoToutthr = crate::Reg<pwm_pwm_fifo_toutthr::PwmPwmFifoToutthrSpec>;
#[doc = "FIFO Timeout Threshold Register"]
pub mod pwm_pwm_fifo_toutthr;
#[doc = "PWM_PWM_FIFO (rw) register accessor: FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_fifo`]
module"]
#[doc(alias = "PWM_PWM_FIFO")]
pub type PwmPwmFifo = crate::Reg<pwm_pwm_fifo::PwmPwmFifoSpec>;
#[doc = "FIFO Register"]
pub mod pwm_pwm_fifo;
