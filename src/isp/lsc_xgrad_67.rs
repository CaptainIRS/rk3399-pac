#[doc = "Register `LSC_XGRAD_67` reader"]
pub type R = crate::R<LscXgrad67Spec>;
#[doc = "Register `LSC_XGRAD_67` writer"]
pub type W = crate::W<LscXgrad67Spec>;
#[doc = "Field `xgrad_6` reader - factor for x-gradient calculation of sector 6\n\n"]
pub type Xgrad6R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_6` writer - factor for x-gradient calculation of sector 6\n\n"]
pub type Xgrad6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xgrad_7` reader - factor for x-gradient calculation of sector 7"]
pub type Xgrad7R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_7` writer - factor for x-gradient calculation of sector 7"]
pub type Xgrad7W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 6\n\n"]
    #[inline(always)]
    pub fn xgrad_6(&self) -> Xgrad6R {
        Xgrad6R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 7"]
    #[inline(always)]
    pub fn xgrad_7(&self) -> Xgrad7R {
        Xgrad7R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 6\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_6(&mut self) -> Xgrad6W<LscXgrad67Spec> {
        Xgrad6W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_7(&mut self) -> Xgrad7W<LscXgrad67Spec> {
        Xgrad7W::new(self, 16)
    }
}
#[doc = "Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXgrad67Spec;
impl crate::RegisterSpec for LscXgrad67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xgrad_67::R`](R) reader structure"]
impl crate::Readable for LscXgrad67Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xgrad_67::W`](W) writer structure"]
impl crate::Writable for LscXgrad67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XGRAD_67 to value 0"]
impl crate::Resettable for LscXgrad67Spec {
    const RESET_VALUE: u32 = 0;
}
