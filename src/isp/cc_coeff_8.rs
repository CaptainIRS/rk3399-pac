#[doc = "Register `CC_COEFF_8` reader"]
pub type R = crate::R<CcCoeff8Spec>;
#[doc = "Register `CC_COEFF_8` writer"]
pub type W = crate::W<CcCoeff8Spec>;
#[doc = "Field `cc_coeff_8` reader - coefficient 8 for color space conversion\n\n"]
pub type CcCoeff8R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_8` writer - coefficient 8 for color space conversion\n\n"]
pub type CcCoeff8W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 8 for color space conversion\n\n"]
    #[inline(always)]
    pub fn cc_coeff_8(&self) -> CcCoeff8R {
        CcCoeff8R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 8 for color space conversion\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_8(&mut self) -> CcCoeff8W<CcCoeff8Spec> {
        CcCoeff8W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff8Spec;
impl crate::RegisterSpec for CcCoeff8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_8::R`](R) reader structure"]
impl crate::Readable for CcCoeff8Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_8::W`](W) writer structure"]
impl crate::Writable for CcCoeff8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_8 to value 0x01f7"]
impl crate::Resettable for CcCoeff8Spec {
    const RESET_VALUE: u32 = 0x01f7;
}
