#[doc = "Register `DDR_PI_REG_155` reader"]
pub type R = crate::R<DdrPiReg155Spec>;
#[doc = "Register `DDR_PI_REG_155` writer"]
pub type W = crate::W<DdrPiReg155Spec>;
#[doc = "Field `PI_BANK_DIFF` reader - Indicates encoded number of banks on the DRAM(s)."]
pub type PiBankDiffR = crate::FieldReader;
#[doc = "Field `PI_BANK_DIFF` writer - Indicates encoded number of banks on the DRAM(s)."]
pub type PiBankDiffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ROW_DIFF` reader - Indicates the difference between the number of address pins available and the number being used."]
pub type PiRowDiffR = crate::FieldReader;
#[doc = "Field `PI_ROW_DIFF` writer - Indicates the difference between the number of address pins available and the number being used."]
pub type PiRowDiffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 16:17 - Indicates encoded number of banks on the DRAM(s)."]
    #[inline(always)]
    pub fn pi_bank_diff(&self) -> PiBankDiffR {
        PiBankDiffR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Indicates the difference between the number of address pins available and the number being used."]
    #[inline(always)]
    pub fn pi_row_diff(&self) -> PiRowDiffR {
        PiRowDiffR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Indicates encoded number of banks on the DRAM(s)."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bank_diff(&mut self) -> PiBankDiffW<DdrPiReg155Spec> {
        PiBankDiffW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Indicates the difference between the number of address pins available and the number being used."]
    #[inline(always)]
    #[must_use]
    pub fn pi_row_diff(&mut self) -> PiRowDiffW<DdrPiReg155Spec> {
        PiRowDiffW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 155\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_155::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_155::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg155Spec;
impl crate::RegisterSpec for DdrPiReg155Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_155::R`](R) reader structure"]
impl crate::Readable for DdrPiReg155Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_155::W`](W) writer structure"]
impl crate::Writable for DdrPiReg155Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_155 to value 0"]
impl crate::Resettable for DdrPiReg155Spec {
    const RESET_VALUE: u32 = 0;
}
