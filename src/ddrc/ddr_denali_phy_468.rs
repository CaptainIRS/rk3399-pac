#[doc = "Register `DDR_DENALI_PHY_468` reader"]
pub type R = crate::R<DdrDenaliPhy468Spec>;
#[doc = "Register `DDR_DENALI_PHY_468` writer"]
pub type W = crate::W<DdrDenaliPhy468Spec>;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_3` reader - Start/end timing values for DQS read based termination enable and select signals for slice 3."]
pub type PhyDqsTselRdTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_3` writer - Start/end timing values for DQS read based termination enable and select signals for slice 3."]
pub type PhyDqsTselRdTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_3` reader - Start/end timing values for DQS write based termination enable and select signals for slice 3."]
pub type PhyDqsTselWrTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_3` writer - Start/end timing values for DQS write based termination enable and select signals for slice 3."]
pub type PhyDqsTselWrTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_3` reader - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn3R = crate::BitReader;
#[doc = "Field `PHY_PER_CS_TRAINING_EN_3` writer - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
pub type PhyPerCsTrainingEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DQ_IE_TIMING_3` reader - Start/end timing values for DQ/DM input enable signals for slice 3."]
pub type PhyDqIeTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQ_IE_TIMING_3` writer - Start/end timing values for DQ/DM input enable signals for slice 3."]
pub type PhyDqIeTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 3."]
    #[inline(always)]
    pub fn phy_dqs_tsel_rd_timing_3(&self) -> PhyDqsTselRdTiming3R {
        PhyDqsTselRdTiming3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 3."]
    #[inline(always)]
    pub fn phy_dqs_tsel_wr_timing_3(&self) -> PhyDqsTselWrTiming3R {
        PhyDqsTselWrTiming3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    pub fn phy_per_cs_training_en_3(&self) -> PhyPerCsTrainingEn3R {
        PhyPerCsTrainingEn3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 3."]
    #[inline(always)]
    pub fn phy_dq_ie_timing_3(&self) -> PhyDqIeTiming3R {
        PhyDqIeTiming3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS read based termination enable and select signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_rd_timing_3(&mut self) -> PhyDqsTselRdTiming3W<DdrDenaliPhy468Spec> {
        PhyDqsTselRdTiming3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQS write based termination enable and select signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_wr_timing_3(&mut self) -> PhyDqsTselWrTiming3W<DdrDenaliPhy468Spec> {
        PhyDqsTselWrTiming3W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the per-rank training and read/write timing capabilities. Must have same value in all slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_per_cs_training_en_3(&mut self) -> PhyPerCsTrainingEn3W<DdrDenaliPhy468Spec> {
        PhyPerCsTrainingEn3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQ/DM input enable signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_ie_timing_3(&mut self) -> PhyDqIeTiming3W<DdrDenaliPhy468Spec> {
        PhyDqIeTiming3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_468::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_468::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy468Spec;
impl crate::RegisterSpec for DdrDenaliPhy468Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_468::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy468Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_468::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy468Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_468 to value 0"]
impl crate::Resettable for DdrDenaliPhy468Spec {
    const RESET_VALUE: u32 = 0;
}
