#[doc = "Register `SRSZ_SCALE_VC` reader"]
pub type R = crate::R<SrszScaleVcSpec>;
#[doc = "Register `SRSZ_SCALE_VC` writer"]
pub type W = crate::W<SrszScaleVcSpec>;
#[doc = "Field `scale_vc` reader - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
pub type ScaleVcR = crate::FieldReader<u16>;
#[doc = "Field `scale_vc` writer - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
pub type ScaleVcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
    #[inline(always)]
    pub fn scale_vc(&self) -> ScaleVcR {
        ScaleVcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vc(&mut self) -> ScaleVcW<SrszScaleVcSpec> {
        ScaleVcW::new(self, 0)
    }
}
#[doc = "vertical chrominance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_vc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_vc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleVcSpec;
impl crate::RegisterSpec for SrszScaleVcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_vc::R`](R) reader structure"]
impl crate::Readable for SrszScaleVcSpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_scale_vc::W`](W) writer structure"]
impl crate::Writable for SrszScaleVcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_SCALE_VC to value 0"]
impl crate::Resettable for SrszScaleVcSpec {
    const RESET_VALUE: u32 = 0;
}
