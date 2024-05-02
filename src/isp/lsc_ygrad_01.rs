#[doc = "Register `LSC_YGRAD_01` reader"]
pub type R = crate::R<LscYgrad01Spec>;
#[doc = "Register `LSC_YGRAD_01` writer"]
pub type W = crate::W<LscYgrad01Spec>;
#[doc = "Field `ygrad_0` reader - factor for y-gradient calculation of sector 0\n\n"]
pub type Ygrad0R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_0` writer - factor for y-gradient calculation of sector 0\n\n"]
pub type Ygrad0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ygrad_1` reader - factor for y-gradient calculation of sector 1"]
pub type Ygrad1R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_1` writer - factor for y-gradient calculation of sector 1"]
pub type Ygrad1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 0\n\n"]
    #[inline(always)]
    pub fn ygrad_0(&self) -> Ygrad0R {
        Ygrad0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 1"]
    #[inline(always)]
    pub fn ygrad_1(&self) -> Ygrad1R {
        Ygrad1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 0\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_0(&mut self) -> Ygrad0W<LscYgrad01Spec> {
        Ygrad0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_1(&mut self) -> Ygrad1W<LscYgrad01Spec> {
        Ygrad1W::new(self, 16)
    }
}
#[doc = "Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYgrad01Spec;
impl crate::RegisterSpec for LscYgrad01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ygrad_01::R`](R) reader structure"]
impl crate::Readable for LscYgrad01Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ygrad_01::W`](W) writer structure"]
impl crate::Writable for LscYgrad01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YGRAD_01 to value 0"]
impl crate::Resettable for LscYgrad01Spec {
    const RESET_VALUE: u32 = 0;
}
