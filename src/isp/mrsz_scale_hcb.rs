#[doc = "Register `MRSZ_SCALE_HCB` reader"]
pub type R = crate::R<MrszScaleHcbSpec>;
#[doc = "Register `MRSZ_SCALE_HCB` writer"]
pub type W = crate::W<MrszScaleHcbSpec>;
#[doc = "Field `scale_hcb` reader - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
pub type ScaleHcbR = crate::FieldReader<u16>;
#[doc = "Field `scale_hcb` writer - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
pub type ScaleHcbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
    #[inline(always)]
    pub fn scale_hcb(&self) -> ScaleHcbR {
        ScaleHcbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the horizontal Cb downscale\n\nfactor or to the reciprocal of the horizontal Cb upscale\n\nfactor\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_hcb(&mut self) -> ScaleHcbW<MrszScaleHcbSpec> {
        ScaleHcbW::new(self, 0)
    }
}
#[doc = "horizontal Cb scale factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_hcb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_hcb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleHcbSpec;
impl crate::RegisterSpec for MrszScaleHcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_hcb::R`](R) reader structure"]
impl crate::Readable for MrszScaleHcbSpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_scale_hcb::W`](W) writer structure"]
impl crate::Writable for MrszScaleHcbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_SCALE_HCB to value 0"]
impl crate::Resettable for MrszScaleHcbSpec {
    const RESET_VALUE: u32 = 0;
}
