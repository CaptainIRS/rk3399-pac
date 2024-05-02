#[doc = "Register `CC_COEFF_0` reader"]
pub type R = crate::R<CcCoeff0Spec>;
#[doc = "Register `CC_COEFF_0` writer"]
pub type W = crate::W<CcCoeff0Spec>;
#[doc = "Field `cc_coeff_0` reader - coefficient 0 for color space conversion"]
pub type CcCoeff0R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_0` writer - coefficient 0 for color space conversion"]
pub type CcCoeff0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 0 for color space conversion"]
    #[inline(always)]
    pub fn cc_coeff_0(&self) -> CcCoeff0R {
        CcCoeff0R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 0 for color space conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_0(&mut self) -> CcCoeff0W<CcCoeff0Spec> {
        CcCoeff0W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 0\n\nNote: all color conversion coefficients are signed integer values with 7 bit fractional \n\n\n\npart, range -2 to 1.992 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff0Spec;
impl crate::RegisterSpec for CcCoeff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_0::R`](R) reader structure"]
impl crate::Readable for CcCoeff0Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_0::W`](W) writer structure"]
impl crate::Writable for CcCoeff0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_0 to value 0x21"]
impl crate::Resettable for CcCoeff0Spec {
    const RESET_VALUE: u32 = 0x21;
}
