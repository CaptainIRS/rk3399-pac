#[doc = "Register `CC_COEFF_7` reader"]
pub type R = crate::R<CcCoeff7Spec>;
#[doc = "Register `CC_COEFF_7` writer"]
pub type W = crate::W<CcCoeff7Spec>;
#[doc = "Field `cc_coeff_7` reader - coefficient 7 for color space conversion\n\n"]
pub type CcCoeff7R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_7` writer - coefficient 7 for color space conversion\n\n"]
pub type CcCoeff7W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 7 for color space conversion\n\n"]
    #[inline(always)]
    pub fn cc_coeff_7(&self) -> CcCoeff7R {
        CcCoeff7R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 7 for color space conversion\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_7(&mut self) -> CcCoeff7W<CcCoeff7Spec> {
        CcCoeff7W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff7Spec;
impl crate::RegisterSpec for CcCoeff7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_7::R`](R) reader structure"]
impl crate::Readable for CcCoeff7Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_7::W`](W) writer structure"]
impl crate::Writable for CcCoeff7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_7 to value 0x01d1"]
impl crate::Resettable for CcCoeff7Spec {
    const RESET_VALUE: u32 = 0x01d1;
}
