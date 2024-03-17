#[doc = "Register `DENALI_PHY_339` reader"]
pub type R = crate::R<DenaliPhy339Spec>;
#[doc = "Register `DENALI_PHY_339` writer"]
pub type W = crate::W<DenaliPhy339Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_2` reader - Start/end timing values for DQ/DM output enable signals for slice 2."]
pub type PhyDqOeTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_2` writer - Start/end timing values for DQ/DM output enable signals for slice 2."]
pub type PhyDqOeTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_2` reader - Start/end timing values for DQ/DM read based termination enable and select signals for slice 2."]
pub type PhyDqTselRdTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_2` writer - Start/end timing values for DQ/DM read based termination enable and select signals for slice 2."]
pub type PhyDqTselRdTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_2` reader - Start/end timing values for DQ/DM write based termination enable and select signals for slice 2."]
pub type PhyDqTselWrTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_2` writer - Start/end timing values for DQ/DM write based termination enable and select signals for slice 2."]
pub type PhyDqTselWrTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_2` reader - Start/end timing values for DQS output enable signals for slice 2."]
pub type PhyDqsOeTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_2` writer - Start/end timing values for DQS output enable signals for slice 2."]
pub type PhyDqsOeTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 2."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_2(&self) -> PhyDqOeTiming2R {
        PhyDqOeTiming2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 2."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_2(&self) -> PhyDqTselRdTiming2R {
        PhyDqTselRdTiming2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 2."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_2(&self) -> PhyDqTselWrTiming2R {
        PhyDqTselWrTiming2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 2."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_2(&self) -> PhyDqsOeTiming2R {
        PhyDqsOeTiming2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_2(&mut self) -> PhyDqOeTiming2W<DenaliPhy339Spec> {
        PhyDqOeTiming2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_2(&mut self) -> PhyDqTselRdTiming2W<DenaliPhy339Spec> {
        PhyDqTselRdTiming2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_2(&mut self) -> PhyDqTselWrTiming2W<DenaliPhy339Spec> {
        PhyDqTselWrTiming2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_2(&mut self) -> PhyDqsOeTiming2W<DenaliPhy339Spec> {
        PhyDqsOeTiming2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_339::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_339::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy339Spec;
impl crate::RegisterSpec for DenaliPhy339Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_339::R`](R) reader structure"]
impl crate::Readable for DenaliPhy339Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_339::W`](W) writer structure"]
impl crate::Writable for DenaliPhy339Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_339 to value 0"]
impl crate::Resettable for DenaliPhy339Spec {
    const RESET_VALUE: u32 = 0;
}
