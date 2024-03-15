#[doc = "Register `TIMER_n_CURRENT_VALUE0` reader"]
pub type R = crate::R<TimerNCurrentValue0Spec>;
#[doc = "Field `CURRENT_CNT_LOW_BITS` reader - Low 32 bits of current value of timer n."]
pub type CurrentCntLowBitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits of current value of timer n."]
    #[inline(always)]
    pub fn current_cnt_low_bits(&self) -> CurrentCntLowBitsR {
        CurrentCntLowBitsR::new(self.bits)
    }
}
#[doc = "Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_current_value0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerNCurrentValue0Spec;
impl crate::RegisterSpec for TimerNCurrentValue0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_n_current_value0::R`](R) reader structure"]
impl crate::Readable for TimerNCurrentValue0Spec {}
#[doc = "`reset()` method sets TIMER_n_CURRENT_VALUE0 to value 0"]
impl crate::Resettable for TimerNCurrentValue0Spec {
    const RESET_VALUE: u32 = 0;
}
