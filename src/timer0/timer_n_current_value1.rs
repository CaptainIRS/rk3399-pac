#[doc = "Register `TIMER_n_CURRENT_VALUE1` reader"]
pub type R = crate::R<TimerNCurrentValue1Spec>;
#[doc = "Field `CURRENT_CNT_HIGH_BITS` reader - High 32 bits of current value of timer n."]
pub type CurrentCntHighBitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - High 32 bits of current value of timer n."]
    #[inline(always)]
    pub fn current_cnt_high_bits(&self) -> CurrentCntHighBitsR {
        CurrentCntHighBitsR::new(self.bits)
    }
}
#[doc = "Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_current_value1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerNCurrentValue1Spec;
impl crate::RegisterSpec for TimerNCurrentValue1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_n_current_value1::R`](R) reader structure"]
impl crate::Readable for TimerNCurrentValue1Spec {}
#[doc = "`reset()` method sets TIMER_n_CURRENT_VALUE1 to value 0"]
impl crate::Resettable for TimerNCurrentValue1Spec {
    const RESET_VALUE: u32 = 0;
}
