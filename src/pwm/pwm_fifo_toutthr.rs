#[doc = "Register `PWM_FIFO_TOUTTHR` reader"]
pub type R = crate::R<PwmFifoToutthrSpec>;
#[doc = "Field `TIMEOUT_THRESHOLD` reader - FIFO Timeout Value(unit pwmclk)"]
pub type TimeoutThresholdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - FIFO Timeout Value(unit pwmclk)"]
    #[inline(always)]
    pub fn timeout_threshold(&self) -> TimeoutThresholdR {
        TimeoutThresholdR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "FIFO Timeout Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_fifo_toutthr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmFifoToutthrSpec;
impl crate::RegisterSpec for PwmFifoToutthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_fifo_toutthr::R`](R) reader structure"]
impl crate::Readable for PwmFifoToutthrSpec {}
#[doc = "`reset()` method sets PWM_FIFO_TOUTTHR to value 0"]
impl crate::Resettable for PwmFifoToutthrSpec {
    const RESET_VALUE: u32 = 0;
}
