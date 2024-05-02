#[doc = "Register `AFM_LUM_B` reader"]
pub type R = crate::R<AfmLumBSpec>;
#[doc = "Field `afm_lum_b` reader - luminance value of window B\n\n\n\n"]
pub type AfmLumBR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - luminance value of window B\n\n\n\n"]
    #[inline(always)]
    pub fn afm_lum_b(&self) -> AfmLumBR {
        AfmLumBR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Luminance Value Status Register of Window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lum_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLumBSpec;
impl crate::RegisterSpec for AfmLumBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lum_b::R`](R) reader structure"]
impl crate::Readable for AfmLumBSpec {}
#[doc = "`reset()` method sets AFM_LUM_B to value 0"]
impl crate::Resettable for AfmLumBSpec {
    const RESET_VALUE: u32 = 0;
}
