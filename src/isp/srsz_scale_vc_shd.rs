#[doc = "Register `SRSZ_SCALE_VC_SHD` reader"]
pub type R = crate::R<SrszScaleVcShdSpec>;
#[doc = "Field `scale_vc_shd` reader - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor\n\n"]
pub type ScaleVcShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor\n\n"]
    #[inline(always)]
    pub fn scale_vc_shd(&self) -> ScaleVcShdR {
        ScaleVcShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "vertical chrominance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vc_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleVcShdSpec;
impl crate::RegisterSpec for SrszScaleVcShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_vc_shd::R`](R) reader structure"]
impl crate::Readable for SrszScaleVcShdSpec {}
#[doc = "`reset()` method sets SRSZ_SCALE_VC_SHD to value 0"]
impl crate::Resettable for SrszScaleVcShdSpec {
    const RESET_VALUE: u32 = 0;
}
