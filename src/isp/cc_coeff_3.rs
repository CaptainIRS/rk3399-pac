#[doc = "Register `CC_COEFF_3` reader"]
pub type R = crate::R<CcCoeff3Spec>;
#[doc = "Register `CC_COEFF_3` writer"]
pub type W = crate::W<CcCoeff3Spec>;
#[doc = "Field `cc_coeff_3` reader - coefficient 3 for color space conversion\n\n"]
pub type CcCoeff3R = crate::FieldReader<u16>;
#[doc = "Field `cc_coeff_3` writer - coefficient 3 for color space conversion\n\n"]
pub type CcCoeff3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - coefficient 3 for color space conversion\n\n"]
    #[inline(always)]
    pub fn cc_coeff_3(&self) -> CcCoeff3R {
        CcCoeff3R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - coefficient 3 for color space conversion\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cc_coeff_3(&mut self) -> CcCoeff3W<CcCoeff3Spec> {
        CcCoeff3W::new(self, 0)
    }
}
#[doc = "Color conversion coefficient 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_coeff_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_coeff_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcCoeff3Spec;
impl crate::RegisterSpec for CcCoeff3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_coeff_3::R`](R) reader structure"]
impl crate::Readable for CcCoeff3Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_coeff_3::W`](W) writer structure"]
impl crate::Writable for CcCoeff3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_COEFF_3 to value 0x01ed"]
impl crate::Resettable for CcCoeff3Spec {
    const RESET_VALUE: u32 = 0x01ed;
}
