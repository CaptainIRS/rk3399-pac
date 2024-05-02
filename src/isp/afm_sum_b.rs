#[doc = "Register `AFM_SUM_B` reader"]
pub type R = crate::R<AfmSumBSpec>;
#[doc = "Field `afm_sum_b` reader - sharpness value of window B\n\n"]
pub type AfmSumBR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sharpness value of window B\n\n"]
    #[inline(always)]
    pub fn afm_sum_b(&self) -> AfmSumBR {
        AfmSumBR::new(self.bits)
    }
}
#[doc = "Sharpness Value Status Register of Window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_sum_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmSumBSpec;
impl crate::RegisterSpec for AfmSumBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_sum_b::R`](R) reader structure"]
impl crate::Readable for AfmSumBSpec {}
#[doc = "`reset()` method sets AFM_SUM_B to value 0"]
impl crate::Resettable for AfmSumBSpec {
    const RESET_VALUE: u32 = 0;
}
