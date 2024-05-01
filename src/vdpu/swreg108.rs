#[doc = "Register `SWREG108` reader"]
pub type R = crate::R<Swreg108Spec>;
#[doc = "Register `SWREG108` writer"]
pub type W = crate::W<Swreg108Spec>;
#[doc = "Field `H264_REFPIC_VALID_FLAG` reader - valid flag for reference picture index"]
pub type H264RefpicValidFlagR = crate::FieldReader<u32>;
#[doc = "Field `H264_REFPIC_VALID_FLAG` writer - valid flag for reference picture index"]
pub type H264RefpicValidFlagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - valid flag for reference picture index"]
    #[inline(always)]
    pub fn h264_refpic_valid_flag(&self) -> H264RefpicValidFlagR {
        H264RefpicValidFlagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - valid flag for reference picture index"]
    #[inline(always)]
    #[must_use]
    pub fn h264_refpic_valid_flag(&mut self) -> H264RefpicValidFlagW<Swreg108Spec> {
        H264RefpicValidFlagW::new(self, 0)
    }
}
#[doc = "valid flag for reference picture index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg108::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg108Spec;
impl crate::RegisterSpec for Swreg108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg108::R`](R) reader structure"]
impl crate::Readable for Swreg108Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg108::W`](W) writer structure"]
impl crate::Writable for Swreg108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG108 to value 0"]
impl crate::Resettable for Swreg108Spec {
    const RESET_VALUE: u32 = 0;
}
