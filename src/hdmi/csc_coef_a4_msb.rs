#[doc = "Register `CSC_COEF_A4_MSB` reader"]
pub type R = crate::R<CscCoefA4MsbSpec>;
#[doc = "Register `CSC_COEF_A4_MSB` writer"]
pub type W = crate::W<CscCoefA4MsbSpec>;
#[doc = "Field `CSC_COEF_A4_MSB` reader - Color Space Converter Matrix A4 Coefficient Register MSB"]
pub type CscCoefA4MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A4_MSB` writer - Color Space Converter Matrix A4 Coefficient Register MSB"]
pub type CscCoefA4MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A4 Coefficient Register MSB"]
    #[inline(always)]
    pub fn csc_coef_a4_msb(&self) -> CscCoefA4MsbR {
        CscCoefA4MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A4 Coefficient Register MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a4_msb(&mut self) -> CscCoefA4MsbW<CscCoefA4MsbSpec> {
        CscCoefA4MsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A4 Coefficient Register MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a4_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a4_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA4MsbSpec;
impl crate::RegisterSpec for CscCoefA4MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a4_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefA4MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a4_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefA4MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A4_MSB to value 0"]
impl crate::Resettable for CscCoefA4MsbSpec {
    const RESET_VALUE: u8 = 0;
}
