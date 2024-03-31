#[doc = "Register `DDR_PI_REG_89` reader"]
pub type R = crate::R<DdrPiReg89Spec>;
#[doc = "Register `DDR_PI_REG_89` writer"]
pub type W = crate::W<DdrPiReg89Spec>;
#[doc = "Field `PI_RD_PREAMBLE_TRAINING_EN` reader - Enables read preamble training during data eye training. Set to 1 to\n\nenable."]
pub type PiRdPreambleTrainingEnR = crate::BitReader;
#[doc = "Field `PI_RD_PREAMBLE_TRAINING_EN` writer - Enables read preamble training during data eye training. Set to 1 to\n\nenable."]
pub type PiRdPreambleTrainingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_REG_DIMM_ENABLE` reader - Enables registered DIMM operation. Set to 1 to enable."]
pub type PiRegDimmEnableR = crate::BitReader;
#[doc = "Field `PI_REG_DIMM_ENABLE` writer - Enables registered DIMM operation. Set to 1 to enable."]
pub type PiRegDimmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_ADJ_F0` reader - Indicates the adjustment value for PHY read timing. The suffix '_f0'\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF0R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_ADJ_F0` writer - Indicates the adjustment value for PHY read timing. The suffix '_f0'\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_RDLVL_ADJ_F1` reader - Indicates the adjustment value for PHY read timing. The suffix '_f1'\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_ADJ_F1` writer - Indicates the adjustment value for PHY read timing. The suffix '_f1'\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enables read preamble training during data eye training. Set to 1 to\n\nenable."]
    #[inline(always)]
    pub fn pi_rd_preamble_training_en(&self) -> PiRdPreambleTrainingEnR {
        PiRdPreambleTrainingEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables registered DIMM operation. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_reg_dimm_enable(&self) -> PiRegDimmEnableR {
        PiRegDimmEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Indicates the adjustment value for PHY read timing. The suffix '_f0'\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rdlvl_adj_f0(&self) -> PiRdlvlAdjF0R {
        PiRdlvlAdjF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the adjustment value for PHY read timing. The suffix '_f1'\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rdlvl_adj_f1(&self) -> PiRdlvlAdjF1R {
        PiRdlvlAdjF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables read preamble training during data eye training. Set to 1 to\n\nenable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_preamble_training_en(&mut self) -> PiRdPreambleTrainingEnW<DdrPiReg89Spec> {
        PiRdPreambleTrainingEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables registered DIMM operation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_reg_dimm_enable(&mut self) -> PiRegDimmEnableW<DdrPiReg89Spec> {
        PiRegDimmEnableW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates the adjustment value for PHY read timing. The suffix '_f0'\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_adj_f0(&mut self) -> PiRdlvlAdjF0W<DdrPiReg89Spec> {
        PiRdlvlAdjF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates the adjustment value for PHY read timing. The suffix '_f1'\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_adj_f1(&mut self) -> PiRdlvlAdjF1W<DdrPiReg89Spec> {
        PiRdlvlAdjF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_89::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_89::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg89Spec;
impl crate::RegisterSpec for DdrPiReg89Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_89::R`](R) reader structure"]
impl crate::Readable for DdrPiReg89Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_89::W`](W) writer structure"]
impl crate::Writable for DdrPiReg89Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_89 to value 0"]
impl crate::Resettable for DdrPiReg89Spec {
    const RESET_VALUE: u32 = 0;
}
