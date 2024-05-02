#[doc = "Register `CC_COEFF_2` reader"]
pub type R = crate::R<CcCoeff2Spec>;
#[doc = "Register `CC_COEFF_2` writer"]
pub type W = crate::W<CcCoeff2Spec>;
#[doc = "Field `cc_coeff_2` reader - coefficient 2 for color space conversion\n\n"]
pub type CcCoeff2R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_2` writer - coefficient 2 for color space conversion\n\n"]
pub type CcCoeff2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 2 for color space conversion\n\n"]
    #[inline(always)]
    pub fn cc_coeff_2(&self) -> CcCoeff2R {
        CcCoeff2R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 2 for color space conversion\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_2(&mut self) -> CcCoeff2W<CcCoeff2Spec> {
        CcCoeff2W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff2Spec;
impl crate::RegisterSpec for CcCoeff2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_2::R`](R) reader structure"]
impl crate::Readable for CcCoeff2Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_2::W`](W) writer structure"]
impl crate::Writable for CcCoeff2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_2 to value 0x0d"]
impl crate::Resettable for CcCoeff2Spec {
    const RESET_VALUE: u32 = 0x0d;
}
