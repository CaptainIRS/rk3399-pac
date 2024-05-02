#[doc = "Register `CC_COEFF_4` reader"]
pub type R = crate::R<CcCoeff4Spec>;
#[doc = "Register `CC_COEFF_4` writer"]
pub type W = crate::W<CcCoeff4Spec>;
#[doc = "Field `cc_coeff_4` reader - coefficient 4 for color space conversion\n\n"]
pub type CcCoeff4R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_4` writer - coefficient 4 for color space conversion\n\n"]
pub type CcCoeff4W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 4 for color space conversion\n\n"]
    #[inline(always)]
    pub fn cc_coeff_4(&self) -> CcCoeff4R {
        CcCoeff4R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 4 for color space conversion\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_4(&mut self) -> CcCoeff4W<CcCoeff4Spec> {
        CcCoeff4W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff4Spec;
impl crate::RegisterSpec for CcCoeff4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_4::R`](R) reader structure"]
impl crate::Readable for CcCoeff4Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_4::W`](W) writer structure"]
impl crate::Writable for CcCoeff4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_4 to value 0x01db"]
impl crate::Resettable for CcCoeff4Spec {
    const RESET_VALUE: u32 = 0x01db;
}
