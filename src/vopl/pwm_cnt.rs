#[doc = "Register `PWM_CNT` reader"]
pub type R = crate::R<PwmCntSpec>;
#[doc = "Field `PWM_CNT` reader - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 0 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
pub type PwmCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 0 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn pwm_cnt(&self) -> PwmCntR {
        PwmCntR::new(self.bits)
    }
}
#[doc = "PWM Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmCntSpec;
impl crate::RegisterSpec for PwmCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_cnt::R`](R) reader structure"]
impl crate::Readable for PwmCntSpec {}
#[doc = "`reset()` method sets PWM_CNT to value 0"]
impl crate::Resettable for PwmCntSpec {
    const RESET_VALUE: u32 = 0;
}
