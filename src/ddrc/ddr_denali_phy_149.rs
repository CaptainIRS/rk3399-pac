#[doc = "Register `DDR_DENALI_PHY_149` reader"]
pub type R = crate::R<DdrDenaliPhy149Spec>;
#[doc = "Register `DDR_DENALI_PHY_149` writer"]
pub type W = crate::W<DdrDenaliPhy149Spec>;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_1` reader - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
pub type PhyRddqsDqEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_1` writer - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
pub type PhyRddqsDqEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_1` reader - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
pub type PhyWrEncObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_1` writer - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
pub type PhyWrEncObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_1` reader - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
pub type PhyWrShiftObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_1` writer - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
pub type PhyWrShiftObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_1` reader - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
pub type PhyFifoPtrObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_1` writer - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
pub type PhyFifoPtrObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq_enc_obs_select_1(&self) -> PhyRddqsDqEncObsSelect1R {
        PhyRddqsDqEncObsSelect1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    pub fn phy_wr_enc_obs_select_1(&self) -> PhyWrEncObsSelect1R {
        PhyWrEncObsSelect1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_select_1(&self) -> PhyWrShiftObsSelect1R {
        PhyWrShiftObsSelect1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_select_1(&self) -> PhyFifoPtrObsSelect1R {
        PhyFifoPtrObsSelect1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_enc_obs_select_1(
        &mut self,
    ) -> PhyRddqsDqEncObsSelect1W<DdrDenaliPhy149Spec> {
        PhyRddqsDqEncObsSelect1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_enc_obs_select_1(&mut self) -> PhyWrEncObsSelect1W<DdrDenaliPhy149Spec> {
        PhyWrEncObsSelect1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_select_1(&mut self) -> PhyWrShiftObsSelect1W<DdrDenaliPhy149Spec> {
        PhyWrShiftObsSelect1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_select_1(&mut self) -> PhyFifoPtrObsSelect1W<DdrDenaliPhy149Spec> {
        PhyFifoPtrObsSelect1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_149::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_149::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy149Spec;
impl crate::RegisterSpec for DdrDenaliPhy149Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_149::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy149Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_149::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy149Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_149 to value 0"]
impl crate::Resettable for DdrDenaliPhy149Spec {
    const RESET_VALUE: u32 = 0;
}
