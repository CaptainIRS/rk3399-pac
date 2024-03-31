#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwm0_cnt: Pwm0Cnt,
    pwm0_period_hpr: Pwm0PeriodHpr,
    pwm0_duty_lpr: Pwm0DutyLpr,
    pwm0_ctrl: Pwm0Ctrl,
    pwm1_cnt: Pwm1Cnt,
    pwm1_period_hpr: Pwm1PeriodHpr,
    pwm1_duty_lpr: Pwm1DutyLpr,
    pwm1_ctrl: Pwm1Ctrl,
    pwm2_cnt: Pwm2Cnt,
    pwm2_period_hpr: Pwm2PeriodHpr,
    pwm2_duty_lpr: Pwm2DutyLpr,
    pwm2_ctrl: Pwm2Ctrl,
    pwm3_cnt: Pwm3Cnt,
    pwm3_period_hpr: Pwm3PeriodHpr,
    pwm3_duty_lpr: Pwm3DutyLpr,
    pwm3_ctrl: Pwm3Ctrl,
    intsts: Intsts,
    int_en: IntEn,
    _reserved18: [u8; 0x08],
    pwm_fifo_ctrl: PwmFifoCtrl,
    pwm_fifo_intsts: PwmFifoIntsts,
    pwm_fifo_toutthr: PwmFifoToutthr,
    _reserved21: [u8; 0x04],
    pwm_fifo: [PwmFifo; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Channel 0 Counter Register"]
    #[inline(always)]
    pub const fn pwm0_cnt(&self) -> &Pwm0Cnt {
        &self.pwm0_cnt
    }
    #[doc = "0x04 - PWM Channel 0 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm0_period_hpr(&self) -> &Pwm0PeriodHpr {
        &self.pwm0_period_hpr
    }
    #[doc = "0x08 - PWM Channel 0 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm0_duty_lpr(&self) -> &Pwm0DutyLpr {
        &self.pwm0_duty_lpr
    }
    #[doc = "0x0c - PWM Channel 0 Control Register"]
    #[inline(always)]
    pub const fn pwm0_ctrl(&self) -> &Pwm0Ctrl {
        &self.pwm0_ctrl
    }
    #[doc = "0x10 - PWM Channel 1 Counter Register"]
    #[inline(always)]
    pub const fn pwm1_cnt(&self) -> &Pwm1Cnt {
        &self.pwm1_cnt
    }
    #[doc = "0x14 - PWM Channel 1 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm1_period_hpr(&self) -> &Pwm1PeriodHpr {
        &self.pwm1_period_hpr
    }
    #[doc = "0x18 - PWM Channel 1 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm1_duty_lpr(&self) -> &Pwm1DutyLpr {
        &self.pwm1_duty_lpr
    }
    #[doc = "0x1c - PWM Channel 1 Control Register"]
    #[inline(always)]
    pub const fn pwm1_ctrl(&self) -> &Pwm1Ctrl {
        &self.pwm1_ctrl
    }
    #[doc = "0x20 - PWM Channel 2 Counter Register"]
    #[inline(always)]
    pub const fn pwm2_cnt(&self) -> &Pwm2Cnt {
        &self.pwm2_cnt
    }
    #[doc = "0x24 - PWM Channel 2 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm2_period_hpr(&self) -> &Pwm2PeriodHpr {
        &self.pwm2_period_hpr
    }
    #[doc = "0x28 - PWM Channel 2 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm2_duty_lpr(&self) -> &Pwm2DutyLpr {
        &self.pwm2_duty_lpr
    }
    #[doc = "0x2c - PWM Channel 2 Control Register"]
    #[inline(always)]
    pub const fn pwm2_ctrl(&self) -> &Pwm2Ctrl {
        &self.pwm2_ctrl
    }
    #[doc = "0x30 - PWM Channel 3 Counter Register"]
    #[inline(always)]
    pub const fn pwm3_cnt(&self) -> &Pwm3Cnt {
        &self.pwm3_cnt
    }
    #[doc = "0x34 - PWM Channel 3 Period Register/High Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm3_period_hpr(&self) -> &Pwm3PeriodHpr {
        &self.pwm3_period_hpr
    }
    #[doc = "0x38 - PWM Channel 3 Duty Register/Low Polarity Capture Register"]
    #[inline(always)]
    pub const fn pwm3_duty_lpr(&self) -> &Pwm3DutyLpr {
        &self.pwm3_duty_lpr
    }
    #[doc = "0x3c - PWM Channel 3 Control Register"]
    #[inline(always)]
    pub const fn pwm3_ctrl(&self) -> &Pwm3Ctrl {
        &self.pwm3_ctrl
    }
    #[doc = "0x40 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsts(&self) -> &Intsts {
        &self.intsts
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x50 - PWM Channel 3 FIFO Mode Control Register"]
    #[inline(always)]
    pub const fn pwm_fifo_ctrl(&self) -> &PwmFifoCtrl {
        &self.pwm_fifo_ctrl
    }
    #[doc = "0x54 - FIFO Interrupts Status Register"]
    #[inline(always)]
    pub const fn pwm_fifo_intsts(&self) -> &PwmFifoIntsts {
        &self.pwm_fifo_intsts
    }
    #[doc = "0x58 - FIFO Timeout Threshold Register"]
    #[inline(always)]
    pub const fn pwm_fifo_toutthr(&self) -> &PwmFifoToutthr {
        &self.pwm_fifo_toutthr
    }
    #[doc = "0x60..0x80 - FIFO Register"]
    #[inline(always)]
    pub const fn pwm_fifo(&self, n: usize) -> &PwmFifo {
        &self.pwm_fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - FIFO Register"]
    #[inline(always)]
    pub fn pwm_fifo_iter(&self) -> impl Iterator<Item = &PwmFifo> {
        self.pwm_fifo.iter()
    }
}
#[doc = "PWM0_CNT (r) register accessor: PWM Channel 0 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_cnt`]
module"]
#[doc(alias = "PWM0_CNT")]
pub type Pwm0Cnt = crate::Reg<pwm0_cnt::Pwm0CntSpec>;
#[doc = "PWM Channel 0 Counter Register"]
pub mod pwm0_cnt;
#[doc = "PWM0_PERIOD_HPR (rw) register accessor: PWM Channel 0 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_period_hpr`]
module"]
#[doc(alias = "PWM0_PERIOD_HPR")]
pub type Pwm0PeriodHpr = crate::Reg<pwm0_period_hpr::Pwm0PeriodHprSpec>;
#[doc = "PWM Channel 0 Period Register/High Polarity Capture Register"]
pub mod pwm0_period_hpr;
#[doc = "PWM0_DUTY_LPR (rw) register accessor: PWM Channel 0 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_duty_lpr`]
module"]
#[doc(alias = "PWM0_DUTY_LPR")]
pub type Pwm0DutyLpr = crate::Reg<pwm0_duty_lpr::Pwm0DutyLprSpec>;
#[doc = "PWM Channel 0 Duty Register/Low Polarity Capture Register"]
pub mod pwm0_duty_lpr;
#[doc = "PWM0_CTRL (rw) register accessor: PWM Channel 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_ctrl`]
module"]
#[doc(alias = "PWM0_CTRL")]
pub type Pwm0Ctrl = crate::Reg<pwm0_ctrl::Pwm0CtrlSpec>;
#[doc = "PWM Channel 0 Control Register"]
pub mod pwm0_ctrl;
#[doc = "PWM1_CNT (r) register accessor: PWM Channel 1 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_cnt`]
module"]
#[doc(alias = "PWM1_CNT")]
pub type Pwm1Cnt = crate::Reg<pwm1_cnt::Pwm1CntSpec>;
#[doc = "PWM Channel 1 Counter Register"]
pub mod pwm1_cnt;
#[doc = "PWM1_PERIOD_HPR (rw) register accessor: PWM Channel 1 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_period_hpr`]
module"]
#[doc(alias = "PWM1_PERIOD_HPR")]
pub type Pwm1PeriodHpr = crate::Reg<pwm1_period_hpr::Pwm1PeriodHprSpec>;
#[doc = "PWM Channel 1 Period Register/High Polarity Capture Register"]
pub mod pwm1_period_hpr;
#[doc = "PWM1_DUTY_LPR (rw) register accessor: PWM Channel 1 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_duty_lpr`]
module"]
#[doc(alias = "PWM1_DUTY_LPR")]
pub type Pwm1DutyLpr = crate::Reg<pwm1_duty_lpr::Pwm1DutyLprSpec>;
#[doc = "PWM Channel 1 Duty Register/Low Polarity Capture Register"]
pub mod pwm1_duty_lpr;
#[doc = "PWM1_CTRL (rw) register accessor: PWM Channel 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_ctrl`]
module"]
#[doc(alias = "PWM1_CTRL")]
pub type Pwm1Ctrl = crate::Reg<pwm1_ctrl::Pwm1CtrlSpec>;
#[doc = "PWM Channel 1 Control Register"]
pub mod pwm1_ctrl;
#[doc = "PWM2_CNT (r) register accessor: PWM Channel 2 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_cnt`]
module"]
#[doc(alias = "PWM2_CNT")]
pub type Pwm2Cnt = crate::Reg<pwm2_cnt::Pwm2CntSpec>;
#[doc = "PWM Channel 2 Counter Register"]
pub mod pwm2_cnt;
#[doc = "PWM2_PERIOD_HPR (rw) register accessor: PWM Channel 2 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm2_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_period_hpr`]
module"]
#[doc(alias = "PWM2_PERIOD_HPR")]
pub type Pwm2PeriodHpr = crate::Reg<pwm2_period_hpr::Pwm2PeriodHprSpec>;
#[doc = "PWM Channel 2 Period Register/High Polarity Capture Register"]
pub mod pwm2_period_hpr;
#[doc = "PWM2_DUTY_LPR (rw) register accessor: PWM Channel 2 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm2_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_duty_lpr`]
module"]
#[doc(alias = "PWM2_DUTY_LPR")]
pub type Pwm2DutyLpr = crate::Reg<pwm2_duty_lpr::Pwm2DutyLprSpec>;
#[doc = "PWM Channel 2 Duty Register/Low Polarity Capture Register"]
pub mod pwm2_duty_lpr;
#[doc = "PWM2_CTRL (rw) register accessor: PWM Channel 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_ctrl`]
module"]
#[doc(alias = "PWM2_CTRL")]
pub type Pwm2Ctrl = crate::Reg<pwm2_ctrl::Pwm2CtrlSpec>;
#[doc = "PWM Channel 2 Control Register"]
pub mod pwm2_ctrl;
#[doc = "PWM3_CNT (r) register accessor: PWM Channel 3 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm3_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_cnt`]
module"]
#[doc(alias = "PWM3_CNT")]
pub type Pwm3Cnt = crate::Reg<pwm3_cnt::Pwm3CntSpec>;
#[doc = "PWM Channel 3 Counter Register"]
pub mod pwm3_cnt;
#[doc = "PWM3_PERIOD_HPR (rw) register accessor: PWM Channel 3 Period Register/High Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm3_period_hpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm3_period_hpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_period_hpr`]
module"]
#[doc(alias = "PWM3_PERIOD_HPR")]
pub type Pwm3PeriodHpr = crate::Reg<pwm3_period_hpr::Pwm3PeriodHprSpec>;
#[doc = "PWM Channel 3 Period Register/High Polarity Capture Register"]
pub mod pwm3_period_hpr;
#[doc = "PWM3_DUTY_LPR (rw) register accessor: PWM Channel 3 Duty Register/Low Polarity Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm3_duty_lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm3_duty_lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_duty_lpr`]
module"]
#[doc(alias = "PWM3_DUTY_LPR")]
pub type Pwm3DutyLpr = crate::Reg<pwm3_duty_lpr::Pwm3DutyLprSpec>;
#[doc = "PWM Channel 3 Duty Register/Low Polarity Capture Register"]
pub mod pwm3_duty_lpr;
#[doc = "PWM3_CTRL (rw) register accessor: PWM Channel 3 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_ctrl`]
module"]
#[doc(alias = "PWM3_CTRL")]
pub type Pwm3Ctrl = crate::Reg<pwm3_ctrl::Pwm3CtrlSpec>;
#[doc = "PWM Channel 3 Control Register"]
pub mod pwm3_ctrl;
#[doc = "INTSTS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`]
module"]
#[doc(alias = "INTSTS")]
pub type Intsts = crate::Reg<intsts::IntstsSpec>;
#[doc = "Interrupt Status Register"]
pub mod intsts;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "Interrupt Enable Register"]
pub mod int_en;
#[doc = "PWM_FIFO_CTRL (rw) register accessor: PWM Channel 3 FIFO Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_fifo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_fifo_ctrl`]
module"]
#[doc(alias = "PWM_FIFO_CTRL")]
pub type PwmFifoCtrl = crate::Reg<pwm_fifo_ctrl::PwmFifoCtrlSpec>;
#[doc = "PWM Channel 3 FIFO Mode Control Register"]
pub mod pwm_fifo_ctrl;
#[doc = "PWM_FIFO_INTSTS (rw) register accessor: FIFO Interrupts Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo_intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_fifo_intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_fifo_intsts`]
module"]
#[doc(alias = "PWM_FIFO_INTSTS")]
pub type PwmFifoIntsts = crate::Reg<pwm_fifo_intsts::PwmFifoIntstsSpec>;
#[doc = "FIFO Interrupts Status Register"]
pub mod pwm_fifo_intsts;
#[doc = "PWM_FIFO_TOUTTHR (r) register accessor: FIFO Timeout Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo_toutthr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_fifo_toutthr`]
module"]
#[doc(alias = "PWM_FIFO_TOUTTHR")]
pub type PwmFifoToutthr = crate::Reg<pwm_fifo_toutthr::PwmFifoToutthrSpec>;
#[doc = "FIFO Timeout Threshold Register"]
pub mod pwm_fifo_toutthr;
#[doc = "PWM_FIFO (rw) register accessor: FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_fifo`]
module"]
#[doc(alias = "PWM_FIFO")]
pub type PwmFifo = crate::Reg<pwm_fifo::PwmFifoSpec>;
#[doc = "FIFO Register"]
pub mod pwm_fifo;
