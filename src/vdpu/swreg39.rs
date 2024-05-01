#[doc = "Register `SWREG39` reader"]
pub type R = crate::R<Swreg39Spec>;
#[doc = "Register `SWREG39` writer"]
pub type W = crate::W<Swreg39Spec>;
#[doc = "Field `SW_DISPLAY_W` reader - the display width\n\nMax support 1920"]
pub type SwDisplayWR = crate::FieldReader<u16>;
#[doc = "Field `SW_DISPLAY_W` writer - the display width\n\nMax support 1920"]
pub type SwDisplayWW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - the display width\n\nMax support 1920"]
    #[inline(always)]
    pub fn sw_display_w(&self) -> SwDisplayWR {
        SwDisplayWR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - the display width\n\nMax support 1920"]
    #[inline(always)]
    #[must_use]
    pub fn sw_display_w(&mut self) -> SwDisplayWW<Swreg39Spec> {
        SwDisplayWW::new(self, 0)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg39Spec;
impl crate::RegisterSpec for Swreg39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg39::R`](R) reader structure"]
impl crate::Readable for Swreg39Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg39::W`](W) writer structure"]
impl crate::Writable for Swreg39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG39 to value 0"]
impl crate::Resettable for Swreg39Spec {
    const RESET_VALUE: u32 = 0;
}
