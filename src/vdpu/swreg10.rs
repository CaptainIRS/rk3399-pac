#[doc = "Register `SWREG10` reader"]
pub type R = crate::R<Swreg10Spec>;
#[doc = "Register `SWREG10` writer"]
pub type W = crate::W<Swreg10Spec>;
#[doc = "Field `SW_MASK_G` reader - color G/(alpha channnel) component 's bit mask"]
pub type SwMaskGR = crate::FieldReader<u32>;
#[doc = "Field `SW_MASK_G` writer - color G/(alpha channnel) component 's bit mask"]
pub type SwMaskGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - color G/(alpha channnel) component 's bit mask"]
    #[inline(always)]
    pub fn sw_mask_g(&self) -> SwMaskGR {
        SwMaskGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - color G/(alpha channnel) component 's bit mask"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask_g(&mut self) -> SwMaskGW<Swreg10Spec> {
        SwMaskGW::new(self, 0)
    }
}
#[doc = "Gmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg10Spec;
impl crate::RegisterSpec for Swreg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg10::R`](R) reader structure"]
impl crate::Readable for Swreg10Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg10::W`](W) writer structure"]
impl crate::Writable for Swreg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG10 to value 0"]
impl crate::Resettable for Swreg10Spec {
    const RESET_VALUE: u32 = 0;
}
