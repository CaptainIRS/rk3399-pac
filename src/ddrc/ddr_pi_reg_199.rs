#[doc = "Register `DDR_PI_REG_199` reader"]
pub type R = crate::R<DdrPiReg199Spec>;
#[doc = "Register `DDR_PI_REG_199` writer"]
pub type W = crate::W<DdrPiReg199Spec>;
#[doc = "Field `PI_COL_DIFF` reader - Indicates the difference between the number of column pins\n\navailable and the number being used."]
pub type PiColDiffR = crate::FieldReader;
#[doc = "Field `PI_COL_DIFF` writer - Indicates the difference between the number of column pins\n\navailable and the number being used."]
pub type PiColDiffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Indicates the difference between the number of column pins\n\navailable and the number being used."]
    #[inline(always)]
    pub fn pi_col_diff(&self) -> PiColDiffR {
        PiColDiffR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the difference between the number of column pins\n\navailable and the number being used."]
    #[inline(always)]
    #[must_use]
    pub fn pi_col_diff(&mut self) -> PiColDiffW<DdrPiReg199Spec> {
        PiColDiffW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_199::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_199::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg199Spec;
impl crate::RegisterSpec for DdrPiReg199Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_199::R`](R) reader structure"]
impl crate::Readable for DdrPiReg199Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_199::W`](W) writer structure"]
impl crate::Writable for DdrPiReg199Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_199 to value 0"]
impl crate::Resettable for DdrPiReg199Spec {
    const RESET_VALUE: u32 = 0;
}
