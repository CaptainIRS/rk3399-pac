#[doc = "Register `PWM_PWM_FIFO_TOUTTHR` reader"]
pub type R = crate::R<PwmPwmFifoToutthrSpec>;
#[doc = "Field `TIMEOUT_THRESHOLD` reader - FIFO Timeout Value(unit pwmclk)"]
pub type TimeoutThresholdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - FIFO Timeout Value(unit pwmclk)"]
    #[inline(always)]
    pub fn timeout_threshold(&self) -> TimeoutThresholdR {
        TimeoutThresholdR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "FIFO Timeout Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_fifo_toutthr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmPwmFifoToutthrSpec;
impl crate::RegisterSpec for PwmPwmFifoToutthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_pwm_fifo_toutthr::R`](R) reader structure"]
impl crate::Readable for PwmPwmFifoToutthrSpec {}
#[doc = "`reset()` method sets PWM_PWM_FIFO_TOUTTHR to value 0"]
impl crate::Resettable for PwmPwmFifoToutthrSpec {
    const RESET_VALUE: u32 = 0;
}
