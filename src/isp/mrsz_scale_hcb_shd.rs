#[doc = "Register `MRSZ_SCALE_HCB_SHD` reader"]
pub type R = crate::R<MrszScaleHcbShdSpec>;
#[doc = "Field `scale_hcb_shd` reader - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
pub type ScaleHcbShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
    #[inline(always)]
    pub fn scale_hcb_shd(&self) -> ScaleHcbShdR {
        ScaleHcbShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal Cb scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcb_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleHcbShdSpec;
impl crate::RegisterSpec for MrszScaleHcbShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_hcb_shd::R`](R) reader structure"]
impl crate::Readable for MrszScaleHcbShdSpec {}
#[doc = "`reset()` method sets MRSZ_SCALE_HCB_SHD to value 0"]
impl crate::Resettable for MrszScaleHcbShdSpec {
    const RESET_VALUE: u32 = 0;
}
