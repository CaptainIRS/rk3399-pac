#[doc = "Register `SBCR` reader"]
pub type R = crate::R<SbcrSpec>;
#[doc = "Register `SBCR` writer"]
pub type W = crate::W<SbcrSpec>;
#[doc = "Field `SHADOW_BREAK_CTRL` reader - Shadow Break Control Bit.\n\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be\n\nused to remove the burden of having to performing a read modify\n\nwrite on the LCR."]
pub type ShadowBreakCtrlR = crate::BitReader;
#[doc = "Field `SHADOW_BREAK_CTRL` writer - Shadow Break Control Bit.\n\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be\n\nused to remove the burden of having to performing a read modify\n\nwrite on the LCR."]
pub type ShadowBreakCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow Break Control Bit.\n\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be\n\nused to remove the burden of having to performing a read modify\n\nwrite on the LCR."]
    #[inline(always)]
    pub fn shadow_break_ctrl(&self) -> ShadowBreakCtrlR {
        ShadowBreakCtrlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Break Control Bit.\n\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be\n\nused to remove the burden of having to performing a read modify\n\nwrite on the LCR."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_break_ctrl(&mut self) -> ShadowBreakCtrlW<SbcrSpec> {
        ShadowBreakCtrlW::new(self, 0)
    }
}
#[doc = "Shadow Break Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbcrSpec;
impl crate::RegisterSpec for SbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcr::R`](R) reader structure"]
impl crate::Readable for SbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sbcr::W`](W) writer structure"]
impl crate::Writable for SbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBCR to value 0"]
impl crate::Resettable for SbcrSpec {
    const RESET_VALUE: u32 = 0;
}
