#[doc = "Register `AFM_LUM_A` reader"]
pub type R = crate::R<AfmLumASpec>;
#[doc = "Field `afm_lum_a` reader - luminance value of window A\n\n"]
pub type AfmLumAR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - luminance value of window A\n\n"]
    #[inline(always)]
    pub fn afm_lum_a(&self) -> AfmLumAR {
        AfmLumAR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Luminance Value Status Register of Window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLumASpec;
impl crate::RegisterSpec for AfmLumASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lum_a::R`](R) reader structure"]
impl crate::Readable for AfmLumASpec {}
#[doc = "`reset()` method sets AFM_LUM_A to value 0"]
impl crate::Resettable for AfmLumASpec {
    const RESET_VALUE: u32 = 0;
}
