#[doc = "Register `DDR_DENALI_PHY_340` reader"]
pub type R = crate::R<DdrDenaliPhy340Spec>;
#[doc = "Register `DDR_DENALI_PHY_340` writer"]
pub type W = crate::W<DdrDenaliPhy340Spec>;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_2` reader - Start/end timing values for DQS read based termination enable and select signals for slice 2."]
pub type PhyDqsTselRdTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_2` writer - Start/end timing values for DQS read based termination enable and select signals for slice 2."]
pub type PhyDqsTselRdTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_2` reader - Start/end timing values for DQS write based termination enable and select signals for slice 2."]
pub type PhyDqsTselWrTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_2` writer - Start/end timing values for DQS write based termination enable and select signals for slice 2."]
pub type PhyDqsTselWrTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_2` reader - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn2R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_2` writer - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DQ_IE_TIMING_2` reader - Start/end timing values for DQ/DM input enable signals for slice 2."]
pub type PhyDqIeTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_2` writer - Start/end timing values for DQ/DM input enable signals for slice 2."]
pub type PhyDqIeTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 2."]
    #[inline(always)]
    pub fn phy_dqs_tsel_rd_timing_2(&self) -> PhyDqsTselRdTiming2R {
        PhyDqsTselRdTiming2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 2."]
    #[inline(always)]
    pub fn phy_dqs_tsel_wr_timing_2(&self) -> PhyDqsTselWrTiming2R {
        PhyDqsTselWrTiming2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    pub fn phy_per_cs_training_en_2(&self) -> PhyPerCsTrainingEn2R {
        PhyPerCsTrainingEn2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 2."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_2(&self) -> PhyDqIeTiming2R {
        PhyDqIeTiming2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_rd_timing_2(&mut self) -> PhyDqsTselRdTiming2W<DdrDenaliPhy340Spec> {
        PhyDqsTselRdTiming2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_wr_timing_2(&mut self) -> PhyDqsTselWrTiming2W<DdrDenaliPhy340Spec> {
        PhyDqsTselWrTiming2W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_en_2(&mut self) -> PhyPerCsTrainingEn2W<DdrDenaliPhy340Spec> {
        PhyPerCsTrainingEn2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_2(&mut self) -> PhyDqIeTiming2W<DdrDenaliPhy340Spec> {
        PhyDqIeTiming2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_340::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_340::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy340Spec;
impl crate::RegisterSpec for DdrDenaliPhy340Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_340::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy340Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_340::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy340Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_340 to value 0"]
impl crate::Resettable for DdrDenaliPhy340Spec {
    const RESET_VALUE: u32 = 0;
}
