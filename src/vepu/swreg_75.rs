#[doc = "Register `SWREG_75` reader"]
pub type R = crate::R<Swreg75Spec>;
#[doc = "Register `SWREG_75` writer"]
pub type W = crate::W<Swreg75Spec>;
#[doc = "Field `INTERMOD` reader - the intra/inter selection for inter macro block mode favor\n\nthe intra/inter selection for inter macro block mode favor"]
pub type IntermodR = crate::FieldReader<u16>;
#[doc = "Field `INTERMOD` writer - the intra/inter selection for inter macro block mode favor\n\nthe intra/inter selection for inter macro block mode favor"]
pub type IntermodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INTRAMOD_16X16` reader - the Intra prediction for 16x16 mode favor"]
pub type Intramod16x16R = crate::FieldReader<u16>;
#[doc = "Field `INTRAMOD_16X16` writer - the Intra prediction for 16x16 mode favor"]
pub type Intramod16x16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the intra/inter selection for inter macro block mode favor\n\nthe intra/inter selection for inter macro block mode favor"]
    #[inline(always)]
    pub fn intermod(&self) -> IntermodR {
        IntermodR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the Intra prediction for 16x16 mode favor"]
    #[inline(always)]
    pub fn intramod_16x16(&self) -> Intramod16x16R {
        Intramod16x16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the intra/inter selection for inter macro block mode favor\n\nthe intra/inter selection for inter macro block mode favor"]
    #[inline(always)]
    #[must_use]
    pub fn intermod(&mut self) -> IntermodW<Swreg75Spec> {
        IntermodW::new(self, 0)
    }
    #[doc = "Bits 16:31 - the Intra prediction for 16x16 mode favor"]
    #[inline(always)]
    #[must_use]
    pub fn intramod_16x16(&mut self) -> Intramod16x16W<Swreg75Spec> {
        Intramod16x16W::new(self, 16)
    }
}
#[doc = "intra/inter mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg75Spec;
impl crate::RegisterSpec for Swreg75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_75::R`](R) reader structure"]
impl crate::Readable for Swreg75Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_75::W`](W) writer structure"]
impl crate::Writable for Swreg75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_75 to value 0"]
impl crate::Resettable for Swreg75Spec {
    const RESET_VALUE: u32 = 0;
}
