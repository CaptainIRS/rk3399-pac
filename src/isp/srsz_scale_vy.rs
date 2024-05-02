#[doc = "Register `SRSZ_SCALE_VY` reader"]
pub type R = crate::R<SrszScaleVySpec>;
#[doc = "Register `SRSZ_SCALE_VY` writer"]
pub type W = crate::W<SrszScaleVySpec>;
#[doc = "Field `scale_vy` reader - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
pub type ScaleVyR = crate::FieldReader<u16>;
#[doc = "Field `scale_vy` writer - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
pub type ScaleVyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
    #[inline(always)]
    pub fn scale_vy(&self) -> ScaleVyR {
        ScaleVyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance\n\ndownscale factor or to the reciprocal of the vertical\n\nluminance upscale factor\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vy(&mut self) -> ScaleVyW<SrszScaleVySpec> {
        ScaleVyW::new(self, 0)
    }
}
#[doc = "vertical luminance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_vy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleVySpec;
impl crate::RegisterSpec for SrszScaleVySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_vy::R`](R) reader structure"]
impl crate::Readable for SrszScaleVySpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_scale_vy::W`](W) writer structure"]
impl crate::Writable for SrszScaleVySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_SCALE_VY to value 0"]
impl crate::Resettable for SrszScaleVySpec {
    const RESET_VALUE: u32 = 0;
}
