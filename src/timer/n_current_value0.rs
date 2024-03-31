#[doc = "Register `n_CURRENT_VALUE0` reader"]
pub type R = crate::R<NCurrentValue0Spec>;
#[doc = "Field `CURRENT_CNT_LOW_BITS` reader - Low 32 bits of current value of timer n."]
pub type CurrentCntLowBitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits of current value of timer n."]
    #[inline(always)]
    pub fn current_cnt_low_bits(&self) -> CurrentCntLowBitsR {
        CurrentCntLowBitsR::new(self.bits)
    }
}
#[doc = "Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_current_value0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCurrentValue0Spec;
impl crate::RegisterSpec for NCurrentValue0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_current_value0::R`](R) reader structure"]
impl crate::Readable for NCurrentValue0Spec {}
#[doc = "`reset()` method sets n_CURRENT_VALUE0 to value 0"]
impl crate::Resettable for NCurrentValue0Spec {
    const RESET_VALUE: u32 = 0;
}
