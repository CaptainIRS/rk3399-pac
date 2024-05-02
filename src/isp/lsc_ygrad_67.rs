#[doc = "Register `LSC_YGRAD_67` reader"]
pub type R = crate::R<LscYgrad67Spec>;
#[doc = "Register `LSC_YGRAD_67` writer"]
pub type W = crate::W<LscYgrad67Spec>;
#[doc = "Field `ygrad_6` reader - factor for y-gradient calculation of sector 6\n\n"]
pub type Ygrad6R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_6` writer - factor for y-gradient calculation of sector 6\n\n"]
pub type Ygrad6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ygrad_7` reader - factor for y-gradient calculation of sector 7"]
pub type Ygrad7R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_7` writer - factor for y-gradient calculation of sector 7"]
pub type Ygrad7W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 6\n\n"]
    #[inline(always)]
    pub fn ygrad_6(&self) -> Ygrad6R {
        Ygrad6R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 7"]
    #[inline(always)]
    pub fn ygrad_7(&self) -> Ygrad7R {
        Ygrad7R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 6\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_6(&mut self) -> Ygrad6W<LscYgrad67Spec> {
        Ygrad6W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_7(&mut self) -> Ygrad7W<LscYgrad67Spec> {
        Ygrad7W::new(self, 16)
    }
}
#[doc = "Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYgrad67Spec;
impl crate::RegisterSpec for LscYgrad67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ygrad_67::R`](R) reader structure"]
impl crate::Readable for LscYgrad67Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ygrad_67::W`](W) writer structure"]
impl crate::Writable for LscYgrad67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YGRAD_67 to value 0"]
impl crate::Resettable for LscYgrad67Spec {
    const RESET_VALUE: u32 = 0;
}
