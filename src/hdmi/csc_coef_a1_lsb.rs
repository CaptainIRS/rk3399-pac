#[doc = "Register `CSC_COEF_A1_LSB` reader"]
pub type R = crate::R<CscCoefA1LsbSpec>;
#[doc = "Register `CSC_COEF_A1_LSB` writer"]
pub type W = crate::W<CscCoefA1LsbSpec>;
#[doc = "Field `CSC_COEF_A1_LSB` reader - Color Space Converter Matrix A1 Coefficient\n\nRegister LSB"]
pub type CscCoefA1LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A1_LSB` writer - Color Space Converter Matrix A1 Coefficient\n\nRegister LSB"]
pub type CscCoefA1LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A1 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    pub fn csc_coef_a1_lsb(&self) -> CscCoefA1LsbR {
        CscCoefA1LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A1 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a1_lsb(&mut self) -> CscCoefA1LsbW<CscCoefA1LsbSpec> {
        CscCoefA1LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A1 Coefficient Register LSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a1_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a1_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA1LsbSpec;
impl crate::RegisterSpec for CscCoefA1LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a1_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefA1LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a1_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefA1LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A1_LSB to value 0"]
impl crate::Resettable for CscCoefA1LsbSpec {
    const RESET_VALUE: u8 = 0;
}
