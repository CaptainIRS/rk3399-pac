#[doc = "Register `DDR_DENALI_PHY_211` reader"]
pub type R = crate::R<DdrDenaliPhy211Spec>;
#[doc = "Register `DDR_DENALI_PHY_211` writer"]
pub type W = crate::W<DdrDenaliPhy211Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_1` reader - Start/end timing values for DQ/DM output enable signals for slice 1."]
pub type PhyDqOeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_1` writer - Start/end timing values for DQ/DM output enable signals for slice 1."]
pub type PhyDqOeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_1` reader - Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
pub type PhyDqTselRdTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_1` writer - Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
pub type PhyDqTselRdTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_1` reader - Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
pub type PhyDqTselWrTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_1` writer - Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
pub type PhyDqTselWrTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_1` reader - Start/end timing values for DQS output enable signals for slice 1."]
pub type PhyDqsOeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_1` writer - Start/end timing values for DQS output enable signals for slice 1."]
pub type PhyDqsOeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_1(&self) -> PhyDqOeTiming1R {
        PhyDqOeTiming1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_1(&self) -> PhyDqTselRdTiming1R {
        PhyDqTselRdTiming1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_1(&self) -> PhyDqTselWrTiming1R {
        PhyDqTselWrTiming1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_1(&self) -> PhyDqsOeTiming1R {
        PhyDqsOeTiming1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_1(&mut self) -> PhyDqOeTiming1W<DdrDenaliPhy211Spec> {
        PhyDqOeTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_1(&mut self) -> PhyDqTselRdTiming1W<DdrDenaliPhy211Spec> {
        PhyDqTselRdTiming1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_1(&mut self) -> PhyDqTselWrTiming1W<DdrDenaliPhy211Spec> {
        PhyDqTselWrTiming1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_1(&mut self) -> PhyDqsOeTiming1W<DdrDenaliPhy211Spec> {
        PhyDqsOeTiming1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_211::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_211::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy211Spec;
impl crate::RegisterSpec for DdrDenaliPhy211Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_211::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy211Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_211::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy211Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_211 to value 0"]
impl crate::Resettable for DdrDenaliPhy211Spec {
    const RESET_VALUE: u32 = 0;
}
