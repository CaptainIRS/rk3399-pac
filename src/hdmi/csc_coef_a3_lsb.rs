#[doc = "Register `CSC_COEF_A3_LSB` reader"]
pub type R = crate::R<CscCoefA3LsbSpec>;
#[doc = "Register `CSC_COEF_A3_LSB` writer"]
pub type W = crate::W<CscCoefA3LsbSpec>;
#[doc = "Field `CSC_COEF_A3_LSB` reader - Color Space Converter Matrix A3 Coefficient Register LSB"]
pub type CscCoefA3LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A3_LSB` writer - Color Space Converter Matrix A3 Coefficient Register LSB"]
pub type CscCoefA3LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A3 Coefficient Register LSB"]
    #[inline(always)]
    pub fn csc_coef_a3_lsb(&self) -> CscCoefA3LsbR {
        CscCoefA3LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A3 Coefficient Register LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a3_lsb(&mut self) -> CscCoefA3LsbW<CscCoefA3LsbSpec> {
        CscCoefA3LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A3 Coefficient Register LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a3_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a3_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA3LsbSpec;
impl crate::RegisterSpec for CscCoefA3LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a3_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefA3LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a3_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefA3LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A3_LSB to value 0"]
impl crate::Resettable for CscCoefA3LsbSpec {
    const RESET_VALUE: u8 = 0;
}
