#[doc = "Register `AFM_SUM_A` reader"]
pub type R = crate::R<AfmSumASpec>;
#[doc = "Field `afm_sum_a` reader - sharpness value of window A\n\n"]
pub type AfmSumAR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sharpness value of window A\n\n"]
    #[inline(always)]
    pub fn afm_sum_a(&self) -> AfmSumAR {
        AfmSumAR::new(self.bits)
    }
}
#[doc = "Sharpness Value Status Register of Window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmSumASpec;
impl crate::RegisterSpec for AfmSumASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_sum_a::R`](R) reader structure"]
impl crate::Readable for AfmSumASpec {}
#[doc = "`reset()` method sets AFM_SUM_A to value 0"]
impl crate::Resettable for AfmSumASpec {
    const RESET_VALUE: u32 = 0;
}
