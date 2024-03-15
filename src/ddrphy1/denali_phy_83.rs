#[doc = "Register `DENALI_PHY_83` reader"]
pub type R = crate::R<DenaliPhy83Spec>;
#[doc = "Register `DENALI_PHY_83` writer"]
pub type W = crate::W<DenaliPhy83Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_0` reader - Start/end timing values for DQ/DM output enable signals for slice 0."]
pub type PhyDqOeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_0` writer - Start/end timing values for DQ/DM output enable signals for slice 0."]
pub type PhyDqOeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_0` reader - Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
pub type PhyDqTselRdTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_0` writer - Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
pub type PhyDqTselRdTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_0` reader - Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
pub type PhyDqTselWrTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_0` writer - Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
pub type PhyDqTselWrTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_0` reader - Start/end timing values for DQS output enable signals for slice 0."]
pub type PhyDqsOeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_0` writer - Start/end timing values for DQS output enable signals for slice 0."]
pub type PhyDqsOeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_0(&self) -> PhyDqOeTiming0R {
        PhyDqOeTiming0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_0(&self) -> PhyDqTselRdTiming0R {
        PhyDqTselRdTiming0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_0(&self) -> PhyDqTselWrTiming0R {
        PhyDqTselWrTiming0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_0(&self) -> PhyDqsOeTiming0R {
        PhyDqsOeTiming0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQ/DM output enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_0(&mut self) -> PhyDqOeTiming0W<DenaliPhy83Spec> {
        PhyDqOeTiming0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_0(&mut self) -> PhyDqTselRdTiming0W<DenaliPhy83Spec> {
        PhyDqTselRdTiming0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_0(&mut self) -> PhyDqTselWrTiming0W<DenaliPhy83Spec> {
        PhyDqTselWrTiming0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start/end timing values for DQS output enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_0(&mut self) -> PhyDqsOeTiming0W<DenaliPhy83Spec> {
        PhyDqsOeTiming0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy83Spec;
impl crate::RegisterSpec for DenaliPhy83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_83::R`](R) reader structure"]
impl crate::Readable for DenaliPhy83Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_83::W`](W) writer structure"]
impl crate::Writable for DenaliPhy83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_83 to value 0"]
impl crate::Resettable for DenaliPhy83Spec {
    const RESET_VALUE: u32 = 0;
}
