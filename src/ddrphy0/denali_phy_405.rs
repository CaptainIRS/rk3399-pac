#[doc = "Register `DENALI_PHY_405` reader"]
pub type R = crate::R<DenaliPhy405Spec>;
#[doc = "Register `DENALI_PHY_405` writer"]
pub type W = crate::W<DenaliPhy405Spec>;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_3` reader - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 3."]
pub type PhyRddqsDqEncObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_ENC_OBS_SELECT_3` writer - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 3."]
pub type PhyRddqsDqEncObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_3` reader - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 3."]
pub type PhyWrEncObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_WR_ENC_OBS_SELECT_3` writer - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 3."]
pub type PhyWrEncObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_3` reader - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 3."]
pub type PhyWrShiftObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_SELECT_3` writer - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 3."]
pub type PhyWrShiftObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_3` reader - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 3."]
pub type PhyFifoPtrObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_SELECT_3` writer - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 3."]
pub type PhyFifoPtrObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq_enc_obs_select_3(&self) -> PhyRddqsDqEncObsSelect3R {
        PhyRddqsDqEncObsSelect3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 3."]
    #[inline(always)]
    pub fn phy_wr_enc_obs_select_3(&self) -> PhyWrEncObsSelect3R {
        PhyWrEncObsSelect3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 3."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_select_3(&self) -> PhyWrShiftObsSelect3R {
        PhyWrShiftObsSelect3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 3."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_select_3(&self) -> PhyFifoPtrObsSelect3R {
        PhyFifoPtrObsSelect3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map the internal read DQS DQ rise/fall slave delay encoded settings to the accessible read DQS DQ rise/fall encoded slave delay observation registers for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_enc_obs_select_3(&mut self) -> PhyRddqsDqEncObsSelect3W<DenaliPhy405Spec> {
        PhyRddqsDqEncObsSelect3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select value to map the internal write DQ slave delay encoded settings to the accessible write DQ encoded slave delay observation register for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_enc_obs_select_3(&mut self) -> PhyWrEncObsSelect3W<DenaliPhy405Spec> {
        PhyWrEncObsSelect3W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Select value to map the internal write DQ/DQS automatic cycle/ half_cycle shift settings to the accessible write DQ/DQS shift observation register for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wr_shift_obs_select_3(&mut self) -> PhyWrShiftObsSelect3W<DenaliPhy405Spec> {
        PhyWrShiftObsSelect3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Select value to map the internal read entry FIFO read/write pointers to the accessible read entry FIFO pointer observation register for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_select_3(&mut self) -> PhyFifoPtrObsSelect3W<DenaliPhy405Spec> {
        PhyFifoPtrObsSelect3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_405::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_405::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy405Spec;
impl crate::RegisterSpec for DenaliPhy405Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_405::R`](R) reader structure"]
impl crate::Readable for DenaliPhy405Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_405::W`](W) writer structure"]
impl crate::Writable for DenaliPhy405Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_405 to value 0"]
impl crate::Resettable for DenaliPhy405Spec {
    const RESET_VALUE: u32 = 0;
}
