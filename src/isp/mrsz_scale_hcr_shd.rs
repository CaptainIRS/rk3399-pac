#[doc = "Register `MRSZ_SCALE_HCR_SHD` reader"]
pub type R = crate::R<MrszScaleHcrShdSpec>;
#[doc = "Field `scale_hcr_shd` reader - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor"]
pub type ScaleHcrShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor"]
    #[inline(always)]
    pub fn scale_hcr_shd(&self) -> ScaleHcrShdR {
        ScaleHcrShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal Cr scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcr_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleHcrShdSpec;
impl crate::RegisterSpec for MrszScaleHcrShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_hcr_shd::R`](R) reader structure"]
impl crate::Readable for MrszScaleHcrShdSpec {}
#[doc = "`reset()` method sets MRSZ_SCALE_HCR_SHD to value 0"]
impl crate::Resettable for MrszScaleHcrShdSpec {
    const RESET_VALUE: u32 = 0;
}
