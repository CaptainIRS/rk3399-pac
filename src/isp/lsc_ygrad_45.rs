#[doc = "Register `LSC_YGRAD_45` reader"]
pub type R = crate::R<LscYgrad45Spec>;
#[doc = "Register `LSC_YGRAD_45` writer"]
pub type W = crate::W<LscYgrad45Spec>;
#[doc = "Field `ygrad_4` reader - factor for y-gradient calculation of sector 4\n\n"]
pub type Ygrad4R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_4` writer - factor for y-gradient calculation of sector 4\n\n"]
pub type Ygrad4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ygrad_5` reader - factor for y-gradient calculation of sector 5"]
pub type Ygrad5R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_5` writer - factor for y-gradient calculation of sector 5"]
pub type Ygrad5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 4\n\n"]
    #[inline(always)]
    pub fn ygrad_4(&self) -> Ygrad4R {
        Ygrad4R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 5"]
    #[inline(always)]
    pub fn ygrad_5(&self) -> Ygrad5R {
        Ygrad5R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 4\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_4(&mut self) -> Ygrad4W<LscYgrad45Spec> {
        Ygrad4W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_5(&mut self) -> Ygrad5W<LscYgrad45Spec> {
        Ygrad5W::new(self, 16)
    }
}
#[doc = "Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYgrad45Spec;
impl crate::RegisterSpec for LscYgrad45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ygrad_45::R`](R) reader structure"]
impl crate::Readable for LscYgrad45Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ygrad_45::W`](W) writer structure"]
impl crate::Writable for LscYgrad45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YGRAD_45 to value 0"]
impl crate::Resettable for LscYgrad45Spec {
    const RESET_VALUE: u32 = 0;
}
