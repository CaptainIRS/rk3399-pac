#[doc = "Register `LSC_XGRAD_23` reader"]
pub type R = crate::R<LscXgrad23Spec>;
#[doc = "Register `LSC_XGRAD_23` writer"]
pub type W = crate::W<LscXgrad23Spec>;
#[doc = "Field `xgrad_2` reader - factor for x-gradient calculation of sector 2\n\n"]
pub type Xgrad2R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_2` writer - factor for x-gradient calculation of sector 2\n\n"]
pub type Xgrad2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xgrad_3` reader - factor for x-gradient calculation of sector 3"]
pub type Xgrad3R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_3` writer - factor for x-gradient calculation of sector 3"]
pub type Xgrad3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 2\n\n"]
    #[inline(always)]
    pub fn xgrad_2(&self) -> Xgrad2R {
        Xgrad2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 3"]
    #[inline(always)]
    pub fn xgrad_3(&self) -> Xgrad3R {
        Xgrad3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 2\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_2(&mut self) -> Xgrad2W<LscXgrad23Spec> {
        Xgrad2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 3"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_3(&mut self) -> Xgrad3W<LscXgrad23Spec> {
        Xgrad3W::new(self, 16)
    }
}
#[doc = "Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXgrad23Spec;
impl crate::RegisterSpec for LscXgrad23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xgrad_23::R`](R) reader structure"]
impl crate::Readable for LscXgrad23Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xgrad_23::W`](W) writer structure"]
impl crate::Writable for LscXgrad23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XGRAD_23 to value 0"]
impl crate::Resettable for LscXgrad23Spec {
    const RESET_VALUE: u32 = 0;
}
