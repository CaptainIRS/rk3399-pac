#[doc = "Register `DDR_DENALI_PHY_277` reader"]
pub type R = crate::R<DdrDenaliPhy277Spec>;
#[doc = "Register `DDR_DENALI_PHY_277` writer"]
pub type W = crate::W<DdrDenaliPhy277Spec>;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_2` reader - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 2."]
pub type PhyRddqsDqEncObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_2` writer - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 2."]
pub type PhyRddqsDqEncObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_2` reader - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 2."]
pub type PhyWrEncObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_2` writer - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 2."]
pub type PhyWrEncObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_2` reader - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 2."]
pub type PhyWrShiftObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_2` writer - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 2."]
pub type PhyWrShiftObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_2` reader - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 2."]
pub type PhyFifoPtrObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_2` writer - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 2."]
pub type PhyFifoPtrObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq_enc_obs_select_2(&self) -> PhyRddqsDqEncObsSelect2R {
        PhyRddqsDqEncObsSelect2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 2."]
    #[inline(always)]
    pub fn phy_wr_enc_obs_select_2(&self) -> PhyWrEncObsSelect2R {
        PhyWrEncObsSelect2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 2."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_select_2(&self) -> PhyWrShiftObsSelect2R {
        PhyWrShiftObsSelect2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 2."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_select_2(&self) -> PhyFifoPtrObsSelect2R {
        PhyFifoPtrObsSelect2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_enc_obs_select_2(
        &mut self,
    ) -> PhyRddqsDqEncObsSelect2W<DdrDenaliPhy277Spec> {
        PhyRddqsDqEncObsSelect2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_enc_obs_select_2(&mut self) -> PhyWrEncObsSelect2W<DdrDenaliPhy277Spec> {
        PhyWrEncObsSelect2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_select_2(&mut self) -> PhyWrShiftObsSelect2W<DdrDenaliPhy277Spec> {
        PhyWrShiftObsSelect2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_select_2(&mut self) -> PhyFifoPtrObsSelect2W<DdrDenaliPhy277Spec> {
        PhyFifoPtrObsSelect2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_277::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_277::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy277Spec;
impl crate::RegisterSpec for DdrDenaliPhy277Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_277::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy277Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_277::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy277Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_277 to value 0"]
impl crate::Resettable for DdrDenaliPhy277Spec {
    const RESET_VALUE: u32 = 0;
}
