#[doc = "Register `AFM_LUM_C` reader"]
pub type R = crate::R<AfmLumCSpec>;
#[doc = "Field `afm_lum_c` reader - luminance value of window C\n\n"]
pub type AfmLumCR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - luminance value of window C\n\n"]
    #[inline(always)]
    pub fn afm_lum_c(&self) -> AfmLumCR {
        AfmLumCR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Luminance Value Status Register of Window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_c::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLumCSpec;
impl crate::RegisterSpec for AfmLumCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lum_c::R`](R) reader structure"]
impl crate::Readable for AfmLumCSpec {}
#[doc = "`reset()` method sets AFM_LUM_C to value 0"]
impl crate::Resettable for AfmLumCSpec {
    const RESET_VALUE: u32 = 0;
}
