#[doc = "Register `DENALI_PHY_212` reader"]
pub type R = crate::R<DenaliPhy212Spec>;
#[doc = "Register `DENALI_PHY_212` writer"]
pub type W = crate::W<DenaliPhy212Spec>;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_1` reader - Start/end timing values for DQS read based termination enable and select signals for slice 1."]
pub type PhyDqsTselRdTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_1` writer - Start/end timing values for DQS read based termination enable and select signals for slice 1."]
pub type PhyDqsTselRdTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_1` reader - Start/end timing values for DQS write based termination enable and select signals for slice 1."]
pub type PhyDqsTselWrTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_1` writer - Start/end timing values for DQS write based termination enable and select signals for slice 1."]
pub type PhyDqsTselWrTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_1` reader - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn1R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_1` writer - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DQ_IE_TIMING_1` reader - Start/end timing values for DQ/DM input enable signals for slice 1."]
pub type PhyDqIeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_1` writer - Start/end timing values for DQ/DM input enable signals for slice 1."]
pub type PhyDqIeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_tsel_rd_timing_1(&self) -> PhyDqsTselRdTiming1R {
        PhyDqsTselRdTiming1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_tsel_wr_timing_1(&self) -> PhyDqsTselWrTiming1R {
        PhyDqsTselWrTiming1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    pub fn phy_per_cs_training_en_1(&self) -> PhyPerCsTrainingEn1R {
        PhyPerCsTrainingEn1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_1(&self) -> PhyDqIeTiming1R {
        PhyDqIeTiming1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_rd_timing_1(&mut self) -> PhyDqsTselRdTiming1W<DenaliPhy212Spec> {
        PhyDqsTselRdTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_wr_timing_1(&mut self) -> PhyDqsTselWrTiming1W<DenaliPhy212Spec> {
        PhyDqsTselWrTiming1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_en_1(&mut self) -> PhyPerCsTrainingEn1W<DenaliPhy212Spec> {
        PhyPerCsTrainingEn1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_1(&mut self) -> PhyDqIeTiming1W<DenaliPhy212Spec> {
        PhyDqIeTiming1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_212::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_212::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy212Spec;
impl crate::RegisterSpec for DenaliPhy212Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_212::R`](R) reader structure"]
impl crate::Readable for DenaliPhy212Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_212::W`](W) writer structure"]
impl crate::Writable for DenaliPhy212Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_212 to value 0"]
impl crate::Resettable for DenaliPhy212Spec {
    const RESET_VALUE: u32 = 0;
}
