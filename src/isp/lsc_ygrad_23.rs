#[doc = "Register `LSC_YGRAD_23` reader"]
pub type R = crate::R<LscYgrad23Spec>;
#[doc = "Register `LSC_YGRAD_23` writer"]
pub type W = crate::W<LscYgrad23Spec>;
#[doc = "Field `ygrad_2` reader - factor for y-gradient calculation of sector 2\n\n"]
pub type Ygrad2R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_2` writer - factor for y-gradient calculation of sector 2\n\n"]
pub type Ygrad2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ygrad_3` reader - factor for y-gradient calculation of sector 3"]
pub type Ygrad3R = crate::FieldReader<u16>;
#[doc = "Field `ygrad_3` writer - factor for y-gradient calculation of sector 3"]
pub type Ygrad3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 2\n\n"]
    #[inline(always)]
    pub fn ygrad_2(&self) -> Ygrad2R {
        Ygrad2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 3"]
    #[inline(always)]
    pub fn ygrad_3(&self) -> Ygrad3R {
        Ygrad3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for y-gradient calculation of sector 2\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_2(&mut self) -> Ygrad2W<LscYgrad23Spec> {
        Ygrad2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for y-gradient calculation of sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn ygrad_3(&mut self) -> Ygrad3W<LscYgrad23Spec> {
        Ygrad3W::new(self, 16)
    }
}
#[doc = "Gradient table y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ygrad_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ygrad_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYgrad23Spec;
impl crate::RegisterSpec for LscYgrad23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ygrad_23::R`](R) reader structure"]
impl crate::Readable for LscYgrad23Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ygrad_23::W`](W) writer structure"]
impl crate::Writable for LscYgrad23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YGRAD_23 to value 0"]
impl crate::Resettable for LscYgrad23Spec {
    const RESET_VALUE: u32 = 0;
}
