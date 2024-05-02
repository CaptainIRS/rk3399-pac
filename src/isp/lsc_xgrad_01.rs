#[doc = "Register `LSC_XGRAD_01` reader"]
pub type R = crate::R<LscXgrad01Spec>;
#[doc = "Register `LSC_XGRAD_01` writer"]
pub type W = crate::W<LscXgrad01Spec>;
#[doc = "Field `xgrad_0` reader - factor for x-gradient calculation of sector 0\n\n"]
pub type Xgrad0R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_0` writer - factor for x-gradient calculation of sector 0\n\n"]
pub type Xgrad0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xgrad_1` reader - factor for x-gradient calculation of sector 1"]
pub type Xgrad1R = crate::FieldReader<u16>;
#[doc = "Field `xgrad_1` writer - factor for x-gradient calculation of sector 1"]
pub type Xgrad1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 0\n\n"]
    #[inline(always)]
    pub fn xgrad_0(&self) -> Xgrad0R {
        Xgrad0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 1"]
    #[inline(always)]
    pub fn xgrad_1(&self) -> Xgrad1R {
        Xgrad1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - factor for x-gradient calculation of sector 0\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_0(&mut self) -> Xgrad0W<LscXgrad01Spec> {
        Xgrad0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - factor for x-gradient calculation of sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn xgrad_1(&mut self) -> Xgrad1W<LscXgrad01Spec> {
        Xgrad1W::new(self, 16)
    }
}
#[doc = "Gradient table x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xgrad_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xgrad_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXgrad01Spec;
impl crate::RegisterSpec for LscXgrad01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xgrad_01::R`](R) reader structure"]
impl crate::Readable for LscXgrad01Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xgrad_01::W`](W) writer structure"]
impl crate::Writable for LscXgrad01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XGRAD_01 to value 0"]
impl crate::Resettable for LscXgrad01Spec {
    const RESET_VALUE: u32 = 0;
}
