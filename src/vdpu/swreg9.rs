#[doc = "Register `SWREG9` reader"]
pub type R = crate::R<Swreg9Spec>;
#[doc = "Register `SWREG9` writer"]
pub type W = crate::W<Swreg9Spec>;
#[doc = "Field `SW_MASK_R` reader - color R/(alpha channnel) component 's bit mask"]
pub type SwMaskRR = crate::FieldReader<u32>;
#[doc = "Field `SW_MASK_R` writer - color R/(alpha channnel) component 's bit mask"]
pub type SwMaskRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - color R/(alpha channnel) component 's bit mask"]
    #[inline(always)]
    pub fn sw_mask_r(&self) -> SwMaskRR {
        SwMaskRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - color R/(alpha channnel) component 's bit mask"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask_r(&mut self) -> SwMaskRW<Swreg9Spec> {
        SwMaskRW::new(self, 0)
    }
}
#[doc = "Rmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg9Spec;
impl crate::RegisterSpec for Swreg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg9::R`](R) reader structure"]
impl crate::Readable for Swreg9Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg9::W`](W) writer structure"]
impl crate::Writable for Swreg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG9 to value 0"]
impl crate::Resettable for Swreg9Spec {
    const RESET_VALUE: u32 = 0;
}
