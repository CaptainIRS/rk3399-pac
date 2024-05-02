#[doc = "Register `SRSZ_SCALE_VY_SHD` reader"]
pub type R = crate::R<SrszScaleVyShdSpec>;
#[doc = "Field `scale_vy_shd` reader - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
pub type ScaleVyShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
    #[inline(always)]
    pub fn scale_vy_shd(&self) -> ScaleVyShdR {
        ScaleVyShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "vertical luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vy_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleVyShdSpec;
impl crate::RegisterSpec for SrszScaleVyShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_vy_shd::R`](R) reader structure"]
impl crate::Readable for SrszScaleVyShdSpec {}
#[doc = "`reset()` method sets SRSZ_SCALE_VY_SHD to value 0"]
impl crate::Resettable for SrszScaleVyShdSpec {
    const RESET_VALUE: u32 = 0;
}
