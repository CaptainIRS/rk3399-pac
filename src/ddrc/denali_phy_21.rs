#[doc = "Register `DENALI_PHY_21` reader"]
pub type R = crate::R<DenaliPhy21Spec>;
#[doc = "Register `DENALI_PHY_21` writer"]
pub type W = crate::W<DenaliPhy21Spec>;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_0` reader - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 0."]
pub type PhyRddqsDqEncObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_0` writer - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 0."]
pub type PhyRddqsDqEncObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_0` reader - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 0."]
pub type PhyWrEncObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_0` writer - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 0."]
pub type PhyWrEncObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_0` reader - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 0."]
pub type PhyWrShiftObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_0` writer - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 0."]
pub type PhyWrShiftObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_0` reader - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 0."]
pub type PhyFifoPtrObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_0` writer - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 0."]
pub type PhyFifoPtrObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq_enc_obs_select_0(&self) -> PhyRddqsDqEncObsSelect0R {
        PhyRddqsDqEncObsSelect0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 0."]
    #[inline(always)]
    pub fn phy_wr_enc_obs_select_0(&self) -> PhyWrEncObsSelect0R {
        PhyWrEncObsSelect0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 0."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_select_0(&self) -> PhyWrShiftObsSelect0R {
        PhyWrShiftObsSelect0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 0."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_select_0(&self) -> PhyFifoPtrObsSelect0R {
        PhyFifoPtrObsSelect0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_enc_obs_select_0(&mut self) -> PhyRddqsDqEncObsSelect0W<DenaliPhy21Spec> {
        PhyRddqsDqEncObsSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_enc_obs_select_0(&mut self) -> PhyWrEncObsSelect0W<DenaliPhy21Spec> {
        PhyWrEncObsSelect0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_select_0(&mut self) -> PhyWrShiftObsSelect0W<DenaliPhy21Spec> {
        PhyWrShiftObsSelect0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_select_0(&mut self) -> PhyFifoPtrObsSelect0W<DenaliPhy21Spec> {
        PhyFifoPtrObsSelect0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy21Spec;
impl crate::RegisterSpec for DenaliPhy21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_21::R`](R) reader structure"]
impl crate::Readable for DenaliPhy21Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_21::W`](W) writer structure"]
impl crate::Writable for DenaliPhy21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_21 to value 0"]
impl crate::Resettable for DenaliPhy21Spec {
    const RESET_VALUE: u32 = 0;
}
