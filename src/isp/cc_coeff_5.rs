#[doc = "Register `CC_COEFF_5` reader"]
pub type R = crate::R<CcCoeff5Spec>;
#[doc = "Register `CC_COEFF_5` writer"]
pub type W = crate::W<CcCoeff5Spec>;
#[doc = "Field `cc_coeff_5` reader - coefficient 5 for color space conversion"]
pub type CcCoeff5R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_5` writer - coefficient 5 for color space conversion"]
pub type CcCoeff5W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 5 for color space conversion"]
    #[inline(always)]
    pub fn cc_coeff_5(&self) -> CcCoeff5R {
        CcCoeff5R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 5 for color space conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_5(&mut self) -> CcCoeff5W<CcCoeff5Spec> {
        CcCoeff5W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff5Spec;
impl crate::RegisterSpec for CcCoeff5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_5::R`](R) reader structure"]
impl crate::Readable for CcCoeff5Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_5::W`](W) writer structure"]
impl crate::Writable for CcCoeff5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_5 to value 0x38"]
impl crate::Resettable for CcCoeff5Spec {
    const RESET_VALUE: u32 = 0x38;
}
