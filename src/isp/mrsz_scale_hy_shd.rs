#[doc = "Register `MRSZ_SCALE_HY_SHD` reader"]
pub type R = crate::R<MrszScaleHyShdSpec>;
#[doc = "Field `scale_hy_shd` reader - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor\n\n"]
pub type ScaleHyShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance\n\ndownscale factor or to the reciprocal of the horizontal\n\nluminance upscale factor\n\n"]
    #[inline(always)]
    pub fn scale_hy_shd(&self) -> ScaleHyShdR {
        ScaleHyShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal luminance scale factor shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hy_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleHyShdSpec;
impl crate::RegisterSpec for MrszScaleHyShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_hy_shd::R`](R) reader structure"]
impl crate::Readable for MrszScaleHyShdSpec {}
#[doc = "`reset()` method sets MRSZ_SCALE_HY_SHD to value 0"]
impl crate::Resettable for MrszScaleHyShdSpec {
    const RESET_VALUE: u32 = 0;
}
