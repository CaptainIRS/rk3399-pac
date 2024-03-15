#[doc = "Register `DENALI_PHY_467` reader"]
pub type R = crate::R<DenaliPhy467Spec>;
#[doc = "Register `DENALI_PHY_467` writer"]
pub type W = crate::W<DenaliPhy467Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_3` reader - Start/end timing values for DQ/DM output enable signals for slice 3."]
pub type PhyDqOeTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_3` writer - Start/end timing values for DQ/DM output enable signals for slice 3."]
pub type PhyDqOeTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_3` reader - Start/end timing values for DQ/DM read based termination enable and select signals for slice 3."]
pub type PhyDqTselRdTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_3` writer - Start/end timing values for DQ/DM read based termination enable and select signals for slice 3."]
pub type PhyDqTselRdTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_3` reader - Start/end timing values for DQ/DM write based termination enable and select signals for slice 3."]
pub type PhyDqTselWrTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_3` writer - Start/end timing values for DQ/DM write based termination enable and select signals for slice 3."]
pub type PhyDqTselWrTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_3` reader - Start/end timing values for DQS output enable signals for slice 3."]
pub type PhyDqsOeTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_3` writer - Start/end timing values for DQS output enable signals for slice 3."]
pub type PhyDqsOeTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 3."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_3(&self) -> PhyDqOeTiming3R {
        PhyDqOeTiming3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 3."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_3(&self) -> PhyDqTselRdTiming3R {
        PhyDqTselRdTiming3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 3."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_3(&self) -> PhyDqTselWrTiming3R {
        PhyDqTselWrTiming3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 3."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_3(&self) -> PhyDqsOeTiming3R {
        PhyDqsOeTiming3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_3(&mut self) -> PhyDqOeTiming3W<DenaliPhy467Spec> {
        PhyDqOeTiming3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_3(&mut self) -> PhyDqTselRdTiming3W<DenaliPhy467Spec> {
        PhyDqTselRdTiming3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_3(&mut self) -> PhyDqTselWrTiming3W<DenaliPhy467Spec> {
        PhyDqTselWrTiming3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_3(&mut self) -> PhyDqsOeTiming3W<DenaliPhy467Spec> {
        PhyDqsOeTiming3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_467::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_467::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy467Spec;
impl crate::RegisterSpec for DenaliPhy467Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_467::R`](R) reader structure"]
impl crate::Readable for DenaliPhy467Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_467::W`](W) writer structure"]
impl crate::Writable for DenaliPhy467Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_467 to value 0"]
impl crate::Resettable for DenaliPhy467Spec {
    const RESET_VALUE: u32 = 0;
}
