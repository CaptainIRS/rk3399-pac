#[doc = "Register `FC_AUDICONF0` reader"]
pub type R = crate::R<FcAudiconf0Spec>;
#[doc = "Register `FC_AUDICONF0` writer"]
pub type W = crate::W<FcAudiconf0Spec>;
#[doc = "Field `CT` reader - Coding Type"]
pub type CtR = crate::FieldReader;
#[doc = "Field `CT` writer - Coding Type"]
pub type CtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC` reader - Channel count"]
pub type CcR = crate::FieldReader;
#[doc = "Field `CC` writer - Channel count"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Coding Type"]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Channel count"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Coding Type"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CtW<FcAudiconf0Spec> {
        CtW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Channel count"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<FcAudiconf0Spec> {
        CcW::new(self, 4)
    }
}
#[doc = "Coding Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudiconf0Spec;
impl crate::RegisterSpec for FcAudiconf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audiconf0::R`](R) reader structure"]
impl crate::Readable for FcAudiconf0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audiconf0::W`](W) writer structure"]
impl crate::Writable for FcAudiconf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDICONF0 to value 0"]
impl crate::Resettable for FcAudiconf0Spec {
    const RESET_VALUE: u8 = 0;
}
