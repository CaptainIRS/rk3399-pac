#[doc = "Register `DENALI_PHY_84` reader"]
pub type R = crate::R<DenaliPhy84Spec>;
#[doc = "Register `DENALI_PHY_84` writer"]
pub type W = crate::W<DenaliPhy84Spec>;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_0` reader - Start/end timing values for DQS read based termination enable and select signals for slice 0."]
pub type PhyDqsTselRdTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_0` writer - Start/end timing values for DQS read based termination enable and select signals for slice 0."]
pub type PhyDqsTselRdTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_0` reader - Start/end timing values for DQS write based termination enable and select signals for slice 0."]
pub type PhyDqsTselWrTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_0` writer - Start/end timing values for DQS write based termination enable and select signals for slice 0."]
pub type PhyDqsTselWrTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_0` reader - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn0R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_0` writer - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DQ_IE_TIMING_0` reader - Start/end timing values for DQ/DM input enable signals for slice 0."]
pub type PhyDqIeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_0` writer - Start/end timing values for DQ/DM input enable signals for slice 0."]
pub type PhyDqIeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_tsel_rd_timing_0(&self) -> PhyDqsTselRdTiming0R {
        PhyDqsTselRdTiming0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_tsel_wr_timing_0(&self) -> PhyDqsTselWrTiming0R {
        PhyDqsTselWrTiming0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    pub fn phy_per_cs_training_en_0(&self) -> PhyPerCsTrainingEn0R {
        PhyPerCsTrainingEn0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_0(&self) -> PhyDqIeTiming0R {
        PhyDqIeTiming0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_rd_timing_0(&mut self) -> PhyDqsTselRdTiming0W<DenaliPhy84Spec> {
        PhyDqsTselRdTiming0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_wr_timing_0(&mut self) -> PhyDqsTselWrTiming0W<DenaliPhy84Spec> {
        PhyDqsTselWrTiming0W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_en_0(&mut self) -> PhyPerCsTrainingEn0W<DenaliPhy84Spec> {
        PhyPerCsTrainingEn0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_0(&mut self) -> PhyDqIeTiming0W<DenaliPhy84Spec> {
        PhyDqIeTiming0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy84Spec;
impl crate::RegisterSpec for DenaliPhy84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_84::R`](R) reader structure"]
impl crate::Readable for DenaliPhy84Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_84::W`](W) writer structure"]
impl crate::Writable for DenaliPhy84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_84 to value 0"]
impl crate::Resettable for DenaliPhy84Spec {
    const RESET_VALUE: u32 = 0;
}
