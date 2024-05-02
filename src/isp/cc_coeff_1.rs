#[doc = "Register `CC_COEFF_1` reader"]
pub type R = crate::R<CcCoeff1Spec>;
#[doc = "Register `CC_COEFF_1` writer"]
pub type W = crate::W<CcCoeff1Spec>;
#[doc = "Field `cc_coeff_1` reader - coefficient 1 for color space conversion"]
pub type CcCoeff1R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_1` writer - coefficient 1 for color space conversion"]
pub type CcCoeff1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 1 for color space conversion"]
    #[inline(always)]
    pub fn cc_coeff_1(&self) -> CcCoeff1R {
        CcCoeff1R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 1 for color space conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_1(&mut self) -> CcCoeff1W<CcCoeff1Spec> {
        CcCoeff1W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff1Spec;
impl crate::RegisterSpec for CcCoeff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_1::R`](R) reader structure"]
impl crate::Readable for CcCoeff1Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_1::W`](W) writer structure"]
impl crate::Writable for CcCoeff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_1 to value 0x40"]
impl crate::Resettable for CcCoeff1Spec {
    const RESET_VALUE: u32 = 0x40;
}
