#[doc = "Register `SWREG59` reader"]
pub type R = crate::R<Swreg59Spec>;
#[doc = "Register `SWREG59` writer"]
pub type W = crate::W<Swreg59Spec>;
#[doc = "Field `SW_PFLT_SET0_TAP2` reader - Prediction filter\n\nset 0, tap 2"]
pub type SwPfltSet0Tap2R = crate::FieldReader<u16>;
#[doc = "Field `SW_PFLT_SET0_TAP2` writer - Prediction filter\n\nset 0, tap 2"]
pub type SwPfltSet0Tap2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_PFLT_SET0_TAP1` reader - Prediction filter\n\nset 0, tap 1"]
pub type SwPfltSet0Tap1R = crate::FieldReader<u16>;
#[doc = "Field `SW_PFLT_SET0_TAP1` writer - Prediction filter\n\nset 0, tap 1"]
pub type SwPfltSet0Tap1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_PFLT_SET0_TAP0` reader - Prediction filter\n\n0, tap 0"]
pub type SwPfltSet0Tap0R = crate::FieldReader<u16>;
#[doc = "Field `SW_PFLT_SET0_TAP0` writer - Prediction filter\n\n0, tap 0"]
pub type SwPfltSet0Tap0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 2:11 - Prediction filter\n\nset 0, tap 2"]
    #[inline(always)]
    pub fn sw_pflt_set0_tap2(&self) -> SwPfltSet0Tap2R {
        SwPfltSet0Tap2R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Prediction filter\n\nset 0, tap 1"]
    #[inline(always)]
    pub fn sw_pflt_set0_tap1(&self) -> SwPfltSet0Tap1R {
        SwPfltSet0Tap1R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - Prediction filter\n\n0, tap 0"]
    #[inline(always)]
    pub fn sw_pflt_set0_tap0(&self) -> SwPfltSet0Tap0R {
        SwPfltSet0Tap0R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - Prediction filter\n\nset 0, tap 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pflt_set0_tap2(&mut self) -> SwPfltSet0Tap2W<Swreg59Spec> {
        SwPfltSet0Tap2W::new(self, 2)
    }
    #[doc = "Bits 12:21 - Prediction filter\n\nset 0, tap 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pflt_set0_tap1(&mut self) -> SwPfltSet0Tap1W<Swreg59Spec> {
        SwPfltSet0Tap1W::new(self, 12)
    }
    #[doc = "Bits 22:31 - Prediction filter\n\n0, tap 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pflt_set0_tap0(&mut self) -> SwPfltSet0Tap0W<Swreg59Spec> {
        SwPfltSet0Tap0W::new(self, 22)
    }
}
#[doc = "H264, MPEG4, VP6 Prediction filter tap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg59Spec;
impl crate::RegisterSpec for Swreg59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg59::R`](R) reader structure"]
impl crate::Readable for Swreg59Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg59::W`](W) writer structure"]
impl crate::Writable for Swreg59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG59 to value 0"]
impl crate::Resettable for Swreg59Spec {
    const RESET_VALUE: u32 = 0;
}
