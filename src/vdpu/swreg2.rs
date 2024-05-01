#[doc = "Register `SWREG2` reader"]
pub type R = crate::R<Swreg2Spec>;
#[doc = "Register `SWREG2` writer"]
pub type W = crate::W<Swreg2Spec>;
#[doc = "Field `SW_COE_3ST` reader - used for green color components calculate,used together with cr\n\npix"]
pub type SwCoe3stR = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_3ST` writer - used for green color components calculate,used together with cr\n\npix"]
pub type SwCoe3stW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_COE_4ST` reader - used for green color components calculate,used together with cb\n\npix"]
pub type SwCoe4stR = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_4ST` writer - used for green color components calculate,used together with cb\n\npix"]
pub type SwCoe4stW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_COE_5ST` reader - used for blue components calculate,used together with cb pix"]
pub type SwCoe5stR = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_5ST` writer - used for blue components calculate,used together with cb pix"]
pub type SwCoe5stW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - used for green color components calculate,used together with cr\n\npix"]
    #[inline(always)]
    pub fn sw_coe_3st(&self) -> SwCoe3stR {
        SwCoe3stR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - used for green color components calculate,used together with cb\n\npix"]
    #[inline(always)]
    pub fn sw_coe_4st(&self) -> SwCoe4stR {
        SwCoe4stR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - used for blue components calculate,used together with cb pix"]
    #[inline(always)]
    pub fn sw_coe_5st(&self) -> SwCoe5stR {
        SwCoe5stR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - used for green color components calculate,used together with cr\n\npix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_3st(&mut self) -> SwCoe3stW<Swreg2Spec> {
        SwCoe3stW::new(self, 0)
    }
    #[doc = "Bits 10:19 - used for green color components calculate,used together with cb\n\npix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_4st(&mut self) -> SwCoe4stW<Swreg2Spec> {
        SwCoe4stW::new(self, 10)
    }
    #[doc = "Bits 20:29 - used for blue components calculate,used together with cb pix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_5st(&mut self) -> SwCoe5stW<Swreg2Spec> {
        SwCoe5stW::new(self, 20)
    }
}
#[doc = "color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg2Spec;
impl crate::RegisterSpec for Swreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg2::R`](R) reader structure"]
impl crate::Readable for Swreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg2::W`](W) writer structure"]
impl crate::Writable for Swreg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG2 to value 0"]
impl crate::Resettable for Swreg2Spec {
    const RESET_VALUE: u32 = 0;
}
