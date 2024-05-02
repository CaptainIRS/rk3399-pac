#[doc = "Register `AFM_SUM_C` reader"]
pub type R = crate::R<AfmSumCSpec>;
#[doc = "Field `afm_sum_c` reader - sharpness value of window C\n\n"]
pub type AfmSumCR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sharpness value of window C\n\n"]
    #[inline(always)]
    pub fn afm_sum_c(&self) -> AfmSumCR {
        AfmSumCR::new(self.bits)
    }
}
#[doc = "Sharpness Value Status Register of Window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_c::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmSumCSpec;
impl crate::RegisterSpec for AfmSumCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_sum_c::R`](R) reader structure"]
impl crate::Readable for AfmSumCSpec {}
#[doc = "`reset()` method sets AFM_SUM_C to value 0"]
impl crate::Resettable for AfmSumCSpec {
    const RESET_VALUE: u32 = 0;
}
