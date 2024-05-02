#[doc = "Register `SRSZ_SCALE_HCR` reader"]
pub type R = crate::R<SrszScaleHcrSpec>;
#[doc = "Register `SRSZ_SCALE_HCR` writer"]
pub type W = crate::W<SrszScaleHcrSpec>;
#[doc = "Field `scale_hcr` reader - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor\n\n"]
pub type ScaleHcrR = crate::FieldReader<u16>;
#[doc = "Field `scale_hcr` writer - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor\n\n"]
pub type ScaleHcrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor\n\n"]
    #[inline(always)]
    pub fn scale_hcr(&self) -> ScaleHcrR {
        ScaleHcrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cr downscale\n\nfactor or to the reciprocal of the horizontal Cr upscale\n\nfactor\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hcr(&mut self) -> ScaleHcrW<SrszScaleHcrSpec> {
        ScaleHcrW::new(self, 0)
    }
}
#[doc = "horizontal chrominance scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_hcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_hcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleHcrSpec;
impl crate::RegisterSpec for SrszScaleHcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_hcr::R`](R) reader structure"]
impl crate::Readable for SrszScaleHcrSpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_scale_hcr::W`](W) writer structure"]
impl crate::Writable for SrszScaleHcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_SCALE_HCR to value 0"]
impl crate::Resettable for SrszScaleHcrSpec {
    const RESET_VALUE: u32 = 0;
}
