#[doc = "Register `DENALI_PHY_258` reader"]
pub type R = crate::R<DenaliPhy258Spec>;
#[doc = "Register `DENALI_PHY_258` writer"]
pub type W = crate::W<DenaliPhy258Spec>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_2` reader - Read DQS bypass mode slave delay setting for slice 2."]
pub type PhyRddqsGateBypassSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_2` writer - Read DQS bypass mode slave delay setting for slice 2."]
pub type PhyRddqsGateBypassSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_2` reader - PHY two_cycle_preamble for bypass mode for slice 2."]
pub type PhyBypassTwoCycPreamble2R = crate::FieldReader;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_2` writer - PHY two_cycle_preamble for bypass mode for slice 2."]
pub type PhyBypassTwoCycPreamble2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_2` reader - Bypass mode override setting for slice 2."]
pub type PhyClkBypassOverride2R = crate::BitReader;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_2` writer - Bypass mode override setting for slice 2."]
pub type PhyClkBypassOverride2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_gate_bypass_slave_delay_2(&self) -> PhyRddqsGateBypassSlaveDelay2R {
        PhyRddqsGateBypassSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 2."]
    #[inline(always)]
    pub fn phy_bypass_two_cyc_preamble_2(&self) -> PhyBypassTwoCycPreamble2R {
        PhyBypassTwoCycPreamble2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 2."]
    #[inline(always)]
    pub fn phy_clk_bypass_override_2(&self) -> PhyClkBypassOverride2R {
        PhyClkBypassOverride2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_bypass_slave_delay_2(
        &mut self,
    ) -> PhyRddqsGateBypassSlaveDelay2W<DenaliPhy258Spec> {
        PhyRddqsGateBypassSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_bypass_two_cyc_preamble_2(&mut self) -> PhyBypassTwoCycPreamble2W<DenaliPhy258Spec> {
        PhyBypassTwoCycPreamble2W::new(self, 16)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_bypass_override_2(&mut self) -> PhyClkBypassOverride2W<DenaliPhy258Spec> {
        PhyClkBypassOverride2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_258::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_258::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy258Spec;
impl crate::RegisterSpec for DenaliPhy258Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_258::R`](R) reader structure"]
impl crate::Readable for DenaliPhy258Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_258::W`](W) writer structure"]
impl crate::Writable for DenaliPhy258Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_258 to value 0"]
impl crate::Resettable for DenaliPhy258Spec {
    const RESET_VALUE: u32 = 0;
}
