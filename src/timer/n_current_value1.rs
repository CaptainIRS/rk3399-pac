#[doc = "Register `n_CURRENT_VALUE1` reader"]
pub type R = crate::R<NCurrentValue1Spec>;
#[doc = "Field `CURRENT_CNT_HIGH_BITS` reader - High 32 bits of current value of timer n."]
pub type CurrentCntHighBitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - High 32 bits of current value of timer n."]
    #[inline(always)]
    pub fn current_cnt_high_bits(&self) -> CurrentCntHighBitsR {
        CurrentCntHighBitsR::new(self.bits)
    }
}
#[doc = "Timer n Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_current_value1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCurrentValue1Spec;
impl crate::RegisterSpec for NCurrentValue1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_current_value1::R`](R) reader structure"]
impl crate::Readable for NCurrentValue1Spec {}
#[doc = "`reset()` method sets n_CURRENT_VALUE1 to value 0"]
impl crate::Resettable for NCurrentValue1Spec {
    const RESET_VALUE: u32 = 0;
}
