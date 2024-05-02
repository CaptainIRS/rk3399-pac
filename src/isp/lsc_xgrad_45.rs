#[doc = "Register `LSC_XGRAD_45` reader"]
pub type R = crate::R<LscXgrad45Spec>;
#[doc = "Register `LSC_XGRAD_45` writer"]
pub type W = crate::W<LscXgrad45Spec>;
#[doc = "Field `xgrad_4` reader - factor for x-gradient calculation of sector 4\n\n"]
pub type Xgrad4R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_4` writer - factor for x-gradient calculation of sector 4\n\n"]
pub type Xgrad4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xgrad_5` reader - factor for x-gradient calculation of sector 5"]
pub type Xgrad5R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_5` writer - factor for x-gradient calculation of sector 5"]
pub type Xgrad5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 4\n\n"]
    #[inline(always)]
    pub fn xgrad_4(&self) -> Xgrad4R {
        Xgrad4R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 5"]
    #[inline(always)]
    pub fn xgrad_5(&self) -> Xgrad5R {
        Xgrad5R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 4\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_4(&mut self) -> Xgrad4W<LscXgrad45Spec> {
        Xgrad4W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_5(&mut self) -> Xgrad5W<LscXgrad45Spec> {
        Xgrad5W::new(self, 16)
    }
}
#[doc = "Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXgrad45Spec;
impl crate::RegisterSpec for LscXgrad45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xgrad_45::R`](R) reader structure"]
impl crate::Readable for LscXgrad45Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xgrad_45::W`](W) writer structure"]
impl crate::Writable for LscXgrad45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XGRAD_45 to value 0"]
impl crate::Resettable for LscXgrad45Spec {
    const RESET_VALUE: u32 = 0;
}
