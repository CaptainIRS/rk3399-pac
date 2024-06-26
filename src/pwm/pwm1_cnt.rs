#[doc = "Register `PWM1_CNT` reader"]
pub type R = crate::R<Pwm1CntSpec>;
#[doc = "Field `CNT` reader - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 1 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Counter\n\nThe 32-bit indicates current value of PWM Channel 1 counter. The\n\ncounter runs at the rate of PWM clock.\n\nThe value ranges from 0 to (2^32-1)."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "PWM Channel 1 Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1CntSpec;
impl crate::RegisterSpec for Pwm1CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_cnt::R`](R) reader structure"]
impl crate::Readable for Pwm1CntSpec {}
#[doc = "`reset()` method sets PWM1_CNT to value 0"]
impl crate::Resettable for Pwm1CntSpec {
    const RESET_VALUE: u32 = 0;
}
