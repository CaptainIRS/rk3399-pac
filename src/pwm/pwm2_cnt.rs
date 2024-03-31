#[doc = "Register `PWM2_CNT` reader"]
pub type R = crate::R<Pwm2CntSpec>;
#[doc = "Field `CNT` reader - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 2 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 2 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "PWM Channel 2 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2CntSpec;
impl crate::RegisterSpec for Pwm2CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2_cnt::R`](R) reader structure"]
impl crate::Readable for Pwm2CntSpec {}
#[doc = "`reset()` method sets PWM2_CNT to value 0"]
impl crate::Resettable for Pwm2CntSpec {
    const RESET_VALUE: u32 = 0;
}
