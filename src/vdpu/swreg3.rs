#[doc = "Register `SWREG3` reader"]
pub type R = crate::R<Swreg3Spec>;
#[doc = "Register `SWREG3` writer"]
pub type W = crate::W<Swreg3Spec>;
#[doc = "Field `SW_COE_6ST` reader - used for burightness adjust,used together with y pix"]
pub type SwCoe6stR = crate::FieldReader;
#[doc = "Field `SW_COE_6ST` writer - used for burightness adjust,used together with y pix"]
pub type SwCoe6stW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - used for burightness adjust,used together with y pix"]
    #[inline(always)]
    pub fn sw_coe_6st(&self) -> SwCoe6stR {
        SwCoe6stR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - used for burightness adjust,used together with y pix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_6st(&mut self) -> SwCoe6stW<Swreg3Spec> {
        SwCoe6stW::new(self, 0)
    }
}
#[doc = "color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg3Spec;
impl crate::RegisterSpec for Swreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg3::R`](R) reader structure"]
impl crate::Readable for Swreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg3::W`](W) writer structure"]
impl crate::Writable for Swreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG3 to value 0"]
impl crate::Resettable for Swreg3Spec {
    const RESET_VALUE: u32 = 0;
}
